// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: wall_height_dt.rs.assembly.json.gz

// FUNCTION system_roof::wall_height_dt::wall_height_dt
// Candidate interfaces: 5588b2b104c6a7f0f14c923c
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3190:248 -> rva_1983280_len_100
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1983280_len_100: RVA 0x1983280, 256 PDB bytes; 66 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_1983280_len_100.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3190:248: system_roof::wall_height_dt::wall_height_dt
// Frames: F1
// 0x141983280  4157                           pushq	%r15
// 0x141983282  4156                           pushq	%r14
// 0x141983284  4154                           pushq	%r12
// 0x141983286  56                             pushq	%rsi
// 0x141983287  57                             pushq	%rdi
// 0x141983288  53                             pushq	%rbx
// Frames: F2 -> F3
// 0x141983289  488b01                         movq	(%rcx), %rax
// Frames: F4 -> F3
// 0x14198328c  488b5108                       movq	0x8(%rcx), %rdx
// 0x141983290  8b4914                         movl	0x14(%rcx), %ecx
// Frames: F5 -> F6 -> F7 -> F4 -> F3
// 0x141983293  4c8b80c8000000                 movq	0xc8(%rax), %r8
// 0x14198329a  4c8b88d0000000                 movq	0xd0(%rax), %r9
// Frames: F8 -> F9 -> F10 -> F5 -> F6 -> F7 -> F4 -> F3
// 0x1419832a1  4f8d0c88                       leaq	(%r8,%r9,4), %r9
// 0x1419832a5  4531db                         xorl	%r11d, %r11d
// Frames: F3
// 0x1419832a8  31ff                           xorl	%edi, %edi
// 0x1419832aa  4531d2                         xorl	%r10d, %r10d
// 0x1419832ad  31db                           xorl	%ebx, %ebx
// 0x1419832af  e98a000000                     jmp	0x14198333e <.text+0x198233e>
// 0x1419832b4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F11 -> F12 -> F13 -> F14 -> F15 -> F16 -> F3
// 0x1419832c0  498d3cf3                       leaq	(%r11,%rsi,8), %rdi
// 0x1419832c4  488b98d8000000                 movq	0xd8(%rax), %rbx
// 0x1419832cb  4c8bb0e0000000                 movq	0xe0(%rax), %r14
// 0x1419832d2  4d8b5cf338                     movq	0x38(%r11,%rsi,8), %r11
// 0x1419832d7  498b34db                       movq	(%r11,%rbx,8), %rsi
// 0x1419832db  48f7d6                         notq	%rsi
// 0x1419832de  4b8b1cf3                       movq	(%r11,%r14,8), %rbx
// 0x1419832e2  48f7d3                         notq	%rbx
// 0x1419832e5  488b7f18                       movq	0x18(%rdi), %rdi
// 0x1419832e9  488d3476                       leaq	(%rsi,%rsi,2), %rsi
// 0x1419832ed  48c1e605                       shlq	$0x5, %rsi
// 0x1419832f1  4c8b5c3710                     movq	0x10(%rdi,%rsi), %r11
// 0x1419832f6  488b743750                     movq	0x50(%rdi,%rsi), %rsi
// 0x1419832fb  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// 0x1419832ff  48c1e305                       shlq	$0x5, %rbx
// 0x141983303  488b7c1f10                     movq	0x10(%rdi,%rbx), %rdi
// 0x141983308  31db                           xorl	%ebx, %ebx
// Frames: F17 -> F18 -> F19 -> F20 -> F16 -> F3
// 0x14198330a  4189de                         movl	%ebx, %r14d
// Frames: F8 -> F21 -> F18 -> F19 -> F20 -> F16 -> F3
// 0x14198330d  4f8d3c76                       leaq	(%r14,%r14,2), %r15
// Frames: F22
// 0x141983311  4f8d24b6                       leaq	(%r14,%r14,4), %r12
// 0x141983315  49c1e404                       shlq	$0x4, %r12
// 0x141983319  f3420f10442740                 movss	0x40(%rdi,%r12), %xmm0
// 0x141983320  0f28c8                         movaps	%xmm0, %xmm1
// 0x141983323  f3430f5c4cfb04                 subss	0x4(%r11,%r15,8), %xmm1
// Frames: F23 -> F16 -> F3
// 0x14198332a  48ffc3                         incq	%rbx
// Frames: F22
// 0x14198332d  f3430f110cfb                   movss	%xmm1, (%r11,%r15,8)
// Frames: F24 -> F25 -> F26
// 0x141983333  42890cb6                       movl	%ecx, (%rsi,%r14,4)
// Frames: F26
// 0x141983337  f3430f1144fb04                 movss	%xmm0, 0x4(%r11,%r15,8)
// Frames: F27 -> F16 -> F3
// 0x14198333e  4c39d3                         cmpq	%r10, %rbx
// 0x141983341  75c7                           jne	0x14198330a <.text+0x198230a>
// 0x141983343  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F28 -> F29 -> F30 -> F16 -> F3
// 0x141983350  4d39c8                         cmpq	%r9, %r8
// Frames: F31 -> F30 -> F16 -> F3
// 0x141983353  7421                           je	0x141983376 <.text+0x1982376>
// Frames: F30 -> F16 -> F3
// 0x141983355  458b10                         movl	(%r8), %r10d
// Frames: F32 -> F33 -> F34 -> F30 -> F16 -> F3
// 0x141983358  4983c004                       addq	$0x4, %r8
// Frames: F35 -> F36 -> F16 -> F3
// 0x14198335c  4c8b9a28010000                 movq	0x128(%rdx), %r11
// Frames: F37 -> F38 -> F35 -> F36 -> F16 -> F3
// 0x141983363  4b8d34d2                       leaq	(%r10,%r10,8), %rsi
// Frames: F39 -> F40 -> F16 -> F3
// 0x141983367  4d8b54f310                     movq	0x10(%r11,%rsi,8), %r10
// Frames: F27 -> F16 -> F3
// 0x14198336c  4d85d2                         testq	%r10, %r10
// 0x14198336f  74df                           je	0x141983350 <.text+0x1982350>
// 0x141983371  e94affffff                     jmp	0x1419832c0 <.text+0x19822c0>
// Frames: F41
// 0x141983376  5b                             popq	%rbx
// 0x141983377  5f                             popq	%rdi
// 0x141983378  5e                             popq	%rsi
// 0x141983379  415c                           popq	%r12
// 0x14198337b  415e                           popq	%r14
// 0x14198337d  415f                           popq	%r15
// 0x14198337f  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/wall_height_dt.rs:4:0 system_roof::wall_height_dt::wall_height_dt::hb8c8180d9df58134
// F2: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:476:0 bevy_ecs::system::query::Query::iter_mut
// F3: crates/systems/roof/src/wall_height_dt.rs:5:0 system_roof::wall_height_dt::wall_height_dt::hb8c8180d9df58134
// F4: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:477:0 bevy_ecs::system::query::Query::iter_mut
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1533:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F6: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:49:0 bevy_ecs::query::iter::QueryIter::new
// F7: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:1246:0 bevy_ecs::query::state::QueryState::iter_unchecked_manual
// F8: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F10: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F11: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:412:0 bevy_ecs::storage::blob_vec::BlobVec::get_slice
// F12: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:322:0 bevy_ecs::storage::table::Column::get_data_slice
// F13: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:957:0 bevy_ecs::query::fetch::impl$21::set_table
// F14: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:196:0 bevy_ecs::query::world_query::impl$2::set_table
// F15: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1602:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F16: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:886:0 bevy_ecs::query::iter::impl$1::next
// F17: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:127:0 bevy_ecs::storage::table::TableRow::as_usize
// F18: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1317:0 bevy_ecs::query::fetch::impl$31::fetch
// F19: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:208:0 bevy_ecs::query::world_query::impl$2::fetch
// F20: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1625:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F21: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs:506:0 bevy_ecs::storage::table::TableRow::as_usize
// F22: crates/systems/roof/src/wall_height_dt.rs:6:0 system_roof::wall_height_dt::wall_height_dt::hb8c8180d9df58134
// F23: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1627:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F24: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$49::set_changed
// F25: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$50::deref_mut
// F26: crates/systems/roof/src/wall_height_dt.rs:7:0 system_roof::wall_height_dt::wall_height_dt::hb8c8180d9df58134
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1596:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F28: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:1798:0 core::ptr::non_null::impl$12::eq
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$181::next
// F30: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1597:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F31: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:162:0 core::slice::iter::impl$181::next
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F34: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F35: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F36: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1598:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F37: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F38: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F39: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:569:0 bevy_ecs::storage::table::Table::entities
// F40: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1605:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F41: crates/systems/roof/src/wall_height_dt.rs:9:0 system_roof::wall_height_dt::wall_height_dt::hb8c8180d9df58134
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/wall_height_dt.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6748f9039281e29b600a1a2ca8e78ed62413960c4d7b129a55e01b85c295a83c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_height_dt.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3190
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::wall_height_dt::wall_height_dt
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3190,"record_offset":248,"rva":26751616,"va":5395460736}
// 编译实例 RVAs: 0x1983280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3190,"record_offset":1140,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3190,"record_offset":580,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3190,"record_offset":384,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3190,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":248,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3190,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":248,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3190,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":248,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3190,"record_offset":432,"parent_record_offset":408,"ancestor_record_offset":248,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3190,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":248,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3190,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3190,"record_offset":408,"parent_record_offset":384,"ancestor_record_offset":248,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3190,"record_offset":776,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3190,"record_offset":664,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3190,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":248,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3190,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":248,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3190,"record_offset":1104,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3190,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":248,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3190,"record_offset":1020,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3190,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3190,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3190,"record_offset":932,"parent_record_offset":896,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3190,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3190,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3190,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":248,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3190,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3190,"record_offset":960,"parent_record_offset":896,"ancestor_record_offset":248,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3190,"record_offset":896,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03190.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_height_dt.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_height_dt.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_height_dt.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_roof::wall_height_dt::wall_height_dt
// 信息 ID: 8dfbb031b8be446f5ecc8cea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5588b2b104c6a7f0f14c923c: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)> > @ 0x12d1ccb -> 0x1983280 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)> @ 0x12de0e0 -> 0x1983280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
