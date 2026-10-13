// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: generate_roof_stone_floor_and_roof_bottom.rs.assembly.json.gz

// FUNCTION system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// Candidate interfaces: ea165d21b691762bea950615
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3463:372 -> rva_21a6d40_len_348
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21a6d40_len_348: RVA 0x21a6d40, 840 PDB bytes; 204 instructions
// Evidence: _assembly/roof-v1/B/pack-001/rva_21a6d40_len_348.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3463:372: system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// Frames: F1
// 0x1421a6d40  4157                           pushq	%r15
// 0x1421a6d42  4156                           pushq	%r14
// 0x1421a6d44  4155                           pushq	%r13
// 0x1421a6d46  4154                           pushq	%r12
// 0x1421a6d48  56                             pushq	%rsi
// 0x1421a6d49  57                             pushq	%rdi
// 0x1421a6d4a  55                             pushq	%rbp
// 0x1421a6d4b  53                             pushq	%rbx
// 0x1421a6d4c  4881ecb8000000                 subq	$0xb8, %rsp
// 0x1421a6d53  440f299424a0000000             movaps	%xmm10, 0xa0(%rsp)
// 0x1421a6d5c  440f298c2490000000             movaps	%xmm9, 0x90(%rsp)
// 0x1421a6d65  440f29842480000000             movaps	%xmm8, 0x80(%rsp)
// 0x1421a6d6e  0f297c2470                     movaps	%xmm7, 0x70(%rsp)
// 0x1421a6d73  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F2 -> F3 -> F4
// 0x1421a6d78  488b39                         movq	(%rcx), %rdi
// Frames: F5 -> F3 -> F4
// 0x1421a6d7b  4c8b5108                       movq	0x8(%rcx), %r10
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x1421a6d7f  4c8b1f                         movq	(%rdi), %r11
// 0x1421a6d82  31c0                           xorl	%eax, %eax
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x1421a6d84  4d89d9                         movq	%r11, %r9
// 0x1421a6d87  4d2b4a18                       subq	0x18(%r10), %r9
// 0x1421a6d8b  4c0f42c8                       cmovbq	%rax, %r9
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x1421a6d8f  4d2b5a38                       subq	0x38(%r10), %r11
// 0x1421a6d93  4c0f42d8                       cmovbq	%rax, %r11
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421a6d97  4c89c9                         movq	%r9, %rcx
// 0x1421a6d9a  48c1e105                       shlq	$0x5, %rcx
// 0x1421a6d9e  49034a08                       addq	0x8(%r10), %rcx
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421a6da2  498b5a10                       movq	0x10(%r10), %rbx
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421a6da6  4d8b7230                       movq	0x30(%r10), %r14
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421a6daa  4c29cb                         subq	%r9, %rbx
// 0x1421a6dad  480f42d8                       cmovbq	%rax, %rbx
// 0x1421a6db1  be08000000                     movl	$0x8, %esi
// 0x1421a6db6  480f42ce                       cmovbq	%rsi, %rcx
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421a6dba  4d89d9                         movq	%r11, %r9
// 0x1421a6dbd  49c1e105                       shlq	$0x5, %r9
// 0x1421a6dc1  4d034a28                       addq	0x28(%r10), %r9
// 0x1421a6dc5  4d29de                         subq	%r11, %r14
// 0x1421a6dc8  4c0f42f0                       cmovbq	%rax, %r14
// 0x1421a6dcc  4c0f42ce                       cmovbq	%rsi, %r9
// Frames: F20 -> F7 -> F8 -> F3 -> F4
// 0x1421a6dd0  4d8b5240                       movq	0x40(%r10), %r10
// 0x1421a6dd4  4d29f2                         subq	%r14, %r10
// 0x1421a6dd7  4929da                         subq	%rbx, %r10
// 0x1421a6dda  48897c2458                     movq	%rdi, 0x58(%rsp)
// 0x1421a6ddf  4c8917                         movq	%r10, (%rdi)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F4
// 0x1421a6de2  48c1e305                       shlq	$0x5, %rbx
// 0x1421a6de6  48895c2438                     movq	%rbx, 0x38(%rsp)
// 0x1421a6deb  49c1e605                       shlq	$0x5, %r14
// 0x1421a6def  4c89742430                     movq	%r14, 0x30(%rsp)
// 0x1421a6df4  498b7008                       movq	0x8(%r8), %rsi
// 0x1421a6df8  4885f6                         testq	%rsi, %rsi
// 0x1421a6dfb  7504                           jne	0x1421a6e01 <.text+0x21a5e01>
// 0x1421a6dfd  498b7010                       movq	0x10(%r8), %rsi
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F4
// 0x1421a6e01  48014c2438                     addq	%rcx, 0x38(%rsp)
// 0x1421a6e06  4c014c2430                     addq	%r9, 0x30(%rsp)
// 0x1421a6e0b  488b2a                         movq	(%rdx), %rbp
// 0x1421a6e0e  4c8b6a08                       movq	0x8(%rdx), %r13
// 0x1421a6e12  f30f10355e628200               movss	0x82625e(%rip), %xmm6   # 0x1429cd078
// 0x1421a6e1a  f30f103de2ea7700               movss	0x77eae2(%rip), %xmm7   # 0x142925904
// 0x1421a6e22  f3440f10055deb7700             movss	0x77eb5d(%rip), %xmm8   # 0x142925988
// 0x1421a6e2b  450f57c9                       xorps	%xmm9, %xmm9
// 0x1421a6e2f  f3440f101598997f00             movss	0x7f9998(%rip), %xmm10  # 0x1429a07d0
// 0x1421a6e38  4885c9                         testq	%rcx, %rcx
// 0x1421a6e3b  7413                           je	0x1421a6e50 <.text+0x21a5e50>
// 0x1421a6e3d  0f1f00                         nopl	(%rax)
// 0x1421a6e40  483b4c2438                     cmpq	0x38(%rsp), %rcx
// 0x1421a6e45  7409                           je	0x1421a6e50 <.text+0x21a5e50>
// 0x1421a6e47  4c8d7920                       leaq	0x20(%rcx), %r15
// 0x1421a6e4b  4d89cc                         movq	%r9, %r12
// 0x1421a6e4e  eb1e                           jmp	0x1421a6e6e <.text+0x21a5e6e>
// Frames: F25 -> F30 -> F31 -> F27 -> F28 -> F29 -> F4
// 0x1421a6e50  4d85c9                         testq	%r9, %r9
// 0x1421a6e53  0f849e010000                   je	0x1421a6ff7 <.text+0x21a5ff7>
// 0x1421a6e59  4c3b4c2430                     cmpq	0x30(%rsp), %r9
// 0x1421a6e5e  0f8493010000                   je	0x1421a6ff7 <.text+0x21a5ff7>
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F27 -> F28 -> F29 -> F4
// 0x1421a6e64  4d8d6120                       leaq	0x20(%r9), %r12
// 0x1421a6e68  4531ff                         xorl	%r15d, %r15d
// 0x1421a6e6b  4c89c9                         movq	%r9, %rcx
// Frames: F36 -> F29 -> F4
// 0x1421a6e6e  488b542458                     movq	0x58(%rsp), %rdx
// 0x1421a6e73  48ff02                         incq	(%rdx)
// Frames: F37
// 0x1421a6e76  4c8b7108                       movq	0x8(%rcx), %r14
// Frames: F38 -> F39 -> F40 -> F37
// 0x1421a6e7a  4489f1                         movl	%r14d, %ecx
// Frames: F41 -> F15 -> F38 -> F39 -> F40 -> F37
// 0x1421a6e7d  493b4d10                       cmpq	0x10(%r13), %rcx
// 0x1421a6e81  0f83b0010000                   jae	0x1421a7037 <.text+0x21a6037>
// Frames: F42 -> F43 -> F44 -> F45 -> F17 -> F46 -> F38 -> F39 -> F40 -> F37
// 0x1421a6e87  498b5508                       movq	0x8(%r13), %rdx
// 0x1421a6e8b  4c89f7                         movq	%r14, %rdi
// 0x1421a6e8e  48c1ef20                       shrq	$0x20, %rdi
// Frames: F47 -> F41 -> F15 -> F38 -> F39 -> F40 -> F37
// 0x1421a6e92  4c8d0489                       leaq	(%rcx,%rcx,4), %r8
// Frames: F48 -> F49 -> F50 -> F39 -> F40 -> F37
// 0x1421a6e96  42393c82                       cmpl	%edi, (%rdx,%r8,4)
// Frames: F50 -> F39 -> F40 -> F37
// 0x1421a6e9a  0f859e010000                   jne	0x1421a703e <.text+0x21a603e>
// Frames: F51 -> F52 -> F39 -> F40 -> F37
// 0x1421a6ea0  4e8d0482                       leaq	(%rdx,%r8,4), %r8
// 0x1421a6ea4  418b5004                       movl	0x4(%r8), %edx
// 0x1421a6ea8  41b9ffffffff                   movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
// 0x1421a6eae  4c39ca                         cmpq	%r9, %rdx
// Frames: F52 -> F39 -> F40 -> F37
// 0x1421a6eb1  0f8487010000                   je	0x1421a703e <.text+0x21a603e>
// Frames: F53 -> F54 -> F40 -> F37
// 0x1421a6eb7  48395538                       cmpq	%rdx, 0x38(%rbp)
// Frames: F55 -> F56 -> F54 -> F40 -> F37
// 0x1421a6ebb  0f866f010000                   jbe	0x1421a7030 <.text+0x21a6030>
// Frames: F57 -> F58 -> F59 -> F55 -> F56 -> F54 -> F40 -> F37
// 0x1421a6ec1  488b4528                       movq	0x28(%rbp), %rax
// Frames: F21 -> F57 -> F58 -> F59 -> F55 -> F56 -> F54 -> F40 -> F37
// 0x1421a6ec5  89d1                           movl	%edx, %ecx
// 0x1421a6ec7  c1e906                         shrl	$0x6, %ecx
// Frames: F60 -> F58 -> F59 -> F55 -> F56 -> F54 -> F40 -> F37
// 0x1421a6eca  488b04c8                       movq	(%rax,%rcx,8), %rax
// Frames: F58 -> F59 -> F55 -> F56 -> F54 -> F40 -> F37
// 0x1421a6ece  480fa3d0                       btq	%rdx, %rax
// Frames: F61 -> F40 -> F37
// 0x1421a6ed2  0f8358010000                   jae	0x1421a7030 <.text+0x21a6030>
// Frames: F42 -> F43 -> F44 -> F45 -> F17 -> F18 -> F62 -> F63 -> F40 -> F37
// 0x1421a6ed8  418b400c                       movl	0xc(%r8), %eax
// 0x1421a6edc  418b4810                       movl	0x10(%r8), %ecx
// 0x1421a6ee0  498b95a8010000                 movq	0x1a8(%r13), %rdx
// Frames: F47 -> F41 -> F64 -> F62 -> F63 -> F40 -> F37
// 0x1421a6ee7  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F65 -> F40 -> F37
// 0x1421a6eeb  4c8b8510010000                 movq	0x110(%rbp), %r8
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F65 -> F40 -> F37
// 0x1421a6ef2  4c8b4cc218                     movq	0x18(%rdx,%rax,8), %r9
// 0x1421a6ef7  488b44c238                     movq	0x38(%rdx,%rax,8), %rax
// Frames: F72 -> F73 -> F74 -> F75 -> F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F65 -> F40 -> F37
// 0x1421a6efc  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F76 -> F77 -> F78 -> F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F65 -> F40 -> F37
// 0x1421a6f00  48f7d0                         notq	%rax
// Frames: F79 -> F80 -> F81 -> F78 -> F82 -> F69 -> F70 -> F71 -> F65 -> F40 -> F37
// 0x1421a6f03  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1421a6f07  48c1e004                       shlq	$0x4, %rax
// 0x1421a6f0b  498b440110                     movq	0x10(%r9,%rax), %rax
// Frames: F21 -> F83 -> F84 -> F85 -> F86 -> F87 -> F88 -> F40 -> F37
// 0x1421a6f10  486bd158                       imulq	$0x58, %rcx, %rdx
// 0x1421a6f14  31c9                           xorl	%ecx, %ecx
// Frames: F89
// 0x1421a6f16  f644100801                     testb	$0x1, 0x8(%rax,%rdx)
// 0x1421a6f1b  753a                           jne	0x1421a6f57 <.text+0x21a5f57>
// Frames: F90
// 0x1421a6f1d  4801d0                         addq	%rdx, %rax
// 0x1421a6f20  f30f104014                     movss	0x14(%rax), %xmm0
// 0x1421a6f25  f30f59c6                       mulss	%xmm6, %xmm0
// Frames: F91 -> F90
// 0x1421a6f29  e822d07700                     callq	0x142923f50 <.text+0x2922f50>
// Frames: F92 -> F90
// 0x1421a6f2e  0f28cf                         movaps	%xmm7, %xmm1
// 0x1421a6f31  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F93 -> F90
// 0x1421a6f35  410f28c0                       movaps	%xmm8, %xmm0
// 0x1421a6f39  f30f5dc1                       minss	%xmm1, %xmm0
// 0x1421a6f3d  31c0                           xorl	%eax, %eax
// Frames: F90
// 0x1421a6f3f  410f2ec1                       ucomiss	%xmm9, %xmm0
// 0x1421a6f43  7205                           jb	0x1421a6f4a <.text+0x21a5f4a>
// 0x1421a6f45  f3480f2cc0                     cvttss2si	%xmm0, %rax
// 0x1421a6f4a  410f2ec2                       ucomiss	%xmm10, %xmm0
// 0x1421a6f4e  b9ffffffff                     movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
// 0x1421a6f53  7702                           ja	0x1421a6f57 <.text+0x21a5f57>
// 0x1421a6f55  89c1                           movl	%eax, %ecx
// Frames: F94
// 0x1421a6f57  e8f4c268fe                     callq	0x140833250 <.text+0x832250>
// Frames: F95 -> F94
// 0x1421a6f5c  488d4c2440                     leaq	0x40(%rsp), %rcx
// 0x1421a6f61  89c2                           movl	%eax, %edx
// 0x1421a6f63  e8f84d68fe                     callq	0x14082bd60 <.text+0x82ad60>
// Frames: F96
// 0x1421a6f68  c644245009                     movb	$0x9, 0x50(%rsp)
// Frames: F97 -> F98 -> F99 -> F100 -> F101 -> F102 -> F103 -> F104 -> F105 -> F96
// 0x1421a6f6d  488b06                         movq	(%rsi), %rax
// 0x1421a6f70  488b5e10                       movq	0x10(%rsi), %rbx
// Frames: F106 -> F98 -> F99 -> F100 -> F101 -> F102 -> F103 -> F104 -> F105 -> F96
// 0x1421a6f74  4829d8                         subq	%rbx, %rax
// Frames: F98 -> F99 -> F100 -> F101 -> F102 -> F103 -> F104 -> F105 -> F96
// 0x1421a6f77  4883f837                       cmpq	$0x37, %rax
// Frames: F99 -> F100 -> F101 -> F102 -> F103 -> F104 -> F105 -> F96
// 0x1421a6f7b  7658                           jbe	0x1421a6fd5 <.text+0x21a5fd5>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F4
// 0x1421a6f7d  488b4608                       movq	0x8(%rsi), %rax
// 0x1421a6f81  488d0d187fc2fe                 leaq	-0x13d80e8(%rip), %rcx  # 0x140dceea0 <.text+0xdcdea0>
// 0x1421a6f88  48890c18                       movq	%rcx, (%rax,%rbx)
// 0x1421a6f8c  0f10442440                     movups	0x40(%rsp), %xmm0
// 0x1421a6f91  0f11441808                     movups	%xmm0, 0x8(%rax,%rbx)
// 0x1421a6f96  488b4c2450                     movq	0x50(%rsp), %rcx
// 0x1421a6f9b  48894c1818                     movq	%rcx, 0x18(%rax,%rbx)
// 0x1421a6fa0  c644182000                     movb	$0x0, 0x20(%rax,%rbx)
// 0x1421a6fa5  4c89741828                     movq	%r14, 0x28(%rax,%rbx)
// 0x1421a6faa  488d0dcf8ac2fe                 leaq	-0x13d7531(%rip), %rcx  # 0x140dcfa80 <.text+0xdcea80>
// 0x1421a6fb1  48894c1830                     movq	%rcx, 0x30(%rax,%rbx)
// 0x1421a6fb6  4883c338                       addq	$0x38, %rbx
// 0x1421a6fba  48895e10                       movq	%rbx, 0x10(%rsi)
// 0x1421a6fbe  4d89e1                         movq	%r12, %r9
// 0x1421a6fc1  4c89f9                         movq	%r15, %rcx
// 0x1421a6fc4  4889f8                         movq	%rdi, %rax
// 0x1421a6fc7  4885c9                         testq	%rcx, %rcx
// 0x1421a6fca  0f8570feffff                   jne	0x1421a6e40 <.text+0x21a5e40>
// 0x1421a6fd0  e97bfeffff                     jmp	0x1421a6e50 <.text+0x21a5e50>
// Frames: F107 -> F100 -> F101 -> F102 -> F103 -> F104 -> F105 -> F96
// 0x1421a6fd5  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421a6fde  41b838000000                   movl	$0x38, %r8d
// 0x1421a6fe4  41b901000000                   movl	$0x1, %r9d
// 0x1421a6fea  4889f1                         movq	%rsi, %rcx
// 0x1421a6fed  4889da                         movq	%rbx, %rdx
// 0x1421a6ff0  e8db3c7300                     callq	0x1428dacd0 <.text+0x28d9cd0>
// 0x1421a6ff5  eb86                           jmp	0x1421a6f7d <.text+0x21a5f7d>
// Frames: F108
// 0x1421a6ff7  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421a6ffc  0f287c2470                     movaps	0x70(%rsp), %xmm7
// 0x1421a7001  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x1421a700a  440f288c2490000000             movaps	0x90(%rsp), %xmm9
// 0x1421a7013  440f289424a0000000             movaps	0xa0(%rsp), %xmm10
// 0x1421a701c  4881c4b8000000                 addq	$0xb8, %rsp
// 0x1421a7023  5b                             popq	%rbx
// 0x1421a7024  5d                             popq	%rbp
// 0x1421a7025  5f                             popq	%rdi
// 0x1421a7026  5e                             popq	%rsi
// 0x1421a7027  415c                           popq	%r12
// 0x1421a7029  415d                           popq	%r13
// 0x1421a702b  415e                           popq	%r14
// 0x1421a702d  415f                           popq	%r15
// 0x1421a702f  c3                             retq
// 0x1421a7030  4889d0                         movq	%rdx, %rax
// 0x1421a7033  31c9                           xorl	%ecx, %ecx
// 0x1421a7035  eb16                           jmp	0x1421a704d <.text+0x21a604d>
// Frames: F109 -> F39 -> F40 -> F37
// 0x1421a7037  49c1ee20                       shrq	$0x20, %r14
// 0x1421a703b  4c89f7                         movq	%r14, %rdi
// Frames: F110 -> F39 -> F40 -> F37
// 0x1421a703e  48c1e720                       shlq	$0x20, %rdi
// 0x1421a7042  4809cf                         orq	%rcx, %rdi
// 0x1421a7045  b901000000                     movl	$0x1, %ecx
// 0x1421a704a  4989fe                         movq	%rdi, %r14
// Frames: F111 -> F37
// 0x1421a704d  48c1e020                       shlq	$0x20, %rax
// 0x1421a7051  4809c8                         orq	%rcx, %rax
// 0x1421a7054  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1421a7059  4c89742448                     movq	%r14, 0x48(%rsp)
// 0x1421a705e  488d050bfce400                 leaq	0xe4fc0b(%rip), %rax    # 0x142ff6c70
// 0x1421a7065  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421a706a  488d0dbffae400                 leaq	0xe4fabf(%rip), %rcx    # 0x142ff6b30
// 0x1421a7071  4c8d0d98fae400                 leaq	0xe4fa98(%rip), %r9     # 0x142ff6b10
// 0x1421a7078  4c8d442440                     leaq	0x40(%rsp), %r8
// 0x1421a707d  ba2b000000                     movl	$0x2b, %edx
// 0x1421a7082  e8d9267300                     callq	0x1428d9760 <.text+0x28d8760>
// 0x1421a7087  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:10:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h336c8f0aeec57486
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1235:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs:28:0 bevy_ecs::event::reader::EventReader::read
// F4: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:15:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h336c8f0aeec57486
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:55:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:89:0 bevy_ecs::event::event_cursor::EventCursor::read_with_id
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:79:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:1860:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:57:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:60:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:659:0 core::num::impl$11::checked_sub
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:366:0 core::slice::index::impl$4::get
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:538:0 core::slice::index::impl$7::get
// F15: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:61:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:62:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:67:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F21: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F22: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F23: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 core::slice::impl$0::iter
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:69:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:729:0 core::option::Option::as_mut
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:333:0 core::iter::adapters::chain::and_then_or_clear
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:89:0 bevy_ecs::event::iterators::impl$3::next
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:15:0 bevy_ecs::event::iterators::impl$0::next
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next::closure$0
// F31: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1552:0 core::option::Option::or_else
// F32: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:617:0 core::ptr::non_null::NonNull::add
// F33: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:108:0 core::slice::iter::Iter::post_inc_start
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:443:0 core::slice::iter::impl$186::next_unchecked
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:165:0 core::slice::iter::impl$182::next
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:95:0 bevy_ecs::event::iterators::impl$3::next
// F37: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:16:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h336c8f0aeec57486
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:843:0 bevy_ecs::entity::Entities::get
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1595:0 bevy_ecs::system::query::Query::get_inner
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1386:0 bevy_ecs::system::query::Query::get
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F42: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F43: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F46: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 core::slice::impl$0::get
// F47: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1742:0 core::cmp::impls::impl$24::ne
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:201:0 core::num::nonzero::impl$2::ne
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:844:0 bevy_ecs::entity::Entities::get
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:76:0 bevy_ecs::archetype::impl$25::eq
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:845:0 bevy_ecs::entity::Entities::get
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1603:0 bevy_ecs::system::query::Query::get_inner
// F55: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/bool.rs:62:0 core::bool::impl$0::then
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F60: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/internal_macros.rs:36:0 fixedbitset::FixedBitSet::get_unchecked
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1600:0 bevy_ecs::system::query::Query::get_inner
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1632:0 bevy_ecs::system::query::Query::get_inner
// F64: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 alloc::vec::impl$8::deref
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1634:0 bevy_ecs::system::query::Query::get_inner
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:367:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1181:0 bevy_ecs::query::fetch::impl$26::set_table
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1169:0 bevy_ecs::query::fetch::impl$26::set_archetype
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F72: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F73: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F74: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::option::Option::and_then
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F78: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:304:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1239:0 bevy_ecs::query::fetch::impl$27::fetch::closure$0
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1234:0 bevy_ecs::query::fetch::impl$27::fetch
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2090:0 bevy_ecs::query::fetch::impl$61::fetch
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1638:0 bevy_ecs::system::query::Query::get_inner
// F89: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:18:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h336c8f0aeec57486
// F90: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:20:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h336c8f0aeec57486
// F91: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:503:0 std::f32::impl$0::log2
// F92: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1418:0 core::f32::impl$0::clamp
// F93: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1421:0 core::f32::impl$0::clamp
// F94: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:28:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h336c8f0aeec57486
// F95: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/convert/mod.rs:761:0 core::convert::impl$3::into
// F96: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:27:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h336c8f0aeec57486
// F97: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:513:0 alloc::raw_vec::RawVecInner::capacity
// F98: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:623:0 alloc::raw_vec::RawVecInner::needs_to_grow
// F99: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:558:0 alloc::raw_vec::RawVecInner::reserve
// F100: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:327:0 alloc::raw_vec::RawVec::reserve
// F101: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1272:0 alloc::vec::Vec::reserve
// F102: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:0:0 bevy_ecs::system::commands::Commands::queue_internal
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:554:0 bevy_ecs::system::commands::Commands::queue
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:1802:0 bevy_ecs::system::commands::EntityCommands::queue
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:1348:0 bevy_ecs::system::commands::EntityCommands::insert
// F106: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:2022:0 alloc::raw_vec::RawVecInner::capacity
// F107: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:559:0 alloc::raw_vec::RawVecInner::reserve
// F108: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:100:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h336c8f0aeec57486
// F109: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:2003:0 core::result::impl$26::branch
// F110: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:2014:0 core::result::impl$27::from_residual
// F111: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1109:0 core::result::Result::unwrap
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 682a63017ef2b107b18be0e2e76c0b4e16a846124607411cbf3090b15dd47b22
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: generate_roof_stone_floor_and_roof_bottom.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3463
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3463,"record_offset":372,"rva":35286336,"va":5403995456}
// 编译实例 RVAs: 0x21a6d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14082bd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26718728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403995456,"raw_hex":"e8f84d68fe","size":5,"soft_fail":false,"target_va":5377277280,"va":5403996003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26688780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403995456,"raw_hex":"e8f4c268fe","size":5,"soft_fail":false,"target_va":5377307216,"va":5403995991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7852066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403995456,"raw_hex":"e822d07700","size":5,"soft_fail":false,"target_va":5411848016,"va":5403995945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7552219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403995456,"raw_hex":"e8db3c7300","size":5,"soft_fail":false,"target_va":5411548368,"va":5403996144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7546585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403995456,"raw_hex":"e8d9267300","size":5,"soft_fail":false,"target_va":5411542880,"va":5403996290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3463,"record_offset":1288,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3463,"record_offset":592,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3463,"record_offset":3340,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3463,"record_offset":1652,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3463,"record_offset":3312,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3463,"record_offset":3280,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3463,"record_offset":3632,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::log2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3463,"record_offset":3252,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":7675,"source_line":502,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3463,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":372,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3463,"record_offset":3480,"parent_record_offset":3452,"ancestor_record_offset":372,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3463,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":372,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3463,"record_offset":3540,"parent_record_offset":3508,"ancestor_record_offset":372,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3463,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":372,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3463,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":372,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3463,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":372,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3463,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":372,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3463,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":372,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3463,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":372,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3463,"record_offset":908,"parent_record_offset":680,"ancestor_record_offset":372,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3463,"record_offset":2068,"parent_record_offset":1736,"ancestor_record_offset":372,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3463,"record_offset":1736,"parent_record_offset":1684,"ancestor_record_offset":372,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3463,"record_offset":656,"parent_record_offset":616,"ancestor_record_offset":372,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3463,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":372,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3463,"record_offset":1316,"parent_record_offset":1288,"ancestor_record_offset":372,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3463,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":372,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3463,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":372,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3463,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":372,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3463,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":372,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3463,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":372,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3463,"record_offset":3020,"parent_record_offset":1684,"ancestor_record_offset":372,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3463,"record_offset":2540,"parent_record_offset":1684,"ancestor_record_offset":372,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3463,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":372,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3463,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":372,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3463,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":372,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3463,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":372,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3463,"record_offset":2640,"parent_record_offset":2612,"ancestor_record_offset":372,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3463,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":372,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3463,"record_offset":2280,"parent_record_offset":1684,"ancestor_record_offset":372,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3463,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":372,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3463,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":372,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3463,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":372,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3463,"record_offset":3424,"parent_record_offset":3396,"ancestor_record_offset":372,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3463,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":372,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3463,"record_offset":1684,"parent_record_offset":1652,"ancestor_record_offset":372,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3463,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":372,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3463,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":372,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3463,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":372,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3463,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":372,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3463,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":372,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3463,"record_offset":1352,"parent_record_offset":1316,"ancestor_record_offset":372,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3463,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":372,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3463,"record_offset":864,"parent_record_offset":836,"ancestor_record_offset":372,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3463,"record_offset":724,"parent_record_offset":680,"ancestor_record_offset":372,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3463,"record_offset":2012,"parent_record_offset":1736,"ancestor_record_offset":372,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3463,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":372,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3463,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":372,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3463,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":372,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3463,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":372,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3463,"record_offset":2800,"parent_record_offset":2664,"ancestor_record_offset":372,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3463,"record_offset":1444,"parent_record_offset":1352,"ancestor_record_offset":372,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3463,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":372,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3463,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":372,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3463,"record_offset":3188,"parent_record_offset":1684,"ancestor_record_offset":372,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3463,"record_offset":3216,"parent_record_offset":1684,"ancestor_record_offset":372,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3463,"record_offset":780,"parent_record_offset":680,"ancestor_record_offset":372,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3463,"record_offset":1132,"parent_record_offset":680,"ancestor_record_offset":372,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3463,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":372,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3463,"record_offset":1808,"parent_record_offset":1768,"ancestor_record_offset":372,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3463,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":372,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3463,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":372,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3463,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":372,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3463,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":372,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3463,"record_offset":1520,"parent_record_offset":1468,"ancestor_record_offset":372,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3463,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":372,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3463,"record_offset":2100,"parent_record_offset":1684,"ancestor_record_offset":372,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3463,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":372,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3463,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":372,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3463,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":372,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03463.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3463,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":372,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03463.jsonl.gz"}
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
// 完整结构及原始类型记录: generate_roof_stone_floor_and_roof_bottom.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: generate_roof_stone_floor_and_roof_bottom.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: generate_roof_stone_floor_and_roof_bottom.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// 信息 ID: 57fec53f3496de062c93a971
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ea165d21b691762bea950615: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > > @ 0x1a2cf34 -> 0x21a6d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0x1a376cc -> 0x21a6d40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
