// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/cottage_dormers/maintain_dormer_tiles_draw.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ad1cdef601cf136c072a68e5da378a936f85a01d46a47d9b305145fb143e1799
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: maintain_dormer_tiles_draw.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 453
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::maintain_dormer_tiles_draw::maintain_dormer_tiles
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":272,"rva":8950672,"va":5377659792}
// 编译实例 RVAs: 0x889390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cef10
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: maintain_dormer_tiles_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4957583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377659792,"raw_hex":"e8715ab4ff","size":5,"soft_fail":false,"target_va":5372702480,"va":5377660058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: maintain_dormer_tiles_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4181006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377659792,"raw_hex":"e8f233c0ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377660041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a9030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_dormer_tiles_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12712816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377659792,"raw_hex":"e870fbc100","size":5,"soft_fail":false,"target_va":5390372912,"va":5377660091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":453,"record_offset":1528,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":453,"record_offset":1184,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":453,"record_offset":476,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":1708,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":1584,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":812,"parent_record_offset":788,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":788,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":453,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":453,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":272,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":453,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":272,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":453,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":272,"inlinee":11539,"source_line":90,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":453,"record_offset":540,"parent_record_offset":500,"ancestor_record_offset":272,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":453,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":272,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":1240,"parent_record_offset":1208,"ancestor_record_offset":272,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":272,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":744,"parent_record_offset":716,"ancestor_record_offset":272,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":604,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":453,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":272,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":453,"record_offset":1464,"parent_record_offset":1208,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":453,"record_offset":1320,"parent_record_offset":1240,"ancestor_record_offset":272,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":660,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":1028,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":272,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":272,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":453,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":1396,"parent_record_offset":1344,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":453,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":272,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":453,"record_offset":1640,"parent_record_offset":1612,"ancestor_record_offset":272,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":453,"record_offset":1668,"parent_record_offset":1640,"ancestor_record_offset":272,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00453.jsonl.gz"}
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
// 完整结构及原始类型记录: maintain_dormer_tiles_draw.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: maintain_dormer_tiles_draw.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: maintain_dormer_tiles_draw.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::maintain_dormer_tiles_draw::maintain_dormer_tiles
// 信息 ID: 248c41e1ae9457715858a584
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a94b47f1ab6a0b23b1573003: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>)> > @ 0x285472 -> 0x889390 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>)> @ 0x2f3f6c -> 0x889390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
