// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/ui_color_gates.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5f3ce908eb409b3dfcd68a88b34908d385f461597e97684251205c85e35f29a0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_gates.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3991, 4024
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4024,"record_offset":408,"rva":42653952,"va":5411363072}
// 编译实例 RVAs: 0x28ad900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-35685633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8ff7adffd","size":5,"soft_fail":false,"target_va":5375678688,"va":5411364316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-35685768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8787adffd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411364467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42642114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e83e5575fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5411364301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142883170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-174813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e82355fdff","size":5,"soft_fail":false,"target_va":5411189104,"va":5411363912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142894170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-105433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e82764feff","size":5,"soft_fail":false,"target_va":5411258736,"va":5411364164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4985065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8e9104c00","size":5,"soft_fail":false,"target_va":5416350496,"va":5411365426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4985741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e88d134c00","size":5,"soft_fail":false,"target_va":5416351056,"va":5411365310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4989074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e892204c00","size":5,"soft_fail":false,"target_va":5416353040,"va":5411363961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5692230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e846db5600","size":5,"soft_fail":false,"target_va":5417057056,"va":5411364821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5255226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e83a305000","size":5,"soft_fail":false,"target_va":5416620769,"va":5411365538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-26569178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e826966afe","size":5,"soft_fail":false,"target_va":5384794864,"va":5411364037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5255313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e891305000","size":5,"soft_fail":false,"target_va":5416620794,"va":5411365476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5256097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8a1335000","size":5,"soft_fail":false,"target_va":5416621600,"va":5411365498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32089864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8f85816fe","size":5,"soft_fail":false,"target_va":5379273888,"va":5411363747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::components::drawable::DrawableMeshData::as_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33273147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8c54a04fe","size":5,"soft_fail":false,"target_va":5378090576,"va":5411363718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29897193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e817ce37fe","size":5,"soft_fail":false,"target_va":5381466592,"va":5411363780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33244499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8adba04fe","size":5,"soft_fail":false,"target_va":5378120752,"va":5411365246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33243355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e825bf04fe","size":5,"soft_fail":false,"target_va":5378120848,"va":5411364198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31339358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8a2cc21fe","size":5,"soft_fail":false,"target_va":5380024784,"va":5411364137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32428642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e89e2d11fe","size":5,"soft_fail":false,"target_va":5378935744,"va":5411364381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31152473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411363072,"raw_hex":"e8a7a624fe","size":5,"soft_fail":false,"target_va":5380211968,"va":5411364436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4024,"record_offset":3932,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4024,"record_offset":4156,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":1868,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":6232,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":1896,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":3456,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4024,"record_offset":880,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4024,"record_offset":2100,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":4024,"record_offset":4016,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":4024,"record_offset":4748,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4024,"record_offset":3384,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4024,"record_offset":4380,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4024,"record_offset":4664,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4024,"record_offset":2008,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4024,"record_offset":596,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4024,"record_offset":1268,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":408,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":4300,"parent_record_offset":4276,"ancestor_record_offset":408,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":408,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":408,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":408,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":408,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":4252,"parent_record_offset":4220,"ancestor_record_offset":408,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":4532,"parent_record_offset":4476,"ancestor_record_offset":408,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":4220,"parent_record_offset":4184,"ancestor_record_offset":408,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":4184,"parent_record_offset":4156,"ancestor_record_offset":408,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4024,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":408,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":408,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":3572,"parent_record_offset":3544,"ancestor_record_offset":408,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":6416,"parent_record_offset":6388,"ancestor_record_offset":408,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":408,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":4504,"parent_record_offset":4476,"ancestor_record_offset":408,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":4476,"parent_record_offset":4452,"ancestor_record_offset":408,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":3640,"parent_record_offset":3604,"ancestor_record_offset":408,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":408,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":408,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":3604,"parent_record_offset":3572,"ancestor_record_offset":408,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":6476,"parent_record_offset":6444,"ancestor_record_offset":408,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":408,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":408,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":6512,"parent_record_offset":6476,"ancestor_record_offset":408,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4024,"record_offset":6340,"parent_record_offset":6268,"ancestor_record_offset":408,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":1072,"parent_record_offset":1028,"ancestor_record_offset":408,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":408,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":2652,"parent_record_offset":2620,"ancestor_record_offset":408,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":408,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":996,"parent_record_offset":952,"ancestor_record_offset":408,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":1028,"parent_record_offset":996,"ancestor_record_offset":408,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":3544,"parent_record_offset":3504,"ancestor_record_offset":408,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":6388,"parent_record_offset":6340,"ancestor_record_offset":408,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":408,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":4024,"record_offset":3892,"parent_record_offset":3504,"ancestor_record_offset":408,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":4024,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":408,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4024,"record_offset":2408,"parent_record_offset":2124,"ancestor_record_offset":408,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4024,"record_offset":1232,"parent_record_offset":908,"ancestor_record_offset":408,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4024,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":408,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4024,"record_offset":2172,"parent_record_offset":2124,"ancestor_record_offset":408,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4024,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":408,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4024,"record_offset":2312,"parent_record_offset":2280,"ancestor_record_offset":408,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4024,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":408,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4024,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":408,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4024,"record_offset":952,"parent_record_offset":908,"ancestor_record_offset":408,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4024,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":408,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4024,"record_offset":2828,"parent_record_offset":2124,"ancestor_record_offset":408,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4024,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":408,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4024,"record_offset":3200,"parent_record_offset":2124,"ancestor_record_offset":408,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4024,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":408,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4024,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":408,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4024,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":408,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4024,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":408,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4024,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":408,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4024,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":408,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4024,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":408,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4024,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":408,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4024,"record_offset":2596,"parent_record_offset":2124,"ancestor_record_offset":408,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4024,"record_offset":5972,"parent_record_offset":5948,"ancestor_record_offset":408,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4024,"record_offset":5948,"parent_record_offset":5924,"ancestor_record_offset":408,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4024,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":408,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4024,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":408,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4024,"record_offset":2444,"parent_record_offset":2408,"ancestor_record_offset":408,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4024,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":408,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4024,"record_offset":6616,"parent_record_offset":6560,"ancestor_record_offset":408,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4024,"record_offset":3836,"parent_record_offset":3504,"ancestor_record_offset":408,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4024,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":408,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4024,"record_offset":6560,"parent_record_offset":6536,"ancestor_record_offset":408,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4024,"record_offset":6588,"parent_record_offset":6560,"ancestor_record_offset":408,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4024,"record_offset":7372,"parent_record_offset":7344,"ancestor_record_offset":408,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4024,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":408,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4024,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":408,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4024,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":408,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4024,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":408,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4024,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":408,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4024,"record_offset":2280,"parent_record_offset":2196,"ancestor_record_offset":408,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4024,"record_offset":3060,"parent_record_offset":2924,"ancestor_record_offset":408,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4024,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":408,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4024,"record_offset":2380,"parent_record_offset":2124,"ancestor_record_offset":408,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4024,"record_offset":832,"parent_record_offset":744,"ancestor_record_offset":408,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4024,"record_offset":6784,"parent_record_offset":6340,"ancestor_record_offset":408,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4024,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":408,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4024,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4024,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":408,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4024,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":408,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4024,"record_offset":2068,"parent_record_offset":2008,"ancestor_record_offset":408,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4024,"record_offset":2040,"parent_record_offset":2008,"ancestor_record_offset":408,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4024,"record_offset":1192,"parent_record_offset":1028,"ancestor_record_offset":408,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4024,"record_offset":2224,"parent_record_offset":2196,"ancestor_record_offset":408,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4024,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":408,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4024,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":408,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4024,"record_offset":3140,"parent_record_offset":3108,"ancestor_record_offset":408,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4024,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":408,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4024,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":408,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4024,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":408,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4024,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":408,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4024,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":408,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4024,"record_offset":1420,"parent_record_offset":1388,"ancestor_record_offset":408,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4024,"record_offset":1452,"parent_record_offset":1420,"ancestor_record_offset":408,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4024,"record_offset":1540,"parent_record_offset":1388,"ancestor_record_offset":408,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4024,"record_offset":1388,"parent_record_offset":1348,"ancestor_record_offset":408,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4024,"record_offset":776,"parent_record_offset":744,"ancestor_record_offset":408,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4024,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":408,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4024,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":408,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":408,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":1776,"parent_record_offset":1292,"ancestor_record_offset":408,"inlinee":8503,"source_line":573,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":1628,"parent_record_offset":1348,"ancestor_record_offset":408,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":744,"parent_record_offset":680,"ancestor_record_offset":408,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":1348,"parent_record_offset":1320,"ancestor_record_offset":408,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":652,"parent_record_offset":624,"ancestor_record_offset":408,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":716,"parent_record_offset":680,"ancestor_record_offset":408,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":408,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4024,"record_offset":1320,"parent_record_offset":1292,"ancestor_record_offset":408,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4024,"record_offset":3304,"parent_record_offset":3272,"ancestor_record_offset":408,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4024,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":408,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E60
// 首条位置: {"module":3991,"inlinee":134752}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3991,"record_offset":12032,"parent_record_offset":11960,"ancestor_record_offset":11960,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3991,"record_offset":12904,"parent_record_offset":11960,"ancestor_record_offset":11960,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":3991,"record_offset":12060,"parent_record_offset":12032,"ancestor_record_offset":11960,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3991,"record_offset":12236,"parent_record_offset":12208,"ancestor_record_offset":11960,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3991,"record_offset":12176,"parent_record_offset":12060,"ancestor_record_offset":11960,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3991,"record_offset":12208,"parent_record_offset":12032,"ancestor_record_offset":11960,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3991,"record_offset":12144,"parent_record_offset":12084,"ancestor_record_offset":11960,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3991,"record_offset":12116,"parent_record_offset":12084,"ancestor_record_offset":11960,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3991,"record_offset":12084,"parent_record_offset":12060,"ancestor_record_offset":11960,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$1::positions::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":3991,"record_offset":12264,"parent_record_offset":12236,"ancestor_record_offset":11960,"inlinee":36789,"source_line":196,"archive":"_pdb/C/modules/03991.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F2E
// 首条位置: {"module":4024,"inlinee":134958}
// 编译实例 RVAs: 0x10bad00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10572887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386251520,"raw_hex":"e8a9ab5eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386251570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10573015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386251520,"raw_hex":"e829ab5eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5386251714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17529368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386251520,"raw_hex":"e8e885f4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386251555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30369093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386251520,"raw_hex":"e84565cf01","size":5,"soft_fail":false,"target_va":5416620769,"va":5386251671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall_and_roof::color_cursor_icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17537228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386251520,"raw_hex":"e8cc980b01","size":5,"soft_fail":false,"target_va":5403788848,"va":5386251615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":9948,"parent_record_offset":9624,"ancestor_record_offset":9624,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":9772,"parent_record_offset":9624,"ancestor_record_offset":9624,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4024,"record_offset":10032,"parent_record_offset":9624,"ancestor_record_offset":9624,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":9624,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":9624,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":10104,"parent_record_offset":10080,"ancestor_record_offset":9624,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":9624,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":9624,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":9624,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":9828,"parent_record_offset":9800,"ancestor_record_offset":9624,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":9624,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":9800,"parent_record_offset":9772,"ancestor_record_offset":9624,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":9624,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4024,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":9624,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":9996,"parent_record_offset":9972,"ancestor_record_offset":9624,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4024,"record_offset":10204,"rva":42657152,"va":5411366272}
// 编译实例 RVAs: 0x28ae580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-35687654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e81a73dffd","size":5,"soft_fail":false,"target_va":5375678688,"va":5411366337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-35688051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e88d71dffd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411366750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42644135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e8594d75fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5411366322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5690576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e8d0d45600","size":5,"soft_fail":false,"target_va":5417057056,"va":5411366475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5254080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e8c02b5000","size":5,"soft_fail":false,"target_va":5416620769,"va":5411366684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5254391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e8f72c5000","size":5,"soft_fail":false,"target_va":5416620794,"va":5411366398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32430264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e8482711fe","size":5,"soft_fail":false,"target_va":5378936272,"va":5411366531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32427858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e8ae3011fe","size":5,"soft_fail":false,"target_va":5378938736,"va":5411366589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32430478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e8722611fe","size":5,"soft_fail":false,"target_va":5378936080,"va":5411366553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32430167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411366272,"raw_hex":"e8a92711fe","size":5,"soft_fail":false,"target_va":5378936352,"va":5411366514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":11120,"parent_record_offset":10204,"ancestor_record_offset":10204,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":10356,"parent_record_offset":10204,"ancestor_record_offset":10204,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":10536,"parent_record_offset":10204,"ancestor_record_offset":10204,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4024,"record_offset":11204,"parent_record_offset":10204,"ancestor_record_offset":10204,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":10204,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":10204,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":10204,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":10204,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":10204,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":11324,"parent_record_offset":11300,"ancestor_record_offset":10204,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":10416,"parent_record_offset":10384,"ancestor_record_offset":10204,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4024,"record_offset":11252,"parent_record_offset":11228,"ancestor_record_offset":10204,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":10384,"parent_record_offset":10356,"ancestor_record_offset":10204,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4024,"record_offset":11228,"parent_record_offset":11204,"ancestor_record_offset":10204,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":10204,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":10744,"parent_record_offset":10712,"ancestor_record_offset":10204,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":10204,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4024,"record_offset":10780,"parent_record_offset":10744,"ancestor_record_offset":10204,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4024,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":10204,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4024,"record_offset":10628,"parent_record_offset":10564,"ancestor_record_offset":10204,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":10600,"parent_record_offset":10564,"ancestor_record_offset":10204,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":10204,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":10664,"parent_record_offset":10628,"ancestor_record_offset":10204,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4024,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":10204,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4024,"record_offset":10804,"parent_record_offset":10780,"ancestor_record_offset":10204,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4024,"record_offset":10884,"parent_record_offset":10828,"ancestor_record_offset":10204,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4024,"record_offset":10828,"parent_record_offset":10804,"ancestor_record_offset":10204,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4024,"record_offset":10856,"parent_record_offset":10828,"ancestor_record_offset":10204,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4024,"record_offset":11052,"parent_record_offset":10628,"ancestor_record_offset":10204,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04024.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4024,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":10204,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04024.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_color_gates.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_color_gates.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_color_gates.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::gates::Gates>,bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::gates::Gates>", "bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >", "bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::gates::Gates>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::WoodenGate>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates
// 信息 ID: 429e140e46e75f78232158cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1b38a78144ad2aaa9b3fa3f2: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::gates::Gates>,bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::gates::Gates>,bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::gates::Gates>,bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> > @ 0x215ca1f -> 0x28ad900 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::gates::Gates>,bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::gates::Gates>,bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> @ 0x21778a8 -> 0x28ad900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$0
// 信息 ID: 2070ca9528438d29063bc2c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >; module=3991, inline_record=11960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$1
// 信息 ID: 04319b755bd2b3e2a98b5b2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$1 <- core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=4024, inline_record=12800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$2
// 信息 ID: bbc16132e0db0bc0357d61a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$2 <- core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$2 <- core::ptr::drop_in_place<system_color::ui_color_gates::ui_color_gates::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x28ae7a3 -> 0x28ae580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
