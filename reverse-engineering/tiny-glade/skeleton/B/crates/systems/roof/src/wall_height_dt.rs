// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: wall_height_dt.rs.assembly.json.gz

// FUNCTION system_roof::wall_height_dt::wall_height_dt
// Candidate interfaces: 5588b2b104c6a7f0f14c923c
// Static incoming reference records: 3; complete records in enriched/assembly sidecars.
// Compiled procedure 3459:260 -> rva_21a2b60_len_23f
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21a2b60_len_23f: RVA 0x21a2b60, 575 PDB bytes; 142 instructions
// Evidence: _assembly/roof-v1/B/pack-001/rva_21a2b60_len_23f.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3459:260: system_roof::wall_height_dt::wall_height_dt
// Frames: F1
// 0x1421a2b60  4157                           pushq	%r15
// 0x1421a2b62  4156                           pushq	%r14
// 0x1421a2b64  4155                           pushq	%r13
// 0x1421a2b66  4154                           pushq	%r12
// 0x1421a2b68  56                             pushq	%rsi
// 0x1421a2b69  57                             pushq	%rdi
// 0x1421a2b6a  53                             pushq	%rbx
// Frames: F2 -> F3 -> F4 -> F5 -> F6
// 0x1421a2b6b  488b01                         movq	(%rcx), %rax
// 0x1421a2b6e  488b5108                       movq	0x8(%rcx), %rdx
// 0x1421a2b72  8b4914                         movl	0x14(%rcx), %ecx
// Frames: F7 -> F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F5 -> F6
// 0x1421a2b75  4c8b80f8000000                 movq	0xf8(%rax), %r8
// Frames: F11 -> F12 -> F13 -> F14 -> F15 -> F5 -> F6
// 0x1421a2b7c  4c8b8800010000                 movq	0x100(%rax), %r9
// Frames: F16 -> F17 -> F18 -> F13 -> F14 -> F15 -> F5 -> F6
// 0x1421a2b83  4f8d0c88                       leaq	(%r8,%r9,4), %r9
// 0x1421a2b87  80b80801000000                 cmpb	$0x0, 0x108(%rax)
// 0x1421a2b8e  740b                           je	0x1421a2b9b <.text+0x21a1b9b>
// 0x1421a2b90  31ff                           xorl	%edi, %edi
// 0x1421a2b92  31f6                           xorl	%esi, %esi
// 0x1421a2b94  4531d2                         xorl	%r10d, %r10d
// 0x1421a2b97  31db                           xorl	%ebx, %ebx
// 0x1421a2b99  eb51                           jmp	0x1421a2bec <.text+0x21a1bec>
// 0x1421a2b9b  be08000000                     movl	$0x8, %esi
// 0x1421a2ba0  4531db                         xorl	%r11d, %r11d
// 0x1421a2ba3  31db                           xorl	%ebx, %ebx
// 0x1421a2ba5  4531d2                         xorl	%r10d, %r10d
// 0x1421a2ba8  4531f6                         xorl	%r14d, %r14d
// 0x1421a2bab  e952010000                     jmp	0x1421a2d02 <.text+0x21a1d02>
// 0x1421a2bb0  31ff                           xorl	%edi, %edi
// 0x1421a2bb2  31db                           xorl	%ebx, %ebx
// Frames: F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F6
// 0x1421a2bb4  4189de                         movl	%ebx, %r14d
// Frames: F16 -> F26 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F6
// 0x1421a2bb7  4f8d3c76                       leaq	(%r14,%r14,2), %r15
// 0x1421a2bbb  48ffc3                         incq	%rbx
// Frames: F27
// 0x1421a2bbe  4d6be658                       imulq	$0x58, %r14, %r12
// 0x1421a2bc2  f3420f1044274c                 movss	0x4c(%rdi,%r12), %xmm0
// 0x1421a2bc9  f3420f5c44fe04                 subss	0x4(%rsi,%r15,8), %xmm0
// Frames: F28 -> F29 -> F27
// 0x1421a2bd0  43890cb3                       movl	%ecx, (%r11,%r14,4)
// Frames: F27
// 0x1421a2bd4  f3420f1104fe                   movss	%xmm0, (%rsi,%r15,8)
// Frames: F30
// 0x1421a2bda  f3420f1044274c                 movss	0x4c(%rdi,%r12), %xmm0
// Frames: F28 -> F29 -> F30
// 0x1421a2be1  43890cb3                       movl	%ecx, (%r11,%r14,4)
// Frames: F30
// 0x1421a2be5  f3420f1144fe04                 movss	%xmm0, 0x4(%rsi,%r15,8)
// Frames: F31 -> F25 -> F6
// 0x1421a2bec  4c39d3                         cmpq	%r10, %rbx
// 0x1421a2bef  75c3                           jne	0x1421a2bb4 <.text+0x21a1bb4>
// Frames: F32 -> F33 -> F34 -> F25 -> F6
// 0x1421a2bf1  4d39c8                         cmpq	%r9, %r8
// Frames: F35 -> F34 -> F25 -> F6
// 0x1421a2bf4  0f8499010000                   je	0x1421a2d93 <.text+0x21a1d93>
// 0x1421a2bfa  4c8b9aa8010000                 movq	0x1a8(%rdx), %r11
// 0x1421a2c01  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F34 -> F25 -> F6
// 0x1421a2c10  458b10                         movl	(%r8), %r10d
// Frames: F36 -> F37 -> F38 -> F39 -> F25 -> F6
// 0x1421a2c13  4b8d34d2                       leaq	(%r10,%r10,8), %rsi
// Frames: F40 -> F41 -> F42 -> F43 -> F25 -> F6
// 0x1421a2c17  4d8b54f310                     movq	0x10(%r11,%rsi,8), %r10
// Frames: F43 -> F25 -> F6
// 0x1421a2c1c  4d85d2                         testq	%r10, %r10
// 0x1421a2c1f  750f                           jne	0x1421a2c30 <.text+0x21a1c30>
// Frames: F32 -> F33 -> F34 -> F25 -> F6
// 0x1421a2c21  4983c004                       addq	$0x4, %r8
// 0x1421a2c25  4d39c8                         cmpq	%r9, %r8
// 0x1421a2c28  75e6                           jne	0x1421a2c10 <.text+0x21a1c10>
// 0x1421a2c2a  e964010000                     jmp	0x1421a2d93 <.text+0x21a1d93>
// 0x1421a2c2f  90                             nop
// Frames: F44 -> F25 -> F6
// 0x1421a2c30  4d8d3cf3                       leaq	(%r11,%rsi,8), %r15
// 0x1421a2c34  4c8b9810010000                 movq	0x110(%rax), %r11
// 0x1421a2c3b  488b9818010000                 movq	0x118(%rax), %rbx
// Frames: F45 -> F46 -> F47 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c42  498b7f18                       movq	0x18(%r15), %rdi
// 0x1421a2c46  4d8b7738                       movq	0x38(%r15), %r14
// Frames: F49 -> F50 -> F51 -> F52 -> F45 -> F46 -> F47 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c4a  4f8b1cde                       movq	(%r14,%r11,8), %r11
// Frames: F53 -> F54 -> F55 -> F45 -> F46 -> F47 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c4e  49f7d3                         notq	%r11
// Frames: F56 -> F57 -> F58 -> F54 -> F55 -> F45 -> F46 -> F47 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c51  4f8d1c5b                       leaq	(%r11,%r11,2), %r11
// 0x1421a2c55  49c1e304                       shlq	$0x4, %r11
// Frames: F59 -> F60 -> F61 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c59  4a8b741f10                     movq	0x10(%rdi,%r11), %rsi
// Frames: F62 -> F63 -> F64 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c5e  4e8b5c1f28                     movq	0x28(%rdi,%r11), %r11
// Frames: F36 -> F37 -> F52 -> F45 -> F46 -> F65 -> F66 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c63  4983c004                       addq	$0x4, %r8
// 0x1421a2c67  49395f40                       cmpq	%rbx, 0x40(%r15)
// 0x1421a2c6b  0f863fffffff                   jbe	0x1421a2bb0 <.text+0x21a1bb0>
// Frames: F49 -> F50 -> F67 -> F52 -> F45 -> F46 -> F65 -> F66 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c71  498b1cde                       movq	(%r14,%rbx,8), %rbx
// 0x1421a2c75  4885db                         testq	%rbx, %rbx
// Frames: F67 -> F52 -> F45 -> F46 -> F65 -> F66 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c78  0f8432ffffff                   je	0x1421a2bb0 <.text+0x21a1bb0>
// Frames: F53 -> F54 -> F55 -> F45 -> F46 -> F65 -> F66 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c7e  48f7d3                         notq	%rbx
// Frames: F59 -> F60 -> F68 -> F55 -> F69 -> F66 -> F48 -> F44 -> F25 -> F6
// 0x1421a2c81  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// 0x1421a2c85  48c1e304                       shlq	$0x4, %rbx
// 0x1421a2c89  488b7c1f10                     movq	0x10(%rdi,%rbx), %rdi
// 0x1421a2c8e  e91fffffff                     jmp	0x1421a2bb2 <.text+0x21a1bb2>
// 0x1421a2c93  4531db                         xorl	%r11d, %r11d
// Frames: F7 -> F8 -> F9 -> F10 -> F11 -> F12 -> F70 -> F71 -> F25 -> F6
// 0x1421a2c96  4c8bb010010000                 movq	0x110(%rax), %r14
// 0x1421a2c9d  4a8b1cf3                       movq	(%rbx,%r14,8), %rbx
// 0x1421a2ca1  48f7d3                         notq	%rbx
// 0x1421a2ca4  4c8d345b                       leaq	(%rbx,%rbx,2), %r14
// 0x1421a2ca8  49c1e604                       shlq	$0x4, %r14
// 0x1421a2cac  4a8b5c3710                     movq	0x10(%rdi,%r14), %rbx
// 0x1421a2cb1  4a8b7c3728                     movq	0x28(%rdi,%r14), %rdi
// 0x1421a2cb6  4983c004                       addq	$0x4, %r8
// 0x1421a2cba  488b7650                       movq	0x50(%rsi), %rsi
// 0x1421a2cbe  4531f6                         xorl	%r14d, %r14d
// Frames: F72 -> F73 -> F25 -> F6
// 0x1421a2cc1  4d89f7                         movq	%r14, %r15
// 0x1421a2cc4  49c1e704                       shlq	$0x4, %r15
// Frames: F72 -> F74 -> F25 -> F6
// 0x1421a2cc8  468b7c3e08                     movl	0x8(%rsi,%r15), %r15d
// Frames: F16 -> F75 -> F20 -> F21 -> F22 -> F23 -> F76 -> F25 -> F6
// 0x1421a2ccd  4f8d247f                       leaq	(%r15,%r15,2), %r12
// 0x1421a2cd1  49ffc6                         incq	%r14
// Frames: F27
// 0x1421a2cd4  4d6bef58                       imulq	$0x58, %r15, %r13
// 0x1421a2cd8  f3430f10442b4c                 movss	0x4c(%r11,%r13), %xmm0
// 0x1421a2cdf  f3420f5c44e304                 subss	0x4(%rbx,%r12,8), %xmm0
// Frames: F28 -> F29 -> F27
// 0x1421a2ce6  42890cbf                       movl	%ecx, (%rdi,%r15,4)
// Frames: F27
// 0x1421a2cea  f3420f1104e3                   movss	%xmm0, (%rbx,%r12,8)
// Frames: F30
// 0x1421a2cf0  f3430f10442b4c                 movss	0x4c(%r11,%r13), %xmm0
// Frames: F28 -> F29 -> F30
// 0x1421a2cf7  42890cbf                       movl	%ecx, (%rdi,%r15,4)
// Frames: F30
// 0x1421a2cfb  f3420f1144e304                 movss	%xmm0, 0x4(%rbx,%r12,8)
// 0x1421a2d02  4d39d6                         cmpq	%r10, %r14
// 0x1421a2d05  75ba                           jne	0x1421a2cc1 <.text+0x21a1cc1>
// Frames: F32 -> F33 -> F77 -> F25 -> F6
// 0x1421a2d07  4d39c8                         cmpq	%r9, %r8
// Frames: F35 -> F77 -> F25 -> F6
// 0x1421a2d0a  0f8483000000                   je	0x1421a2d93 <.text+0x21a1d93>
// 0x1421a2d10  488bb200010000                 movq	0x100(%rdx), %rsi
// 0x1421a2d17  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F77 -> F25 -> F6
// 0x1421a2d20  458b10                         movl	(%r8), %r10d
// Frames: F36 -> F37 -> F78 -> F79 -> F25 -> F6
// 0x1421a2d23  4f8d1c92                       leaq	(%r10,%r10,4), %r11
// 0x1421a2d27  49c1e305                       shlq	$0x5, %r11
// Frames: F40 -> F41 -> F80 -> F81 -> F25 -> F6
// 0x1421a2d2b  4e8b541e58                     movq	0x58(%rsi,%r11), %r10
// Frames: F81 -> F25 -> F6
// 0x1421a2d30  4d85d2                         testq	%r10, %r10
// 0x1421a2d33  750b                           jne	0x1421a2d40 <.text+0x21a1d40>
// Frames: F32 -> F33 -> F77 -> F25 -> F6
// 0x1421a2d35  4983c004                       addq	$0x4, %r8
// 0x1421a2d39  4d39c8                         cmpq	%r9, %r8
// Frames: F35 -> F77 -> F25 -> F6
// 0x1421a2d3c  75e2                           jne	0x1421a2d20 <.text+0x21a1d20>
// 0x1421a2d3e  eb53                           jmp	0x1421a2d93 <.text+0x21a1d93>
// Frames: F82 -> F83 -> F25 -> F6
// 0x1421a2d40  4c01de                         addq	%r11, %rsi
// 0x1421a2d43  448b9e94000000                 movl	0x94(%rsi), %r11d
// Frames: F36 -> F37 -> F84 -> F83 -> F25 -> F6
// 0x1421a2d4a  4c8bb2a8010000                 movq	0x1a8(%rdx), %r14
// 0x1421a2d51  4f8d3cdb                       leaq	(%r11,%r11,8), %r15
// Frames: F85 -> F25 -> F6
// 0x1421a2d55  4c8b9818010000                 movq	0x118(%rax), %r11
// Frames: F45 -> F46 -> F47 -> F86 -> F87 -> F85 -> F25 -> F6
// 0x1421a2d5c  4b8b7cfe18                     movq	0x18(%r14,%r15,8), %rdi
// 0x1421a2d61  4b8b5cfe38                     movq	0x38(%r14,%r15,8), %rbx
// Frames: F36 -> F37 -> F52 -> F45 -> F46 -> F65 -> F66 -> F88 -> F87 -> F85 -> F25 -> F6
// 0x1421a2d66  4f395cfe40                     cmpq	%r11, 0x40(%r14,%r15,8)
// 0x1421a2d6b  0f8622ffffff                   jbe	0x1421a2c93 <.text+0x21a1c93>
// Frames: F49 -> F50 -> F67 -> F52 -> F45 -> F46 -> F65 -> F66 -> F88 -> F87 -> F85 -> F25 -> F6
// 0x1421a2d71  4e8b1cdb                       movq	(%rbx,%r11,8), %r11
// 0x1421a2d75  4d85db                         testq	%r11, %r11
// Frames: F67 -> F52 -> F45 -> F46 -> F65 -> F66 -> F88 -> F87 -> F85 -> F25 -> F6
// 0x1421a2d78  0f8415ffffff                   je	0x1421a2c93 <.text+0x21a1c93>
// Frames: F53 -> F54 -> F55 -> F45 -> F46 -> F65 -> F66 -> F88 -> F87 -> F85 -> F25 -> F6
// 0x1421a2d7e  49f7d3                         notq	%r11
// Frames: F59 -> F60 -> F68 -> F55 -> F69 -> F66 -> F88 -> F87 -> F85 -> F25 -> F6
// 0x1421a2d81  4f8d1c5b                       leaq	(%r11,%r11,2), %r11
// 0x1421a2d85  49c1e304                       shlq	$0x4, %r11
// 0x1421a2d89  4e8b5c1f10                     movq	0x10(%rdi,%r11), %r11
// 0x1421a2d8e  e903ffffff                     jmp	0x1421a2c96 <.text+0x21a1c96>
// Frames: F89
// 0x1421a2d93  5b                             popq	%rbx
// 0x1421a2d94  5f                             popq	%rdi
// 0x1421a2d95  5e                             popq	%rsi
// 0x1421a2d96  415c                           popq	%r12
// 0x1421a2d98  415d                           popq	%r13
// 0x1421a2d9a  415e                           popq	%r14
// 0x1421a2d9c  415f                           popq	%r15
// 0x1421a2d9e  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/wall_height_dt.rs:4:0 system_roof::wall_height_dt::wall_height_dt::h1629aca14a9c81d9
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:640:0 bevy_ecs::system::query::Query::copy_unsafe
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:622:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:604:0 bevy_ecs::system::query::Query::reborrow
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:702:0 bevy_ecs::system::query::Query::iter_mut
// F6: crates/systems/roof/src/wall_height_dt.rs:5:0 system_roof::wall_height_dt::wall_height_dt::h1629aca14a9c81d9
// F7: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F8: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F10: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F11: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F13: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2372:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F14: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:52:0 bevy_ecs::query::iter::QueryIter::new
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:2579:0 bevy_ecs::system::query::Query::reborrow
// F16: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 alloc::vec::impl$8::deref
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:129:0 bevy_ecs::storage::table::TableRow::as_usize
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1608:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1601:0 bevy_ecs::query::fetch::impl$37::fetch
// F23: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2090:0 bevy_ecs::query::fetch::impl$61::fetch
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2498:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F25: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:889:0 bevy_ecs::query::iter::impl$1::next
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ecs::storage::table::TableRow::as_usize
// F27: crates/systems/roof/src/wall_height_dt.rs:6:0 system_roof::wall_height_dt::wall_height_dt::h1629aca14a9c81d9
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F30: crates/systems/roof/src/wall_height_dt.rs:7:0 system_roof::wall_height_dt::wall_height_dt::h1629aca14a9c81d9
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2467:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F32: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:1623:0 core::ptr::non_null::impl$14::eq
// F33: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$182::next
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2468:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$182::next
// F36: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F37: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2469:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F40: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2681:0 alloc::vec::Vec::len
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2706:0 alloc::vec::Vec::is_empty
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:616:0 bevy_ecs::storage::table::Table::is_empty
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2470:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2476:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1544:0 bevy_ecs::query::fetch::impl$36::set_table
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$2::set_table
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F51: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::option::Option::and_then
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F55: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F57: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:256:0 core::slice::index::impl$2::get_unchecked
// F58: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 nonmax::NonMaxUsize::get
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:304:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1546:0 bevy_ecs::query::fetch::impl$36::set_table
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs:305:0 bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:320:0 bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1548:0 bevy_ecs::query::fetch::impl$36::set_table
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:367:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1181:0 bevy_ecs::query::fetch::impl$26::set_table
// F67: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::slice::impl$0::get
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:467:0 bevy_ecs::archetype::Archetype::entities
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2528:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:326:0 bevy_ecs::archetype::ArchetypeEntity::table_row
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2540:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2555:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2552:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2506:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:896:0 bevy_ecs::archetype::Archetypes::get
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2507:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:616:0 bevy_ecs::archetype::Archetype::is_empty
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2508:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:461:0 bevy_ecs::archetype::Archetype::table_id
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2511:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::archetype::Archetype::table_id
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2515:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1533:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1169:0 bevy_ecs::query::fetch::impl$26::set_archetype
// F89: crates/systems/roof/src/wall_height_dt.rs:9:0 system_roof::wall_height_dt::wall_height_dt::h1629aca14a9c81d9
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/wall_height_dt.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6748f9039281e29b600a1a2ca8e78ed62413960c4d7b129a55e01b85c295a83c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_height_dt.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3459
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::wall_height_dt::wall_height_dt
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3459,"record_offset":260,"rva":35269472,"va":5403978592}
// 编译实例 RVAs: 0x21a2b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3459,"record_offset":3668,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3459,"record_offset":796,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3459,"record_offset":396,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3459,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3459,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3459,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3459,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":260,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3459,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3459,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":260,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3459,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":260,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3459,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3459,"record_offset":2276,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3459,"record_offset":2856,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3459,"record_offset":2940,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3459,"record_offset":2472,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3459,"record_offset":2772,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3459,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":260,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3459,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":260,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3459,"record_offset":3168,"parent_record_offset":3032,"ancestor_record_offset":260,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3459,"record_offset":1812,"parent_record_offset":1324,"ancestor_record_offset":260,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3459,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":260,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3459,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":260,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3459,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":260,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3459,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":260,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3459,"record_offset":908,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3459,"record_offset":500,"parent_record_offset":416,"ancestor_record_offset":260,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3459,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":260,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3459,"record_offset":828,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3459,"record_offset":3032,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3459,"record_offset":1324,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3459,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":260,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3459,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":260,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3459,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":260,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3459,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":260,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3459,"record_offset":1380,"parent_record_offset":1348,"ancestor_record_offset":260,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3459,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":260,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3459,"record_offset":1240,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3459,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":260,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3459,"record_offset":1156,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3459,"record_offset":1752,"parent_record_offset":1348,"ancestor_record_offset":260,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3459,"record_offset":1696,"parent_record_offset":1348,"ancestor_record_offset":260,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3459,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":260,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3459,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":260,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3459,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":260,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3459,"record_offset":416,"parent_record_offset":396,"ancestor_record_offset":260,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3459,"record_offset":444,"parent_record_offset":416,"ancestor_record_offset":260,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3459,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":260,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3459,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":260,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3459,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":260,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3459,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":260,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3459,"record_offset":1540,"parent_record_offset":1404,"ancestor_record_offset":260,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3459,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3459,"record_offset":1120,"parent_record_offset":1084,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3459,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3459,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":260,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3459,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":260,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3459,"record_offset":1620,"parent_record_offset":1588,"ancestor_record_offset":260,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3459,"record_offset":720,"parent_record_offset":548,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3459,"record_offset":1084,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03459.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3459,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":260,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03459.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_roof::wall_height_dt::wall_height_dt
// 信息 ID: 5ae68467ef4638576136c3e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5588b2b104c6a7f0f14c923c: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)> > > @ 0x1a2ac2e -> 0x21a2b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)> > > @ 0x1a2daee -> 0x21a2b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)> > @ 0x1a35855 -> 0x21a2b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
