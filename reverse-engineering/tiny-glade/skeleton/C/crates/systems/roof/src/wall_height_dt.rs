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
// Compiled procedure 3772:268 -> rva_2820e80_len_23a
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_2820e80_len_23a: RVA 0x2820e80, 570 PDB bytes; 146 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_2820e80_len_23a.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3772:268: system_roof::wall_height_dt::wall_height_dt
// Frames: F1
// 0x142820e80  4157                           pushq	%r15
// 0x142820e82  4156                           pushq	%r14
// 0x142820e84  4154                           pushq	%r12
// 0x142820e86  56                             pushq	%rsi
// 0x142820e87  57                             pushq	%rdi
// 0x142820e88  55                             pushq	%rbp
// 0x142820e89  53                             pushq	%rbx
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7
// 0x142820e8a  488b01                         movq	(%rcx), %rax
// 0x142820e8d  488b5108                       movq	0x8(%rcx), %rdx
// 0x142820e91  8b4914                         movl	0x14(%rcx), %ecx
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x142820e94  4c8b80f8000000                 movq	0xf8(%rax), %r8
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x142820e9b  4c8b8800010000                 movq	0x100(%rax), %r9
// Frames: F17 -> F18 -> F19 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x142820ea2  4f8d0c88                       leaq	(%r8,%r9,4), %r9
// 0x142820ea6  80b80801000000                 cmpb	$0x0, 0x108(%rax)
// 0x142820ead  740f                           je	0x142820ebe <.text+0x281febe>
// 0x142820eaf  4531d2                         xorl	%r10d, %r10d
// 0x142820eb2  4531db                         xorl	%r11d, %r11d
// 0x142820eb5  31ff                           xorl	%edi, %edi
// 0x142820eb7  31db                           xorl	%ebx, %ebx
// 0x142820eb9  e981010000                     jmp	0x14282103f <.text+0x282003f>
// Frames: F20 -> F21 -> F7
// 0x142820ebe  4531d2                         xorl	%r10d, %r10d
// 0x142820ec1  bb08000000                     movl	$0x8, %ebx
// 0x142820ec6  4531db                         xorl	%r11d, %r11d
// 0x142820ec9  31ff                           xorl	%edi, %edi
// 0x142820ecb  31ed                           xorl	%ebp, %ebp
// 0x142820ecd  4439d5                         cmpl	%r10d, %ebp
// 0x142820ed0  740e                           je	0x142820ee0 <.text+0x281fee0>
// Frames: F22 -> F21 -> F7
// 0x142820ed2  4189ee                         movl	%ebp, %r14d
// 0x142820ed5  ffc5                           incl	%ebp
// 0x142820ed7  e9c7000000                     jmp	0x142820fa3 <.text+0x281ffa3>
// 0x142820edc  0f1f4000                       nopl	(%rax)
// Frames: F23 -> F24 -> F25 -> F21 -> F7
// 0x142820ee0  4d39c8                         cmpq	%r9, %r8
// Frames: F24 -> F25 -> F21 -> F7
// 0x142820ee3  0f84c6010000                   je	0x1428210af <.text+0x28200af>
// 0x142820ee9  488b9a00010000                 movq	0x100(%rdx), %rbx
// Frames: F25 -> F21 -> F7
// 0x142820ef0  458b10                         movl	(%r8), %r10d
// Frames: F26 -> F27 -> F28 -> F29 -> F21 -> F7
// 0x142820ef3  4f8d1c92                       leaq	(%r10,%r10,4), %r11
// 0x142820ef7  49c1e305                       shlq	$0x5, %r11
// Frames: F30 -> F31 -> F32 -> F33 -> F21 -> F7
// 0x142820efb  4e8b541b58                     movq	0x58(%rbx,%r11), %r10
// Frames: F33 -> F21 -> F7
// 0x142820f00  4d85d2                         testq	%r10, %r10
// 0x142820f03  751b                           jne	0x142820f20 <.text+0x281ff20>
// Frames: F23 -> F24 -> F25 -> F21 -> F7
// 0x142820f05  4983c004                       addq	$0x4, %r8
// 0x142820f09  4d39c8                         cmpq	%r9, %r8
// Frames: F24 -> F25 -> F21 -> F7
// 0x142820f0c  75e2                           jne	0x142820ef0 <.text+0x281fef0>
// 0x142820f0e  e99c010000                     jmp	0x1428210af <.text+0x28200af>
// 0x142820f13  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F34 -> F35 -> F21 -> F7
// 0x142820f20  4c01db                         addq	%r11, %rbx
// 0x142820f23  448b9b94000000                 movl	0x94(%rbx), %r11d
// Frames: F26 -> F27 -> F36 -> F35 -> F21 -> F7
// 0x142820f2a  4c8bb2a0010000                 movq	0x1a0(%rdx), %r14
// 0x142820f31  4f8d3cdb                       leaq	(%r11,%r11,8), %r15
// Frames: F37 -> F21 -> F7
// 0x142820f35  4c8b9818010000                 movq	0x118(%rax), %r11
// Frames: F38 -> F39 -> F40 -> F41 -> F42 -> F37 -> F21 -> F7
// 0x142820f3c  4b8b74fe18                     movq	0x18(%r14,%r15,8), %rsi
// 0x142820f41  4b8b7cfe38                     movq	0x38(%r14,%r15,8), %rdi
// Frames: F26 -> F27 -> F43 -> F38 -> F39 -> F44 -> F45 -> F46 -> F42 -> F37 -> F21 -> F7
// 0x142820f46  4f3b5cfe40                     cmpq	0x40(%r14,%r15,8), %r11
// 0x142820f4b  7323                           jae	0x142820f70 <.text+0x281ff70>
// Frames: F47 -> F48 -> F49 -> F43 -> F38 -> F39 -> F44 -> F45 -> F46 -> F42 -> F37 -> F21 -> F7
// 0x142820f4d  4e8b1cdf                       movq	(%rdi,%r11,8), %r11
// 0x142820f51  4d85db                         testq	%r11, %r11
// 0x142820f54  741a                           je	0x142820f70 <.text+0x281ff70>
// Frames: F50 -> F51 -> F52 -> F38 -> F39 -> F44 -> F45 -> F46 -> F42 -> F37 -> F21 -> F7
// 0x142820f56  49f7d3                         notq	%r11
// Frames: F53 -> F54 -> F51 -> F52 -> F38 -> F39 -> F44 -> F45 -> F46 -> F42 -> F37 -> F21 -> F7
// 0x142820f59  4f8d1c5b                       leaq	(%r11,%r11,2), %r11
// 0x142820f5d  49c1e304                       shlq	$0x4, %r11
// Frames: F55 -> F56 -> F57 -> F52 -> F58 -> F45 -> F46 -> F42 -> F37 -> F21 -> F7
// 0x142820f61  4e8b5c1e10                     movq	0x10(%rsi,%r11), %r11
// 0x142820f66  eb0b                           jmp	0x142820f73 <.text+0x281ff73>
// 0x142820f68  0f1f840000000000               nopl	(%rax,%rax)
// 0x142820f70  4531db                         xorl	%r11d, %r11d
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F59 -> F60 -> F21 -> F7
// 0x142820f73  4c8bb010010000                 movq	0x110(%rax), %r14
// 0x142820f7a  4a8b3cf7                       movq	(%rdi,%r14,8), %rdi
// 0x142820f7e  48f7d7                         notq	%rdi
// 0x142820f81  4c8d347f                       leaq	(%rdi,%rdi,2), %r14
// 0x142820f85  49c1e604                       shlq	$0x4, %r14
// 0x142820f89  4a8b7c3610                     movq	0x10(%rsi,%r14), %rdi
// 0x142820f8e  4a8b743628                     movq	0x28(%rsi,%r14), %rsi
// 0x142820f93  4983c004                       addq	$0x4, %r8
// 0x142820f97  488b5b50                       movq	0x50(%rbx), %rbx
// 0x142820f9b  bd01000000                     movl	$0x1, %ebp
// 0x142820fa0  4531f6                         xorl	%r14d, %r14d
// Frames: F53 -> F61 -> F22 -> F21 -> F7
// 0x142820fa3  49c1e604                       shlq	$0x4, %r14
// Frames: F62 -> F63 -> F21 -> F7
// 0x142820fa7  468b743308                     movl	0x8(%rbx,%r14), %r14d
// Frames: F64 -> F65 -> F66 -> F67 -> F68 -> F69 -> F70 -> F21 -> F7
// 0x142820fac  41f7d6                         notl	%r14d
// Frames: F17 -> F71 -> F66 -> F67 -> F68 -> F69 -> F70 -> F21 -> F7
// 0x142820faf  4f8d3c76                       leaq	(%r14,%r14,2), %r15
// Frames: F17 -> F72 -> F73 -> F67 -> F74 -> F69 -> F70 -> F21 -> F7
// 0x142820fb3  4d6be658                       imulq	$0x58, %r14, %r12
// Frames: F75
// 0x142820fb7  f3430f1044234c                 movss	0x4c(%r11,%r12), %xmm0
// 0x142820fbe  0f28c8                         movaps	%xmm0, %xmm1
// 0x142820fc1  f3420f5c4cff04                 subss	0x4(%rdi,%r15,8), %xmm1
// 0x142820fc8  f3420f110cff                   movss	%xmm1, (%rdi,%r15,8)
// Frames: F76 -> F77 -> F78
// 0x142820fce  42890cb6                       movl	%ecx, (%rsi,%r14,4)
// Frames: F78
// 0x142820fd2  f3420f1144ff04                 movss	%xmm0, 0x4(%rdi,%r15,8)
// 0x142820fd9  4439d5                         cmpl	%r10d, %ebp
// Frames: F20 -> F21 -> F7
// 0x142820fdc  0f84fefeffff                   je	0x142820ee0 <.text+0x281fee0>
// 0x142820fe2  e9ebfeffff                     jmp	0x142820ed2 <.text+0x281fed2>
// 0x142820fe7  4531db                         xorl	%r11d, %r11d
// 0x142820fea  488b9810010000                 movq	0x110(%rax), %rbx
// 0x142820ff1  488b3cdf                       movq	(%rdi,%rbx,8), %rdi
// 0x142820ff5  48f7d7                         notq	%rdi
// 0x142820ff8  488d1c7f                       leaq	(%rdi,%rdi,2), %rbx
// 0x142820ffc  48c1e304                       shlq	$0x4, %rbx
// 0x142821000  488b7c1e10                     movq	0x10(%rsi,%rbx), %rdi
// 0x142821005  488b741e28                     movq	0x28(%rsi,%rbx), %rsi
// 0x14282100a  4983c004                       addq	$0x4, %r8
// 0x14282100e  31db                           xorl	%ebx, %ebx
// Frames: F79 -> F21 -> F7
// 0x142821010  4189de                         movl	%ebx, %r14d
// Frames: F17 -> F72 -> F66 -> F67 -> F68 -> F69 -> F80 -> F21 -> F7
// 0x142821013  4f8d3c76                       leaq	(%r14,%r14,2), %r15
// Frames: F17 -> F72 -> F73 -> F67 -> F74 -> F69 -> F80 -> F21 -> F7
// 0x142821017  4d6be658                       imulq	$0x58, %r14, %r12
// Frames: F75
// 0x14282101b  f3430f1044234c                 movss	0x4c(%r11,%r12), %xmm0
// 0x142821022  0f28c8                         movaps	%xmm0, %xmm1
// 0x142821025  f3420f5c4cff04                 subss	0x4(%rdi,%r15,8), %xmm1
// 0x14282102c  ffc3                           incl	%ebx
// 0x14282102e  f3420f110cff                   movss	%xmm1, (%rdi,%r15,8)
// Frames: F76 -> F77 -> F78
// 0x142821034  42890cb6                       movl	%ecx, (%rsi,%r14,4)
// Frames: F78
// 0x142821038  f3420f1144ff04                 movss	%xmm0, 0x4(%rdi,%r15,8)
// Frames: F81 -> F21 -> F7
// 0x14282103f  4439d3                         cmpl	%r10d, %ebx
// 0x142821042  75cc                           jne	0x142821010 <.text+0x2820010>
// Frames: F23 -> F24 -> F82 -> F21 -> F7
// 0x142821044  4d39c8                         cmpq	%r9, %r8
// Frames: F24 -> F82 -> F21 -> F7
// 0x142821047  7466                           je	0x1428210af <.text+0x28200af>
// 0x142821049  4c8b9aa0010000                 movq	0x1a0(%rdx), %r11
// Frames: F82 -> F21 -> F7
// 0x142821050  458b10                         movl	(%r8), %r10d
// Frames: F26 -> F27 -> F83 -> F84 -> F21 -> F7
// 0x142821053  4b8d34d2                       leaq	(%r10,%r10,8), %rsi
// Frames: F30 -> F31 -> F85 -> F86 -> F21 -> F7
// 0x142821057  4d8b54f310                     movq	0x10(%r11,%rsi,8), %r10
// Frames: F86 -> F21 -> F7
// 0x14282105c  4d85d2                         testq	%r10, %r10
// 0x14282105f  750f                           jne	0x142821070 <.text+0x2820070>
// Frames: F23 -> F24 -> F82 -> F21 -> F7
// 0x142821061  4983c004                       addq	$0x4, %r8
// 0x142821065  4d39c8                         cmpq	%r9, %r8
// Frames: F24 -> F82 -> F21 -> F7
// 0x142821068  75e6                           jne	0x142821050 <.text+0x2820050>
// 0x14282106a  eb43                           jmp	0x1428210af <.text+0x28200af>
// 0x14282106c  0f1f4000                       nopl	(%rax)
// Frames: F87 -> F21 -> F7
// 0x142821070  498d1cf3                       leaq	(%r11,%rsi,8), %rbx
// 0x142821074  4c8b9818010000                 movq	0x118(%rax), %r11
// Frames: F38 -> F39 -> F40 -> F88 -> F87 -> F21 -> F7
// 0x14282107b  488b7318                       movq	0x18(%rbx), %rsi
// 0x14282107f  488b7b38                       movq	0x38(%rbx), %rdi
// Frames: F26 -> F27 -> F43 -> F38 -> F39 -> F44 -> F45 -> F88 -> F87 -> F21 -> F7
// 0x142821083  4c3b5b40                       cmpq	0x40(%rbx), %r11
// 0x142821087  0f835affffff                   jae	0x142820fe7 <.text+0x281ffe7>
// Frames: F47 -> F48 -> F49 -> F43 -> F38 -> F39 -> F44 -> F45 -> F88 -> F87 -> F21 -> F7
// 0x14282108d  4e8b1cdf                       movq	(%rdi,%r11,8), %r11
// 0x142821091  4d85db                         testq	%r11, %r11
// 0x142821094  0f844dffffff                   je	0x142820fe7 <.text+0x281ffe7>
// Frames: F50 -> F51 -> F52 -> F38 -> F39 -> F44 -> F45 -> F88 -> F87 -> F21 -> F7
// 0x14282109a  49f7d3                         notq	%r11
// Frames: F53 -> F54 -> F51 -> F52 -> F38 -> F39 -> F44 -> F45 -> F88 -> F87 -> F21 -> F7
// 0x14282109d  4f8d1c5b                       leaq	(%r11,%r11,2), %r11
// 0x1428210a1  49c1e304                       shlq	$0x4, %r11
// Frames: F55 -> F56 -> F57 -> F52 -> F58 -> F45 -> F88 -> F87 -> F21 -> F7
// 0x1428210a5  4e8b5c1e10                     movq	0x10(%rsi,%r11), %r11
// 0x1428210aa  e93bffffff                     jmp	0x142820fea <.text+0x281ffea>
// Frames: F89
// 0x1428210af  5b                             popq	%rbx
// 0x1428210b0  5d                             popq	%rbp
// 0x1428210b1  5f                             popq	%rdi
// 0x1428210b2  5e                             popq	%rsi
// 0x1428210b3  415c                           popq	%r12
// 0x1428210b5  415e                           popq	%r14
// 0x1428210b7  415f                           popq	%r15
// 0x1428210b9  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/wall_height_dt.rs:4:0 system_roof::wall_height_dt::wall_height_dt::h3d7133e8a09ce183
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:528:0 bevy_ecs::system::query::Query::new
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:643:0 bevy_ecs::system::query::Query::copy_unsafe
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:625:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:607:0 bevy_ecs::system::query::Query::reborrow
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:705:0 bevy_ecs::system::query::Query::iter_mut
// F7: crates/systems/roof/src/wall_height_dt.rs:5:0 system_roof::wall_height_dt::wall_height_dt::h3d7133e8a09ce183
// F8: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F9: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F10: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F11: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F14: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2421:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:53:0 bevy_ecs::query::iter::QueryIter::new
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:2510:0 bevy_ecs::system::query::Query::reborrow
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F19: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 alloc::vec::impl$9::deref
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2561:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:915:0 bevy_ecs::query::iter::impl$1::next
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2593:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F24: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:180:0 core::slice::iter::impl$171::next
// F25: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2562:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:865:0 bevy_ecs::archetype::Archetypes::get
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2563:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2997:0 alloc::vec::Vec::len
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3022:0 alloc::vec::Vec::is_empty
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:656:0 bevy_ecs::archetype::Archetype::is_empty
// F33: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2564:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:479:0 bevy_ecs::archetype::Archetype::table_id
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2567:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::archetype::Archetype::table_id
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2571:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1932:0 bevy_ecs::query::fetch::impl$48::set_table
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1921:0 bevy_ecs::query::fetch::impl$48::set_archetype
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:381:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1552:0 bevy_ecs::query::fetch::impl$36::set_table
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1539:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F48: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::slice::impl$0::get
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F52: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F53: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F54: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:298:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:485:0 bevy_ecs::archetype::Archetype::entities
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2584:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F61: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 core::slice::impl$0::get_unchecked
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:347:0 bevy_ecs::archetype::ArchetypeEntity::table_row
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2615:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2000:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1993:0 bevy_ecs::query::fetch::impl$49::fetch
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2529:0 bevy_ecs::query::fetch::impl$89::fetch
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2611:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ecs::storage::table::TableRow::index
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ptr::ThinSlicePtr::get
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1609:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1604:0 bevy_ecs::query::fetch::impl$37::fetch
// F75: crates/systems/roof/src/wall_height_dt.rs:6:0 system_roof::wall_height_dt::wall_height_dt::h3d7133e8a09ce183
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F78: crates/systems/roof/src/wall_height_dt.rs:7:0 system_roof::wall_height_dt::wall_height_dt::h3d7133e8a09ce183
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2540:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2554:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2520:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2521:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2522:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:647:0 bevy_ecs::storage::table::Table::is_empty
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2523:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2529:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$2::set_table
// F89: crates/systems/roof/src/wall_height_dt.rs:9:0 system_roof::wall_height_dt::wall_height_dt::h3d7133e8a09ce183
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/wall_height_dt.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 6748f9039281e29b600a1a2ca8e78ed62413960c4d7b129a55e01b85c295a83c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_height_dt.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3772
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
// 首条位置: {"module":3772,"record_offset":268,"rva":42077824,"va":5410786944}
// 编译实例 RVAs: 0x2820e80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3772,"record_offset":3728,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3772,"record_offset":832,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3772,"record_offset":404,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3772,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3772,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3772,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3772,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3772,"record_offset":616,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3772,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":268,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3772,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3772,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3772,"record_offset":1948,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3772,"record_offset":1108,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3772,"record_offset":1192,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3772,"record_offset":2200,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3772,"record_offset":1024,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3772,"record_offset":3756,"parent_record_offset":3728,"ancestor_record_offset":268,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":1420,"parent_record_offset":1284,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":2428,"parent_record_offset":2228,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":268,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":268,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":268,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":268,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3772,"record_offset":2228,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3772,"record_offset":536,"parent_record_offset":424,"ancestor_record_offset":268,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3772,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":268,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3772,"record_offset":864,"parent_record_offset":832,"ancestor_record_offset":268,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3772,"record_offset":1284,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3772,"record_offset":3112,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":3772,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3772,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3772,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3772,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3772,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3772,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3772,"record_offset":3028,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3772,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3772,"record_offset":2944,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3772,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3772,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3772,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":268,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3772,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":268,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3772,"record_offset":424,"parent_record_offset":404,"ancestor_record_offset":268,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3772,"record_offset":452,"parent_record_offset":424,"ancestor_record_offset":268,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3772,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":268,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3772,"record_offset":1628,"parent_record_offset":1568,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3772,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3772,"record_offset":1692,"parent_record_offset":1520,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3772,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3772,"record_offset":988,"parent_record_offset":960,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3772,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3772,"record_offset":2144,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3772,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3772,"record_offset":1772,"parent_record_offset":1740,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3772,"record_offset":756,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3772,"record_offset":960,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3772,"record_offset":2356,"parent_record_offset":2324,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03772.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3772,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03772.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_roof::wall_height_dt::wall_height_dt
// 信息 ID: cc67639153e5bb182e8825eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5588b2b104c6a7f0f14c923c: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)> > @ 0x1f2b829 -> 0x2820e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)> > @ 0x1f2ea21 -> 0x2820e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)> @ 0x1f3d1ba -> 0x2820e80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
