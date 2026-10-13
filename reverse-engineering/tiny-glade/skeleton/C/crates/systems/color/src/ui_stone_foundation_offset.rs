// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/ui_stone_foundation_offset.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 2ae6be75f2570c04aaad36e1b263e3b4a6522c1b5a7bffbcf11edb734675eb0a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_stone_foundation_offset.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1127, 2689, 2704, 3991, 4023, 4782, 5023
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A70A
// 首条位置: {"module":2704,"inlinee":108298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":5696,"parent_record_offset":5664,"ancestor_record_offset":5664,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":5664,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":5664,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":5664,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6100,"parent_record_offset":6044,"ancestor_record_offset":5664,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":5664,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":5664,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":5664,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5664,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5664,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":5664,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":5948,"parent_record_offset":5880,"ancestor_record_offset":5664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2704,"record_offset":6612,"parent_record_offset":6552,"ancestor_record_offset":5664,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2704,"record_offset":6584,"parent_record_offset":6552,"ancestor_record_offset":5664,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2704,"record_offset":6520,"parent_record_offset":5728,"ancestor_record_offset":5664,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2704,"record_offset":6552,"parent_record_offset":6520,"ancestor_record_offset":5664,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::WidgetId::from_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":5880,"parent_record_offset":5728,"ancestor_record_offset":5664,"inlinee":26801,"source_line":21,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":5664,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$1::to_widget_name_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":5920,"parent_record_offset":5880,"ancestor_record_offset":5664,"inlinee":26803,"source_line":56,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":5728,"parent_record_offset":5696,"ancestor_record_offset":5664,"inlinee":108300,"source_line":61,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":6644,"parent_record_offset":6520,"ancestor_record_offset":5664,"inlinee":108303,"source_line":66,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::stone_foundation_offset_widget_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":5768,"parent_record_offset":5728,"ancestor_record_offset":5664,"inlinee":108301,"source_line":79,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A70C
// 首条位置: {"module":2704,"inlinee":108300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2704,"record_offset":6520,"parent_record_offset":5728,"ancestor_record_offset":5728,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::WidgetId::from_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":5880,"parent_record_offset":5728,"ancestor_record_offset":5728,"inlinee":26801,"source_line":21,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] system_color::ui_stone_foundation_offset::stone_foundation_offset_widget_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":5768,"parent_record_offset":5728,"ancestor_record_offset":5728,"inlinee":108301,"source_line":79,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":5728,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":5728,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":5728,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6100,"parent_record_offset":6044,"ancestor_record_offset":5728,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":5728,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":5728,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":5728,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5728,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5728,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":5728,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":5948,"parent_record_offset":5880,"ancestor_record_offset":5728,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2704,"record_offset":6612,"parent_record_offset":6552,"ancestor_record_offset":5728,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2704,"record_offset":6584,"parent_record_offset":6552,"ancestor_record_offset":5728,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2704,"record_offset":6552,"parent_record_offset":6520,"ancestor_record_offset":5728,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":5728,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$1::to_widget_name_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":5920,"parent_record_offset":5880,"ancestor_record_offset":5728,"inlinee":26803,"source_line":56,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":6644,"parent_record_offset":6520,"ancestor_record_offset":5728,"inlinee":108303,"source_line":66,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A70F
// 首条位置: {"module":2704,"inlinee":108303}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":6644,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::impl$0::default
// 记录形式: inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1063A
// 首条位置: {"module":1127,"inlinee":67130}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::stone_foundation_offset_widget_name
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A70D
// 首条位置: {"module":2704,"inlinee":108301}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":5768,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":5768,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":5768,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset
// 记录形式: compiled_procedure; instances=1
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2704,"record_offset":1600,"rva":35059520,"va":5403768640}
// 编译实例 RVAs: 0x216f740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-28093728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8e05253fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403772411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t-28090867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e80d5e53fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403769566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=7
//     target_aliases=1699 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35050209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e81f2de9fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403772396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34538859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e895faf0fd","size":5,"soft_fail":false,"target_va":5369240528,"va":5403779382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28143519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8619052fe","size":5,"soft_fail":false,"target_va":5375626064,"va":5403769578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727090
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27563776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e800695bfe","size":5,"soft_fail":false,"target_va":5376209040,"va":5403772811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25652473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8079378fe","size":5,"soft_fail":false,"target_va":5378120480,"va":5403772948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9c50
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25654095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8b18c78fe","size":5,"soft_fail":false,"target_va":5378120784,"va":5403774874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140994b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25022465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8ff2f82fe","size":5,"soft_fail":false,"target_va":5378755328,"va":5403777788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a3e90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24959773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8e32483fe","size":5,"soft_fail":false,"target_va":5378817680,"va":5403777448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a4d290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24259083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8f5d58dfe","size":5,"soft_fail":false,"target_va":5379510928,"va":5403770006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22705361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e82f8ba5fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5403769532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20446784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8c001c8fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5403771835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410a1870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17632052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8ccf4f2fe","size":5,"soft_fail":false,"target_va":5386147952,"va":5403779999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421151f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-372402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e84e51faff","size":5,"soft_fail":false,"target_va":5403398640,"va":5403771037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142894170
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7486454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8f63b7200","size":5,"soft_fail":false,"target_va":5411258736,"va":5403772277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b2f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7614213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8052f7400","size":5,"soft_fail":false,"target_va":5411385168,"va":5403770950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12574570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e86adfbf00","size":5,"soft_fail":false,"target_va":5416350944,"va":5403776369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13282907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e85baeca00","size":5,"soft_fail":false,"target_va":5417057056,"va":5403774144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t12844172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e88cfcc300","size":5,"soft_fail":false,"target_va":5416620769,"va":5403776592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-18974626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e85e78defe","size":5,"soft_fail":false,"target_va":5384794864,"va":5403769485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12844271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8effcc300","size":5,"soft_fail":false,"target_va":5416620794,"va":5403776518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t12845052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8fcffc300","size":5,"soft_fail":false,"target_va":5416621600,"va":5403776543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23223343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8d1a39dfe","size":5,"soft_fail":false,"target_va":5380552224,"va":5403775562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22944329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8b7e5a1fe","size":5,"soft_fail":false,"target_va":5380831168,"va":5403775492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22573311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8018fa7fe","size":5,"soft_fail":false,"target_va":5381197456,"va":5403770762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::clamp_stone_foundation_offset
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22543315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e82d04a8fe","size":5,"soft_fail":false,"target_va":5381232912,"va":5403776222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::derived_foundation_base_y
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22538491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e80517a8fe","size":5,"soft_fail":false,"target_va":5381232336,"va":5403770822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22541098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8d60ca8fe","size":5,"soft_fail":false,"target_va":5381231904,"va":5403772997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::premade_wooden_wall_outer_shape
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22535424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e80023a8fe","size":5,"soft_fail":false,"target_va":5381233968,"va":5403769387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::stone_foundation_offset_range
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22539173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e85b14a8fe","size":5,"soft_fail":false,"target_va":5381233184,"va":5403772352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::stone_foundation_transition_y
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22537310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8a21ba8fe","size":5,"soft_fail":false,"target_va":5381233488,"va":5403770793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22865832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e85818a3fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5403769363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_component
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22867866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e86610a3fe","size":5,"soft_fail":false,"target_va":5380902880,"va":5403770741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22015236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8fc12b0fe","size":5,"soft_fail":false,"target_va":5381756080,"va":5403771311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24454416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8f0da8afe","size":5,"soft_fail":false,"target_va":5379317488,"va":5403771899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25644441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e867b278fe","size":5,"soft_fail":false,"target_va":5378126128,"va":5403770564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25652097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e87f9478fe","size":5,"soft_fail":false,"target_va":5378120816,"va":5403772908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25652632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8689278fe","size":5,"soft_fail":false,"target_va":5378120848,"va":5403773475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::origin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25655178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8768878fe","size":5,"soft_fail":false,"target_va":5378120576,"va":5403775749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::boop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23746857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8d7a695fe","size":5,"soft_fail":false,"target_va":5380025296,"va":5403772148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23747068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e804a695fe","size":5,"soft_fail":false,"target_va":5380025136,"va":5403772199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23747346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8eea495fe","size":5,"soft_fail":false,"target_va":5380024784,"va":5403772125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23747676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8a4a395fe","size":5,"soft_fail":false,"target_va":5380024576,"va":5403772247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23747334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8faa495fe","size":5,"soft_fail":false,"target_va":5380024896,"va":5403772225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24838104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8280085fe","size":5,"soft_fail":false,"target_va":5378935744,"va":5403773843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23561862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e87a7998fe","size":5,"soft_fail":false,"target_va":5380212048,"va":5403773905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::owned_by
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23560817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e88f7d98fe","size":5,"soft_fail":false,"target_va":5380212032,"va":5403772844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23562837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8ab7598fe","size":5,"soft_fail":false,"target_va":5380211968,"va":5403774800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24173537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e81f248ffe","size":5,"soft_fail":false,"target_va":5379597392,"va":5403770924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::expand
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24173622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8ca238ffe","size":5,"soft_fail":false,"target_va":5379597280,"va":5403770897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::dashed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24094169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8275a90fe","size":5,"soft_fail":false,"target_va":5379679152,"va":5403773316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24093820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8845b90fe","size":5,"soft_fail":false,"target_va":5379678976,"va":5403772791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24093723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8e55b90fe","size":5,"soft_fail":false,"target_va":5379679024,"va":5403772742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24093672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8185c90fe","size":5,"soft_fail":false,"target_va":5379679200,"va":5403772867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22574512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8508aa7fe","size":5,"soft_fail":false,"target_va":5381201344,"va":5403775851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22574506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8568aa7fe","size":5,"soft_fail":false,"target_va":5381201392,"va":5403775893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22574664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8b889a7fe","size":5,"soft_fail":false,"target_va":5381201472,"va":5403776131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::world_space_to_screen_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24732995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8bd9a86fe","size":5,"soft_fail":false,"target_va":5379038368,"va":5403771358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20491544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768640,"raw_hex":"e8e852c7fe","size":5,"soft_fail":false,"target_va":5383280144,"va":5403771683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":10652,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":10300,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":11168,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2704,"record_offset":11280,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":11788,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":10736,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2704,"record_offset":14996,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2704,"record_offset":14688,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":2704,"record_offset":14136,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2704,"record_offset":1888,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":108292,"source_line":2000,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2704,"record_offset":2540,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2704,"record_offset":7920,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2704,"record_offset":10540,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2704,"record_offset":11252,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":2704,"record_offset":6716,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":10792,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":3004,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":9348,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$56::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":2704,"record_offset":10484,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":32763,"source_line":38,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":7832,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":7512,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":9884,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":14880,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":7892,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":7548,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::WidgetId::from_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":7428,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":26801,"source_line":21,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2704,"record_offset":9972,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2704,"record_offset":10188,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2704,"record_offset":10244,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2704,"record_offset":9916,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":5664,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":108298,"source_line":60,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] system_color::ui_stone_foundation_offset::show_foundation_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":12520,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":108305,"source_line":317,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":7952,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":108304,"source_line":394,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] system_color::ui_stone_foundation_offset::stone_foundation_offset_widget_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":7316,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":108301,"source_line":79,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] system_color::ui_stone_foundation_offset::update_stone_foundation_offset_drag
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":16032,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":108307,"source_line":343,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":1600,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":10408,"parent_record_offset":10384,"ancestor_record_offset":1600,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":1600,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":1600,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":1600,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":1600,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":10360,"parent_record_offset":10328,"ancestor_record_offset":1600,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":6100,"parent_record_offset":6044,"ancestor_record_offset":1600,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":10328,"parent_record_offset":10300,"ancestor_record_offset":1600,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":11308,"parent_record_offset":11280,"ancestor_record_offset":1600,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":16848,"parent_record_offset":16824,"ancestor_record_offset":1600,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":1600,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":1600,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":1600,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":12084,"parent_record_offset":12056,"ancestor_record_offset":1600,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":1600,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":1600,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":1600,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":1600,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":12144,"parent_record_offset":12112,"ancestor_record_offset":1600,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":12112,"parent_record_offset":12084,"ancestor_record_offset":1600,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":1600,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":12180,"parent_record_offset":12144,"ancestor_record_offset":1600,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":1600,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":12008,"parent_record_offset":11824,"ancestor_record_offset":1600,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2704,"record_offset":12696,"parent_record_offset":12520,"ancestor_record_offset":1600,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":16332,"parent_record_offset":16292,"ancestor_record_offset":1600,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":1600,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":1600,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":1600,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":1600,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":1600,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":16264,"parent_record_offset":16228,"ancestor_record_offset":1600,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":16292,"parent_record_offset":16264,"ancestor_record_offset":1600,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":12056,"parent_record_offset":12008,"ancestor_record_offset":1600,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":1600,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":8008,"parent_record_offset":7952,"ancestor_record_offset":1600,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":2028,"parent_record_offset":2000,"ancestor_record_offset":1600,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2704,"record_offset":4980,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2704,"record_offset":4272,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2704,"record_offset":4356,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2704,"record_offset":5232,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2704,"record_offset":2444,"parent_record_offset":2208,"ancestor_record_offset":1600,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2704,"record_offset":15020,"parent_record_offset":14996,"ancestor_record_offset":1600,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2704,"record_offset":16168,"parent_record_offset":16144,"ancestor_record_offset":1600,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2704,"record_offset":14232,"parent_record_offset":14200,"ancestor_record_offset":1600,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2704,"record_offset":14720,"parent_record_offset":14688,"ancestor_record_offset":1600,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":2704,"record_offset":14168,"parent_record_offset":14136,"ancestor_record_offset":1600,"inlinee":9341,"source_line":79,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":2704,"record_offset":14200,"parent_record_offset":14168,"ancestor_record_offset":1600,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2704,"record_offset":16144,"parent_record_offset":16032,"ancestor_record_offset":1600,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2704,"record_offset":16228,"parent_record_offset":16168,"ancestor_record_offset":1600,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2704,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":1600,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2704,"record_offset":4448,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2704,"record_offset":3452,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2704,"record_offset":3956,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2704,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":1600,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2704,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1600,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2704,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1600,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2704,"record_offset":2208,"parent_record_offset":1912,"ancestor_record_offset":1600,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2704,"record_offset":3076,"parent_record_offset":3036,"ancestor_record_offset":1600,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2704,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":1600,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2704,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":1600,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2704,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":1600,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2704,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":1600,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2704,"record_offset":3504,"parent_record_offset":3476,"ancestor_record_offset":1600,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2704,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":1600,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2704,"record_offset":3368,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2704,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":1600,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2704,"record_offset":2304,"parent_record_offset":2208,"ancestor_record_offset":1600,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2704,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":1600,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2704,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":1600,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_nop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2704,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1600,"inlinee":108293,"source_line":552,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2704,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":1600,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2704,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":1600,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2704,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":1600,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2704,"record_offset":2604,"parent_record_offset":2568,"ancestor_record_offset":1600,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2704,"record_offset":1952,"parent_record_offset":1912,"ancestor_record_offset":1600,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2704,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":1600,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2704,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":1600,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2704,"record_offset":9808,"parent_record_offset":9636,"ancestor_record_offset":1600,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2704,"record_offset":9636,"parent_record_offset":9612,"ancestor_record_offset":1600,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2704,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":1600,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2704,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":1600,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2704,"record_offset":9768,"parent_record_offset":9688,"ancestor_record_offset":1600,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2704,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":1600,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2704,"record_offset":12668,"parent_record_offset":12520,"ancestor_record_offset":1600,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2704,"record_offset":12284,"parent_record_offset":12228,"ancestor_record_offset":1600,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2704,"record_offset":14756,"parent_record_offset":14720,"ancestor_record_offset":1600,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2704,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":1600,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":8064,"parent_record_offset":7952,"ancestor_record_offset":1600,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":5444,"parent_record_offset":3036,"ancestor_record_offset":1600,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":9612,"parent_record_offset":9456,"ancestor_record_offset":1600,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":9372,"parent_record_offset":9348,"ancestor_record_offset":1600,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":3036,"parent_record_offset":3004,"ancestor_record_offset":1600,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":12228,"parent_record_offset":12204,"ancestor_record_offset":1600,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":1600,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":12256,"parent_record_offset":12228,"ancestor_record_offset":1600,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":14280,"parent_record_offset":14232,"ancestor_record_offset":1600,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":9844,"parent_record_offset":9456,"ancestor_record_offset":1600,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":8280,"parent_record_offset":8248,"ancestor_record_offset":1600,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2704,"record_offset":6996,"parent_record_offset":6972,"ancestor_record_offset":1600,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2704,"record_offset":6972,"parent_record_offset":6948,"ancestor_record_offset":1600,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2704,"record_offset":10268,"parent_record_offset":10244,"ancestor_record_offset":1600,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2704,"record_offset":10152,"parent_record_offset":9996,"ancestor_record_offset":1600,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2704,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":1600,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2704,"record_offset":3644,"parent_record_offset":3580,"ancestor_record_offset":1600,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2704,"record_offset":9664,"parent_record_offset":9636,"ancestor_record_offset":1600,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2704,"record_offset":16116,"parent_record_offset":16032,"ancestor_record_offset":1600,"inlinee":8913,"source_line":390,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":14784,"parent_record_offset":14756,"ancestor_record_offset":1600,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":1600,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":7284,"parent_record_offset":6716,"ancestor_record_offset":1600,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":5696,"parent_record_offset":5664,"ancestor_record_offset":1600,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":3708,"parent_record_offset":3528,"ancestor_record_offset":1600,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":1600,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":14952,"parent_record_offset":14924,"ancestor_record_offset":1600,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":5540,"parent_record_offset":5508,"ancestor_record_offset":1600,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2704,"record_offset":7180,"parent_record_offset":6844,"ancestor_record_offset":1600,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2704,"record_offset":12452,"parent_record_offset":12008,"ancestor_record_offset":1600,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2704,"record_offset":9576,"parent_record_offset":9552,"ancestor_record_offset":1600,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":12476,"parent_record_offset":12452,"ancestor_record_offset":1600,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":5948,"parent_record_offset":5880,"ancestor_record_offset":1600,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2704,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":1600,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2704,"record_offset":9552,"parent_record_offset":9528,"ancestor_record_offset":1600,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2704,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":1600,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2704,"record_offset":3252,"parent_record_offset":3192,"ancestor_record_offset":1600,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2704,"record_offset":9528,"parent_record_offset":9456,"ancestor_record_offset":1600,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2704,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":1600,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":16452,"parent_record_offset":16292,"ancestor_record_offset":1600,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2704,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":1600,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2704,"record_offset":5176,"parent_record_offset":3108,"ancestor_record_offset":1600,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2704,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":1600,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2704,"record_offset":3788,"parent_record_offset":3756,"ancestor_record_offset":1600,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2704,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":1600,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2704,"record_offset":14416,"parent_record_offset":14392,"ancestor_record_offset":1600,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2704,"record_offset":2928,"parent_record_offset":2752,"ancestor_record_offset":1600,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2704,"record_offset":6520,"parent_record_offset":5728,"ancestor_record_offset":1600,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2704,"record_offset":9456,"parent_record_offset":9372,"ancestor_record_offset":1600,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2704,"record_offset":2248,"parent_record_offset":2208,"ancestor_record_offset":1600,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::WidgetId::from_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":5880,"parent_record_offset":5728,"ancestor_record_offset":1600,"inlinee":26801,"source_line":21,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":1600,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$1::to_widget_name_hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2704,"record_offset":5920,"parent_record_offset":5880,"ancestor_record_offset":1600,"inlinee":26803,"source_line":56,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2704,"record_offset":10056,"parent_record_offset":10024,"ancestor_record_offset":1600,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2704,"record_offset":10024,"parent_record_offset":9996,"ancestor_record_offset":1600,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2704,"record_offset":9996,"parent_record_offset":9972,"ancestor_record_offset":1600,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2704,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":1600,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2704,"record_offset":6948,"parent_record_offset":6916,"ancestor_record_offset":1600,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2704,"record_offset":7036,"parent_record_offset":6884,"ancestor_record_offset":1600,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2704,"record_offset":6884,"parent_record_offset":6844,"ancestor_record_offset":1600,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2704,"record_offset":7068,"parent_record_offset":6844,"ancestor_record_offset":1600,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2704,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":1600,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2704,"record_offset":7124,"parent_record_offset":6844,"ancestor_record_offset":1600,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2704,"record_offset":6844,"parent_record_offset":6812,"ancestor_record_offset":1600,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2704,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":1600,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2704,"record_offset":5364,"parent_record_offset":5332,"ancestor_record_offset":1600,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2704,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":1600,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2704,"record_offset":6764,"parent_record_offset":6740,"ancestor_record_offset":1600,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$49::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2704,"record_offset":6740,"parent_record_offset":6716,"ancestor_record_offset":1600,"inlinee":11018,"source_line":2201,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2704,"record_offset":10088,"parent_record_offset":10056,"ancestor_record_offset":1600,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::radial_menu::ColorRadialMenu::foundation_wall_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/radial_menu/mod.rs
//     evidence={"module":2704,"record_offset":5508,"parent_record_offset":5476,"ancestor_record_offset":1600,"inlinee":108296,"source_line":391,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":5728,"parent_record_offset":5696,"ancestor_record_offset":1600,"inlinee":108300,"source_line":61,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":6644,"parent_record_offset":6520,"ancestor_record_offset":1600,"inlinee":108303,"source_line":66,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::stone_foundation_offset_widget_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":5768,"parent_record_offset":5728,"ancestor_record_offset":1600,"inlinee":108301,"source_line":79,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":5476,"parent_record_offset":5444,"ancestor_record_offset":1600,"inlinee":108294,"source_line":120,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":9688,"parent_record_offset":9664,"ancestor_record_offset":1600,"inlinee":67132,"source_line":168,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":1600,"inlinee":67134,"source_line":169,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":14924,"parent_record_offset":14880,"ancestor_record_offset":1600,"inlinee":108306,"source_line":284,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A706
// 首条位置: {"module":2704,"inlinee":108294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_color::radial_menu::ColorRadialMenu::foundation_wall_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/radial_menu/mod.rs
//     evidence={"module":2704,"record_offset":5508,"parent_record_offset":5476,"ancestor_record_offset":5476,"inlinee":108296,"source_line":391,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":5540,"parent_record_offset":5508,"ancestor_record_offset":5476,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1063C
// 首条位置: {"module":1127,"inlinee":67132}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2704,"record_offset":9768,"parent_record_offset":9688,"ancestor_record_offset":9688,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_stone_foundation_offset.rs
//     evidence={"module":2704,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":9688,"inlinee":67134,"source_line":169,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1063E
// 首条位置: {"module":1127,"inlinee":67134}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$3
// 记录形式: inlinee_declaration; instances=1
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x237D4
// 首条位置: {"module":4782,"inlinee":145364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2704,"record_offset":18756,"rva":35073520,"va":5403782640}
// 编译实例 RVAs: 0x2172df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28104016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e8b02a53fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403782699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28104365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e8532953fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403783064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35060497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e8ef04e9fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403782684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12837777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e891e3c300","size":5,"soft_fail":false,"target_va":5416620769,"va":5403782987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12837786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e89ae3c300","size":5,"soft_fail":false,"target_va":5416620794,"va":5403783003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24846622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e8e2de84fe","size":5,"soft_fail":false,"target_va":5378936272,"va":5403782889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24844190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e862e884fe","size":5,"soft_fail":false,"target_va":5378938736,"va":5403782921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24846464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e880df84fe","size":5,"soft_fail":false,"target_va":5378936352,"va":5403782811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24836105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e8f70785fe","size":5,"soft_fail":false,"target_va":5378946752,"va":5403782852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24836221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403782640,"raw_hex":"e8830785fe","size":5,"soft_fail":false,"target_va":5378946656,"va":5403782872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":19644,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":18932,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2704,"record_offset":19112,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":19728,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":19016,"parent_record_offset":18992,"ancestor_record_offset":18756,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":19040,"parent_record_offset":19016,"ancestor_record_offset":18756,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":19800,"parent_record_offset":19776,"ancestor_record_offset":18756,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":19824,"parent_record_offset":19800,"ancestor_record_offset":18756,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":19064,"parent_record_offset":19040,"ancestor_record_offset":18756,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":19848,"parent_record_offset":19824,"ancestor_record_offset":18756,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":18992,"parent_record_offset":18960,"ancestor_record_offset":18756,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":19776,"parent_record_offset":19752,"ancestor_record_offset":18756,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":18960,"parent_record_offset":18932,"ancestor_record_offset":18756,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":19752,"parent_record_offset":19728,"ancestor_record_offset":18756,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":19344,"parent_record_offset":19316,"ancestor_record_offset":18756,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":19404,"parent_record_offset":19372,"ancestor_record_offset":18756,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":19372,"parent_record_offset":19344,"ancestor_record_offset":18756,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":19668,"parent_record_offset":19644,"ancestor_record_offset":18756,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":19224,"parent_record_offset":19196,"ancestor_record_offset":18756,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":19252,"parent_record_offset":19224,"ancestor_record_offset":18756,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":19280,"parent_record_offset":19252,"ancestor_record_offset":18756,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":19196,"parent_record_offset":19168,"ancestor_record_offset":18756,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2704,"record_offset":19168,"parent_record_offset":19140,"ancestor_record_offset":18756,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2704,"record_offset":19140,"parent_record_offset":19112,"ancestor_record_offset":18756,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":19692,"parent_record_offset":19668,"ancestor_record_offset":18756,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":19316,"parent_record_offset":19280,"ancestor_record_offset":18756,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2704,"record_offset":19560,"parent_record_offset":19280,"ancestor_record_offset":18756,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":19584,"parent_record_offset":19560,"ancestor_record_offset":18756,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 284
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A712
// 首条位置: {"module":2704,"inlinee":108306}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":14952,"parent_record_offset":14924,"ancestor_record_offset":14924,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::show_foundation_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A711
// 首条位置: {"module":2704,"inlinee":108305}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":12584,"parent_record_offset":12520,"ancestor_record_offset":12520,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2704,"record_offset":12696,"parent_record_offset":12520,"ancestor_record_offset":12520,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2704,"record_offset":12668,"parent_record_offset":12520,"ancestor_record_offset":12520,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":13280,"parent_record_offset":12520,"ancestor_record_offset":12520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":12832,"parent_record_offset":12808,"ancestor_record_offset":12520,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":12856,"parent_record_offset":12832,"ancestor_record_offset":12520,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":13160,"parent_record_offset":13136,"ancestor_record_offset":12520,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":13184,"parent_record_offset":13160,"ancestor_record_offset":12520,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":12880,"parent_record_offset":12856,"ancestor_record_offset":12520,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":12520,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":12808,"parent_record_offset":12776,"ancestor_record_offset":12520,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":13136,"parent_record_offset":13048,"ancestor_record_offset":12520,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":12776,"parent_record_offset":12732,"ancestor_record_offset":12520,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":12732,"parent_record_offset":12696,"ancestor_record_offset":12520,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2704,"record_offset":12632,"parent_record_offset":12608,"ancestor_record_offset":12520,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":13076,"parent_record_offset":13048,"ancestor_record_offset":12520,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":13048,"parent_record_offset":13024,"ancestor_record_offset":12520,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":12520,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":13104,"parent_record_offset":13076,"ancestor_record_offset":12520,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2704,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":12520,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":12928,"parent_record_offset":12732,"ancestor_record_offset":12520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::show_foundation_curve::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2704,"record_offset":408,"rva":35059008,"va":5403768128}
// 编译实例 RVAs: 0x216f540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28089510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e85a6353fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403768193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28089907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8cd6153fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403768606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35045991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8993de9fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403768178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_stone_foundation_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13288720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e810c5ca00","size":5,"soft_fail":false,"target_va":5417057056,"va":5403768331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12852210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8f21bc400","size":5,"soft_fail":false,"target_va":5416620769,"va":5403768554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12852535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8371dc400","size":5,"soft_fail":false,"target_va":5416620794,"va":5403768254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24832120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8881785fe","size":5,"soft_fail":false,"target_va":5378936272,"va":5403768387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24829728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8e02085fe","size":5,"soft_fail":false,"target_va":5378938736,"va":5403768459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::disable_default_animation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24832086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8aa1785fe","size":5,"soft_fail":false,"target_va":5378936320,"va":5403768401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24832348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8a41685fe","size":5,"soft_fail":false,"target_va":5378936080,"va":5403768423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24832023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403768128,"raw_hex":"e8e91785fe","size":5,"soft_fail":false,"target_va":5378936352,"va":5403768370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":1344,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":580,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":760,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":1428,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":408,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":408,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":408,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":408,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":408,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":408,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":640,"parent_record_offset":608,"ancestor_record_offset":408,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":408,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":408,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2704,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":408,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":408,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":968,"parent_record_offset":936,"ancestor_record_offset":408,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":408,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":1004,"parent_record_offset":968,"ancestor_record_offset":408,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":408,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2704,"record_offset":852,"parent_record_offset":788,"ancestor_record_offset":408,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":824,"parent_record_offset":788,"ancestor_record_offset":408,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":408,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":888,"parent_record_offset":852,"ancestor_record_offset":408,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":408,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2704,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":408,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2704,"record_offset":1108,"parent_record_offset":1052,"ancestor_record_offset":408,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":408,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":1080,"parent_record_offset":1052,"ancestor_record_offset":408,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2704,"record_offset":1276,"parent_record_offset":852,"ancestor_record_offset":408,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":408,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::update_stone_foundation_offset_drag
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 343
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A713
// 首条位置: {"module":2704,"inlinee":108307}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2704,"record_offset":16144,"parent_record_offset":16032,"ancestor_record_offset":16032,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2704,"record_offset":16116,"parent_record_offset":16032,"ancestor_record_offset":16032,"inlinee":8913,"source_line":390,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":16364,"parent_record_offset":16332,"ancestor_record_offset":16032,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":16032,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":16388,"parent_record_offset":16364,"ancestor_record_offset":16032,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":16332,"parent_record_offset":16292,"ancestor_record_offset":16032,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":16264,"parent_record_offset":16228,"ancestor_record_offset":16032,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":16292,"parent_record_offset":16264,"ancestor_record_offset":16032,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2704,"record_offset":16200,"parent_record_offset":16168,"ancestor_record_offset":16032,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2704,"record_offset":16168,"parent_record_offset":16144,"ancestor_record_offset":16032,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2704,"record_offset":16228,"parent_record_offset":16168,"ancestor_record_offset":16032,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":16452,"parent_record_offset":16292,"ancestor_record_offset":16032,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A710
// 首条位置: {"module":2704,"inlinee":108304}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":8008,"parent_record_offset":7952,"ancestor_record_offset":7952,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2704,"record_offset":8064,"parent_record_offset":7952,"ancestor_record_offset":7952,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":8148,"parent_record_offset":7952,"ancestor_record_offset":7952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":7952,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":8360,"parent_record_offset":8336,"ancestor_record_offset":7952,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":8384,"parent_record_offset":8360,"ancestor_record_offset":7952,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2704,"record_offset":8312,"parent_record_offset":8220,"ancestor_record_offset":7952,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":8248,"parent_record_offset":8220,"ancestor_record_offset":7952,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":8220,"parent_record_offset":8196,"ancestor_record_offset":7952,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2704,"record_offset":8196,"parent_record_offset":8172,"ancestor_record_offset":7952,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2704,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":7952,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2704,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":7952,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2704,"record_offset":8280,"parent_record_offset":8248,"ancestor_record_offset":7952,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2704,"record_offset":8172,"parent_record_offset":8148,"ancestor_record_offset":7952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2704,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":7952,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 401
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A69B
// 首条位置: {"module":2689,"inlinee":108187}
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
// 完整结构及原始类型记录: ui_stone_foundation_offset.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 29
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$132::init_access<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History> >", "constructor": "bevy_ecs::system::system_param::impl$132::init_access", "arguments": ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>", "bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_stone_foundation_offset.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_stone_foundation_offset.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>", "bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::impl$0::default
// 信息 ID: c4d23e94999a046442f4f470
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor
// 信息 ID: 1f5b3737dc8490d51cc1a82c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=5664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0
// 信息 ID: 991ec200ccdb40f4ca2a4190
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=5728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor::closure$0::closure$0
// 信息 ID: b50c5e4c717447902545dc80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_stone_foundation_offset::selected_foundation_handle_under_cursor", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=6644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::show_foundation_curve
// 信息 ID: d31a2adbf6595d9b639982c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=12520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::show_foundation_curve::closure$0
// 信息 ID: 0d9bbf0f9273c9159dcc6adc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_stone_foundation_offset::show_foundation_curve", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_stone_foundation_offset::show_foundation_curve::closure_env$0 <- core::ops::function::FnOnce::call_once<system_color::ui_stone_foundation_offset::show_foundation_curve::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_stone_foundation_offset::show_foundation_curve::closure_env$0 <- core::ptr::drop_in_place<system_color::ui_stone_foundation_offset::show_foundation_curve::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_stone_foundation_offset::show_foundation_curve::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x2172fe3 -> 0x216f540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve
// 信息 ID: 19d503c1354a7076298b5a2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=7952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve::closure$0
// 信息 ID: b335b75692b8f10297bbe63b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,system_color::ui_stone_foundation_offset::stone_foundation_gizmo_curve::closure_env$0> >; module=2689, inline_record=4512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::stone_foundation_offset_widget_name
// 信息 ID: 7e76746a59577e43b9d45421
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=5768; 不是独立调用边。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=7316; 不是独立调用边。
// [PDB 内联宿主] system_color::autoclose_color_menu::autoclose_color_menu; module=4023, inline_record=9944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset
// 信息 ID: ff1271ae85d5194da79200e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 409fb396fbeb9250ee9678d0: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> @ 0x10b71c7 -> 0x216f740 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::system::system_param::Local<system_color::ui_stone_foundation_offset::StoneFoundationOffsetDragCache>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<glam::f32::vec3::Vec3> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> > @ 0x215d85b -> 0x216f740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$0
// 信息 ID: 6b7a0b3c21b8eaa66e12d317
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_stone_foundation_offset::ui_stone_foundation_offset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=5476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$2
// 信息 ID: 0f6d2de03640efbc444e7be4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_stone_foundation_offset::ui_stone_foundation_offset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=9688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$2::closure$0
// 信息 ID: c723a142d31761608c110d4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_stone_foundation_offset::ui_stone_foundation_offset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=9712; 不是独立调用边。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=9740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$3
// 信息 ID: c28d51b94eebebc872e64fb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_color::ui_stone_foundation_offset::ui_stone_foundation_offset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$4
// 信息 ID: 9b5e61cb61ec25ca21eb570e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_color::ui_stone_foundation_offset::ui_stone_foundation_offset", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure_env$4 <- core::ops::function::FnOnce::call_once<system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure_env$4,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure_env$4,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x2172fb0 -> 0x2172df0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::ui_stone_foundation_offset::closure$5
// 信息 ID: 7fc9115b50c13cafb08762fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_color::ui_stone_foundation_offset::ui_stone_foundation_offset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=14924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_stone_foundation_offset::update_stone_foundation_offset_drag
// 信息 ID: 1c7e02a9756c855cf7db52d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset; module=2704, inline_record=16032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
