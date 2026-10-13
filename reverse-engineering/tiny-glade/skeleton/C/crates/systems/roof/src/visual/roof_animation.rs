// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: roof_animation.rs.assembly.json.gz

// FUNCTION system_roof::visual::roof_animation::roof_animation
// Candidate interfaces: 1e44a27472b886e31da12704
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3773:268 -> rva_28210c0_len_45f
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_28210c0_len_45f: RVA 0x28210c0, 1119 PDB bytes; 250 instructions
// Evidence: _assembly/roof-v1/C/pack-002/rva_28210c0_len_45f.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3773:268: system_roof::visual::roof_animation::roof_animation
// Frames: F1
// 0x1428210c0  4157                           pushq	%r15
// 0x1428210c2  4156                           pushq	%r14
// 0x1428210c4  4155                           pushq	%r13
// 0x1428210c6  4154                           pushq	%r12
// 0x1428210c8  56                             pushq	%rsi
// 0x1428210c9  57                             pushq	%rdi
// 0x1428210ca  55                             pushq	%rbp
// 0x1428210cb  53                             pushq	%rbx
// 0x1428210cc  4881ecf8000000                 subq	$0xf8, %rsp
// 0x1428210d3  440f29ac24e0000000             movaps	%xmm13, 0xe0(%rsp)
// 0x1428210dc  440f29a424d0000000             movaps	%xmm12, 0xd0(%rsp)
// 0x1428210e5  440f299c24c0000000             movaps	%xmm11, 0xc0(%rsp)
// 0x1428210ee  440f299424b0000000             movaps	%xmm10, 0xb0(%rsp)
// 0x1428210f7  440f298c24a0000000             movaps	%xmm9, 0xa0(%rsp)
// 0x142821100  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x142821109  0f29bc2480000000               movaps	%xmm7, 0x80(%rsp)
// 0x142821111  0f29742470                     movaps	%xmm6, 0x70(%rsp)
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7
// 0x142821116  4c8b01                         movq	(%rcx), %r8
// 0x142821119  488b4108                       movq	0x8(%rcx), %rax
// 0x14282111d  4889442440                     movq	%rax, 0x40(%rsp)
// 0x142821122  8b4114                         movl	0x14(%rcx), %eax
// 0x142821125  8944243c                       movl	%eax, 0x3c(%rsp)
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x142821129  4d8bb0f8000000                 movq	0xf8(%r8), %r14
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x142821130  498b8000010000                 movq	0x100(%r8), %rax
// Frames: F17 -> F18 -> F19 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x142821137  4d8d3c86                       leaq	(%r14,%rax,4), %r15
// 0x14282113b  4c89442450                     movq	%r8, 0x50(%rsp)
// Frames: F20 -> F15 -> F16 -> F6 -> F7
// 0x142821140  410fb68008010000               movzbl	0x108(%r8), %eax
// 0x142821148  8844242f                       movb	%al, 0x2f(%rsp)
// 0x14282114c  488b02                         movq	(%rdx), %rax
// 0x14282114f  4889442468                     movq	%rax, 0x68(%rsp)
// 0x142821154  4531ed                         xorl	%r13d, %r13d
// 0x142821157  b808000000                     movl	$0x8, %eax
// 0x14282115c  4889442460                     movq	%rax, 0x60(%rsp)
// 0x142821161  f30f1035173e6000               movss	0x603e17(%rip), %xmm6   # 0x142e24f80
// 0x142821169  f30f103d1fd47600               movss	0x76d41f(%rip), %xmm7   # 0x142f8e590
// 0x142821171  450f57c0                       xorps	%xmm8, %xmm8
// 0x142821175  f3440f100da6386e00             movss	0x6e38a6(%rip), %xmm9   # 0x142f04a24
// 0x14282117e  f3440f10154d187000             movss	0x70184d(%rip), %xmm10  # 0x142f229d4
// 0x142821187  f3440f101d0471d600             movss	0xd67104(%rip), %xmm11  # 0x143588294
// 0x142821190  f3440f1025ff70d600             movss	0xd670ff(%rip), %xmm12  # 0x143588298
// 0x142821199  f3440f102d2e907e00             movss	0x7e902e(%rip), %xmm13  # 0x14300a1d0
// 0x1428211a2  31ed                           xorl	%ebp, %ebp
// 0x1428211a4  31ff                           xorl	%edi, %edi
// 0x1428211a6  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x1428211af  b808000000                     movl	$0x8, %eax
// 0x1428211b4  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1428211b9  31db                           xorl	%ebx, %ebx
// Frames: F21 -> F22 -> F7
// 0x1428211bb  807c242f00                     cmpb	$0x0, 0x2f(%rsp)
// 0x1428211c0  743e                           je	0x142821200 <.text+0x2820200>
// Frames: F23 -> F22 -> F7
// 0x1428211c2  4439eb                         cmpl	%r13d, %ebx
// 0x1428211c5  0f858d010000                   jne	0x142821358 <.text+0x2820358>
// Frames: F24 -> F25 -> F26 -> F22 -> F7
// 0x1428211cb  4d39fe                         cmpq	%r15, %r14
// Frames: F25 -> F26 -> F22 -> F7
// 0x1428211ce  0f84f4020000                   je	0x1428214c8 <.text+0x28204c8>
// 0x1428211d4  488b442440                     movq	0x40(%rsp), %rax
// 0x1428211d9  488b80a0010000                 movq	0x1a0(%rax), %rax
// Frames: F26 -> F22 -> F7
// 0x1428211e0  418b0e                         movl	(%r14), %ecx
// Frames: F27 -> F28 -> F29 -> F30 -> F22 -> F7
// 0x1428211e3  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F31 -> F32 -> F33 -> F34 -> F22 -> F7
// 0x1428211e7  4c8b6cc810                     movq	0x10(%rax,%rcx,8), %r13
// Frames: F34 -> F22 -> F7
// 0x1428211ec  4d85ed                         testq	%r13, %r13
// 0x1428211ef  7566                           jne	0x142821257 <.text+0x2820257>
// Frames: F24 -> F25 -> F26 -> F22 -> F7
// 0x1428211f1  4983c604                       addq	$0x4, %r14
// 0x1428211f5  4d39fe                         cmpq	%r15, %r14
// Frames: F25 -> F26 -> F22 -> F7
// 0x1428211f8  75e6                           jne	0x1428211e0 <.text+0x28201e0>
// 0x1428211fa  e9c9020000                     jmp	0x1428214c8 <.text+0x28204c8>
// 0x1428211ff  90                             nop
// Frames: F35 -> F22 -> F7
// 0x142821200  4439eb                         cmpl	%r13d, %ebx
// 0x142821203  754b                           jne	0x142821250 <.text+0x2820250>
// Frames: F24 -> F25 -> F36 -> F22 -> F7
// 0x142821205  4d39fe                         cmpq	%r15, %r14
// Frames: F25 -> F36 -> F22 -> F7
// 0x142821208  0f84ba020000                   je	0x1428214c8 <.text+0x28204c8>
// 0x14282120e  488b442440                     movq	0x40(%rsp), %rax
// 0x142821213  488b8000010000                 movq	0x100(%rax), %rax
// 0x14282121a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F36 -> F22 -> F7
// 0x142821220  418b0e                         movl	(%r14), %ecx
// Frames: F27 -> F28 -> F37 -> F38 -> F22 -> F7
// 0x142821223  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x142821227  48c1e105                       shlq	$0x5, %rcx
// Frames: F31 -> F32 -> F39 -> F40 -> F22 -> F7
// 0x14282122b  4c8b6c0858                     movq	0x58(%rax,%rcx), %r13
// Frames: F40 -> F22 -> F7
// 0x142821230  4d85ed                         testq	%r13, %r13
// 0x142821233  7567                           jne	0x14282129c <.text+0x282029c>
// Frames: F24 -> F25 -> F36 -> F22 -> F7
// 0x142821235  4983c604                       addq	$0x4, %r14
// 0x142821239  4d39fe                         cmpq	%r15, %r14
// Frames: F25 -> F36 -> F22 -> F7
// 0x14282123c  75e2                           jne	0x142821220 <.text+0x2820220>
// 0x14282123e  e985020000                     jmp	0x1428214c8 <.text+0x28204c8>
// 0x142821243  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F41 -> F22 -> F7
// 0x142821250  89d9                           movl	%ebx, %ecx
// 0x142821252  e976010000                     jmp	0x1428213cd <.text+0x28203cd>
// Frames: F42 -> F43 -> F22 -> F7
// 0x142821257  488d04c8                       leaq	(%rax,%rcx,8), %rax
// 0x14282125b  4c8b542450                     movq	0x50(%rsp), %r10
// 0x142821260  498b9210010000                 movq	0x110(%r10), %rdx
// 0x142821267  488b4838                       movq	0x38(%rax), %rcx
// Frames: F44 -> F45 -> F46 -> F47 -> F42 -> F43 -> F22 -> F7
// 0x14282126b  4c8b4040                       movq	0x40(%rax), %r8
// Frames: F27 -> F28 -> F48 -> F44 -> F45 -> F46 -> F47 -> F42 -> F43 -> F22 -> F7
// 0x14282126f  4c39c2                         cmpq	%r8, %rdx
// 0x142821272  0f8385000000                   jae	0x1428212fd <.text+0x28202fd>
// Frames: F49 -> F50 -> F51 -> F48 -> F44 -> F45 -> F46 -> F47 -> F42 -> F43 -> F22 -> F7
// 0x142821278  4c8b0cd1                       movq	(%rcx,%rdx,8), %r9
// Frames: F44 -> F45 -> F52 -> F42 -> F43 -> F22 -> F7
// 0x14282127c  488b5018                       movq	0x18(%rax), %rdx
// Frames: F49 -> F50 -> F51 -> F48 -> F44 -> F45 -> F46 -> F47 -> F42 -> F43 -> F22 -> F7
// 0x142821280  4d85c9                         testq	%r9, %r9
// 0x142821283  747c                           je	0x142821301 <.text+0x2820301>
// Frames: F53 -> F54 -> F55 -> F44 -> F45 -> F46 -> F47 -> F42 -> F43 -> F22 -> F7
// 0x142821285  49f7d1                         notq	%r9
// Frames: F56 -> F57 -> F54 -> F55 -> F44 -> F45 -> F46 -> F47 -> F42 -> F43 -> F22 -> F7
// 0x142821288  4f8d0c49                       leaq	(%r9,%r9,2), %r9
// 0x14282128c  49c1e104                       shlq	$0x4, %r9
// Frames: F58 -> F59 -> F60 -> F55 -> F61 -> F47 -> F42 -> F43 -> F22 -> F7
// 0x142821290  4e8b4c0a10                     movq	0x10(%rdx,%r9), %r9
// 0x142821295  4c894c2430                     movq	%r9, 0x30(%rsp)
// 0x14282129a  eb6e                           jmp	0x14282130a <.text+0x282030a>
// Frames: F62 -> F63 -> F22 -> F7
// 0x14282129c  4801c8                         addq	%rcx, %rax
// 0x14282129f  8b8894000000                   movl	0x94(%rax), %ecx
// 0x1428212a5  488b542440                     movq	0x40(%rsp), %rdx
// Frames: F27 -> F28 -> F64 -> F63 -> F22 -> F7
// 0x1428212aa  4c8b8aa0010000                 movq	0x1a0(%rdx), %r9
// 0x1428212b1  4c8d14c9                       leaq	(%rcx,%rcx,8), %r10
// 0x1428212b5  4b8d14d1                       leaq	(%r9,%r10,8), %rdx
// 0x1428212b9  4c8b5c2450                     movq	0x50(%rsp), %r11
// Frames: F65 -> F66 -> F22 -> F7
// 0x1428212be  4d8b8310010000                 movq	0x110(%r11), %r8
// 0x1428212c5  4b8b4cd138                     movq	0x38(%r9,%r10,8), %rcx
// Frames: F27 -> F28 -> F48 -> F44 -> F45 -> F46 -> F47 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1428212ca  4f3b44d140                     cmpq	0x40(%r9,%r10,8), %r8
// 0x1428212cf  0f8390000000                   jae	0x142821365 <.text+0x2820365>
// Frames: F49 -> F50 -> F51 -> F48 -> F44 -> F45 -> F46 -> F47 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1428212d5  4e8b04c1                       movq	(%rcx,%r8,8), %r8
// Frames: F44 -> F45 -> F52 -> F68 -> F65 -> F66 -> F22 -> F7
// 0x1428212d9  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F49 -> F50 -> F51 -> F48 -> F44 -> F45 -> F46 -> F47 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1428212dd  4d85c0                         testq	%r8, %r8
// 0x1428212e0  0f8483000000                   je	0x142821369 <.text+0x2820369>
// Frames: F53 -> F54 -> F55 -> F44 -> F45 -> F46 -> F47 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1428212e6  49f7d0                         notq	%r8
// Frames: F56 -> F57 -> F54 -> F55 -> F44 -> F45 -> F46 -> F47 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1428212e9  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x1428212ed  49c1e004                       shlq	$0x4, %r8
// Frames: F58 -> F59 -> F60 -> F55 -> F61 -> F47 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1428212f1  4e8b440210                     movq	0x10(%rdx,%r8), %r8
// 0x1428212f6  4c89442430                     movq	%r8, 0x30(%rsp)
// 0x1428212fb  eb75                           jmp	0x142821372 <.text+0x2820372>
// Frames: F44 -> F45 -> F52 -> F42 -> F43 -> F22 -> F7
// 0x1428212fd  488b5018                       movq	0x18(%rax), %rdx
// 0x142821301  48c744243000000000             movq	$0x0, 0x30(%rsp)
// Frames: F69 -> F70 -> F42 -> F43 -> F22 -> F7
// 0x14282130a  4d8b8a20010000                 movq	0x120(%r10), %r9
// 0x142821311  31db                           xorl	%ebx, %ebx
// 0x142821313  bd00000000                     movl	$0x0, %ebp
// Frames: F27 -> F28 -> F71 -> F72 -> F73 -> F74 -> F75 -> F70 -> F42 -> F43 -> F22 -> F7
// 0x142821318  4d39c1                         cmpq	%r8, %r9
// 0x14282131b  7309                           jae	0x142821326 <.text+0x2820326>
// Frames: F76 -> F50 -> F77 -> F71 -> F72 -> F73 -> F74 -> F75 -> F70 -> F42 -> F43 -> F22 -> F7
// 0x14282131d  4a833cc900                     cmpq	$0x0, (%rcx,%r9,8)
// 0x142821322  400f95c5                       setne	%bpl
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F78 -> F79 -> F22 -> F7
// 0x142821326  4983c604                       addq	$0x4, %r14
// 0x14282132a  4d8b8218010000                 movq	0x118(%r10), %r8
// 0x142821331  4a8b0cc1                       movq	(%rcx,%r8,8), %rcx
// 0x142821335  48f7d1                         notq	%rcx
// 0x142821338  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x14282133c  48c1e104                       shlq	$0x4, %rcx
// 0x142821340  488b7c0a10                     movq	0x10(%rdx,%rcx), %rdi
// 0x142821345  488b4c0a28                     movq	0x28(%rdx,%rcx), %rcx
// 0x14282134a  48894c2448                     movq	%rcx, 0x48(%rsp)
// 0x14282134f  488b4008                       movq	0x8(%rax), %rax
// 0x142821353  4889442460                     movq	%rax, 0x60(%rsp)
// Frames: F80 -> F22 -> F7
// 0x142821358  89da                           movl	%ebx, %edx
// 0x14282135a  488b442460                     movq	0x60(%rsp), %rax
// Frames: F56 -> F81 -> F80 -> F22 -> F7
// 0x14282135f  488d04d0                       leaq	(%rax,%rdx,8), %rax
// 0x142821363  eb7b                           jmp	0x1428213e0 <.text+0x28203e0>
// Frames: F44 -> F45 -> F52 -> F68 -> F65 -> F66 -> F22 -> F7
// 0x142821365  488b5218                       movq	0x18(%rdx), %rdx
// 0x142821369  48c744243000000000             movq	$0x0, 0x30(%rsp)
// Frames: F69 -> F82 -> F65 -> F66 -> F22 -> F7
// 0x142821372  4d8b8320010000                 movq	0x120(%r11), %r8
// 0x142821379  31db                           xorl	%ebx, %ebx
// 0x14282137b  bd00000000                     movl	$0x0, %ebp
// Frames: F27 -> F28 -> F71 -> F72 -> F83 -> F84 -> F75 -> F82 -> F65 -> F66 -> F22 -> F7
// 0x142821380  4c3b8088000000                 cmpq	0x88(%rax), %r8
// 0x142821387  7310                           jae	0x142821399 <.text+0x2820399>
// Frames: F72 -> F83 -> F84 -> F75 -> F82 -> F65 -> F66 -> F22 -> F7
// 0x142821389  4c8b8880000000                 movq	0x80(%rax), %r9
// Frames: F76 -> F50 -> F77 -> F71 -> F72 -> F83 -> F84 -> F75 -> F82 -> F65 -> F66 -> F22 -> F7
// 0x142821390  4b833cc100                     cmpq	$0x0, (%r9,%r8,8)
// 0x142821395  400f95c5                       setne	%bpl
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F85 -> F86 -> F22 -> F7
// 0x142821399  4983c604                       addq	$0x4, %r14
// 0x14282139d  4d8b8318010000                 movq	0x118(%r11), %r8
// 0x1428213a4  4a8b0cc1                       movq	(%rcx,%r8,8), %rcx
// 0x1428213a8  48f7d1                         notq	%rcx
// 0x1428213ab  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1428213af  48c1e104                       shlq	$0x4, %rcx
// 0x1428213b3  488b7c0a10                     movq	0x10(%rdx,%rcx), %rdi
// 0x1428213b8  488b4c0a28                     movq	0x28(%rdx,%rcx), %rcx
// 0x1428213bd  48894c2448                     movq	%rcx, 0x48(%rsp)
// 0x1428213c2  488b4050                       movq	0x50(%rax), %rax
// 0x1428213c6  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1428213cb  31c9                           xorl	%ecx, %ecx
// Frames: F56 -> F81 -> F41 -> F22 -> F7
// 0x1428213cd  48c1e104                       shlq	$0x4, %rcx
// 0x1428213d1  488b542458                     movq	0x58(%rsp), %rdx
// 0x1428213d6  488d040a                       leaq	(%rdx,%rcx), %rax
// Frames: F87 -> F88 -> F22 -> F7
// 0x1428213da  8b540a08                       movl	0x8(%rdx,%rcx), %edx
// Frames: F89 -> F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F22 -> F7
// 0x1428213de  f7d2                           notl	%edx
// Frames: F17 -> F96 -> F91 -> F92 -> F93 -> F94 -> F97 -> F22 -> F7
// 0x1428213e0  486bca58                       imulq	$0x58, %rdx, %rcx
// 0x1428213e4  48034c2430                     addq	0x30(%rsp), %rcx
// Frames: F17 -> F96 -> F98 -> F92 -> F99 -> F94 -> F97 -> F22 -> F7
// 0x1428213e9  4c8d24d7                       leaq	(%rdi,%rdx,8), %r12
// 0x1428213ed  4c8b442448                     movq	0x48(%rsp), %r8
// Frames: F17 -> F96 -> F100 -> F92 -> F99 -> F94 -> F97 -> F22 -> F7
// 0x1428213f2  498d3490                       leaq	(%r8,%rdx,4), %rsi
// Frames: F7
// 0x1428213f6  833800                         cmpl	$0x0, (%rax)
// 0x1428213f9  0f84c9000000                   je	0x1428214c8 <.text+0x28204c8>
// Frames: F101
// 0x1428213ff  e88c7a3cfe                     callq	0x140be8e90 <.text+0xbe7e90>
// Frames: F102
// 0x142821404  40f6c501                       testb	$0x1, %bpl
// 0x142821408  7426                           je	0x142821430 <.text+0x2820430>
// Frames: F103
// 0x14282140a  488b442468                     movq	0x68(%rsp), %rax
// 0x14282140f  f30f104028                     movss	0x28(%rax), %xmm0
// 0x142821414  0f28c8                         movaps	%xmm0, %xmm1
// 0x142821417  f30f59ce                       mulss	%xmm6, %xmm1
// Frames: F104
// 0x14282141b  f30f59c7                       mulss	%xmm7, %xmm0
// Frames: F105
// 0x14282141f  410f2ec8                       ucomiss	%xmm8, %xmm1
// 0x142821423  754c                           jne	0x142821471 <.text+0x2820471>
// 0x142821425  7a4a                           jp	0x142821471 <.text+0x2820471>
// 0x142821427  eb69                           jmp	0x142821492 <.text+0x2820492>
// 0x142821429  0f1f8000000000                 nopl	(%rax)
// Frames: F101
// 0x142821430  410f28d2                       movaps	%xmm10, %xmm2
// 0x142821434  84c0                           testb	%al, %al
// 0x142821436  7504                           jne	0x14282143c <.text+0x282043c>
// 0x142821438  410f28d1                       movaps	%xmm9, %xmm2
// Frames: F106
// 0x14282143c  488b442468                     movq	0x68(%rsp), %rax
// 0x142821441  f30f105828                     movss	0x28(%rax), %xmm3
// 0x142821446  0f28cb                         movaps	%xmm3, %xmm1
// 0x142821449  f3410f59cb                     mulss	%xmm11, %xmm1
// Frames: F107
// 0x14282144e  f3410f59dc                     mulss	%xmm12, %xmm3
// 0x142821453  f30f59da                       mulss	%xmm2, %xmm3
// 0x142821457  410f28c5                       movaps	%xmm13, %xmm0
// 0x14282145b  f3410fc2042401                 cmpltss	(%r12), %xmm0
// 0x142821462  0f54c3                         andps	%xmm3, %xmm0
// Frames: F108
// 0x142821465  f30f59ca                       mulss	%xmm2, %xmm1
// Frames: F105
// 0x142821469  410f2ec8                       ucomiss	%xmm8, %xmm1
// 0x14282146d  7502                           jne	0x142821471 <.text+0x2820471>
// 0x14282146f  7b21                           jnp	0x142821492 <.text+0x2820492>
// Frames: F109 -> F110 -> F111
// 0x142821471  8b44243c                       movl	0x3c(%rsp), %eax
// 0x142821475  8906                           movl	%eax, (%rsi)
// Frames: F111
// 0x142821477  f3410f580c24                   addss	(%r12), %xmm1
// Frames: F112 -> F113
// 0x14282147d  0f57d2                         xorps	%xmm2, %xmm2
// 0x142821480  f30f5fd1                       maxss	%xmm1, %xmm2
// Frames: F114 -> F113
// 0x142821484  410f28c9                       movaps	%xmm9, %xmm1
// 0x142821488  f30f5dca                       minss	%xmm2, %xmm1
// Frames: F113
// 0x14282148c  f3410f110c24                   movss	%xmm1, (%r12)
// Frames: F115
// 0x142821492  ffc3                           incl	%ebx
// 0x142821494  410f2ec0                       ucomiss	%xmm8, %xmm0
// 0x142821498  7506                           jne	0x1428214a0 <.text+0x28204a0>
// 0x14282149a  0f8b1bfdffff                   jnp	0x1428211bb <.text+0x28201bb>
// Frames: F109 -> F110 -> F116
// 0x1428214a0  8b44243c                       movl	0x3c(%rsp), %eax
// 0x1428214a4  8906                           movl	%eax, (%rsi)
// Frames: F116
// 0x1428214a6  f3410f58442404                 addss	0x4(%r12), %xmm0
// Frames: F112 -> F117
// 0x1428214ad  0f57c9                         xorps	%xmm1, %xmm1
// 0x1428214b0  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F114 -> F117
// 0x1428214b4  410f28c1                       movaps	%xmm9, %xmm0
// 0x1428214b8  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F117
// 0x1428214bc  f3410f11442404                 movss	%xmm0, 0x4(%r12)
// Frames: F115
// 0x1428214c3  e9f3fcffff                     jmp	0x1428211bb <.text+0x28201bb>
// Frames: F118
// 0x1428214c8  0f28742470                     movaps	0x70(%rsp), %xmm6
// 0x1428214cd  0f28bc2480000000               movaps	0x80(%rsp), %xmm7
// 0x1428214d5  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x1428214de  440f288c24a0000000             movaps	0xa0(%rsp), %xmm9
// 0x1428214e7  440f289424b0000000             movaps	0xb0(%rsp), %xmm10
// 0x1428214f0  440f289c24c0000000             movaps	0xc0(%rsp), %xmm11
// 0x1428214f9  440f28a424d0000000             movaps	0xd0(%rsp), %xmm12
// 0x142821502  440f28ac24e0000000             movaps	0xe0(%rsp), %xmm13
// 0x14282150b  4881c4f8000000                 addq	$0xf8, %rsp
// 0x142821512  5b                             popq	%rbx
// 0x142821513  5d                             popq	%rbp
// 0x142821514  5f                             popq	%rdi
// 0x142821515  5e                             popq	%rsi
// 0x142821516  415c                           popq	%r12
// 0x142821518  415d                           popq	%r13
// 0x14282151a  415e                           popq	%r14
// 0x14282151c  415f                           popq	%r15
// 0x14282151e  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/roof_animation.rs:11:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:528:0 bevy_ecs::system::query::Query::new
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:643:0 bevy_ecs::system::query::Query::copy_unsafe
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:625:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:607:0 bevy_ecs::system::query::Query::reborrow
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:705:0 bevy_ecs::system::query::Query::iter_mut
// F7: crates/systems/roof/src/visual/roof_animation.rs:20:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
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
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2422:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2517:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:915:0 bevy_ecs::query::iter::impl$1::next
// F23: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2520:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F24: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:180:0 core::slice::iter::impl$171::next
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2521:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F28: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F30: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2522:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2997:0 alloc::vec::Vec::len
// F32: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3022:0 alloc::vec::Vec::is_empty
// F33: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:647:0 bevy_ecs::storage::table::Table::is_empty
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2523:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2561:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2562:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:865:0 bevy_ecs::archetype::Archetypes::get
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2563:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:656:0 bevy_ecs::archetype::Archetype::is_empty
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2564:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2593:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$4::set_table
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2529:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:381:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1552:0 bevy_ecs::query::fetch::impl$36::set_table
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F50: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F51: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::slice::impl$0::get
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1932:0 bevy_ecs::query::fetch::impl$48::set_table
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F55: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F57: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:298:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:479:0 bevy_ecs::archetype::Archetype::table_id
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2567:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::archetype::Archetype::table_id
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$4::set_archetype
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2571:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1539:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1921:0 bevy_ecs::query::fetch::impl$48::set_archetype
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1578:0 bevy_ecs::query::fetch::impl$36::matches_component_set
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2224:0 bevy_ecs::query::fetch::impl$55::set_table
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:49:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:399:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:511:0 bevy_ecs::storage::table::Table::has_column
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2224:0 bevy_ecs::query::fetch::impl$55::set_table::closure$0
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1581:0 bevy_ecs::query::fetch::impl$36::matches_component_set
// F76: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:636:0 core::option::Option::is_some
// F77: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:664:0 core::slice::impl$0::get
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:209:0 bevy_ecs::storage::table::Table::entities
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2532:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2540:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F81: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 core::slice::impl$0::get_unchecked
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2209:0 bevy_ecs::query::fetch::impl$55::set_archetype
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:662:0 bevy_ecs::archetype::Archetype::contains
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2209:0 bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:485:0 bevy_ecs::archetype::Archetype::entities
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2584:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:347:0 bevy_ecs::archetype::ArchetypeEntity::table_row
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2615:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1609:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1604:0 bevy_ecs::query::fetch::impl$37::fetch
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2529:0 bevy_ecs::query::fetch::impl$95::fetch
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2611:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ptr::ThinSlicePtr::get
// F97: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:0:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2000:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F99: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1993:0 bevy_ecs::query::fetch::impl$37::fetch
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2004:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F101: crates/systems/roof/src/visual/roof_animation.rs:21:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F102: crates/systems/roof/src/visual/roof_animation.rs:23:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F103: crates/systems/roof/src/visual/roof_animation.rs:25:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F104: crates/systems/roof/src/visual/roof_animation.rs:26:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F105: crates/systems/roof/src/visual/roof_animation.rs:40:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F106: crates/systems/roof/src/visual/roof_animation.rs:30:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F107: crates/systems/roof/src/visual/roof_animation.rs:31:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F108: crates/systems/roof/src/visual/roof_animation.rs:37:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F111: crates/systems/roof/src/visual/roof_animation.rs:41:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F112: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1512:0 core::f32::impl$0::clamp
// F113: crates/systems/roof/src/visual/roof_animation.rs:42:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F114: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1515:0 core::f32::impl$0::clamp
// F115: crates/systems/roof/src/visual/roof_animation.rs:45:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F116: crates/systems/roof/src/visual/roof_animation.rs:46:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F117: crates/systems/roof/src/visual/roof_animation.rs:47:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// F118: crates/systems/roof/src/visual/roof_animation.rs:50:0 system_roof::visual::roof_animation::roof_animation::hf04904153e7689a8
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/visual/roof_animation.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 29b5ee6b87c6eb106b3a0c5c13831027a5be153d6ad7343ac17efb742b3dc5e5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3773
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::roof_animation::roof_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3773,"record_offset":268,"rva":42078400,"va":5410787520}
// 编译实例 RVAs: 0x28210c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29590900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410787520,"raw_hex":"e88c7a3cfe","size":5,"soft_fail":false,"target_va":5381197456,"va":5410788351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3773,"record_offset":4556,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3773,"record_offset":852,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3773,"record_offset":416,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3773,"record_offset":4612,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3773,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3773,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3773,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3773,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3773,"record_offset":636,"parent_record_offset":604,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3773,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":268,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3773,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3773,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3773,"record_offset":3276,"parent_record_offset":3252,"ancestor_record_offset":268,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3773,"record_offset":3756,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3773,"record_offset":1360,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3773,"record_offset":2392,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3773,"record_offset":4008,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3773,"record_offset":1276,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3773,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":268,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":268,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":2512,"parent_record_offset":2484,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":3072,"parent_record_offset":2484,"ancestor_record_offset":268,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":2000,"parent_record_offset":1444,"ancestor_record_offset":268,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":268,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":3200,"parent_record_offset":2484,"ancestor_record_offset":268,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":3252,"parent_record_offset":3224,"ancestor_record_offset":268,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":2096,"parent_record_offset":1444,"ancestor_record_offset":268,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":2148,"parent_record_offset":2120,"ancestor_record_offset":268,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$95::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3773,"record_offset":4036,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":18908,"source_line":2520,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3773,"record_offset":552,"parent_record_offset":440,"ancestor_record_offset":268,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3773,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3773,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":268,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3773,"record_offset":2484,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":18906,"source_line":182,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3773,"record_offset":1444,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":18905,"source_line":195,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":3773,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3773,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":268,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3773,"record_offset":1588,"parent_record_offset":1560,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3773,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":268,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3773,"record_offset":1560,"parent_record_offset":1532,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3773,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3773,"record_offset":3504,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3773,"record_offset":1532,"parent_record_offset":1500,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3773,"record_offset":1500,"parent_record_offset":1472,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3773,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":268,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3773,"record_offset":1128,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3773,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3773,"record_offset":1044,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3773,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3773,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3773,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":268,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3773,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":268,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3773,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":268,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3773,"record_offset":468,"parent_record_offset":440,"ancestor_record_offset":268,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3773,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":268,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3773,"record_offset":1680,"parent_record_offset":1616,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3773,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3773,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":268,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3773,"record_offset":1752,"parent_record_offset":1560,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3773,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3773,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3773,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3773,"record_offset":3700,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3773,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3773,"record_offset":1832,"parent_record_offset":1800,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3773,"record_offset":776,"parent_record_offset":604,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3773,"record_offset":980,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3773,"record_offset":4156,"parent_record_offset":4132,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03773.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3773,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03773.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_roof::visual::roof_animation::roof_animation
// 信息 ID: d2c1e4f84804fe5ce12c7dda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e44a27472b886e31da12704: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x1f271d1 -> 0x28210c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x1f3d630 -> 0x28210c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
