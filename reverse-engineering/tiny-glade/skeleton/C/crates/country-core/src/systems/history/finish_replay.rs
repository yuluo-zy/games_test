// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/history/finish_replay.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ff895edf0f63f69c85d531b7e3b31ceadea359abe25cc82a0e8af183e6a43b72
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: finish_replay.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 594
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::finish_replay::finish_replay
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":594,"record_offset":18548,"rva":12717744,"va":5381426864}
// 编译实例 RVAs: 0xc20eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3181201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381426864,"raw_hex":"e8918a3000","size":5,"soft_fail":false,"target_va":5384609408,"va":5381428202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c23e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: finish_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381426864,"raw_hex":"e884290000","size":5,"soft_fail":false,"target_va":5381438976,"va":5381428343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: finish_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34923695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381426864,"raw_hex":"e8afe41402","size":5,"soft_fail":false,"target_va":5416350720,"va":5381427020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35193796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381426864,"raw_hex":"e8c4031902","size":5,"soft_fail":false,"target_va":5416621762,"va":5381427961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::toolbar::Toolbar::switch_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-874513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381426864,"raw_hex":"e8efa7f2ff","size":5,"soft_fail":false,"target_va":5380553360,"va":5381427868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":18744,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":13892,"source_line":2019,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":594,"record_offset":22616,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":594,"record_offset":19568,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":594,"record_offset":19072,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":594,"record_offset":21212,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":20328,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::pressed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":594,"record_offset":19540,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":29412,"source_line":238,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::MetaCommander::peek
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":594,"record_offset":20756,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":32499,"source_line":154,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":594,"record_offset":19484,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":34441,"source_line":198,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::history_process::PendingHistoryCommands::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/history_process.rs
//     evidence={"module":594,"record_offset":19428,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":37827,"source_line":127,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_mode::RestoreAppTo::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_mode.rs
//     evidence={"module":594,"record_offset":22672,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":36672,"source_line":52,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::mode_manager::switch_app_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":594,"record_offset":22760,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":35932,"source_line":622,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":594,"record_offset":21268,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":594,"record_offset":21128,"parent_record_offset":18548,"ancestor_record_offset":18548,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":24072,"parent_record_offset":23836,"ancestor_record_offset":18548,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":18852,"parent_record_offset":18824,"ancestor_record_offset":18548,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":20780,"parent_record_offset":20756,"ancestor_record_offset":18548,"inlinee":12112,"source_line":1974,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":18768,"parent_record_offset":18744,"ancestor_record_offset":18548,"inlinee":12113,"source_line":907,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":19452,"parent_record_offset":19428,"ancestor_record_offset":18548,"inlinee":12110,"source_line":1650,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":23836,"parent_record_offset":23732,"ancestor_record_offset":18548,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":18952,"parent_record_offset":18768,"ancestor_record_offset":18548,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":18796,"parent_record_offset":18768,"ancestor_record_offset":18548,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":18824,"parent_record_offset":18796,"ancestor_record_offset":18548,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":18884,"parent_record_offset":18852,"ancestor_record_offset":18548,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":18976,"parent_record_offset":18952,"ancestor_record_offset":18548,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":18908,"parent_record_offset":18884,"ancestor_record_offset":18548,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":19024,"parent_record_offset":19000,"ancestor_record_offset":18548,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":19000,"parent_record_offset":18976,"ancestor_record_offset":18548,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":19268,"parent_record_offset":19228,"ancestor_record_offset":18548,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":19956,"parent_record_offset":19932,"ancestor_record_offset":18548,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":22896,"parent_record_offset":22796,"ancestor_record_offset":18548,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":19196,"parent_record_offset":19160,"ancestor_record_offset":18548,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":19228,"parent_record_offset":19196,"ancestor_record_offset":18548,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":18548,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":19932,"parent_record_offset":19680,"ancestor_record_offset":18548,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":594,"record_offset":19132,"parent_record_offset":19096,"ancestor_record_offset":18548,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":594,"record_offset":19096,"parent_record_offset":19072,"ancestor_record_offset":18548,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":594,"record_offset":19680,"parent_record_offset":19648,"ancestor_record_offset":18548,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":594,"record_offset":20400,"parent_record_offset":20360,"ancestor_record_offset":18548,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":594,"record_offset":20428,"parent_record_offset":20400,"ancestor_record_offset":18548,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":594,"record_offset":19648,"parent_record_offset":19600,"ancestor_record_offset":18548,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":594,"record_offset":19160,"parent_record_offset":19096,"ancestor_record_offset":18548,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":594,"record_offset":19600,"parent_record_offset":19568,"ancestor_record_offset":18548,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":594,"record_offset":20484,"parent_record_offset":20460,"ancestor_record_offset":18548,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":594,"record_offset":20460,"parent_record_offset":20428,"ancestor_record_offset":18548,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":594,"record_offset":20564,"parent_record_offset":20540,"ancestor_record_offset":18548,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":20692,"parent_record_offset":20360,"ancestor_record_offset":18548,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":20360,"parent_record_offset":20328,"ancestor_record_offset":18548,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":594,"record_offset":19884,"parent_record_offset":19852,"ancestor_record_offset":18548,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":594,"record_offset":19732,"parent_record_offset":19680,"ancestor_record_offset":18548,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":594,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":18548,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":594,"record_offset":20508,"parent_record_offset":20484,"ancestor_record_offset":18548,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":594,"record_offset":21236,"parent_record_offset":21212,"ancestor_record_offset":18548,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":594,"record_offset":20540,"parent_record_offset":20460,"ancestor_record_offset":18548,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":594,"record_offset":20220,"parent_record_offset":20196,"ancestor_record_offset":18548,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":594,"record_offset":20640,"parent_record_offset":20616,"ancestor_record_offset":18548,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":594,"record_offset":19380,"parent_record_offset":19228,"ancestor_record_offset":18548,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":594,"record_offset":19788,"parent_record_offset":19680,"ancestor_record_offset":18548,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":594,"record_offset":20172,"parent_record_offset":19680,"ancestor_record_offset":18548,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":594,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":18548,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":594,"record_offset":22864,"parent_record_offset":22840,"ancestor_record_offset":18548,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":594,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":18548,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":594,"record_offset":19852,"parent_record_offset":19820,"ancestor_record_offset":18548,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":594,"record_offset":19820,"parent_record_offset":19788,"ancestor_record_offset":18548,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":594,"record_offset":20196,"parent_record_offset":20172,"ancestor_record_offset":18548,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":594,"record_offset":20616,"parent_record_offset":20564,"ancestor_record_offset":18548,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":594,"record_offset":21152,"parent_record_offset":21128,"ancestor_record_offset":18548,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":594,"record_offset":21176,"parent_record_offset":21152,"ancestor_record_offset":18548,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::toolbar::Toolbar::switch_category
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":594,"record_offset":22796,"parent_record_offset":22760,"ancestor_record_offset":18548,"inlinee":35840,"source_line":285,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::toolbar::ToolbarCategoryModes::last_selected_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":594,"record_offset":22988,"parent_record_offset":22796,"ancestor_record_offset":18548,"inlinee":35845,"source_line":46,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::toolbar::ToolbarCategoryModes::next_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":594,"record_offset":22952,"parent_record_offset":22796,"ancestor_record_offset":18548,"inlinee":35844,"source_line":40,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::toolbar::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":594,"record_offset":22924,"parent_record_offset":22796,"ancestor_record_offset":18548,"inlinee":35842,"source_line":53,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::finish_replay::finish_replay::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/finish_replay.rs
//     evidence={"module":594,"record_offset":20716,"parent_record_offset":20692,"ancestor_record_offset":18548,"inlinee":37829,"source_line":52,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":594,"record_offset":21316,"parent_record_offset":21292,"ancestor_record_offset":18548,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":594,"record_offset":21292,"parent_record_offset":21268,"ancestor_record_offset":18548,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":594,"record_offset":21340,"parent_record_offset":21316,"ancestor_record_offset":18548,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::finish_replay::finish_replay::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x93C5
// 首条位置: {"module":594,"inlinee":37829}
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
// 完整结构及原始类型记录: finish_replay.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: finish_replay.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: finish_replay.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>

// ------------------------------------------------------------------
// 函数: country_core::systems::history::finish_replay::finish_replay
// 信息 ID: c7d36a4efe6d3c09f6fb761b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9e2f950e31b4f755565e8ad4: void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)> > @ 0x1945222 -> 0xc20eb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)> @ 0x19b8634 -> 0xc20eb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::finish_replay::finish_replay::closure$1
// 信息 ID: 11d24e3c7c7d4d1683c7ef69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::history::finish_replay::finish_replay", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::finish_replay::finish_replay; module=594, inline_record=20716; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::finish_replay::finish_replay; module=594, inline_record=22548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
