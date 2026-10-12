// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/egui_input.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c2aa10f0f1a1f78238506071f8eecef31bec8ec4cf3d476584f7936fb44c3579
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: egui_input.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 531, 550, 1698
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_input; // namespace candidate
//   package identities: [{"name":"bevy_input","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use smol_str; // namespace candidate
//   package identities: [{"name":"smol_str","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":62556,"rva":11299312,"va":5380008432}
// 编译实例 RVAs: 0xac69f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$egui..data..input..RawInput$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2624199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380008432,"raw_hex":"e8c70a2800","size":5,"soft_fail":false,"target_va":5382632656,"va":5380008452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1dcc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-691596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380008432,"raw_hex":"e87472f5ff","size":5,"soft_fail":false,"target_va":5379316928,"va":5380008519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14151ae90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10830905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380008432,"raw_hex":"e83944a500","size":5,"soft_fail":false,"target_va":5390839440,"va":5380008530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14186fcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14324349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380008432,"raw_hex":"e87d92da00","size":5,"soft_fail":false,"target_va":5394332848,"va":5380008494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":550,"record_offset":62720,"parent_record_offset":62556,"ancestor_record_offset":62556,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::egui_input::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/egui_input.rs
//     evidence={"module":550,"record_offset":62692,"parent_record_offset":62556,"ancestor_record_offset":62556,"inlinee":34284,"source_line":24,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x85EC
// 首条位置: {"module":550,"inlinee":34284}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::EguiInputResource::on_cursor_moved
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":4640,"rva":11277632,"va":5379986752}
// 编译实例 RVAs: 0xac1540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36363669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379986752,"raw_hex":"e895dd2a02","size":5,"soft_fail":false,"target_va":5416350496,"va":5379986822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<egui::data::input::Event> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379986752,"raw_hex":"e8d7490000","size":5,"soft_fail":false,"target_va":5380005808,"va":5379986900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":4796,"parent_record_offset":4640,"ancestor_record_offset":4640,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":4904,"parent_record_offset":4872,"ancestor_record_offset":4640,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":4640,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":4928,"parent_record_offset":4904,"ancestor_record_offset":4640,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":4872,"parent_record_offset":4824,"ancestor_record_offset":4640,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":4824,"parent_record_offset":4796,"ancestor_record_offset":4640,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":4992,"parent_record_offset":4824,"ancestor_record_offset":4640,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::EguiInputResource::on_keyboard_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":5528,"rva":11278048,"va":5379987168}
// 编译实例 RVAs: 0xac16e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$smol_str..SmolStr$u20$as$u20$core..ops..deref..Deref$GT$::deref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4622496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e8a0884600","size":5,"soft_fail":false,"target_va":5384609872,"va":5379987371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4308958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e82240beff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379987641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11265441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e85f1a54ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379987628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36362702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e8ced92a02","size":5,"soft_fail":false,"target_va":5416350496,"va":5379987789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37069384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e848a23502","size":5,"soft_fail":false,"target_va":5417057056,"va":5379987667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36633097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e809fa2e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5379987692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36633823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e8dffc2e02","size":5,"soft_fail":false,"target_va":5416622208,"va":5379988380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<egui::data::input::Event> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e89e430000","size":5,"soft_fail":false,"target_va":5380005808,"va":5379988493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36633912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e838fd2e02","size":5,"soft_fail":false,"target_va":5416622320,"va":5379988403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::data::key::Key::from_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2637146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379987168,"raw_hex":"e85a3d2800","size":5,"soft_fail":false,"target_va":5382625344,"va":5379988193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":7676,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::ButtonState::is_pressed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/lib.rs
//     evidence={"module":550,"record_offset":5712,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":11541,"source_line":157,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":550,"record_offset":5868,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":550,"record_offset":8588,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":550,"record_offset":5756,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::egui_input::bevy_to_egui_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/egui_input.rs
//     evidence={"module":550,"record_offset":7908,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":34223,"source_line":166,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::egui_input::bevy_to_egui_physical_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/egui_input.rs
//     evidence={"module":550,"record_offset":8560,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":34224,"source_line":214,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] smol_str::SmolStr::to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":550,"record_offset":6304,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":34222,"source_line":94,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":5528,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":5528,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":5528,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":7344,"parent_record_offset":7296,"ancestor_record_offset":5528,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":7784,"parent_record_offset":7752,"ancestor_record_offset":5528,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":7204,"parent_record_offset":7164,"ancestor_record_offset":5528,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":5528,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":7808,"parent_record_offset":7784,"ancestor_record_offset":5528,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":7296,"parent_record_offset":7244,"ancestor_record_offset":5528,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":7244,"parent_record_offset":7204,"ancestor_record_offset":5528,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":7456,"parent_record_offset":7296,"ancestor_record_offset":5528,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":550,"record_offset":7036,"parent_record_offset":6996,"ancestor_record_offset":5528,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":550,"record_offset":7076,"parent_record_offset":7036,"ancestor_record_offset":5528,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":550,"record_offset":7116,"parent_record_offset":7076,"ancestor_record_offset":5528,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":550,"record_offset":6996,"parent_record_offset":6956,"ancestor_record_offset":5528,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":550,"record_offset":6956,"parent_record_offset":6916,"ancestor_record_offset":5528,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":550,"record_offset":6876,"parent_record_offset":6340,"ancestor_record_offset":5528,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":550,"record_offset":6916,"parent_record_offset":6876,"ancestor_record_offset":5528,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":550,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":5528,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":550,"record_offset":6644,"parent_record_offset":6428,"ancestor_record_offset":5528,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":7752,"parent_record_offset":7704,"ancestor_record_offset":5528,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":7704,"parent_record_offset":7676,"ancestor_record_offset":5528,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":7164,"parent_record_offset":7116,"ancestor_record_offset":5528,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":550,"record_offset":7480,"parent_record_offset":7456,"ancestor_record_offset":5528,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":550,"record_offset":6484,"parent_record_offset":6428,"ancestor_record_offset":5528,"inlinee":11987,"source_line":389,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_control
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":550,"record_offset":5996,"parent_record_offset":5960,"ancestor_record_offset":5528,"inlinee":34221,"source_line":952,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":550,"record_offset":7528,"parent_record_offset":7504,"ancestor_record_offset":5528,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":550,"record_offset":5924,"parent_record_offset":5892,"ancestor_record_offset":5528,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":550,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":5528,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":550,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":5528,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":550,"record_offset":6600,"parent_record_offset":6568,"ancestor_record_offset":5528,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":550,"record_offset":5960,"parent_record_offset":5924,"ancestor_record_offset":5528,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":550,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":5528,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":550,"record_offset":7584,"parent_record_offset":7116,"ancestor_record_offset":5528,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":5528,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":550,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":5528,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":550,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":5528,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":550,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":5528,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":7872,"parent_record_offset":7704,"ancestor_record_offset":5528,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":550,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":5528,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":550,"record_offset":6512,"parent_record_offset":6484,"ancestor_record_offset":5528,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":550,"record_offset":6568,"parent_record_offset":6540,"ancestor_record_offset":5528,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":550,"record_offset":6540,"parent_record_offset":6512,"ancestor_record_offset":5528,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":550,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":5528,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":550,"record_offset":6120,"parent_record_offset":6068,"ancestor_record_offset":5528,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":550,"record_offset":6044,"parent_record_offset":5892,"ancestor_record_offset":5528,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":550,"record_offset":6728,"parent_record_offset":6428,"ancestor_record_offset":5528,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":550,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":5528,"inlinee":15555,"source_line":161,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":550,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":5528,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":550,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":5528,"inlinee":15548,"source_line":237,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":550,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":5528,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":550,"record_offset":6204,"parent_record_offset":6068,"ancestor_record_offset":5528,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":550,"record_offset":6176,"parent_record_offset":6068,"ancestor_record_offset":5528,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::Repr::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":550,"record_offset":6428,"parent_record_offset":6340,"ancestor_record_offset":5528,"inlinee":19239,"source_line":480,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::SmolStr::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":550,"record_offset":6340,"parent_record_offset":6304,"ancestor_record_offset":5528,"inlinee":19238,"source_line":89,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::EguiInputResource::on_mouse_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":5032,"rva":11277808,"va":5379986928}
// 编译实例 RVAs: 0xac15f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36363436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379986928,"raw_hex":"e8acdc2a02","size":5,"soft_fail":false,"target_va":5416350496,"va":5379987055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<egui::data::input::Event> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379986928,"raw_hex":"e8e7480000","size":5,"soft_fail":false,"target_va":5380005808,"va":5379987140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":5188,"parent_record_offset":5032,"ancestor_record_offset":5032,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":5300,"parent_record_offset":5268,"ancestor_record_offset":5032,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":5032,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":5032,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":5268,"parent_record_offset":5220,"ancestor_record_offset":5032,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":5220,"parent_record_offset":5188,"ancestor_record_offset":5032,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":5388,"parent_record_offset":5220,"ancestor_record_offset":5032,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::EguiInputResource::on_mouse_wheel
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":4024,"rva":11277152,"va":5379986272}
// 编译实例 RVAs: 0xac1360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_input.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36364121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379986272,"raw_hex":"e859df2a02","size":5,"soft_fail":false,"target_va":5416350496,"va":5379986370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<egui::data::input::Event> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379986272,"raw_hex":"e8cd4a0000","size":5,"soft_fail":false,"target_va":5380005808,"va":5379986654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":4176,"parent_record_offset":4024,"ancestor_record_offset":4024,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":4284,"parent_record_offset":4252,"ancestor_record_offset":4024,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":4024,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":4024,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":4252,"parent_record_offset":4204,"ancestor_record_offset":4024,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":4024,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":4372,"parent_record_offset":4204,"ancestor_record_offset":4024,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::update_egui_input
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D53
// 首条位置: {"module":531,"inlinee":32083}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":531,"record_offset":41636,"parent_record_offset":41604,"ancestor_record_offset":41604,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":531,"record_offset":41660,"parent_record_offset":41636,"ancestor_record_offset":41604,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::bevy_to_egui_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x85AF
// 首条位置: {"module":550,"inlinee":34223}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] smol_str::SmolStr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":550,"record_offset":8064,"parent_record_offset":7908,"ancestor_record_offset":7908,"inlinee":19238,"source_line":89,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":550,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":7908,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":550,"record_offset":8320,"parent_record_offset":8104,"ancestor_record_offset":7908,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":550,"record_offset":8160,"parent_record_offset":8104,"ancestor_record_offset":7908,"inlinee":11987,"source_line":389,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":550,"record_offset":8276,"parent_record_offset":8244,"ancestor_record_offset":7908,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":550,"record_offset":8504,"parent_record_offset":8476,"ancestor_record_offset":7908,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":550,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":7908,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":550,"record_offset":8188,"parent_record_offset":8160,"ancestor_record_offset":7908,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":550,"record_offset":8244,"parent_record_offset":8216,"ancestor_record_offset":7908,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":550,"record_offset":8216,"parent_record_offset":8188,"ancestor_record_offset":7908,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":550,"record_offset":8404,"parent_record_offset":8104,"ancestor_record_offset":7908,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":550,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":7908,"inlinee":15555,"source_line":161,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":550,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":7908,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":550,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":7908,"inlinee":15548,"source_line":237,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::Repr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":550,"record_offset":8104,"parent_record_offset":8064,"ancestor_record_offset":7908,"inlinee":19239,"source_line":480,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::egui_input::bevy_to_egui_physical_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x85B0
// 首条位置: {"module":550,"inlinee":34224}
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
// 完整结构及原始类型记录: egui_input.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::egui_input::EguiInputPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::egui_input::EguiInputPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::egui_input::EguiInputPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::egui_input::EguiInputPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::egui_input::EguiInputPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::egui_input::EguiInputPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::egui_input::EguiInputPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::egui_input::EguiInputPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::egui_input::EguiInputPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::egui_input::EguiInputPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::egui_input::EguiInputPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::systems::egui_input::EguiInputPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::systems::egui_input::EguiInputPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::systems::egui_input::EguiInputPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::systems::egui_input::EguiInputPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::systems::egui_input::EguiInputPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::systems::egui_input::EguiInputPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::mod_editor::ModEditorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::startup::load_glade_tod_and_weather::SnowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::egui_input::EguiInputPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::grass2::Grass2Plugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::mod_editor::ModEditorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::startup::load_glade_tod_and_weather::SnowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::egui_input::EguiInputPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::grass2::Grass2Plugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::egui_input::EguiInputResource
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 31
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>", "constructor": "bevy_ecs::change_detection::ResMut", "arguments": ["country_core::systems::egui_input::EguiInputResource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$8::init_state<country_core::systems::egui_input::EguiInputResource>", "constructor": "bevy_ecs::system::system_param::impl$8::init_state", "arguments": ["country_core::systems::egui_input::EguiInputResource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::systems::egui_input::EguiInputResource>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::systems::egui_input::EguiInputResource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr<country_core::systems::egui_input::EguiInputResource>", "constructor": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr", "arguments": ["country_core::systems::egui_input::EguiInputResource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: egui_input.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: egui_input.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 282; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::EguiInputResource::on_cursor_moved
// 信息 ID: d3f1c6fad528cb010f32255c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::on_cursor_moved_absolute @ 0x578eee -> 0xac1540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::EguiInputResource::on_keyboard_input
// 信息 ID: ca122651a3fa645e15c3fc34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::EguiInputResource::on_mouse_button
// 信息 ID: e66ecacb0ee0532bf735cdb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::EguiInputResource::on_mouse_wheel
// 信息 ID: 2b9a24c43e3dee2124552fa6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::bevy_to_egui_key
// 信息 ID: 862017a6333c0ae693a47844
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::egui_input::EguiInputResource::on_keyboard_input; module=550, inline_record=7908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::bevy_to_egui_physical_key
// 信息 ID: 2e29909b932da63048f1afea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::egui_input::EguiInputResource::on_keyboard_input; module=550, inline_record=8560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::impl$0::build
// 信息 ID: c5d18c3f2a93ea41d21cd095
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::impl$3::default
// 信息 ID: 1675ae98ac2ac9e9102837fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::egui_input::impl$0::build; module=550, inline_record=62692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::egui_input::update_egui_input
// 信息 ID: fcb3b7c232925f8cd10137c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 854cdc0663e84630276277f8: void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)>; module=531, inline_record=41604; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::egui_input::EguiInputResource>)> >; module=1698, inline_record=3057996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
