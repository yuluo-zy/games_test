// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: roof_animation.rs.assembly.json.gz

// FUNCTION system_roof::visual::roof_animation::roof_animation
// Candidate interfaces: 79706e80d24ef99b821c9ef4
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3191:248 -> rva_1983380_len_1f6
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1983380_len_1f6: RVA 0x1983380, 502 PDB bytes; 121 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_1983380_len_1f6.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3191:248: system_roof::visual::roof_animation::roof_animation
// Frames: F1
// 0x141983380  4157                           pushq	%r15
// 0x141983382  4156                           pushq	%r14
// 0x141983384  56                             pushq	%rsi
// 0x141983385  57                             pushq	%rdi
// 0x141983386  53                             pushq	%rbx
// 0x141983387  4883ec40                       subq	$0x40, %rsp
// 0x14198338b  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141983391  440f29442420                   movaps	%xmm8, 0x20(%rsp)
// 0x141983397  0f297c2410                     movaps	%xmm7, 0x10(%rsp)
// 0x14198339c  0f293424                       movaps	%xmm6, (%rsp)
// Frames: F2 -> F3
// 0x1419833a0  488b01                         movq	(%rcx), %rax
// Frames: F4 -> F3
// 0x1419833a3  4c8b4108                       movq	0x8(%rcx), %r8
// 0x1419833a7  8b4914                         movl	0x14(%rcx), %ecx
// Frames: F5 -> F6 -> F7 -> F4 -> F3
// 0x1419833aa  4c8b88c8000000                 movq	0xc8(%rax), %r9
// 0x1419833b1  4c8b90d0000000                 movq	0xd0(%rax), %r10
// Frames: F8 -> F9 -> F10 -> F5 -> F6 -> F7 -> F4 -> F3
// 0x1419833b8  4f8d1491                       leaq	(%r9,%r10,4), %r10
// 0x1419833bc  488b12                         movq	(%rdx), %rdx
// 0x1419833bf  4531db                         xorl	%r11d, %r11d
// 0x1419833c2  f30f100566f7d600               movss	0xd6f766(%rip), %xmm0   # 0x1426f2b30
// 0x1419833ca  f30f100d62f7d600               movss	0xd6f762(%rip), %xmm1   # 0x1426f2b34
// 0x1419833d2  f30f101532908d00               movss	0x8d9032(%rip), %xmm2   # 0x14225c40c
// 0x1419833da  0f57db                         xorps	%xmm3, %xmm3
// 0x1419833dd  f30f1025db537c00               movss	0x7c53db(%rip), %xmm4   # 0x1421487c0
// 0x1419833e5  f30f102d37b49000               movss	0x90b437(%rip), %xmm5   # 0x14228e824
// 0x1419833ed  f30f1035b3e98400               movss	0x84e9b3(%rip), %xmm6   # 0x1421d1da8
// 0x1419833f5  4531f6                         xorl	%r14d, %r14d
// 0x1419833f8  31ff                           xorl	%edi, %edi
// Frames: F3
// 0x1419833fa  31f6                           xorl	%esi, %esi
// Frames: F11 -> F12 -> F3
// 0x1419833fc  4d39de                         cmpq	%r11, %r14
// 0x1419833ff  7429                           je	0x14198342a <.text+0x198242a>
// 0x141983401  e981000000                     jmp	0x141983487 <.text+0x1982487>
// 0x141983406  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F12 -> F3
// 0x141983410  48833cf700                     cmpq	$0x0, (%rdi,%rsi,8)
// 0x141983415  400f95c6                       setne	%sil
// Frames: F24 -> F25 -> F12 -> F3
// 0x141983419  4983c104                       addq	$0x4, %r9
// 0x14198341d  498d1cdb                       leaq	(%r11,%rbx,8), %rbx
// 0x141983421  4c8b5b10                       movq	0x10(%rbx), %r11
// Frames: F11 -> F12 -> F3
// 0x141983425  4d85db                         testq	%r11, %r11
// 0x141983428  7536                           jne	0x141983460 <.text+0x1982460>
// Frames: F26 -> F27 -> F28 -> F12 -> F3
// 0x14198342a  4d39d1                         cmpq	%r10, %r9
// Frames: F29 -> F28 -> F12 -> F3
// 0x14198342d  0f8422010000                   je	0x141983555 <.text+0x1982555>
// Frames: F28 -> F12 -> F3
// 0x141983433  418b31                         movl	(%r9), %esi
// Frames: F30 -> F31 -> F12 -> F3
// 0x141983436  4d8b9828010000                 movq	0x128(%r8), %r11
// Frames: F32 -> F33 -> F30 -> F31 -> F12 -> F3
// 0x14198343d  488d1cf6                       leaq	(%rsi,%rsi,8), %rbx
// Frames: F34 -> F35 -> F36 -> F22 -> F23 -> F12 -> F3
// 0x141983441  498b7cdb38                     movq	0x38(%r11,%rbx,8), %rdi
// Frames: F37 -> F21 -> F22 -> F23 -> F12 -> F3
// 0x141983446  488bb0e0000000                 movq	0xe0(%rax), %rsi
// Frames: F32 -> F38 -> F16 -> F17 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F12 -> F3
// 0x14198344d  493974db40                     cmpq	%rsi, 0x40(%r11,%rbx,8)
// Frames: F39 -> F16 -> F17 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F12 -> F3
// 0x141983452  77bc                           ja	0x141983410 <.text+0x1982410>
// 0x141983454  31f6                           xorl	%esi, %esi
// 0x141983456  ebc1                           jmp	0x141983419 <.text+0x1982419>
// 0x141983458  0f1f840000000000               nopl	(%rax,%rax)
// 0x141983460  4c8bb0d8000000                 movq	0xd8(%rax), %r14
// 0x141983467  4a8b3cf7                       movq	(%rdi,%r14,8), %rdi
// 0x14198346b  48f7d7                         notq	%rdi
// 0x14198346e  488b5b18                       movq	0x18(%rbx), %rbx
// 0x141983472  4c8d347f                       leaq	(%rdi,%rdi,2), %r14
// 0x141983476  49c1e605                       shlq	$0x5, %r14
// 0x14198347a  4a8b7c3310                     movq	0x10(%rbx,%r14), %rdi
// 0x14198347f  4a8b5c3350                     movq	0x50(%rbx,%r14), %rbx
// 0x141983484  4531f6                         xorl	%r14d, %r14d
// Frames: F40 -> F41 -> F42 -> F43 -> F12 -> F3
// 0x141983487  4589f7                         movl	%r14d, %r15d
// Frames: F44 -> F12 -> F3
// 0x14198348a  49ffc6                         incq	%r14
// 0x14198348d  f30f107a28                     movss	0x28(%rdx), %xmm7
// Frames: F45
// 0x141983492  440f28c7                       movaps	%xmm7, %xmm8
// Frames: F46 -> F42 -> F43 -> F12 -> F3
// 0x141983496  4084f6                         testb	%sil, %sil
// Frames: F47 -> F48 -> F42 -> F43 -> F12 -> F3
// 0x141983499  7425                           je	0x1419834c0 <.text+0x19824c0>
// Frames: F45
// 0x14198349b  f3440f59c5                     mulss	%xmm5, %xmm8
// 0x1419834a0  440f28ce                       movaps	%xmm6, %xmm9
// 0x1419834a4  f3410f59f9                     mulss	%xmm9, %xmm7
// Frames: F49
// 0x1419834a9  440f2ec3                       ucomiss	%xmm3, %xmm8
// 0x1419834ad  7551                           jne	0x141983500 <.text+0x1982500>
// 0x1419834af  7a4f                           jp	0x141983500 <.text+0x1982500>
// Frames: F50
// 0x1419834b1  0f2efb                         ucomiss	%xmm3, %xmm7
// 0x1419834b4  7577                           jne	0x14198352d <.text+0x198252d>
// 0x1419834b6  7a75                           jp	0x14198352d <.text+0x198252d>
// 0x1419834b8  e93fffffff                     jmp	0x1419833fc <.text+0x19823fc>
// 0x1419834bd  0f1f00                         nopl	(%rax)
// Frames: F51
// 0x1419834c0  f3440f59c0                     mulss	%xmm0, %xmm8
// Frames: F52
// 0x1419834c5  f3460f100cff                   movss	(%rdi,%r15,8), %xmm9
// 0x1419834cb  440f2eca                       ucomiss	%xmm2, %xmm9
// 0x1419834cf  7613                           jbe	0x1419834e4 <.text+0x19824e4>
// Frames: F49
// 0x1419834d1  440f28c9                       movaps	%xmm1, %xmm9
// 0x1419834d5  f3410f59f9                     mulss	%xmm9, %xmm7
// 0x1419834da  440f2ec3                       ucomiss	%xmm3, %xmm8
// 0x1419834de  7520                           jne	0x141983500 <.text+0x1982500>
// 0x1419834e0  7a1e                           jp	0x141983500 <.text+0x1982500>
// 0x1419834e2  ebcd                           jmp	0x1419834b1 <.text+0x19824b1>
// 0x1419834e4  0f57ff                         xorps	%xmm7, %xmm7
// 0x1419834e7  440f2ec7                       ucomiss	%xmm7, %xmm8
// 0x1419834eb  7513                           jne	0x141983500 <.text+0x1982500>
// 0x1419834ed  0f8b09ffffff                   jnp	0x1419833fc <.text+0x19823fc>
// 0x1419834f3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F53 -> F54 -> F55
// 0x141983500  42890cbb                       movl	%ecx, (%rbx,%r15,4)
// Frames: F55
// 0x141983504  f3460f5804ff                   addss	(%rdi,%r15,8), %xmm8
// Frames: F56 -> F57
// 0x14198350a  450f57c9                       xorps	%xmm9, %xmm9
// 0x14198350e  f3450f5fc8                     maxss	%xmm8, %xmm9
// Frames: F58 -> F57
// 0x141983513  440f28c4                       movaps	%xmm4, %xmm8
// 0x141983517  f3450f5dc1                     minss	%xmm9, %xmm8
// Frames: F57
// 0x14198351c  f3460f1104ff                   movss	%xmm8, (%rdi,%r15,8)
// Frames: F50
// 0x141983522  0f2efb                         ucomiss	%xmm3, %xmm7
// 0x141983525  7506                           jne	0x14198352d <.text+0x198252d>
// 0x141983527  0f8bcffeffff                   jnp	0x1419833fc <.text+0x19823fc>
// Frames: F53 -> F54 -> F59
// 0x14198352d  42890cbb                       movl	%ecx, (%rbx,%r15,4)
// Frames: F59
// 0x141983531  f3420f587cff04                 addss	0x4(%rdi,%r15,8), %xmm7
// Frames: F56 -> F60
// 0x141983538  450f57c0                       xorps	%xmm8, %xmm8
// 0x14198353c  f3440f5fc7                     maxss	%xmm7, %xmm8
// Frames: F58 -> F60
// 0x141983541  0f28fc                         movaps	%xmm4, %xmm7
// 0x141983544  f3410f5df8                     minss	%xmm8, %xmm7
// Frames: F60
// 0x141983549  f3420f117cff04                 movss	%xmm7, 0x4(%rdi,%r15,8)
// 0x141983550  e9a7feffff                     jmp	0x1419833fc <.text+0x19823fc>
// Frames: F61
// 0x141983555  0f283424                       movaps	(%rsp), %xmm6
// 0x141983559  0f287c2410                     movaps	0x10(%rsp), %xmm7
// 0x14198355e  440f28442420                   movaps	0x20(%rsp), %xmm8
// 0x141983564  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x14198356a  4883c440                       addq	$0x40, %rsp
// 0x14198356e  5b                             popq	%rbx
// 0x14198356f  5f                             popq	%rdi
// 0x141983570  5e                             popq	%rsi
// 0x141983571  415e                           popq	%r14
// 0x141983573  415f                           popq	%r15
// 0x141983575  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/roof_animation.rs:11:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F2: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:476:0 bevy_ecs::system::query::Query::iter_mut
// F3: crates/systems/roof/src/visual/roof_animation.rs:15:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F4: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:477:0 bevy_ecs::system::query::Query::iter_mut
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1533:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F6: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:49:0 bevy_ecs::query::iter::QueryIter::new
// F7: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:1246:0 bevy_ecs::query::state::QueryState::iter_unchecked_manual
// F8: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F10: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F11: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1596:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F12: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:886:0 bevy_ecs::query::iter::impl$1::next
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:608:0 core::option::Option::is_some
// F14: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:49:0 bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
// F15: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F16: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:49:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
// F17: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:416:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
// F18: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:714:0 bevy_ecs::storage::table::Table::has_column
// F19: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1548:0 bevy_ecs::query::fetch::impl$36::set_table::closure$0
// F20: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1010:0 bevy_ecs::query::fetch::impl$21::matches_component_set
// F21: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1548:0 bevy_ecs::query::fetch::impl$36::set_table
// F22: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:196:0 bevy_ecs::query::world_query::impl$3::set_table
// F23: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1602:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F24: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:569:0 bevy_ecs::storage::table::Table::entities
// F25: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1605:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F26: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:1798:0 core::ptr::non_null::impl$12::eq
// F27: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$181::next
// F28: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1597:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:162:0 core::slice::iter::impl$181::next
// F30: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F31: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1598:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F34: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F35: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:693:0 bevy_ecs::storage::table::Table::get_column
// F36: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1296:0 bevy_ecs::query::fetch::impl$31::set_table
// F37: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1007:0 bevy_ecs::query::fetch::impl$21::matches_component_set
// F38: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::option::Option::map
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::option::Option::map
// F40: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:127:0 bevy_ecs::storage::table::TableRow::as_usize
// F41: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1317:0 bevy_ecs::query::fetch::impl$31::fetch
// F42: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:208:0 bevy_ecs::query::world_query::impl$3::fetch
// F43: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1625:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F44: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1627:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F45: crates/systems/roof/src/visual/roof_animation.rs:18:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F46: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1563:0 bevy_ecs::query::fetch::impl$36::fetch
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 core::bool::impl$0::then
// F48: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1566:0 bevy_ecs::query::fetch::impl$36::fetch
// F49: crates/systems/roof/src/visual/roof_animation.rs:33:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F50: crates/systems/roof/src/visual/roof_animation.rs:38:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F51: crates/systems/roof/src/visual/roof_animation.rs:23:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F52: crates/systems/roof/src/visual/roof_animation.rs:24:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F53: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$49::set_changed
// F54: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$50::deref_mut
// F55: crates/systems/roof/src/visual/roof_animation.rs:34:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1521:0 core::f32::impl$0::clamp
// F57: crates/systems/roof/src/visual/roof_animation.rs:35:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F58: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1524:0 core::f32::impl$0::clamp
// F59: crates/systems/roof/src/visual/roof_animation.rs:39:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F60: crates/systems/roof/src/visual/roof_animation.rs:40:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// F61: crates/systems/roof/src/visual/roof_animation.rs:43:0 system_roof::visual::roof_animation::roof_animation::h13da525408572e0c
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/visual/roof_animation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9025e479720e349cbbec63222c4327a3756af450524aaf3372be9133d4595938
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3191
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::roof_animation::roof_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3191,"record_offset":248,"rva":26751872,"va":5395460992}
// 编译实例 RVAs: 0x1983380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3191,"record_offset":1448,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3191,"record_offset":592,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3191,"record_offset":396,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3191,"record_offset":1504,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3191,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":248,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3191,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":248,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3191,"record_offset":1320,"parent_record_offset":1292,"ancestor_record_offset":248,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3191,"record_offset":1028,"parent_record_offset":672,"ancestor_record_offset":248,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3191,"record_offset":1376,"parent_record_offset":1292,"ancestor_record_offset":248,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3191,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":248,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3191,"record_offset":760,"parent_record_offset":728,"ancestor_record_offset":248,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3191,"record_offset":444,"parent_record_offset":420,"ancestor_record_offset":248,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3191,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":248,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3191,"record_offset":620,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3191,"record_offset":420,"parent_record_offset":396,"ancestor_record_offset":248,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3191,"record_offset":1292,"parent_record_offset":620,"ancestor_record_offset":248,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3191,"record_offset":672,"parent_record_offset":620,"ancestor_record_offset":248,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3191,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":248,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3191,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":248,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3191,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":248,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3191,"record_offset":916,"parent_record_offset":872,"ancestor_record_offset":248,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3191,"record_offset":1116,"parent_record_offset":620,"ancestor_record_offset":248,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3191,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":248,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3191,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":248,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3191,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":248,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3191,"record_offset":1208,"parent_record_offset":620,"ancestor_record_offset":248,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3191,"record_offset":1404,"parent_record_offset":1376,"ancestor_record_offset":248,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3191,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":248,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3191,"record_offset":872,"parent_record_offset":844,"ancestor_record_offset":248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3191,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3191,"record_offset":1176,"parent_record_offset":1144,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3191,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3191,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3191,"record_offset":972,"parent_record_offset":872,"ancestor_record_offset":248,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3191,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3191,"record_offset":1144,"parent_record_offset":620,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03191.jsonl.gz"}
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
// 完整结构及原始类型记录: roof_animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_roof::visual::roof_animation::roof_animation
// 信息 ID: b500d3cba4fa6e4427e4a0fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 79706e80d24ef99b821c9ef4: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x12d87a6 -> 0x1983380 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x12de53b -> 0x1983380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
