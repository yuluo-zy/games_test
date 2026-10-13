// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: generate_roof_stone_floor_and_roof_bottom.rs.assembly.json.gz

// FUNCTION system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// Candidate interfaces: 48530805ba8ff59d838ea481
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3193:512 -> rva_1987840_len_3cd
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1987840_len_3cd: RVA 0x1987840, 973 PDB bytes; 228 instructions
// Evidence: _assembly/roof-v1/A/pack-002/rva_1987840_len_3cd.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3193:512: system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// Frames: F1
// 0x141987840  4157                           pushq	%r15
// 0x141987842  4156                           pushq	%r14
// 0x141987844  4155                           pushq	%r13
// 0x141987846  4154                           pushq	%r12
// 0x141987848  56                             pushq	%rsi
// 0x141987849  57                             pushq	%rdi
// 0x14198784a  55                             pushq	%rbp
// 0x14198784b  53                             pushq	%rbx
// 0x14198784c  4881ecd8000000                 subq	$0xd8, %rsp
// 0x141987853  440f299424c0000000             movaps	%xmm10, 0xc0(%rsp)
// 0x14198785c  440f298c24b0000000             movaps	%xmm9, 0xb0(%rsp)
// 0x141987865  440f298424a0000000             movaps	%xmm8, 0xa0(%rsp)
// 0x14198786e  0f29bc2490000000               movaps	%xmm7, 0x90(%rsp)
// 0x141987876  0f29b42480000000               movaps	%xmm6, 0x80(%rsp)
// Frames: F2 -> F3 -> F4
// 0x14198787e  488b39                         movq	(%rcx), %rdi
// Frames: F5 -> F3 -> F4
// 0x141987881  4c8b4908                       movq	0x8(%rcx), %r9
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x141987885  4c8b1f                         movq	(%rdi), %r11
// 0x141987888  4531d2                         xorl	%r10d, %r10d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x14198788b  4c89d9                         movq	%r11, %rcx
// 0x14198788e  492b4918                       subq	0x18(%r9), %rcx
// 0x141987892  490f42ca                       cmovbq	%r10, %rcx
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x141987896  4d2b5938                       subq	0x38(%r9), %r11
// 0x14198789a  4d0f42da                       cmovbq	%r10, %r11
// Frames: F12 -> F13 -> F14 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x14198789e  4889c8                         movq	%rcx, %rax
// 0x1419878a1  48c1e005                       shlq	$0x5, %rax
// 0x1419878a5  49034108                       addq	0x8(%r9), %rax
// Frames: F15 -> F7 -> F8 -> F3 -> F4
// 0x1419878a9  498b5910                       movq	0x10(%r9), %rbx
// Frames: F16 -> F7 -> F8 -> F3 -> F4
// 0x1419878ad  4d8b7130                       movq	0x30(%r9), %r14
// Frames: F17 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x1419878b1  4829cb                         subq	%rcx, %rbx
// 0x1419878b4  490f42da                       cmovbq	%r10, %rbx
// 0x1419878b8  488d35f1099500                 leaq	0x9509f1(%rip), %rsi    # 0x1422d82b0
// 0x1419878bf  480f42c6                       cmovbq	%rsi, %rax
// Frames: F12 -> F13 -> F14 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1419878c3  4c89d9                         movq	%r11, %rcx
// 0x1419878c6  48c1e105                       shlq	$0x5, %rcx
// 0x1419878ca  49034928                       addq	0x28(%r9), %rcx
// 0x1419878ce  4d39f3                         cmpq	%r14, %r11
// Frames: F17 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1419878d1  480f47ce                       cmovaq	%rsi, %rcx
// 0x1419878d5  4d29de                         subq	%r11, %r14
// 0x1419878d8  4d0f42f2                       cmovbq	%r10, %r14
// Frames: F18 -> F7 -> F8 -> F3 -> F4
// 0x1419878dc  4d8b4940                       movq	0x40(%r9), %r9
// 0x1419878e0  4d29f1                         subq	%r14, %r9
// 0x1419878e3  4929d9                         subq	%rbx, %r9
// 0x1419878e6  48897c2448                     movq	%rdi, 0x48(%rsp)
// 0x1419878eb  4c890f                         movq	%r9, (%rdi)
// Frames: F19 -> F20 -> F21 -> F22 -> F7 -> F8 -> F3 -> F4
// 0x1419878ee  48c1e305                       shlq	$0x5, %rbx
// 0x1419878f2  4801c3                         addq	%rax, %rbx
// 0x1419878f5  48895c2438                     movq	%rbx, 0x38(%rsp)
// 0x1419878fa  49c1e605                       shlq	$0x5, %r14
// 0x1419878fe  4901ce                         addq	%rcx, %r14
// 0x141987901  4c89742430                     movq	%r14, 0x30(%rsp)
// 0x141987906  4c8b32                         movq	(%rdx), %r14
// 0x141987909  488b5a08                       movq	0x8(%rdx), %rbx
// 0x14198790d  498b10                         movq	(%r8), %rdx
// 0x141987910  4889542440                     movq	%rdx, 0x40(%rsp)
// 0x141987915  498b7808                       movq	0x8(%r8), %rdi
// 0x141987919  498b5010                       movq	0x10(%r8), %rdx
// 0x14198791d  4889542428                     movq	%rdx, 0x28(%rsp)
// 0x141987922  f30f10359e108400               movss	0x84109e(%rip), %xmm6   # 0x1421c89c8
// 0x14198792a  f30f103d8e0e7c00               movss	0x7c0e8e(%rip), %xmm7   # 0x1421487c0
// 0x141987932  f3440f1005ddb98a00             movss	0x8ab9dd(%rip), %xmm8   # 0x142233318
// 0x14198793b  450f57c9                       xorps	%xmm9, %xmm9
// 0x14198793f  f3440f1015f8f77f00             movss	0x7ff7f8(%rip), %xmm10  # 0x142187140
// 0x141987948  eb51                           jmp	0x14198799b <.text+0x198699b>
// 0x14198794a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x141987950  488b07                         movq	(%rdi), %rax
// Frames: F33 -> F34 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x141987953  4c8b6f10                       movq	0x10(%rdi), %r13
// Frames: F35 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x141987957  4c29e8                         subq	%r13, %rax
// Frames: F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x14198795a  4883f827                       cmpq	$0x27, %rax
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x14198795e  0f86fc010000                   jbe	0x141987b60 <.text+0x1986b60>
// Frames: F36 -> F37 -> F38 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x141987964  488b4708                       movq	0x8(%rdi), %rax
// Frames: F39 -> F40 -> F41 -> F42 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x141987968  488d0d9145f4fe                 leaq	-0x10bba6f(%rip), %rcx  # 0x1408cbf00 <.text+0x8caf00>
// 0x14198796f  4a890c28                       movq	%rcx, (%rax,%r13)
// 0x141987973  0f10442450                     movups	0x50(%rsp), %xmm0
// 0x141987978  420f11442808                   movups	%xmm0, 0x8(%rax,%r13)
// 0x14198797e  488b4c2460                     movq	0x60(%rsp), %rcx
// 0x141987983  4a894c2818                     movq	%rcx, 0x18(%rax,%r13)
// 0x141987988  4e89642820                     movq	%r12, 0x20(%rax,%r13)
// Frames: F43 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x14198798d  4983c528                       addq	$0x28, %r13
// Frames: F44 -> F43 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x141987991  4c896f10                       movq	%r13, 0x10(%rdi)
// 0x141987995  4c89f9                         movq	%r15, %rcx
// 0x141987998  4889f0                         movq	%rsi, %rax
// Frames: F45 -> F46 -> F47 -> F48 -> F49 -> F4
// 0x14198799b  4885c0                         testq	%rax, %rax
// 0x14198799e  7410                           je	0x1419879b0 <.text+0x19869b0>
// 0x1419879a0  483b442438                     cmpq	0x38(%rsp), %rax
// 0x1419879a5  7409                           je	0x1419879b0 <.text+0x19869b0>
// 0x1419879a7  488d7020                       leaq	0x20(%rax), %rsi
// 0x1419879ab  4989cf                         movq	%rcx, %r15
// 0x1419879ae  eb1d                           jmp	0x1419879cd <.text+0x19869cd>
// Frames: F45 -> F50 -> F51 -> F47 -> F48 -> F49 -> F4
// 0x1419879b0  4885c9                         testq	%rcx, %rcx
// 0x1419879b3  0f84d0010000                   je	0x141987b89 <.text+0x1986b89>
// 0x1419879b9  483b4c2430                     cmpq	0x30(%rsp), %rcx
// 0x1419879be  0f84c5010000                   je	0x141987b89 <.text+0x1986b89>
// Frames: F52 -> F53 -> F54 -> F50 -> F51 -> F47 -> F48 -> F49 -> F4
// 0x1419879c4  4c8d7920                       leaq	0x20(%rcx), %r15
// 0x1419879c8  31f6                           xorl	%esi, %esi
// 0x1419879ca  4889c8                         movq	%rcx, %rax
// Frames: F55 -> F49 -> F4
// 0x1419879cd  488b4c2448                     movq	0x48(%rsp), %rcx
// 0x1419879d2  48ff01                         incq	(%rcx)
// Frames: F56
// 0x1419879d5  4c8b6008                       movq	0x8(%rax), %r12
// Frames: F57 -> F58 -> F59 -> F56
// 0x1419879d9  4489e1                         movl	%r12d, %ecx
// 0x1419879dc  b801000000                     movl	$0x1, %eax
// Frames: F60 -> F14 -> F57 -> F58 -> F59 -> F56
// 0x1419879e1  483b4b10                       cmpq	0x10(%rbx), %rcx
// Frames: F57 -> F58 -> F59 -> F56
// 0x1419879e5  0f83df010000                   jae	0x141987bca <.text+0x1986bca>
// Frames: F61 -> F62 -> F63 -> F58 -> F59 -> F56
// 0x1419879eb  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x1419879ef  48c1e102                       shlq	$0x2, %rcx
// 0x1419879f3  48034b08                       addq	0x8(%rbx), %rcx
// 0x1419879f7  4c89e2                         movq	%r12, %rdx
// 0x1419879fa  48c1ea20                       shrq	$0x20, %rdx
// 0x1419879fe  3911                           cmpl	%edx, (%rcx)
// Frames: F63 -> F58 -> F59 -> F56
// 0x141987a00  0f85c4010000                   jne	0x141987bca <.text+0x1986bca>
// Frames: F64 -> F65 -> F58 -> F59 -> F56
// 0x141987a06  8b5104                         movl	0x4(%rcx), %edx
// 0x141987a09  41b8ffffffff                   movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
// 0x141987a0f  4c39c2                         cmpq	%r8, %rdx
// Frames: F65 -> F58 -> F59 -> F56
// 0x141987a12  0f84b2010000                   je	0x141987bca <.text+0x1986bca>
// Frames: F66 -> F67 -> F59 -> F56
// 0x141987a18  49395638                       cmpq	%rdx, 0x38(%r14)
// Frames: F68 -> F69 -> F67 -> F59 -> F56
// 0x141987a1c  0f86a6010000                   jbe	0x141987bc8 <.text+0x1986bc8>
// Frames: F67 -> F59 -> F56
// 0x141987a22  498b4628                       movq	0x28(%r14), %rax
// Frames: F19 -> F70 -> F71 -> F72 -> F68 -> F69 -> F67 -> F59 -> F56
// 0x141987a26  4189d0                         movl	%edx, %r8d
// 0x141987a29  41c1e806                       shrl	$0x6, %r8d
// Frames: F73 -> F71 -> F72 -> F68 -> F69 -> F67 -> F59 -> F56
// 0x141987a2d  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F71 -> F72 -> F68 -> F69 -> F67 -> F59 -> F56
// 0x141987a31  480fa3d0                       btq	%rdx, %rax
// Frames: F67 -> F59 -> F56
// 0x141987a35  0f838d010000                   jae	0x141987bc8 <.text+0x1986bc8>
// Frames: F74 -> F58 -> F59 -> F56
// 0x141987a3b  488b4108                       movq	0x8(%rcx), %rax
// 0x141987a3f  8b4910                         movl	0x10(%rcx), %ecx
// 0x141987a42  48c1e820                       shrq	$0x20, %rax
// Frames: F75 -> F76 -> F59 -> F56
// 0x141987a46  488b9328010000                 movq	0x128(%rbx), %rdx
// Frames: F77 -> F59 -> F56
// 0x141987a4d  4d8b86d8000000                 movq	0xd8(%r14), %r8
// Frames: F78 -> F79 -> F80 -> F81 -> F77 -> F59 -> F56
// 0x141987a54  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F82 -> F78 -> F79 -> F80 -> F81 -> F77 -> F59 -> F56
// 0x141987a58  4c8b4cc218                     movq	0x18(%rdx,%rax,8), %r9
// 0x141987a5d  488b44c238                     movq	0x38(%rdx,%rax,8), %rax
// Frames: F83 -> F84 -> F85 -> F86 -> F82 -> F78 -> F79 -> F80 -> F81 -> F77 -> F59 -> F56
// 0x141987a62  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F87 -> F88 -> F85 -> F82 -> F78 -> F79 -> F80 -> F81 -> F77 -> F59 -> F56
// 0x141987a66  48f7d0                         notq	%rax
// Frames: F89 -> F90 -> F91 -> F80 -> F81 -> F77 -> F59 -> F56
// 0x141987a69  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141987a6d  48c1e005                       shlq	$0x5, %rax
// 0x141987a71  498b440110                     movq	0x10(%r9,%rax), %rax
// Frames: F19 -> F92 -> F93 -> F94 -> F95 -> F59 -> F56
// 0x141987a76  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x141987a7a  48c1e104                       shlq	$0x4, %rcx
// 0x141987a7e  31ed                           xorl	%ebp, %ebp
// Frames: F96
// 0x141987a80  837c080800                     cmpl	$0x0, 0x8(%rax,%rcx)
// 0x141987a85  7549                           jne	0x141987ad0 <.text+0x1986ad0>
// Frames: F97
// 0x141987a87  4801c8                         addq	%rcx, %rax
// 0x141987a8a  f30f104014                     movss	0x14(%rax), %xmm0
// 0x141987a8f  f30f59c6                       mulss	%xmm6, %xmm0
// Frames: F98 -> F99 -> F97
// 0x141987a93  e848d37b00                     callq	0x142144de0 <.text+0x2143de0>
// Frames: F100 -> F97
// 0x141987a98  0f28cf                         movaps	%xmm7, %xmm1
// 0x141987a9b  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F101 -> F97
// 0x141987a9f  410f28c0                       movaps	%xmm8, %xmm0
// 0x141987aa3  f30f5dc1                       minss	%xmm1, %xmm0
// 0x141987aa7  31c0                           xorl	%eax, %eax
// Frames: F97
// 0x141987aa9  410f2ec1                       ucomiss	%xmm9, %xmm0
// 0x141987aad  7205                           jb	0x141987ab4 <.text+0x1986ab4>
// 0x141987aaf  f3480f2cc0                     cvttss2si	%xmm0, %rax
// 0x141987ab4  410f2ec2                       ucomiss	%xmm10, %xmm0
// 0x141987ab8  bdffffffff                     movl	$0xffffffff, %ebp       # imm = 0xFFFFFFFF
// 0x141987abd  7711                           ja	0x141987ad0 <.text+0x1986ad0>
// 0x141987abf  89c5                           movl	%eax, %ebp
// 0x141987ac1  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F102 -> F103 -> F32
// 0x141987ad0  488b4c2440                     movq	0x40(%rsp), %rcx
// 0x141987ad5  4c89e2                         movq	%r12, %rdx
// 0x141987ad8  e893c5e9fe                     callq	0x140824070 <.text+0x823070>
// Frames: F104 -> F102 -> F103 -> F32
// 0x141987add  84c0                           testb	%al, %al
// 0x141987adf  0f8418010000                   je	0x141987bfd <.text+0x1986bfd>
// Frames: F105
// 0x141987ae5  89e9                           movl	%ebp, %ecx
// 0x141987ae7  e864b7c0fe                     callq	0x140593250 <.text+0x592250>
// Frames: F106 -> F105
// 0x141987aec  488d4c2450                     leaq	0x50(%rsp), %rcx
// 0x141987af1  89c2                           movl	%eax, %edx
// 0x141987af3  e8b88acdfe                     callq	0x1406605b0 <.text+0x65f5b0>
// Frames: F32
// 0x141987af8  c644246009                     movb	$0x9, 0x60(%rsp)
// 0x141987afd  4885ff                         testq	%rdi, %rdi
// Frames: F107 -> F29 -> F30 -> F31 -> F32
// 0x141987b00  0f854afeffff                   jne	0x141987950 <.text+0x1986950>
// Frames: F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b06  488d4c2468                     leaq	0x68(%rsp), %rcx
// 0x141987b0b  488b542428                     movq	0x28(%rsp), %rdx
// 0x141987b10  e82bcae9fe                     callq	0x140824540 <.text+0x823540>
// 0x141987b15  4c8b6c2468                     movq	0x68(%rsp), %r13
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b1a  498b4500                       movq	(%r13), %rax
// Frames: F33 -> F34 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b1e  498b6d10                       movq	0x10(%r13), %rbp
// Frames: F35 -> F24 -> F25 -> F26 -> F27 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b22  4829e8                         subq	%rbp, %rax
// Frames: F24 -> F25 -> F26 -> F27 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b25  4883f827                       cmpq	$0x27, %rax
// Frames: F25 -> F26 -> F27 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b29  764b                           jbe	0x141987b76 <.text+0x1986b76>
// Frames: F36 -> F37 -> F38 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b2b  498b4508                       movq	0x8(%r13), %rax
// Frames: F39 -> F40 -> F41 -> F42 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b2f  488d0dca43f4fe                 leaq	-0x10bbc36(%rip), %rcx  # 0x1408cbf00 <.text+0x8caf00>
// 0x141987b36  48890c28                       movq	%rcx, (%rax,%rbp)
// 0x141987b3a  0f10442450                     movups	0x50(%rsp), %xmm0
// 0x141987b3f  0f11442808                     movups	%xmm0, 0x8(%rax,%rbp)
// 0x141987b44  488b4c2460                     movq	0x60(%rsp), %rcx
// 0x141987b49  48894c2818                     movq	%rcx, 0x18(%rax,%rbp)
// 0x141987b4e  4c89642820                     movq	%r12, 0x20(%rax,%rbp)
// Frames: F43 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b53  4883c528                       addq	$0x28, %rbp
// Frames: F44 -> F43 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b57  49896d10                       movq	%rbp, 0x10(%r13)
// 0x141987b5b  e935feffff                     jmp	0x141987995 <.text+0x1986995>
// Frames: F110 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32
// 0x141987b60  41b828000000                   movl	$0x28, %r8d
// 0x141987b66  4889f9                         movq	%rdi, %rcx
// 0x141987b69  4c89ea                         movq	%r13, %rdx
// 0x141987b6c  e82f857100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x141987b71  e9eefdffff                     jmp	0x141987964 <.text+0x1986964>
// Frames: F110 -> F26 -> F27 -> F108 -> F109 -> F29 -> F30 -> F31 -> F32
// 0x141987b76  41b828000000                   movl	$0x28, %r8d
// 0x141987b7c  4c89e9                         movq	%r13, %rcx
// 0x141987b7f  4889ea                         movq	%rbp, %rdx
// 0x141987b82  e819857100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x141987b87  eba2                           jmp	0x141987b2b <.text+0x1986b2b>
// Frames: F111
// 0x141987b89  0f28b42480000000               movaps	0x80(%rsp), %xmm6
// 0x141987b91  0f28bc2490000000               movaps	0x90(%rsp), %xmm7
// 0x141987b99  440f288424a0000000             movaps	0xa0(%rsp), %xmm8
// 0x141987ba2  440f288c24b0000000             movaps	0xb0(%rsp), %xmm9
// 0x141987bab  440f289424c0000000             movaps	0xc0(%rsp), %xmm10
// 0x141987bb4  4881c4d8000000                 addq	$0xd8, %rsp
// 0x141987bbb  5b                             popq	%rbx
// 0x141987bbc  5d                             popq	%rbp
// 0x141987bbd  5f                             popq	%rdi
// 0x141987bbe  5e                             popq	%rsi
// 0x141987bbf  415c                           popq	%r12
// 0x141987bc1  415d                           popq	%r13
// 0x141987bc3  415e                           popq	%r14
// 0x141987bc5  415f                           popq	%r15
// 0x141987bc7  c3                             retq
// 0x141987bc8  31c0                           xorl	%eax, %eax
// Frames: F112 -> F56
// 0x141987bca  4889442468                     movq	%rax, 0x68(%rsp)
// 0x141987bcf  4c89642470                     movq	%r12, 0x70(%rsp)
// 0x141987bd4  488d051db2d600                 leaq	0xd6b21d(%rip), %rax    # 0x1426f2df8
// 0x141987bdb  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141987be0  488d0d21b1d600                 leaq	0xd6b121(%rip), %rcx    # 0x1426f2d08
// 0x141987be7  4c8d0d4ab1d600                 leaq	0xd6b14a(%rip), %r9     # 0x1426f2d38
// 0x141987bee  4c8d442468                     leaq	0x68(%rsp), %r8
// 0x141987bf3  ba2b000000                     movl	$0x2b, %edx
// 0x141987bf8  e823737200                     callq	0x1420aef20 <.text+0x20adf20>
// Frames: F113 -> F32
// 0x141987bfd  488d150cb2d600                 leaq	0xd6b20c(%rip), %rdx    # 0x1426f2e10
// 0x141987c04  4c89e1                         movq	%r12, %rcx
// 0x141987c07  e8a4537200                     callq	0x1420acfb0 <.text+0x20abfb0>
// 0x141987c0c  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:10:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h458ba166bf04b41d
// F2: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:806:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:469:0 bevy_ecs::event::EventReader::read
// F4: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:15:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h458ba166bf04b41d
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
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:271:0 alloc::raw_vec::RawVec::capacity
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:415:0 alloc::raw_vec::RawVec::needs_to_grow
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:332:0 alloc::raw_vec::RawVec::reserve
// F26: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:945:0 alloc::vec::Vec::reserve
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:200:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F28: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:510:0 bevy_ecs::system::commands::Commands::push
// F29: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:749:0 bevy_ecs::system::commands::Commands::add
// F30: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:1101:0 bevy_ecs::system::commands::EntityCommands::add
// F31: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:946:0 bevy_ecs::system::commands::EntityCommands::insert
// F32: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:27:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h458ba166bf04b41d
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2203:0 alloc::vec::Vec::len
// F34: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:197:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F35: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 alloc::raw_vec::RawVec::capacity
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F37: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F38: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:204:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2987:0 core::intrinsics::copy_nonoverlapping
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1584:0 core::ptr::write_unaligned
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1551:0 core::ptr::mut_ptr::impl$0::write_unaligned
// F42: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:213:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F43: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:220:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F44: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1458:0 alloc::vec::Vec::set_len
// F45: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:705:0 core::option::Option::as_mut
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:300:0 core::iter::adapters::chain::and_then_or_clear
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next
// F48: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:845:0 bevy_ecs::event::impl$23::next
// F49: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:771:0 bevy_ecs::event::impl$20::next
// F50: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next::closure$0
// F51: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1513:0 core::option::Option::or_else
// F52: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F53: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F55: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:850:0 bevy_ecs::event::impl$23::next
// F56: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:16:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h458ba166bf04b41d
// F57: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:740:0 bevy_ecs::entity::Entities::get
// F58: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:866:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F59: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:850:0 bevy_ecs::system::query::Query::get
// F60: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F61: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1488:0 core::cmp::impls::impl$24::ne
// F62: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/nonzero.rs:137:0 core::num::nonzero::impl$2::ne
// F63: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:741:0 bevy_ecs::entity::Entities::get
// F64: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:74:0 bevy_ecs::archetype::impl$24::eq
// F65: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:742:0 bevy_ecs::entity::Entities::get
// F66: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F67: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:870:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F68: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 core::bool::impl$0::then
// F69: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F70: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F71: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F72: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F73: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/bit.rs:178:0 fixedbitset::FixedBitSet::get_unchecked
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1952:0 core::result::impl$26::branch
// F75: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F76: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:886:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F77: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:888:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F78: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:693:0 bevy_ecs::storage::table::Table::get_column
// F79: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:955:0 bevy_ecs::query::fetch::impl$21::set_table
// F80: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:942:0 bevy_ecs::query::fetch::impl$21::set_archetype
// F81: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:188:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F82: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F83: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F84: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::impl$11::get::closure$0
// F85: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F86: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F87: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs:398:0 nonmax::NonMaxUsize::get
// F88: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:372:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F89: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:412:0 bevy_ecs::storage::blob_vec::BlobVec::get_slice
// F90: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:322:0 bevy_ecs::storage::table::Column::get_data_slice
// F91: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:957:0 bevy_ecs::query::fetch::impl$21::set_table
// F92: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs:506:0 bevy_ptr::ThinSlicePtr::get
// F93: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:973:0 bevy_ecs::query::fetch::impl$21::fetch
// F94: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:208:0 bevy_ecs::query::world_query::impl$2::fetch
// F95: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:892:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F96: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:18:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h458ba166bf04b41d
// F97: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:20:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h458ba166bf04b41d
// F98: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/pal/mod.rs:87:0 std::sys::pal::log2f32
// F99: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:570:0 std::f32::impl$0::log2
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1521:0 core::f32::impl$0::clamp
// F101: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1524:0 core::f32::impl$0::clamp
// F102: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:448:0 bevy_ecs::system::commands::Commands::get_entity
// F103: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:408:0 bevy_ecs::system::commands::Commands::entity
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:34:0 core::bool::impl$0::then_some
// F105: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:28:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h458ba166bf04b41d
// F106: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/convert/mod.rs:759:0 core::convert::impl$3::into
// F107: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:502:0 bevy_ecs::system::commands::Commands::push
// F108: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:84:0 bevy_ecs::world::command_queue::CommandQueue::push
// F109: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:504:0 bevy_ecs::system::commands::Commands::push
// F110: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:333:0 alloc::raw_vec::RawVec::reserve
// F111: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:100:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h458ba166bf04b41d
// F112: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1077:0 core::result::Result::unwrap
// F113: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:410:0 bevy_ecs::system::commands::Commands::entity
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 45af26093512070499fe6b04b917221b5bf3bd79faaec725b39de61b12542009
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: generate_roof_stone_floor_and_roof_bottom.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3193
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
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
// 首条位置: {"module":3193,"record_offset":512,"rva":26769472,"va":5395478592}
// 编译实例 RVAs: 0x1987840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20924572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395478592,"raw_hex":"e864b7c0fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5395479271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406605b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20084040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395478592,"raw_hex":"e8b88acdfe","size":5,"soft_fail":false,"target_va":5375395248,"va":5395479283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144de0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8115016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395478592,"raw_hex":"e848d37b00","size":5,"soft_fail":false,"target_va":5403594208,"va":5395479187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7439663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395478592,"raw_hex":"e82f857100","size":5,"soft_fail":false,"target_va":5402919072,"va":5395479404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18233965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395478592,"raw_hex":"e893c5e9fe","size":5,"soft_fail":false,"target_va":5377245296,"va":5395479256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7492516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395478592,"raw_hex":"e8a4537200","size":5,"soft_fail":false,"target_va":5402972080,"va":5395479559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18232789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395478592,"raw_hex":"e82bcae9fe","size":5,"soft_fail":false,"target_va":5377246528,"va":5395479312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7500579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395478592,"raw_hex":"e823737200","size":5,"soft_fail":false,"target_va":5402980128,"va":5395479544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3193,"record_offset":736,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3193,"record_offset":2248,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3193,"record_offset":3652,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3193,"record_offset":1272,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3193,"record_offset":2560,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3193,"record_offset":3744,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3193,"record_offset":3620,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3193,"record_offset":3772,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::log2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3193,"record_offset":3564,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":6459,"source_line":569,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3193,"record_offset":1552,"parent_record_offset":1524,"ancestor_record_offset":512,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3193,"record_offset":1524,"parent_record_offset":1488,"ancestor_record_offset":512,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3193,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":512,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3193,"record_offset":1488,"parent_record_offset":1456,"ancestor_record_offset":512,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3193,"record_offset":1632,"parent_record_offset":1408,"ancestor_record_offset":512,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3193,"record_offset":1604,"parent_record_offset":1408,"ancestor_record_offset":512,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3193,"record_offset":1456,"parent_record_offset":1408,"ancestor_record_offset":512,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3193,"record_offset":1772,"parent_record_offset":1408,"ancestor_record_offset":512,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3193,"record_offset":2772,"parent_record_offset":2628,"ancestor_record_offset":512,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3193,"record_offset":2628,"parent_record_offset":2584,"ancestor_record_offset":512,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3193,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":512,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3193,"record_offset":800,"parent_record_offset":760,"ancestor_record_offset":512,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3193,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":512,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3193,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":512,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3193,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":512,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3193,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":512,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3193,"record_offset":2584,"parent_record_offset":2560,"ancestor_record_offset":512,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3193,"record_offset":3444,"parent_record_offset":2584,"ancestor_record_offset":512,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3193,"record_offset":3048,"parent_record_offset":2584,"ancestor_record_offset":512,"inlinee":21665,"source_line":179,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3193,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":512,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3193,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":512,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3193,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":512,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3193,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":512,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3193,"record_offset":3308,"parent_record_offset":3284,"ancestor_record_offset":512,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3193,"record_offset":3376,"parent_record_offset":3096,"ancestor_record_offset":512,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3193,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":512,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3193,"record_offset":3020,"parent_record_offset":2584,"ancestor_record_offset":512,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3193,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":512,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3193,"record_offset":3684,"parent_record_offset":3652,"ancestor_record_offset":512,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3193,"record_offset":1368,"parent_record_offset":1336,"ancestor_record_offset":512,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3193,"record_offset":1304,"parent_record_offset":1272,"ancestor_record_offset":512,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3193,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":512,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3193,"record_offset":1804,"parent_record_offset":1368,"ancestor_record_offset":512,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3193,"record_offset":1408,"parent_record_offset":1368,"ancestor_record_offset":512,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3193,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":512,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3193,"record_offset":2836,"parent_record_offset":2804,"ancestor_record_offset":512,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3193,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":512,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3193,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":512,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3193,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":512,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3193,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":512,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3193,"record_offset":2300,"parent_record_offset":2272,"ancestor_record_offset":512,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3193,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":512,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3193,"record_offset":868,"parent_record_offset":824,"ancestor_record_offset":512,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3193,"record_offset":2716,"parent_record_offset":2628,"ancestor_record_offset":512,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3193,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":512,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3193,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":512,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3193,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":512,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3193,"record_offset":2380,"parent_record_offset":2300,"ancestor_record_offset":512,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3193,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":512,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3193,"record_offset":1688,"parent_record_offset":1408,"ancestor_record_offset":512,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3193,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":512,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3193,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":512,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3193,"record_offset":2992,"parent_record_offset":2584,"ancestor_record_offset":512,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3193,"record_offset":932,"parent_record_offset":824,"ancestor_record_offset":512,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3193,"record_offset":1116,"parent_record_offset":824,"ancestor_record_offset":512,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3193,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":512,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3193,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":512,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3193,"record_offset":964,"parent_record_offset":932,"ancestor_record_offset":512,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3193,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":512,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3193,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":512,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3193,"record_offset":2456,"parent_record_offset":2404,"ancestor_record_offset":512,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3193,"record_offset":2804,"parent_record_offset":2584,"ancestor_record_offset":512,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3193,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":512,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3193,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":512,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3193,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":512,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3193,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":512,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03193.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::log2f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\mod.rs
//     evidence={"module":3193,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":512,"inlinee":6461,"source_line":86,"archive":"_pdb/A/modules/03193.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// 信息 ID: 53353f7108247fc2f77bfe6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 48530805ba8ff59d838ea481: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0x12d645b -> 0x1987840 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> @ 0x12e1f53 -> 0x1987840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
