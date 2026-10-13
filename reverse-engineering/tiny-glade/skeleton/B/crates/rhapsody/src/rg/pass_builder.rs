// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/pass_builder.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d5dc5a7f2f140fd6f7e9f31fbd43aa9bcf9dd0b3cd0f776dfe4fcb14c2f066bb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: pass_builder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 61, 102, 130, 145, 149, 219, 253, 482, 485, 500, 502, 503, 518, 533, 595, 1623, 1632, 1638
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use vk_sync_fork; // namespace candidate
//   package identities: [{"name":"vk-sync-fork","source":"local_or_path","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$0::drop
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B39
// 首条位置: {"module":219,"inlinee":15161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":61940,"parent_record_offset":61912,"ancestor_record_offset":61912,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":62024,"parent_record_offset":61912,"ancestor_record_offset":61912,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":219,"record_offset":62052,"parent_record_offset":61912,"ancestor_record_offset":61912,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":62160,"parent_record_offset":62128,"ancestor_record_offset":61912,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":62208,"parent_record_offset":62184,"ancestor_record_offset":61912,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":62184,"parent_record_offset":62160,"ancestor_record_offset":61912,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":219,"record_offset":62128,"parent_record_offset":62080,"ancestor_record_offset":61912,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":219,"record_offset":62080,"parent_record_offset":62052,"ancestor_record_offset":61912,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":219,"record_offset":61964,"parent_record_offset":61940,"ancestor_record_offset":61912,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":235684,"parent_record_offset":235660,"ancestor_record_offset":235608,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":219,"record_offset":61988,"parent_record_offset":61964,"ancestor_record_offset":61912,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::rg_mut
// 记录形式: inline_instance, inlinee_declaration; instances=119
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D6A
// 首条位置: {"module":102,"inlinee":11626}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::write_impl
// 记录形式: inline_instance, inlinee_declaration; instances=90
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2598
// 首条位置: {"module":61,"inlinee":9624}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::render
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B41
// 首条位置: {"module":533,"inlinee":15169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":26992,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":26928,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27292,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27176,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":26956,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":27348,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":26868,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":26868,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":27020,"parent_record_offset":26992,"ancestor_record_offset":26868,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":27052,"parent_record_offset":27020,"ancestor_record_offset":26868,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":27628,"parent_record_offset":27596,"ancestor_record_offset":26868,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":27676,"parent_record_offset":27652,"ancestor_record_offset":26868,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":27652,"parent_record_offset":27628,"ancestor_record_offset":26868,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":27596,"parent_record_offset":27548,"ancestor_record_offset":26868,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":27548,"parent_record_offset":27520,"ancestor_record_offset":26868,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":533,"record_offset":27200,"parent_record_offset":27176,"ancestor_record_offset":26868,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27316,"parent_record_offset":27292,"ancestor_record_offset":26868,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27408,"parent_record_offset":27376,"ancestor_record_offset":26868,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27492,"parent_record_offset":27376,"ancestor_record_offset":26868,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":27228,"parent_record_offset":27200,"ancestor_record_offset":26868,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":27128,"parent_record_offset":27100,"ancestor_record_offset":26868,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":27256,"parent_record_offset":27200,"ancestor_record_offset":26868,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":27376,"parent_record_offset":27348,"ancestor_record_offset":26868,"inlinee":15161,"source_line":25,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":27520,"parent_record_offset":27376,"ancestor_record_offset":26868,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":21352,"rva":10343632,"va":5379052752}
// 编译实例 RVAs: 0x9dd4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ad810
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9633513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e817016dff","size":5,"soft_fail":false,"target_va":5369419792,"va":5379053300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4212559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e8b1b8bfff","size":5,"soft_fail":false,"target_va":5374840880,"va":5379053434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3971357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e8e366c3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379052840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1441463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e84901eaff","size":5,"soft_fail":false,"target_va":5377611920,"va":5379053378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dbeb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e8f1e7ffff","size":5,"soft_fail":false,"target_va":5379047088,"va":5379053242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32794345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e8e966f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379052818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32487784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e868b9ef01","size":5,"soft_fail":false,"target_va":5411540992,"va":5379053203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4697627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e8e551b8ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5379052998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32488606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e89ebcef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379053165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32488929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052752,"raw_hex":"e8e1bdef01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379053130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":21736,"parent_record_offset":21352,"ancestor_record_offset":21352,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":21668,"parent_record_offset":21352,"ancestor_record_offset":21352,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":22032,"parent_record_offset":21352,"ancestor_record_offset":21352,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":21920,"parent_record_offset":21352,"ancestor_record_offset":21352,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":21700,"parent_record_offset":21352,"ancestor_record_offset":21352,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":22088,"parent_record_offset":21352,"ancestor_record_offset":21352,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":21824,"parent_record_offset":21800,"ancestor_record_offset":21352,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":21848,"parent_record_offset":21824,"ancestor_record_offset":21352,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":21768,"parent_record_offset":21736,"ancestor_record_offset":21352,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":21800,"parent_record_offset":21768,"ancestor_record_offset":21352,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":22368,"parent_record_offset":22336,"ancestor_record_offset":21352,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":22416,"parent_record_offset":22392,"ancestor_record_offset":21352,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":22392,"parent_record_offset":22368,"ancestor_record_offset":21352,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":22336,"parent_record_offset":22288,"ancestor_record_offset":21352,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":22288,"parent_record_offset":22260,"ancestor_record_offset":21352,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":533,"record_offset":21944,"parent_record_offset":21920,"ancestor_record_offset":21352,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":22056,"parent_record_offset":22032,"ancestor_record_offset":21352,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":22148,"parent_record_offset":22116,"ancestor_record_offset":21352,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":22232,"parent_record_offset":22116,"ancestor_record_offset":21352,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":21968,"parent_record_offset":21944,"ancestor_record_offset":21352,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":21872,"parent_record_offset":21848,"ancestor_record_offset":21352,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":21996,"parent_record_offset":21944,"ancestor_record_offset":21352,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":22116,"parent_record_offset":22088,"ancestor_record_offset":21352,"inlinee":15161,"source_line":25,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":22260,"parent_record_offset":22116,"ancestor_record_offset":21352,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":17456,"rva":10341648,"va":5379050768}
// 编译实例 RVAs: 0x9dcd10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ad810
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9631529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e8d7086dff","size":5,"soft_fail":false,"target_va":5369419792,"va":5379051316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4210575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e871c0bfff","size":5,"soft_fail":false,"target_va":5374840880,"va":5379051450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3969373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e8a36ec3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379050856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1439479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e80909eaff","size":5,"soft_fail":false,"target_va":5377611920,"va":5379051394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dbb20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e821ecffff","size":5,"soft_fail":false,"target_va":5379046176,"va":5379051258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32796329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e8a96ef401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379050834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32489768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e828c1ef01","size":5,"soft_fail":false,"target_va":5411540992,"va":5379051219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4695643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e8a559b8ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5379051014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32490590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e85ec4ef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379051181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32490913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379050768,"raw_hex":"e8a1c5ef01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379051146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":17824,"parent_record_offset":17456,"ancestor_record_offset":17456,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":17756,"parent_record_offset":17456,"ancestor_record_offset":17456,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":18120,"parent_record_offset":17456,"ancestor_record_offset":17456,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":18008,"parent_record_offset":17456,"ancestor_record_offset":17456,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":17788,"parent_record_offset":17456,"ancestor_record_offset":17456,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":18176,"parent_record_offset":17456,"ancestor_record_offset":17456,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":17912,"parent_record_offset":17888,"ancestor_record_offset":17456,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":17456,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":17856,"parent_record_offset":17824,"ancestor_record_offset":17456,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":17888,"parent_record_offset":17856,"ancestor_record_offset":17456,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":18456,"parent_record_offset":18424,"ancestor_record_offset":17456,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":17456,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":18480,"parent_record_offset":18456,"ancestor_record_offset":17456,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":18424,"parent_record_offset":18376,"ancestor_record_offset":17456,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":18376,"parent_record_offset":18348,"ancestor_record_offset":17456,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":533,"record_offset":18032,"parent_record_offset":18008,"ancestor_record_offset":17456,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":18144,"parent_record_offset":18120,"ancestor_record_offset":17456,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":18236,"parent_record_offset":18204,"ancestor_record_offset":17456,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":18320,"parent_record_offset":18204,"ancestor_record_offset":17456,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":17456,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":17456,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":18084,"parent_record_offset":18032,"ancestor_record_offset":17456,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":18204,"parent_record_offset":18176,"ancestor_record_offset":17456,"inlinee":15161,"source_line":25,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":18348,"parent_record_offset":18204,"ancestor_record_offset":17456,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":18592,"rva":10342352,"va":5379051472}
// 编译实例 RVAs: 0x9dcfd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ad810
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9632179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e84d066dff","size":5,"soft_fail":false,"target_va":5369419792,"va":5379051966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4211199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e801bebfff","size":5,"soft_fail":false,"target_va":5374840880,"va":5379052074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3970056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e8f86bc3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379051539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1440110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e89206eaff","size":5,"soft_fail":false,"target_va":5377611920,"va":5379052025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dc120
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e891efffff","size":5,"soft_fail":false,"target_va":5379047712,"va":5379051914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32795595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e8cb6bf401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379051568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32489119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e89fbeef01","size":5,"soft_fail":false,"target_va":5411540992,"va":5379051868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4696325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e8fb56b8ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5379051696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32489932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e8ccc1ef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379051839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32490246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379051472,"raw_hex":"e806c3ef01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379051813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":18988,"parent_record_offset":18592,"ancestor_record_offset":18592,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":18920,"parent_record_offset":18592,"ancestor_record_offset":18592,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":19284,"parent_record_offset":18592,"ancestor_record_offset":18592,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":19172,"parent_record_offset":18592,"ancestor_record_offset":18592,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":18952,"parent_record_offset":18592,"ancestor_record_offset":18592,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":19340,"parent_record_offset":18592,"ancestor_record_offset":18592,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":19076,"parent_record_offset":19052,"ancestor_record_offset":18592,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":19100,"parent_record_offset":19076,"ancestor_record_offset":18592,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":19020,"parent_record_offset":18988,"ancestor_record_offset":18592,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":19052,"parent_record_offset":19020,"ancestor_record_offset":18592,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":19620,"parent_record_offset":19588,"ancestor_record_offset":18592,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":19668,"parent_record_offset":19644,"ancestor_record_offset":18592,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":19644,"parent_record_offset":19620,"ancestor_record_offset":18592,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":19588,"parent_record_offset":19540,"ancestor_record_offset":18592,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":19540,"parent_record_offset":19512,"ancestor_record_offset":18592,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":533,"record_offset":19196,"parent_record_offset":19172,"ancestor_record_offset":18592,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":19308,"parent_record_offset":19284,"ancestor_record_offset":18592,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":19400,"parent_record_offset":19368,"ancestor_record_offset":18592,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":19484,"parent_record_offset":19368,"ancestor_record_offset":18592,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":19220,"parent_record_offset":19196,"ancestor_record_offset":18592,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":19124,"parent_record_offset":19100,"ancestor_record_offset":18592,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":19248,"parent_record_offset":19196,"ancestor_record_offset":18592,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":19368,"parent_record_offset":19340,"ancestor_record_offset":18592,"inlinee":15161,"source_line":25,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":19512,"parent_record_offset":19368,"ancestor_record_offset":18592,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":19756,"rva":10342976,"va":5379052096}
// 编译实例 RVAs: 0x9dd240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ad810
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9632817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e8cf036dff","size":5,"soft_fail":false,"target_va":5369419792,"va":5379052604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4211853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e873bbbfff","size":5,"soft_fail":false,"target_va":5374840880,"va":5379052728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3970678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e88a69c3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379052161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1440768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e80004eaff","size":5,"soft_fail":false,"target_va":5377611920,"va":5379052683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2117791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e89f502000","size":5,"soft_fail":false,"target_va":5381170352,"va":5379052556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32488497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e831bcef01","size":5,"soft_fail":false,"target_va":5411540992,"va":5379052490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4696949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e88b54b8ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5379052320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32489310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e85ebfef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379052461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32489624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379052096,"raw_hex":"e898c0ef01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379052435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":20128,"parent_record_offset":19756,"ancestor_record_offset":19756,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":20060,"parent_record_offset":19756,"ancestor_record_offset":19756,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":20652,"parent_record_offset":19756,"ancestor_record_offset":19756,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":20540,"parent_record_offset":19756,"ancestor_record_offset":19756,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":20092,"parent_record_offset":19756,"ancestor_record_offset":19756,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":20708,"parent_record_offset":19756,"ancestor_record_offset":19756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":19756,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":20240,"parent_record_offset":20216,"ancestor_record_offset":19756,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":20160,"parent_record_offset":20128,"ancestor_record_offset":19756,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":20192,"parent_record_offset":20160,"ancestor_record_offset":19756,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":20988,"parent_record_offset":20956,"ancestor_record_offset":19756,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":21036,"parent_record_offset":21012,"ancestor_record_offset":19756,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":21012,"parent_record_offset":20988,"ancestor_record_offset":19756,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":20956,"parent_record_offset":20908,"ancestor_record_offset":19756,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":20908,"parent_record_offset":20880,"ancestor_record_offset":19756,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":533,"record_offset":20384,"parent_record_offset":20356,"ancestor_record_offset":19756,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":533,"record_offset":20564,"parent_record_offset":20540,"ancestor_record_offset":19756,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":20676,"parent_record_offset":20652,"ancestor_record_offset":19756,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":20768,"parent_record_offset":20736,"ancestor_record_offset":19756,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":20852,"parent_record_offset":20736,"ancestor_record_offset":19756,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":20308,"parent_record_offset":20128,"ancestor_record_offset":19756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":20408,"parent_record_offset":20384,"ancestor_record_offset":19756,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":20264,"parent_record_offset":20240,"ancestor_record_offset":19756,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":20616,"parent_record_offset":20564,"ancestor_record_offset":19756,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":533,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":19756,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":20736,"parent_record_offset":20708,"ancestor_record_offset":19756,"inlinee":15161,"source_line":25,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":20880,"parent_record_offset":20736,"ancestor_record_offset":19756,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":219,"record_offset":61368,"rva":5455248,"va":5374164368}
// 编译实例 RVAs: 0x533d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ad810
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4744912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164368,"raw_hex":"e83099b7ff","size":5,"soft_fail":false,"target_va":5369419792,"va":5374164699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t676084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164368,"raw_hex":"e8f4500a00","size":5,"soft_fail":false,"target_va":5374840880,"va":5374164791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t679412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164368,"raw_hex":"e8f45d0a00","size":5,"soft_fail":false,"target_va":5374844160,"va":5374164743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t190835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164368,"raw_hex":"e873e90200","size":5,"soft_fail":false,"target_va":5374355376,"va":5374164536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37377111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164368,"raw_hex":"e857543a02","size":5,"soft_fail":false,"target_va":5411541776,"va":5374164660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37377417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164368,"raw_hex":"e889553a02","size":5,"soft_fail":false,"target_va":5411542064,"va":5374164642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":61652,"parent_record_offset":61368,"ancestor_record_offset":61368,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":61828,"parent_record_offset":61368,"ancestor_record_offset":61368,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":61716,"parent_record_offset":61368,"ancestor_record_offset":61368,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":61680,"parent_record_offset":61368,"ancestor_record_offset":61368,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":219,"record_offset":61884,"parent_record_offset":61368,"ancestor_record_offset":61368,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":62160,"parent_record_offset":62128,"ancestor_record_offset":61368,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":62208,"parent_record_offset":62184,"ancestor_record_offset":61368,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":62184,"parent_record_offset":62160,"ancestor_record_offset":61368,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":219,"record_offset":62128,"parent_record_offset":62080,"ancestor_record_offset":61368,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":219,"record_offset":62080,"parent_record_offset":62052,"ancestor_record_offset":61368,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":219,"record_offset":61740,"parent_record_offset":61716,"ancestor_record_offset":61368,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":61852,"parent_record_offset":61828,"ancestor_record_offset":61368,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":61940,"parent_record_offset":61912,"ancestor_record_offset":61368,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":62024,"parent_record_offset":61912,"ancestor_record_offset":61368,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":219,"record_offset":61764,"parent_record_offset":61740,"ancestor_record_offset":61368,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":219,"record_offset":61792,"parent_record_offset":61740,"ancestor_record_offset":61368,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":219,"record_offset":61912,"parent_record_offset":61884,"ancestor_record_offset":61368,"inlinee":15161,"source_line":25,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":219,"record_offset":62052,"parent_record_offset":61912,"ancestor_record_offset":61368,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":219,"record_offset":60580,"rva":5454608,"va":5374163728}
// 编译实例 RVAs: 0x533b10, 0x9dca90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6234176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374163728,"raw_hex":"e840205f00","size":5,"soft_fail":false,"target_va":5380398224,"va":5374164043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t192648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374163728,"raw_hex":"e888f00200","size":5,"soft_fail":false,"target_va":5374356720,"va":5374164067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37377445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374163728,"raw_hex":"e8a5553a02","size":5,"soft_fail":false,"target_va":5411541776,"va":5374164326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37377953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374163728,"raw_hex":"e8a1573a02","size":5,"soft_fail":false,"target_va":5411542296,"va":5374164338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37377585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374163728,"raw_hex":"e831563a02","size":5,"soft_fail":false,"target_va":5411541904,"va":5374164314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::recorder::ResourceSyncGroup::is_equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7049901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374163728,"raw_hex":"e8ad926b00","size":5,"soft_fail":false,"target_va":5381213792,"va":5374163886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":219,"record_offset":61152,"parent_record_offset":60580,"ancestor_record_offset":60580,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":219,"record_offset":60880,"parent_record_offset":60580,"ancestor_record_offset":60580,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":219,"record_offset":61336,"parent_record_offset":60580,"ancestor_record_offset":60580,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":60816,"parent_record_offset":60580,"ancestor_record_offset":60580,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":219,"record_offset":60844,"parent_record_offset":60580,"ancestor_record_offset":60580,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::impl$7::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":219,"record_offset":60988,"parent_record_offset":60580,"ancestor_record_offset":60580,"inlinee":15159,"source_line":248,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::impl$5::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":219,"record_offset":61124,"parent_record_offset":60580,"ancestor_record_offset":60580,"inlinee":11962,"source_line":21,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":61216,"parent_record_offset":61184,"ancestor_record_offset":60580,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":61264,"parent_record_offset":61240,"ancestor_record_offset":60580,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":219,"record_offset":61240,"parent_record_offset":61216,"ancestor_record_offset":60580,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":219,"record_offset":61184,"parent_record_offset":61152,"ancestor_record_offset":60580,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":219,"record_offset":61304,"parent_record_offset":61152,"ancestor_record_offset":60580,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":219,"record_offset":60912,"parent_record_offset":60880,"ancestor_record_offset":60580,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":219,"record_offset":60944,"parent_record_offset":60912,"ancestor_record_offset":60580,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00219.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::write
// 记录形式: inline_instance, inlinee_declaration; instances=71
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2596
// 首条位置: {"module":61,"inlinee":9622}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=59
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":61,"record_offset":3612,"parent_record_offset":3580,"ancestor_record_offset":3580,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00061.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::write_concurrent
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73CF
// 首条位置: {"module":518,"inlinee":29647}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":518,"record_offset":19824,"parent_record_offset":19796,"ancestor_record_offset":19796,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::raster
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6858
// 首条位置: {"module":482,"inlinee":26712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":482,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":11664,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::read
// 记录形式: inline_instance, inlinee_declaration; instances=106
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25A0
// 首条位置: {"module":61,"inlinee":9632}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::raster_read
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6855
// 首条位置: {"module":482,"inlinee":26709}
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
// 完整结构及原始类型记录: pass_builder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::pass_builder::PassBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::add_resource_access", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::pass_builder::PassBuilder", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: pass_builder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: pass_builder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 190; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: cf26538b4f7838b7ecdf766b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::add_resource_access -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 167 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1557fc -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x15589c -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1561d9 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x15627d -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2afba9 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2afc4d -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x2b0121 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo> @ 0x2b0221 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo> @ 0x2b0321 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x2b0421 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x2b0521 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo> @ 0x2b0621 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x2b0721 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31dcee -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x364209 -> 0x533b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x3642ad -> 0x533b10 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render
// 信息 ID: 254f0842156bf005160cca24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::clear_rg_buffer; module=533, inline_record=26868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::clear_rg_color_image; module=533, inline_record=28028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
// 信息 ID: 657e4283f17cd9003058929c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 124 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu @ 0x86675b -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi @ 0x8746fb -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi @ 0x874b44 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi @ 0x875070 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics @ 0x8a4971 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x8a6cc0 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x8a6f11 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x8a73e7 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_pass @ 0x8a87ba -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x8a93da -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c840d -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c86d6 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c893a -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings @ 0x8e5114 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings @ 0x8e555e -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute @ 0x8f5041 -> 0x9dd4d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>
// 信息 ID: eaca0e8167b511068f1386b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x868a61 -> 0x9dcd10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>
// 信息 ID: 30840248b2a040423b3f3c46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x90ae6e -> 0x9dcfd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >
// 信息 ID: 226b043589e3e96c1e7bd8e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 -> ["u32"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32> @ 0x90dbcf -> 0x9dd240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>
// 信息 ID: edff7ae69c7fad278e524138
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31dd34 -> 0x533d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::rg_mut
// 信息 ID: 1e3fcfbe872f28c46d801b92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 114 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>; module=102, inline_record=6016; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>; module=102, inline_record=6688; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo>; module=102, inline_record=7360; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=102, inline_record=8024; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=102, inline_record=8692; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo>; module=102, inline_record=9364; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo>; module=102, inline_record=10028; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::draw_ext::impl$0::nani_draw; module=503, inline_record=10896; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::write_impl
// 信息 ID: 3593ec4bf045cbd5c5ce04e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 74 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=3612; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=9172; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=102, inline_record=3740; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>; module=145, inline_record=3716; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=149, inline_record=25596; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=482, inline_record=11688; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=500, inline_record=11128; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=500, inline_record=15956; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$0::drop
// 信息 ID: 548cf471f55c1abd5f91fd46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>; module=219, inline_record=61912; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources> >; module=253, inline_record=235608; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>; module=253, inline_record=1155432; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>; module=253, inline_record=1274684; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources> >; module=485, inline_record=243116; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>; module=485, inline_record=929488; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>; module=485, inline_record=1049748; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>; module=533, inline_record=18204; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::raster
// 信息 ID: 8e0ee0b7cf516588c8601c05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=482, inline_record=11664; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref_mut$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> > >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0>; module=595, inline_record=156344; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1623, inline_record=1440; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1623, inline_record=1644; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1623, inline_record=1840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::raster_read
// 信息 ID: 8c863c64bff332aa2aa8f28b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=482, inline_record=11608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::read
// 信息 ID: 14b11c182c45c4d628b34b64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 92 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=3676; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=9236; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=102, inline_record=3804; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>; module=102, inline_record=6100; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>; module=102, inline_record=6772; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo>; module=102, inline_record=7444; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=102, inline_record=8108; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=102, inline_record=8776; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::write
// 信息 ID: d3d3b2a217cc3fabd1f9d85a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 59 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=3580; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=9140; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=102, inline_record=3708; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>; module=145, inline_record=3684; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=149, inline_record=25564; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=500, inline_record=11096; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=500, inline_record=15924; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=500, inline_record=21856; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::write_concurrent
// 信息 ID: 1bb7c000d4c5376171619f78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer_concurrent; module=518, inline_record=19796; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>; module=518, inline_record=98364; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>; module=518, inline_record=99044; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>; module=518, inline_record=99732; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>; module=518, inline_record=100408; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniDrawListSsbo>; module=518, inline_record=101072; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>; module=518, inline_record=101760; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>; module=518, inline_record=102436; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
