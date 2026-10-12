// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: edit_roof.rs.assembly.json.gz

// FUNCTION system_roof::edit_roof::edit_roof
// Candidate interfaces: 1dc3c96ef7aaa87058952b28
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3197:496 -> rva_1988f90_len_b3c
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1988f90_len_b3c: RVA 0x1988f90, 2876 PDB bytes; 644 instructions
// Evidence: _assembly/roof-v1/A/pack-002/rva_1988f90_len_b3c.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3197:496: system_roof::edit_roof::edit_roof
// Frames: F1
// 0x141988f90  4157                           pushq	%r15
// 0x141988f92  4156                           pushq	%r14
// 0x141988f94  4155                           pushq	%r13
// 0x141988f96  4154                           pushq	%r12
// 0x141988f98  56                             pushq	%rsi
// 0x141988f99  57                             pushq	%rdi
// 0x141988f9a  55                             pushq	%rbp
// 0x141988f9b  53                             pushq	%rbx
// 0x141988f9c  4881ecb8010000                 subq	$0x1b8, %rsp            # imm = 0x1B8
// 0x141988fa3  440f29bc24a0010000             movaps	%xmm15, 0x1a0(%rsp)
// 0x141988fac  440f29b42490010000             movaps	%xmm14, 0x190(%rsp)
// 0x141988fb5  440f29ac2480010000             movaps	%xmm13, 0x180(%rsp)
// 0x141988fbe  440f29a42470010000             movaps	%xmm12, 0x170(%rsp)
// 0x141988fc7  440f299c2460010000             movaps	%xmm11, 0x160(%rsp)
// 0x141988fd0  440f29942450010000             movaps	%xmm10, 0x150(%rsp)
// 0x141988fd9  440f298c2440010000             movaps	%xmm9, 0x140(%rsp)
// 0x141988fe2  440f29842430010000             movaps	%xmm8, 0x130(%rsp)
// 0x141988feb  0f29bc2420010000               movaps	%xmm7, 0x120(%rsp)
// 0x141988ff3  0f29b42410010000               movaps	%xmm6, 0x110(%rsp)
// 0x141988ffb  4c8b942430020000               movq	0x230(%rsp), %r10
// Frames: F2 -> F3 -> F4
// 0x141989003  4d8b38                         movq	(%r8), %r15
// Frames: F5 -> F3 -> F4
// 0x141989006  4d8b5808                       movq	0x8(%r8), %r11
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x14198900a  498b1f                         movq	(%r15), %rbx
// 0x14198900d  31f6                           xorl	%esi, %esi
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x14198900f  4889d8                         movq	%rbx, %rax
// 0x141989012  492b4318                       subq	0x18(%r11), %rax
// 0x141989016  480f42c6                       cmovbq	%rsi, %rax
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x14198901a  492b5b38                       subq	0x38(%r11), %rbx
// 0x14198901e  4c8b842428020000               movq	0x228(%rsp), %r8
// 0x141989026  480f42de                       cmovbq	%rsi, %rbx
// Frames: F12 -> F13 -> F14 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x14198902a  4889c7                         movq	%rax, %rdi
// 0x14198902d  48c1e706                       shlq	$0x6, %rdi
// 0x141989031  49037b08                       addq	0x8(%r11), %rdi
// Frames: F15 -> F7 -> F8 -> F3 -> F4
// 0x141989035  4d8b6310                       movq	0x10(%r11), %r12
// Frames: F16 -> F7 -> F8 -> F3 -> F4
// 0x141989039  4d8b6b30                       movq	0x30(%r11), %r13
// Frames: F17 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x14198903d  4929c4                         subq	%rax, %r12
// 0x141989040  4c0f42e6                       cmovbq	%rsi, %r12
// 0x141989044  4c8d3565f29400                 leaq	0x94f265(%rip), %r14    # 0x1422d82b0
// 0x14198904b  490f42fe                       cmovbq	%r14, %rdi
// Frames: F12 -> F13 -> F14 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x14198904f  4889d8                         movq	%rbx, %rax
// 0x141989052  48c1e006                       shlq	$0x6, %rax
// 0x141989056  49034328                       addq	0x28(%r11), %rax
// 0x14198905a  4c39eb                         cmpq	%r13, %rbx
// Frames: F17 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x14198905d  490f47c6                       cmovaq	%r14, %rax
// 0x141989061  4929dd                         subq	%rbx, %r13
// 0x141989064  4c0f42ee                       cmovbq	%rsi, %r13
// Frames: F18 -> F7 -> F8 -> F3 -> F4
// 0x141989068  4d8b5b40                       movq	0x40(%r11), %r11
// 0x14198906c  4d29eb                         subq	%r13, %r11
// 0x14198906f  4d29e3                         subq	%r12, %r11
// 0x141989072  4c89bc24f8000000               movq	%r15, 0xf8(%rsp)
// 0x14198907a  4d891f                         movq	%r11, (%r15)
// Frames: F19 -> F20 -> F21 -> F22 -> F7 -> F8 -> F3 -> F4
// 0x14198907d  49c1e406                       shlq	$0x6, %r12
// 0x141989081  4901fc                         addq	%rdi, %r12
// 0x141989084  4c89a424e0000000               movq	%r12, 0xe0(%rsp)
// 0x14198908c  49c1e506                       shlq	$0x6, %r13
// 0x141989090  4901c5                         addq	%rax, %r13
// 0x141989093  4c89ac24d8000000               movq	%r13, 0xd8(%rsp)
// 0x14198909b  488b09                         movq	(%rcx), %rcx
// 0x14198909e  48898c24f0000000               movq	%rcx, 0xf0(%rsp)
// 0x1419890a6  4c8b32                         movq	(%rdx), %r14
// 0x1419890a9  4c8b6208                       movq	0x8(%rdx), %r12
// 0x1419890ad  8b4a14                         movl	0x14(%rdx), %ecx
// 0x1419890b0  894c2428                       movl	%ecx, 0x28(%rsp)
// 0x1419890b4  418b481c                       movl	0x1c(%r8), %ecx
// 0x1419890b8  894c2468                       movl	%ecx, 0x68(%rsp)
// 0x1419890bc  498b10                         movq	(%r8), %rdx
// 0x1419890bf  498b4810                       movq	0x10(%r8), %rcx
// 0x1419890c3  48898c24d0000000               movq	%rcx, 0xd0(%rsp)
// 0x1419890cb  418b4a1c                       movl	0x1c(%r10), %ecx
// 0x1419890cf  894c242c                       movl	%ecx, 0x2c(%rsp)
// 0x1419890d3  498b2a                         movq	(%r10), %rbp
// 0x1419890d6  498b4a10                       movq	0x10(%r10), %rcx
// 0x1419890da  48894c2458                     movq	%rcx, 0x58(%rsp)
// 0x1419890df  488b8c2420020000               movq	0x220(%rsp), %rcx
// 0x1419890e7  448b411c                       movl	0x1c(%rcx), %r8d
// 0x1419890eb  4489442474                     movl	%r8d, 0x74(%rsp)
// 0x1419890f0  4c8b01                         movq	(%rcx), %r8
// 0x1419890f3  4c89842498000000               movq	%r8, 0x98(%rsp)
// 0x1419890fb  488b4910                       movq	0x10(%rcx), %rcx
// 0x1419890ff  48898c24e8000000               movq	%rcx, 0xe8(%rsp)
// 0x141989107  418b491c                       movl	0x1c(%r9), %ecx
// 0x14198910b  894c244c                       movl	%ecx, 0x4c(%rsp)
// 0x14198910f  498b09                         movq	(%r9), %rcx
// 0x141989112  48898c2490000000               movq	%rcx, 0x90(%rsp)
// 0x14198911a  4d8b4910                       movq	0x10(%r9), %r9
// 0x14198911e  4c898c2480000000               movq	%r9, 0x80(%rsp)
// 0x141989126  450f57db                       xorps	%xmm11, %xmm11
// 0x14198912a  f3440f1025d51c8300             movss	0x831cd5(%rip), %xmm12  # 0x1421bae08
// 0x141989133  f3440f103584f67b00             movss	0x7bf684(%rip), %xmm14  # 0x1421487c0
// 0x14198913c  4889942488000000               movq	%rdx, 0x88(%rsp)
// 0x141989144  488d5220                       leaq	0x20(%rdx), %rdx
// 0x141989148  48899424c0000000               movq	%rdx, 0xc0(%rsp)
// 0x141989150  488d5520                       leaq	0x20(%rbp), %rdx
// 0x141989154  4889542450                     movq	%rdx, 0x50(%rsp)
// 0x141989159  498d5020                       leaq	0x20(%r8), %rdx
// 0x14198915d  48899424c8000000               movq	%rdx, 0xc8(%rsp)
// 0x141989165  4883c120                       addq	$0x20, %rcx
// 0x141989169  48894c2478                     movq	%rcx, 0x78(%rsp)
// 0x14198916e  4c89b424a8000000               movq	%r14, 0xa8(%rsp)
// 0x141989176  4c89a424a0000000               movq	%r12, 0xa0(%rsp)
// 0x14198917e  eb30                           jmp	0x1419891b0 <.text+0x19881b0>
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F28
// 0x141989180  498b4f28                       movq	0x28(%r15), %rcx
// Frames: F29 -> F25 -> F26 -> F27 -> F28
// 0x141989184  48c1e205                       shlq	$0x5, %rdx
// 0x141989188  488d0411                       leaq	(%rcx,%rdx), %rax
// Frames: F30 -> F31 -> F26 -> F27 -> F28
// 0x14198918c  48893411                       movq	%rsi, (%rcx,%rdx)
// 0x141989190  31c9                           xorl	%ecx, %ecx
// Frames: F30 -> F31 -> F26 -> F27 -> F32
// 0x141989192  48895808                       movq	%rbx, 0x8(%rax)
// 0x141989196  4c894010                       movq	%r8, 0x10(%rax)
// 0x14198919a  884818                         movb	%cl, 0x18(%rax)
// Frames: F33 -> F26 -> F27 -> F32
// 0x14198919d  49ff4730                       incq	0x30(%r15)
// Frames: F34 -> F27 -> F32
// 0x1419891a1  49ff4740                       incq	0x40(%r15)
// 0x1419891a5  4c89e8                         movq	%r13, %rax
// 0x1419891a8  488bbc24b8000000               movq	0xb8(%rsp), %rdi
// Frames: F35 -> F36 -> F37 -> F38 -> F39 -> F4
// 0x1419891b0  4885ff                         testq	%rdi, %rdi
// 0x1419891b3  741b                           je	0x1419891d0 <.text+0x19881d0>
// 0x1419891b5  483bbc24e0000000               cmpq	0xe0(%rsp), %rdi
// 0x1419891bd  7411                           je	0x1419891d0 <.text+0x19881d0>
// 0x1419891bf  488d7740                       leaq	0x40(%rdi), %rsi
// 0x1419891c3  4989c5                         movq	%rax, %r13
// 0x1419891c6  eb28                           jmp	0x1419891f0 <.text+0x19881f0>
// 0x1419891c8  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F35 -> F40 -> F41 -> F37 -> F38 -> F39 -> F4
// 0x1419891d0  4885c0                         testq	%rax, %rax
// 0x1419891d3  0f8445080000                   je	0x141989a1e <.text+0x1988a1e>
// 0x1419891d9  483b8424d8000000               cmpq	0xd8(%rsp), %rax
// 0x1419891e1  0f8437080000                   je	0x141989a1e <.text+0x1988a1e>
// Frames: F42 -> F43 -> F44 -> F40 -> F41 -> F37 -> F38 -> F39 -> F4
// 0x1419891e7  4c8d6840                       leaq	0x40(%rax), %r13
// 0x1419891eb  31f6                           xorl	%esi, %esi
// 0x1419891ed  4889c7                         movq	%rax, %rdi
// Frames: F45 -> F39 -> F4
// 0x1419891f0  488b8424f8000000               movq	0xf8(%rsp), %rax
// 0x1419891f8  48ff00                         incq	(%rax)
// Frames: F46
// 0x1419891fb  488b5720                       movq	0x20(%rdi), %rdx
// 0x1419891ff  488b8c24f0000000               movq	0xf0(%rsp), %rcx
// 0x141989207  4889542460                     movq	%rdx, 0x60(%rsp)
// 0x14198920c  e8ef12cefe                     callq	0x14066a500 <.text+0x669500>
// Frames: F47 -> F46
// 0x141989211  4889c2                         movq	%rax, %rdx
// 0x141989214  48c1ea20                       shrq	$0x20, %rdx
// 0x141989218  0f84a1080000                   je	0x141989abf <.text+0x1988abf>
// Frames: F48 -> F49 -> F50 -> F51
// 0x14198921e  4889c3                         movq	%rax, %rbx
// 0x141989221  89d9                           movl	%ebx, %ecx
// 0x141989223  b801000000                     movl	$0x1, %eax
// Frames: F52 -> F14 -> F48 -> F49 -> F50 -> F51
// 0x141989228  493b4c2410                     cmpq	0x10(%r12), %rcx
// Frames: F48 -> F49 -> F50 -> F51
// 0x14198922d  0f8359080000                   jae	0x141989a8c <.text+0x1988a8c>
// Frames: F53 -> F54 -> F55 -> F49 -> F50 -> F51
// 0x141989233  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x141989237  48c1e102                       shlq	$0x2, %rcx
// 0x14198923b  49034c2408                     addq	0x8(%r12), %rcx
// 0x141989240  3911                           cmpl	%edx, (%rcx)
// Frames: F55 -> F49 -> F50 -> F51
// 0x141989242  0f8544080000                   jne	0x141989a8c <.text+0x1988a8c>
// Frames: F56 -> F57 -> F49 -> F50 -> F51
// 0x141989248  8b5104                         movl	0x4(%rcx), %edx
// 0x14198924b  41b8ffffffff                   movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
// 0x141989251  4c39c2                         cmpq	%r8, %rdx
// Frames: F57 -> F49 -> F50 -> F51
// 0x141989254  0f8432080000                   je	0x141989a8c <.text+0x1988a8c>
// Frames: F58 -> F59 -> F50 -> F51
// 0x14198925a  49395638                       cmpq	%rdx, 0x38(%r14)
// Frames: F60 -> F61 -> F59 -> F50 -> F51
// 0x14198925e  0f8626080000                   jbe	0x141989a8a <.text+0x1988a8a>
// Frames: F59 -> F50 -> F51
// 0x141989264  4889b424b8000000               movq	%rsi, 0xb8(%rsp)
// 0x14198926c  498b4628                       movq	0x28(%r14), %rax
// Frames: F19 -> F62 -> F63 -> F64 -> F60 -> F61 -> F59 -> F50 -> F51
// 0x141989270  4189d0                         movl	%edx, %r8d
// 0x141989273  41c1e806                       shrl	$0x6, %r8d
// Frames: F65 -> F63 -> F64 -> F60 -> F61 -> F59 -> F50 -> F51
// 0x141989277  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F63 -> F64 -> F60 -> F61 -> F59 -> F50 -> F51
// 0x14198927b  480fa3d0                       btq	%rdx, %rax
// Frames: F59 -> F50 -> F51
// 0x14198927f  0f8305080000                   jae	0x141989a8a <.text+0x1988a8a>
// Frames: F66 -> F49 -> F50 -> F51
// 0x141989285  488b4108                       movq	0x8(%rcx), %rax
// 0x141989289  8b7110                         movl	0x10(%rcx), %esi
// 0x14198928c  48c1e820                       shrq	$0x20, %rax
// Frames: F67 -> F68 -> F50 -> F51
// 0x141989290  498b8c2428010000               movq	0x128(%r12), %rcx
// Frames: F69 -> F50 -> F51
// 0x141989298  498b96d8000000                 movq	0xd8(%r14), %rdx
// Frames: F70 -> F71 -> F72 -> F69 -> F50 -> F51
// 0x14198929f  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F73 -> F70 -> F71 -> F72 -> F69 -> F50 -> F51
// 0x1419892a3  4c8b44c118                     movq	0x18(%rcx,%rax,8), %r8
// 0x1419892a8  488b44c138                     movq	0x38(%rcx,%rax,8), %rax
// Frames: F74 -> F75 -> F76 -> F77 -> F73 -> F70 -> F71 -> F72 -> F69 -> F50 -> F51
// 0x1419892ad  488b04d0                       movq	(%rax,%rdx,8), %rax
// Frames: F78 -> F79 -> F76 -> F73 -> F70 -> F71 -> F72 -> F69 -> F50 -> F51
// 0x1419892b1  48f7d0                         notq	%rax
// Frames: F80 -> F81 -> F82 -> F79 -> F76 -> F73 -> F70 -> F71 -> F72 -> F69 -> F50 -> F51
// 0x1419892b4  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1419892b8  48c1e005                       shlq	$0x5, %rax
// Frames: F83 -> F84 -> F72 -> F69 -> F50 -> F51
// 0x1419892bc  4d8b740050                     movq	0x50(%r8,%rax), %r14
// Frames: F19 -> F85 -> F86 -> F87 -> F50 -> F51
// 0x1419892c1  4c8d3cb6                       leaq	(%rsi,%rsi,4), %r15
// 0x1419892c5  49c1e704                       shlq	$0x4, %r15
// 0x1419892c9  4d037c0010                     addq	0x10(%r8,%rax), %r15
// Frames: F88
// 0x1419892ce  8b07                           movl	(%rdi), %eax
// 0x1419892d0  0f57f6                         xorps	%xmm6, %xmm6
// 0x1419892d3  0f57ff                         xorps	%xmm7, %xmm7
// 0x1419892d6  85c0                           testl	%eax, %eax
// 0x1419892d8  745e                           je	0x141989338 <.text+0x1988338>
// 0x1419892da  83f802                         cmpl	$0x2, %eax
// 0x1419892dd  7531                           jne	0x141989310 <.text+0x1988310>
// Frames: F89
// 0x1419892df  f30f107f04                     movss	0x4(%rdi), %xmm7
// 0x1419892e4  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F92
// 0x1419892e8  418904b6                       movl	%eax, (%r14,%rsi,4)
// 0x1419892ec  f3410f104724                   movss	0x24(%r15), %xmm0
// Frames: F92
// 0x1419892f2  f30f58c7                       addss	%xmm7, %xmm0
// Frames: F93 -> F94
// 0x1419892f6  0f57c9                         xorps	%xmm1, %xmm1
// 0x1419892f9  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F95 -> F94
// 0x1419892fd  410f28c6                       movaps	%xmm14, %xmm0
// 0x141989301  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F94
// 0x141989305  f3410f114724                   movss	%xmm0, 0x24(%r15)
// 0x14198930b  eb2b                           jmp	0x141989338 <.text+0x1988338>
// 0x14198930d  0f1f00                         nopl	(%rax)
// Frames: F96
// 0x141989310  f30f107f04                     movss	0x4(%rdi), %xmm7
// Frames: F93 -> F97
// 0x141989315  0f57c0                         xorps	%xmm0, %xmm0
// 0x141989318  f30f5fc7                       maxss	%xmm7, %xmm0
// Frames: F95 -> F97
// 0x14198931c  410f28ce                       movaps	%xmm14, %xmm1
// 0x141989320  f30f5dc8                       minss	%xmm0, %xmm1
// Frames: F98
// 0x141989324  f3410f5c7f24                   subss	0x24(%r15), %xmm7
// 0x14198932a  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F97
// 0x14198932e  418904b6                       movl	%eax, (%r14,%rsi,4)
// Frames: F97
// 0x141989332  f3410f114f24                   movss	%xmm1, 0x24(%r15)
// Frames: F99
// 0x141989338  f3450f106f28                   movss	0x28(%r15), %xmm13
// Frames: F100
// 0x14198933e  8b4708                         movl	0x8(%rdi), %eax
// 0x141989341  85c0                           testl	%eax, %eax
// 0x141989343  4c89ac24b0000000               movq	%r13, 0xb0(%rsp)
// 0x14198934b  7443                           je	0x141989390 <.text+0x1988390>
// 0x14198934d  83f802                         cmpl	$0x2, %eax
// 0x141989350  754e                           jne	0x1419893a0 <.text+0x19883a0>
// Frames: F101
// 0x141989352  f30f10770c                     movss	0xc(%rdi), %xmm6
// 0x141989357  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F102
// 0x14198935b  418904b6                       movl	%eax, (%r14,%rsi,4)
// 0x14198935f  f3410f104728                   movss	0x28(%r15), %xmm0
// Frames: F102
// 0x141989365  f30f58c6                       addss	%xmm6, %xmm0
// Frames: F93 -> F103
// 0x141989369  410f28cc                       movaps	%xmm12, %xmm1
// 0x14198936d  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F95 -> F103
// 0x141989371  450f28fe                       movaps	%xmm14, %xmm15
// 0x141989375  f3440f5df9                     minss	%xmm1, %xmm15
// Frames: F103
// 0x14198937a  f3450f117f28                   movss	%xmm15, 0x28(%r15)
// 0x141989380  eb46                           jmp	0x1419893c8 <.text+0x19883c8>
// 0x141989382  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x141989390  4531ed                         xorl	%r13d, %r13d
// 0x141989393  450f28fd                       movaps	%xmm13, %xmm15
// 0x141989397  eb32                           jmp	0x1419893cb <.text+0x19883cb>
// 0x141989399  0f1f8000000000                 nopl	(%rax)
// Frames: F104
// 0x1419893a0  f30f10770c                     movss	0xc(%rdi), %xmm6
// Frames: F93 -> F105
// 0x1419893a5  0f57c0                         xorps	%xmm0, %xmm0
// 0x1419893a8  f30f5fc6                       maxss	%xmm6, %xmm0
// Frames: F95 -> F105
// 0x1419893ac  450f28fe                       movaps	%xmm14, %xmm15
// 0x1419893b0  f3440f5df8                     minss	%xmm0, %xmm15
// 0x1419893b5  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F105
// 0x1419893b9  418904b6                       movl	%eax, (%r14,%rsi,4)
// Frames: F105
// 0x1419893bd  f3450f117f28                   movss	%xmm15, 0x28(%r15)
// Frames: F106
// 0x1419893c3  f3410f5cf5                     subss	%xmm13, %xmm6
// 0x1419893c8  41b501                         movb	$0x1, %r13b
// Frames: F107
// 0x1419893cb  8b4728                         movl	0x28(%rdi), %eax
// 0x1419893ce  450f57c0                       xorps	%xmm8, %xmm8
// 0x1419893d2  450f57d2                       xorps	%xmm10, %xmm10
// 0x1419893d6  85c0                           testl	%eax, %eax
// 0x1419893d8  0f84b1000000                   je	0x14198948f <.text+0x198848f>
// 0x1419893de  83f802                         cmpl	$0x2, %eax
// 0x1419893e1  754d                           jne	0x141989430 <.text+0x1988430>
// Frames: F108
// 0x1419893e3  f20f10472c                     movsd	0x2c(%rdi), %xmm0
// 0x1419893e8  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F109
// 0x1419893ec  418904b6                       movl	%eax, (%r14,%rsi,4)
// Frames: F110 -> F111 -> F109
// 0x1419893f0  f2410f104f2c                   movsd	0x2c(%r15), %xmm1
// 0x1419893f6  0f58c8                         addps	%xmm0, %xmm1
// Frames: F112 -> F113 -> F114 -> F115
// 0x1419893f9  0f28d1                         movaps	%xmm1, %xmm2
// 0x1419893fc  0f59d1                         mulps	%xmm1, %xmm2
// 0x1419893ff  f30f16da                       movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
// 0x141989403  f30f58da                       addss	%xmm2, %xmm3
// Frames: F116 -> F115
// 0x141989407  0f57d2                         xorps	%xmm2, %xmm2
// 0x14198940a  f30f51d3                       sqrtss	%xmm3, %xmm2
// 0x14198940e  410f28e6                       movaps	%xmm14, %xmm4
// 0x141989412  f30fc2e301                     cmpltss	%xmm3, %xmm4
// 0x141989417  0f54d4                         andps	%xmm4, %xmm2
// 0x14198941a  410f55e6                       andnps	%xmm14, %xmm4
// 0x14198941e  0f56e2                         orps	%xmm2, %xmm4
// 0x141989421  f30f12d4                       movsldup	%xmm4, %xmm2            # xmm2 = xmm4[0,0,2,2]
// 0x141989425  0f5eca                         divps	%xmm2, %xmm1
// Frames: F115
// 0x141989428  410f134f2c                     movlps	%xmm1, 0x2c(%r15)
// 0x14198942d  eb4c                           jmp	0x14198947b <.text+0x198847b>
// 0x14198942f  90                             nop
// Frames: F117
// 0x141989430  f20f10472c                     movsd	0x2c(%rdi), %xmm0
// Frames: F118
// 0x141989435  f2410f104f2c                   movsd	0x2c(%r15), %xmm1
// Frames: F112 -> F113 -> F114 -> F118
// 0x14198943b  0f28d1                         movaps	%xmm1, %xmm2
// 0x14198943e  0f59d1                         mulps	%xmm1, %xmm2
// 0x141989441  f30f16da                       movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
// 0x141989445  f30f58da                       addss	%xmm2, %xmm3
// Frames: F116 -> F118
// 0x141989449  0f57d2                         xorps	%xmm2, %xmm2
// 0x14198944c  f30f51d3                       sqrtss	%xmm3, %xmm2
// 0x141989450  410f28e6                       movaps	%xmm14, %xmm4
// 0x141989454  f30fc2e301                     cmpltss	%xmm3, %xmm4
// 0x141989459  0f54d4                         andps	%xmm4, %xmm2
// 0x14198945c  410f55e6                       andnps	%xmm14, %xmm4
// 0x141989460  0f56e2                         orps	%xmm2, %xmm4
// 0x141989463  f30f12d4                       movsldup	%xmm4, %xmm2            # xmm2 = xmm4[0,0,2,2]
// 0x141989467  0f5eca                         divps	%xmm2, %xmm1
// 0x14198946a  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F119
// 0x14198946e  418904b6                       movl	%eax, (%r14,%rsi,4)
// Frames: F119
// 0x141989472  f2410f11472c                   movsd	%xmm0, 0x2c(%r15)
// Frames: F120 -> F121 -> F122
// 0x141989478  0f5cc1                         subps	%xmm1, %xmm0
// Frames: F112 -> F123 -> F32
// 0x14198947b  0f59c0                         mulps	%xmm0, %xmm0
// 0x14198947e  f30f16c8                       movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
// 0x141989482  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F124 -> F125 -> F123 -> F32
// 0x141989486  450f57d2                       xorps	%xmm10, %xmm10
// 0x14198948a  f3440f51d1                     sqrtss	%xmm1, %xmm10
// Frames: F126
// 0x14198948f  8b4718                         movl	0x18(%rdi), %eax
// 0x141989492  85c0                           testl	%eax, %eax
// 0x141989494  7464                           je	0x1419894fa <.text+0x19884fa>
// 0x141989496  83f802                         cmpl	$0x2, %eax
// 0x141989499  7535                           jne	0x1419894d0 <.text+0x19884d0>
// Frames: F127
// 0x14198949b  f3440f10471c                   movss	0x1c(%rdi), %xmm8
// 0x1419894a1  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F128
// 0x1419894a5  418904b6                       movl	%eax, (%r14,%rsi,4)
// 0x1419894a9  f3410f104738                   movss	0x38(%r15), %xmm0
// Frames: F128
// 0x1419894af  f3410f58c0                     addss	%xmm8, %xmm0
// Frames: F93 -> F129
// 0x1419894b4  0f57c9                         xorps	%xmm1, %xmm1
// 0x1419894b7  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F95 -> F129
// 0x1419894bb  410f28c6                       movaps	%xmm14, %xmm0
// 0x1419894bf  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F129
// 0x1419894c3  f3410f114738                   movss	%xmm0, 0x38(%r15)
// 0x1419894c9  eb2f                           jmp	0x1419894fa <.text+0x19884fa>
// 0x1419894cb  0f1f440000                     nopl	(%rax,%rax)
// Frames: F130
// 0x1419894d0  f3440f10471c                   movss	0x1c(%rdi), %xmm8
// Frames: F93 -> F131
// 0x1419894d6  0f57c0                         xorps	%xmm0, %xmm0
// 0x1419894d9  f3410f5f4738                   maxss	0x38(%r15), %xmm0
// Frames: F95 -> F131
// 0x1419894df  410f28ce                       movaps	%xmm14, %xmm1
// 0x1419894e3  f30f5dc8                       minss	%xmm0, %xmm1
// 0x1419894e7  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F132
// 0x1419894eb  418904b6                       movl	%eax, (%r14,%rsi,4)
// Frames: F132
// 0x1419894ef  f3450f114738                   movss	%xmm8, 0x38(%r15)
// Frames: F133
// 0x1419894f5  f3440f5cc1                     subss	%xmm1, %xmm8
// Frames: F134
// 0x1419894fa  4c89f9                         movq	%r15, %rcx
// 0x1419894fd  e86e0fd3fe                     callq	0x1406ba470 <.text+0x6b9470>
// 0x141989502  4189c4                         movl	%eax, %r12d
// Frames: F135
// 0x141989505  8b4710                         movl	0x10(%rdi), %eax
// 0x141989508  85c0                           testl	%eax, %eax
// 0x14198950a  7444                           je	0x141989550 <.text+0x1988550>
// 0x14198950c  83f802                         cmpl	$0x2, %eax
// 0x14198950f  754f                           jne	0x141989560 <.text+0x1988560>
// Frames: F136
// 0x141989511  f3440f104f14                   movss	0x14(%rdi), %xmm9
// 0x141989517  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F137
// 0x14198951b  418904b6                       movl	%eax, (%r14,%rsi,4)
// 0x14198951f  f3410f104734                   movss	0x34(%r15), %xmm0
// Frames: F137
// 0x141989525  f3410f58c1                     addss	%xmm9, %xmm0
// Frames: F93 -> F138
// 0x14198952a  0f57c9                         xorps	%xmm1, %xmm1
// 0x14198952d  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F95 -> F138
// 0x141989531  410f28c6                       movaps	%xmm14, %xmm0
// 0x141989535  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F138
// 0x141989539  f3410f114734                   movss	%xmm0, 0x34(%r15)
// Frames: F139
// 0x14198953f  4c89f9                         movq	%r15, %rcx
// 0x141989542  e8290fd3fe                     callq	0x1406ba470 <.text+0x6b9470>
// 0x141989547  89c7                           movl	%eax, %edi
// 0x141989549  eb66                           jmp	0x1419895b1 <.text+0x19885b1>
// 0x14198954b  0f1f440000                     nopl	(%rax,%rax)
// 0x141989550  450f57c9                       xorps	%xmm9, %xmm9
// 0x141989554  4489e7                         movl	%r12d, %edi
// 0x141989557  eb58                           jmp	0x1419895b1 <.text+0x19885b1>
// 0x141989559  0f1f8000000000                 nopl	(%rax)
// Frames: F140
// 0x141989560  450f28dd                       movaps	%xmm13, %xmm11
// 0x141989564  450f28ec                       movaps	%xmm12, %xmm13
// 0x141989568  450f28e2                       movaps	%xmm10, %xmm12
// 0x14198956c  f3440f104f14                   movss	0x14(%rdi), %xmm9
// Frames: F93 -> F141
// 0x141989572  0f57c0                         xorps	%xmm0, %xmm0
// 0x141989575  f3410f5f4734                   maxss	0x34(%r15), %xmm0
// Frames: F95 -> F141
// 0x14198957b  450f28d6                       movaps	%xmm14, %xmm10
// 0x14198957f  f3440f5dd0                     minss	%xmm0, %xmm10
// 0x141989584  8b442428                       movl	0x28(%rsp), %eax
// Frames: F90 -> F91 -> F142
// 0x141989588  418904b6                       movl	%eax, (%r14,%rsi,4)
// Frames: F142
// 0x14198958c  f3450f114f34                   movss	%xmm9, 0x34(%r15)
// Frames: F143
// 0x141989592  4c89f9                         movq	%r15, %rcx
// 0x141989595  e8d60ed3fe                     callq	0x1406ba470 <.text+0x6b9470>
// 0x14198959a  89c7                           movl	%eax, %edi
// Frames: F144
// 0x14198959c  f3450f5cca                     subss	%xmm10, %xmm9
// 0x1419895a1  450f28d4                       movaps	%xmm12, %xmm10
// 0x1419895a5  450f28e5                       movaps	%xmm13, %xmm12
// 0x1419895a9  450f28eb                       movaps	%xmm11, %xmm13
// 0x1419895ad  450f57db                       xorps	%xmm11, %xmm11
// Frames: F145
// 0x1419895b1  410f2efb                       ucomiss	%xmm11, %xmm7
// 0x1419895b5  0f95c1                         setne	%cl
// 0x1419895b8  450f2ec3                       ucomiss	%xmm11, %xmm8
// 0x1419895bc  0f95c0                         setne	%al
// 0x1419895bf  4138fc                         cmpb	%dil, %r12b
// 0x1419895c2  755c                           jne	0x141989620 <.text+0x1988620>
// 0x1419895c4  4408e9                         orb	%r13b, %cl
// 0x1419895c7  4020f9                         andb	%dil, %cl
// 0x1419895ca  7554                           jne	0x141989620 <.text+0x1988620>
// 0x1419895cc  4020f8                         andb	%dil, %al
// 0x1419895cf  754f                           jne	0x141989620 <.text+0x1988620>
// Frames: F146
// 0x1419895d1  4584e4                         testb	%r12b, %r12b
// 0x1419895d4  0f84ab000000                   je	0x141989685 <.text+0x1988685>
// Frames: F147
// 0x1419895da  4084ff                         testb	%dil, %dil
// 0x1419895dd  4c8bb424a8000000               movq	0xa8(%rsp), %r14
// 0x1419895e5  4c8ba424a0000000               movq	0xa0(%rsp), %r12
// 0x1419895ed  4c8bac2498000000               movq	0x98(%rsp), %r13
// 0x1419895f5  0f8508010000                   jne	0x141989703 <.text+0x1988703>
// Frames: F148
// 0x1419895fb  488d4c2440                     leaq	0x40(%rsp), %rcx
// 0x141989600  4c89fa                         movq	%r15, %rdx
// 0x141989603  e8e80dd3fe                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F149
// 0x141989608  c64424300b                     movb	$0xb, 0x30(%rsp)
// 0x14198960d  e9a2000000                     jmp	0x1419896b4 <.text+0x19886b4>
// 0x141989612  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F150
// 0x141989620  498b37                         movq	(%r15), %rsi
// 0x141989623  8b442468                       movl	0x68(%rsp), %eax
// 0x141989627  488b8c24d0000000               movq	0xd0(%rsp), %rcx
// Frames: F151 -> F152 -> F27 -> F150
// 0x14198962f  8901                           movl	%eax, (%rcx)
// 0x141989631  488b842488000000               movq	0x88(%rsp), %rax
// Frames: F153 -> F26 -> F27 -> F150
// 0x141989639  488b5030                       movq	0x30(%rax), %rdx
// Frames: F154 -> F27 -> F150
// 0x14198963d  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F153 -> F26 -> F27 -> F150
// 0x141989641  483b5020                       cmpq	0x20(%rax), %rdx
// 0x141989645  7519                           jne	0x141989660 <.text+0x1988660>
// Frames: F155 -> F26 -> F27 -> F150
// 0x141989647  488b8c24c0000000               movq	0xc0(%rsp), %rcx
// 0x14198964f  e85c058ffe                     callq	0x140279bb0 <.text+0x278bb0>
// 0x141989654  488b842488000000               movq	0x88(%rsp), %rax
// Frames: F25 -> F26 -> F27 -> F150
// 0x14198965c  488b5030                       movq	0x30(%rax), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F150
// 0x141989660  4889c1                         movq	%rax, %rcx
// 0x141989663  488b4028                       movq	0x28(%rax), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F150
// 0x141989667  48c1e204                       shlq	$0x4, %rdx
// Frames: F30 -> F31 -> F26 -> F27 -> F150
// 0x14198966b  4c893410                       movq	%r14, (%rax,%rdx)
// 0x14198966f  4889741008                     movq	%rsi, 0x8(%rax,%rdx)
// Frames: F33 -> F26 -> F27 -> F150
// 0x141989674  48ff4130                       incq	0x30(%rcx)
// Frames: F34 -> F27 -> F150
// 0x141989678  48ff4140                       incq	0x40(%rcx)
// Frames: F146
// 0x14198967c  4584e4                         testb	%r12b, %r12b
// 0x14198967f  0f8555ffffff                   jne	0x1419895da <.text+0x19885da>
// 0x141989685  4084ff                         testb	%dil, %dil
// 0x141989688  4c8bb424a8000000               movq	0xa8(%rsp), %r14
// 0x141989690  4c8ba424a0000000               movq	0xa0(%rsp), %r12
// 0x141989698  4c8bac2498000000               movq	0x98(%rsp), %r13
// 0x1419896a0  7461                           je	0x141989703 <.text+0x1988703>
// Frames: F156
// 0x1419896a2  488d4c2440                     leaq	0x40(%rsp), %rcx
// 0x1419896a7  4c89fa                         movq	%r15, %rdx
// 0x1419896aa  e8410dd3fe                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F157
// 0x1419896af  c64424300a                     movb	$0xa, 0x30(%rsp)
// Frames: F151 -> F152 -> F27 -> F32
// 0x1419896b4  8b44242c                       movl	0x2c(%rsp), %eax
// 0x1419896b8  488b4c2458                     movq	0x58(%rsp), %rcx
// 0x1419896bd  8901                           movl	%eax, (%rcx)
// Frames: F153 -> F158 -> F27 -> F32
// 0x1419896bf  488b5530                       movq	0x30(%rbp), %rdx
// Frames: F159 -> F27 -> F32
// 0x1419896c3  488b7540                       movq	0x40(%rbp), %rsi
// Frames: F153 -> F158 -> F27 -> F32
// 0x1419896c7  483b5520                       cmpq	0x20(%rbp), %rdx
// 0x1419896cb  750e                           jne	0x1419896db <.text+0x19886db>
// Frames: F155 -> F158 -> F27 -> F32
// 0x1419896cd  488b4c2450                     movq	0x50(%rsp), %rcx
// 0x1419896d2  e8390e8ffe                     callq	0x14027a510 <.text+0x279510>
// Frames: F25 -> F158 -> F27 -> F32
// 0x1419896d7  488b5530                       movq	0x30(%rbp), %rdx
// Frames: F23 -> F24 -> F25 -> F158 -> F27 -> F32
// 0x1419896db  488b4528                       movq	0x28(%rbp), %rax
// Frames: F29 -> F25 -> F158 -> F27 -> F32
// 0x1419896df  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F30 -> F31 -> F158 -> F27 -> F32
// 0x1419896e3  488934c8                       movq	%rsi, (%rax,%rcx,8)
// 0x1419896e7  0f10442430                     movups	0x30(%rsp), %xmm0
// 0x1419896ec  0f104c243c                     movups	0x3c(%rsp), %xmm1
// 0x1419896f1  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// 0x1419896f6  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// Frames: F33 -> F158 -> F27 -> F32
// 0x1419896fb  48ff4530                       incq	0x30(%rbp)
// Frames: F160 -> F27 -> F32
// 0x1419896ff  48ff4540                       incq	0x40(%rbp)
// Frames: F161
// 0x141989703  450f2ec3                       ucomiss	%xmm11, %xmm8
// 0x141989707  7527                           jne	0x141989730 <.text+0x1988730>
// 0x141989709  410f2efb                       ucomiss	%xmm11, %xmm7
// 0x14198970d  7521                           jne	0x141989730 <.text+0x1988730>
// 0x14198970f  410f2ef3                       ucomiss	%xmm11, %xmm6
// 0x141989713  751b                           jne	0x141989730 <.text+0x1988730>
// 0x141989715  450f2ecb                       ucomiss	%xmm11, %xmm9
// 0x141989719  7515                           jne	0x141989730 <.text+0x1988730>
// Frames: F162
// 0x14198971b  450f2ed3                       ucomiss	%xmm11, %xmm10
// 0x14198971f  0f8742010000                   ja	0x141989867 <.text+0x1988867>
// 0x141989725  e9ca010000                     jmp	0x1419898f4 <.text+0x19888f4>
// 0x14198972a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F163 -> F161
// 0x141989730  f3440f1154246c                 movss	%xmm10, 0x6c(%rsp)
// 0x141989737  440f29bc2400010000             movaps	%xmm15, 0x100(%rsp)
// 0x141989740  f3440f116c2470                 movss	%xmm13, 0x70(%rsp)
// 0x141989747  450f28ee                       movaps	%xmm14, %xmm13
// 0x14198974b  450f28f4                       movaps	%xmm12, %xmm14
// 0x14198974f  440f283dc9bf7b00               movaps	0x7bbfc9(%rip), %xmm15  # 0x142145720
// 0x141989757  410f54f7                       andps	%xmm15, %xmm6
// Frames: F164 -> F165
// 0x14198975b  0f28c6                         movaps	%xmm6, %xmm0
// 0x14198975e  e88db57b00                     callq	0x142144cf0 <.text+0x2143cf0>
// 0x141989763  440f28d0                       movaps	%xmm0, %xmm10
// Frames: F163 -> F166
// 0x141989767  410f54ff                       andps	%xmm15, %xmm7
// Frames: F164 -> F166
// 0x14198976b  0f28c7                         movaps	%xmm7, %xmm0
// 0x14198976e  e87db57b00                     callq	0x142144cf0 <.text+0x2143cf0>
// 0x141989773  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F163 -> F166
// 0x141989777  450f54cf                       andps	%xmm15, %xmm9
// Frames: F164 -> F166
// 0x14198977b  410f28c1                       movaps	%xmm9, %xmm0
// 0x14198977f  e86cb57b00                     callq	0x142144cf0 <.text+0x2143cf0>
// 0x141989784  440f28e0                       movaps	%xmm0, %xmm12
// Frames: F166
// 0x141989788  f3450f58e3                     addss	%xmm11, %xmm12
// Frames: F163 -> F166
// 0x14198978d  450f54c7                       andps	%xmm15, %xmm8
// Frames: F164 -> F166
// 0x141989791  410f28c0                       movaps	%xmm8, %xmm0
// 0x141989795  e856b57b00                     callq	0x142144cf0 <.text+0x2143cf0>
// 0x14198979a  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F166
// 0x14198979e  f3450f58dc                     addss	%xmm12, %xmm11
// Frames: F167
// 0x1419897a3  f3440f58cf                     addss	%xmm7, %xmm9
// 0x1419897a8  f3450f58c8                     addss	%xmm8, %xmm9
// 0x1419897ad  0f57ff                         xorps	%xmm7, %xmm7
// 0x1419897b0  f3410fc2fa01                   cmpltss	%xmm10, %xmm7
// 0x1419897b6  0f54f7                         andps	%xmm7, %xmm6
// 0x1419897b9  410f55f9                       andnps	%xmm9, %xmm7
// 0x1419897bd  0f56fe                         orps	%xmm6, %xmm7
// Frames: F168
// 0x1419897c0  488d4c2440                     leaq	0x40(%rsp), %rcx
// 0x1419897c5  4c89fa                         movq	%r15, %rdx
// 0x1419897c8  e8230cd3fe                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F169
// 0x1419897cd  c644243002                     movb	$0x2, 0x30(%rsp)
// 0x1419897d2  40887c2431                     movb	%dil, 0x31(%rsp)
// 0x1419897d7  f3440f11542434                 movss	%xmm10, 0x34(%rsp)
// 0x1419897de  f3440f115c2438                 movss	%xmm11, 0x38(%rsp)
// 0x1419897e5  f30f117c243c                   movss	%xmm7, 0x3c(%rsp)
// 0x1419897eb  8b44242c                       movl	0x2c(%rsp), %eax
// 0x1419897ef  488b4c2458                     movq	0x58(%rsp), %rcx
// Frames: F151 -> F152 -> F27 -> F169
// 0x1419897f4  8901                           movl	%eax, (%rcx)
// Frames: F153 -> F26 -> F27 -> F169
// 0x1419897f6  488b5530                       movq	0x30(%rbp), %rdx
// Frames: F154 -> F27 -> F169
// 0x1419897fa  488b7540                       movq	0x40(%rbp), %rsi
// Frames: F153 -> F26 -> F27 -> F169
// 0x1419897fe  483b5520                       cmpq	0x20(%rbp), %rdx
// 0x141989802  750e                           jne	0x141989812 <.text+0x1988812>
// Frames: F155 -> F26 -> F27 -> F169
// 0x141989804  488b4c2450                     movq	0x50(%rsp), %rcx
// 0x141989809  e8020d8ffe                     callq	0x14027a510 <.text+0x279510>
// Frames: F25 -> F26 -> F27 -> F169
// 0x14198980e  488b5530                       movq	0x30(%rbp), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F169
// 0x141989812  488b4528                       movq	0x28(%rbp), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F169
// 0x141989816  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F30 -> F31 -> F26 -> F27 -> F169
// 0x14198981a  488934c8                       movq	%rsi, (%rax,%rcx,8)
// 0x14198981e  0f10442430                     movups	0x30(%rsp), %xmm0
// 0x141989823  0f104c243c                     movups	0x3c(%rsp), %xmm1
// 0x141989828  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// 0x14198982d  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// Frames: F33 -> F26 -> F27 -> F169
// 0x141989832  48ff4530                       incq	0x30(%rbp)
// Frames: F34 -> F27 -> F169
// 0x141989836  48ff4540                       incq	0x40(%rbp)
// 0x14198983a  450f57db                       xorps	%xmm11, %xmm11
// 0x14198983e  450f28e6                       movaps	%xmm14, %xmm12
// 0x141989842  450f28f5                       movaps	%xmm13, %xmm14
// 0x141989846  f3440f106c2470                 movss	0x70(%rsp), %xmm13
// 0x14198984d  440f28bc2400010000             movaps	0x100(%rsp), %xmm15
// 0x141989856  f3440f1054246c                 movss	0x6c(%rsp), %xmm10
// Frames: F162
// 0x14198985d  450f2ed3                       ucomiss	%xmm11, %xmm10
// 0x141989861  0f868d000000                   jbe	0x1419898f4 <.text+0x19888f4>
// Frames: F170
// 0x141989867  f3410f10472c                   movss	0x2c(%r15), %xmm0
// 0x14198986d  f3410f104f30                   movss	0x30(%r15), %xmm1
// Frames: F112 -> F123 -> F170
// 0x141989873  f30f59c0                       mulss	%xmm0, %xmm0
// 0x141989877  f30f59c9                       mulss	%xmm1, %xmm1
// 0x14198987b  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F124 -> F125 -> F123 -> F170
// 0x14198987f  0f57f6                         xorps	%xmm6, %xmm6
// 0x141989882  f30f51f1                       sqrtss	%xmm1, %xmm6
// Frames: F171
// 0x141989886  488d4c2440                     leaq	0x40(%rsp), %rcx
// 0x14198988b  4c89fa                         movq	%r15, %rdx
// 0x14198988e  e85d0bd3fe                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F172
// 0x141989893  c644243003                     movb	$0x3, 0x30(%rsp)
// 0x141989898  f30f11742434                   movss	%xmm6, 0x34(%rsp)
// 0x14198989e  f3440f11542438                 movss	%xmm10, 0x38(%rsp)
// 0x1419898a5  8b44242c                       movl	0x2c(%rsp), %eax
// 0x1419898a9  488b4c2458                     movq	0x58(%rsp), %rcx
// Frames: F151 -> F152 -> F27 -> F172
// 0x1419898ae  8901                           movl	%eax, (%rcx)
// Frames: F153 -> F26 -> F27 -> F172
// 0x1419898b0  488b5530                       movq	0x30(%rbp), %rdx
// Frames: F154 -> F27 -> F172
// 0x1419898b4  488b7540                       movq	0x40(%rbp), %rsi
// Frames: F153 -> F26 -> F27 -> F172
// 0x1419898b8  483b5520                       cmpq	0x20(%rbp), %rdx
// 0x1419898bc  750e                           jne	0x1419898cc <.text+0x19888cc>
// Frames: F155 -> F26 -> F27 -> F172
// 0x1419898be  488b4c2450                     movq	0x50(%rsp), %rcx
// 0x1419898c3  e8480c8ffe                     callq	0x14027a510 <.text+0x279510>
// Frames: F25 -> F26 -> F27 -> F172
// 0x1419898c8  488b5530                       movq	0x30(%rbp), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F172
// 0x1419898cc  488b4528                       movq	0x28(%rbp), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F172
// 0x1419898d0  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F30 -> F31 -> F26 -> F27 -> F172
// 0x1419898d4  488934c8                       movq	%rsi, (%rax,%rcx,8)
// 0x1419898d8  0f10442430                     movups	0x30(%rsp), %xmm0
// 0x1419898dd  0f104c243c                     movups	0x3c(%rsp), %xmm1
// 0x1419898e2  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// 0x1419898e7  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// Frames: F33 -> F26 -> F27 -> F172
// 0x1419898ec  48ff4530                       incq	0x30(%rbp)
// Frames: F34 -> F27 -> F172
// 0x1419898f0  48ff4540                       incq	0x40(%rbp)
// Frames: F151 -> F152 -> F27 -> F173
// 0x1419898f4  8b442474                       movl	0x74(%rsp), %eax
// 0x1419898f8  488b8c24e8000000               movq	0xe8(%rsp), %rcx
// 0x141989900  8901                           movl	%eax, (%rcx)
// Frames: F153 -> F26 -> F27 -> F173
// 0x141989902  498b5530                       movq	0x30(%r13), %rdx
// Frames: F154 -> F27 -> F173
// 0x141989906  498b7540                       movq	0x40(%r13), %rsi
// Frames: F153 -> F26 -> F27 -> F173
// 0x14198990a  493b5520                       cmpq	0x20(%r13), %rdx
// 0x14198990e  7511                           jne	0x141989921 <.text+0x1988921>
// Frames: F155 -> F26 -> F27 -> F173
// 0x141989910  488b8c24c8000000               movq	0xc8(%rsp), %rcx
// 0x141989918  e8f3038ffe                     callq	0x140279d10 <.text+0x278d10>
// Frames: F25 -> F26 -> F27 -> F173
// 0x14198991d  498b5530                       movq	0x30(%r13), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F173
// 0x141989921  4c8b442460                     movq	0x60(%rsp), %r8
// 0x141989926  498b4528                       movq	0x28(%r13), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F173
// 0x14198992a  48c1e205                       shlq	$0x5, %rdx
// Frames: F30 -> F31 -> F26 -> F27 -> F173
// 0x14198992e  48893410                       movq	%rsi, (%rax,%rdx)
// 0x141989932  48895c1008                     movq	%rbx, 0x8(%rax,%rdx)
// 0x141989937  4c89441010                     movq	%r8, 0x10(%rax,%rdx)
// 0x14198993c  c644101801                     movb	$0x1, 0x18(%rax,%rdx)
// Frames: F33 -> F26 -> F27 -> F173
// 0x141989941  49ff4530                       incq	0x30(%r13)
// Frames: F34 -> F27 -> F173
// 0x141989945  49ff4540                       incq	0x40(%r13)
// Frames: F174
// 0x141989949  450f2eec                       ucomiss	%xmm12, %xmm13
// 0x14198994d  7651                           jbe	0x1419899a0 <.text+0x19889a0>
// 0x14198994f  450f2ee7                       ucomiss	%xmm15, %xmm12
// 0x141989953  724b                           jb	0x1419899a0 <.text+0x19889a0>
// Frames: F151 -> F152 -> F27 -> F28
// 0x141989955  8b44244c                       movl	0x4c(%rsp), %eax
// 0x141989959  488b8c2480000000               movq	0x80(%rsp), %rcx
// 0x141989961  8901                           movl	%eax, (%rcx)
// 0x141989963  4c8bbc2490000000               movq	0x90(%rsp), %r15
// Frames: F153 -> F26 -> F27 -> F28
// 0x14198996b  498b5730                       movq	0x30(%r15), %rdx
// Frames: F154 -> F27 -> F28
// 0x14198996f  498b7740                       movq	0x40(%r15), %rsi
// Frames: F153 -> F26 -> F27 -> F28
// 0x141989973  493b5720                       cmpq	0x20(%r15), %rdx
// 0x141989977  4c8bac24b0000000               movq	0xb0(%rsp), %r13
// 0x14198997f  0f85fbf7ffff                   jne	0x141989180 <.text+0x1988180>
// Frames: F155 -> F26 -> F27 -> F28
// 0x141989985  488b4c2478                     movq	0x78(%rsp), %rcx
// 0x14198998a  e881038ffe                     callq	0x140279d10 <.text+0x278d10>
// 0x14198998f  4c8b442460                     movq	0x60(%rsp), %r8
// Frames: F25 -> F26 -> F27 -> F28
// 0x141989994  498b5730                       movq	0x30(%r15), %rdx
// 0x141989998  e9e3f7ffff                     jmp	0x141989180 <.text+0x1988180>
// 0x14198999d  0f1f00                         nopl	(%rax)
// Frames: F175
// 0x1419899a0  450f2ee5                       ucomiss	%xmm13, %xmm12
// 0x1419899a4  4c8bac24b0000000               movq	0xb0(%rsp), %r13
// 0x1419899ac  4c89e8                         movq	%r13, %rax
// 0x1419899af  488b8c24b8000000               movq	0xb8(%rsp), %rcx
// 0x1419899b7  4889cf                         movq	%rcx, %rdi
// 0x1419899ba  4c8bbc2490000000               movq	0x90(%rsp), %r15
// 0x1419899c2  0f82e8f7ffff                   jb	0x1419891b0 <.text+0x19881b0>
// 0x1419899c8  450f2efc                       ucomiss	%xmm12, %xmm15
// 0x1419899cc  4c89e8                         movq	%r13, %rax
// 0x1419899cf  4889cf                         movq	%rcx, %rdi
// 0x1419899d2  0f86d8f7ffff                   jbe	0x1419891b0 <.text+0x19881b0>
// Frames: F151 -> F152 -> F27 -> F176
// 0x1419899d8  8b44244c                       movl	0x4c(%rsp), %eax
// 0x1419899dc  488b8c2480000000               movq	0x80(%rsp), %rcx
// 0x1419899e4  8901                           movl	%eax, (%rcx)
// Frames: F153 -> F26 -> F27 -> F176
// 0x1419899e6  498b5730                       movq	0x30(%r15), %rdx
// Frames: F154 -> F27 -> F176
// 0x1419899ea  498b7740                       movq	0x40(%r15), %rsi
// Frames: F153 -> F26 -> F27 -> F176
// 0x1419899ee  493b5720                       cmpq	0x20(%r15), %rdx
// 0x1419899f2  7513                           jne	0x141989a07 <.text+0x1988a07>
// Frames: F155 -> F26 -> F27 -> F176
// 0x1419899f4  488b4c2478                     movq	0x78(%rsp), %rcx
// 0x1419899f9  e812038ffe                     callq	0x140279d10 <.text+0x278d10>
// 0x1419899fe  4c8b442460                     movq	0x60(%rsp), %r8
// Frames: F25 -> F26 -> F27 -> F176
// 0x141989a03  498b5730                       movq	0x30(%r15), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F176
// 0x141989a07  498b4f28                       movq	0x28(%r15), %rcx
// Frames: F29 -> F25 -> F26 -> F27 -> F176
// 0x141989a0b  48c1e205                       shlq	$0x5, %rdx
// 0x141989a0f  488d0411                       leaq	(%rcx,%rdx), %rax
// Frames: F30 -> F31 -> F26 -> F27 -> F176
// 0x141989a13  48893411                       movq	%rsi, (%rcx,%rdx)
// 0x141989a17  b101                           movb	$0x1, %cl
// 0x141989a19  e974f7ffff                     jmp	0x141989192 <.text+0x1988192>
// Frames: F177
// 0x141989a1e  0f28b42410010000               movaps	0x110(%rsp), %xmm6
// 0x141989a26  0f28bc2420010000               movaps	0x120(%rsp), %xmm7
// 0x141989a2e  440f28842430010000             movaps	0x130(%rsp), %xmm8
// 0x141989a37  440f288c2440010000             movaps	0x140(%rsp), %xmm9
// 0x141989a40  440f28942450010000             movaps	0x150(%rsp), %xmm10
// 0x141989a49  440f289c2460010000             movaps	0x160(%rsp), %xmm11
// 0x141989a52  440f28a42470010000             movaps	0x170(%rsp), %xmm12
// 0x141989a5b  440f28ac2480010000             movaps	0x180(%rsp), %xmm13
// 0x141989a64  440f28b42490010000             movaps	0x190(%rsp), %xmm14
// 0x141989a6d  440f28bc24a0010000             movaps	0x1a0(%rsp), %xmm15
// 0x141989a76  4881c4b8010000                 addq	$0x1b8, %rsp            # imm = 0x1B8
// 0x141989a7d  5b                             popq	%rbx
// 0x141989a7e  5d                             popq	%rbp
// 0x141989a7f  5f                             popq	%rdi
// 0x141989a80  5e                             popq	%rsi
// 0x141989a81  415c                           popq	%r12
// 0x141989a83  415d                           popq	%r13
// 0x141989a85  415e                           popq	%r14
// 0x141989a87  415f                           popq	%r15
// 0x141989a89  c3                             retq
// 0x141989a8a  31c0                           xorl	%eax, %eax
// Frames: F178 -> F51
// 0x141989a8c  4889442430                     movq	%rax, 0x30(%rsp)
// 0x141989a91  48895c2438                     movq	%rbx, 0x38(%rsp)
// 0x141989a96  488d054397d600                 leaq	0xd69743(%rip), %rax    # 0x1426f31e0
// 0x141989a9d  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141989aa2  488d0d4f96d600                 leaq	0xd6964f(%rip), %rcx    # 0x1426f30f8
// 0x141989aa9  4c8d0d7896d600                 leaq	0xd69678(%rip), %r9     # 0x1426f3128
// 0x141989ab0  4c8d442430                     leaq	0x30(%rsp), %r8
// 0x141989ab5  ba2b000000                     movl	$0x2b, %edx
// 0x141989aba  e861547200                     callq	0x1420aef20 <.text+0x20adf20>
// Frames: F179 -> F46
// 0x141989abf  488d0d0297d600                 leaq	0xd69702(%rip), %rcx    # 0x1426f31c8
// 0x141989ac6  e8754f7200                     callq	0x1420aea40 <.text+0x20ada40>
// 0x141989acb  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/edit_roof.rs:10:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F2: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:806:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:469:0 bevy_ecs::event::EventReader::read
// F4: crates/systems/roof/src/edit_roof.rs:23:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:547:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F6: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:811:0 bevy_ecs::event::EventIteratorWithId::new
// F7: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:723:0 bevy_ecs::event::ManualEventReader::read_with_id
// F8: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:718:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:813:0 bevy_ecs::event::EventIteratorWithId::new
// F11: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:816:0 bevy_ecs::event::EventIteratorWithId::new
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:354:0 core::slice::index::impl$4::get
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:480:0 core::slice::index::impl$6::get
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F15: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:817:0 bevy_ecs::event::EventIteratorWithId::new
// F16: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:818:0 bevy_ecs::event::EventIteratorWithId::new
// F17: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1007:0 core::slice::impl$0::get
// F18: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:823:0 bevy_ecs::event::EventIteratorWithId::new
// F19: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F20: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F22: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:825:0 bevy_ecs::event::EventIteratorWithId::new
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1974:0 alloc::vec::Vec::push
// F26: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::event::Events::send
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:625:0 bevy_ecs::event::EventWriter::send
// F28: crates/systems/roof/src/edit_roof.rs:172:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 alloc::vec::Vec::as_mut_ptr
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F31: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1975:0 alloc::vec::Vec::push
// F32: crates/systems/roof/src/edit_roof.rs:0:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1976:0 alloc::vec::Vec::push
// F34: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::event::Events::send
// F35: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:705:0 core::option::Option::as_mut
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:300:0 core::iter::adapters::chain::and_then_or_clear
// F37: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next
// F38: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:845:0 bevy_ecs::event::impl$23::next
// F39: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:771:0 bevy_ecs::event::impl$20::next
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next::closure$0
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1513:0 core::option::Option::or_else
// F42: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F43: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F44: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F45: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:850:0 bevy_ecs::event::impl$23::next
// F46: crates/systems/roof/src/edit_roof.rs:24:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:931:0 core::option::Option::unwrap
// F48: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:740:0 bevy_ecs::entity::Entities::get
// F49: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:866:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F50: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:967:0 bevy_ecs::system::query::Query::get_mut
// F51: crates/systems/roof/src/edit_roof.rs:26:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F52: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F53: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1488:0 core::cmp::impls::impl$24::ne
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/nonzero.rs:137:0 core::num::nonzero::impl$2::ne
// F55: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:741:0 bevy_ecs::entity::Entities::get
// F56: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:74:0 bevy_ecs::archetype::impl$24::eq
// F57: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:742:0 bevy_ecs::entity::Entities::get
// F58: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F59: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:870:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F60: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 core::bool::impl$0::then
// F61: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F62: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F63: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F64: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F65: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/bit.rs:178:0 fixedbitset::FixedBitSet::get_unchecked
// F66: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1952:0 core::result::impl$26::branch
// F67: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F68: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:886:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F69: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:888:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F70: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:693:0 bevy_ecs::storage::table::Table::get_column
// F71: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1296:0 bevy_ecs::query::fetch::impl$31::set_table
// F72: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1285:0 bevy_ecs::query::fetch::impl$31::set_archetype
// F73: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F75: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::impl$11::get::closure$0
// F76: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F77: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F78: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs:398:0 nonmax::NonMaxUsize::get
// F79: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:372:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F80: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:964:0 core::ptr::const_ptr::impl$0::add
// F81: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:246:0 core::slice::index::impl$2::get_unchecked
// F82: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:686:0 nonmax::NonMaxUsize::get
// F83: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:342:0 bevy_ecs::storage::table::Column::get_changed_ticks_slice
// F84: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1300:0 bevy_ecs::query::fetch::impl$31::set_table
// F85: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs:506:0 bevy_ptr::ThinSlicePtr::get
// F86: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1317:0 bevy_ecs::query::fetch::impl$31::fetch
// F87: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:892:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F88: crates/systems/roof/src/edit_roof.rs:28:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F89: crates/systems/roof/src/edit_roof.rs:35:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F90: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$49::set_changed
// F91: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$50::deref_mut
// F92: crates/systems/roof/src/edit_roof.rs:36:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F93: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1521:0 core::f32::impl$0::clamp
// F94: crates/systems/roof/src/edit_roof.rs:37:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F95: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1524:0 core::f32::impl$0::clamp
// F96: crates/systems/roof/src/edit_roof.rs:30:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F97: crates/systems/roof/src/edit_roof.rs:32:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F98: crates/systems/roof/src/edit_roof.rs:33:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F99: crates/systems/roof/src/edit_roof.rs:44:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F100: crates/systems/roof/src/edit_roof.rs:45:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F101: crates/systems/roof/src/edit_roof.rs:53:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F102: crates/systems/roof/src/edit_roof.rs:55:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F103: crates/systems/roof/src/edit_roof.rs:56:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F104: crates/systems/roof/src/edit_roof.rs:47:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F105: crates/systems/roof/src/edit_roof.rs:50:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F106: crates/systems/roof/src/edit_roof.rs:51:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F107: crates/systems/roof/src/edit_roof.rs:62:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F108: crates/systems/roof/src/edit_roof.rs:69:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F109: crates/systems/roof/src/edit_roof.rs:70:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F110: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:768:0 core::ops::arith::impl$308::add_assign
// F111: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:1013:0 glam::f32::vec2::impl$13::add_assign
// F112: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:158:0 glam::f32::vec2::Vec2::dot
// F113: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:391:0 glam::f32::vec2::Vec2::length_squared
// F114: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:742:0 glam::f32::vec2::Vec2::clamp_length_max
// F115: crates/systems/roof/src/edit_roof.rs:71:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F116: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:743:0 glam::f32::vec2::Vec2::clamp_length_max
// F117: crates/systems/roof/src/edit_roof.rs:64:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F118: crates/systems/roof/src/edit_roof.rs:65:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F119: crates/systems/roof/src/edit_roof.rs:66:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F120: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:221:0 core::ops::arith::impl$68::sub
// F121: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:1053:0 glam::f32::vec2::impl$17::sub
// F122: crates/systems/roof/src/edit_roof.rs:67:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F123: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:381:0 glam::f32::vec2::Vec2::length
// F124: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:442:0 std::f32::impl$0::sqrt
// F125: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs:188:0 glam::f32::vec2::Vec2::dot
// F126: crates/systems/roof/src/edit_roof.rs:76:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F127: crates/systems/roof/src/edit_roof.rs:83:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F128: crates/systems/roof/src/edit_roof.rs:84:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F129: crates/systems/roof/src/edit_roof.rs:85:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F130: crates/systems/roof/src/edit_roof.rs:78:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F131: crates/systems/roof/src/edit_roof.rs:79:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F132: crates/systems/roof/src/edit_roof.rs:80:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F133: crates/systems/roof/src/edit_roof.rs:81:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F134: crates/systems/roof/src/edit_roof.rs:90:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F135: crates/systems/roof/src/edit_roof.rs:92:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F136: crates/systems/roof/src/edit_roof.rs:100:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F137: crates/systems/roof/src/edit_roof.rs:101:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F138: crates/systems/roof/src/edit_roof.rs:102:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F139: crates/systems/roof/src/edit_roof.rs:103:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F140: crates/systems/roof/src/edit_roof.rs:94:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F141: crates/systems/roof/src/edit_roof.rs:95:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F142: crates/systems/roof/src/edit_roof.rs:96:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F143: crates/systems/roof/src/edit_roof.rs:97:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F144: crates/systems/roof/src/edit_roof.rs:98:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F145: crates/systems/roof/src/edit_roof.rs:109:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F146: crates/systems/roof/src/edit_roof.rs:117:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F147: crates/systems/roof/src/edit_roof.rs:124:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F148: crates/systems/roof/src/edit_roof.rs:127:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F149: crates/systems/roof/src/edit_roof.rs:125:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F150: crates/systems/roof/src/edit_roof.rs:114:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F151: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$29::set_changed
// F152: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F153: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1970:0 alloc::vec::Vec::push
// F154: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::event::Events::send
// F155: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1971:0 alloc::vec::Vec::push
// F156: crates/systems/roof/src/edit_roof.rs:120:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F157: crates/systems/roof/src/edit_roof.rs:118:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F158: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::change_detection::impl$30::deref_mut
// F159: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::change_detection::impl$30::deref_mut
// F160: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::change_detection::impl$30::deref_mut
// F161: crates/systems/roof/src/edit_roof.rs:131:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F162: crates/systems/roof/src/edit_roof.rs:154:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F163: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:199:0 std::f32::impl$0::abs
// F164: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:74:0 std::f32::impl$0::ceil
// F165: crates/systems/roof/src/edit_roof.rs:137:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F166: crates/systems/roof/src/edit_roof.rs:138:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F167: crates/systems/roof/src/edit_roof.rs:143:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F168: crates/systems/roof/src/edit_roof.rs:150:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F169: crates/systems/roof/src/edit_roof.rs:139:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F170: crates/systems/roof/src/edit_roof.rs:157:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F171: crates/systems/roof/src/edit_roof.rs:160:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F172: crates/systems/roof/src/edit_roof.rs:155:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F173: crates/systems/roof/src/edit_roof.rs:164:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F174: crates/systems/roof/src/edit_roof.rs:171:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F175: crates/systems/roof/src/edit_roof.rs:177:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F176: crates/systems/roof/src/edit_roof.rs:178:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F177: crates/systems/roof/src/edit_roof.rs:185:0 system_roof::edit_roof::edit_roof::h98bc9e392c2c2063
// F178: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1077:0 core::result::Result::unwrap
// F179: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:933:0 core::option::Option::unwrap
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/edit_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0b50676655d6ee9154bde21133b33465f56ce90bb6dd3aabd1fdf78e85050fc0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: edit_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3197
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::edit_roof::edit_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3197,"record_offset":496,"rva":26775440,"va":5395484560}
// 编译实例 RVAs: 0x1988f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: edit_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8107405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395484560,"raw_hex":"e88db57b00","size":5,"soft_fail":false,"target_va":5403593968,"va":5395486558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-24181412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395484560,"raw_hex":"e85c058ffe","size":5,"soft_fail":false,"target_va":5371304880,"va":5395486287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7491445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395484560,"raw_hex":"e8754f7200","size":5,"soft_fail":false,"target_va":5402978880,"va":5395487430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7492705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395484560,"raw_hex":"e861547200","size":5,"soft_fail":false,"target_va":5402980128,"va":5395487418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-19722386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395484560,"raw_hex":"e86e0fd3fe","size":5,"soft_fail":false,"target_va":5375763568,"va":5395485949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-19722776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395484560,"raw_hex":"e8e80dd3fe","size":5,"soft_fail":false,"target_va":5375763440,"va":5395486211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20049169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395484560,"raw_hex":"e8ef12cefe","size":5,"soft_fail":false,"target_va":5375436032,"va":5395485196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3197,"record_offset":3148,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3197,"record_offset":668,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3197,"record_offset":1212,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3197,"record_offset":1816,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3197,"record_offset":2164,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3197,"record_offset":3204,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3197,"record_offset":2128,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3197,"record_offset":5952,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3197,"record_offset":3612,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":22247,"source_line":741,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3197,"record_offset":3900,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3197,"record_offset":3556,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3197,"record_offset":3844,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3197,"record_offset":4604,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3197,"record_offset":4632,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3197,"record_offset":1352,"parent_record_offset":1320,"ancestor_record_offset":496,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3197,"record_offset":1320,"parent_record_offset":1276,"ancestor_record_offset":496,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3197,"record_offset":1276,"parent_record_offset":1240,"ancestor_record_offset":496,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3197,"record_offset":2376,"parent_record_offset":2232,"ancestor_record_offset":496,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3197,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":496,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3197,"record_offset":1420,"parent_record_offset":1212,"ancestor_record_offset":496,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3197,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":496,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3197,"record_offset":2232,"parent_record_offset":2188,"ancestor_record_offset":496,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3197,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":496,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3197,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":496,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3197,"record_offset":732,"parent_record_offset":692,"ancestor_record_offset":496,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3197,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":496,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3197,"record_offset":3056,"parent_record_offset":2188,"ancestor_record_offset":496,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3197,"record_offset":2652,"parent_record_offset":2188,"ancestor_record_offset":496,"inlinee":23285,"source_line":1276,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3197,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":496,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3197,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":496,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3197,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":496,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3197,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":496,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3197,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":496,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3197,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":496,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_changed_ticks_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3197,"record_offset":3020,"parent_record_offset":2676,"ancestor_record_offset":496,"inlinee":23284,"source_line":341,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3197,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":496,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3197,"record_offset":2624,"parent_record_offset":2188,"ancestor_record_offset":496,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3197,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":496,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3197,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":496,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3197,"record_offset":2440,"parent_record_offset":2408,"ancestor_record_offset":496,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3197,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":496,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3197,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":496,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3197,"record_offset":1868,"parent_record_offset":1840,"ancestor_record_offset":496,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3197,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":496,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3197,"record_offset":804,"parent_record_offset":756,"ancestor_record_offset":496,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3197,"record_offset":2320,"parent_record_offset":2232,"ancestor_record_offset":496,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3197,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":496,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3197,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":496,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3197,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":496,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3197,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":496,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3197,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":496,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3197,"record_offset":1948,"parent_record_offset":1868,"ancestor_record_offset":496,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3197,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":496,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3197,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":496,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3197,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":496,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3197,"record_offset":1384,"parent_record_offset":1276,"ancestor_record_offset":496,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3197,"record_offset":2596,"parent_record_offset":2188,"ancestor_record_offset":496,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3197,"record_offset":868,"parent_record_offset":756,"ancestor_record_offset":496,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3197,"record_offset":1056,"parent_record_offset":756,"ancestor_record_offset":496,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3197,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":496,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3197,"record_offset":2944,"parent_record_offset":2912,"ancestor_record_offset":496,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3197,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":496,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3197,"record_offset":904,"parent_record_offset":868,"ancestor_record_offset":496,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3197,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":496,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3197,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":496,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3197,"record_offset":2024,"parent_record_offset":1972,"ancestor_record_offset":496,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3197,"record_offset":2408,"parent_record_offset":2188,"ancestor_record_offset":496,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3197,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":496,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3197,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":496,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3197,"record_offset":2468,"parent_record_offset":2440,"ancestor_record_offset":496,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3197,"record_offset":3664,"parent_record_offset":3640,"ancestor_record_offset":496,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3197,"record_offset":3640,"parent_record_offset":3612,"ancestor_record_offset":496,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3197,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":496,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03197.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3197,"record_offset":3956,"parent_record_offset":3924,"ancestor_record_offset":496,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03197.jsonl.gz"}
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
// 完整结构及原始类型记录: edit_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: edit_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: edit_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>

// ------------------------------------------------------------------
// 函数: system_roof::edit_roof::edit_roof
// 信息 ID: c9dfec03bb709b82704f1a30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1dc3c96ef7aaa87058952b28: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > @ 0x12d384e -> 0x1988f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> @ 0x12df75a -> 0x1988f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
