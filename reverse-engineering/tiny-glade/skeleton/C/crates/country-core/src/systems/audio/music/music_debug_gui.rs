// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/audio/music/music_debug_gui.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f63d03bce65ff550fe85f9bbd912e69b161325a8f25cab3a1a490630322dfa2a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: music_debug_gui.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1762
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
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_debug_gui::music_debug_gui
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1762,"record_offset":45324,"rva":28034208,"va":5396743328}
// 编译实例 RVAs: 0x1abc4a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21064808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396743328,"raw_hex":"e89893befe","size":5,"soft_fail":false,"target_va":5375678688,"va":5396743491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: music_debug_gui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28021289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396743328,"raw_hex":"e8d76d54fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5396743476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e910
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: music_debug_gui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27385099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396743328,"raw_hex":"e8f5225efe","size":5,"soft_fail":false,"target_va":5369358608,"va":5396743702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c89cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: music_debug_gui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14886983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396743328,"raw_hex":"e8b9d71cff","size":5,"soft_fail":false,"target_va":5381856448,"va":5396743426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8a010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: music_debug_gui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14886274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396743328,"raw_hex":"e87eda1cff","size":5,"soft_fail":false,"target_va":5381857296,"va":5396743565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: music_debug_gui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20313580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396743328,"raw_hex":"e8ecf53501","size":5,"soft_fail":false,"target_va":5417057056,"va":5396743471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19877132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396743328,"raw_hex":"e80c4d2f01","size":5,"soft_fail":false,"target_va":5416620769,"va":5396743632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13971701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396743328,"raw_hex":"e80bcf2aff","size":5,"soft_fail":false,"target_va":5382771904,"va":5396743600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1762,"record_offset":45580,"parent_record_offset":45324,"ancestor_record_offset":45324,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$39::as_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1762,"record_offset":45496,"parent_record_offset":45324,"ancestor_record_offset":45324,"inlinee":35767,"source_line":443,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1762,"record_offset":45900,"parent_record_offset":45324,"ancestor_record_offset":45324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":1762,"record_offset":45636,"parent_record_offset":45324,"ancestor_record_offset":45324,"inlinee":14593,"source_line":81,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":1762,"record_offset":45664,"parent_record_offset":45324,"ancestor_record_offset":45324,"inlinee":7250,"source_line":365,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":45796,"parent_record_offset":45772,"ancestor_record_offset":45324,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":45820,"parent_record_offset":45796,"ancestor_record_offset":45324,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":45844,"parent_record_offset":45820,"ancestor_record_offset":45324,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":45772,"parent_record_offset":45736,"ancestor_record_offset":45324,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1762,"record_offset":45736,"parent_record_offset":45696,"ancestor_record_offset":45324,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1762,"record_offset":45696,"parent_record_offset":45664,"ancestor_record_offset":45324,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1762,"record_offset":46064,"parent_record_offset":46028,"ancestor_record_offset":45324,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1762,"record_offset":46028,"parent_record_offset":46000,"ancestor_record_offset":45324,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1762,"record_offset":45544,"parent_record_offset":45520,"ancestor_record_offset":45324,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1762,"record_offset":45520,"parent_record_offset":45496,"ancestor_record_offset":45324,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1762,"record_offset":45924,"parent_record_offset":45900,"ancestor_record_offset":45324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1762,"record_offset":46096,"parent_record_offset":46064,"ancestor_record_offset":45324,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1762,"record_offset":46124,"parent_record_offset":46064,"ancestor_record_offset":45324,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1762,"record_offset":46156,"parent_record_offset":46028,"ancestor_record_offset":45324,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_debug_gui::music_debug_gui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14EA0
// 首条位置: {"module":1762,"inlinee":85664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1762,"record_offset":73764,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1762,"record_offset":74240,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1762,"record_offset":75200,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1762,"record_offset":73988,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1762,"record_offset":74776,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":1762,"record_offset":77244,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":33507,"source_line":544,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::MusicPlayer::current_track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":1762,"record_offset":77360,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":34005,"source_line":232,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":1762,"record_offset":76208,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":1762,"record_offset":76104,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":1762,"record_offset":73844,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::separator
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":1762,"record_offset":77816,"parent_record_offset":73580,"ancestor_record_offset":73580,"inlinee":7345,"source_line":1559,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":74592,"parent_record_offset":74568,"ancestor_record_offset":73580,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":74616,"parent_record_offset":74592,"ancestor_record_offset":73580,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":75092,"parent_record_offset":75068,"ancestor_record_offset":73580,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":75116,"parent_record_offset":75092,"ancestor_record_offset":73580,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":74640,"parent_record_offset":74616,"ancestor_record_offset":73580,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":75140,"parent_record_offset":75116,"ancestor_record_offset":73580,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":74568,"parent_record_offset":74540,"ancestor_record_offset":73580,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1762,"record_offset":75068,"parent_record_offset":75012,"ancestor_record_offset":73580,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1762,"record_offset":73812,"parent_record_offset":73788,"ancestor_record_offset":73580,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1762,"record_offset":74480,"parent_record_offset":74452,"ancestor_record_offset":73580,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1762,"record_offset":75040,"parent_record_offset":75012,"ancestor_record_offset":73580,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1762,"record_offset":75012,"parent_record_offset":74988,"ancestor_record_offset":73580,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1762,"record_offset":74540,"parent_record_offset":74508,"ancestor_record_offset":73580,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1762,"record_offset":74508,"parent_record_offset":74480,"ancestor_record_offset":73580,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1762,"record_offset":74988,"parent_record_offset":74964,"ancestor_record_offset":73580,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1762,"record_offset":74352,"parent_record_offset":74304,"ancestor_record_offset":73580,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1762,"record_offset":74384,"parent_record_offset":74352,"ancestor_record_offset":73580,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1762,"record_offset":74416,"parent_record_offset":74384,"ancestor_record_offset":73580,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1762,"record_offset":74304,"parent_record_offset":74272,"ancestor_record_offset":73580,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1762,"record_offset":74272,"parent_record_offset":74240,"ancestor_record_offset":73580,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1762,"record_offset":74104,"parent_record_offset":74068,"ancestor_record_offset":73580,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1762,"record_offset":74068,"parent_record_offset":74040,"ancestor_record_offset":73580,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1762,"record_offset":74452,"parent_record_offset":74416,"ancestor_record_offset":73580,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1762,"record_offset":73896,"parent_record_offset":73868,"ancestor_record_offset":73580,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1762,"record_offset":77284,"parent_record_offset":77244,"ancestor_record_offset":73580,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1762,"record_offset":73788,"parent_record_offset":73764,"ancestor_record_offset":73580,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1762,"record_offset":74696,"parent_record_offset":74416,"ancestor_record_offset":73580,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1762,"record_offset":74720,"parent_record_offset":74696,"ancestor_record_offset":73580,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1762,"record_offset":74012,"parent_record_offset":73988,"ancestor_record_offset":73580,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1762,"record_offset":74136,"parent_record_offset":74104,"ancestor_record_offset":73580,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1762,"record_offset":77312,"parent_record_offset":77244,"ancestor_record_offset":73580,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1762,"record_offset":77388,"parent_record_offset":77360,"ancestor_record_offset":73580,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1762,"record_offset":74800,"parent_record_offset":74776,"ancestor_record_offset":73580,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1762,"record_offset":74164,"parent_record_offset":74104,"ancestor_record_offset":73580,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1762,"record_offset":74196,"parent_record_offset":74068,"ancestor_record_offset":73580,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":1762,"record_offset":76260,"parent_record_offset":76236,"ancestor_record_offset":73580,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":1762,"record_offset":76236,"parent_record_offset":76208,"ancestor_record_offset":73580,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":1762,"record_offset":73944,"parent_record_offset":73920,"ancestor_record_offset":73580,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":1762,"record_offset":73920,"parent_record_offset":73896,"ancestor_record_offset":73580,"inlinee":6019,"source_line":707,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":1762,"record_offset":76136,"parent_record_offset":76104,"ancestor_record_offset":73580,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":1762,"record_offset":73868,"parent_record_offset":73844,"ancestor_record_offset":73580,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/01762.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::separator::impl$0::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/separator.rs
//     evidence={"module":1762,"record_offset":77840,"parent_record_offset":77816,"ancestor_record_offset":73580,"inlinee":7347,"source_line":22,"archive":"_pdb/C/modules/01762.jsonl.gz"}
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
// 完整结构及原始类型记录: music_debug_gui.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: music_debug_gui.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: music_debug_gui.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_debug_gui::music_debug_gui
// 信息 ID: e5498577f61aaa4e19c0af95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] debf22a3c1f3fe13d5577fbc: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> @ 0x17ccfae -> 0x1abc4a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> > @ 0x1916ea8 -> 0x1abc4a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_debug_gui::music_debug_gui::closure$0
// 信息 ID: 0efe45c845c2395a1eb8913f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::audio::music::music_debug_gui::music_debug_gui", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::audio::music::music_debug_gui::music_debug_gui::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::audio::music::music_debug_gui::music_debug_gui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::audio::music::music_debug_gui::music_debug_gui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=1762, inline_record=73580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
