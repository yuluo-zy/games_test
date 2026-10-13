// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/menu/save_error.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6545fd170c210bc72b79c8a07dac44b0174a3c09c41c2d7ce80922c78d64adc4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: save_error.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1181, 1205, 1225, 2846, 3956, 4539
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2846,"record_offset":256,"rva":32247008,"va":5400956128}
// 编译实例 RVAs: 0x1ec0ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26280377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e847fe6efe","size":5,"soft_fail":false,"target_va":5374676336,"va":5400956708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26226948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8fcce6ffe","size":5,"soft_fail":false,"target_va":5374729632,"va":5400956575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25875749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8db2a75fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400957248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a439a0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21484407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8892cb8fe","size":5,"soft_fail":false,"target_va":5379471776,"va":5400956178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b92210
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20114194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8ee14cdfe","size":5,"soft_fail":false,"target_va":5380842000,"va":5400956189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f99dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15889500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8a48b0dff","size":5,"soft_fail":false,"target_va":5385067984,"va":5400957479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9e970
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15869802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e896d80dff","size":5,"soft_fail":false,"target_va":5385087344,"va":5400957141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fba110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15757288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e818900fff","size":5,"soft_fail":false,"target_va":5385199888,"va":5400957171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbb840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15751377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e82fa70fff","size":5,"soft_fail":false,"target_va":5385205824,"va":5400957196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbc2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15748697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8a7b10fff","size":5,"soft_fail":false,"target_va":5385208528,"va":5400957220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18116332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e81491ebfe","size":5,"soft_fail":false,"target_va":5382840208,"va":5400956535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-25752848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8f00a77fe","size":5,"soft_fail":false,"target_va":5375203792,"va":5400956635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t10584506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8ba81a100","size":5,"soft_fail":false,"target_va":5411541776,"va":5400957265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22548845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e893eea7fe","size":5,"soft_fail":false,"target_va":5378407392,"va":5400956232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20068620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8f4c6cdfe","size":5,"soft_fail":false,"target_va":5380887680,"va":5400956295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-18207787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e8d52beafe","size":5,"soft_fail":false,"target_va":5382748432,"va":5400956214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18208661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e86b28eafe","size":5,"soft_fail":false,"target_va":5382747616,"va":5400956272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25415679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400956128,"raw_hex":"e801307cfe","size":5,"soft_fail":false,"target_va":5375540928,"va":5400956602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2846,"record_offset":596,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2846,"record_offset":848,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2846,"record_offset":416,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2846,"record_offset":560,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2846,"record_offset":3576,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2846,"record_offset":532,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":2846,"record_offset":500,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2846,"record_offset":3548,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":908,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":10705,"source_line":445,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":2380,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":20067,"source_line":1426,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":1196,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":12682,"source_line":1296,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::save_error::ui::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/save_error.rs
//     evidence={"module":2846,"record_offset":3804,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":100472,"source_line":38,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2846,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2846,"record_offset":3728,"parent_record_offset":3672,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2846,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":256,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":1068,"parent_record_offset":1036,"ancestor_record_offset":256,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":3700,"parent_record_offset":3672,"ancestor_record_offset":256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":256,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":256,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2846,"record_offset":1036,"parent_record_offset":1000,"ancestor_record_offset":256,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2846,"record_offset":1000,"parent_record_offset":932,"ancestor_record_offset":256,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2846,"record_offset":876,"parent_record_offset":848,"ancestor_record_offset":256,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2846,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":256,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2846,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":256,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2846,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2846,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":256,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2846,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":256,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2846,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":256,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2846,"record_offset":2164,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2846,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":256,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2846,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":256,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2846,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":256,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2846,"record_offset":2340,"parent_record_offset":1268,"ancestor_record_offset":256,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2846,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2846,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":256,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2846,"record_offset":2424,"parent_record_offset":2380,"ancestor_record_offset":256,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2846,"record_offset":1156,"parent_record_offset":1000,"ancestor_record_offset":256,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":2846,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":256,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2846,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":256,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":1748,"parent_record_offset":1720,"ancestor_record_offset":256,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":1780,"parent_record_offset":1748,"ancestor_record_offset":256,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":1720,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":1580,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":2192,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":1664,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2846,"record_offset":1552,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2846,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":256,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":256,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":256,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":2048,"parent_record_offset":1880,"ancestor_record_offset":256,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":1392,"parent_record_offset":1360,"ancestor_record_offset":256,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1988,"parent_record_offset":1904,"ancestor_record_offset":256,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1932,"parent_record_offset":1904,"ancestor_record_offset":256,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":2284,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":256,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":256,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1420,"parent_record_offset":1360,"ancestor_record_offset":256,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":256,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1524,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1880,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2846,"record_offset":1304,"parent_record_offset":1268,"ancestor_record_offset":256,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2846,"record_offset":5344,"parent_record_offset":3804,"ancestor_record_offset":256,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":256,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":3892,"parent_record_offset":3804,"ancestor_record_offset":256,"inlinee":10705,"source_line":445,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":4180,"parent_record_offset":3804,"ancestor_record_offset":256,"inlinee":20067,"source_line":1426,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":1268,"parent_record_offset":1236,"ancestor_record_offset":256,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2846,"record_offset":3488,"parent_record_offset":2380,"ancestor_record_offset":256,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":972,"parent_record_offset":932,"ancestor_record_offset":256,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::save_error::ui::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/save_error.rs
//     evidence={"module":2846,"record_offset":1236,"parent_record_offset":1196,"ancestor_record_offset":256,"inlinee":61837,"source_line":87,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18878
// 首条位置: {"module":2846,"inlinee":100472}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2846,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":3804,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2846,"record_offset":5344,"parent_record_offset":3804,"ancestor_record_offset":3804,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":3892,"parent_record_offset":3804,"ancestor_record_offset":3804,"inlinee":10705,"source_line":445,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":4180,"parent_record_offset":3804,"ancestor_record_offset":3804,"inlinee":20067,"source_line":1426,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":4052,"parent_record_offset":4020,"ancestor_record_offset":3804,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":3804,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2846,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":3804,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2846,"record_offset":4020,"parent_record_offset":3984,"ancestor_record_offset":3804,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2846,"record_offset":3984,"parent_record_offset":3916,"ancestor_record_offset":3804,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2846,"record_offset":3860,"parent_record_offset":3832,"ancestor_record_offset":3804,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2846,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":3804,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2846,"record_offset":4548,"parent_record_offset":4524,"ancestor_record_offset":3804,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2846,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":3804,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2846,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":3804,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2846,"record_offset":5108,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2846,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":3804,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2846,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":3804,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2846,"record_offset":5312,"parent_record_offset":4180,"ancestor_record_offset":3804,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2846,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":3804,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2846,"record_offset":4220,"parent_record_offset":4180,"ancestor_record_offset":3804,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2846,"record_offset":4140,"parent_record_offset":3984,"ancestor_record_offset":3804,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2846,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":3804,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":4692,"parent_record_offset":4664,"ancestor_record_offset":3804,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":4724,"parent_record_offset":4692,"ancestor_record_offset":3804,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":4664,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":4524,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":5136,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":4608,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2846,"record_offset":4496,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2846,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":3804,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":3804,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":4276,"parent_record_offset":4248,"ancestor_record_offset":3804,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":4992,"parent_record_offset":4824,"ancestor_record_offset":3804,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":4336,"parent_record_offset":4304,"ancestor_record_offset":3804,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":4932,"parent_record_offset":4848,"ancestor_record_offset":3804,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":4876,"parent_record_offset":4848,"ancestor_record_offset":3804,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":5228,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":3804,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":3804,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":4364,"parent_record_offset":4304,"ancestor_record_offset":3804,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":3804,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":4468,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":4824,"parent_record_offset":4412,"ancestor_record_offset":3804,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2846,"record_offset":4248,"parent_record_offset":4180,"ancestor_record_offset":3804,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3804,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2846,"record_offset":5284,"parent_record_offset":4180,"ancestor_record_offset":3804,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":3956,"parent_record_offset":3916,"ancestor_record_offset":3804,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF17B
// 首条位置: {"module":1181,"inlinee":61819}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1181,"record_offset":115460,"parent_record_offset":113208,"ancestor_record_offset":113208,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":113252,"parent_record_offset":113208,"ancestor_record_offset":113208,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1181,"record_offset":114100,"parent_record_offset":114076,"ancestor_record_offset":113208,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":113608,"parent_record_offset":113584,"ancestor_record_offset":113208,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":113692,"parent_record_offset":113668,"ancestor_record_offset":113208,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":113632,"parent_record_offset":113608,"ancestor_record_offset":113208,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":114168,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":113840,"parent_record_offset":113812,"ancestor_record_offset":113208,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":113812,"parent_record_offset":113784,"ancestor_record_offset":113208,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":115428,"parent_record_offset":115404,"ancestor_record_offset":113208,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1181,"record_offset":113960,"parent_record_offset":113936,"ancestor_record_offset":113208,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":113284,"parent_record_offset":113252,"ancestor_record_offset":113208,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1181,"record_offset":114124,"parent_record_offset":114100,"ancestor_record_offset":113208,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":113752,"parent_record_offset":113724,"ancestor_record_offset":113208,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":113784,"parent_record_offset":113752,"ancestor_record_offset":113208,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":113724,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":113584,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":114196,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":113668,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1181,"record_offset":113556,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1181,"record_offset":114076,"parent_record_offset":114052,"ancestor_record_offset":113208,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":113368,"parent_record_offset":113340,"ancestor_record_offset":113208,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":113340,"parent_record_offset":113312,"ancestor_record_offset":113208,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":114052,"parent_record_offset":113884,"ancestor_record_offset":113208,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":113400,"parent_record_offset":113368,"ancestor_record_offset":113208,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":113992,"parent_record_offset":113908,"ancestor_record_offset":113208,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":113936,"parent_record_offset":113908,"ancestor_record_offset":113208,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":114288,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":113908,"parent_record_offset":113884,"ancestor_record_offset":113208,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":113452,"parent_record_offset":113428,"ancestor_record_offset":113208,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":113428,"parent_record_offset":113368,"ancestor_record_offset":113208,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":113476,"parent_record_offset":113452,"ancestor_record_offset":113208,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":113528,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":113884,"parent_record_offset":113476,"ancestor_record_offset":113208,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":113312,"parent_record_offset":113252,"ancestor_record_offset":113208,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF18D
// 首条位置: {"module":1181,"inlinee":61837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2846,"record_offset":1268,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2846,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":1236,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2846,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":1236,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2846,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1236,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2846,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1236,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2846,"record_offset":2164,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2846,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":1236,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2846,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":1236,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2846,"record_offset":2340,"parent_record_offset":1268,"ancestor_record_offset":1236,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2846,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":1236,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2846,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":1236,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":1748,"parent_record_offset":1720,"ancestor_record_offset":1236,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":1780,"parent_record_offset":1748,"ancestor_record_offset":1236,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2846,"record_offset":1720,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":1580,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":2192,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2846,"record_offset":1664,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2846,"record_offset":1552,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2846,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1236,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":1236,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":1236,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":2048,"parent_record_offset":1880,"ancestor_record_offset":1236,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2846,"record_offset":1392,"parent_record_offset":1360,"ancestor_record_offset":1236,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1988,"parent_record_offset":1904,"ancestor_record_offset":1236,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1932,"parent_record_offset":1904,"ancestor_record_offset":1236,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":2284,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":1236,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":1236,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1420,"parent_record_offset":1360,"ancestor_record_offset":1236,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":1236,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1524,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2846,"record_offset":1880,"parent_record_offset":1468,"ancestor_record_offset":1236,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2846,"record_offset":1304,"parent_record_offset":1268,"ancestor_record_offset":1236,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/02846.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1205,"record_offset":204500,"rva":16548096,"va":5385257216}
// 编译实例 RVAs: 0xfc8100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10528881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e88f575fff","size":5,"soft_fail":false,"target_va":5374728464,"va":5385257340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10527651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e85d5c5fff","size":5,"soft_fail":false,"target_va":5374729632,"va":5385257278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10527308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e8b45d5fff","size":5,"soft_fail":false,"target_va":5374730016,"va":5385257319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10176245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e80bb964ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385257744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2429892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e83cecdaff","size":5,"soft_fail":false,"target_va":5382827728,"va":5385257615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9d930
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-174189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e89357fdff","size":5,"soft_fail":false,"target_va":5385083184,"va":5385257368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac150
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-114933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e80b3ffeff","size":5,"soft_fail":false,"target_va":5385142608,"va":5385257536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-113899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e81543feff","size":5,"soft_fail":false,"target_va":5385143488,"va":5385257382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2417512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e8981cdbff","size":5,"soft_fail":false,"target_va":5382840208,"va":5385257715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4369924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e8fc51bdff","size":5,"soft_fail":false,"target_va":5380887680,"va":5385257599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2428770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e89ef0daff","size":5,"soft_fail":false,"target_va":5382828864,"va":5385257629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4420277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e84b8dbcff","size":5,"soft_fail":false,"target_va":5380837184,"va":5385257456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4421018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e8668abcff","size":5,"soft_fail":false,"target_va":5380836480,"va":5385257493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4420531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e84d8cbcff","size":5,"soft_fail":false,"target_va":5380837056,"va":5385257582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9707491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385257216,"raw_hex":"e81de06bff","size":5,"soft_fail":false,"target_va":5375549872,"va":5385257358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":205496,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":205580,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":205440,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":18557,"source_line":2278,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":205300,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":204952,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":19821,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":204864,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":19824,"source_line":276,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":205040,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":19822,"source_line":266,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":205216,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":19820,"source_line":254,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":205128,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":204660,"parent_record_offset":204500,"ancestor_record_offset":204500,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":205780,"parent_record_offset":205756,"ancestor_record_offset":204500,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":205756,"parent_record_offset":205700,"ancestor_record_offset":204500,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":205544,"parent_record_offset":205520,"ancestor_record_offset":204500,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":205728,"parent_record_offset":205700,"ancestor_record_offset":204500,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":205700,"parent_record_offset":205676,"ancestor_record_offset":204500,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":205676,"parent_record_offset":205652,"ancestor_record_offset":204500,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":205520,"parent_record_offset":205496,"ancestor_record_offset":204500,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":205604,"parent_record_offset":205580,"ancestor_record_offset":204500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":205464,"parent_record_offset":205440,"ancestor_record_offset":204500,"inlinee":6016,"source_line":906,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":205348,"parent_record_offset":205324,"ancestor_record_offset":204500,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":205396,"parent_record_offset":205372,"ancestor_record_offset":204500,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":205372,"parent_record_offset":205348,"ancestor_record_offset":204500,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":205324,"parent_record_offset":205300,"ancestor_record_offset":204500,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":204772,"parent_record_offset":204712,"ancestor_record_offset":204500,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":204712,"parent_record_offset":204660,"ancestor_record_offset":204500,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":204916,"parent_record_offset":204892,"ancestor_record_offset":204500,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":204684,"parent_record_offset":204660,"ancestor_record_offset":204500,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":204744,"parent_record_offset":204712,"ancestor_record_offset":204500,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":204796,"parent_record_offset":204772,"ancestor_record_offset":204500,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":204892,"parent_record_offset":204864,"ancestor_record_offset":204500,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":204828,"parent_record_offset":204712,"ancestor_record_offset":204500,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1205,"record_offset":207164,"rva":16549168,"va":5385258288}
// 编译实例 RVAs: 0xfc8530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10530128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e8b0525fff","size":5,"soft_fail":false,"target_va":5374728464,"va":5385258587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10528898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e87e575fff","size":5,"soft_fail":false,"target_va":5374729632,"va":5385258525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10528555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e8d5585fff","size":5,"soft_fail":false,"target_va":5374730016,"va":5385258566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10177347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e8bdb464ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385258846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9d930
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-175434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e8b652fdff","size":5,"soft_fail":false,"target_va":5385083184,"va":5385258613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-116177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e82f3afeff","size":5,"soft_fail":false,"target_va":5385142608,"va":5385258780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-115145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e8373efeff","size":5,"soft_fail":false,"target_va":5385143488,"va":5385258628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2418293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e88b19dbff","size":5,"soft_fail":false,"target_va":5382840208,"va":5385258496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4370657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e81f4fbdff","size":5,"soft_fail":false,"target_va":5380887680,"va":5385258332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4421522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e86e88bcff","size":5,"soft_fail":false,"target_va":5380837184,"va":5385258701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4422262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e88a85bcff","size":5,"soft_fail":false,"target_va":5380836480,"va":5385258737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4421771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e87587bcff","size":5,"soft_fail":false,"target_va":5380837056,"va":5385258822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9708735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258288,"raw_hex":"e841db6bff","size":5,"soft_fail":false,"target_va":5375549872,"va":5385258602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":207316,"parent_record_offset":207164,"ancestor_record_offset":207164,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":208040,"parent_record_offset":207164,"ancestor_record_offset":207164,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":207692,"parent_record_offset":207164,"ancestor_record_offset":207164,"inlinee":19821,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":207604,"parent_record_offset":207164,"ancestor_record_offset":207164,"inlinee":19824,"source_line":276,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":207780,"parent_record_offset":207164,"ancestor_record_offset":207164,"inlinee":19822,"source_line":266,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":207956,"parent_record_offset":207164,"ancestor_record_offset":207164,"inlinee":19820,"source_line":254,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":207868,"parent_record_offset":207164,"ancestor_record_offset":207164,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":207400,"parent_record_offset":207164,"ancestor_record_offset":207164,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":208216,"parent_record_offset":208192,"ancestor_record_offset":207164,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":208192,"parent_record_offset":208136,"ancestor_record_offset":207164,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":207364,"parent_record_offset":207340,"ancestor_record_offset":207164,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":208164,"parent_record_offset":208136,"ancestor_record_offset":207164,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":208136,"parent_record_offset":208112,"ancestor_record_offset":207164,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":208112,"parent_record_offset":208088,"ancestor_record_offset":207164,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":207340,"parent_record_offset":207316,"ancestor_record_offset":207164,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":208064,"parent_record_offset":208040,"ancestor_record_offset":207164,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":207512,"parent_record_offset":207452,"ancestor_record_offset":207164,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":207452,"parent_record_offset":207400,"ancestor_record_offset":207164,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":207656,"parent_record_offset":207632,"ancestor_record_offset":207164,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":207424,"parent_record_offset":207400,"ancestor_record_offset":207164,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":207484,"parent_record_offset":207452,"ancestor_record_offset":207164,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":207536,"parent_record_offset":207512,"ancestor_record_offset":207164,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":207632,"parent_record_offset":207604,"ancestor_record_offset":207164,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":207568,"parent_record_offset":207452,"ancestor_record_offset":207164,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1205,"record_offset":208500,"rva":16549808,"va":5385258928}
// 编译实例 RVAs: 0xfc87b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10531697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e88f4c5fff","size":5,"soft_fail":false,"target_va":5374728464,"va":5385260156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10530446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e872515fff","size":5,"soft_fail":false,"target_va":5374729632,"va":5385260073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10530115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8bd525fff","size":5,"soft_fail":false,"target_va":5374730016,"va":5385260126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10360609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8dfe861ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5385261340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10177703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e859b364ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385259202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140877160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7676088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e848df8aff","size":5,"soft_fail":false,"target_va":5377585504,"va":5385261587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2431274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8d6e6daff","size":5,"soft_fail":false,"target_va":5382827728,"va":5385258997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d8d6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2339519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8414ddcff","size":5,"soft_fail":false,"target_va":5382919872,"va":5385259386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9d930
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-177009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e88f4cfdff","size":5,"soft_fail":false,"target_va":5385083184,"va":5385260188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-117813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8cb33feff","size":5,"soft_fail":false,"target_va":5385142608,"va":5385260416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-116719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e81138feff","size":5,"soft_fail":false,"target_va":5385143488,"va":5385260202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fb6eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-72489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8d7e4feff","size":5,"soft_fail":false,"target_va":5385186992,"va":5385259476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fcf110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e867680000","size":5,"soft_fail":false,"target_va":5385285904,"va":5385259172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe41e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t111469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e86db30100","size":5,"soft_fail":false,"target_va":5385372128,"va":5385260654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e58d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15270487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e85702e900","size":5,"soft_fail":false,"target_va":5400530240,"va":5385259748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26026618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e87a228d01","size":5,"soft_fail":false,"target_va":5411285568,"va":5385258945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26588120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8d8b39501","size":5,"soft_fail":false,"target_va":5411847168,"va":5385259043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2419367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e85915dbff","size":5,"soft_fail":false,"target_va":5382840208,"va":5385259570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26069545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e829ca8d01","size":5,"soft_fail":false,"target_va":5411330256,"va":5385260706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2604049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8ef43d8ff","size":5,"soft_fail":false,"target_va":5382655024,"va":5385259068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26282103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e877089101","size":5,"soft_fail":false,"target_va":5411542880,"va":5385260772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15272998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8260ce900","size":5,"soft_fail":false,"target_va":5400533872,"va":5385260869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4728434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e88ed9b7ff","size":5,"soft_fail":false,"target_va":5380530560,"va":5385258989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5113052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e824fbb1ff","size":5,"soft_fail":false,"target_va":5380145968,"va":5385259015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4371582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8824bbdff","size":5,"soft_fail":false,"target_va":5380887680,"va":5385259257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4423127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e82982bcff","size":5,"soft_fail":false,"target_va":5380837184,"va":5385260306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4423883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8357fbcff","size":5,"soft_fail":false,"target_va":5380836480,"va":5385260358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4423423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e80181bcff","size":5,"soft_fail":false,"target_va":5380837056,"va":5385260474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9710307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e81dd56bff","size":5,"soft_fail":false,"target_va":5375549872,"va":5385260174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::local_time::system_time_to_local_time
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3370268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385258928,"raw_hex":"e8e492ccff","size":5,"soft_fail":false,"target_va":5381889424,"va":5385259687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":209984,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1205,"record_offset":209388,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$27::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1205,"record_offset":210768,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":41381,"source_line":2655,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1205,"record_offset":209568,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1205,"record_offset":210740,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1205,"record_offset":208980,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":209076,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":210584,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$6::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":208832,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":32591,"source_line":1768,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1205,"record_offset":210524,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":208692,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":1205,"record_offset":210556,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":12241,"source_line":1271,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":212024,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":19821,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":211936,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":19824,"source_line":276,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":212112,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":19822,"source_line":266,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":212288,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":19820,"source_line":254,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":212200,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":211732,"parent_record_offset":208500,"ancestor_record_offset":208500,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":209324,"parent_record_offset":209300,"ancestor_record_offset":208500,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":209300,"parent_record_offset":209244,"ancestor_record_offset":208500,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":210032,"parent_record_offset":210008,"ancestor_record_offset":208500,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":211140,"parent_record_offset":211100,"ancestor_record_offset":208500,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":210948,"parent_record_offset":210920,"ancestor_record_offset":208500,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":211044,"parent_record_offset":211016,"ancestor_record_offset":208500,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":209272,"parent_record_offset":209244,"ancestor_record_offset":208500,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":209244,"parent_record_offset":209220,"ancestor_record_offset":208500,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":211016,"parent_record_offset":210980,"ancestor_record_offset":208500,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":211188,"parent_record_offset":211164,"ancestor_record_offset":208500,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":211164,"parent_record_offset":211140,"ancestor_record_offset":208500,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":210980,"parent_record_offset":210948,"ancestor_record_offset":208500,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":209220,"parent_record_offset":209196,"ancestor_record_offset":208500,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1205,"record_offset":209424,"parent_record_offset":209388,"ancestor_record_offset":208500,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1205,"record_offset":209656,"parent_record_offset":209604,"ancestor_record_offset":208500,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1205,"record_offset":210796,"parent_record_offset":210768,"ancestor_record_offset":208500,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1205,"record_offset":209604,"parent_record_offset":209568,"ancestor_record_offset":208500,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":210880,"parent_record_offset":210852,"ancestor_record_offset":208500,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":211100,"parent_record_offset":210880,"ancestor_record_offset":208500,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":210824,"parent_record_offset":210796,"ancestor_record_offset":208500,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":211228,"parent_record_offset":210880,"ancestor_record_offset":208500,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":210920,"parent_record_offset":210880,"ancestor_record_offset":208500,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":209008,"parent_record_offset":208980,"ancestor_record_offset":208500,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":212624,"parent_record_offset":212600,"ancestor_record_offset":208500,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1205,"record_offset":210852,"parent_record_offset":210824,"ancestor_record_offset":208500,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1205,"record_offset":209684,"parent_record_offset":209604,"ancestor_record_offset":208500,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":209500,"parent_record_offset":209460,"ancestor_record_offset":208500,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1205,"record_offset":209036,"parent_record_offset":209008,"ancestor_record_offset":208500,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1205,"record_offset":213164,"parent_record_offset":213136,"ancestor_record_offset":208500,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":210008,"parent_record_offset":209984,"ancestor_record_offset":208500,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":208888,"parent_record_offset":208860,"ancestor_record_offset":208500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":209712,"parent_record_offset":209604,"ancestor_record_offset":208500,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":208860,"parent_record_offset":208832,"ancestor_record_offset":208500,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1205,"record_offset":209460,"parent_record_offset":209424,"ancestor_record_offset":208500,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":208740,"parent_record_offset":208716,"ancestor_record_offset":208500,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":208788,"parent_record_offset":208764,"ancestor_record_offset":208500,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":208764,"parent_record_offset":208740,"ancestor_record_offset":208500,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":208716,"parent_record_offset":208692,"ancestor_record_offset":208500,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":211844,"parent_record_offset":211784,"ancestor_record_offset":208500,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":211784,"parent_record_offset":211732,"ancestor_record_offset":208500,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":211988,"parent_record_offset":211964,"ancestor_record_offset":208500,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":211756,"parent_record_offset":211732,"ancestor_record_offset":208500,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":211816,"parent_record_offset":211784,"ancestor_record_offset":208500,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":211868,"parent_record_offset":211844,"ancestor_record_offset":208500,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":211964,"parent_record_offset":211936,"ancestor_record_offset":208500,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":211900,"parent_record_offset":211784,"ancestor_record_offset":208500,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$4::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF36D
// 首条位置: {"module":1225,"inlinee":62317}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ordering::reverse
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":15336,"inlinee":10270,"source_line":547,"archive":"_pdb/B/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] std::time::impl$32::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":4539,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":3936,"inlinee":75102,"source_line":242,"archive":"_pdb/B/modules/04539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::SystemTime::intervals
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":4539,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3936,"inlinee":28246,"source_line":84,"archive":"_pdb/B/modules/04539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$5::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":4539,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3936,"inlinee":16241,"source_line":124,"archive":"_pdb/B/modules/04539.jsonl.gz"}
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
// 完整结构及原始类型记录: save_error.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: save_error.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: save_error.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui
// 信息 ID: cb4956bc32c2cfada644d0ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 512a91430b0e044dadca5407: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >)> > @ 0xffa31e -> 0x1ec0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >)> > > @ 0x1e125d3 -> 0x1ec0ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$0
// 信息 ID: 7be0b9407761a50e24530058
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::save_error::ui; module=2846, inline_record=3804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$0::closure$0
// 信息 ID: 9de3819be0a9ebacc70941aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$0::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::save_error::ui::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::save_error::ui::closure$0::closure_env$0>; module=1181, inline_record=113208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$1
// 信息 ID: 3ec1c2583ddc73174cf0227b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::save_error::ui; module=2846, inline_record=1236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$2
// 信息 ID: a5a004745ef23f98c0bd685a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$2,ref$<str$> > @ 0xfba3ee -> 0xfc8100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$3
// 信息 ID: 724fc3a79c12a863aec7782c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$3,ref$<str$> > @ 0xfbbb04 -> 0xfc8530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$4
// 信息 ID: a9bec82282c5aa5d2e01e8c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$4,ref$<str$> > @ 0xfbc594 -> 0xfc87b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$4::closure$0
// 信息 ID: 3ebfa2fe894839879e6730ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 24 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=1225, inline_record=15336; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=1225, inline_record=15512; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=1225, inline_record=15628; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=1225, inline_record=15752; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=1225, inline_record=15924; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=1225, inline_record=16368; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=1225, inline_record=16512; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=1225, inline_record=16656; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
