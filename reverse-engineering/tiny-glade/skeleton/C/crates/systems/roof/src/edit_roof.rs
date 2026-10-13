// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: edit_roof.rs.assembly.json.gz

// FUNCTION system_roof::edit_roof::edit_roof::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3780:1920 -> host 3780:268 / rva_2827a90_len_c7f; exact_symbolizer_function_frame
//   Function-frame RVA span [0x28286e1, 0x282870f), 9 instructions
//
// FUNCTION system_roof::edit_roof::edit_roof
// Candidate interfaces: 8f96330335713f64e26c5e5a
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3780:268 -> rva_2827a90_len_c7f
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_2827a90_len_c7f: RVA 0x2827a90, 3199 PDB bytes; 700 instructions
// Evidence: _assembly/roof-v1/C/pack-002/rva_2827a90_len_c7f.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3780:268: system_roof::edit_roof::edit_roof
// Frames: F1
// 0x142827a90  4157                           pushq	%r15
// 0x142827a92  4156                           pushq	%r14
// 0x142827a94  4155                           pushq	%r13
// 0x142827a96  4154                           pushq	%r12
// 0x142827a98  56                             pushq	%rsi
// 0x142827a99  57                             pushq	%rdi
// 0x142827a9a  55                             pushq	%rbp
// 0x142827a9b  53                             pushq	%rbx
// 0x142827a9c  4881ecd8010000                 subq	$0x1d8, %rsp            # imm = 0x1D8
// 0x142827aa3  440f29bc24c0010000             movaps	%xmm15, 0x1c0(%rsp)
// 0x142827aac  440f29b424b0010000             movaps	%xmm14, 0x1b0(%rsp)
// 0x142827ab5  440f29ac24a0010000             movaps	%xmm13, 0x1a0(%rsp)
// 0x142827abe  440f29a42490010000             movaps	%xmm12, 0x190(%rsp)
// 0x142827ac7  440f299c2480010000             movaps	%xmm11, 0x180(%rsp)
// 0x142827ad0  440f29942470010000             movaps	%xmm10, 0x170(%rsp)
// 0x142827ad9  440f298c2460010000             movaps	%xmm9, 0x160(%rsp)
// 0x142827ae2  440f29842450010000             movaps	%xmm8, 0x150(%rsp)
// 0x142827aeb  0f29bc2440010000               movaps	%xmm7, 0x140(%rsp)
// 0x142827af3  0f29b42430010000               movaps	%xmm6, 0x130(%rsp)
// 0x142827afb  4c8b942450020000               movq	0x250(%rsp), %r10
// Frames: F2 -> F3 -> F4
// 0x142827b03  498b28                         movq	(%r8), %rbp
// Frames: F5 -> F3 -> F4
// 0x142827b06  4d8b5808                       movq	0x8(%r8), %r11
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x142827b0a  488b5d00                       movq	(%rbp), %rbx
// 0x142827b0e  31ff                           xorl	%edi, %edi
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x142827b10  4989d8                         movq	%rbx, %r8
// 0x142827b13  4d2b4318                       subq	0x18(%r11), %r8
// 0x142827b17  4c0f42c7                       cmovbq	%rdi, %r8
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x142827b1b  492b5b38                       subq	0x38(%r11), %rbx
// 0x142827b1f  480f42df                       cmovbq	%rdi, %rbx
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142827b23  4c89c0                         movq	%r8, %rax
// 0x142827b26  48c1e006                       shlq	$0x6, %rax
// 0x142827b2a  49034308                       addq	0x8(%r11), %rax
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142827b2e  4d8b7b10                       movq	0x10(%r11), %r15
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142827b32  4d29c7                         subq	%r8, %r15
// 0x142827b35  4c0f42ff                       cmovbq	%rdi, %r15
// 0x142827b39  488bb42448020000               movq	0x248(%rsp), %rsi
// 0x142827b41  41be08000000                   movl	$0x8, %r14d
// 0x142827b47  490f42c6                       cmovbq	%r14, %rax
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142827b4b  4989d8                         movq	%rbx, %r8
// 0x142827b4e  49c1e006                       shlq	$0x6, %r8
// 0x142827b52  4d034328                       addq	0x28(%r11), %r8
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142827b56  4d8b6330                       movq	0x30(%r11), %r12
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142827b5a  4929dc                         subq	%rbx, %r12
// 0x142827b5d  4d0f42c6                       cmovbq	%r14, %r8
// 0x142827b61  4c0f42e7                       cmovbq	%rdi, %r12
// Frames: F20 -> F7 -> F8 -> F3 -> F4
// 0x142827b65  4d8b5b40                       movq	0x40(%r11), %r11
// 0x142827b69  4d29e3                         subq	%r12, %r11
// 0x142827b6c  4d29fb                         subq	%r15, %r11
// 0x142827b6f  4c895d00                       movq	%r11, (%rbp)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F4
// 0x142827b73  49c1e706                       shlq	$0x6, %r15
// 0x142827b77  4901c7                         addq	%rax, %r15
// 0x142827b7a  4c89bc2400010000               movq	%r15, 0x100(%rsp)
// 0x142827b82  49c1e406                       shlq	$0x6, %r12
// 0x142827b86  4d01c4                         addq	%r8, %r12
// 0x142827b89  4c89a424f8000000               movq	%r12, 0xf8(%rsp)
// 0x142827b91  488b09                         movq	(%rcx), %rcx
// 0x142827b94  48898c2420010000               movq	%rcx, 0x120(%rsp)
// 0x142827b9c  488b0a                         movq	(%rdx), %rcx
// 0x142827b9f  4c8b5a08                       movq	0x8(%rdx), %r11
// 0x142827ba3  4c899c2418010000               movq	%r11, 0x118(%rsp)
// 0x142827bab  8b5a14                         movl	0x14(%rdx), %ebx
// 0x142827bae  8b561c                         movl	0x1c(%rsi), %edx
// 0x142827bb1  89942490000000                 movl	%edx, 0x90(%rsp)
// 0x142827bb8  488b16                         movq	(%rsi), %rdx
// 0x142827bbb  4c8b5e10                       movq	0x10(%rsi), %r11
// 0x142827bbf  4c899c24f0000000               movq	%r11, 0xf0(%rsp)
// 0x142827bc7  4889ce                         movq	%rcx, %rsi
// 0x142827bca  418b4a1c                       movl	0x1c(%r10), %ecx
// 0x142827bce  894c2434                       movl	%ecx, 0x34(%rsp)
// 0x142827bd2  4d8b1a                         movq	(%r10), %r11
// 0x142827bd5  498b4a10                       movq	0x10(%r10), %rcx
// 0x142827bd9  48894c2470                     movq	%rcx, 0x70(%rsp)
// 0x142827bde  488b8c2440020000               movq	0x240(%rsp), %rcx
// 0x142827be6  448b511c                       movl	0x1c(%rcx), %r10d
// 0x142827bea  44899424a0000000               movl	%r10d, 0xa0(%rsp)
// 0x142827bf2  4c8b29                         movq	(%rcx), %r13
// 0x142827bf5  488b4910                       movq	0x10(%rcx), %rcx
// 0x142827bf9  48898c2408010000               movq	%rcx, 0x108(%rsp)
// 0x142827c01  418b491c                       movl	0x1c(%r9), %ecx
// 0x142827c05  894c2440                       movl	%ecx, 0x40(%rsp)
// 0x142827c09  498b09                         movq	(%r9), %rcx
// 0x142827c0c  4d8b4910                       movq	0x10(%r9), %r9
// 0x142827c10  4c898c24b0000000               movq	%r9, 0xb0(%rsp)
// 0x142827c18  450f57db                       xorps	%xmm11, %xmm11
// 0x142827c1c  f3440f10250fae6f00             movss	0x6fae0f(%rip), %xmm12  # 0x142f22a34
// 0x142827c25  f3440f102df6cd6d00             movss	0x6dcdf6(%rip), %xmm13  # 0x142f04a24
// 0x142827c2e  48899424b8000000               movq	%rdx, 0xb8(%rsp)
// 0x142827c36  488d5220                       leaq	0x20(%rdx), %rdx
// 0x142827c3a  48899424d8000000               movq	%rdx, 0xd8(%rsp)
// 0x142827c42  4c895c2438                     movq	%r11, 0x38(%rsp)
// 0x142827c47  498d5320                       leaq	0x20(%r11), %rdx
// 0x142827c4b  4889542468                     movq	%rdx, 0x68(%rsp)
// 0x142827c50  498d5520                       leaq	0x20(%r13), %rdx
// 0x142827c54  48899424e8000000               movq	%rdx, 0xe8(%rsp)
// 0x142827c5c  48898c2480000000               movq	%rcx, 0x80(%rsp)
// 0x142827c64  4883c120                       addq	$0x20, %rcx
// 0x142827c68  48898c24e0000000               movq	%rcx, 0xe0(%rsp)
// 0x142827c70  895c2444                       movl	%ebx, 0x44(%rsp)
// 0x142827c74  4c89ac2410010000               movq	%r13, 0x110(%rsp)
// 0x142827c7c  eb66                           jmp	0x142827ce4 <.text+0x2826ce4>
// 0x142827c7e  6690                           nop
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30
// 0x142827c80  488b8c24e0000000               movq	0xe0(%rsp), %rcx
// 0x142827c88  e823765400                     callq	0x142d6f2b0 <.text+0x2d6e2b0>
// 0x142827c8d  4489e0                         movl	%r12d, %eax
// Frames: F31 -> F32 -> F33 -> F34 -> F35 -> F26 -> F27 -> F28 -> F29 -> F30
// 0x142827c90  4c8b842480000000               movq	0x80(%rsp), %r8
// 0x142827c98  498b4828                       movq	0x28(%r8), %rcx
// Frames: F36 -> F35 -> F26 -> F27 -> F28 -> F29 -> F30
// 0x142827c9c  4889da                         movq	%rbx, %rdx
// 0x142827c9f  48c1e205                       shlq	$0x5, %rdx
// Frames: F37 -> F38 -> F26 -> F27 -> F28 -> F29 -> F30
// 0x142827ca3  4c893c11                       movq	%r15, (%rcx,%rdx)
// 0x142827ca7  4c8b8c24d0000000               movq	0xd0(%rsp), %r9
// 0x142827caf  4c894c1108                     movq	%r9, 0x8(%rcx,%rdx)
// 0x142827cb4  4c8b8c24c8000000               movq	0xc8(%rsp), %r9
// 0x142827cbc  4c894c1110                     movq	%r9, 0x10(%rcx,%rdx)
// 0x142827cc1  4488741118                     movb	%r14b, 0x18(%rcx,%rdx)
// 0x142827cc6  88441119                       movb	%al, 0x19(%rcx,%rdx)
// Frames: F39 -> F26 -> F27 -> F28 -> F29 -> F30
// 0x142827cca  48ffc3                         incq	%rbx
// 0x142827ccd  49895830                       movq	%rbx, 0x30(%r8)
// Frames: F40 -> F28 -> F29 -> F30
// 0x142827cd1  49ff4040                       incq	0x40(%r8)
// 0x142827cd5  4889f8                         movq	%rdi, %rax
// 0x142827cd8  4c8b8424c0000000               movq	0xc0(%rsp), %r8
// 0x142827ce0  8b5c2444                       movl	0x44(%rsp), %ebx
// Frames: F41 -> F42 -> F43 -> F44 -> F45 -> F4
// 0x142827ce4  4885c0                         testq	%rax, %rax
// 0x142827ce7  7427                           je	0x142827d10 <.text+0x2826d10>
// 0x142827ce9  483b842400010000               cmpq	0x100(%rsp), %rax
// 0x142827cf1  741d                           je	0x142827d10 <.text+0x2826d10>
// 0x142827cf3  488d4840                       leaq	0x40(%rax), %rcx
// 0x142827cf7  48898c2488000000               movq	%rcx, 0x88(%rsp)
// 0x142827cff  4c89c7                         movq	%r8, %rdi
// 0x142827d02  4989c7                         movq	%rax, %r15
// 0x142827d05  eb33                           jmp	0x142827d3a <.text+0x2826d3a>
// 0x142827d07  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F41 -> F46 -> F47 -> F43 -> F44 -> F45 -> F4
// 0x142827d10  4d85c0                         testq	%r8, %r8
// 0x142827d13  0f8413090000                   je	0x14282862c <.text+0x282762c>
// 0x142827d19  4c3b8424f8000000               cmpq	0xf8(%rsp), %r8
// 0x142827d21  0f8405090000                   je	0x14282862c <.text+0x282762c>
// Frames: F48 -> F49 -> F46 -> F47 -> F43 -> F44 -> F45 -> F4
// 0x142827d27  498d7840                       leaq	0x40(%r8), %rdi
// 0x142827d2b  48c784248800000000000000       movq	$0x0, 0x88(%rsp)
// 0x142827d37  4d89c7                         movq	%r8, %r15
// Frames: F50 -> F45 -> F4
// 0x142827d3a  48ff4500                       incq	(%rbp)
// Frames: F51
// 0x142827d3e  498b5720                       movq	0x20(%r15), %rdx
// 0x142827d42  488b8c2420010000               movq	0x120(%rsp), %rcx
// 0x142827d4a  48899424d0000000               movq	%rdx, 0xd0(%rsp)
// 0x142827d52  e8d99037fe                     callq	0x140ba0e30 <.text+0xb9fe30>
// 0x142827d57  4989c2                         movq	%rax, %r10
// 0x142827d5a  b9ffffffff                     movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
// Frames: F52 -> F53
// 0x142827d5f  4821c8                         andq	%rcx, %rax
// 0x142827d62  0f8479090000                   je	0x1428286e1 <.text+0x28276e1>
// Frames: F54 -> F55 -> F56 -> F57 -> F58
// 0x142827d68  4831c8                         xorq	%rcx, %rax
// 0x142827d6b  4c8b842418010000               movq	0x118(%rsp), %r8
// Frames: F59 -> F15 -> F54 -> F55 -> F56 -> F57 -> F58
// 0x142827d73  493b4010                       cmpq	0x10(%r8), %rax
// 0x142827d77  4c8b8c2428010000               movq	0x128(%rsp), %r9
// Frames: F60 -> F61 -> F55 -> F56 -> F57 -> F58
// 0x142827d7f  0f8313090000                   jae	0x142828698 <.text+0x2827698>
// Frames: F62 -> F63 -> F64 -> F61 -> F55 -> F56 -> F57 -> F58
// 0x142827d85  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x142827d89  48c1e003                       shlq	$0x3, %rax
// 0x142827d8d  49034008                       addq	0x8(%r8), %rax
// 0x142827d91  4c89d1                         movq	%r10, %rcx
// 0x142827d94  48c1e920                       shrq	$0x20, %rcx
// 0x142827d98  394810                         cmpl	%ecx, 0x10(%rax)
// Frames: F65 -> F61 -> F55 -> F56 -> F57 -> F58
// 0x142827d9b  0f85f7080000                   jne	0x142828698 <.text+0x2827698>
// Frames: F66 -> F67 -> F57 -> F58
// 0x142827da1  833800                         cmpl	$0x0, (%rax)
// 0x142827da4  0f84ee080000                   je	0x142828698 <.text+0x2827698>
// Frames: F68 -> F69 -> F57 -> F58
// 0x142827daa  4889bc24c0000000               movq	%rdi, 0xc0(%rsp)
// 0x142827db2  4c8b4808                       movq	0x8(%rax), %r9
// 0x142827db6  4489c9                         movl	%r9d, %ecx
// Frames: F70 -> F69 -> F57 -> F58
// 0x142827db9  483b4e38                       cmpq	0x38(%rsi), %rcx
// Frames: F71 -> F72 -> F69 -> F57 -> F58
// 0x142827dbd  0f83e7080000                   jae	0x1428286aa <.text+0x28276aa>
// Frames: F73 -> F74 -> F75 -> F71 -> F72 -> F69 -> F57 -> F58
// 0x142827dc3  488b5628                       movq	0x28(%rsi), %rdx
// Frames: F21 -> F73 -> F74 -> F75 -> F71 -> F72 -> F69 -> F57 -> F58
// 0x142827dc7  c1e906                         shrl	$0x6, %ecx
// Frames: F76 -> F74 -> F75 -> F71 -> F72 -> F69 -> F57 -> F58
// 0x142827dca  488b0cca                       movq	(%rdx,%rcx,8), %rcx
// Frames: F74 -> F75 -> F71 -> F72 -> F69 -> F57 -> F58
// 0x142827dce  4c0fa3c9                       btq	%r9, %rcx
// Frames: F77 -> F57 -> F58
// 0x142827dd2  0f83d2080000                   jae	0x1428286aa <.text+0x28276aa>
// Frames: F31 -> F32 -> F33 -> F78 -> F17 -> F18 -> F79 -> F80 -> F57 -> F58
// 0x142827dd8  448b6804                       movl	0x4(%rax), %r13d
// 0x142827ddc  49c1e920                       shrq	$0x20, %r9
// 0x142827de0  498b80a0010000                 movq	0x1a0(%r8), %rax
// Frames: F81 -> F82 -> F79 -> F80 -> F57 -> F58
// 0x142827de7  4b8d0cc9                       leaq	(%r9,%r9,8), %rcx
// Frames: F83 -> F57 -> F58
// 0x142827deb  488b9610010000                 movq	0x110(%rsi), %rdx
// Frames: F84 -> F85 -> F86 -> F87 -> F83 -> F57 -> F58
// 0x142827df2  4c8b44c818                     movq	0x18(%rax,%rcx,8), %r8
// 0x142827df7  488b44c838                     movq	0x38(%rax,%rcx,8), %rax
// Frames: F88 -> F89 -> F90 -> F91 -> F84 -> F85 -> F86 -> F87 -> F83 -> F57 -> F58
// 0x142827dfc  488b04d0                       movq	(%rax,%rdx,8), %rax
// Frames: F92 -> F93 -> F94 -> F84 -> F85 -> F86 -> F87 -> F83 -> F57 -> F58
// 0x142827e00  48f7d0                         notq	%rax
// Frames: F95 -> F96 -> F93 -> F94 -> F84 -> F85 -> F86 -> F87 -> F83 -> F57 -> F58
// 0x142827e03  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x142827e07  48c1e004                       shlq	$0x4, %rax
// Frames: F97 -> F98 -> F99 -> F100 -> F101 -> F102 -> F57 -> F58
// 0x142827e0b  41f7d5                         notl	%r13d
// Frames: F21 -> F103 -> F99 -> F100 -> F101 -> F102 -> F57 -> F58
// 0x142827e0e  4d6bf558                       imulq	$0x58, %r13, %r14
// 0x142827e12  4d03740010                     addq	0x10(%r8,%rax), %r14
// Frames: F104 -> F105 -> F106 -> F87 -> F83 -> F57 -> F58
// 0x142827e17  4d8b640028                     movq	0x28(%r8,%rax), %r12
// Frames: F21 -> F107 -> F108 -> F100 -> F101 -> F102 -> F57 -> F58
// 0x142827e1c  4a8d0cad00000000               leaq	(,%r13,4), %rcx
// 0x142827e24  49034c0020                     addq	0x20(%r8,%rax), %rcx
// Frames: F109
// 0x142827e29  418b07                         movl	(%r15), %eax
// 0x142827e2c  0f57f6                         xorps	%xmm6, %xmm6
// 0x142827e2f  0f57ff                         xorps	%xmm7, %xmm7
// 0x142827e32  85c0                           testl	%eax, %eax
// 0x142827e34  742a                           je	0x142827e60 <.text+0x2826e60>
// 0x142827e36  83f801                         cmpl	$0x1, %eax
// 0x142827e39  7565                           jne	0x142827ea0 <.text+0x2826ea0>
// Frames: F110
// 0x142827e3b  f3410f107f04                   movss	0x4(%r15), %xmm7
// Frames: F111 -> F112
// 0x142827e41  0f57c0                         xorps	%xmm0, %xmm0
// 0x142827e44  f30f5fc7                       maxss	%xmm7, %xmm0
// Frames: F113 -> F112
// 0x142827e48  410f28cd                       movaps	%xmm13, %xmm1
// 0x142827e4c  f30f5dc8                       minss	%xmm0, %xmm1
// Frames: F114
// 0x142827e50  f3410f5c7e2c                   subss	0x2c(%r14), %xmm7
// Frames: F115 -> F116 -> F112
// 0x142827e56  43891cac                       movl	%ebx, (%r12,%r13,4)
// Frames: F112
// 0x142827e5a  f3410f114e2c                   movss	%xmm1, 0x2c(%r14)
// Frames: F117
// 0x142827e60  f3450f107630                   movss	0x30(%r14), %xmm14
// Frames: F118
// 0x142827e66  418b4708                       movl	0x8(%r15), %eax
// 0x142827e6a  85c0                           testl	%eax, %eax
// 0x142827e6c  7469                           je	0x142827ed7 <.text+0x2826ed7>
// 0x142827e6e  83f802                         cmpl	$0x2, %eax
// 0x142827e71  756d                           jne	0x142827ee0 <.text+0x2826ee0>
// Frames: F119
// 0x142827e73  f3410f10770c                   movss	0xc(%r15), %xmm6
// Frames: F115 -> F116 -> F120
// 0x142827e79  43891cac                       movl	%ebx, (%r12,%r13,4)
// 0x142827e7d  f3410f104630                   movss	0x30(%r14), %xmm0
// Frames: F120
// 0x142827e83  f30f58c6                       addss	%xmm6, %xmm0
// Frames: F111 -> F121
// 0x142827e87  410f28cc                       movaps	%xmm12, %xmm1
// 0x142827e8b  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F113 -> F121
// 0x142827e8f  450f28fd                       movaps	%xmm13, %xmm15
// 0x142827e93  f3440f5df9                     minss	%xmm1, %xmm15
// Frames: F121
// 0x142827e98  f3450f117e30                   movss	%xmm15, 0x30(%r14)
// 0x142827e9e  eb65                           jmp	0x142827f05 <.text+0x2826f05>
// Frames: F122
// 0x142827ea0  f3410f107f04                   movss	0x4(%r15), %xmm7
// Frames: F115 -> F116 -> F123
// 0x142827ea6  43891cac                       movl	%ebx, (%r12,%r13,4)
// 0x142827eaa  f3410f10462c                   movss	0x2c(%r14), %xmm0
// Frames: F123
// 0x142827eb0  f30f58c7                       addss	%xmm7, %xmm0
// Frames: F111 -> F124
// 0x142827eb4  0f57c9                         xorps	%xmm1, %xmm1
// 0x142827eb7  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F113 -> F124
// 0x142827ebb  410f28c5                       movaps	%xmm13, %xmm0
// 0x142827ebf  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F124
// 0x142827ec3  f3410f11462c                   movss	%xmm0, 0x2c(%r14)
// Frames: F117
// 0x142827ec9  f3450f107630                   movss	0x30(%r14), %xmm14
// Frames: F118
// 0x142827ecf  418b4708                       movl	0x8(%r15), %eax
// 0x142827ed3  85c0                           testl	%eax, %eax
// 0x142827ed5  7597                           jne	0x142827e6e <.text+0x2826e6e>
// 0x142827ed7  31ff                           xorl	%edi, %edi
// 0x142827ed9  450f28fe                       movaps	%xmm14, %xmm15
// 0x142827edd  eb29                           jmp	0x142827f08 <.text+0x2826f08>
// 0x142827edf  90                             nop
// Frames: F125
// 0x142827ee0  f3410f10770c                   movss	0xc(%r15), %xmm6
// Frames: F111 -> F126
// 0x142827ee6  0f57c0                         xorps	%xmm0, %xmm0
// 0x142827ee9  f30f5fc6                       maxss	%xmm6, %xmm0
// Frames: F113 -> F126
// 0x142827eed  450f28fd                       movaps	%xmm13, %xmm15
// 0x142827ef1  f3440f5df8                     minss	%xmm0, %xmm15
// Frames: F115 -> F116 -> F126
// 0x142827ef6  43891cac                       movl	%ebx, (%r12,%r13,4)
// Frames: F126
// 0x142827efa  f3450f117e30                   movss	%xmm15, 0x30(%r14)
// Frames: F127
// 0x142827f00  f3410f5cf6                     subss	%xmm14, %xmm6
// 0x142827f05  40b701                         movb	$0x1, %dil
// Frames: F128
// 0x142827f08  418b4728                       movl	0x28(%r15), %eax
// 0x142827f0c  450f57c0                       xorps	%xmm8, %xmm8
// 0x142827f10  450f57d2                       xorps	%xmm10, %xmm10
// 0x142827f14  85c0                           testl	%eax, %eax
// 0x142827f16  0f84d5000000                   je	0x142827ff1 <.text+0x2826ff1>
// 0x142827f1c  83f802                         cmpl	$0x2, %eax
// 0x142827f1f  755f                           jne	0x142827f80 <.text+0x2826f80>
// Frames: F129
// 0x142827f21  f2410f10472c                   movsd	0x2c(%r15), %xmm0
// Frames: F115 -> F116 -> F130
// 0x142827f27  43891cac                       movl	%ebx, (%r12,%r13,4)
// Frames: F131 -> F132 -> F130
// 0x142827f2b  f2410f104e24                   movsd	0x24(%r14), %xmm1
// 0x142827f31  0f58c8                         addps	%xmm0, %xmm1
// Frames: F133 -> F134 -> F135 -> F136
// 0x142827f34  0f28d1                         movaps	%xmm1, %xmm2
// 0x142827f37  0f59d1                         mulps	%xmm1, %xmm2
// 0x142827f3a  f30f16da                       movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
// 0x142827f3e  f30f58da                       addss	%xmm2, %xmm3
// Frames: F137 -> F136
// 0x142827f42  31c0                           xorl	%eax, %eax
// 0x142827f44  410f2edd                       ucomiss	%xmm13, %xmm3
// 0x142827f48  0f97c0                         seta	%al
// 0x142827f4b  0f57d2                         xorps	%xmm2, %xmm2
// 0x142827f4e  f30f51d3                       sqrtss	%xmm3, %xmm2
// 0x142827f52  f30f12d2                       movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
// 0x142827f56  0f28d9                         movaps	%xmm1, %xmm3
// 0x142827f59  0f5eda                         divps	%xmm2, %xmm3
// 0x142827f5c  660f6ed0                       movd	%eax, %xmm2
// 0x142827f60  660f70d200                     pshufd	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
// 0x142827f65  660f72f21f                     pslld	$0x1f, %xmm2
// 0x142827f6a  660f72e21f                     psrad	$0x1f, %xmm2
// 0x142827f6f  0f54da                         andps	%xmm2, %xmm3
// 0x142827f72  0f55d1                         andnps	%xmm1, %xmm2
// 0x142827f75  0f56d3                         orps	%xmm3, %xmm2
// Frames: F136
// 0x142827f78  410f135624                     movlps	%xmm2, 0x24(%r14)
// 0x142827f7d  eb5e                           jmp	0x142827fdd <.text+0x2826fdd>
// 0x142827f7f  90                             nop
// Frames: F138
// 0x142827f80  f2410f10472c                   movsd	0x2c(%r15), %xmm0
// Frames: F139
// 0x142827f86  f2410f104e24                   movsd	0x24(%r14), %xmm1
// Frames: F133 -> F134 -> F135 -> F139
// 0x142827f8c  0f28d1                         movaps	%xmm1, %xmm2
// 0x142827f8f  0f59d1                         mulps	%xmm1, %xmm2
// 0x142827f92  f30f16da                       movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
// 0x142827f96  f30f58da                       addss	%xmm2, %xmm3
// Frames: F137 -> F139
// 0x142827f9a  31c0                           xorl	%eax, %eax
// 0x142827f9c  410f2edd                       ucomiss	%xmm13, %xmm3
// 0x142827fa0  0f97c0                         seta	%al
// 0x142827fa3  0f57d2                         xorps	%xmm2, %xmm2
// 0x142827fa6  f30f51d3                       sqrtss	%xmm3, %xmm2
// 0x142827faa  f30f12d2                       movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
// 0x142827fae  0f28d9                         movaps	%xmm1, %xmm3
// 0x142827fb1  0f5eda                         divps	%xmm2, %xmm3
// 0x142827fb4  660f6ed0                       movd	%eax, %xmm2
// 0x142827fb8  660f70d200                     pshufd	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
// 0x142827fbd  660f72f21f                     pslld	$0x1f, %xmm2
// 0x142827fc2  660f72e21f                     psrad	$0x1f, %xmm2
// 0x142827fc7  0f54da                         andps	%xmm2, %xmm3
// 0x142827fca  0f55d1                         andnps	%xmm1, %xmm2
// 0x142827fcd  0f56d3                         orps	%xmm3, %xmm2
// Frames: F115 -> F116 -> F140
// 0x142827fd0  43891cac                       movl	%ebx, (%r12,%r13,4)
// Frames: F140
// 0x142827fd4  f2410f114624                   movsd	%xmm0, 0x24(%r14)
// Frames: F141 -> F142 -> F143
// 0x142827fda  0f5cc2                         subps	%xmm2, %xmm0
// Frames: F133 -> F144 -> F30
// 0x142827fdd  0f59c0                         mulps	%xmm0, %xmm0
// 0x142827fe0  f30f16c8                       movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
// 0x142827fe4  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F145 -> F146 -> F147 -> F144 -> F30
// 0x142827fe8  450f57d2                       xorps	%xmm10, %xmm10
// 0x142827fec  f3440f51d1                     sqrtss	%xmm1, %xmm10
// Frames: F148
// 0x142827ff1  418b4718                       movl	0x18(%r15), %eax
// 0x142827ff5  85c0                           testl	%eax, %eax
// 0x142827ff7  745d                           je	0x142828056 <.text+0x2827056>
// 0x142827ff9  83f802                         cmpl	$0x2, %eax
// 0x142827ffc  7532                           jne	0x142828030 <.text+0x2827030>
// Frames: F149
// 0x142827ffe  f3450f10471c                   movss	0x1c(%r15), %xmm8
// Frames: F115 -> F116 -> F150
// 0x142828004  43891cac                       movl	%ebx, (%r12,%r13,4)
// 0x142828008  f3410f104638                   movss	0x38(%r14), %xmm0
// Frames: F150
// 0x14282800e  f3410f58c0                     addss	%xmm8, %xmm0
// Frames: F111 -> F151
// 0x142828013  0f57c9                         xorps	%xmm1, %xmm1
// 0x142828016  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F113 -> F151
// 0x14282801a  410f28c5                       movaps	%xmm13, %xmm0
// 0x14282801e  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F151
// 0x142828022  f3410f114638                   movss	%xmm0, 0x38(%r14)
// 0x142828028  eb2c                           jmp	0x142828056 <.text+0x2827056>
// 0x14282802a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F152
// 0x142828030  f3450f10471c                   movss	0x1c(%r15), %xmm8
// Frames: F111 -> F153
// 0x142828036  0f57c0                         xorps	%xmm0, %xmm0
// 0x142828039  f3410f5f4638                   maxss	0x38(%r14), %xmm0
// Frames: F113 -> F153
// 0x14282803f  410f28cd                       movaps	%xmm13, %xmm1
// 0x142828043  f30f5dc8                       minss	%xmm0, %xmm1
// Frames: F115 -> F116 -> F154
// 0x142828047  43891cac                       movl	%ebx, (%r12,%r13,4)
// Frames: F154
// 0x14282804b  f3450f114638                   movss	%xmm8, 0x38(%r14)
// Frames: F155
// 0x142828051  f3440f5cc1                     subss	%xmm1, %xmm8
// Frames: F156
// 0x142828056  4c899424c8000000               movq	%r10, 0xc8(%rsp)
// 0x14282805e  48898c2428010000               movq	%rcx, 0x128(%rsp)
// 0x142828066  4c89f1                         movq	%r14, %rcx
// 0x142828069  e8320e3cfe                     callq	0x140be8ea0 <.text+0xbe7ea0>
// 0x14282806e  4c89f9                         movq	%r15, %rcx
// 0x142828071  4189c7                         movl	%eax, %r15d
// 0x142828074  48894c2478                     movq	%rcx, 0x78(%rsp)
// Frames: F157
// 0x142828079  8b4110                         movl	0x10(%rcx), %eax
// 0x14282807c  85c0                           testl	%eax, %eax
// 0x14282807e  7450                           je	0x1428280d0 <.text+0x28270d0>
// 0x142828080  83f802                         cmpl	$0x2, %eax
// 0x142828083  755b                           jne	0x1428280e0 <.text+0x28270e0>
// Frames: F158
// 0x142828085  488b442478                     movq	0x78(%rsp), %rax
// 0x14282808a  f3440f104814                   movss	0x14(%rax), %xmm9
// Frames: F115 -> F116 -> F159
// 0x142828090  43891cac                       movl	%ebx, (%r12,%r13,4)
// 0x142828094  f3410f104634                   movss	0x34(%r14), %xmm0
// Frames: F159
// 0x14282809a  f3410f58c1                     addss	%xmm9, %xmm0
// Frames: F111 -> F160
// 0x14282809f  0f57c9                         xorps	%xmm1, %xmm1
// 0x1428280a2  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F113 -> F160
// 0x1428280a6  410f28c5                       movaps	%xmm13, %xmm0
// 0x1428280aa  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F160
// 0x1428280ae  f3410f114634                   movss	%xmm0, 0x34(%r14)
// Frames: F161
// 0x1428280b4  4c89f1                         movq	%r14, %rcx
// 0x1428280b7  e8e40d3cfe                     callq	0x140be8ea0 <.text+0xbe7ea0>
// 0x1428280bc  4189c5                         movl	%eax, %r13d
// 0x1428280bf  eb72                           jmp	0x142828133 <.text+0x2827133>
// 0x1428280c1  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F157
// 0x1428280d0  450f57c9                       xorps	%xmm9, %xmm9
// 0x1428280d4  4589fd                         movl	%r15d, %r13d
// 0x1428280d7  eb5a                           jmp	0x142828133 <.text+0x2827133>
// 0x1428280d9  0f1f8000000000                 nopl	(%rax)
// Frames: F162
// 0x1428280e0  450f28de                       movaps	%xmm14, %xmm11
// 0x1428280e4  450f28f4                       movaps	%xmm12, %xmm14
// 0x1428280e8  450f28e2                       movaps	%xmm10, %xmm12
// 0x1428280ec  488b442478                     movq	0x78(%rsp), %rax
// 0x1428280f1  f3440f104814                   movss	0x14(%rax), %xmm9
// Frames: F111 -> F163
// 0x1428280f7  0f57c0                         xorps	%xmm0, %xmm0
// 0x1428280fa  f3410f5f4634                   maxss	0x34(%r14), %xmm0
// Frames: F113 -> F163
// 0x142828100  450f28d5                       movaps	%xmm13, %xmm10
// 0x142828104  f3440f5dd0                     minss	%xmm0, %xmm10
// Frames: F115 -> F116 -> F164
// 0x142828109  43891cac                       movl	%ebx, (%r12,%r13,4)
// Frames: F164
// 0x14282810d  f3450f114e34                   movss	%xmm9, 0x34(%r14)
// Frames: F165
// 0x142828113  4c89f1                         movq	%r14, %rcx
// 0x142828116  e8850d3cfe                     callq	0x140be8ea0 <.text+0xbe7ea0>
// 0x14282811b  4189c5                         movl	%eax, %r13d
// Frames: F166
// 0x14282811e  f3450f5cca                     subss	%xmm10, %xmm9
// 0x142828123  450f28d4                       movaps	%xmm12, %xmm10
// 0x142828127  450f28e6                       movaps	%xmm14, %xmm12
// 0x14282812b  450f28f3                       movaps	%xmm11, %xmm14
// 0x14282812f  450f57db                       xorps	%xmm11, %xmm11
// Frames: F167
// 0x142828133  410f2efb                       ucomiss	%xmm11, %xmm7
// 0x142828137  0f95c1                         setne	%cl
// 0x14282813a  450f2ec3                       ucomiss	%xmm11, %xmm8
// 0x14282813e  0f95c0                         setne	%al
// 0x142828141  4538ef                         cmpb	%r13b, %r15b
// 0x142828144  0f8596000000                   jne	0x1428281e0 <.text+0x28271e0>
// 0x14282814a  4008f9                         orb	%dil, %cl
// 0x14282814d  4420e9                         andb	%r13b, %cl
// 0x142828150  0f858a000000                   jne	0x1428281e0 <.text+0x28271e0>
// 0x142828156  4420e8                         andb	%r13b, %al
// 0x142828159  0f8581000000                   jne	0x1428281e0 <.text+0x28271e0>
// Frames: F168
// 0x14282815f  4584ff                         testb	%r15b, %r15b
// 0x142828162  0f84ef000000                   je	0x142828257 <.text+0x2827257>
// Frames: F169
// 0x142828168  4584ed                         testb	%r13b, %r13b
// 0x14282816b  0f8576010000                   jne	0x1428282e7 <.text+0x28272e7>
// Frames: F170
// 0x142828171  488d8c24a4000000               leaq	0xa4(%rsp), %rcx
// 0x142828179  4c89f2                         movq	%r14, %rdx
// 0x14282817c  e84f0d3cfe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// Frames: F171
// 0x142828181  8b8424ac000000                 movl	0xac(%rsp), %eax
// 0x142828188  488d4c2457                     leaq	0x57(%rsp), %rcx
// 0x14282818d  894108                         movl	%eax, 0x8(%rcx)
// 0x142828190  488b8424a4000000               movq	0xa4(%rsp), %rax
// 0x142828198  488901                         movq	%rax, (%rcx)
// 0x14282819b  8b442434                       movl	0x34(%rsp), %eax
// 0x14282819f  488b4c2470                     movq	0x70(%rsp), %rcx
// Frames: F172 -> F173 -> F29 -> F171
// 0x1428281a4  8901                           movl	%eax, (%rcx)
// 0x1428281a6  488b442438                     movq	0x38(%rsp), %rax
// Frames: F174 -> F26 -> F27 -> F175 -> F29 -> F171
// 0x1428281ab  4c8b7830                       movq	0x30(%rax), %r15
// Frames: F176 -> F175 -> F29 -> F171
// 0x1428281af  488b7840                       movq	0x40(%rax), %rdi
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F171
// 0x1428281b3  4c3b7820                       cmpq	0x20(%rax), %r15
// 0x1428281b7  750a                           jne	0x1428281c3 <.text+0x28271c3>
// Frames: F25 -> F26 -> F27 -> F175 -> F29 -> F171
// 0x1428281b9  488b4c2468                     movq	0x68(%rsp), %rcx
// 0x1428281be  e85d715400                     callq	0x142d6f320 <.text+0x2d6e320>
// Frames: F31 -> F32 -> F33 -> F34 -> F35 -> F26 -> F27 -> F175 -> F29 -> F171
// 0x1428281c3  488b542438                     movq	0x38(%rsp), %rdx
// 0x1428281c8  488b4228                       movq	0x28(%rdx), %rax
// Frames: F36 -> F35 -> F26 -> F27 -> F175 -> F29 -> F171
// 0x1428281cc  4b8d0cbf                       leaq	(%r15,%r15,4), %rcx
// Frames: F37 -> F38 -> F26 -> F27 -> F175 -> F29 -> F171
// 0x1428281d0  48893cc8                       movq	%rdi, (%rax,%rcx,8)
// 0x1428281d4  c644c8080b                     movb	$0xb, 0x8(%rax,%rcx,8)
// 0x1428281d9  e9ea000000                     jmp	0x1428282c8 <.text+0x28272c8>
// 0x1428281de  6690                           nop
// Frames: F178
// 0x1428281e0  4889f3                         movq	%rsi, %rbx
// 0x1428281e3  4889ee                         movq	%rbp, %rsi
// 0x1428281e6  498b3e                         movq	(%r14), %rdi
// 0x1428281e9  8b842490000000                 movl	0x90(%rsp), %eax
// 0x1428281f0  488b8c24f0000000               movq	0xf0(%rsp), %rcx
// Frames: F172 -> F173 -> F29 -> F178
// 0x1428281f8  8901                           movl	%eax, (%rcx)
// 0x1428281fa  488b8424b8000000               movq	0xb8(%rsp), %rax
// Frames: F174 -> F26 -> F27 -> F175 -> F29 -> F178
// 0x142828202  4c8b6030                       movq	0x30(%rax), %r12
// Frames: F176 -> F175 -> F29 -> F178
// 0x142828206  488b6840                       movq	0x40(%rax), %rbp
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F178
// 0x14282820a  4c3b6020                       cmpq	0x20(%rax), %r12
// 0x14282820e  750d                           jne	0x14282821d <.text+0x282721d>
// Frames: F25 -> F26 -> F27 -> F175 -> F29 -> F178
// 0x142828210  488b8c24d8000000               movq	0xd8(%rsp), %rcx
// 0x142828218  e833735400                     callq	0x142d6f550 <.text+0x2d6e550>
// Frames: F31 -> F32 -> F33 -> F34 -> F35 -> F26 -> F27 -> F175 -> F29 -> F178
// 0x14282821d  488b9424b8000000               movq	0xb8(%rsp), %rdx
// 0x142828225  488b4228                       movq	0x28(%rdx), %rax
// Frames: F36 -> F35 -> F26 -> F27 -> F175 -> F29 -> F178
// 0x142828229  4c89e1                         movq	%r12, %rcx
// 0x14282822c  48c1e104                       shlq	$0x4, %rcx
// Frames: F37 -> F38 -> F26 -> F27 -> F175 -> F29 -> F178
// 0x142828230  48892c08                       movq	%rbp, (%rax,%rcx)
// 0x142828234  48897c0808                     movq	%rdi, 0x8(%rax,%rcx)
// Frames: F39 -> F26 -> F27 -> F175 -> F29 -> F178
// 0x142828239  49ffc4                         incq	%r12
// 0x14282823c  4c896230                       movq	%r12, 0x30(%rdx)
// Frames: F40 -> F175 -> F29 -> F178
// 0x142828240  48ff4240                       incq	0x40(%rdx)
// 0x142828244  4889f5                         movq	%rsi, %rbp
// 0x142828247  4889de                         movq	%rbx, %rsi
// 0x14282824a  8b5c2444                       movl	0x44(%rsp), %ebx
// Frames: F168
// 0x14282824e  4584ff                         testb	%r15b, %r15b
// 0x142828251  0f8511ffffff                   jne	0x142828168 <.text+0x2827168>
// 0x142828257  4584ed                         testb	%r13b, %r13b
// 0x14282825a  0f8487000000                   je	0x1428282e7 <.text+0x28272e7>
// Frames: F179
// 0x142828260  488d8c24a4000000               leaq	0xa4(%rsp), %rcx
// 0x142828268  4c89f2                         movq	%r14, %rdx
// 0x14282826b  e8600c3cfe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// Frames: F180
// 0x142828270  8b8424ac000000                 movl	0xac(%rsp), %eax
// 0x142828277  488d4c2457                     leaq	0x57(%rsp), %rcx
// 0x14282827c  894108                         movl	%eax, 0x8(%rcx)
// 0x14282827f  488b8424a4000000               movq	0xa4(%rsp), %rax
// 0x142828287  488901                         movq	%rax, (%rcx)
// 0x14282828a  8b442434                       movl	0x34(%rsp), %eax
// 0x14282828e  488b4c2470                     movq	0x70(%rsp), %rcx
// Frames: F172 -> F173 -> F29 -> F180
// 0x142828293  8901                           movl	%eax, (%rcx)
// 0x142828295  488b442438                     movq	0x38(%rsp), %rax
// Frames: F174 -> F26 -> F27 -> F175 -> F29 -> F180
// 0x14282829a  4c8b7830                       movq	0x30(%rax), %r15
// Frames: F176 -> F175 -> F29 -> F180
// 0x14282829e  488b7840                       movq	0x40(%rax), %rdi
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F180
// 0x1428282a2  4c3b7820                       cmpq	0x20(%rax), %r15
// 0x1428282a6  750a                           jne	0x1428282b2 <.text+0x28272b2>
// Frames: F25 -> F26 -> F27 -> F175 -> F29 -> F180
// 0x1428282a8  488b4c2468                     movq	0x68(%rsp), %rcx
// 0x1428282ad  e86e705400                     callq	0x142d6f320 <.text+0x2d6e320>
// Frames: F31 -> F32 -> F33 -> F34 -> F35 -> F26 -> F27 -> F175 -> F29 -> F180
// 0x1428282b2  488b542438                     movq	0x38(%rsp), %rdx
// 0x1428282b7  488b4228                       movq	0x28(%rdx), %rax
// Frames: F36 -> F35 -> F26 -> F27 -> F175 -> F29 -> F180
// 0x1428282bb  4b8d0cbf                       leaq	(%r15,%r15,4), %rcx
// Frames: F37 -> F38 -> F26 -> F27 -> F175 -> F29 -> F180
// 0x1428282bf  48893cc8                       movq	%rdi, (%rax,%rcx,8)
// 0x1428282c3  c644c8080a                     movb	$0xa, 0x8(%rax,%rcx,8)
// Frames: F37 -> F38 -> F26 -> F27 -> F28 -> F29 -> F30
// 0x1428282c8  0f10442448                     movups	0x48(%rsp), %xmm0
// 0x1428282cd  0f104c2453                     movups	0x53(%rsp), %xmm1
// 0x1428282d2  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1428282d7  0f1144c809                     movups	%xmm0, 0x9(%rax,%rcx,8)
// Frames: F39 -> F26 -> F27 -> F28 -> F29 -> F30
// 0x1428282dc  49ffc7                         incq	%r15
// 0x1428282df  4c897a30                       movq	%r15, 0x30(%rdx)
// Frames: F40 -> F28 -> F29 -> F30
// 0x1428282e3  48ff4240                       incq	0x40(%rdx)
// Frames: F181
// 0x1428282e7  450f2ec3                       ucomiss	%xmm11, %xmm8
// 0x1428282eb  7533                           jne	0x142828320 <.text+0x2827320>
// 0x1428282ed  410f2efb                       ucomiss	%xmm11, %xmm7
// 0x1428282f1  752d                           jne	0x142828320 <.text+0x2827320>
// 0x1428282f3  410f2ef3                       ucomiss	%xmm11, %xmm6
// 0x1428282f7  7527                           jne	0x142828320 <.text+0x2827320>
// 0x1428282f9  450f2ecb                       ucomiss	%xmm11, %xmm9
// 0x1428282fd  7521                           jne	0x142828320 <.text+0x2827320>
// Frames: F182
// 0x1428282ff  450f2ed3                       ucomiss	%xmm11, %xmm10
// 0x142828303  4c8bac2410010000               movq	0x110(%rsp), %r13
// 0x14282830b  0f8763010000                   ja	0x142828474 <.text+0x2827474>
// 0x142828311  e9ea010000                     jmp	0x142828500 <.text+0x2827500>
// 0x142828316  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F183 -> F181
// 0x142828320  f3440f11942494000000           movss	%xmm10, 0x94(%rsp)
// 0x14282832a  f3440f11bc2498000000           movss	%xmm15, 0x98(%rsp)
// 0x142828334  f3440f11b4249c000000           movss	%xmm14, 0x9c(%rsp)
// 0x14282833e  450f28f5                       movaps	%xmm13, %xmm14
// 0x142828342  450f28fc                       movaps	%xmm12, %xmm15
// 0x142828346  440f281df25e6d00               movaps	0x6d5ef2(%rip), %xmm11  # 0x142efe240
// 0x14282834e  410f54f3                       andps	%xmm11, %xmm6
// Frames: F184 -> F185 -> F186
// 0x142828352  0f28c6                         movaps	%xmm6, %xmm0
// 0x142828355  e8263a5f00                     callq	0x142e1bd80 <.text+0x2e1ad80>
// 0x14282835a  440f28d0                       movaps	%xmm0, %xmm10
// Frames: F183 -> F187
// 0x14282835e  410f54fb                       andps	%xmm11, %xmm7
// Frames: F184 -> F185 -> F187
// 0x142828362  0f28c7                         movaps	%xmm7, %xmm0
// 0x142828365  e8163a5f00                     callq	0x142e1bd80 <.text+0x2e1ad80>
// 0x14282836a  440f28e0                       movaps	%xmm0, %xmm12
// Frames: F183 -> F187
// 0x14282836e  450f54cb                       andps	%xmm11, %xmm9
// Frames: F184 -> F185 -> F187
// 0x142828372  410f28c1                       movaps	%xmm9, %xmm0
// 0x142828376  e8053a5f00                     callq	0x142e1bd80 <.text+0x2e1ad80>
// 0x14282837b  440f28e8                       movaps	%xmm0, %xmm13
// Frames: F183 -> F187
// 0x14282837f  450f54c3                       andps	%xmm11, %xmm8
// Frames: F184 -> F185 -> F187
// 0x142828383  410f28c0                       movaps	%xmm8, %xmm0
// 0x142828387  e8f4395f00                     callq	0x142e1bd80 <.text+0x2e1ad80>
// 0x14282838c  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F188
// 0x142828390  488d4c2448                     leaq	0x48(%rsp), %rcx
// 0x142828395  4c89f2                         movq	%r14, %rdx
// 0x142828398  e8330b3cfe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// 0x14282839d  8b442434                       movl	0x34(%rsp), %eax
// 0x1428283a1  488b4c2470                     movq	0x70(%rsp), %rcx
// Frames: F172 -> F173 -> F29 -> F189
// 0x1428283a6  8901                           movl	%eax, (%rcx)
// 0x1428283a8  488b442438                     movq	0x38(%rsp), %rax
// Frames: F174 -> F26 -> F27 -> F175 -> F29 -> F189
// 0x1428283ad  4c8b7830                       movq	0x30(%rax), %r15
// Frames: F176 -> F175 -> F29 -> F189
// 0x1428283b1  4c8b6040                       movq	0x40(%rax), %r12
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F189
// 0x1428283b5  4c3b7820                       cmpq	0x20(%rax), %r15
// 0x1428283b9  750a                           jne	0x1428283c5 <.text+0x28273c5>
// Frames: F25 -> F26 -> F27 -> F175 -> F29 -> F189
// 0x1428283bb  488b4c2468                     movq	0x68(%rsp), %rcx
// 0x1428283c0  e85b6f5400                     callq	0x142d6f320 <.text+0x2d6e320>
// Frames: F31 -> F32 -> F33 -> F34 -> F35 -> F26 -> F27 -> F175 -> F29 -> F189
// 0x1428283c5  f3450f58e5                     addss	%xmm13, %xmm12
// 0x1428283ca  f3450f58dc                     addss	%xmm12, %xmm11
// 0x1428283cf  f3410f58f9                     addss	%xmm9, %xmm7
// 0x1428283d4  f3440f58c7                     addss	%xmm7, %xmm8
// 0x1428283d9  0f57c0                         xorps	%xmm0, %xmm0
// 0x1428283dc  f3410fc2c201                   cmpltss	%xmm10, %xmm0
// 0x1428283e2  0f54f0                         andps	%xmm0, %xmm6
// 0x1428283e5  410f55c0                       andnps	%xmm8, %xmm0
// 0x1428283e9  0f56c6                         orps	%xmm6, %xmm0
// 0x1428283ec  4c8b442438                     movq	0x38(%rsp), %r8
// 0x1428283f1  498b4028                       movq	0x28(%r8), %rax
// Frames: F36 -> F35 -> F26 -> F27 -> F175 -> F29 -> F189
// 0x1428283f5  4b8d0cbf                       leaq	(%r15,%r15,4), %rcx
// Frames: F37 -> F38 -> F26 -> F27 -> F175 -> F29 -> F189
// 0x1428283f9  4c8924c8                       movq	%r12, (%rax,%rcx,8)
// 0x1428283fd  c644c80802                     movb	$0x2, 0x8(%rax,%rcx,8)
// 0x142828402  44886cc809                     movb	%r13b, 0x9(%rax,%rcx,8)
// 0x142828407  f3440f1154c80c                 movss	%xmm10, 0xc(%rax,%rcx,8)
// 0x14282840e  f3440f115cc810                 movss	%xmm11, 0x10(%rax,%rcx,8)
// 0x142828415  f30f1144c814                   movss	%xmm0, 0x14(%rax,%rcx,8)
// 0x14282841b  8b542450                       movl	0x50(%rsp), %edx
// 0x14282841f  8954c820                       movl	%edx, 0x20(%rax,%rcx,8)
// 0x142828423  488b542448                     movq	0x48(%rsp), %rdx
// 0x142828428  488954c818                     movq	%rdx, 0x18(%rax,%rcx,8)
// Frames: F39 -> F26 -> F27 -> F175 -> F29 -> F189
// 0x14282842d  49ffc7                         incq	%r15
// 0x142828430  4d897830                       movq	%r15, 0x30(%r8)
// Frames: F40 -> F175 -> F29 -> F189
// 0x142828434  49ff4040                       incq	0x40(%r8)
// 0x142828438  450f57db                       xorps	%xmm11, %xmm11
// 0x14282843c  450f28e7                       movaps	%xmm15, %xmm12
// 0x142828440  450f28ee                       movaps	%xmm14, %xmm13
// 0x142828444  f3440f10b4249c000000           movss	0x9c(%rsp), %xmm14
// 0x14282844e  f3440f10bc2498000000           movss	0x98(%rsp), %xmm15
// 0x142828458  f3440f10942494000000           movss	0x94(%rsp), %xmm10
// Frames: F182
// 0x142828462  450f2ed3                       ucomiss	%xmm11, %xmm10
// 0x142828466  4c8bac2410010000               movq	0x110(%rsp), %r13
// 0x14282846e  0f868c000000                   jbe	0x142828500 <.text+0x2827500>
// Frames: F190
// 0x142828474  f3410f107624                   movss	0x24(%r14), %xmm6
// 0x14282847a  f3410f107e28                   movss	0x28(%r14), %xmm7
// Frames: F191
// 0x142828480  488d4c244c                     leaq	0x4c(%rsp), %rcx
// 0x142828485  4c89f2                         movq	%r14, %rdx
// 0x142828488  e8430a3cfe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// 0x14282848d  8b442434                       movl	0x34(%rsp), %eax
// 0x142828491  488b4c2470                     movq	0x70(%rsp), %rcx
// Frames: F172 -> F173 -> F29 -> F192
// 0x142828496  8901                           movl	%eax, (%rcx)
// 0x142828498  488b442438                     movq	0x38(%rsp), %rax
// Frames: F174 -> F26 -> F27 -> F175 -> F29 -> F192
// 0x14282849d  4c8b7030                       movq	0x30(%rax), %r14
// Frames: F176 -> F175 -> F29 -> F192
// 0x1428284a1  4c8b7840                       movq	0x40(%rax), %r15
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F192
// 0x1428284a5  4c3b7020                       cmpq	0x20(%rax), %r14
// 0x1428284a9  750a                           jne	0x1428284b5 <.text+0x28274b5>
// Frames: F25 -> F26 -> F27 -> F175 -> F29 -> F192
// 0x1428284ab  488b4c2468                     movq	0x68(%rsp), %rcx
// 0x1428284b0  e86b6e5400                     callq	0x142d6f320 <.text+0x2d6e320>
// Frames: F31 -> F32 -> F33 -> F34 -> F35 -> F26 -> F27 -> F175 -> F29 -> F192
// 0x1428284b5  f30f59f6                       mulss	%xmm6, %xmm6
// 0x1428284b9  f30f59ff                       mulss	%xmm7, %xmm7
// 0x1428284bd  f30f58fe                       addss	%xmm6, %xmm7
// 0x1428284c1  0f57c0                         xorps	%xmm0, %xmm0
// 0x1428284c4  f30f51c7                       sqrtss	%xmm7, %xmm0
// 0x1428284c8  488b542438                     movq	0x38(%rsp), %rdx
// 0x1428284cd  488b4228                       movq	0x28(%rdx), %rax
// Frames: F36 -> F35 -> F26 -> F27 -> F175 -> F29 -> F192
// 0x1428284d1  4b8d0cb6                       leaq	(%r14,%r14,4), %rcx
// Frames: F37 -> F38 -> F26 -> F27 -> F175 -> F29 -> F192
// 0x1428284d5  4c893cc8                       movq	%r15, (%rax,%rcx,8)
// 0x1428284d9  c644c80803                     movb	$0x3, 0x8(%rax,%rcx,8)
// 0x1428284de  f30f1144c80c                   movss	%xmm0, 0xc(%rax,%rcx,8)
// 0x1428284e4  f3440f1154c810                 movss	%xmm10, 0x10(%rax,%rcx,8)
// 0x1428284eb  0f10442448                     movups	0x48(%rsp), %xmm0
// 0x1428284f0  0f1144c814                     movups	%xmm0, 0x14(%rax,%rcx,8)
// Frames: F39 -> F26 -> F27 -> F175 -> F29 -> F192
// 0x1428284f5  49ffc6                         incq	%r14
// 0x1428284f8  4c897230                       movq	%r14, 0x30(%rdx)
// Frames: F40 -> F175 -> F29 -> F192
// 0x1428284fc  48ff4240                       incq	0x40(%rdx)
// Frames: F172 -> F173 -> F29 -> F193
// 0x142828500  8b8424a0000000                 movl	0xa0(%rsp), %eax
// 0x142828507  488b8c2408010000               movq	0x108(%rsp), %rcx
// 0x14282850f  8901                           movl	%eax, (%rcx)
// Frames: F174 -> F26 -> F27 -> F175 -> F29 -> F193
// 0x142828511  4d8b7530                       movq	0x30(%r13), %r14
// Frames: F176 -> F175 -> F29 -> F193
// 0x142828515  498b7d40                       movq	0x40(%r13), %rdi
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F193
// 0x142828519  4d3b7520                       cmpq	0x20(%r13), %r14
// 0x14282851d  4c8bbc24c8000000               movq	0xc8(%rsp), %r15
// 0x142828525  750d                           jne	0x142828534 <.text+0x2827534>
// Frames: F25 -> F26 -> F27 -> F175 -> F29 -> F193
// 0x142828527  488b8c24e8000000               movq	0xe8(%rsp), %rcx
// 0x14282852f  e87c6d5400                     callq	0x142d6f2b0 <.text+0x2d6e2b0>
// Frames: F31 -> F32 -> F33 -> F34 -> F35 -> F26 -> F27 -> F175 -> F29 -> F193
// 0x142828534  498b4528                       movq	0x28(%r13), %rax
// Frames: F36 -> F35 -> F26 -> F27 -> F175 -> F29 -> F193
// 0x142828538  4c89f1                         movq	%r14, %rcx
// 0x14282853b  48c1e105                       shlq	$0x5, %rcx
// Frames: F37 -> F38 -> F26 -> F27 -> F175 -> F29 -> F193
// 0x14282853f  48893c08                       movq	%rdi, (%rax,%rcx)
// 0x142828543  4c897c0808                     movq	%r15, 0x8(%rax,%rcx)
// 0x142828548  488b9424d0000000               movq	0xd0(%rsp), %rdx
// 0x142828550  4889540810                     movq	%rdx, 0x10(%rax,%rcx)
// 0x142828555  c644081801                     movb	$0x1, 0x18(%rax,%rcx)
// Frames: F39 -> F26 -> F27 -> F175 -> F29 -> F193
// 0x14282855a  49ffc6                         incq	%r14
// 0x14282855d  4d897530                       movq	%r14, 0x30(%r13)
// Frames: F40 -> F175 -> F29 -> F193
// 0x142828561  49ff4540                       incq	0x40(%r13)
// Frames: F194
// 0x142828565  450f2ef4                       ucomiss	%xmm12, %xmm14
// 0x142828569  7655                           jbe	0x1428285c0 <.text+0x28275c0>
// 0x14282856b  450f2ee7                       ucomiss	%xmm15, %xmm12
// 0x14282856f  724f                           jb	0x1428285c0 <.text+0x28275c0>
// Frames: F195
// 0x142828571  488b442478                     movq	0x78(%rsp), %rax
// 0x142828576  440fb67034                     movzbl	0x34(%rax), %r14d
// 0x14282857b  8b442440                       movl	0x40(%rsp), %eax
// 0x14282857f  488b8c24b0000000               movq	0xb0(%rsp), %rcx
// Frames: F172 -> F173 -> F29 -> F196
// 0x142828587  8901                           movl	%eax, (%rcx)
// 0x142828589  488b8c2480000000               movq	0x80(%rsp), %rcx
// Frames: F174 -> F26 -> F27 -> F175 -> F29 -> F196
// 0x142828591  488b5930                       movq	0x30(%rcx), %rbx
// Frames: F176 -> F175 -> F29 -> F196
// 0x142828595  4c8b7940                       movq	0x40(%rcx), %r15
// 0x142828599  4531e4                         xorl	%r12d, %r12d
// 0x14282859c  b800000000                     movl	$0x0, %eax
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F196
// 0x1428285a1  483b5920                       cmpq	0x20(%rcx), %rbx
// 0x1428285a5  488bbc2488000000               movq	0x88(%rsp), %rdi
// 0x1428285ad  0f84cdf6ffff                   je	0x142827c80 <.text+0x2826c80>
// 0x1428285b3  e9d8f6ffff                     jmp	0x142827c90 <.text+0x2826c90>
// 0x1428285b8  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F197
// 0x1428285c0  450f2ee6                       ucomiss	%xmm14, %xmm12
// 0x1428285c4  488bbc2488000000               movq	0x88(%rsp), %rdi
// 0x1428285cc  4889f8                         movq	%rdi, %rax
// 0x1428285cf  488b8c24c0000000               movq	0xc0(%rsp), %rcx
// 0x1428285d7  4989c8                         movq	%rcx, %r8
// 0x1428285da  488b542478                     movq	0x78(%rsp), %rdx
// 0x1428285df  0f82fff6ffff                   jb	0x142827ce4 <.text+0x2826ce4>
// 0x1428285e5  450f2efc                       ucomiss	%xmm12, %xmm15
// 0x1428285e9  4889f8                         movq	%rdi, %rax
// 0x1428285ec  4989c8                         movq	%rcx, %r8
// 0x1428285ef  0f86eff6ffff                   jbe	0x142827ce4 <.text+0x2826ce4>
// Frames: F198
// 0x1428285f5  440fb67234                     movzbl	0x34(%rdx), %r14d
// 0x1428285fa  8b442440                       movl	0x40(%rsp), %eax
// 0x1428285fe  488b8c24b0000000               movq	0xb0(%rsp), %rcx
// Frames: F172 -> F173 -> F29 -> F199
// 0x142828606  8901                           movl	%eax, (%rcx)
// 0x142828608  488b8c2480000000               movq	0x80(%rsp), %rcx
// Frames: F174 -> F26 -> F27 -> F175 -> F29 -> F199
// 0x142828610  488b5930                       movq	0x30(%rcx), %rbx
// Frames: F176 -> F175 -> F29 -> F199
// 0x142828614  4c8b7940                       movq	0x40(%rcx), %r15
// 0x142828618  41b401                         movb	$0x1, %r12b
// 0x14282861b  b001                           movb	$0x1, %al
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F199
// 0x14282861d  483b5920                       cmpq	0x20(%rcx), %rbx
// 0x142828621  0f8459f6ffff                   je	0x142827c80 <.text+0x2826c80>
// Frames: F177 -> F26 -> F27 -> F175 -> F29 -> F196
// 0x142828627  e964f6ffff                     jmp	0x142827c90 <.text+0x2826c90>
// Frames: F200
// 0x14282862c  0f28b42430010000               movaps	0x130(%rsp), %xmm6
// 0x142828634  0f28bc2440010000               movaps	0x140(%rsp), %xmm7
// 0x14282863c  440f28842450010000             movaps	0x150(%rsp), %xmm8
// 0x142828645  440f288c2460010000             movaps	0x160(%rsp), %xmm9
// 0x14282864e  440f28942470010000             movaps	0x170(%rsp), %xmm10
// 0x142828657  440f289c2480010000             movaps	0x180(%rsp), %xmm11
// 0x142828660  440f28a42490010000             movaps	0x190(%rsp), %xmm12
// 0x142828669  440f28ac24a0010000             movaps	0x1a0(%rsp), %xmm13
// 0x142828672  440f28b424b0010000             movaps	0x1b0(%rsp), %xmm14
// 0x14282867b  440f28bc24c0010000             movaps	0x1c0(%rsp), %xmm15
// 0x142828684  4881c4d8010000                 addq	$0x1d8, %rsp            # imm = 0x1D8
// 0x14282868b  5b                             popq	%rbx
// 0x14282868c  5d                             popq	%rbp
// 0x14282868d  5f                             popq	%rdi
// 0x14282868e  5e                             popq	%rsi
// 0x14282868f  415c                           popq	%r12
// 0x142828691  415d                           popq	%r13
// 0x142828693  415e                           popq	%r14
// 0x142828695  415f                           popq	%r15
// 0x142828697  c3                             retq
// Frames: F201 -> F202 -> F57 -> F58
// 0x142828698  48b800000000ffffffff           movabsq	$-0x100000000, %rax # imm = 0xFFFFFFFF00000000
// 0x1428286a2  4921c1                         andq	%rax, %r9
// 0x1428286a5  49ffc1                         incq	%r9
// 0x1428286a8  eb04                           jmp	0x1428286ae <.text+0x28276ae>
// Frames: F203 -> F57 -> F58
// 0x1428286aa  49c1e120                       shlq	$0x20, %r9
// Frames: F204 -> F58
// 0x1428286ae  4c894c2448                     movq	%r9, 0x48(%rsp)
// 0x1428286b3  4c89542450                     movq	%r10, 0x50(%rsp)
// 0x1428286b8  488d05f902d600                 leaq	0xd602f9(%rip), %rax    # 0x1435889b8
// 0x1428286bf  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1428286c4  488d0d5d03d600                 leaq	0xd6035d(%rip), %rcx    # 0x143588a28
// 0x1428286cb  4c8d0d3603d600                 leaq	0xd60336(%rip), %r9     # 0x143588a08
// 0x1428286d2  4c8d442448                     leaq	0x48(%rsp), %r8
// 0x1428286d7  ba2b000000                     movl	$0x2b, %edx
// 0x1428286dc  e85f8f5800                     callq	0x142db1640 <.text+0x2db0640>
// Frames: F205 -> F206 -> F53
// 0x1428286e1  4983c720                       addq	$0x20, %r15
// 0x1428286e5  4c897c2448                     movq	%r15, 0x48(%rsp)
// 0x1428286ea  488d051f000000                 leaq	0x1f(%rip), %rax        # 0x142828710 <.text+0x2827710>
// 0x1428286f1  4889442450                     movq	%rax, 0x50(%rsp)
// 0x1428286f6  488d0dd302d600                 leaq	0xd602d3(%rip), %rcx    # 0x1435889d0
// 0x1428286fd  4c8d05ec02d600                 leaq	0xd602ec(%rip), %r8     # 0x1435889f0
// 0x142828704  488d542448                     leaq	0x48(%rsp), %rdx
// 0x142828709  e892915800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x14282870e  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/edit_roof.rs:10:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:1065:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:28:0 bevy_ecs::message::message_reader::MessageReader::read
// F4: crates/systems/roof/src/edit_roof.rs:23:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:55:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:93:0 bevy_ecs::message::message_cursor::MessageCursor::read_with_id
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:80:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F9: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2345:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:57:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:60:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:923:0 core::num::impl$11::checked_sub
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:368:0 core::slice::index::impl$4::get
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:546:0 core::slice::index::impl$7::get
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:61:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:62:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:67:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F22: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 core::slice::impl$0::iter
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:69:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1029:0 alloc::vec::Vec::push_mut
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:992:0 alloc::vec::Vec::push
// F27: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:143:0 bevy_ecs::message::messages::Messages::write_with_caller
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::message::messages::Messages::write
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:70:0 bevy_ecs::message::message_writer::MessageWriter::write
// F30: crates/systems/roof/src/edit_roof.rs:0:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F32: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F34: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2000:0 alloc::vec::Vec::as_mut_ptr
// F35: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1032:0 alloc::vec::Vec::push_mut
// F36: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 alloc::vec::Vec::as_mut_ptr
// F37: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1917:0 core::ptr::write
// F38: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:0 alloc::vec::Vec::push_mut
// F39: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1034:0 alloc::vec::Vec::push_mut
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:144:0 bevy_ecs::message::messages::Messages::write_with_caller
// F41: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:767:0 core::option::Option::as_mut
// F42: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:332:0 core::iter::adapters::chain::and_then_or_clear
// F43: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:89:0 bevy_ecs::message::iterators::impl$3::next
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:15:0 bevy_ecs::message::iterators::impl$0::next
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next::closure$0
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1656:0 core::option::Option::or_else
// F48: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:659:0 core::ptr::non_null::NonNull::add
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:185:0 core::slice::iter::impl$171::next
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:95:0 bevy_ecs::message::iterators::impl$3::next
// F51: crates/systems/roof/src/edit_roof.rs:25:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F52: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1065:0 core::option::Option::unwrap_or_else
// F53: crates/systems/roof/src/edit_roof.rs:26:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:957:0 bevy_ecs::entity::Entities::get_id_location
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:949:0 bevy_ecs::entity::Entities::get
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1546:0 bevy_ecs::system::query::Query::get_inner
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1525:0 bevy_ecs::system::query::Query::get_mut
// F58: crates/systems/roof/src/edit_roof.rs:28:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F59: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F60: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1583:0 core::option::Option::filter
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::Entities::get_id_location
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:214:0 bevy_ecs::entity::impl$37::eq
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::impl$18::get_id_location::closure$0
// F64: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1584:0 core::option::Option::filter
// F65: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1589:0 core::option::Option::filter
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1338:0 core::option::Option::ok_or
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1547:0 bevy_ecs::system::query::Query::get_inner
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:123:0 bevy_ecs::archetype::ArchetypeId::index
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1551:0 bevy_ecs::system::query::Query::get_inner
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 bevy_ecs::archetype::ArchetypeId::index
// F71: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 bevy_ecs::archetype::ArchetypeId::index
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F76: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/internal_macros.rs:28:0 fixedbitset::FixedBitSet::get_unchecked
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1548:0 bevy_ecs::system::query::Query::get_inner
// F78: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1580:0 bevy_ecs::system::query::Query::get_inner
// F81: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:221:0 core::slice::index::impl$2::get
// F82: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 alloc::vec::impl$9::deref
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1582:0 bevy_ecs::system::query::Query::get_inner
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1932:0 bevy_ecs::query::fetch::impl$48::set_table
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1921:0 bevy_ecs::query::fetch::impl$48::set_archetype
// F88: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F89: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F90: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::option::Option::and_then
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F94: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F95: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F96: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F97: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F99: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2000:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1993:0 bevy_ecs::query::fetch::impl$49::fetch
// F102: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1591:0 bevy_ecs::system::query::Query::get_inner
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ecs::storage::table::TableRow::index
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs:305:0 bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:314:0 bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1939:0 bevy_ecs::query::fetch::impl$48::set_table
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ptr::ThinSlicePtr::get
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2002:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F109: crates/systems/roof/src/edit_roof.rs:30:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F110: crates/systems/roof/src/edit_roof.rs:32:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F111: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1512:0 core::f32::impl$0::clamp
// F112: crates/systems/roof/src/edit_roof.rs:34:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F113: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1515:0 core::f32::impl$0::clamp
// F114: crates/systems/roof/src/edit_roof.rs:35:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F117: crates/systems/roof/src/edit_roof.rs:46:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F118: crates/systems/roof/src/edit_roof.rs:47:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F119: crates/systems/roof/src/edit_roof.rs:55:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F120: crates/systems/roof/src/edit_roof.rs:57:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F121: crates/systems/roof/src/edit_roof.rs:58:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F122: crates/systems/roof/src/edit_roof.rs:37:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F123: crates/systems/roof/src/edit_roof.rs:38:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F124: crates/systems/roof/src/edit_roof.rs:39:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F125: crates/systems/roof/src/edit_roof.rs:49:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F126: crates/systems/roof/src/edit_roof.rs:52:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F127: crates/systems/roof/src/edit_roof.rs:53:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F128: crates/systems/roof/src/edit_roof.rs:64:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F129: crates/systems/roof/src/edit_roof.rs:71:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F130: crates/systems/roof/src/edit_roof.rs:72:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F131: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:791:0 core::ops::arith::impl$353::add_assign
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1429:0 glam::f32::vec2::impl$38::add_assign
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:186:0 glam::f32::vec2::Vec2::dot
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:472:0 glam::f32::vec2::Vec2::length_squared
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:869:0 glam::f32::vec2::Vec2::clamp_length_max
// F136: crates/systems/roof/src/edit_roof.rs:73:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:870:0 glam::f32::vec2::Vec2::clamp_length_max
// F138: crates/systems/roof/src/edit_roof.rs:66:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F139: crates/systems/roof/src/edit_roof.rs:67:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F140: crates/systems/roof/src/edit_roof.rs:68:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F141: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:218:0 core::ops::arith::impl$77::sub
// F142: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1531:0 glam::f32::vec2::impl$50::sub
// F143: crates/systems/roof/src/edit_roof.rs:69:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:462:0 glam::f32::vec2::Vec2::length
// F145: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:2041:0 core::f32::math::sqrt
// F146: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:385:0 std::f32::impl$0::sqrt
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs:193:0 glam::f32::vec2::Vec2::dot
// F148: crates/systems/roof/src/edit_roof.rs:78:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F149: crates/systems/roof/src/edit_roof.rs:85:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F150: crates/systems/roof/src/edit_roof.rs:86:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F151: crates/systems/roof/src/edit_roof.rs:87:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F152: crates/systems/roof/src/edit_roof.rs:80:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F153: crates/systems/roof/src/edit_roof.rs:81:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F154: crates/systems/roof/src/edit_roof.rs:82:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F155: crates/systems/roof/src/edit_roof.rs:83:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F156: crates/systems/roof/src/edit_roof.rs:92:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F157: crates/systems/roof/src/edit_roof.rs:94:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F158: crates/systems/roof/src/edit_roof.rs:102:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F159: crates/systems/roof/src/edit_roof.rs:103:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F160: crates/systems/roof/src/edit_roof.rs:104:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F161: crates/systems/roof/src/edit_roof.rs:105:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F162: crates/systems/roof/src/edit_roof.rs:96:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F163: crates/systems/roof/src/edit_roof.rs:97:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F164: crates/systems/roof/src/edit_roof.rs:98:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F165: crates/systems/roof/src/edit_roof.rs:99:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F166: crates/systems/roof/src/edit_roof.rs:100:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F167: crates/systems/roof/src/edit_roof.rs:111:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F168: crates/systems/roof/src/edit_roof.rs:119:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F169: crates/systems/roof/src/edit_roof.rs:126:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F170: crates/systems/roof/src/edit_roof.rs:129:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F171: crates/systems/roof/src/edit_roof.rs:127:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F172: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F173: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F174: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1025:0 alloc::vec::Vec::push_mut
// F175: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::change_detection::impl$38::deref_mut
// F176: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:131:0 bevy_ecs::message::messages::Messages::write_with_caller
// F177: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1028:0 alloc::vec::Vec::push_mut
// F178: crates/systems/roof/src/edit_roof.rs:116:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F179: crates/systems/roof/src/edit_roof.rs:122:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F180: crates/systems/roof/src/edit_roof.rs:120:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F181: crates/systems/roof/src/edit_roof.rs:133:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F182: crates/systems/roof/src/edit_roof.rs:156:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F183: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1570:0 core::f32::impl$0::abs
// F184: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1750:0 core::f32::math::ceil
// F185: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:73:0 std::f32::impl$0::ceil
// F186: crates/systems/roof/src/edit_roof.rs:139:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F187: crates/systems/roof/src/edit_roof.rs:140:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F188: crates/systems/roof/src/edit_roof.rs:152:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F189: crates/systems/roof/src/edit_roof.rs:141:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F190: crates/systems/roof/src/edit_roof.rs:159:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F191: crates/systems/roof/src/edit_roof.rs:162:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F192: crates/systems/roof/src/edit_roof.rs:157:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F193: crates/systems/roof/src/edit_roof.rs:166:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F194: crates/systems/roof/src/edit_roof.rs:173:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F195: crates/systems/roof/src/edit_roof.rs:178:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F196: crates/systems/roof/src/edit_roof.rs:174:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F197: crates/systems/roof/src/edit_roof.rs:180:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F198: crates/systems/roof/src/edit_roof.rs:185:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F199: crates/systems/roof/src/edit_roof.rs:181:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F200: crates/systems/roof/src/edit_roof.rs:189:0 system_roof::edit_roof::edit_roof::h7b4296151b2b0f42
// F201: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:2189:0 core::result::impl$28::from_residual
// F202: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1543:0 bevy_ecs::system::query::Query::get_inner
// F203: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1553:0 bevy_ecs::system::query::Query::get_inner
// F204: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1233:0 core::result::Result::unwrap
// F205: crates/systems/roof/src/edit_roof.rs:26:0 system_roof::edit_roof::edit_roof::closure$0
// F206: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1067:0 core::option::Option::unwrap_or_else
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/edit_roof.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 31f40abbbe125e1c25a958868465ddae142bef7336dba1ad879138cb9e7a502e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: edit_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3780
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::edit_roof::edit_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3780,"record_offset":268,"rva":42105488,"va":5410814608}
// 编译实例 RVAs: 0x2827a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: edit_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5535267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e823765400","size":5,"soft_fail":false,"target_va":5416350384,"va":5410815112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: edit_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5534045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e85d715400","size":5,"soft_fail":false,"target_va":5416350496,"va":5410816446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5534515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e833735400","size":5,"soft_fail":false,"target_va":5416351056,"va":5410816536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: edit_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6240806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e8263a5f00","size":5,"soft_fail":false,"target_va":5417057664,"va":5410816853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5804434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e892915800","size":5,"soft_fail":false,"target_va":5416622240,"va":5410817801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5803871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e85f8f5800","size":5,"soft_fail":false,"target_va":5416621632,"va":5410817756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29618638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e8320e3cfe","size":5,"soft_fail":false,"target_va":5381197472,"va":5410816105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29618865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e84f0d3cfe","size":5,"soft_fail":false,"target_va":5381197520,"va":5410816380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29912871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410814608,"raw_hex":"e8d99037fe","size":5,"soft_fail":false,"target_va":5380902448,"va":5410815314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3780,"record_offset":3432,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3780,"record_offset":1604,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3780,"record_offset":440,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3780,"record_offset":1148,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3780,"record_offset":1952,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3780,"record_offset":5676,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3780,"record_offset":3400,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3780,"record_offset":1888,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3780,"record_offset":7484,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3780,"record_offset":3864,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":32131,"source_line":867,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3780,"record_offset":4152,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3780,"record_offset":3808,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3780,"record_offset":4096,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3780,"record_offset":5704,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3780,"record_offset":1320,"parent_record_offset":1288,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3780,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3780,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3780,"record_offset":1288,"parent_record_offset":1252,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3780,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3780,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3780,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3780,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3780,"record_offset":752,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3780,"record_offset":2288,"parent_record_offset":1984,"ancestor_record_offset":268,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3780,"record_offset":4680,"parent_record_offset":4648,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3780,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3780,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":268,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3780,"record_offset":2052,"parent_record_offset":1984,"ancestor_record_offset":268,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3780,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":268,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3780,"record_offset":2192,"parent_record_offset":2160,"ancestor_record_offset":268,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3780,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":268,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3780,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3780,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3780,"record_offset":504,"parent_record_offset":464,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3780,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":268,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3780,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3780,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3780,"record_offset":2708,"parent_record_offset":1984,"ancestor_record_offset":268,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3780,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":268,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3780,"record_offset":3120,"parent_record_offset":1984,"ancestor_record_offset":268,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3780,"record_offset":3176,"parent_record_offset":3148,"ancestor_record_offset":268,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3780,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3780,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3780,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3780,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3780,"record_offset":3204,"parent_record_offset":3176,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3780,"record_offset":2476,"parent_record_offset":1984,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3780,"record_offset":3056,"parent_record_offset":2736,"ancestor_record_offset":268,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":3780,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":268,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3780,"record_offset":1984,"parent_record_offset":1952,"ancestor_record_offset":268,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3780,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":268,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3780,"record_offset":3296,"parent_record_offset":3176,"ancestor_record_offset":268,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3780,"record_offset":2324,"parent_record_offset":2288,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3780,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":268,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3780,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":268,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3780,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3780,"record_offset":1656,"parent_record_offset":1628,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3780,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3780,"record_offset":708,"parent_record_offset":680,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3780,"record_offset":568,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3780,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":268,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3780,"record_offset":4120,"parent_record_offset":4096,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3780,"record_offset":2864,"parent_record_offset":2840,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3780,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3780,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3780,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3780,"record_offset":2160,"parent_record_offset":2076,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3780,"record_offset":2928,"parent_record_offset":2792,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3780,"record_offset":2260,"parent_record_offset":1984,"ancestor_record_offset":268,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3780,"record_offset":1736,"parent_record_offset":1656,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3780,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3780,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3780,"record_offset":1408,"parent_record_offset":1252,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3780,"record_offset":3364,"parent_record_offset":1984,"ancestor_record_offset":268,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3780,"record_offset":624,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3780,"record_offset":992,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3780,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3780,"record_offset":3008,"parent_record_offset":2976,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3780,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3780,"record_offset":652,"parent_record_offset":624,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3780,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3780,"record_offset":1812,"parent_record_offset":1760,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3780,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":268,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3780,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":268,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3780,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":268,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3780,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":268,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3780,"record_offset":3892,"parent_record_offset":3864,"ancestor_record_offset":268,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3780,"record_offset":3236,"parent_record_offset":3204,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3780,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3780,"record_offset":4208,"parent_record_offset":4176,"ancestor_record_offset":268,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::edit_roof::edit_roof::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/edit_roof.rs
//     evidence={"module":3780,"record_offset":1920,"parent_record_offset":1888,"ancestor_record_offset":268,"inlinee":133053,"source_line":26,"archive":"_pdb/C/modules/03780.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::edit_roof::edit_roof::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x207BD
// 首条位置: {"module":3780,"inlinee":133053}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>

// ------------------------------------------------------------------
// 函数: system_roof::edit_roof::edit_roof
// 信息 ID: 8c63c60a917e214b02a5db99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8f96330335713f64e26c5e5a: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > @ 0x1f2b5fb -> 0x2827a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> @ 0x1f2fef6 -> 0x2827a90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::edit_roof::edit_roof::closure$0
// 信息 ID: 1cf061339d42f8b794bffa65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::edit_roof::edit_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::edit_roof::edit_roof; module=3780, inline_record=1920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
