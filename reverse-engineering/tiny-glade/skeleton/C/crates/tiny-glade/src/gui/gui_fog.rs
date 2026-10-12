// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/gui/gui_fog.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b6a0e487dd20e70dbc2da0e5e926a625febbc62d8e356cdf4c1d4598508d6552
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_fog.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 67, 84, 246
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::FogParam::range
// 记录形式: inlinee_declaration; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x455B
// 首条位置: {"module":246,"inlinee":17755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::FogParam::slider_to_value
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BD1
// 首条位置: {"module":84,"inlinee":11217}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":84,"record_offset":16216,"parent_record_offset":16176,"ancestor_record_offset":16176,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":16352,"parent_record_offset":16272,"ancestor_record_offset":16176,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":16176,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":16176,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::FogParam::value_to_slider
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BD0
// 首条位置: {"module":84,"inlinee":11216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::FogParam::get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BCF
// 首条位置: {"module":84,"inlinee":11215}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::FogParam::set
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BD2
// 首条位置: {"module":84,"inlinee":11218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::FogParam::track_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BCE
// 首条位置: {"module":84,"inlinee":11214}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_weather_slots
// 记录形式: compiled_procedure; instances=1
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":21868,"rva":6764160,"va":5375473280}
// 编译实例 RVAs: 0x673680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140315510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3531302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375473280,"raw_hex":"e8da1dcaff","size":5,"soft_fail":false,"target_va":5371942160,"va":5375473457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403168e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3526260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375473280,"raw_hex":"e88c31caff","size":5,"soft_fail":false,"target_va":5371947232,"va":5375473487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140317840
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3522403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375473280,"raw_hex":"e89d40caff","size":5,"soft_fail":false,"target_va":5371951168,"va":5375473566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403187f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3518244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375473280,"raw_hex":"e8dc50caff","size":5,"soft_fail":false,"target_va":5371955184,"va":5375473423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b05d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4793951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375473280,"raw_hex":"e85f264900","size":5,"soft_fail":false,"target_va":5380267296,"va":5375473340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tiny_glade::gui::gui_fog::fill_precipitation_slots
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":246,"record_offset":21996,"parent_record_offset":21868,"ancestor_record_offset":21868,"inlinee":17754,"source_line":209,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_weather_slots::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C04
// 首条位置: {"module":84,"inlinee":11268}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_weather_slots::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":15392,"rva":3212080,"va":5371921200}
// 编译实例 RVAs: 0x310330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140223a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-969247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e8e135f1ff","size":5,"soft_fail":false,"target_va":5370952224,"va":5371921466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4257996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e8ccf84000","size":5,"soft_fail":false,"target_va":5376179280,"va":5371921279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=10 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4258419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e873fa4000","size":5,"soft_fail":false,"target_va":5376179648,"va":5371921224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44429030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e8e6eea502","size":5,"soft_fail":false,"target_va":5416350384,"va":5371921349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::block_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4311766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e8d6ca4100","size":5,"soft_fail":false,"target_va":5376233024,"va":5371921253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::ignore_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4311553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e801ca4100","size":5,"soft_fail":false,"target_va":5376233040,"va":5371921482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::set_sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4311766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e8d6ca4100","size":5,"soft_fail":false,"target_va":5376233008,"va":5371921237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4311828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e814cb4100","size":5,"soft_fail":false,"target_va":5376233120,"va":5371921287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::with_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4311876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921200,"raw_hex":"e844cb4100","size":5,"soft_fail":false,"target_va":5376233152,"va":5371921271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":84,"record_offset":15532,"parent_record_offset":15392,"ancestor_record_offset":15392,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":15756,"parent_record_offset":15724,"ancestor_record_offset":15392,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":15804,"parent_record_offset":15780,"ancestor_record_offset":15392,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":15780,"parent_record_offset":15756,"ancestor_record_offset":15392,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":15724,"parent_record_offset":15684,"ancestor_record_offset":15392,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":15652,"parent_record_offset":15616,"ancestor_record_offset":15392,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":15684,"parent_record_offset":15652,"ancestor_record_offset":15392,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":15392,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":15392,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":84,"record_offset":15616,"parent_record_offset":15556,"ancestor_record_offset":15392,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":15844,"parent_record_offset":15684,"ancestor_record_offset":15392,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_weather_slots::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":15896,"rva":3212384,"va":5371921504}
// 编译实例 RVAs: 0x310460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11352313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921504,"raw_hex":"e8f938ad00","size":5,"soft_fail":false,"target_va":5383273968,"va":5371921650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::gui::gui_fog::fill_slider
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3550491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921504,"raw_hex":"e81b2d3600","size":5,"soft_fail":false,"target_va":5375472304,"va":5371921808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":84,"record_offset":16144,"parent_record_offset":15896,"ancestor_record_offset":15896,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_fog::FogParam::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":84,"record_offset":16084,"parent_record_offset":15896,"ancestor_record_offset":15896,"inlinee":11215,"source_line":81,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_fog::FogParam::set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":84,"record_offset":16472,"parent_record_offset":15896,"ancestor_record_offset":15896,"inlinee":11218,"source_line":89,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_fog::FogParam::slider_to_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":84,"record_offset":16176,"parent_record_offset":15896,"ancestor_record_offset":15896,"inlinee":11217,"source_line":60,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_fog::FogParam::track_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":84,"record_offset":16056,"parent_record_offset":15896,"ancestor_record_offset":15896,"inlinee":11214,"source_line":122,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_fog::FogParam::value_to_slider
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":84,"record_offset":16112,"parent_record_offset":15896,"ancestor_record_offset":15896,"inlinee":11216,"source_line":73,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":16352,"parent_record_offset":16272,"ancestor_record_offset":15896,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":15896,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":15896,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":84,"record_offset":16216,"parent_record_offset":16176,"ancestor_record_offset":15896,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_precipitation_slots
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x455A
// 首条位置: {"module":246,"inlinee":17754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":16516,"rva":3212832,"va":5371921952}
// 编译实例 RVAs: 0x310620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t12686893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e82d96c100","size":5,"soft_fail":false,"target_va":5384609408,"va":5371922510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=3
//     target_aliases=17 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2380210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e84eaedbff","size":5,"soft_fail":false,"target_va":5369542000,"va":5371922205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140223a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-970468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e81c31f1ff","size":5,"soft_fail":false,"target_va":5370952224,"va":5371922687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1562328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e8d8d61700","size":5,"soft_fail":false,"target_va":5373484384,"va":5371922051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1562708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e854d81700","size":5,"soft_fail":false,"target_va":5373484784,"va":5371922071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4257022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e8fef44000","size":5,"soft_fail":false,"target_va":5376179648,"va":5371922621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140725200
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4279187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e8934b4100","size":5,"soft_fail":false,"target_va":5376201216,"va":5371922024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727070
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4286718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e8fe684100","size":5,"soft_fail":false,"target_va":5376209008,"va":5371922285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727090
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4286873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e899694100","size":5,"soft_fail":false,"target_va":5376209040,"va":5371922162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4317892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e8c4e24100","size":5,"soft_fail":false,"target_va":5376240016,"va":5371922119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$1
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e86b030000","size":5,"soft_fail":false,"target_va":5371924112,"va":5371923232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::gui::gui_fog::fill_slider
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3548781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e86d263600","size":5,"soft_fail":false,"target_va":5375472304,"va":5371923518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::block_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4310390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e876c54100","size":5,"soft_fail":false,"target_va":5376233024,"va":5371922629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::ignore_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4310332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e83cc54100","size":5,"soft_fail":false,"target_va":5376233040,"va":5371922703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4286273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e841674100","size":5,"soft_fail":false,"target_va":5376208368,"va":5371922090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::has_global_id
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4285880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371921952,"raw_hex":"e8b8654100","size":5,"soft_fail":false,"target_va":5376208144,"va":5371922259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::config::TemplateView::with_slot
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":84,"record_offset":16780,"parent_record_offset":16516,"ancestor_record_offset":16516,"inlinee":11220,"source_line":233,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":84,"record_offset":16668,"parent_record_offset":16516,"ancestor_record_offset":16516,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":84,"record_offset":17004,"parent_record_offset":16780,"ancestor_record_offset":16516,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":84,"record_offset":18020,"parent_record_offset":17976,"ancestor_record_offset":16516,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":84,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":16516,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":84,"record_offset":16944,"parent_record_offset":16920,"ancestor_record_offset":16516,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":84,"record_offset":16968,"parent_record_offset":16944,"ancestor_record_offset":16516,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":84,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":16516,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":84,"record_offset":17060,"parent_record_offset":16780,"ancestor_record_offset":16516,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":84,"record_offset":17084,"parent_record_offset":17060,"ancestor_record_offset":16516,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":84,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":16516,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":84,"record_offset":16920,"parent_record_offset":16780,"ancestor_record_offset":16516,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":84,"record_offset":17288,"parent_record_offset":16780,"ancestor_record_offset":16516,"inlinee":11222,"source_line":227,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":84,"record_offset":17976,"parent_record_offset":17836,"ancestor_record_offset":16516,"inlinee":11223,"source_line":266,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":84,"record_offset":17172,"parent_record_offset":16780,"ancestor_record_offset":16516,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":84,"record_offset":16716,"parent_record_offset":16692,"ancestor_record_offset":16516,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":84,"record_offset":16744,"parent_record_offset":16692,"ancestor_record_offset":16516,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":84,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":16516,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BD6
// 首条位置: {"module":84,"inlinee":11222}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":18512,"rva":3214992,"va":5371924112}
// 编译实例 RVAs: 0x310e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140223a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-972159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e8812af1ff","size":5,"soft_fail":false,"target_va":5370952224,"va":5371924378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4255084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e86ced4000","size":5,"soft_fail":false,"target_va":5376179280,"va":5371924191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=10 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4255507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e813ef4000","size":5,"soft_fail":false,"target_va":5376179648,"va":5371924136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44426118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e886e3a502","size":5,"soft_fail":false,"target_va":5416350384,"va":5371924261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::block_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4308854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e876bf4100","size":5,"soft_fail":false,"target_va":5376233024,"va":5371924165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::ignore_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4308641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e8a1be4100","size":5,"soft_fail":false,"target_va":5376233040,"va":5371924394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::set_sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4308854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e876bf4100","size":5,"soft_fail":false,"target_va":5376233008,"va":5371924149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4308916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e8b4bf4100","size":5,"soft_fail":false,"target_va":5376233120,"va":5371924199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::with_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4308964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371924112,"raw_hex":"e8e4bf4100","size":5,"soft_fail":false,"target_va":5376233152,"va":5371924183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":84,"record_offset":18668,"parent_record_offset":18512,"ancestor_record_offset":18512,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":18892,"parent_record_offset":18860,"ancestor_record_offset":18512,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":18940,"parent_record_offset":18916,"ancestor_record_offset":18512,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":18916,"parent_record_offset":18892,"ancestor_record_offset":18512,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":18860,"parent_record_offset":18820,"ancestor_record_offset":18512,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":18788,"parent_record_offset":18752,"ancestor_record_offset":18512,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":18820,"parent_record_offset":18788,"ancestor_record_offset":18512,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":18724,"parent_record_offset":18692,"ancestor_record_offset":18512,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":18692,"parent_record_offset":18668,"ancestor_record_offset":18512,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":84,"record_offset":18752,"parent_record_offset":18692,"ancestor_record_offset":18512,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":18980,"parent_record_offset":18820,"ancestor_record_offset":18512,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BD7
// 首条位置: {"module":84,"inlinee":11223}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":84,"record_offset":18020,"parent_record_offset":17976,"ancestor_record_offset":17976,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_slider
// 记录形式: compiled_procedure; instances=1
// 源码行: 295
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":20356,"rva":6763184,"va":5375472304}
// 编译实例 RVAs: 0x6732b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140222390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4526753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e85fedbaff","size":5,"soft_fail":false,"target_va":5370946448,"va":5375473196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140222650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4525325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8f3f2baff","size":5,"soft_fail":false,"target_va":5370947152,"va":5375472472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140223000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4523000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e808fcbaff","size":5,"soft_fail":false,"target_va":5370949632,"va":5375472627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140223a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4520215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8e906bbff","size":5,"soft_fail":false,"target_va":5370952224,"va":5375472434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t706550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8f6c70a00","size":5,"soft_fail":false,"target_va":5376179280,"va":5375472725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=3
//     target_aliases=10 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t707252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8b4ca0a00","size":5,"soft_fail":false,"target_va":5376179648,"va":5375472391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140731e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t780960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8a0ea0b00","size":5,"soft_fail":false,"target_va":5376253472,"va":5375472507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7801172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e854097700","size":5,"soft_fail":false,"target_va":5383273968,"va":5375472791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40877415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e867bd6f02","size":5,"soft_fail":false,"target_va":5416350384,"va":5375472964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::block_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t760620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e82c9b0b00","size":5,"soft_fail":false,"target_va":5376233024,"va":5375472399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::force_interactive
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t760448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8809a0b00","size":5,"soft_fail":false,"target_va":5376233072,"va":5375472619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::ignore_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t760585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8099b0b00","size":5,"soft_fail":false,"target_va":5376233040,"va":5375472450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::prev_computed_layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t760350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e81e9a0b00","size":5,"soft_fail":false,"target_va":5376233088,"va":5375472733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::interaction
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t734738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e812360b00","size":5,"soft_fail":false,"target_va":5376207392,"va":5375472649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t736928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8a03e0b00","size":5,"soft_fail":false,"target_va":5376209536,"va":5375472603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::with_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t737341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e83d400b00","size":5,"soft_fail":false,"target_va":5376209840,"va":5375472494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::ComputedLayout::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t781322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e80aec0b00","size":5,"soft_fail":false,"target_va":5376254080,"va":5375472753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::align_center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t781037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8edea0b00","size":5,"soft_fail":false,"target_va":5376253568,"va":5375472526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t781251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e8c3eb0b00","size":5,"soft_fail":false,"target_va":5376253840,"va":5375472584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::xalign_center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t781181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375472304,"raw_hex":"e87deb0b00","size":5,"soft_fail":false,"target_va":5376253728,"va":5375472542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":246,"record_offset":20772,"parent_record_offset":20356,"ancestor_record_offset":20356,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":246,"record_offset":20496,"parent_record_offset":20356,"ancestor_record_offset":20356,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":20560,"parent_record_offset":20356,"ancestor_record_offset":20356,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":246,"record_offset":20620,"parent_record_offset":20356,"ancestor_record_offset":20356,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":246,"record_offset":20528,"parent_record_offset":20356,"ancestor_record_offset":20356,"inlinee":11244,"source_line":603,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":21000,"parent_record_offset":20968,"ancestor_record_offset":20356,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":21048,"parent_record_offset":21024,"ancestor_record_offset":20356,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":20356,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":20968,"parent_record_offset":20928,"ancestor_record_offset":20356,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":20896,"parent_record_offset":20860,"ancestor_record_offset":20356,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":20928,"parent_record_offset":20896,"ancestor_record_offset":20356,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":246,"record_offset":20832,"parent_record_offset":20796,"ancestor_record_offset":20356,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":246,"record_offset":20796,"parent_record_offset":20772,"ancestor_record_offset":20356,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":246,"record_offset":20860,"parent_record_offset":20796,"ancestor_record_offset":20356,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":246,"record_offset":20644,"parent_record_offset":20620,"ancestor_record_offset":20356,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":21088,"parent_record_offset":20928,"ancestor_record_offset":20356,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_fog::fill_slider::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":246,"record_offset":20588,"parent_record_offset":20560,"ancestor_record_offset":20356,"inlinee":17752,"source_line":336,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::fill_slider::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 336
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4558
// 首条位置: {"module":246,"inlinee":17752}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::impl$6::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 370
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27F3
// 首条位置: {"module":67,"inlinee":10227}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::impl$1::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27F1
// 首条位置: {"module":67,"inlinee":10225}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":67,"record_offset":9364,"parent_record_offset":9308,"ancestor_record_offset":9308,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_fog::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":67,"record_offset":9336,"parent_record_offset":9308,"ancestor_record_offset":9308,"inlinee":10227,"source_line":370,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":67,"record_offset":9412,"parent_record_offset":9388,"ancestor_record_offset":9308,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":67,"record_offset":9388,"parent_record_offset":9364,"ancestor_record_offset":9308,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":67,"record_offset":9448,"parent_record_offset":9412,"ancestor_record_offset":9308,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":67,"record_offset":9472,"parent_record_offset":9448,"ancestor_record_offset":9308,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":67,"record_offset":9504,"parent_record_offset":9412,"ancestor_record_offset":9308,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::impl$2::prepare_layout
// 记录形式: compiled_procedure; instances=1
// 源码行: 389
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":24600,"rva":6766352,"va":5375475472}
// 编译实例 RVAs: 0x673f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140726440
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t730393","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375475472,"raw_hex":"e919250b00","size":5,"soft_fail":false,"target_va":5376205888,"va":5375475490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::impl$2::process_input_action
// 记录形式: compiled_procedure; instances=1
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":24724,"rva":6766384,"va":5375475504}
// 编译实例 RVAs: 0x673f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::input::action_set::ActionSetAction::just_pressed
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2929028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475504,"raw_hex":"e884b12c00","size":5,"soft_fail":false,"target_va":5378404560,"va":5375475527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::ActionSetAction::pressed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2928978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475504,"raw_hex":"e852b12c00","size":5,"soft_fail":false,"target_va":5378404608,"va":5375475625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::impl$2::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 440
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":24856,"rva":6766560,"va":5375475680}
// 编译实例 RVAs: 0x673fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=21
//     evidence={"assembly":"\tcallq\t202732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475680,"raw_hex":"e8ec170300","size":5,"soft_fail":false,"target_va":5375678704,"va":5375475967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5950
//     owner=unresolved_namespace; count=6
//     target_aliases=4 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6090239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475680,"raw_hex":"e80112a3ff","size":5,"soft_fail":false,"target_va":5369387344,"va":5375477578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b8f4d0
//     owner=unresolved_namespace; count=6
//     target_aliases=20 candidates; complete list: gui_fog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5354608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475680,"raw_hex":"e870b45100","size":5,"soft_fail":false,"target_va":5380830416,"va":5375475803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::corner_radius
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6549080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475680,"raw_hex":"e858ee6300","size":5,"soft_fail":false,"target_va":5382024928,"va":5375475843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::encode_no_images
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6549240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475680,"raw_hex":"e8f8ee6300","size":5,"soft_fail":false,"target_va":5382025104,"va":5375475859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::fill_color
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6548923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475680,"raw_hex":"e8bbed6300","size":5,"soft_fail":false,"target_va":5382024752,"va":5375475824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::RenderContext::draw_rect
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t734047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375475680,"raw_hex":"e85f330b00","size":5,"soft_fail":false,"target_va":5376210112,"va":5375476060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":25776,"parent_record_offset":24856,"ancestor_record_offset":24856,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_fog::impl$2::render::closure$0
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_fog.rs
//     evidence={"module":246,"record_offset":24984,"parent_record_offset":24856,"ancestor_record_offset":24856,"inlinee":17768,"source_line":450,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":24856,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":25400,"parent_record_offset":25376,"ancestor_record_offset":24856,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":25424,"parent_record_offset":25400,"ancestor_record_offset":24856,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":25352,"parent_record_offset":25296,"ancestor_record_offset":24856,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":25324,"parent_record_offset":25296,"ancestor_record_offset":24856,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":25296,"parent_record_offset":25272,"ancestor_record_offset":24856,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":25272,"parent_record_offset":25248,"ancestor_record_offset":24856,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":25076,"parent_record_offset":25052,"ancestor_record_offset":24856,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":25608,"parent_record_offset":25580,"ancestor_record_offset":24856,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=81
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":25028,"parent_record_offset":24984,"ancestor_record_offset":24856,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_fog::impl$2::render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4568
// 首条位置: {"module":246,"inlinee":17768}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":25028,"parent_record_offset":24984,"ancestor_record_offset":24984,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":24984,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":25400,"parent_record_offset":25376,"ancestor_record_offset":24984,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":25424,"parent_record_offset":25400,"ancestor_record_offset":24984,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":25352,"parent_record_offset":25296,"ancestor_record_offset":24984,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":25324,"parent_record_offset":25296,"ancestor_record_offset":24984,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":25296,"parent_record_offset":25272,"ancestor_record_offset":24984,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":25272,"parent_record_offset":25248,"ancestor_record_offset":24984,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":25076,"parent_record_offset":25052,"ancestor_record_offset":24984,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":25608,"parent_record_offset":25580,"ancestor_record_offset":24984,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":25052,"parent_record_offset":25028,"ancestor_record_offset":24984,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
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
// 完整结构及原始类型记录: gui_fog.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::gui_fog::FogParam
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::gui_fog::FogSliderContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content_mut<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "toolbar::mei::core::NodeContext::content_mut", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::child_with_content<tiny_glade::gui::gui_fog::FogSliderContent,ref$<str$>,tiny_glade::gui::gui_fog::fill_slider::closure_env$0>", "constructor": "toolbar::mei::core::NodeContext::child_with_content", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent", "ref$<str$>", "tiny_glade::gui::gui_fog::fill_slider::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "toolbar::mei::core::NodeContext::content", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any_mut<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "toolbar::mei::core::impl$21::as_any_mut", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "toolbar::mei::core::impl$21::as_any", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::type_name<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "toolbar::mei::core::impl$21::type_name", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::pre_update<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "toolbar::mei::core::NodeContent::pre_update", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::on_input<tiny_glade::gui::gui_fog::FogSliderContent>", "constructor": "toolbar::mei::core::NodeContent::on_input", "arguments": ["tiny_glade::gui::gui_fog::FogSliderContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gui_fog.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gui_fog.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::FogParam::get
// 信息 ID: 0babc9b632d432313a086973
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_weather_slots::closure$2; module=84, inline_record=16084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::FogParam::range
// 信息 ID: 1e4e8e60f9c7707c7d6592c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::FogParam::set
// 信息 ID: 8294f4b80a5d685f7c32d5ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_weather_slots::closure$2; module=84, inline_record=16472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::FogParam::slider_to_value
// 信息 ID: d772b4906d95babb062537d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_weather_slots::closure$2; module=84, inline_record=16176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::FogParam::track_id
// 信息 ID: 58f5a866d1f6cb5e0c1efc82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_weather_slots::closure$2; module=84, inline_record=16056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::FogParam::value_to_slider
// 信息 ID: 3320f769732e5b8b1df6b108
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_weather_slots::closure$2; module=84, inline_record=16112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_precipitation_slots
// 信息 ID: 551e8fbf4098942022c55f19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_weather_slots; module=246, inline_record=21996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0
// 信息 ID: d7e25e954a8ca9fef215375f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_fog::fill_precipitation_slots", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_fog::fill_precipitation_slots::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_fog::fill_precipitation_slots::closure_env$0,ref$<str$> > @ 0x3189cc -> 0x310620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$0
// 信息 ID: bfbf71f0f42db1174c251302
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_fog::fill_precipitation_slots", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0; module=84, inline_record=17288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$1
// 信息 ID: a4206bf17f2fd670a230abf4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_fog::fill_precipitation_slots", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0 @ 0x310b20 -> 0x310e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0::closure$2
// 信息 ID: 6f05f3d70ea7da423a3ecf93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_fog::fill_precipitation_slots", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0; module=84, inline_record=17976; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0; module=84, inline_record=18424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_slider
// 信息 ID: 26110e3a1c8ee8fb5ee72c24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_weather_slots::closure$2 @ 0x310590 -> 0x6732b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0 @ 0x310c3e -> 0x6732b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_precipitation_slots::closure$0 @ 0x310e4a -> 0x6732b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_slider::closure$1
// 信息 ID: 990fae68f6ca5f52511ee799
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_fog::fill_slider", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::fill_slider; module=246, inline_record=20588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_weather_slots
// 信息 ID: 51e890c672fb02999322b8ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen @ 0x3fdd80 -> 0x673680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_weather_slots::closure$0
// 信息 ID: 149f33d1bb60fd851929ef08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_fog::fill_weather_slots", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$0,ref$<str$> >; module=84, inline_record=47172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_weather_slots::closure$1
// 信息 ID: 49daa05591227b67434245d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_fog::fill_weather_slots", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$1,ref$<str$> > @ 0x316ad3 -> 0x310330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::fill_weather_slots::closure$2
// 信息 ID: 16b4fc4645b953d435356202
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::gui::gui_fog::fill_weather_slots", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_fog::fill_weather_slots::closure_env$2,ref$<str$> > @ 0x317a1c -> 0x310460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::impl$1::create_node_content
// 信息 ID: da0955142b313f0cce11e234
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::child_with_content<tiny_glade::gui::gui_fog::FogSliderContent,ref$<str$>,tiny_glade::gui::gui_fog::fill_slider::closure_env$0>; module=67, inline_record=9308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::impl$2::prepare_layout
// 信息 ID: eac4cba4f7a65e54c6e0eb9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::impl$2::process_input_action
// 信息 ID: 7ec2484a4b294c49beffe7a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::impl$2::render
// 信息 ID: 8b9feb66cdc942db8d59956c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::impl$2::render::closure$0
// 信息 ID: 6c19476ed6c1cd50939b356f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_fog::impl$2::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::impl$2::render; module=246, inline_record=24984; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::impl$2::render; module=246, inline_record=26096; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::impl$2::render; module=246, inline_record=27208; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::impl$2::render; module=246, inline_record=27560; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::impl$2::render; module=246, inline_record=28088; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::impl$2::render; module=246, inline_record=28936; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::gui_fog::impl$2::render; module=246, inline_record=30052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_fog::impl$6::default
// 信息 ID: c403bf094c51183d0c800f58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::child_with_content<tiny_glade::gui::gui_fog::FogSliderContent,ref$<str$>,tiny_glade::gui::gui_fog::fill_slider::closure_env$0>; module=67, inline_record=9336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
