// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/load_shaders.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b9fc12b9cb38952c38649e948e15ba1e6da18ed07fc207713ca5dad83e3fbd37
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_shaders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 500, 1650, 1682, 1822, 3272, 3300
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_tasks; // namespace candidate
//   package identities: [{"name":"bevy_tasks","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.13.1"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use shaderface; // namespace candidate
//   package identities: [{"name":"shaderface","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::impl$2::should_load
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":63888,"rva":9402720,"va":5378111840}
// 编译实例 RVAs: 0x8f7960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38945041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378111840,"raw_hex":"e811415202","size":5,"soft_fail":false,"target_va":5417057040,"va":5378111994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":64212,"parent_record_offset":63888,"ancestor_record_offset":63888,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_shaders::is_game_shader
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":500,"record_offset":64032,"parent_record_offset":63888,"ancestor_record_offset":63888,"inlinee":28024,"source_line":186,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":64116,"parent_record_offset":64092,"ancestor_record_offset":63888,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":64136,"parent_record_offset":64116,"ancestor_record_offset":63888,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":64160,"parent_record_offset":64136,"ancestor_record_offset":63888,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":64092,"parent_record_offset":64072,"ancestor_record_offset":63888,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":500,"record_offset":64052,"parent_record_offset":64032,"ancestor_record_offset":63888,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":64072,"parent_record_offset":64052,"ancestor_record_offset":63888,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::RenderPasses::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":6172,"rva":9382176,"va":5378091296}
// 编译实例 RVAs: 0x8f2920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409930a0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t655926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091296,"raw_hex":"e836020a00","size":5,"soft_fail":false,"target_va":5378748576,"va":5378092645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38529644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091296,"raw_hex":"e86cea4b02","size":5,"soft_fail":false,"target_va":5416622240,"va":5378092591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::format_properties
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5841347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091296,"raw_hex":"e8c3215900","size":5,"soft_fail":false,"target_va":5383932720,"va":5378091368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::render_pass::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_render_pass
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5612772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091296,"raw_hex":"e8e4a45500","size":5,"soft_fail":false,"target_va":5383704320,"va":5378091543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_depth_stencil_format
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs
//     evidence={"module":500,"record_offset":6316,"parent_record_offset":6172,"ancestor_record_offset":6172,"inlinee":27985,"source_line":21,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::render_pass::RenderPassDesc::new
//     owner=git_dependency_definition; count=13
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/render_pass.rs
//     evidence={"module":500,"record_offset":6496,"parent_record_offset":6172,"ancestor_record_offset":6172,"inlinee":27989,"source_line":63,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":500,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":6172,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$84::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":500,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":6172,"inlinee":12534,"source_line":4190,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$26::from_iter
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":500,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":6172,"inlinee":13948,"source_line":1145,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::FormatFeatureFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":500,"record_offset":6456,"parent_record_offset":6316,"ancestor_record_offset":6172,"inlinee":27987,"source_line":32,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":500,"record_offset":6524,"parent_record_offset":6496,"ancestor_record_offset":6172,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::physical_device
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":500,"record_offset":6372,"parent_record_offset":6316,"ancestor_record_offset":6172,"inlinee":17756,"source_line":710,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::is_game_shader
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D78
// 首条位置: {"module":500,"inlinee":28024}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":500,"record_offset":64052,"parent_record_offset":64032,"ancestor_record_offset":64032,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":64116,"parent_record_offset":64092,"ancestor_record_offset":64032,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":64136,"parent_record_offset":64116,"ancestor_record_offset":64032,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":64160,"parent_record_offset":64136,"ancestor_record_offset":64032,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":64092,"parent_record_offset":64072,"ancestor_record_offset":64032,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":64072,"parent_record_offset":64052,"ancestor_record_offset":64032,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::impl$3::should_load
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6DAB
// 首条位置: {"module":3272,"inlinee":28075}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::startup::load_shaders::is_game_shader
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":71592,"parent_record_offset":71564,"ancestor_record_offset":71564,"inlinee":28024,"source_line":186,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":71704,"parent_record_offset":71676,"ancestor_record_offset":71564,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":71728,"parent_record_offset":71704,"ancestor_record_offset":71564,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":71752,"parent_record_offset":71728,"ancestor_record_offset":71564,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":71676,"parent_record_offset":71648,"ancestor_record_offset":71564,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3272,"record_offset":71620,"parent_record_offset":71592,"ancestor_record_offset":71564,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3272,"record_offset":71648,"parent_record_offset":71620,"ancestor_record_offset":71564,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF7C
// 首条位置: {"module":3272,"inlinee":118652}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3272,"record_offset":33004,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3272,"record_offset":32272,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3272,"record_offset":63132,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] bevy_tasks::slice::ParallelSlice::par_chunk_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/slice.rs
//     evidence={"module":3272,"record_offset":32612,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":28302,"source_line":37,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":32444,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3272,"record_offset":31904,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3272,"record_offset":31848,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":30912,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":30952,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":11351,"source_line":1943,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":31012,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":3272,"record_offset":30856,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_shaders::preload_shaders::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":33152,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":118653,"source_line":302,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":3272,"record_offset":32332,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":17196,"source_line":370,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3272,"record_offset":32500,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3272,"record_offset":32360,"parent_record_offset":30648,"ancestor_record_offset":30648,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43288,"parent_record_offset":43264,"ancestor_record_offset":30648,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43312,"parent_record_offset":43288,"ancestor_record_offset":30648,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":31444,"parent_record_offset":31420,"ancestor_record_offset":30648,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":31468,"parent_record_offset":31444,"ancestor_record_offset":30648,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43336,"parent_record_offset":43312,"ancestor_record_offset":30648,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":31492,"parent_record_offset":31468,"ancestor_record_offset":30648,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43264,"parent_record_offset":43232,"ancestor_record_offset":30648,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":31420,"parent_record_offset":31356,"ancestor_record_offset":30648,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$0::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":3272,"record_offset":45920,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":6961,"source_line":77,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3272,"record_offset":43232,"parent_record_offset":43204,"ancestor_record_offset":30648,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3272,"record_offset":43204,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3272,"record_offset":59156,"parent_record_offset":59132,"ancestor_record_offset":30648,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3272,"record_offset":46264,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3272,"record_offset":46312,"parent_record_offset":46288,"ancestor_record_offset":30648,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":30648,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":31388,"parent_record_offset":31356,"ancestor_record_offset":30648,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":31356,"parent_record_offset":31328,"ancestor_record_offset":30648,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":35024,"parent_record_offset":35000,"ancestor_record_offset":30648,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":35000,"parent_record_offset":34976,"ancestor_record_offset":30648,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":31328,"parent_record_offset":31300,"ancestor_record_offset":30648,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$55::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3272,"record_offset":45972,"parent_record_offset":45948,"ancestor_record_offset":30648,"inlinee":22224,"source_line":3244,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3272,"record_offset":30880,"parent_record_offset":30856,"ancestor_record_offset":30648,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":39784,"parent_record_offset":39740,"ancestor_record_offset":30648,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":30648,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":34928,"parent_record_offset":34904,"ancestor_record_offset":30648,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":45860,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":37920,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":39708,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":39740,"parent_record_offset":39708,"ancestor_record_offset":30648,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":45436,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":31944,"parent_record_offset":31904,"ancestor_record_offset":30648,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":34364,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":31084,"parent_record_offset":31060,"ancestor_record_offset":30648,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":34904,"parent_record_offset":34848,"ancestor_record_offset":30648,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":3272,"record_offset":32024,"parent_record_offset":31984,"ancestor_record_offset":30648,"inlinee":10798,"source_line":249,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":3272,"record_offset":31984,"parent_record_offset":31944,"ancestor_record_offset":30648,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$2::collect_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":3272,"record_offset":32084,"parent_record_offset":32024,"ancestor_record_offset":30648,"inlinee":10800,"source_line":371,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3272,"record_offset":62072,"parent_record_offset":62036,"ancestor_record_offset":30648,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3272,"record_offset":62036,"parent_record_offset":62004,"ancestor_record_offset":30648,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3272,"record_offset":62320,"parent_record_offset":62296,"ancestor_record_offset":30648,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3272,"record_offset":62156,"parent_record_offset":62128,"ancestor_record_offset":30648,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3272,"record_offset":32300,"parent_record_offset":32272,"ancestor_record_offset":30648,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3272,"record_offset":63160,"parent_record_offset":63132,"ancestor_record_offset":30648,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3272,"record_offset":63252,"parent_record_offset":63224,"ancestor_record_offset":30648,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3272,"record_offset":63224,"parent_record_offset":63132,"ancestor_record_offset":30648,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3272,"record_offset":63280,"parent_record_offset":63252,"ancestor_record_offset":30648,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task_pool::TaskPool::scope
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task_pool.rs
//     evidence={"module":3272,"record_offset":32648,"parent_record_offset":32612,"ancestor_record_offset":30648,"inlinee":10713,"source_line":310,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task_pool::impl$2::scope::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task_pool.rs
//     evidence={"module":3272,"record_offset":32772,"parent_record_offset":32716,"ancestor_record_offset":30648,"inlinee":6586,"source_line":315,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3272,"record_offset":49056,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":34616,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=251
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":35240,"parent_record_offset":35216,"ancestor_record_offset":30648,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3272,"record_offset":45948,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":3272,"record_offset":46348,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3272,"record_offset":31872,"parent_record_offset":31848,"ancestor_record_offset":30648,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3272,"record_offset":32112,"parent_record_offset":32084,"ancestor_record_offset":30648,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3272,"record_offset":62128,"parent_record_offset":62100,"ancestor_record_offset":30648,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3272,"record_offset":62100,"parent_record_offset":62072,"ancestor_record_offset":30648,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3272,"record_offset":30976,"parent_record_offset":30952,"ancestor_record_offset":30648,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3272,"record_offset":31680,"parent_record_offset":31652,"ancestor_record_offset":30648,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":3272,"record_offset":48892,"parent_record_offset":48864,"ancestor_record_offset":30648,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":51592,"parent_record_offset":51520,"ancestor_record_offset":30648,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":51616,"parent_record_offset":51592,"ancestor_record_offset":30648,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":32468,"parent_record_offset":32444,"ancestor_record_offset":30648,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":46288,"parent_record_offset":46264,"ancestor_record_offset":30648,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":50548,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":50256,"parent_record_offset":49900,"ancestor_record_offset":30648,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3272,"record_offset":32744,"parent_record_offset":32716,"ancestor_record_offset":30648,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3272,"record_offset":32164,"parent_record_offset":32140,"ancestor_record_offset":30648,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=146
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":31036,"parent_record_offset":31012,"ancestor_record_offset":30648,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":34436,"parent_record_offset":34412,"ancestor_record_offset":30648,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":32140,"parent_record_offset":32084,"ancestor_record_offset":30648,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":48864,"parent_record_offset":48840,"ancestor_record_offset":30648,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":48920,"parent_record_offset":48864,"ancestor_record_offset":30648,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":48944,"parent_record_offset":48920,"ancestor_record_offset":30648,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":33064,"parent_record_offset":33004,"ancestor_record_offset":30648,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":48816,"parent_record_offset":48792,"ancestor_record_offset":30648,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":62184,"parent_record_offset":62156,"ancestor_record_offset":30648,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":33092,"parent_record_offset":33004,"ancestor_record_offset":30648,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":48840,"parent_record_offset":48816,"ancestor_record_offset":30648,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":33036,"parent_record_offset":33004,"ancestor_record_offset":30648,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":33116,"parent_record_offset":33092,"ancestor_record_offset":30648,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":39904,"parent_record_offset":39740,"ancestor_record_offset":30648,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3272,"record_offset":35688,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=251
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":35264,"parent_record_offset":35240,"ancestor_record_offset":30648,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=251
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":35288,"parent_record_offset":35264,"ancestor_record_offset":30648,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":45728,"parent_record_offset":45700,"ancestor_record_offset":30648,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":34812,"parent_record_offset":34784,"ancestor_record_offset":30648,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":34388,"parent_record_offset":34364,"ancestor_record_offset":30648,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":35216,"parent_record_offset":35192,"ancestor_record_offset":30648,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3272,"record_offset":45464,"parent_record_offset":45436,"ancestor_record_offset":30648,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3272,"record_offset":45492,"parent_record_offset":45464,"ancestor_record_offset":30648,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3272,"record_offset":34412,"parent_record_offset":34388,"ancestor_record_offset":30648,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3272,"record_offset":45532,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3272,"record_offset":34476,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3272,"record_offset":45672,"parent_record_offset":45636,"ancestor_record_offset":30648,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3272,"record_offset":35168,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3272,"record_offset":35192,"parent_record_offset":35168,"ancestor_record_offset":30648,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3272,"record_offset":45700,"parent_record_offset":45672,"ancestor_record_offset":30648,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=246
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3272,"record_offset":35336,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":48992,"parent_record_offset":48764,"ancestor_record_offset":30648,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":36124,"parent_record_offset":36000,"ancestor_record_offset":30648,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":35800,"parent_record_offset":35760,"ancestor_record_offset":30648,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":32384,"parent_record_offset":32360,"ancestor_record_offset":30648,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":49016,"parent_record_offset":48992,"ancestor_record_offset":30648,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":36148,"parent_record_offset":36124,"ancestor_record_offset":30648,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":35824,"parent_record_offset":35800,"ancestor_record_offset":30648,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":32408,"parent_record_offset":32384,"ancestor_record_offset":30648,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::shader_library::AssetShaderLibrary::register_world_raster_pipelines
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":3272,"record_offset":51520,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":32913,"source_line":103,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":3272,"record_offset":46236,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::impl$3::should_load
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":71564,"parent_record_offset":70132,"ancestor_record_offset":67672,"inlinee":28075,"source_line":217,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::is_game_shader
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":71592,"parent_record_offset":71564,"ancestor_record_offset":67672,"inlinee":28024,"source_line":186,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":45636,"parent_record_offset":45532,"ancestor_record_offset":30648,"inlinee":118663,"source_line":740,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$5
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":49080,"parent_record_offset":49056,"ancestor_record_offset":30648,"inlinee":118681,"source_line":908,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":49460,"parent_record_offset":49432,"ancestor_record_offset":30648,"inlinee":118682,"source_line":916,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$7
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":49900,"parent_record_offset":49876,"ancestor_record_offset":30648,"inlinee":118684,"source_line":930,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":3272,"record_offset":35728,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$28::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":3272,"record_offset":35976,"parent_record_offset":35952,"ancestor_record_offset":30648,"inlinee":12297,"source_line":1829,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3272,"record_offset":32548,"parent_record_offset":32524,"ancestor_record_offset":30648,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3272,"record_offset":34672,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3272,"record_offset":32524,"parent_record_offset":32500,"ancestor_record_offset":30648,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3272,"record_offset":32572,"parent_record_offset":32548,"ancestor_record_offset":30648,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3272,"record_offset":34532,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"modular-bitfield","version":"0.13.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.13.1/src/private/proc.rs
//     evidence={"module":3272,"record_offset":47656,"parent_record_offset":47632,"ancestor_record_offset":30648,"inlinee":118666,"source_line":67,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3272,"record_offset":35856,"parent_record_offset":35760,"ancestor_record_offset":30648,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3272,"record_offset":36036,"parent_record_offset":36000,"ancestor_record_offset":30648,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3272,"record_offset":35760,"parent_record_offset":35728,"ancestor_record_offset":30648,"inlinee":6092,"source_line":67,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3272,"record_offset":36000,"parent_record_offset":35976,"ancestor_record_offset":30648,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3272,"record_offset":35884,"parent_record_offset":35856,"ancestor_record_offset":30648,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3272,"record_offset":36064,"parent_record_offset":36036,"ancestor_record_offset":30648,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$12::clone
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":48740,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":13123,"source_line":78,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":48792,"parent_record_offset":48764,"ancestor_record_offset":30648,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":48764,"parent_record_offset":48740,"ancestor_record_offset":30648,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47864,"parent_record_offset":47840,"ancestor_record_offset":30648,"inlinee":118672,"source_line":811,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47888,"parent_record_offset":47864,"ancestor_record_offset":30648,"inlinee":118673,"source_line":811,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47608,"parent_record_offset":47584,"ancestor_record_offset":30648,"inlinee":118664,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47632,"parent_record_offset":47608,"ancestor_record_offset":30648,"inlinee":118665,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47976,"parent_record_offset":47952,"ancestor_record_offset":30648,"inlinee":118675,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48000,"parent_record_offset":47976,"ancestor_record_offset":30648,"inlinee":118676,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48064,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":118677,"source_line":798,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48088,"parent_record_offset":48064,"ancestor_record_offset":30648,"inlinee":118678,"source_line":798,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48148,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":118679,"source_line":807,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48172,"parent_record_offset":48148,"ancestor_record_offset":30648,"inlinee":118680,"source_line":807,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_topology
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47724,"parent_record_offset":47700,"ancestor_record_offset":30648,"inlinee":118668,"source_line":794,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47748,"parent_record_offset":47724,"ancestor_record_offset":30648,"inlinee":118669,"source_line":794,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_blend_mode
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47840,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":118671,"source_line":811,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_cull_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47584,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":42285,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_depth_clamp_enable
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":49840,"parent_record_offset":49460,"ancestor_record_offset":30648,"inlinee":118683,"source_line":799,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_depth_compare_mode
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47952,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":118674,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_stencil_pass_op
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47812,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":118670,"source_line":807,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_topology
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47700,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":118667,"source_line":794,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::binding_set::BindingSetDesc::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3272,"record_offset":45892,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":100461,"source_line":141,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::binding_set::impl$10::default
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3272,"record_offset":39676,"parent_record_offset":39652,"ancestor_record_offset":30648,"inlinee":12184,"source_line":5,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::binding_set::impl$5::into_binding_set_desc
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3272,"record_offset":39652,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":12198,"source_line":58,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::PrecompiledShaderEntry::is_compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3272,"record_offset":34784,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":118655,"source_line":129,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$1::is_compute::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3272,"record_offset":34848,"parent_record_offset":34812,"ancestor_record_offset":30648,"inlinee":118657,"source_line":130,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$1::is_compute::closure$0::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3272,"record_offset":35096,"parent_record_offset":35072,"ancestor_record_offset":30648,"inlinee":118659,"source_line":134,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$7::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3272,"record_offset":35120,"parent_record_offset":35096,"ancestor_record_offset":30648,"inlinee":118661,"source_line":12,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3272,"record_offset":51564,"parent_record_offset":51520,"ancestor_record_offset":30648,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3272,"record_offset":32716,"parent_record_offset":32680,"ancestor_record_offset":30648,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3272,"record_offset":32680,"parent_record_offset":32648,"ancestor_record_offset":30648,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3272,"record_offset":46404,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3272,"record_offset":46428,"parent_record_offset":46404,"ancestor_record_offset":30648,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] ustr::ustr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3272,"record_offset":46376,"parent_record_offset":33152,"ancestor_record_offset":30648,"inlinee":27983,"source_line":469,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F02
// 首条位置: {"module":1650,"inlinee":81666}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":39064,"parent_record_offset":39044,"ancestor_record_offset":39044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":39244,"parent_record_offset":39220,"ancestor_record_offset":39044,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":39268,"parent_record_offset":39244,"ancestor_record_offset":39044,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":39292,"parent_record_offset":39268,"ancestor_record_offset":39044,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":39220,"parent_record_offset":39164,"ancestor_record_offset":39044,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":39192,"parent_record_offset":39164,"ancestor_record_offset":39044,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":39164,"parent_record_offset":39136,"ancestor_record_offset":39044,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":39136,"parent_record_offset":39112,"ancestor_record_offset":39044,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":39088,"parent_record_offset":39064,"ancestor_record_offset":39044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF7D
// 首条位置: {"module":3272,"inlinee":118653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::impl$0::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":3272,"record_offset":45920,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":6961,"source_line":77,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3272,"record_offset":43204,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3272,"record_offset":46264,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":45860,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":37920,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":39708,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":45436,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":34364,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3272,"record_offset":49056,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":34616,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3272,"record_offset":45948,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":3272,"record_offset":46348,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":50548,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":35952,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3272,"record_offset":35688,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3272,"record_offset":45532,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3272,"record_offset":34476,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3272,"record_offset":35168,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=492
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3272,"record_offset":35336,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::shader_library::AssetShaderLibrary::register_world_raster_pipelines
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":3272,"record_offset":51520,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":32913,"source_line":103,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":3272,"record_offset":46236,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_shaders::impl$3::should_load
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":71564,"parent_record_offset":70132,"ancestor_record_offset":70132,"inlinee":28075,"source_line":217,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":3272,"record_offset":35728,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3272,"record_offset":34672,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3272,"record_offset":34532,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::render_pass::impl$12::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":48740,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":13123,"source_line":78,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48400,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":118672,"source_line":811,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48064,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":118677,"source_line":798,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48148,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":118679,"source_line":807,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_blend_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47840,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":118671,"source_line":811,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_cull_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47584,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":42285,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_depth_compare_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47952,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":118674,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_stencil_pass_op
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47812,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":118670,"source_line":807,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_topology
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47700,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":118667,"source_line":794,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] shaderface::binding_set::BindingSetDesc::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3272,"record_offset":45892,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":100461,"source_line":141,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] shaderface::binding_set::impl$5::into_binding_set_desc
//     owner=git_dependency_definition; count=20
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3272,"record_offset":39652,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":12198,"source_line":58,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] shaderface::shader::PrecompiledShaderEntry::is_compute
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3272,"record_offset":34784,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":118655,"source_line":129,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3272,"record_offset":46404,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3272,"record_offset":46376,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":27983,"source_line":469,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43288,"parent_record_offset":43264,"ancestor_record_offset":33152,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43312,"parent_record_offset":43288,"ancestor_record_offset":33152,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43076,"parent_record_offset":43044,"ancestor_record_offset":33152,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43108,"parent_record_offset":43076,"ancestor_record_offset":33152,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43336,"parent_record_offset":43312,"ancestor_record_offset":33152,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43144,"parent_record_offset":43108,"ancestor_record_offset":33152,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43264,"parent_record_offset":43232,"ancestor_record_offset":33152,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":43044,"parent_record_offset":42980,"ancestor_record_offset":33152,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3272,"record_offset":43232,"parent_record_offset":43204,"ancestor_record_offset":33152,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3272,"record_offset":59156,"parent_record_offset":59132,"ancestor_record_offset":33152,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3272,"record_offset":46312,"parent_record_offset":46288,"ancestor_record_offset":33152,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":33152,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":43016,"parent_record_offset":42980,"ancestor_record_offset":33152,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":42980,"parent_record_offset":42948,"ancestor_record_offset":33152,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":35024,"parent_record_offset":35000,"ancestor_record_offset":33152,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":35000,"parent_record_offset":34976,"ancestor_record_offset":33152,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":42948,"parent_record_offset":42920,"ancestor_record_offset":33152,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$55::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3272,"record_offset":45972,"parent_record_offset":45948,"ancestor_record_offset":33152,"inlinee":22224,"source_line":3244,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":39784,"parent_record_offset":39740,"ancestor_record_offset":33152,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":33152,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":34928,"parent_record_offset":34904,"ancestor_record_offset":33152,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":39740,"parent_record_offset":39708,"ancestor_record_offset":33152,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":50632,"parent_record_offset":50608,"ancestor_record_offset":33152,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":34904,"parent_record_offset":34848,"ancestor_record_offset":33152,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=502
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":35240,"parent_record_offset":35216,"ancestor_record_offset":33152,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3272,"record_offset":50860,"parent_record_offset":50824,"ancestor_record_offset":33152,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":3272,"record_offset":48892,"parent_record_offset":48864,"ancestor_record_offset":33152,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":51592,"parent_record_offset":51520,"ancestor_record_offset":33152,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":51616,"parent_record_offset":51592,"ancestor_record_offset":33152,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":34640,"parent_record_offset":34616,"ancestor_record_offset":33152,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":46288,"parent_record_offset":46264,"ancestor_record_offset":33152,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":50256,"parent_record_offset":49900,"ancestor_record_offset":33152,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=94
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":42892,"parent_record_offset":42864,"ancestor_record_offset":33152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":34436,"parent_record_offset":34412,"ancestor_record_offset":33152,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":48864,"parent_record_offset":48840,"ancestor_record_offset":33152,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":48920,"parent_record_offset":48864,"ancestor_record_offset":33152,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":48944,"parent_record_offset":48920,"ancestor_record_offset":33152,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":71532,"parent_record_offset":71476,"ancestor_record_offset":70132,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":48816,"parent_record_offset":48792,"ancestor_record_offset":33152,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":48840,"parent_record_offset":48816,"ancestor_record_offset":33152,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":34500,"parent_record_offset":34476,"ancestor_record_offset":33152,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":39904,"parent_record_offset":39740,"ancestor_record_offset":33152,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=502
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":35264,"parent_record_offset":35240,"ancestor_record_offset":33152,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=502
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":35288,"parent_record_offset":35264,"ancestor_record_offset":33152,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":45728,"parent_record_offset":45700,"ancestor_record_offset":33152,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":34812,"parent_record_offset":34784,"ancestor_record_offset":33152,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":34388,"parent_record_offset":34364,"ancestor_record_offset":33152,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":35216,"parent_record_offset":35192,"ancestor_record_offset":33152,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3272,"record_offset":45464,"parent_record_offset":45436,"ancestor_record_offset":33152,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3272,"record_offset":45492,"parent_record_offset":45464,"ancestor_record_offset":33152,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3272,"record_offset":34412,"parent_record_offset":34388,"ancestor_record_offset":33152,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3272,"record_offset":45572,"parent_record_offset":45532,"ancestor_record_offset":33152,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3272,"record_offset":45672,"parent_record_offset":45636,"ancestor_record_offset":33152,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3272,"record_offset":71620,"parent_record_offset":71592,"ancestor_record_offset":70132,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3272,"record_offset":35192,"parent_record_offset":35168,"ancestor_record_offset":33152,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3272,"record_offset":45700,"parent_record_offset":45672,"ancestor_record_offset":33152,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":48992,"parent_record_offset":48764,"ancestor_record_offset":33152,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":36124,"parent_record_offset":36000,"ancestor_record_offset":33152,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":35800,"parent_record_offset":35760,"ancestor_record_offset":33152,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":34556,"parent_record_offset":34532,"ancestor_record_offset":33152,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":49016,"parent_record_offset":48992,"ancestor_record_offset":33152,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":36148,"parent_record_offset":36124,"ancestor_record_offset":33152,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":35824,"parent_record_offset":35800,"ancestor_record_offset":33152,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":34580,"parent_record_offset":34556,"ancestor_record_offset":33152,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::is_game_shader
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":71592,"parent_record_offset":71564,"ancestor_record_offset":70132,"inlinee":28024,"source_line":186,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":45636,"parent_record_offset":45532,"ancestor_record_offset":33152,"inlinee":118663,"source_line":740,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$5
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":49080,"parent_record_offset":49056,"ancestor_record_offset":33152,"inlinee":118681,"source_line":908,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$6
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":49460,"parent_record_offset":49432,"ancestor_record_offset":33152,"inlinee":118682,"source_line":916,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$7
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":49900,"parent_record_offset":49876,"ancestor_record_offset":33152,"inlinee":118684,"source_line":930,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$28::drop
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":3272,"record_offset":35976,"parent_record_offset":35952,"ancestor_record_offset":33152,"inlinee":12297,"source_line":1829,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3272,"record_offset":34720,"parent_record_offset":34696,"ancestor_record_offset":33152,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3272,"record_offset":34696,"parent_record_offset":34672,"ancestor_record_offset":33152,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3272,"record_offset":34744,"parent_record_offset":34720,"ancestor_record_offset":33152,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=56
//     packages=[{"name":"modular-bitfield","version":"0.13.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.13.1/src/private/proc.rs
//     evidence={"module":3272,"record_offset":47656,"parent_record_offset":47632,"ancestor_record_offset":33152,"inlinee":118666,"source_line":67,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3272,"record_offset":35856,"parent_record_offset":35760,"ancestor_record_offset":33152,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3272,"record_offset":36036,"parent_record_offset":36000,"ancestor_record_offset":33152,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3272,"record_offset":35760,"parent_record_offset":35728,"ancestor_record_offset":33152,"inlinee":6092,"source_line":67,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3272,"record_offset":36000,"parent_record_offset":35976,"ancestor_record_offset":33152,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3272,"record_offset":35884,"parent_record_offset":35856,"ancestor_record_offset":33152,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3272,"record_offset":36064,"parent_record_offset":36036,"ancestor_record_offset":33152,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$12::clone
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49108,"parent_record_offset":49080,"ancestor_record_offset":33152,"inlinee":13123,"source_line":78,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":48792,"parent_record_offset":48764,"ancestor_record_offset":33152,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":48764,"parent_record_offset":48740,"ancestor_record_offset":33152,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47864,"parent_record_offset":47840,"ancestor_record_offset":33152,"inlinee":118672,"source_line":811,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47888,"parent_record_offset":47864,"ancestor_record_offset":33152,"inlinee":118673,"source_line":811,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47608,"parent_record_offset":47584,"ancestor_record_offset":33152,"inlinee":118664,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47632,"parent_record_offset":47608,"ancestor_record_offset":33152,"inlinee":118665,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47976,"parent_record_offset":47952,"ancestor_record_offset":33152,"inlinee":118675,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48000,"parent_record_offset":47976,"ancestor_record_offset":33152,"inlinee":118676,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48088,"parent_record_offset":48064,"ancestor_record_offset":33152,"inlinee":118678,"source_line":798,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":48172,"parent_record_offset":48148,"ancestor_record_offset":33152,"inlinee":118680,"source_line":807,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_topology
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47724,"parent_record_offset":47700,"ancestor_record_offset":33152,"inlinee":118668,"source_line":794,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":47748,"parent_record_offset":47724,"ancestor_record_offset":33152,"inlinee":118669,"source_line":794,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_cull_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":50284,"parent_record_offset":49900,"ancestor_record_offset":33152,"inlinee":42285,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_depth_clamp_enable
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":49840,"parent_record_offset":49460,"ancestor_record_offset":33152,"inlinee":118683,"source_line":799,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_depth_compare_mode
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":49812,"parent_record_offset":49460,"ancestor_record_offset":33152,"inlinee":118674,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::binding_set::impl$10::default
//     owner=git_dependency_definition; count=20
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3272,"record_offset":39676,"parent_record_offset":39652,"ancestor_record_offset":33152,"inlinee":12184,"source_line":5,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$1::is_compute::closure$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3272,"record_offset":34848,"parent_record_offset":34812,"ancestor_record_offset":33152,"inlinee":118657,"source_line":130,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$1::is_compute::closure$0::closure$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3272,"record_offset":35096,"parent_record_offset":35072,"ancestor_record_offset":33152,"inlinee":118659,"source_line":134,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$7::eq
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3272,"record_offset":35120,"parent_record_offset":35096,"ancestor_record_offset":33152,"inlinee":118661,"source_line":12,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3272,"record_offset":51564,"parent_record_offset":51520,"ancestor_record_offset":33152,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3272,"record_offset":46428,"parent_record_offset":46404,"ancestor_record_offset":33152,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>
// 记录形式: compiled_procedure; instances=2
// 源码行: 319
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3272,"record_offset":26644,"rva":48270032,"va":5416979152}
// 编译实例 RVAs: 0x2e08ad0, 0x2e09440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-41300657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416979152,"raw_hex":"e84fcd89fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416979356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-356977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416979152,"raw_hex":"e88f8dfaff","size":5,"soft_fail":false,"target_va":5416622240,"va":5416979212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":26864,"parent_record_offset":26644,"ancestor_record_offset":26644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":27052,"parent_record_offset":27028,"ancestor_record_offset":26644,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":26644,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":26644,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":27028,"parent_record_offset":26972,"ancestor_record_offset":26644,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":27000,"parent_record_offset":26972,"ancestor_record_offset":26644,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":26972,"parent_record_offset":26948,"ancestor_record_offset":26644,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":26948,"parent_record_offset":26928,"ancestor_record_offset":26644,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":27208,"parent_record_offset":27184,"ancestor_record_offset":26644,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3272,"record_offset":27992,"parent_record_offset":27964,"ancestor_record_offset":26644,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":26888,"parent_record_offset":26864,"ancestor_record_offset":26644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$1<country_core::startup::load_shaders::LoadOnlyBootupShaders>
// 记录形式: compiled_procedure; instances=2
// 源码行: 368
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3272,"record_offset":24168,"rva":48269632,"va":5416978752}
// 编译实例 RVAs: 0x2e08940, 0x2e092b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-41300257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416978752,"raw_hex":"e8dfce89fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416978956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-356577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416978752,"raw_hex":"e81f8ffaff","size":5,"soft_fail":false,"target_va":5416622240,"va":5416978812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":24388,"parent_record_offset":24168,"ancestor_record_offset":24168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":24576,"parent_record_offset":24552,"ancestor_record_offset":24168,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":24600,"parent_record_offset":24576,"ancestor_record_offset":24168,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":24624,"parent_record_offset":24600,"ancestor_record_offset":24168,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":24552,"parent_record_offset":24496,"ancestor_record_offset":24168,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":24524,"parent_record_offset":24496,"ancestor_record_offset":24168,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":24496,"parent_record_offset":24472,"ancestor_record_offset":24168,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":24472,"parent_record_offset":24452,"ancestor_record_offset":24168,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":24732,"parent_record_offset":24708,"ancestor_record_offset":24168,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3272,"record_offset":25516,"parent_record_offset":25488,"ancestor_record_offset":24168,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":24412,"parent_record_offset":24388,"ancestor_record_offset":24168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 740
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF87
// 首条位置: {"module":3272,"inlinee":118663}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3272,"record_offset":45672,"parent_record_offset":45636,"ancestor_record_offset":45636,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":45756,"parent_record_offset":45728,"ancestor_record_offset":45636,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":45780,"parent_record_offset":45756,"ancestor_record_offset":45636,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":45804,"parent_record_offset":45780,"ancestor_record_offset":45636,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3272,"record_offset":45728,"parent_record_offset":45700,"ancestor_record_offset":45636,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3272,"record_offset":45700,"parent_record_offset":45672,"ancestor_record_offset":45636,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$3<country_core::startup::load_shaders::LoadOnlyBootupShaders>
// 记录形式: compiled_procedure; instances=2
// 源码行: 784
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3272,"record_offset":21692,"rva":48269232,"va":5416978352}
// 编译实例 RVAs: 0x2e087b0, 0x2e09120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-41299857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416978352,"raw_hex":"e86fd089fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416978556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-356177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416978352,"raw_hex":"e8af90faff","size":5,"soft_fail":false,"target_va":5416622240,"va":5416978412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":21912,"parent_record_offset":21692,"ancestor_record_offset":21692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":22100,"parent_record_offset":22076,"ancestor_record_offset":21692,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":22124,"parent_record_offset":22100,"ancestor_record_offset":21692,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":22148,"parent_record_offset":22124,"ancestor_record_offset":21692,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":22076,"parent_record_offset":22020,"ancestor_record_offset":21692,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":22048,"parent_record_offset":22020,"ancestor_record_offset":21692,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":22020,"parent_record_offset":21996,"ancestor_record_offset":21692,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":21996,"parent_record_offset":21976,"ancestor_record_offset":21692,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":22256,"parent_record_offset":22232,"ancestor_record_offset":21692,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3272,"record_offset":23040,"parent_record_offset":23012,"ancestor_record_offset":21692,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":21692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders>
// 记录形式: compiled_procedure; instances=2
// 源码行: 789
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3272,"record_offset":17152,"rva":39437152,"va":5408146272}
// 编译实例 RVAs: 0x259c360, 0x26013b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-32467658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8369510fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5408146341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-32468060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8a49310fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5408146759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39424139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8756fa6fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5408146326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38916137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8d72faefd","size":5,"soft_fail":false,"target_va":5369231648,"va":5408147780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ace80
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38730283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8d505b1fd","size":5,"soft_fail":false,"target_va":5369417344,"va":5408147622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409930a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29399164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e884673ffe","size":5,"soft_fail":false,"target_va":5378748576,"va":5408147735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e47c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16482052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8fc8004ff","size":5,"soft_fail":false,"target_va":5391665088,"va":5408147135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f82d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16400897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8ffbd05ff","size":5,"soft_fail":false,"target_va":5391745744,"va":5408146636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141600850
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16366566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e81a4406ff","size":5,"soft_fail":false,"target_va":5391779920,"va":5408146481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7931a
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8244071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e867cb7d00","size":5,"soft_fail":false,"target_va":5416391450,"va":5408147374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8910097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e811f58700","size":5,"soft_fail":false,"target_va":5417057040,"va":5408146938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8910635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e82bf78700","size":5,"soft_fail":false,"target_va":5417057056,"va":5408146416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8473329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8f14a8100","size":5,"soft_fail":false,"target_va":5416620794,"va":5408147460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23523769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8470e99fe","size":5,"soft_fail":false,"target_va":5384624080,"va":5408147844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8474797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8ad508100","size":5,"soft_fail":false,"target_va":5416622240,"va":5408147438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8474292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8b44e8100","size":5,"soft_fail":false,"target_va":5416621632,"va":5408147335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_shaders::create_vertex_layout_via_pipeline_reflection
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30053232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e8906c35fe","size":5,"soft_fail":false,"target_va":5378093376,"va":5408146603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$0<country_core::startup::load_shaders::LoadAllGameShaders>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8832051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e833c48600","size":5,"soft_fail":false,"target_va":5416979552,"va":5408147496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shaderface::permutation::ShaderPermutationKey::to_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     evidence={"assembly":"\tcallq\t-24147298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408146272,"raw_hex":"e89e8a8ffe","size":5,"soft_fail":false,"target_va":5383999216,"va":5408146509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":3272,"record_offset":17420,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3272,"record_offset":20200,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3272,"record_offset":18620,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":19460,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":18648,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":21380,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":18704,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3272,"record_offset":20316,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3272,"record_offset":18468,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3272,"record_offset":18504,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":3272,"record_offset":20140,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineDesc::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":20288,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":42287,"source_line":840,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::vertex::impl$37::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/vertex.rs
//     evidence={"module":3272,"record_offset":18676,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":42289,"source_line":538,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] shaderface::permutation::impl$1::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/permutation.rs
//     evidence={"module":3272,"record_offset":18356,"parent_record_offset":17152,"ancestor_record_offset":17152,"inlinee":33396,"source_line":31,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":17708,"parent_record_offset":17684,"ancestor_record_offset":17152,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":17732,"parent_record_offset":17708,"ancestor_record_offset":17152,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":17152,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":19096,"parent_record_offset":19072,"ancestor_record_offset":17152,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":17756,"parent_record_offset":17732,"ancestor_record_offset":17152,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":19120,"parent_record_offset":19096,"ancestor_record_offset":17152,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":17684,"parent_record_offset":17656,"ancestor_record_offset":17152,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3272,"record_offset":19048,"parent_record_offset":18948,"ancestor_record_offset":17152,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":17596,"parent_record_offset":17568,"ancestor_record_offset":17152,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":18980,"parent_record_offset":18948,"ancestor_record_offset":17152,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":18948,"parent_record_offset":18920,"ancestor_record_offset":17152,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":17656,"parent_record_offset":17624,"ancestor_record_offset":17152,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":17624,"parent_record_offset":17596,"ancestor_record_offset":17152,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3272,"record_offset":18920,"parent_record_offset":18892,"ancestor_record_offset":17152,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3272,"record_offset":17476,"parent_record_offset":17448,"ancestor_record_offset":17152,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3272,"record_offset":17504,"parent_record_offset":17476,"ancestor_record_offset":17152,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3272,"record_offset":17532,"parent_record_offset":17504,"ancestor_record_offset":17152,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3272,"record_offset":17448,"parent_record_offset":17420,"ancestor_record_offset":17152,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3272,"record_offset":20168,"parent_record_offset":20140,"ancestor_record_offset":17152,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":19764,"parent_record_offset":19740,"ancestor_record_offset":17152,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":17568,"parent_record_offset":17532,"ancestor_record_offset":17152,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":19704,"parent_record_offset":19680,"ancestor_record_offset":17152,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":18404,"parent_record_offset":18380,"ancestor_record_offset":17152,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":18788,"parent_record_offset":18760,"ancestor_record_offset":17152,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3272,"record_offset":19740,"parent_record_offset":19704,"ancestor_record_offset":17152,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":3272,"record_offset":19508,"parent_record_offset":19484,"ancestor_record_offset":17152,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":19624,"parent_record_offset":19560,"ancestor_record_offset":17152,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":19936,"parent_record_offset":19908,"ancestor_record_offset":17152,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3272,"record_offset":19852,"parent_record_offset":19828,"ancestor_record_offset":17152,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":3272,"record_offset":18428,"parent_record_offset":18404,"ancestor_record_offset":17152,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3272,"record_offset":18380,"parent_record_offset":18356,"ancestor_record_offset":17152,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3272,"record_offset":19016,"parent_record_offset":18980,"ancestor_record_offset":17152,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3272,"record_offset":19596,"parent_record_offset":19560,"ancestor_record_offset":17152,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3272,"record_offset":17812,"parent_record_offset":17532,"ancestor_record_offset":17152,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":17836,"parent_record_offset":17812,"ancestor_record_offset":17152,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3272,"record_offset":18732,"parent_record_offset":18704,"ancestor_record_offset":17152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":18556,"parent_record_offset":18532,"ancestor_record_offset":17152,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":19532,"parent_record_offset":19508,"ancestor_record_offset":17152,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":19560,"parent_record_offset":19532,"ancestor_record_offset":17152,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3272,"record_offset":19992,"parent_record_offset":19964,"ancestor_record_offset":17152,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3272,"record_offset":18532,"parent_record_offset":18504,"ancestor_record_offset":17152,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3272,"record_offset":19908,"parent_record_offset":19880,"ancestor_record_offset":17152,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":20228,"parent_record_offset":20200,"ancestor_record_offset":17152,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":20252,"parent_record_offset":20228,"ancestor_record_offset":17152,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":3272,"record_offset":19828,"parent_record_offset":19796,"ancestor_record_offset":17152,"inlinee":27934,"source_line":34,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":18588,"parent_record_offset":18504,"ancestor_record_offset":17152,"inlinee":118650,"source_line":807,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3272,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":17152,"inlinee":118651,"source_line":810,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::vertex::impl$39::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/vertex.rs
//     evidence={"module":3272,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":17152,"inlinee":32861,"source_line":538,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::vertex::impl$51::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/vertex.rs
//     evidence={"module":3272,"record_offset":20064,"parent_record_offset":20040,"ancestor_record_offset":17152,"inlinee":32865,"source_line":592,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::vertex::impl$57::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/vertex.rs
//     evidence={"module":3272,"record_offset":19652,"parent_record_offset":19624,"ancestor_record_offset":17152,"inlinee":32863,"source_line":598,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$0<country_core::startup::load_shaders::LoadAllGameShaders>
// 记录形式: compiled_procedure; instances=2
// 源码行: 797
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3272,"record_offset":29120,"rva":48270432,"va":5416979552}
// 编译实例 RVAs: 0x2e08c60, 0x2e095d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-357389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416979552,"raw_hex":"e8f38bfaff","size":5,"soft_fail":false,"target_va":5416622240,"va":5416979624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 807
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF7A
// 首条位置: {"module":3272,"inlinee":118650}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 810
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF7B
// 首条位置: {"module":3272,"inlinee":118651}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 908
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF99
// 首条位置: {"module":3272,"inlinee":118681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::render_pass::impl$12::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49108,"parent_record_offset":49080,"ancestor_record_offset":49080,"inlinee":13123,"source_line":78,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":3272,"record_offset":49260,"parent_record_offset":49232,"ancestor_record_offset":49080,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":49232,"parent_record_offset":49208,"ancestor_record_offset":49080,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":49288,"parent_record_offset":49232,"ancestor_record_offset":49080,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":49312,"parent_record_offset":49288,"ancestor_record_offset":49080,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":49184,"parent_record_offset":49160,"ancestor_record_offset":49080,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":49208,"parent_record_offset":49184,"ancestor_record_offset":49080,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":49360,"parent_record_offset":49132,"ancestor_record_offset":49080,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":49384,"parent_record_offset":49360,"ancestor_record_offset":49080,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49160,"parent_record_offset":49132,"ancestor_record_offset":49080,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49132,"parent_record_offset":49108,"ancestor_record_offset":49080,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 916
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF9A
// 首条位置: {"module":3272,"inlinee":118682}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::render_pass::impl$12::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49496,"parent_record_offset":49460,"ancestor_record_offset":49460,"inlinee":13123,"source_line":78,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_depth_clamp_enable
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":49840,"parent_record_offset":49460,"ancestor_record_offset":49460,"inlinee":118683,"source_line":799,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_depth_compare_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":49812,"parent_record_offset":49460,"ancestor_record_offset":49460,"inlinee":118674,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":3272,"record_offset":49648,"parent_record_offset":49620,"ancestor_record_offset":49460,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":49620,"parent_record_offset":49596,"ancestor_record_offset":49460,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":49676,"parent_record_offset":49620,"ancestor_record_offset":49460,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":49700,"parent_record_offset":49676,"ancestor_record_offset":49460,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":49572,"parent_record_offset":49548,"ancestor_record_offset":49460,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":49596,"parent_record_offset":49572,"ancestor_record_offset":49460,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":49748,"parent_record_offset":49520,"ancestor_record_offset":49460,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":49772,"parent_record_offset":49748,"ancestor_record_offset":49460,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49548,"parent_record_offset":49520,"ancestor_record_offset":49460,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49520,"parent_record_offset":49496,"ancestor_record_offset":49460,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$1::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 930
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF9C
// 首条位置: {"module":3272,"inlinee":118684}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3272,"record_offset":50256,"parent_record_offset":49900,"ancestor_record_offset":49900,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::render_pass::impl$12::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49940,"parent_record_offset":49900,"ancestor_record_offset":49900,"inlinee":13123,"source_line":78,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_cull_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":50284,"parent_record_offset":49900,"ancestor_record_offset":49900,"inlinee":42285,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_depth_clamp_enable
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":50512,"parent_record_offset":49900,"ancestor_record_offset":49900,"inlinee":118683,"source_line":799,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::with_depth_compare_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":50396,"parent_record_offset":49900,"ancestor_record_offset":49900,"inlinee":118674,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":3272,"record_offset":50092,"parent_record_offset":50064,"ancestor_record_offset":49900,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":50064,"parent_record_offset":50040,"ancestor_record_offset":49900,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":50120,"parent_record_offset":50064,"ancestor_record_offset":49900,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3272,"record_offset":50144,"parent_record_offset":50120,"ancestor_record_offset":49900,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":50016,"parent_record_offset":49992,"ancestor_record_offset":49900,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3272,"record_offset":50040,"parent_record_offset":50016,"ancestor_record_offset":49900,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":50192,"parent_record_offset":49964,"ancestor_record_offset":49900,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3272,"record_offset":50216,"parent_record_offset":50192,"ancestor_record_offset":49900,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"modular-bitfield","version":"0.13.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.13.1/src/private/proc.rs
//     evidence={"module":3272,"record_offset":50356,"parent_record_offset":50332,"ancestor_record_offset":49900,"inlinee":118666,"source_line":67,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49992,"parent_record_offset":49964,"ancestor_record_offset":49900,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3272,"record_offset":49964,"parent_record_offset":49940,"ancestor_record_offset":49900,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":50308,"parent_record_offset":50284,"ancestor_record_offset":49900,"inlinee":118664,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":50332,"parent_record_offset":50308,"ancestor_record_offset":49900,"inlinee":118665,"source_line":795,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":50420,"parent_record_offset":50396,"ancestor_record_offset":49900,"inlinee":118675,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3272,"record_offset":50444,"parent_record_offset":50420,"ancestor_record_offset":49900,"inlinee":118676,"source_line":797,"archive":"_pdb/C/modules/03272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::preload_shaders::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 974
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D42E
// 首条位置: {"module":3300,"inlinee":119854}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::startup::load_shaders::preload_shaders::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3300,"record_offset":19064,"parent_record_offset":19016,"ancestor_record_offset":19016,"inlinee":118653,"source_line":302,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3300,"record_offset":29184,"parent_record_offset":29160,"ancestor_record_offset":19016,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3300,"record_offset":29208,"parent_record_offset":29184,"ancestor_record_offset":19016,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3300,"record_offset":28972,"parent_record_offset":28940,"ancestor_record_offset":19016,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3300,"record_offset":29004,"parent_record_offset":28972,"ancestor_record_offset":19016,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3300,"record_offset":29232,"parent_record_offset":29208,"ancestor_record_offset":19016,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3300,"record_offset":29040,"parent_record_offset":29004,"ancestor_record_offset":19016,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3300,"record_offset":29160,"parent_record_offset":29128,"ancestor_record_offset":19016,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3300,"record_offset":28940,"parent_record_offset":28876,"ancestor_record_offset":19016,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$0::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":3300,"record_offset":31828,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":6961,"source_line":77,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3300,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":19016,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3300,"record_offset":29100,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3300,"record_offset":44596,"parent_record_offset":44572,"ancestor_record_offset":19016,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3300,"record_offset":32172,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3300,"record_offset":32220,"parent_record_offset":32196,"ancestor_record_offset":19016,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3300,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":19016,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3300,"record_offset":28912,"parent_record_offset":28876,"ancestor_record_offset":19016,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3300,"record_offset":28876,"parent_record_offset":28844,"ancestor_record_offset":19016,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3300,"record_offset":20936,"parent_record_offset":20912,"ancestor_record_offset":19016,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3300,"record_offset":20912,"parent_record_offset":20888,"ancestor_record_offset":19016,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3300,"record_offset":28844,"parent_record_offset":28816,"ancestor_record_offset":19016,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$55::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3300,"record_offset":31880,"parent_record_offset":31856,"ancestor_record_offset":19016,"inlinee":22224,"source_line":3244,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":25696,"parent_record_offset":25652,"ancestor_record_offset":19016,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":20864,"parent_record_offset":20840,"ancestor_record_offset":19016,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":20840,"parent_record_offset":20816,"ancestor_record_offset":19016,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":31768,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":23832,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":25620,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":25652,"parent_record_offset":25620,"ancestor_record_offset":19016,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":31344,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":20276,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":36540,"parent_record_offset":36516,"ancestor_record_offset":19016,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3300,"record_offset":20816,"parent_record_offset":20760,"ancestor_record_offset":19016,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3300,"record_offset":34964,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3300,"record_offset":20528,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=251
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3300,"record_offset":21152,"parent_record_offset":21128,"ancestor_record_offset":19016,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3300,"record_offset":31856,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":3300,"record_offset":32256,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3300,"record_offset":36768,"parent_record_offset":36732,"ancestor_record_offset":19016,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":3300,"record_offset":34800,"parent_record_offset":34772,"ancestor_record_offset":19016,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3300,"record_offset":37500,"parent_record_offset":37428,"ancestor_record_offset":19016,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3300,"record_offset":37524,"parent_record_offset":37500,"ancestor_record_offset":19016,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3300,"record_offset":20552,"parent_record_offset":20528,"ancestor_record_offset":19016,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3300,"record_offset":32196,"parent_record_offset":32172,"ancestor_record_offset":19016,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3300,"record_offset":36456,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3300,"record_offset":36164,"parent_record_offset":35808,"ancestor_record_offset":19016,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3300,"record_offset":21864,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3300,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":19016,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3300,"record_offset":34772,"parent_record_offset":34748,"ancestor_record_offset":19016,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3300,"record_offset":34828,"parent_record_offset":34772,"ancestor_record_offset":19016,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3300,"record_offset":34852,"parent_record_offset":34828,"ancestor_record_offset":19016,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3300,"record_offset":49048,"parent_record_offset":48988,"ancestor_record_offset":47612,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3300,"record_offset":34724,"parent_record_offset":34700,"ancestor_record_offset":19016,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3300,"record_offset":34748,"parent_record_offset":34724,"ancestor_record_offset":19016,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3300,"record_offset":20412,"parent_record_offset":20388,"ancestor_record_offset":19016,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3300,"record_offset":25816,"parent_record_offset":25652,"ancestor_record_offset":19016,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3300,"record_offset":21600,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=251
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3300,"record_offset":21176,"parent_record_offset":21152,"ancestor_record_offset":19016,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=251
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3300,"record_offset":21200,"parent_record_offset":21176,"ancestor_record_offset":19016,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3300,"record_offset":31636,"parent_record_offset":31608,"ancestor_record_offset":19016,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3300,"record_offset":20724,"parent_record_offset":20696,"ancestor_record_offset":19016,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3300,"record_offset":20300,"parent_record_offset":20276,"ancestor_record_offset":19016,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3300,"record_offset":21128,"parent_record_offset":21104,"ancestor_record_offset":19016,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3300,"record_offset":31372,"parent_record_offset":31344,"ancestor_record_offset":19016,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3300,"record_offset":31400,"parent_record_offset":31372,"ancestor_record_offset":19016,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3300,"record_offset":20324,"parent_record_offset":20300,"ancestor_record_offset":19016,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3300,"record_offset":31440,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3300,"record_offset":20388,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3300,"record_offset":31580,"parent_record_offset":31544,"ancestor_record_offset":19016,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3300,"record_offset":21080,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3300,"record_offset":21104,"parent_record_offset":21080,"ancestor_record_offset":19016,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3300,"record_offset":31608,"parent_record_offset":31580,"ancestor_record_offset":19016,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=246
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3300,"record_offset":21248,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3300,"record_offset":34900,"parent_record_offset":34672,"ancestor_record_offset":19016,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3300,"record_offset":22036,"parent_record_offset":21912,"ancestor_record_offset":19016,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3300,"record_offset":21712,"parent_record_offset":21672,"ancestor_record_offset":19016,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3300,"record_offset":20468,"parent_record_offset":20444,"ancestor_record_offset":19016,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3300,"record_offset":34924,"parent_record_offset":34900,"ancestor_record_offset":19016,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3300,"record_offset":22060,"parent_record_offset":22036,"ancestor_record_offset":19016,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3300,"record_offset":21736,"parent_record_offset":21712,"ancestor_record_offset":19016,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3300,"record_offset":20492,"parent_record_offset":20468,"ancestor_record_offset":19016,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::shader_library::AssetShaderLibrary::register_world_raster_pipelines
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":3300,"record_offset":37428,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":32913,"source_line":103,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":3300,"record_offset":32144,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::impl$3::should_load
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3300,"record_offset":49080,"parent_record_offset":47660,"ancestor_record_offset":47612,"inlinee":28075,"source_line":217,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::is_game_shader
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3300,"record_offset":49108,"parent_record_offset":49080,"ancestor_record_offset":47612,"inlinee":28024,"source_line":186,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3300,"record_offset":31544,"parent_record_offset":31440,"ancestor_record_offset":19016,"inlinee":118663,"source_line":740,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$5
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3300,"record_offset":34988,"parent_record_offset":34964,"ancestor_record_offset":19016,"inlinee":118681,"source_line":908,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3300,"record_offset":35368,"parent_record_offset":35340,"ancestor_record_offset":19016,"inlinee":118682,"source_line":916,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_shaders::preload_shaders::closure$1::closure$7
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_shaders.rs
//     evidence={"module":3300,"record_offset":35808,"parent_record_offset":35784,"ancestor_record_offset":19016,"inlinee":118684,"source_line":930,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":3300,"record_offset":21640,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$28::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":3300,"record_offset":21888,"parent_record_offset":21864,"ancestor_record_offset":19016,"inlinee":12297,"source_line":1829,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3300,"record_offset":20632,"parent_record_offset":20608,"ancestor_record_offset":19016,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3300,"record_offset":20584,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3300,"record_offset":20608,"parent_record_offset":20584,"ancestor_record_offset":19016,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3300,"record_offset":20656,"parent_record_offset":20632,"ancestor_record_offset":19016,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3300,"record_offset":20444,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"modular-bitfield","version":"0.13.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.13.1/src/private/proc.rs
//     evidence={"module":3300,"record_offset":33564,"parent_record_offset":33540,"ancestor_record_offset":19016,"inlinee":118666,"source_line":67,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3300,"record_offset":21768,"parent_record_offset":21672,"ancestor_record_offset":19016,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3300,"record_offset":21948,"parent_record_offset":21912,"ancestor_record_offset":19016,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3300,"record_offset":21672,"parent_record_offset":21640,"ancestor_record_offset":19016,"inlinee":6092,"source_line":67,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":3300,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":19016,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3300,"record_offset":21796,"parent_record_offset":21768,"ancestor_record_offset":19016,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3300,"record_offset":21976,"parent_record_offset":21948,"ancestor_record_offset":19016,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$12::clone
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3300,"record_offset":34648,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":13123,"source_line":78,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3300,"record_offset":34700,"parent_record_offset":34672,"ancestor_record_offset":19016,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3300,"record_offset":34672,"parent_record_offset":34648,"ancestor_record_offset":19016,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33772,"parent_record_offset":33748,"ancestor_record_offset":19016,"inlinee":118672,"source_line":811,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33796,"parent_record_offset":33772,"ancestor_record_offset":19016,"inlinee":118673,"source_line":811,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33516,"parent_record_offset":33492,"ancestor_record_offset":19016,"inlinee":118664,"source_line":795,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33540,"parent_record_offset":33516,"ancestor_record_offset":19016,"inlinee":118665,"source_line":795,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33884,"parent_record_offset":33860,"ancestor_record_offset":19016,"inlinee":118675,"source_line":797,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33908,"parent_record_offset":33884,"ancestor_record_offset":19016,"inlinee":118676,"source_line":797,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33972,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":118677,"source_line":798,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33996,"parent_record_offset":33972,"ancestor_record_offset":19016,"inlinee":118678,"source_line":798,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":34056,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":118679,"source_line":807,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":34080,"parent_record_offset":34056,"ancestor_record_offset":19016,"inlinee":118680,"source_line":807,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_topology
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33632,"parent_record_offset":33608,"ancestor_record_offset":19016,"inlinee":118668,"source_line":794,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33656,"parent_record_offset":33632,"ancestor_record_offset":19016,"inlinee":118669,"source_line":794,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_blend_mode
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33748,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":118671,"source_line":811,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_cull_mode
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33492,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":42285,"source_line":795,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_depth_clamp_enable
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":35748,"parent_record_offset":35368,"ancestor_record_offset":19016,"inlinee":118683,"source_line":799,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_depth_compare_mode
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33860,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":118674,"source_line":797,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_stencil_pass_op
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33720,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":118670,"source_line":807,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::with_topology
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":3300,"record_offset":33608,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":118667,"source_line":794,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::binding_set::BindingSetDesc::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3300,"record_offset":31800,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":100461,"source_line":141,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::binding_set::impl$10::default
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3300,"record_offset":25588,"parent_record_offset":25564,"ancestor_record_offset":19016,"inlinee":12184,"source_line":5,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::binding_set::impl$5::into_binding_set_desc
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/binding_set.rs
//     evidence={"module":3300,"record_offset":25564,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":12198,"source_line":58,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::PrecompiledShaderEntry::is_compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3300,"record_offset":20696,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":118655,"source_line":129,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$1::is_compute::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3300,"record_offset":20760,"parent_record_offset":20724,"ancestor_record_offset":19016,"inlinee":118657,"source_line":130,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$1::is_compute::closure$0::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3300,"record_offset":21008,"parent_record_offset":20984,"ancestor_record_offset":19016,"inlinee":118659,"source_line":134,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] shaderface::shader::impl$7::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"shaderface","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed"}]
//     definition_source=/home/h3/.cargo/git/checkouts/shabriri-635d53344d427385/2212a07/crates/shaderface/src/shader.rs
//     evidence={"module":3300,"record_offset":21032,"parent_record_offset":21008,"ancestor_record_offset":19016,"inlinee":118661,"source_line":12,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3300,"record_offset":37472,"parent_record_offset":37428,"ancestor_record_offset":19016,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3300,"record_offset":32312,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3300,"record_offset":32336,"parent_record_offset":32312,"ancestor_record_offset":19016,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   [pdb_inline_transitive] ustr::ustr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":3300,"record_offset":32284,"parent_record_offset":19064,"ancestor_record_offset":19016,"inlinee":27983,"source_line":469,"archive":"_pdb/C/modules/03300.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shaders::create_vertex_layout_via_pipeline_reflection
// 记录形式: compiled_procedure; instances=1
// 源码行: 995
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":7644,"rva":9384256,"va":5378093376}
// 编译实例 RVAs: 0x8f3140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$LP$$RF$str$C$rhapsody..vertex..VertexFormat$RP$$u20$as$u20$rhapsody..vertex..InterleaveVertices$GT$::interleave_vertices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5566934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e8d6f15400","size":5,"soft_fail":false,"target_va":5383661136,"va":5378094197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$$RF$$u5b$$LP$$RF$str$C$rhapsody..vertex..VertexFormat$RP$$u5d$$u20$as$u20$rhapsody..vertex..InterleaveVertices$GT$::interleave_vertices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5567168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e8c0f25400","size":5,"soft_fail":false,"target_va":5383661424,"va":5378094251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2415665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e8cf23dbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378094364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ace80
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8677397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e8eb977bff","size":5,"soft_fail":false,"target_va":5369417344,"va":5378094736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7e7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1618310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e886b11800","size":5,"soft_fail":false,"target_va":5379712928,"va":5378094613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152dd20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12822478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e8cea7c300","size":5,"soft_fail":false,"target_va":5390916896,"va":5378094413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38256163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e823be4702","size":5,"soft_fail":false,"target_va":5416350384,"va":5378094216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_shaders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38256929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e821c14702","size":5,"soft_fail":false,"target_va":5416350944,"va":5378094010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38527728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378093376,"raw_hex":"e8f0e24b02","size":5,"soft_fail":false,"target_va":5416622240,"va":5378094507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":9296,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":8020,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":8340,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":10108,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":9356,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":7864,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":10772,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":9156,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":11893,"source_line":3240,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":10052,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":500,"record_offset":9384,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":9016,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::vertex::impl$35::default
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/vertex.rs
//     evidence={"module":500,"record_offset":9328,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":13125,"source_line":538,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$35::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":500,"record_offset":8048,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":8500,"source_line":2025,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":500,"record_offset":8564,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":500,"record_offset":9936,"parent_record_offset":9912,"ancestor_record_offset":7644,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":500,"record_offset":9960,"parent_record_offset":9936,"ancestor_record_offset":7644,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":500,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":7644,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":500,"record_offset":9912,"parent_record_offset":9820,"ancestor_record_offset":7644,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":8440,"parent_record_offset":8408,"ancestor_record_offset":7644,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":9848,"parent_record_offset":9820,"ancestor_record_offset":7644,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":9820,"parent_record_offset":9796,"ancestor_record_offset":7644,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":8488,"parent_record_offset":8464,"ancestor_record_offset":7644,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":7644,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":9796,"parent_record_offset":9772,"ancestor_record_offset":7644,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":8408,"parent_record_offset":8368,"ancestor_record_offset":7644,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":7644,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":7920,"parent_record_offset":7892,"ancestor_record_offset":7644,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":500,"record_offset":8256,"parent_record_offset":8232,"ancestor_record_offset":7644,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":500,"record_offset":9232,"parent_record_offset":9192,"ancestor_record_offset":7644,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":9880,"parent_record_offset":9848,"ancestor_record_offset":7644,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":500,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":7644,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":500,"record_offset":8288,"parent_record_offset":8204,"ancestor_record_offset":7644,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":9724,"parent_record_offset":9528,"ancestor_record_offset":7644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":7644,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":7644,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":7644,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":8528,"parent_record_offset":8368,"ancestor_record_offset":7644,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":7644,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":7976,"parent_record_offset":7948,"ancestor_record_offset":7644,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":7644,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":10184,"parent_record_offset":10156,"ancestor_record_offset":7644,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":10132,"parent_record_offset":10108,"ancestor_record_offset":7644,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":7644,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":500,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":7644,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":500,"record_offset":9192,"parent_record_offset":9156,"ancestor_record_offset":7644,"inlinee":11895,"source_line":22,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":7644,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":500,"record_offset":8708,"parent_record_offset":8680,"ancestor_record_offset":7644,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":500,"record_offset":8740,"parent_record_offset":8708,"ancestor_record_offset":7644,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":500,"record_offset":8680,"parent_record_offset":8640,"ancestor_record_offset":7644,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":500,"record_offset":8232,"parent_record_offset":8204,"ancestor_record_offset":7644,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":500,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7644,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":500,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":7644,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":500,"record_offset":8860,"parent_record_offset":8640,"ancestor_record_offset":7644,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":500,"record_offset":8204,"parent_record_offset":8144,"ancestor_record_offset":7644,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":500,"record_offset":8640,"parent_record_offset":8612,"ancestor_record_offset":7644,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":500,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":7644,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":500,"record_offset":8176,"parent_record_offset":8144,"ancestor_record_offset":7644,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":500,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":7644,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":500,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":7644,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":500,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":7644,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00500.jsonl.gz"}
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
// 完整结构及原始类型记录: load_shaders.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_shaders::LoadAllGameShaders
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>", "constructor": "country_core::startup::load_shaders::preload_shaders::closure_env$2", "arguments": ["country_core::startup::load_shaders::LoadAllGameShaders"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0", "arguments": ["alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>", "shaderface::shader::PrecompiledShaderModule", "country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "async_executor::impl$5::run::async_fn$0::async_block_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >,enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >,enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >,enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "futures_lite::future::block_on::closure_env$0", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >,alloc::vec::Vec<tuple$<>,alloc::alloc::Global> >", "constructor": "std::thread::local::LocalKey::with", "arguments": ["core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >", "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "alloc::vec::Vec<tuple$<>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> >", "constructor": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0", "arguments": ["alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>", "shaderface::shader::PrecompiledShaderModule", "country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::closure_env$1", "arguments": ["enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::async_block_env$0", "arguments": ["enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::clone_waker<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >,enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::clone_waker", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >", "enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::drop_future<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >,enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::drop_future", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >", "enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::run<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >,enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::run", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >", "enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::async_block$0", "arguments": ["enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders>", "constructor": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$4", "arguments": ["country_core::startup::load_shaders::LoadAllGameShaders"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$0<country_core::startup::load_shaders::LoadAllGameShaders>", "constructor": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$0", "arguments": ["country_core::startup::load_shaders::LoadAllGameShaders"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_shaders::LoadOnlyBootupShaders
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 27
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>", "constructor": "country_core::startup::load_shaders::preload_shaders::closure_env$2", "arguments": ["country_core::startup::load_shaders::LoadOnlyBootupShaders"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0", "arguments": ["alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>", "shaderface::shader::PrecompiledShaderModule", "country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "futures_lite::future::block_on::closure_env$0", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >,alloc::vec::Vec<tuple$<>,alloc::alloc::Global> >", "constructor": "std::thread::local::LocalKey::with", "arguments": ["core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >", "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "alloc::vec::Vec<tuple$<>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> >", "constructor": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0", "arguments": ["alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>", "shaderface::shader::PrecompiledShaderModule", "country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::closure_env$1", "arguments": ["enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<tuple$<>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<tuple$<>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_shaders::preload_shaders::closure_env$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>", "constructor": "country_core::startup::load_shaders::preload_shaders::closure_env$0", "arguments": ["country_core::startup::load_shaders::LoadOnlyBootupShaders"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>,shaderface::shader::PrecompiledShaderModule,alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> >,country_core::startup::load_shaders::preload_shaders::closure_env$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<shaderface::shader::PrecompiledShaderModule> >", "constructor": "core::iter::adapters::map::map_try_fold::closure_env$0", "arguments": ["tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>", "shaderface::shader::PrecompiledShaderModule", "alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> >", "country_core::startup::load_shaders::preload_shaders::closure_env$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>", "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<shaderface::shader::PrecompiledShaderModule>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::try_fold<tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>,shaderface::shader::PrecompiledShaderModule,alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> >,country_core::startup::load_shaders::preload_shaders::closure_env$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<shaderface::shader::PrecompiledShaderModule> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> > >", "constructor": "alloc::vec::into_iter::impl$6::try_fold", "arguments": ["tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>", "alloc::alloc::Global", "alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>", "core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>,shaderface::shader::PrecompiledShaderModule,alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> >,country_core::startup::load_shaders::preload_shaders::closure_env$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<shaderface::shader::PrecompiledShaderModule> >", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::async_block_env$0", "arguments": ["enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::clone_waker<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >,enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::clone_waker", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >", "enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::drop_future<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >,enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::drop_future", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >", "enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::run<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >,enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::run", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >", "enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::async_block$0", "arguments": ["enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$3<country_core::startup::load_shaders::LoadOnlyBootupShaders>", "constructor": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$3", "arguments": ["country_core::startup::load_shaders::LoadOnlyBootupShaders"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$1<country_core::startup::load_shaders::LoadOnlyBootupShaders>", "constructor": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$1", "arguments": ["country_core::startup::load_shaders::LoadOnlyBootupShaders"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>", "constructor": "country_core::startup::load_shaders::preload_shaders::closure$1::closure$0", "arguments": ["country_core::startup::load_shaders::LoadOnlyBootupShaders"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["tuple$<>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> >,tuple$<> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_shaders::RenderPasses
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_shaders::RenderPasses>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_shaders::RenderPasses"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_shaders::ShaderLoadingOptions
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 52
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::startup::load_shaders::ShaderLoadingOptions>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::startup::load_shaders::ShaderLoadingOptions"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "constructor": "bevy_ecs::change_detection::Res", "arguments": ["country_core::startup::load_shaders::ShaderLoadingOptions"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>,bevy_app::main_schedule::PreStartup,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "bevy_app::main_schedule::PreStartup", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>,country_core::systems::TgLoadShaders,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "country_core::systems::TgLoadShaders", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<country_core::startup::load_shaders::ShaderLoadingOptions>", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["country_core::startup::load_shaders::ShaderLoadingOptions"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$114::init_access<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "constructor": "bevy_ecs::system::system_param::impl$114::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::init_resource<country_core::startup::load_shaders::ShaderLoadingOptions>", "constructor": "bevy_app::app::App::init_resource", "arguments": ["country_core::startup::load_shaders::ShaderLoadingOptions"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>,country_core::systems::TgLoadShaders,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::schedule::schedule::Schedules::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "country_core::systems::TgLoadShaders", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>,bevy_app::main_schedule::PreStartup,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::schedule::schedule::Schedules::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "bevy_app::main_schedule::PreStartup", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$114::validate_param<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "constructor": "bevy_ecs::system::system_param::impl$114::validate_param", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$114::get_param<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "constructor": "bevy_ecs::system::system_param::impl$114::get_param", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "constructor": "tuple$", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_shaders::ShaderModDeps
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 50
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::startup::load_shaders::ShaderModDeps>", "constructor": "core::option::Option", "arguments": ["country_core::startup::load_shaders::ShaderModDeps"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::startup::load_shaders::ShaderModDeps>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_platform::cell::sync_cell::SyncCell<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "constructor": "bevy_platform::cell::sync_cell::SyncCell", "arguments": ["enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_platform::cell::sync_cell::SyncCell<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_platform::cell::sync_cell::SyncCell<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>,bevy_app::main_schedule::PreStartup,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "bevy_app::main_schedule::PreStartup", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>,country_core::systems::TgLoadShaders,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "country_core::systems::TgLoadShaders", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$114::init_access<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "constructor": "bevy_ecs::system::system_param::impl$114::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>,country_core::systems::TgLoadShaders,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::schedule::schedule::Schedules::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "country_core::systems::TgLoadShaders", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>,bevy_app::main_schedule::PreStartup,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::schedule::schedule::Schedules::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "bevy_app::main_schedule::PreStartup", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$114::validate_param<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "constructor": "bevy_ecs::system::system_param::impl$114::validate_param", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$114::get_param<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "constructor": "bevy_ecs::system::system_param::impl$114::get_param", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "constructor": "tuple$", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_shaders.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_shaders.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 16; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::RenderParams>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>
// [文件级接口候选，具体函数归属 unknown] country_core::resources::history::HistoryChainSnapshot (*)(country_core::resources::history::ResolvedHistoryChainSnapshot)
// 候选参数类型: ["country_core::resources::history::ResolvedHistoryChainSnapshot"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::RenderPasses::new
// 信息 ID: dfca12a027d94dc4556cd6aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259ca9c -> 0x8f2920 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a3dec -> 0x8f2920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::create_vertex_layout_via_pipeline_reflection
// 信息 ID: fd9b0bd5d34b4bd4b0c238ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders> @ 0x259c4ab -> 0x8f3140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders> @ 0x26014fb -> 0x8f3140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::impl$2::should_load
// 信息 ID: ea23515f2bfa6efa539f5e57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259cfe4 -> 0x8f7960 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f3e64 -> 0x8f7960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::impl$3::should_load
// 信息 ID: bad2e33d56efa3c2c53119ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=71564; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >; module=3300, inline_record=49080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::is_game_shader
// 信息 ID: b3126e7926706f828d7b2e76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_shaders::impl$2::should_load; module=500, inline_record=64032; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=71592; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >; module=3300, inline_record=49108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders
// 信息 ID: 04f1cf87675d972c04bb7fe4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=30648; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=67672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$0
// 信息 ID: 8cb5077b32acc37a6c66e012
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$0 <- alloc::vec::into_iter::impl$6::try_fold<tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>,shaderface::shader::PrecompiledShaderModule,alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> >,country_core::startup::load_shaders::preload_shaders::closure_env$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<shaderface::shader::PrecompiledShaderModule> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::try_fold<tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<alloc::string::String,shaderface::shader::PrecompiledShaderModule>,shaderface::shader::PrecompiledShaderModule,alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> >,country_core::startup::load_shaders::preload_shaders::closure_env$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<shaderface::shader::PrecompiledShaderModule> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<shaderface::shader::PrecompiledShaderModule>,never$> > >; module=1822, inline_record=39044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1
// 信息 ID: 407a5da7cc70072973b093fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=33152; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=70132; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >; module=3300, inline_record=19064; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >; module=3300, inline_record=47660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$0<country_core::startup::load_shaders::LoadOnlyBootupShaders>
// 信息 ID: 7a8da71942130b40b8b2d80a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [类型关系] country_core::startup::load_shaders::preload_shaders::closure$1::closure$0 -> ["country_core::startup::load_shaders::LoadOnlyBootupShaders"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a0fa2 -> 0x2e08ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a85ed -> 0x2e08ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f7df0 -> 0x2e09440 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fe009 -> 0x2e09440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$1<country_core::startup::load_shaders::LoadOnlyBootupShaders>
// 信息 ID: 1abdcbd9732ecfec4a465a1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [类型关系] country_core::startup::load_shaders::preload_shaders::closure$1::closure$1 -> ["country_core::startup::load_shaders::LoadOnlyBootupShaders"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a0f31 -> 0x2e08940 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a857a -> 0x2e08940 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f7d83 -> 0x2e092b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fdf9c -> 0x2e092b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$2
// 信息 ID: 2c6e1de393433c294c6c257d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=45636; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=82864; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >; module=3300, inline_record=31544; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >; module=3300, inline_record=60404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$3<country_core::startup::load_shaders::LoadOnlyBootupShaders>
// 信息 ID: 77bad7fd6c8d1a13d186ee7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [类型关系] country_core::startup::load_shaders::preload_shaders::closure$1::closure$3 -> ["country_core::startup::load_shaders::LoadOnlyBootupShaders"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a0ec0 -> 0x2e087b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a8507 -> 0x2e087b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f7d16 -> 0x2e09120 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fdf2f -> 0x2e09120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$0<country_core::startup::load_shaders::LoadAllGameShaders>
// 信息 ID: 2b863c641d7ce9b31678e05c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$0 -> ["country_core::startup::load_shaders::LoadAllGameShaders"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders> @ 0x259c828 -> 0x2e08c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders> @ 0x2601878 -> 0x2e095d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$1
// 信息 ID: 704441aa646c031c41e9eeac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders>; module=3272, inline_record=18588; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders>; module=3300, inline_record=91544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$4::closure$2
// 信息 ID: ab97e80d3ef541057c5b58d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders>; module=3272, inline_record=21404; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders>; module=3300, inline_record=94360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$4<country_core::startup::load_shaders::LoadAllGameShaders>
// 信息 ID: 6ee3f1a42601f4917574d4aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [类型关系] country_core::startup::load_shaders::preload_shaders::closure$1::closure$4 -> ["country_core::startup::load_shaders::LoadAllGameShaders"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259ef77 -> 0x259c360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259efdd -> 0x259c360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259f04d -> 0x259c360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259f0dd -> 0x259c360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a64a3 -> 0x259c360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a6502 -> 0x259c360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a657d -> 0x259c360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a6620 -> 0x259c360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f5fa1 -> 0x26013b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f5ffd -> 0x26013b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f606a -> 0x26013b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f60fa -> 0x26013b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fc290 -> 0x26013b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fc2ec -> 0x26013b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fc35c -> 0x26013b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fc3ff -> 0x26013b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$5
// 信息 ID: c4ec50465f0a7937f9118d52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=49080; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=86308; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >; module=3300, inline_record=34988; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >; module=3300, inline_record=63848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$6
// 信息 ID: 8bd59f52ff174a17381da560
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=49460; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=86688; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >; module=3300, inline_record=35368; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >; module=3300, inline_record=64228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$1::closure$7
// 信息 ID: 03a35ddfde9dad9fdfea2c2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=49900; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)>; module=3272, inline_record=87128; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >; module=3300, inline_record=35808; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >; module=3300, inline_record=64668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shaders::preload_shaders::closure$2
// 信息 ID: 5859d18fea2ce1dc9419c73d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::load_shaders::preload_shaders", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$2 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$2 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$2 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$2 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$2 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$2 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$2 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_shaders::preload_shaders::closure_env$2 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<tuple$<>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> >,tuple$<> > > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 59 条。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > >; module=3300, inline_record=19016; 不是独立调用边。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > >; module=3300, inline_record=47612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
