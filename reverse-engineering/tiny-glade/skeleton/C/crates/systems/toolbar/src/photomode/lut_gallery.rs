// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/photomode/lut_gallery.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 991c9040d5badc046d016115fafd5450b6ee828bc25f9ddda133ac1f1260a902
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lut_gallery.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1239, 2962
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::lut_gallery::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2962,"record_offset":2080,"rva":37519408,"va":5406228528}
// 编译实例 RVAs: 0x23c8030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21434589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e823efb8fe","size":5,"soft_fail":false,"target_va":5384795984,"va":5406230568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone_from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21434559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e841efb8fe","size":5,"soft_fail":false,"target_va":5384795872,"va":5406230426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30552020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e82cd02dfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5406230719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25164977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e84f0380fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5406229148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24622549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e82b4a88fe","size":5,"soft_fail":false,"target_va":5381607664,"va":5406230208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141173c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19220270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8d2b8dafe","size":5,"soft_fail":false,"target_va":5387009056,"va":5406229321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117b2b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19190626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e89e2cdbfe","size":5,"soft_fail":false,"target_va":5387039408,"va":5406230029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19184927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8e142dbfe","size":5,"soft_fail":false,"target_va":5387045984,"va":5406230906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411924e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19096760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8489bdcfe","size":5,"soft_fail":false,"target_va":5387134176,"va":5406230931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ac1c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18989498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8463edefe","size":5,"soft_fail":false,"target_va":5387239872,"va":5406229365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10120954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8fa6e9a00","size":5,"soft_fail":false,"target_va":5416351056,"va":5406230097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10122177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8c1739a00","size":5,"soft_fail":false,"target_va":5416351392,"va":5406229210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10390388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8748b9e00","size":5,"soft_fail":false,"target_va":5416621408,"va":5406231015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t10390439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8a78b9e00","size":5,"soft_fail":false,"target_va":5416621536,"va":5406231092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t10390716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8bc8c9e00","size":5,"soft_fail":false,"target_va":5416622240,"va":5406231519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e87efbffff","size":5,"soft_fail":false,"target_va":5406227472,"va":5406228621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10389944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8b8899e00","size":5,"soft_fail":false,"target_va":5416621376,"va":5406231427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::image_formation::ImageFormationConfigs::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27367050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e876695efe","size":5,"soft_fail":false,"target_va":5378863504,"va":5406230549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::image_formation::ImageFormationConfigs::iter_non_default_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27366884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e81c6a5efe","size":5,"soft_fail":false,"target_va":5378863472,"va":5406230351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22265214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e88242acfe","size":5,"soft_fail":false,"target_va":5383963792,"va":5406229001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22256707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8bd63acfe","size":5,"soft_fail":false,"target_va":5383972224,"va":5406228926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22265597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e80341acfe","size":5,"soft_fail":false,"target_va":5383964912,"va":5406230504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22265767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e85940acfe","size":5,"soft_fail":false,"target_va":5383962816,"va":5406228578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10425530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e8ba149f00","size":5,"soft_fail":false,"target_va":5416656560,"va":5406231025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18900713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e81799dffe","size":5,"soft_fail":false,"target_va":5387331072,"va":5406231780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10388288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e840839e00","size":5,"soft_fail":false,"target_va":5416619360,"va":5406231067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30022813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e863e335fe","size":5,"soft_fail":false,"target_va":5376206368,"va":5406229176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::initial_focused_item
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1010657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406228528,"raw_hex":"e81f94f0ff","size":5,"soft_fail":false,"target_va":5405218688,"va":5406229340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5406228528,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5406229403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2962,"record_offset":5608,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2962,"record_offset":5232,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2962,"record_offset":8192,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2962,"record_offset":9240,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2962,"record_offset":9212,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2962,"record_offset":3680,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2962,"record_offset":9096,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":2962,"record_offset":9896,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2962,"record_offset":4424,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2962,"record_offset":2280,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2962,"record_offset":2668,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2962,"record_offset":11028,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2962,"record_offset":5292,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":10267,"source_line":466,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::wide_safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2962,"record_offset":10712,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":10270,"source_line":483,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::clicked_item
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2962,"record_offset":8612,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":111436,"source_line":112,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::gap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2962,"record_offset":6184,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":111443,"source_line":147,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::horizontal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2962,"record_offset":7160,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":111433,"source_line":160,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::hovered_item
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2962,"record_offset":8136,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":111437,"source_line":104,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::nav_scroll_speed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2962,"record_offset":6672,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":111441,"source_line":185,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::run_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2962,"record_offset":7648,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":111445,"source_line":153,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::tile_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2962,"record_offset":5696,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":111434,"source_line":141,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2962,"record_offset":10544,"parent_record_offset":10520,"ancestor_record_offset":2080,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2962,"record_offset":10568,"parent_record_offset":10544,"ancestor_record_offset":2080,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2962,"record_offset":10592,"parent_record_offset":10568,"ancestor_record_offset":2080,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2962,"record_offset":10520,"parent_record_offset":10464,"ancestor_record_offset":2080,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2962,"record_offset":5476,"parent_record_offset":5444,"ancestor_record_offset":2080,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2962,"record_offset":10492,"parent_record_offset":10464,"ancestor_record_offset":2080,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2962,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":2080,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2962,"record_offset":5524,"parent_record_offset":5500,"ancestor_record_offset":2080,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2962,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":2080,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2962,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":2080,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2962,"record_offset":5444,"parent_record_offset":5408,"ancestor_record_offset":2080,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2962,"record_offset":5384,"parent_record_offset":5316,"ancestor_record_offset":2080,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2962,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":2080,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2962,"record_offset":5260,"parent_record_offset":5232,"ancestor_record_offset":2080,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2962,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":2080,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2962,"record_offset":8280,"parent_record_offset":8216,"ancestor_record_offset":2080,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":2962,"record_offset":6000,"parent_record_offset":5812,"ancestor_record_offset":2080,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":2962,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":2080,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":2962,"record_offset":6060,"parent_record_offset":6028,"ancestor_record_offset":2080,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2962,"record_offset":6088,"parent_record_offset":6060,"ancestor_record_offset":2080,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2962,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":2080,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2962,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":2080,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2962,"record_offset":4024,"parent_record_offset":3996,"ancestor_record_offset":2080,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2962,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":2080,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2962,"record_offset":4164,"parent_record_offset":4048,"ancestor_record_offset":2080,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2962,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":2080,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2962,"record_offset":9288,"parent_record_offset":9264,"ancestor_record_offset":2080,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2962,"record_offset":9428,"parent_record_offset":9312,"ancestor_record_offset":2080,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2962,"record_offset":9264,"parent_record_offset":9240,"ancestor_record_offset":2080,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2962,"record_offset":9456,"parent_record_offset":9428,"ancestor_record_offset":2080,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2962,"record_offset":9312,"parent_record_offset":9288,"ancestor_record_offset":2080,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2962,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":2080,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2962,"record_offset":3896,"parent_record_offset":3856,"ancestor_record_offset":2080,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2962,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":2080,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2962,"record_offset":5900,"parent_record_offset":5812,"ancestor_record_offset":2080,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2962,"record_offset":5936,"parent_record_offset":5812,"ancestor_record_offset":2080,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2962,"record_offset":2616,"parent_record_offset":2588,"ancestor_record_offset":2080,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2962,"record_offset":4372,"parent_record_offset":3856,"ancestor_record_offset":2080,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2962,"record_offset":3716,"parent_record_offset":3680,"ancestor_record_offset":2080,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2962,"record_offset":9396,"parent_record_offset":9340,"ancestor_record_offset":2080,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2962,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2080,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2962,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":2080,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2962,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":2080,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2962,"record_offset":5564,"parent_record_offset":5408,"ancestor_record_offset":2080,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2962,"record_offset":3372,"parent_record_offset":3344,"ancestor_record_offset":2080,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2962,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":2080,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2962,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":2080,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2962,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":2080,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2962,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":2080,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2962,"record_offset":3400,"parent_record_offset":3372,"ancestor_record_offset":2080,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2962,"record_offset":2992,"parent_record_offset":2876,"ancestor_record_offset":2080,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2962,"record_offset":9340,"parent_record_offset":9312,"ancestor_record_offset":2080,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2962,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":2080,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2962,"record_offset":2704,"parent_record_offset":2668,"ancestor_record_offset":2080,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2962,"record_offset":2844,"parent_record_offset":2812,"ancestor_record_offset":2080,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2962,"record_offset":2812,"parent_record_offset":2776,"ancestor_record_offset":2080,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2962,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2080,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2962,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2080,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2962,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":2080,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2962,"record_offset":3316,"parent_record_offset":2776,"ancestor_record_offset":2080,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2962,"record_offset":2384,"parent_record_offset":2352,"ancestor_record_offset":2080,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2962,"record_offset":2352,"parent_record_offset":2324,"ancestor_record_offset":2080,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2962,"record_offset":2508,"parent_record_offset":2476,"ancestor_record_offset":2080,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2962,"record_offset":2564,"parent_record_offset":2508,"ancestor_record_offset":2080,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2962,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":2080,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2962,"record_offset":2324,"parent_record_offset":2280,"ancestor_record_offset":2080,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2962,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":2080,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2962,"record_offset":2876,"parent_record_offset":2844,"ancestor_record_offset":2080,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2962,"record_offset":3160,"parent_record_offset":2876,"ancestor_record_offset":2080,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2962,"record_offset":3132,"parent_record_offset":2876,"ancestor_record_offset":2080,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2962,"record_offset":2936,"parent_record_offset":2876,"ancestor_record_offset":2080,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2962,"record_offset":5636,"parent_record_offset":5608,"ancestor_record_offset":2080,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2962,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":2080,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$1::iter_non_default_mut::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":2962,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":2080,"inlinee":70317,"source_line":110,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2962,"record_offset":3784,"parent_record_offset":3748,"ancestor_record_offset":2080,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2962,"record_offset":3748,"parent_record_offset":3716,"ancestor_record_offset":2080,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2962,"record_offset":4200,"parent_record_offset":3996,"ancestor_record_offset":2080,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2962,"record_offset":3924,"parent_record_offset":3896,"ancestor_record_offset":2080,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2962,"record_offset":3996,"parent_record_offset":3856,"ancestor_record_offset":2080,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2962,"record_offset":4876,"parent_record_offset":4672,"ancestor_record_offset":2080,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2962,"record_offset":5872,"parent_record_offset":5812,"ancestor_record_offset":2080,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2962,"record_offset":3952,"parent_record_offset":3924,"ancestor_record_offset":2080,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2962,"record_offset":3856,"parent_record_offset":3820,"ancestor_record_offset":2080,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2962,"record_offset":3820,"parent_record_offset":3784,"ancestor_record_offset":2080,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2962,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":2080,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2962,"record_offset":5776,"parent_record_offset":5740,"ancestor_record_offset":2080,"inlinee":10273,"source_line":1571,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::get_content_mut_impl
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2962,"record_offset":5812,"parent_record_offset":5776,"ancestor_record_offset":2080,"inlinee":10210,"source_line":1579,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2962,"record_offset":5356,"parent_record_offset":5316,"ancestor_record_offset":2080,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$18::get_content_mut_impl::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2962,"record_offset":5968,"parent_record_offset":5936,"ancestor_record_offset":2080,"inlinee":10214,"source_line":1588,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::content
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2962,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":2080,"inlinee":111424,"source_line":229,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2962,"record_offset":5740,"parent_record_offset":5696,"ancestor_record_offset":2080,"inlinee":111423,"source_line":234,"archive":"_pdb/C/modules/02962.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::lut_gallery::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1121F
// 首条位置: {"module":1239,"inlinee":70175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":72456,"parent_record_offset":72336,"ancestor_record_offset":72336,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":72372,"parent_record_offset":72336,"ancestor_record_offset":72336,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::hotkey
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":70028,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":21803,"source_line":1358,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$54::deref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":70088,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":21691,"source_line":216,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":72484,"parent_record_offset":72456,"ancestor_record_offset":72336,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":70896,"parent_record_offset":70872,"ancestor_record_offset":69996,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":70464,"parent_record_offset":70440,"ancestor_record_offset":69996,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":70548,"parent_record_offset":70524,"ancestor_record_offset":69996,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":70964,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":70696,"parent_record_offset":70668,"ancestor_record_offset":69996,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":70668,"parent_record_offset":70640,"ancestor_record_offset":69996,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":72588,"parent_record_offset":72564,"ancestor_record_offset":72336,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":70488,"parent_record_offset":70464,"ancestor_record_offset":69996,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":70812,"parent_record_offset":70788,"ancestor_record_offset":69996,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":70136,"parent_record_offset":70112,"ancestor_record_offset":69996,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":70920,"parent_record_offset":70896,"ancestor_record_offset":69996,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":70608,"parent_record_offset":70580,"ancestor_record_offset":69996,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":70640,"parent_record_offset":70608,"ancestor_record_offset":69996,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":70580,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":70440,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":70992,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":70524,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":70384,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":70872,"parent_record_offset":70848,"ancestor_record_offset":69996,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":70220,"parent_record_offset":70192,"ancestor_record_offset":69996,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":70848,"parent_record_offset":70740,"ancestor_record_offset":69996,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":70252,"parent_record_offset":70220,"ancestor_record_offset":69996,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":70788,"parent_record_offset":70764,"ancestor_record_offset":69996,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":71084,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":70764,"parent_record_offset":70740,"ancestor_record_offset":69996,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":70304,"parent_record_offset":70280,"ancestor_record_offset":69996,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":70280,"parent_record_offset":70220,"ancestor_record_offset":69996,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":70328,"parent_record_offset":70304,"ancestor_record_offset":69996,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":70412,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":70740,"parent_record_offset":70328,"ancestor_record_offset":69996,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":70192,"parent_record_offset":70164,"ancestor_record_offset":69996,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":70164,"parent_record_offset":70112,"ancestor_record_offset":69996,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":70112,"parent_record_offset":70088,"ancestor_record_offset":69996,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":72396,"parent_record_offset":72372,"ancestor_record_offset":72336,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":72420,"parent_record_offset":72396,"ancestor_record_offset":72336,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::PillButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":70056,"parent_record_offset":70028,"ancestor_record_offset":69996,"inlinee":21722,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
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
// 完整结构及原始类型记录: lut_gallery.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::photomode::lut_gallery::LutGalleryUiState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 27
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["toolbar::photomode::lut_gallery::LutGalleryUiState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$118::init_access<bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >", "constructor": "bevy_ecs::system::system_param::impl$118::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>", "bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lut_gallery.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lut_gallery.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>", "bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>

// ------------------------------------------------------------------
// 函数: toolbar::photomode::lut_gallery::ui
// 信息 ID: 0bfd4adf9c3478c3ae443cf2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e3fe939b0e54b7d8508ec413: void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> @ 0x11e5a8a -> 0x23c8030 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> > @ 0x230cba5 -> 0x23c8030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::photomode::lut_gallery::ui::closure$0
// 信息 ID: 01eebf25efb24256ac21961b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::photomode::lut_gallery::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::photomode::lut_gallery::ui::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::photomode::lut_gallery::ui::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::photomode::lut_gallery::ui::closure_env$0,ref$<str$> >; module=1239, inline_record=69996; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::photomode::lut_gallery::ui::closure_env$0,ref$<str$> >; module=1239, inline_record=71180; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::photomode::lut_gallery::ui::closure_env$0,ref$<str$> >; module=1239, inline_record=72336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
