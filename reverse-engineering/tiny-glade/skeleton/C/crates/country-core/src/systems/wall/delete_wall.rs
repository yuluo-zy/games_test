// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/delete_wall.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 749084bdee250d9c6949b29deb3a2ebb1b31e2ab586358b55be27c42ca9f2661
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: delete_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 558
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::delete_wall::delete_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":55860,"rva":11503008,"va":5380212128}
// 编译实例 RVAs: 0xaf85a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t-4534368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380212128,"raw_hex":"e8a0cfbaff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380213067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ce3b0
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7513292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380212128,"raw_hex":"e8345b8dff","size":5,"soft_fail":false,"target_va":5372699568,"va":5380212855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cef10
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7512963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380212128,"raw_hex":"e87d5c8dff","size":5,"soft_fail":false,"target_va":5372702480,"va":5380215438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=7
//     target_aliases=6 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6733638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380212128,"raw_hex":"e8ba4099ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380212673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097cc50
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1558128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380212128,"raw_hex":"e89039e8ff","size":5,"soft_fail":false,"target_va":5378657360,"va":5380215483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098aae0
//     owner=unresolved_namespace; count=4
//     target_aliases=7 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1501224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380212128,"raw_hex":"e8d817e9ff","size":5,"soft_fail":false,"target_va":5378714336,"va":5380215555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca6520
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1760577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380212128,"raw_hex":"e841dd1a00","size":5,"soft_fail":false,"target_va":5381973280,"va":5380212698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4206162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380212128,"raw_hex":"e8522e4000","size":5,"soft_fail":false,"target_va":5384418976,"va":5380212809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":558,"record_offset":57080,"parent_record_offset":55860,"ancestor_record_offset":55860,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":558,"record_offset":56736,"parent_record_offset":55860,"ancestor_record_offset":55860,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":558,"record_offset":56024,"parent_record_offset":55860,"ancestor_record_offset":55860,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":558,"record_offset":57424,"parent_record_offset":55860,"ancestor_record_offset":55860,"inlinee":10329,"source_line":442,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":558,"record_offset":57628,"parent_record_offset":55860,"ancestor_record_offset":55860,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":558,"record_offset":59572,"parent_record_offset":55860,"ancestor_record_offset":55860,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":558,"record_offset":57136,"parent_record_offset":55860,"ancestor_record_offset":55860,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":558,"record_offset":58672,"parent_record_offset":58648,"ancestor_record_offset":55860,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":558,"record_offset":58696,"parent_record_offset":58672,"ancestor_record_offset":55860,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":558,"record_offset":58720,"parent_record_offset":58696,"ancestor_record_offset":55860,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":558,"record_offset":58648,"parent_record_offset":58560,"ancestor_record_offset":55860,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":558,"record_offset":58588,"parent_record_offset":58560,"ancestor_record_offset":55860,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":558,"record_offset":58560,"parent_record_offset":58536,"ancestor_record_offset":55860,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":558,"record_offset":58536,"parent_record_offset":58512,"ancestor_record_offset":55860,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":56364,"parent_record_offset":56340,"ancestor_record_offset":55860,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":57872,"parent_record_offset":57848,"ancestor_record_offset":55860,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":56340,"parent_record_offset":56112,"ancestor_record_offset":55860,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":558,"record_offset":58440,"parent_record_offset":58416,"ancestor_record_offset":55860,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":558,"record_offset":58488,"parent_record_offset":58464,"ancestor_record_offset":55860,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":558,"record_offset":58184,"parent_record_offset":58160,"ancestor_record_offset":55860,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":558,"record_offset":57104,"parent_record_offset":57080,"ancestor_record_offset":55860,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":558,"record_offset":56112,"parent_record_offset":56088,"ancestor_record_offset":55860,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":558,"record_offset":56760,"parent_record_offset":56736,"ancestor_record_offset":55860,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":558,"record_offset":57040,"parent_record_offset":57016,"ancestor_record_offset":55860,"inlinee":11539,"source_line":90,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":558,"record_offset":56088,"parent_record_offset":56048,"ancestor_record_offset":55860,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":558,"record_offset":56048,"parent_record_offset":56024,"ancestor_record_offset":55860,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":558,"record_offset":56816,"parent_record_offset":56792,"ancestor_record_offset":55860,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":558,"record_offset":56792,"parent_record_offset":56760,"ancestor_record_offset":55860,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":558,"record_offset":56896,"parent_record_offset":56872,"ancestor_record_offset":55860,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":558,"record_offset":58096,"parent_record_offset":57660,"ancestor_record_offset":55860,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":558,"record_offset":57660,"parent_record_offset":57628,"ancestor_record_offset":55860,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":558,"record_offset":57736,"parent_record_offset":57708,"ancestor_record_offset":55860,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":558,"record_offset":57708,"parent_record_offset":57660,"ancestor_record_offset":55860,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":558,"record_offset":57764,"parent_record_offset":57736,"ancestor_record_offset":55860,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":558,"record_offset":58012,"parent_record_offset":57988,"ancestor_record_offset":55860,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":558,"record_offset":56296,"parent_record_offset":56268,"ancestor_record_offset":55860,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":558,"record_offset":56156,"parent_record_offset":56112,"ancestor_record_offset":55860,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":558,"record_offset":58616,"parent_record_offset":58588,"ancestor_record_offset":55860,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":558,"record_offset":58160,"parent_record_offset":58096,"ancestor_record_offset":55860,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":558,"record_offset":57824,"parent_record_offset":57792,"ancestor_record_offset":55860,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":56840,"parent_record_offset":56816,"ancestor_record_offset":55860,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":58336,"parent_record_offset":58096,"ancestor_record_offset":55860,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":58360,"parent_record_offset":58336,"ancestor_record_offset":55860,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":57016,"parent_record_offset":56760,"ancestor_record_offset":55860,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":56872,"parent_record_offset":56792,"ancestor_record_offset":55860,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":57988,"parent_record_offset":57964,"ancestor_record_offset":55860,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$20::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":57964,"parent_record_offset":57940,"ancestor_record_offset":55860,"inlinee":10740,"source_line":2481,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$36::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":558,"record_offset":57940,"parent_record_offset":57764,"ancestor_record_offset":55860,"inlinee":10739,"source_line":2616,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=100
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":558,"record_offset":58392,"parent_record_offset":58096,"ancestor_record_offset":55860,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":558,"record_offset":56628,"parent_record_offset":56604,"ancestor_record_offset":55860,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":558,"record_offset":56972,"parent_record_offset":56948,"ancestor_record_offset":55860,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":558,"record_offset":58272,"parent_record_offset":58184,"ancestor_record_offset":55860,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":558,"record_offset":58216,"parent_record_offset":58184,"ancestor_record_offset":55860,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":558,"record_offset":58296,"parent_record_offset":58272,"ancestor_record_offset":55860,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":558,"record_offset":56212,"parent_record_offset":56112,"ancestor_record_offset":55860,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":558,"record_offset":56580,"parent_record_offset":56112,"ancestor_record_offset":55860,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":56268,"parent_record_offset":56240,"ancestor_record_offset":55860,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":56240,"parent_record_offset":56212,"ancestor_record_offset":55860,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":558,"record_offset":56604,"parent_record_offset":56580,"ancestor_record_offset":55860,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":558,"record_offset":56948,"parent_record_offset":56896,"ancestor_record_offset":55860,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::delete_wall::delete_wall::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/delete_wall.rs
//     evidence={"module":558,"record_offset":57848,"parent_record_offset":57824,"ancestor_record_offset":55860,"inlinee":34914,"source_line":29,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::delete_wall::delete_wall::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/delete_wall.rs
//     evidence={"module":558,"record_offset":61284,"parent_record_offset":61260,"ancestor_record_offset":55860,"inlinee":34915,"source_line":37,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::delete_wall::delete_wall::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/delete_wall.rs
//     evidence={"module":558,"record_offset":64720,"parent_record_offset":64696,"ancestor_record_offset":55860,"inlinee":34916,"source_line":45,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::delete_wall::delete_wall::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/delete_wall.rs
//     evidence={"module":558,"record_offset":68156,"parent_record_offset":68132,"ancestor_record_offset":55860,"inlinee":34917,"source_line":53,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":558,"record_offset":57160,"parent_record_offset":57136,"ancestor_record_offset":55860,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":558,"record_offset":57184,"parent_record_offset":57160,"ancestor_record_offset":55860,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":558,"record_offset":57212,"parent_record_offset":57184,"ancestor_record_offset":55860,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::delete_wall::delete_wall::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8862
// 首条位置: {"module":558,"inlinee":34914}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":57872,"parent_record_offset":57848,"ancestor_record_offset":57848,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":558,"record_offset":57896,"parent_record_offset":57872,"ancestor_record_offset":57848,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::delete_wall::delete_wall::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8863
// 首条位置: {"module":558,"inlinee":34915}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":61308,"parent_record_offset":61284,"ancestor_record_offset":61284,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":558,"record_offset":61332,"parent_record_offset":61308,"ancestor_record_offset":61284,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::delete_wall::delete_wall::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8864
// 首条位置: {"module":558,"inlinee":34916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":64744,"parent_record_offset":64720,"ancestor_record_offset":64720,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":558,"record_offset":64768,"parent_record_offset":64744,"ancestor_record_offset":64720,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::delete_wall::delete_wall::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8865
// 首条位置: {"module":558,"inlinee":34917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":68180,"parent_record_offset":68156,"ancestor_record_offset":68156,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":558,"record_offset":68204,"parent_record_offset":68180,"ancestor_record_offset":68156,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00558.jsonl.gz"}
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
// 完整结构及原始类型记录: delete_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: delete_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: delete_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::delete_wall::delete_wall
// 信息 ID: 9b1c71ec48f4499c4d62e080
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c309736f54a5a6d2363c62b2: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)> > @ 0x2a44c0 -> 0xaf85a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)> @ 0x60aec8 -> 0xaf85a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::delete_wall::delete_wall::closure$0
// 信息 ID: e2811457d647286e635a37f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::delete_wall::delete_wall", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::delete_wall::delete_wall; module=558, inline_record=57848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::delete_wall::delete_wall::closure$1
// 信息 ID: d984fda277df7174daadaaa2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::delete_wall::delete_wall", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::delete_wall::delete_wall; module=558, inline_record=61284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::delete_wall::delete_wall::closure$2
// 信息 ID: 8842754ca00ed8c4ba00168b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::delete_wall::delete_wall", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::delete_wall::delete_wall; module=558, inline_record=64720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::delete_wall::delete_wall::closure$3
// 信息 ID: 480a91f0f9f154c996136965
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::wall::delete_wall::delete_wall", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::delete_wall::delete_wall::closure_env$3 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::option::IntoIter<alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >,alloc::vec::into_iter::IntoIter<bevy_ecs::entity::Entity,alloc::alloc::Global>,country_core::systems::wall::delete_wall::delete_wall::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::wall::delete_wall::delete_wall; module=558, inline_record=68156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
