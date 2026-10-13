// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/pass_builder.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a4469950ede6c9c06482c491ae43fccd83e8cc8d7994a2c81ccbc13919ee780a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: pass_builder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 41, 102, 120, 195, 219, 228, 234, 398, 402, 424, 449, 451, 464, 467, 473, 485, 490, 500, 1318, 1320, 1324, 1407
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use vk_sync_fork; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$0::drop
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3135
// 首条位置: {"module":195,"inlinee":12597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19864,"parent_record_offset":19836,"ancestor_record_offset":19836,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19948,"parent_record_offset":19836,"ancestor_record_offset":19836,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":195,"record_offset":19976,"parent_record_offset":19836,"ancestor_record_offset":19836,"inlinee":9079,"source_line":107,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":195,"record_offset":20076,"parent_record_offset":20044,"ancestor_record_offset":19836,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":195,"record_offset":20044,"parent_record_offset":20004,"ancestor_record_offset":19836,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":195,"record_offset":20004,"parent_record_offset":19976,"ancestor_record_offset":19836,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":195,"record_offset":19888,"parent_record_offset":19864,"ancestor_record_offset":19836,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":156612,"parent_record_offset":156588,"ancestor_record_offset":156536,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":195,"record_offset":19912,"parent_record_offset":19888,"ancestor_record_offset":19836,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::rg_mut
// 记录形式: inline_instance, inlinee_declaration; instances=110
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D02
// 首条位置: {"module":41,"inlinee":7426}
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
// 记录形式: inline_instance, inlinee_declaration; instances=97
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF6
// 首条位置: {"module":41,"inlinee":7414}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 首条位置: {"module":467,"record_offset":16800,"rva":6605744,"va":5375314864}
// 编译实例 RVAs: 0x64cbb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6455515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e8257f9dff","size":5,"soft_fail":false,"target_va":5368859888,"va":5375315398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3050177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e83f75d1ff","size":5,"soft_fail":false,"target_va":5372265280,"va":5375315452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368300
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3033908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e8ccb4d1ff","size":5,"soft_fail":false,"target_va":5372281600,"va":5375315503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2565270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e86adbd8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375314961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064b690
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e8ffe8ffff","size":5,"soft_fail":false,"target_va":5375309456,"va":5375315340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t587053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e82df50800","size":5,"soft_fail":false,"target_va":5375902656,"va":5375315598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28278416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e8907eaf01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375314939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27662903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e8371aa601","size":5,"soft_fail":false,"target_va":5402978144,"va":5375315236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4009503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e8e1d1c2ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5375315098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27663677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e83d1da601","size":5,"soft_fail":false,"target_va":5402978880,"va":5375315198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27663850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314864,"raw_hex":"e8ea1da601","size":5,"soft_fail":false,"target_va":5402979152,"va":5375315297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":467,"record_offset":17184,"parent_record_offset":16800,"ancestor_record_offset":16800,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":17116,"parent_record_offset":16800,"ancestor_record_offset":16800,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":17488,"parent_record_offset":16800,"ancestor_record_offset":16800,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":17372,"parent_record_offset":16800,"ancestor_record_offset":16800,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":17148,"parent_record_offset":16800,"ancestor_record_offset":16800,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":17544,"parent_record_offset":16800,"ancestor_record_offset":16800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":17276,"parent_record_offset":17252,"ancestor_record_offset":16800,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":17300,"parent_record_offset":17276,"ancestor_record_offset":16800,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":17220,"parent_record_offset":17184,"ancestor_record_offset":16800,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":17252,"parent_record_offset":17220,"ancestor_record_offset":16800,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":467,"record_offset":17812,"parent_record_offset":17780,"ancestor_record_offset":16800,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":17780,"parent_record_offset":17740,"ancestor_record_offset":16800,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":17740,"parent_record_offset":17712,"ancestor_record_offset":16800,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":467,"record_offset":17396,"parent_record_offset":17372,"ancestor_record_offset":16800,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":17512,"parent_record_offset":17488,"ancestor_record_offset":16800,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":17600,"parent_record_offset":17572,"ancestor_record_offset":16800,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":17684,"parent_record_offset":17572,"ancestor_record_offset":16800,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":17424,"parent_record_offset":17396,"ancestor_record_offset":16800,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":17324,"parent_record_offset":17300,"ancestor_record_offset":16800,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":17452,"parent_record_offset":17396,"ancestor_record_offset":16800,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":467,"record_offset":17572,"parent_record_offset":17544,"ancestor_record_offset":16800,"inlinee":12597,"source_line":25,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":467,"record_offset":17712,"parent_record_offset":17572,"ancestor_record_offset":16800,"inlinee":9079,"source_line":107,"archive":"_pdb/A/modules/00467.jsonl.gz"}
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
// 首条位置: {"module":467,"record_offset":15724,"rva":6604976,"va":5375314096}
// 编译实例 RVAs: 0x64c8b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6454747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e825829dff","size":5,"soft_fail":false,"target_va":5368859888,"va":5375314630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3049409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e83f78d1ff","size":5,"soft_fail":false,"target_va":5372265280,"va":5375314684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368300
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3033140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e8ccb7d1ff","size":5,"soft_fail":false,"target_va":5372281600,"va":5375314735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2564502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e86aded8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375314193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064b360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e8cfe8ffff","size":5,"soft_fail":false,"target_va":5375308640,"va":5375314572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t587821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e82df80800","size":5,"soft_fail":false,"target_va":5375902656,"va":5375314830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28279184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e89081af01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375314171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27663671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e8371da601","size":5,"soft_fail":false,"target_va":5402978144,"va":5375314468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4008735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e8e1d4c2ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5375314330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27664445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e83d20a601","size":5,"soft_fail":false,"target_va":5402978880,"va":5375314430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27664618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375314096,"raw_hex":"e8ea20a601","size":5,"soft_fail":false,"target_va":5402979152,"va":5375314529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":467,"record_offset":16092,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":16024,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":16396,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":16280,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":16056,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":16452,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":16184,"parent_record_offset":16160,"ancestor_record_offset":15724,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":16208,"parent_record_offset":16184,"ancestor_record_offset":15724,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":16128,"parent_record_offset":16092,"ancestor_record_offset":15724,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":16160,"parent_record_offset":16128,"ancestor_record_offset":15724,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":467,"record_offset":16720,"parent_record_offset":16688,"ancestor_record_offset":15724,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":16688,"parent_record_offset":16648,"ancestor_record_offset":15724,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":16648,"parent_record_offset":16620,"ancestor_record_offset":15724,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":467,"record_offset":16304,"parent_record_offset":16280,"ancestor_record_offset":15724,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":15724,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":16508,"parent_record_offset":16480,"ancestor_record_offset":15724,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":16592,"parent_record_offset":16480,"ancestor_record_offset":15724,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":16332,"parent_record_offset":16304,"ancestor_record_offset":15724,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":16232,"parent_record_offset":16208,"ancestor_record_offset":15724,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":16360,"parent_record_offset":16304,"ancestor_record_offset":15724,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":467,"record_offset":16480,"parent_record_offset":16452,"ancestor_record_offset":15724,"inlinee":12597,"source_line":25,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":467,"record_offset":16620,"parent_record_offset":16480,"ancestor_record_offset":15724,"inlinee":9079,"source_line":107,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_buffer::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":467,"record_offset":10992,"rva":6602528,"va":5375311648}
// 编译实例 RVAs: 0x64bf20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6452147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e84d8c9dff","size":5,"soft_fail":false,"target_va":5368859888,"va":5375312030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3046799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e87182d1ff","size":5,"soft_fail":false,"target_va":5372265280,"va":5375312074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368300
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3030527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e801c2d1ff","size":5,"soft_fail":false,"target_va":5372281600,"va":5375312122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2562030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e812e8d8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375311721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t590481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e891020900","size":5,"soft_fail":false,"target_va":5375902656,"va":5375312170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27666199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e81727a601","size":5,"soft_fail":false,"target_va":5402978144,"va":5375311940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4006243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e89ddec2ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5375311838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27666956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e80c2aa601","size":5,"soft_fail":false,"target_va":5402978880,"va":5375311919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27667169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375311648,"raw_hex":"e8e12aa601","size":5,"soft_fail":false,"target_va":5402979152,"va":5375311978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":467,"record_offset":11344,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":11280,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":11636,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":11524,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":11308,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":11692,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":10992,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":11452,"parent_record_offset":11428,"ancestor_record_offset":10992,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":11372,"parent_record_offset":11344,"ancestor_record_offset":10992,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":11404,"parent_record_offset":11372,"ancestor_record_offset":10992,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":467,"record_offset":11960,"parent_record_offset":11928,"ancestor_record_offset":10992,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":11928,"parent_record_offset":11888,"ancestor_record_offset":10992,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":11888,"parent_record_offset":11860,"ancestor_record_offset":10992,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":467,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":10992,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":10992,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":11748,"parent_record_offset":11720,"ancestor_record_offset":10992,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":11832,"parent_record_offset":11720,"ancestor_record_offset":10992,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":11572,"parent_record_offset":11548,"ancestor_record_offset":10992,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":10992,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":11600,"parent_record_offset":11548,"ancestor_record_offset":10992,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":467,"record_offset":11720,"parent_record_offset":11692,"ancestor_record_offset":10992,"inlinee":12597,"source_line":25,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":467,"record_offset":11860,"parent_record_offset":11720,"ancestor_record_offset":10992,"inlinee":9079,"source_line":107,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_color_image::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":467,"record_offset":13140,"rva":6603744,"va":5375312864}
// 编译实例 RVAs: 0x64c3e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6453363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e88d879dff","size":5,"soft_fail":false,"target_va":5368859888,"va":5375313246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3048015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e8b17dd1ff","size":5,"soft_fail":false,"target_va":5372265280,"va":5375313290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368300
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3031743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e841bdd1ff","size":5,"soft_fail":false,"target_va":5372281600,"va":5375313338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2563246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e852e3d8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375312937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t589265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e8d1fd0800","size":5,"soft_fail":false,"target_va":5375902656,"va":5375313386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27664983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e85722a601","size":5,"soft_fail":false,"target_va":5402978144,"va":5375313156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4007459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e8ddd9c2ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5375313054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27665740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e84c25a601","size":5,"soft_fail":false,"target_va":5402978880,"va":5375313135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27665953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312864,"raw_hex":"e82126a601","size":5,"soft_fail":false,"target_va":5402979152,"va":5375313194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":467,"record_offset":13496,"parent_record_offset":13140,"ancestor_record_offset":13140,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":13432,"parent_record_offset":13140,"ancestor_record_offset":13140,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":13788,"parent_record_offset":13140,"ancestor_record_offset":13140,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":13676,"parent_record_offset":13140,"ancestor_record_offset":13140,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":13460,"parent_record_offset":13140,"ancestor_record_offset":13140,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":13844,"parent_record_offset":13140,"ancestor_record_offset":13140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":13140,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":13604,"parent_record_offset":13580,"ancestor_record_offset":13140,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":13524,"parent_record_offset":13496,"ancestor_record_offset":13140,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":13556,"parent_record_offset":13524,"ancestor_record_offset":13140,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":467,"record_offset":14112,"parent_record_offset":14080,"ancestor_record_offset":13140,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":14080,"parent_record_offset":14040,"ancestor_record_offset":13140,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":14040,"parent_record_offset":14012,"ancestor_record_offset":13140,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":467,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":13140,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":13140,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":13900,"parent_record_offset":13872,"ancestor_record_offset":13140,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":13984,"parent_record_offset":13872,"ancestor_record_offset":13140,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":13724,"parent_record_offset":13700,"ancestor_record_offset":13140,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":13628,"parent_record_offset":13604,"ancestor_record_offset":13140,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":13752,"parent_record_offset":13700,"ancestor_record_offset":13140,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":467,"record_offset":13872,"parent_record_offset":13844,"ancestor_record_offset":13140,"inlinee":12597,"source_line":25,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":467,"record_offset":14012,"parent_record_offset":13872,"ancestor_record_offset":13140,"inlinee":9079,"source_line":107,"archive":"_pdb/A/modules/00467.jsonl.gz"}
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
// 首条位置: {"module":467,"record_offset":12040,"rva":6603072,"va":5375312192}
// 编译实例 RVAs: 0x64c140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6452773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e8db899dff","size":5,"soft_fail":false,"target_va":5368859888,"va":5375312656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3047441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e8ef7fd1ff","size":5,"soft_fail":false,"target_va":5372265280,"va":5375312716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368300
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3031169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e87fbfd1ff","size":5,"soft_fail":false,"target_va":5372281600,"va":5375312764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2562577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e8efe5d8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375312268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064b8c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e8dff5ffff","size":5,"soft_fail":false,"target_va":5375310016,"va":5375312604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t589814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e8f6ff0800","size":5,"soft_fail":false,"target_va":5375902656,"va":5375312837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28281058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e8e288af01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375312297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27665618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e8d224a601","size":5,"soft_fail":false,"target_va":5402978144,"va":5375312521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4006806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e86adcc2ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5375312401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27666383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e8cf27a601","size":5,"soft_fail":false,"target_va":5402978880,"va":5375312492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27666580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375312192,"raw_hex":"e89428a601","size":5,"soft_fail":false,"target_va":5402979152,"va":5375312567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":467,"record_offset":12436,"parent_record_offset":12040,"ancestor_record_offset":12040,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":12368,"parent_record_offset":12040,"ancestor_record_offset":12040,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":12736,"parent_record_offset":12040,"ancestor_record_offset":12040,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":12624,"parent_record_offset":12040,"ancestor_record_offset":12040,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":12400,"parent_record_offset":12040,"ancestor_record_offset":12040,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":12792,"parent_record_offset":12040,"ancestor_record_offset":12040,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":12528,"parent_record_offset":12504,"ancestor_record_offset":12040,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":12552,"parent_record_offset":12528,"ancestor_record_offset":12040,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":12472,"parent_record_offset":12436,"ancestor_record_offset":12040,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":12504,"parent_record_offset":12472,"ancestor_record_offset":12040,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":467,"record_offset":13060,"parent_record_offset":13028,"ancestor_record_offset":12040,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":13028,"parent_record_offset":12988,"ancestor_record_offset":12040,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":12988,"parent_record_offset":12960,"ancestor_record_offset":12040,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":467,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":12040,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":12040,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":12848,"parent_record_offset":12820,"ancestor_record_offset":12040,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":12932,"parent_record_offset":12820,"ancestor_record_offset":12040,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":12672,"parent_record_offset":12648,"ancestor_record_offset":12040,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":12576,"parent_record_offset":12552,"ancestor_record_offset":12040,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":12700,"parent_record_offset":12648,"ancestor_record_offset":12040,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":467,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":12040,"inlinee":12597,"source_line":25,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":467,"record_offset":12960,"parent_record_offset":12820,"ancestor_record_offset":12040,"inlinee":9079,"source_line":107,"archive":"_pdb/A/modules/00467.jsonl.gz"}
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
// 首条位置: {"module":467,"record_offset":14192,"rva":6604288,"va":5375313408}
// 编译实例 RVAs: 0x64c600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6454003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e80d859dff","size":5,"soft_fail":false,"target_va":5368859888,"va":5375313886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3048655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e8317bd1ff","size":5,"soft_fail":false,"target_va":5372265280,"va":5375313930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368300
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3032383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e8c1bad1ff","size":5,"soft_fail":false,"target_va":5372281600,"va":5375313978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2563791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e831e1d8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375313482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t588580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e824fb0800","size":5,"soft_fail":false,"target_va":5375902656,"va":5375314071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t926077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e87d210e00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375313838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27664404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e81420a601","size":5,"soft_fail":false,"target_va":5402978144,"va":5375313735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4008022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e8aad7c2ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5375313617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27665169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e81123a601","size":5,"soft_fail":false,"target_va":5402978880,"va":5375313706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27665366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375313408,"raw_hex":"e8d623a601","size":5,"soft_fail":false,"target_va":5402979152,"va":5375313781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":467,"record_offset":14564,"parent_record_offset":14192,"ancestor_record_offset":14192,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":14496,"parent_record_offset":14192,"ancestor_record_offset":14192,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":15092,"parent_record_offset":14192,"ancestor_record_offset":14192,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":14980,"parent_record_offset":14192,"ancestor_record_offset":14192,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":14528,"parent_record_offset":14192,"ancestor_record_offset":14192,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":15148,"parent_record_offset":14192,"ancestor_record_offset":14192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":14656,"parent_record_offset":14632,"ancestor_record_offset":14192,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":14680,"parent_record_offset":14656,"ancestor_record_offset":14192,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":14600,"parent_record_offset":14564,"ancestor_record_offset":14192,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":467,"record_offset":14632,"parent_record_offset":14600,"ancestor_record_offset":14192,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":467,"record_offset":15416,"parent_record_offset":15384,"ancestor_record_offset":14192,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":15384,"parent_record_offset":15344,"ancestor_record_offset":14192,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":467,"record_offset":15344,"parent_record_offset":15316,"ancestor_record_offset":14192,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":467,"record_offset":14824,"parent_record_offset":14796,"ancestor_record_offset":14192,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":467,"record_offset":15004,"parent_record_offset":14980,"ancestor_record_offset":14192,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":14192,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":15204,"parent_record_offset":15176,"ancestor_record_offset":14192,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":467,"record_offset":15288,"parent_record_offset":15176,"ancestor_record_offset":14192,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":14748,"parent_record_offset":14564,"ancestor_record_offset":14192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":14848,"parent_record_offset":14824,"ancestor_record_offset":14192,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":14192,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":467,"record_offset":15056,"parent_record_offset":15004,"ancestor_record_offset":14192,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":467,"record_offset":14796,"parent_record_offset":14772,"ancestor_record_offset":14192,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":467,"record_offset":15176,"parent_record_offset":15148,"ancestor_record_offset":14192,"inlinee":12597,"source_line":25,"archive":"_pdb/A/modules/00467.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":467,"record_offset":15316,"parent_record_offset":15176,"ancestor_record_offset":14192,"inlinee":9079,"source_line":107,"archive":"_pdb/A/modules/00467.jsonl.gz"}
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
// 首条位置: {"module":195,"record_offset":19292,"rva":3128080,"va":5371837200}
// 编译实例 RVAs: 0x2fbb10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2977615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371837200,"raw_hex":"e8b190d2ff","size":5,"soft_fail":false,"target_va":5368859888,"va":5371837498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t427733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371837200,"raw_hex":"e8d5860600","size":5,"soft_fail":false,"target_va":5372265280,"va":5371837542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t429781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371837200,"raw_hex":"e8d58e0600","size":5,"soft_fail":false,"target_va":5372267424,"va":5371837638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368300
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t444005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371837200,"raw_hex":"e865c60600","size":5,"soft_fail":false,"target_va":5372281600,"va":5371837590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-531755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371837200,"raw_hex":"e8d5e2f7ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5371837350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31141448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371837200,"raw_hex":"e8482edb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5371837427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31141682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371837200,"raw_hex":"e8322fdb01","size":5,"soft_fail":false,"target_va":5402979152,"va":5371837465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19576,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19752,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19640,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19604,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":195,"record_offset":19808,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":195,"record_offset":20076,"parent_record_offset":20044,"ancestor_record_offset":19292,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":195,"record_offset":20044,"parent_record_offset":20004,"ancestor_record_offset":19292,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":195,"record_offset":20004,"parent_record_offset":19976,"ancestor_record_offset":19292,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":195,"record_offset":19664,"parent_record_offset":19640,"ancestor_record_offset":19292,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19776,"parent_record_offset":19752,"ancestor_record_offset":19292,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19864,"parent_record_offset":19836,"ancestor_record_offset":19292,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":19948,"parent_record_offset":19836,"ancestor_record_offset":19292,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":195,"record_offset":19688,"parent_record_offset":19664,"ancestor_record_offset":19292,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":195,"record_offset":19716,"parent_record_offset":19664,"ancestor_record_offset":19292,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":195,"record_offset":19836,"parent_record_offset":19808,"ancestor_record_offset":19292,"inlinee":12597,"source_line":25,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":195,"record_offset":19976,"parent_record_offset":19836,"ancestor_record_offset":19292,"inlinee":9079,"source_line":107,"archive":"_pdb/A/modules/00195.jsonl.gz"}
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
// 首条位置: {"module":195,"record_offset":18560,"rva":3127424,"va":5371836544}
// 编译实例 RVAs: 0x2fb880, 0x64bc90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pass_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3372902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371836544,"raw_hex":"e866773300","size":5,"soft_fail":false,"target_va":5375209776,"va":5371836869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-530735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371836544,"raw_hex":"e8d1e6f7ff","size":5,"soft_fail":false,"target_va":5371306160,"va":5371836890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31141863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371836544,"raw_hex":"e8e72fdb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5371837012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31142354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371836544,"raw_hex":"e8d231db01","size":5,"soft_fail":false,"target_va":5402979344,"va":5371836985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31141754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371836544,"raw_hex":"e87a2fdb01","size":5,"soft_fail":false,"target_va":5402978944,"va":5371837185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::recorder::ResourceSyncGroup::is_equivalent
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4422002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371836544,"raw_hex":"e872794300","size":5,"soft_fail":false,"target_va":5376258720,"va":5371836713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":195,"record_offset":19132,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":195,"record_offset":18860,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":195,"record_offset":19260,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":18796,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":195,"record_offset":18824,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::impl$5::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":195,"record_offset":18968,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":12593,"source_line":223,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::impl$5::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":195,"record_offset":19104,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":12595,"source_line":21,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":195,"record_offset":19196,"parent_record_offset":19164,"ancestor_record_offset":18560,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":195,"record_offset":19164,"parent_record_offset":19132,"ancestor_record_offset":18560,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":195,"record_offset":19228,"parent_record_offset":19132,"ancestor_record_offset":18560,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":195,"record_offset":18892,"parent_record_offset":18860,"ancestor_record_offset":18560,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":195,"record_offset":18924,"parent_record_offset":18892,"ancestor_record_offset":18560,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00195.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::write
// 记录形式: inline_instance, inlinee_declaration; instances=79
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF4
// 首条位置: {"module":41,"inlinee":7412}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=62
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":41,"record_offset":3672,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::write_concurrent
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x593C
// 首条位置: {"module":398,"inlinee":22844}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":398,"record_offset":13184,"parent_record_offset":13156,"ancestor_record_offset":13156,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::raster
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68F2
// 首条位置: {"module":473,"inlinee":26866}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":473,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_builder::impl$2::read
// 记录形式: inline_instance, inlinee_declaration; instances=100
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF7
// 首条位置: {"module":41,"inlinee":7415}
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
// 源码行: 224
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67ED
// 首条位置: {"module":473,"inlinee":26605}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::add_resource_access", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::pass_builder::PassBuilder", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_buffer::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::clear_rg_buffer::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_color_image::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::clear_rg_color_image::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>", "constructor": "rhapsody::rg::pass_builder::PassBuilder::render", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 136; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>", "bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::geometry::instanced_wall::InstancedWall> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::geometry::instanced_wall::InstancedWall> >,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::geometry::instanced_wall::InstancedWall> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: f1143bbaaf680d05908ba764
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::add_resource_access -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 162 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d04 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d69 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x65271 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x65371 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x65471 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x65571 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15bd20 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1ccc34 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1ccc99 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd594 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd5f9 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338de4 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338e49 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x35849b -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x358732 -> 0x2fb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer @ 0x53ed8a -> 0x64bc90 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
// 信息 ID: 50a674937779a2f77cbba0e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 110 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute @ 0x509359 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute @ 0x50a746 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus @ 0x50fcfc -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings @ 0x5369bc -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings @ 0x536e46 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw @ 0x53fbb1 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d @ 0x53fd7c -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d @ 0x53ff0c -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_groups @ 0x540098 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof::dof_prepare @ 0x54b4fc -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof::calc_dof @ 0x54bd59 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows @ 0x552d64 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean @ 0x55319a -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture @ 0x554245 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain @ 0x554f8e -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug @ 0x578918 -> 0x64cbb0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>
// 信息 ID: 93316f77d083b040ea6ab76d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x66195c -> 0x64c8b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_buffer::closure_env$0>
// 信息 ID: b4fe3f4312913c97dc66aa7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::clear_rg_buffer::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x608d40 -> 0x64bf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x611716 -> 0x64bf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x61274c -> 0x64bf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass::grass_compute @ 0x692b11 -> 0x64bf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0x6af203 -> 0x64bf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo> @ 0x6af443 -> 0x64bf20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_color_image::closure_env$0>
// 信息 ID: 867efca35fcff26b3a9c0e5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::clear_rg_color_image::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60a852 -> 0x64c3e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>
// 信息 ID: 09de662677d18ae3eee98ddd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x553939 -> 0x64c140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >
// 信息 ID: 9b9cb236a1e1a78a72d912f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 -> ["u32"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32> @ 0x6af71f -> 0x64c600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>
// 信息 ID: e9dce1b4bc8734d209d4ed3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::pass_builder::PassBuilder::render -> ["country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15bd66 -> 0x2fbb10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::rg_mut
// 信息 ID: 63919ffc9b4974f8b0687038
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 104 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>; module=41, inline_record=5840; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=41, inline_record=6452; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo>; module=41, inline_record=7064; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=41, inline_record=7680; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::draw_ext::impl$0::nani_draw; module=490, inline_record=28460; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::draw_ext::impl$0::nani_draw; module=490, inline_record=28488; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indirect; module=490, inline_record=43104; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect; module=490, inline_record=44136; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::PassBuilder::write_impl
// 信息 ID: 5ddf74727ea3d848a8927fea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 75 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=3672; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=3668; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=9060; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>; module=219, inline_record=3648; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=228, inline_record=13188; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer; module=398, inline_record=11456; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer_concurrent; module=398, inline_record=13184; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>; module=398, inline_record=23860; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$0::drop
// 信息 ID: d02966c44ab8a786bd05670e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>; module=195, inline_record=19836; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<rhapsody::rg::pass_builder::PassBuilder<country_core::render::pass_builder::pass_resources::PassResources> >; module=234, inline_record=156536; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>; module=234, inline_record=861596; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>; module=234, inline_record=947924; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_buffer::closure_env$0>; module=467, inline_record=11720; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>; module=467, inline_record=12820; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_color_image::closure_env$0>; module=467, inline_record=13872; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >; module=467, inline_record=15176; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::raster
// 信息 ID: 002ded8e10358338841f5127
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref_mut$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> > >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0>; module=473, inline_record=808; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=473, inline_record=2540; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1407, inline_record=4492; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1407, inline_record=4696; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1407, inline_record=4900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::raster_read
// 信息 ID: 9eda1869aefc29bc708aaae6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=473, inline_record=2484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::read
// 信息 ID: 2313c24f1fd6aee45dfcc98a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 86 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=3840; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>; module=41, inline_record=5924; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=41, inline_record=6536; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo>; module=41, inline_record=7148; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=41, inline_record=7764; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::screenshot::handle_screenshot_opportunity; module=102, inline_record=23744; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=3836; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=9228; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::write
// 信息 ID: 590f69f5380b558d523ecf82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 62 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=3640; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=3636; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=9028; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>; module=219, inline_record=3616; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=228, inline_record=13156; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer; module=398, inline_record=11428; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>; module=398, inline_record=23828; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>; module=398, inline_record=33444; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_builder::impl$2::write_concurrent
// 信息 ID: 40b586f7159c02b2f937ba03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer_concurrent; module=398, inline_record=13156; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>; module=1320, inline_record=73800; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>; module=1320, inline_record=74484; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>; module=1320, inline_record=75168; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>; module=1320, inline_record=75864; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniDrawListSsbo>; module=1320, inline_record=76536; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>; module=1320, inline_record=77232; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>; module=1320, inline_record=77916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
