// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/ui/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1fcaa7b3cbc17c28164b736d359c41102d3ddf894e5f80a6f544a3dd8d87fec9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 67, 90, 131, 148, 213, 346, 347, 348, 356, 511, 558, 566, 1235, 1237, 1239, 1249, 1265, 1271, 1282, 1297, 2645, 2866, 2868, 2873, 2874, 2875, 2893, 2896, 2897, 2898, 2929, 2957, 2963, 3983, 4207, 4208, 4209, 4210, 4234, 4779
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use append_only_vec; // namespace candidate
//   package identities: [{"name":"append-only-vec","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8","version":"0.1.2"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::impl$2::eq
// 记录形式: inline_instance, inlinee_declaration; instances=456
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27E4
// 首条位置: {"module":67,"inlinee":10212}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::impl$4::hash
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x381C
// 首条位置: {"module":148,"inlinee":14364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":24196,"parent_record_offset":24172,"ancestor_record_offset":24172,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":148,"record_offset":24220,"parent_record_offset":24196,"ancestor_record_offset":24172,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":148,"record_offset":24296,"parent_record_offset":24272,"ancestor_record_offset":24172,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4779,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":612,"inlinee":6726,"source_line":256,"archive":"_pdb/C/modules/04779.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":4779,"record_offset":764,"parent_record_offset":732,"ancestor_record_offset":612,"inlinee":6731,"source_line":362,"archive":"_pdb/C/modules/04779.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4779,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":612,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/04779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4779,"record_offset":848,"parent_record_offset":764,"ancestor_record_offset":612,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/04779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":148,"record_offset":24244,"parent_record_offset":24220,"ancestor_record_offset":24172,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4779,"record_offset":820,"parent_record_offset":764,"ancestor_record_offset":612,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04779.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":148,"record_offset":24272,"parent_record_offset":24220,"ancestor_record_offset":24172,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/00148.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::impl$0::add_ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":101676,"rva":9708448,"va":5378417568}
// 编译实例 RVAs: 0x9423a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6226543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e86f025f00","size":5,"soft_fail":false,"target_va":5384644912,"va":5378418364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9212844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e8546c73ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5378418503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7819754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e816ae88ff","size":5,"soft_fail":false,"target_va":5370597904,"va":5378417653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t82294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e876410100","size":5,"soft_fail":false,"target_va":5378499936,"va":5378417637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14151ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12421681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e8318abd00","size":5,"soft_fail":false,"target_va":5390839440,"va":5378417754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e0410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15327303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e847e0e900","size":5,"soft_fail":false,"target_va":5393744912,"va":5378417604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141852170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15792753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e871faf000","size":5,"soft_fail":false,"target_va":5394211184,"va":5378418426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14185a1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15826182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e8067df100","size":5,"soft_fail":false,"target_va":5394244032,"va":5378417845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14185a700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15827555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e86382f100","size":5,"soft_fail":false,"target_va":5394245376,"va":5378417816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141865a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15873434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e89a35f200","size":5,"soft_fail":false,"target_va":5394291216,"va":5378417777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141867b10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15881303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e85754f200","size":5,"soft_fail":false,"target_va":5394299664,"va":5378418356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418745b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15933705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e80921f300","size":5,"soft_fail":false,"target_va":5394351536,"va":5378417826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38638738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e892944d02","size":5,"soft_fail":false,"target_va":5417057088,"va":5378418345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15694262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e8b679ef00","size":5,"soft_fail":false,"target_va":5394112352,"va":5378418085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-292107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e8f58afbff","size":5,"soft_fail":false,"target_va":5378125664,"va":5378417766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6308237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378417568,"raw_hex":"e88d416000","size":5,"soft_fail":false,"target_va":5384726688,"va":5378418446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":511,"record_offset":101984,"parent_record_offset":101676,"ancestor_record_offset":101676,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":511,"record_offset":101900,"parent_record_offset":101676,"ancestor_record_offset":101676,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":101828,"parent_record_offset":101676,"ancestor_record_offset":101676,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::popup_bounds::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/popup_bounds.rs
//     evidence={"module":511,"record_offset":101956,"parent_record_offset":101676,"ancestor_record_offset":101676,"inlinee":29422,"source_line":7,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::impl$73::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":511,"record_offset":101928,"parent_record_offset":101676,"ancestor_record_offset":101676,"inlinee":13695,"source_line":417,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":511,"record_offset":105404,"parent_record_offset":101676,"ancestor_record_offset":101676,"inlinee":29429,"source_line":25,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":511,"record_offset":105320,"parent_record_offset":101676,"ancestor_record_offset":101676,"inlinee":29424,"source_line":54,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":103380,"parent_record_offset":103356,"ancestor_record_offset":101676,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":103428,"parent_record_offset":103404,"ancestor_record_offset":101676,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":103404,"parent_record_offset":103380,"ancestor_record_offset":101676,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":103356,"parent_record_offset":103332,"ancestor_record_offset":101676,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":103332,"parent_record_offset":103292,"ancestor_record_offset":101676,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":103292,"parent_record_offset":103268,"ancestor_record_offset":101676,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":511,"record_offset":105368,"parent_record_offset":105344,"ancestor_record_offset":101676,"inlinee":29427,"source_line":177,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$3::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":511,"record_offset":105344,"parent_record_offset":105320,"ancestor_record_offset":101676,"inlinee":29426,"source_line":217,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":511,"record_offset":102008,"parent_record_offset":101984,"ancestor_record_offset":101676,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":511,"record_offset":102068,"parent_record_offset":102036,"ancestor_record_offset":101676,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":511,"record_offset":101868,"parent_record_offset":101828,"ancestor_record_offset":101676,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":511,"record_offset":103600,"parent_record_offset":103576,"ancestor_record_offset":101676,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":511,"record_offset":103576,"parent_record_offset":103552,"ancestor_record_offset":101676,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":511,"record_offset":103040,"parent_record_offset":103016,"ancestor_record_offset":101676,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":103088,"parent_record_offset":103064,"ancestor_record_offset":101676,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":103064,"parent_record_offset":103040,"ancestor_record_offset":101676,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":103268,"parent_record_offset":103244,"ancestor_record_offset":101676,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":511,"record_offset":102036,"parent_record_offset":102008,"ancestor_record_offset":101676,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":511,"record_offset":103016,"parent_record_offset":102992,"ancestor_record_offset":101676,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":511,"record_offset":102092,"parent_record_offset":102068,"ancestor_record_offset":101676,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":511,"record_offset":102740,"parent_record_offset":102716,"ancestor_record_offset":101676,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":511,"record_offset":102768,"parent_record_offset":102740,"ancestor_record_offset":101676,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":511,"record_offset":105976,"parent_record_offset":105948,"ancestor_record_offset":101676,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":511,"record_offset":105876,"parent_record_offset":105672,"ancestor_record_offset":101676,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":511,"record_offset":102308,"parent_record_offset":102284,"ancestor_record_offset":101676,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":511,"record_offset":102392,"parent_record_offset":102368,"ancestor_record_offset":101676,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":511,"record_offset":105840,"parent_record_offset":105816,"ancestor_record_offset":101676,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":511,"record_offset":106004,"parent_record_offset":105948,"ancestor_record_offset":101676,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":511,"record_offset":102540,"parent_record_offset":102512,"ancestor_record_offset":101676,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":511,"record_offset":102512,"parent_record_offset":102484,"ancestor_record_offset":101676,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":511,"record_offset":103180,"parent_record_offset":103116,"ancestor_record_offset":101676,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":102992,"parent_record_offset":102036,"ancestor_record_offset":101676,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":103204,"parent_record_offset":103180,"ancestor_record_offset":101676,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":103528,"parent_record_offset":102036,"ancestor_record_offset":101676,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":103244,"parent_record_offset":103064,"ancestor_record_offset":101676,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":105704,"parent_record_offset":105672,"ancestor_record_offset":101676,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":102332,"parent_record_offset":102308,"ancestor_record_offset":101676,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":511,"record_offset":105792,"parent_record_offset":105672,"ancestor_record_offset":101676,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":511,"record_offset":102656,"parent_record_offset":102632,"ancestor_record_offset":101676,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":105816,"parent_record_offset":105792,"ancestor_record_offset":101676,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":511,"record_offset":103116,"parent_record_offset":103088,"ancestor_record_offset":101676,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":103152,"parent_record_offset":103116,"ancestor_record_offset":101676,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":103472,"parent_record_offset":103292,"ancestor_record_offset":101676,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":511,"record_offset":102716,"parent_record_offset":102692,"ancestor_record_offset":101676,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":102452,"parent_record_offset":102424,"ancestor_record_offset":101676,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":102484,"parent_record_offset":102452,"ancestor_record_offset":101676,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":102424,"parent_record_offset":102204,"ancestor_record_offset":101676,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":102284,"parent_record_offset":102204,"ancestor_record_offset":101676,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":102844,"parent_record_offset":102204,"ancestor_record_offset":101676,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":102368,"parent_record_offset":102204,"ancestor_record_offset":101676,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":511,"record_offset":102128,"parent_record_offset":102092,"ancestor_record_offset":101676,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":511,"record_offset":102692,"parent_record_offset":102584,"ancestor_record_offset":101676,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":102632,"parent_record_offset":102608,"ancestor_record_offset":101676,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":102936,"parent_record_offset":102204,"ancestor_record_offset":101676,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":102608,"parent_record_offset":102584,"ancestor_record_offset":101676,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":102180,"parent_record_offset":102156,"ancestor_record_offset":101676,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":102156,"parent_record_offset":102128,"ancestor_record_offset":101676,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":102204,"parent_record_offset":102180,"ancestor_record_offset":101676,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":102256,"parent_record_offset":102204,"ancestor_record_offset":101676,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":102584,"parent_record_offset":102204,"ancestor_record_offset":101676,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":511,"record_offset":102816,"parent_record_offset":102204,"ancestor_record_offset":101676,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":511,"record_offset":105440,"parent_record_offset":105404,"ancestor_record_offset":101676,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":105496,"parent_record_offset":105468,"ancestor_record_offset":101676,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":511,"record_offset":105752,"parent_record_offset":105728,"ancestor_record_offset":101676,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":511,"record_offset":105612,"parent_record_offset":105584,"ancestor_record_offset":101676,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":105948,"parent_record_offset":105552,"ancestor_record_offset":101676,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":105468,"parent_record_offset":105440,"ancestor_record_offset":101676,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":511,"record_offset":105640,"parent_record_offset":105612,"ancestor_record_offset":101676,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":511,"record_offset":105672,"parent_record_offset":105640,"ancestor_record_offset":101676,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":511,"record_offset":105552,"parent_record_offset":105524,"ancestor_record_offset":101676,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":511,"record_offset":105524,"parent_record_offset":105496,"ancestor_record_offset":101676,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifier_config_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8860
// 首条位置: {"module":558,"inlinee":34912}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":50784,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00558.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ui::GlobalId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 24
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::systems::ui::GlobalId>", "constructor": "ref$", "arguments": ["country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<country_core::systems::ui::GlobalId>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option::and_then<country_core::systems::ui::GlobalId,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>,toolbar::mei::core::impl$17::perform_layout::closure_env$1>", "constructor": "core::option::Option::and_then", "arguments": ["country_core::systems::ui::GlobalId", "tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>", "toolbar::mei::core::impl$17::perform_layout::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::systems::ui::GlobalId>", "constructor": "core::option::Option", "arguments": ["country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::systems::ui::GlobalId> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::systems::ui::GlobalId>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,enum2$<toolbar::mei::core::IsRoot>,enum2$<core::option::Option<country_core::systems::ui::GlobalId> > >", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "enum2$<toolbar::mei::core::IsRoot>", "enum2$<core::option::Option<country_core::systems::ui::GlobalId> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<country_core::systems::ui::GlobalId,enum2$<toolbar::mei::core::IsRoot>,enum2$<core::option::Option<country_core::systems::ui::GlobalId> > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<country_core::systems::ui::GlobalId,enum2$<toolbar::mei::core::IsRoot>,enum2$<core::option::Option<country_core::systems::ui::GlobalId> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId>", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::iter_subtree<country_core::systems::ui::GlobalId,toolbar::mei::core::update_nav::closure_env$0>", "constructor": "toolbar::mei::core::iter_subtree", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::update_nav::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,u32>", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "u32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,u32> >", "constructor": "utils::calculate_hash", "arguments": ["tuple$<country_core::systems::ui::GlobalId,u32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType>", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "country_core::input::action_set::DigitalActionSetActionType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType> >", "constructor": "utils::calculate_hash", "arguments": ["tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$44::next<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "constructor": "hashbrown::map::impl$44::next", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::contains_key<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::ui::GlobalId>", "constructor": "hashbrown::map::HashMap::contains_key", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node", "std::hash::random::RandomState", "alloc::alloc::Global", "country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::systems::ui::GlobalId", "country_core::systems::ui::GlobalId", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::get_mut<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::ui::GlobalId>", "constructor": "hashbrown::map::HashMap::get_mut", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node", "std::hash::random::RandomState", "alloc::alloc::Global", "country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "country_core::systems::ui::GlobalId", "toolbar::mei::core::Node"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,toolbar::mei::core::Node> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,toolbar::mei::core::Node>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "country_core::systems::ui::GlobalId", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::systems::ui::GlobalId>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::slice::iter::Iter<country_core::systems::ui::GlobalId>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<country_core::systems::ui::GlobalId,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["country_core::systems::ui::GlobalId", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::ValuesMut<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "constructor": "std::collections::hash::map::ValuesMut", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref_mut$<toolbar::mei::core::Node>,std::collections::hash::map::ValuesMut<country_core::systems::ui::GlobalId,toolbar::mei::core::Node> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref_mut$<toolbar::mei::core::Node>", "std::collections::hash::map::ValuesMut<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "toolbar::mei::core::impl$17::perform_layout::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<toolbar::mei::layout::LayoutTemp,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["toolbar::mei::layout::LayoutTemp", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "toolbar::mei::core::impl$17::perform_layout::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >", "constructor": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::layout::LayoutTemp", "tuple$<>", "toolbar::mei::core::impl$17::perform_layout::closure_env$2", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<country_core::systems::ui::GlobalId>,country_core::systems::ui::GlobalId,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > > > >", "constructor": "core::iter::adapters::copied::impl$1::fold", "arguments": ["core::slice::iter::Iter<country_core::systems::ui::GlobalId>", "country_core::systems::ui::GlobalId", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$21::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 34; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::SteamdeckKeyboardRequested>,bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History> >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::SteamdeckKeyboardRequested>", "bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History> >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::message::message_writer::MessageWriter<feedback_reporter::ReportPlayerFeedbackCmd>", "bevy_ecs::message::message_reader::MessageReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::SteamdeckKeyboardRequested>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<feedback_reporter::ReportPlayerFeedbackCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [文件级接口候选，具体函数归属 unknown] core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] core::cell::Cell<tuple$<u64,u64> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] core::cell::RefCell<puffin::ThreadProfiler> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] core::cell::Cell<enum2$<core::option::Option<std::sync::mpmc::context::Context> > > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] fastrand::Rng (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<bevy_tasks::thread_executor::ThreadExecutor,alloc::alloc::Global> (*)()
// 候选参数类型: []
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::impl$0::add_ui
// 信息 ID: 3cd8a323836f5e3087e1ab5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41e239 -> 0x9423a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::impl$2::eq
// 信息 ID: 39e71eb10556e64d783997f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 419 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64>; module=67, inline_record=1612; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::content_mut<tiny_glade::gui::gui_fog::FogSliderContent>; module=67, inline_record=5844; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::content_mut<toolbar::mei::widgets::FrameContent>; module=67, inline_record=7292; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::child_with_content<tiny_glade::gui::gui_fog::FogSliderContent,ref$<str$>,tiny_glade::gui::gui_fog::fill_slider::closure_env$0>; module=67, inline_record=8920; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<tiny_glade::gui::wall_selection_tags::GiftTagContent>; module=67, inline_record=10900; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::ImageContent>; module=67, inline_record=12852; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::content<tiny_glade::gui::gui_fog::FogSliderContent>; module=67, inline_record=16788; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::content<tiny_glade::gui::gui_tod::TodCircleContent>; module=67, inline_record=18240; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::impl$4::hash
// 信息 ID: 360ffde96f7e1b9b25904d59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >; module=148, inline_record=24172; 不是独立调用边。
// [PDB 内联宿主] utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,u32> >; module=1237, inline_record=480; 不是独立调用边。
// [PDB 内联宿主] utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType> >; module=1237, inline_record=3084; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >; module=4779, inline_record=612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifier_config_hash
// 信息 ID: 85c39a044ee90e498ba338a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::signifiers::signifier_create::signifier_create; module=558, inline_record=50760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
