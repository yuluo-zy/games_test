// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: assemble_circular_roof.rs.assembly.json.gz

// FUNCTION system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3475:4436 -> host 3475:2160 / rva_21b3120_len_13e8; exact_symbolizer_function_frame
//   Function-frame RVA span [0x21b357f, 0x21b35bf), 12 instructions
//   Function-frame RVA span [0x21b35c2, 0x21b35f8), 10 instructions
//   Function-frame RVA span [0x21b3f99, 0x21b40b5), 40 instructions
//   Function-frame RVA span [0x21b41ee, 0x21b4216), 10 instructions
//
// FUNCTION system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3475:2160 -> rva_21b3120_len_13e8
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21b3120_len_13e8: RVA 0x21b3120, 5096 PDB bytes; 1002 instructions
// Evidence: _assembly/roof-v1/B/pack-003/rva_21b3120_len_13e8.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3475:2160: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>
// Frames: F1
// 0x1421b3120  55                             pushq	%rbp
// 0x1421b3121  4157                           pushq	%r15
// 0x1421b3123  4156                           pushq	%r14
// 0x1421b3125  4155                           pushq	%r13
// 0x1421b3127  4154                           pushq	%r12
// 0x1421b3129  56                             pushq	%rsi
// 0x1421b312a  57                             pushq	%rdi
// 0x1421b312b  53                             pushq	%rbx
// 0x1421b312c  4881ec08030000                 subq	$0x308, %rsp            # imm = 0x308
// 0x1421b3133  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1421b313b  440f29bd70020000               movaps	%xmm15, 0x270(%rbp)
// 0x1421b3143  440f29b560020000               movaps	%xmm14, 0x260(%rbp)
// 0x1421b314b  440f29ad50020000               movaps	%xmm13, 0x250(%rbp)
// 0x1421b3153  440f29a540020000               movaps	%xmm12, 0x240(%rbp)
// 0x1421b315b  440f299d30020000               movaps	%xmm11, 0x230(%rbp)
// 0x1421b3163  440f299520020000               movaps	%xmm10, 0x220(%rbp)
// 0x1421b316b  440f298d10020000               movaps	%xmm9, 0x210(%rbp)
// 0x1421b3173  440f298500020000               movaps	%xmm8, 0x200(%rbp)
// 0x1421b317b  0f29bdf0010000                 movaps	%xmm7, 0x1f0(%rbp)
// 0x1421b3182  0f29b5e0010000                 movaps	%xmm6, 0x1e0(%rbp)
// 0x1421b3189  48c785d8010000feffffff         movq	$-0x2, 0x1d8(%rbp)
// 0x1421b3194  4c898d18010000                 movq	%r9, 0x118(%rbp)
// 0x1421b319b  4588c6                         movb	%r8b, %r14b
// 0x1421b319e  4889d3                         movq	%rdx, %rbx
// 0x1421b31a1  4889cf                         movq	%rcx, %rdi
// Frames: F2
// 0x1421b31a4  48c7855801000000000000         movq	$0x0, 0x158(%rbp)
// 0x1421b31af  488db590010000                 leaq	0x190(%rbp), %rsi
// Frames: F3
// 0x1421b31b6  4889f1                         movq	%rsi, %rcx
// 0x1421b31b9  e8024f8dfe                     callq	0x140a880c0 <.text+0xa870c0>
// 0x1421b31be  4889f1                         movq	%rsi, %rcx
// 0x1421b31c1  e83a508dfe                     callq	0x140a88200 <.text+0xa87200>
// 0x1421b31c6  0f28f0                         movaps	%xmm0, %xmm6
// 0x1421b31c9  0f28f9                         movaps	%xmm1, %xmm7
// 0x1421b31cc  488d4da8                       leaq	-0x58(%rbp), %rcx
// Frames: F4
// 0x1421b31d0  4889da                         movq	%rbx, %rdx
// 0x1421b31d3  e8480273fe                     callq	0x1408e3420 <.text+0x8e2420>
// Frames: F5
// 0x1421b31d8  4889d9                         movq	%rbx, %rcx
// 0x1421b31db  e8800073fe                     callq	0x1408e3260 <.text+0x8e2260>
// 0x1421b31e0  f30f11442420                   movss	%xmm0, 0x20(%rsp)
// 0x1421b31e6  488d4d58                       leaq	0x58(%rbp), %rcx
// 0x1421b31ea  488d55a8                       leaq	-0x58(%rbp), %rdx
// 0x1421b31ee  0f28d6                         movaps	%xmm6, %xmm2
// 0x1421b31f1  0f28df                         movaps	%xmm7, %xmm3
// 0x1421b31f4  e8470b73fe                     callq	0x1408e3d40 <.text+0x8e2d40>
// Frames: F6 -> F7
// 0x1421b31f9  f30f10b588000000               movss	0x88(%rbp), %xmm6
// Frames: F8
// 0x1421b3201  0f28c6                         movaps	%xmm6, %xmm0
// 0x1421b3204  f30f5e05904ce400               divss	0xe44c90(%rip), %xmm0   # 0x142ff7e9c
// Frames: F9 -> F8
// 0x1421b320c  e84f0c7700                     callq	0x142923e60 <.text+0x2922e60>
// Frames: F10 -> F8
// 0x1421b3211  f30f5f05d7b47700               maxss	0x77b4d7(%rip), %xmm0   # 0x14292e6f0
// Frames: F11
// 0x1421b3219  f30f2cc8                       cvttss2si	%xmm0, %ecx
// 0x1421b321d  0f2e0598177f00                 ucomiss	0x7f1798(%rip), %xmm0   # 0x1429a49bc
// 0x1421b3224  b8ffffff7f                     movl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
// 0x1421b3229  0f47c8                         cmoval	%eax, %ecx
// 0x1421b322c  31c0                           xorl	%eax, %eax
// 0x1421b322e  0f2ec0                         ucomiss	%xmm0, %xmm0
// 0x1421b3231  0f4ac8                         cmovpl	%eax, %ecx
// 0x1421b3234  f30f5ef0                       divss	%xmm0, %xmm6
// 0x1421b3238  f30f5805a46e7a00               addss	0x7a6ea4(%rip), %xmm0   # 0x14295a0e4
// 0x1421b3240  f30f118548010000               movss	%xmm0, 0x148(%rbp)
// 0x1421b3248  f20f1007                       movsd	(%rdi), %xmm0
// 0x1421b324c  0f294520                       movaps	%xmm0, 0x20(%rbp)
// 0x1421b3250  f30f5935484ce400               mulss	0xe44c48(%rip), %xmm6   # 0x142ff7ea0
// 0x1421b3258  f30f11b540010000               movss	%xmm6, 0x140(%rbp)
// 0x1421b3260  f30f10470c                     movss	0xc(%rdi), %xmm0
// 0x1421b3265  f30f11854c010000               movss	%xmm0, 0x14c(%rbp)
// 0x1421b326d  8b03                           movl	(%rbx), %eax
// 0x1421b326f  898574010000                   movl	%eax, 0x174(%rbp)
// 0x1421b3275  89c8                           movl	%ecx, %eax
// 0x1421b3277  f7d8                           negl	%eax
// 0x1421b3279  89853c010000                   movl	%eax, 0x13c(%rbp)
// 0x1421b327f  450f57c0                       xorps	%xmm8, %xmm8
// 0x1421b3283  f3440f103578267700             movss	0x772678(%rip), %xmm14  # 0x142925904
// 0x1421b328c  4531ed                         xorl	%r13d, %r13d
// 0x1421b328f  31ff                           xorl	%edi, %edi
// 0x1421b3291  898d50010000                   movl	%ecx, 0x150(%rbp)
// Frames: F12 -> F13 -> F11
// 0x1421b3297  4439e9                         cmpl	%r13d, %ecx
// 0x1421b329a  4489e8                         movl	%r13d, %eax
// 0x1421b329d  0f4fc1                         cmovgl	%ecx, %eax
// 0x1421b32a0  41f7dd                         negl	%r13d
// 0x1421b32a3  f7d8                           negl	%eax
// 0x1421b32a5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F14 -> F12 -> F13 -> F11
// 0x1421b32b0  4439e8                         cmpl	%r13d, %eax
// Frames: F12 -> F13 -> F11
// 0x1421b32b3  0f840e0c0000                   je	0x1421b3ec7 <.text+0x21b2ec7>
// Frames: F15
// 0x1421b32b9  41ffcd                         decl	%r13d
// 0x1421b32bc  4439ad3c010000                 cmpl	%r13d, 0x13c(%rbp)
// 0x1421b32c3  74eb                           je	0x1421b32b0 <.text+0x21b22b0>
// Frames: F16
// 0x1421b32c5  4489e8                         movl	%r13d, %eax
// 0x1421b32c8  f7d0                           notl	%eax
// 0x1421b32ca  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421b32cd  f30f2ac8                       cvtsi2ss	%eax, %xmm1
// 0x1421b32d1  f30f5e8d48010000               divss	0x148(%rbp), %xmm1
// 0x1421b32d9  f30f118d60010000               movss	%xmm1, 0x160(%rbp)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F16
// 0x1421b32e1  410f2ec8                       ucomiss	%xmm8, %xmm1
// Frames: F21 -> F16
// 0x1421b32e5  0f82580e0000                   jb	0x1421b4143 <.text+0x21b3143>
// 0x1421b32eb  440f2ef1                       ucomiss	%xmm1, %xmm14
// 0x1421b32ef  0f824e0e0000                   jb	0x1421b4143 <.text+0x21b3143>
// Frames: F22 -> F23 -> F16
// 0x1421b32f5  0f2ec9                         ucomiss	%xmm1, %xmm1
// Frames: F23 -> F16
// 0x1421b32f8  0f8ab60e0000                   jp	0x1421b41b4 <.text+0x21b31b4>
// Frames: F24 -> F25
// 0x1421b32fe  488d4d58                       leaq	0x58(%rbp), %rcx
// 0x1421b3302  0f298de0000000                 movaps	%xmm1, 0xe0(%rbp)
// 0x1421b3309  e80243feff                     callq	0x142197610 <.text+0x2196610>
// 0x1421b330e  488b5568                       movq	0x68(%rbp), %rdx
// Frames: F26 -> F27 -> F24 -> F25
// 0x1421b3312  89c1                           movl	%eax, %ecx
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F25
// 0x1421b3314  4839ca                         cmpq	%rcx, %rdx
// 0x1421b3317  0f860e0f0000                   jbe	0x1421b422b <.text+0x21b322b>
// Frames: F31 -> F27 -> F24 -> F25
// 0x1421b331d  488d4101                       leaq	0x1(%rcx), %rax
// 0x1421b3321  48899580010000                 movq	%rdx, 0x180(%rbp)
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F25
// 0x1421b3328  4839d0                         cmpq	%rdx, %rax
// 0x1421b332b  0f83080f0000                   jae	0x1421b4239 <.text+0x21b3239>
// Frames: F24 -> F25
// 0x1421b3331  4c8b6560                       movq	0x60(%rbp), %r12
// Frames: F32 -> F33 -> F27 -> F24 -> F25
// 0x1421b3335  450f28d6                       movaps	%xmm14, %xmm10
// 0x1421b3339  f3440f5cd0                     subss	%xmm0, %xmm10
// Frames: F34 -> F35 -> F32 -> F33 -> F27 -> F24 -> F25
// 0x1421b333e  f3450f5914cc                   mulss	(%r12,%rcx,8), %xmm10
// 0x1421b3344  f3410f1074cc08                 movss	0x8(%r12,%rcx,8), %xmm6
// 0x1421b334b  f30f59f0                       mulss	%xmm0, %xmm6
// Frames: F24 -> F36
// 0x1421b334f  488d4d58                       leaq	0x58(%rbp), %rcx
// 0x1421b3353  0f288de0000000                 movaps	0xe0(%rbp), %xmm1
// 0x1421b335a  e8b142feff                     callq	0x142197610 <.text+0x2196610>
// Frames: F26 -> F27 -> F24 -> F36
// 0x1421b335f  89c1                           movl	%eax, %ecx
// 0x1421b3361  488b9580010000                 movq	0x180(%rbp), %rdx
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F36
// 0x1421b3368  4839ca                         cmpq	%rcx, %rdx
// 0x1421b336b  0f86e00e0000                   jbe	0x1421b4251 <.text+0x21b3251>
// Frames: F31 -> F27 -> F24 -> F36
// 0x1421b3371  488d4101                       leaq	0x1(%rcx), %rax
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F36
// 0x1421b3375  4839d0                         cmpq	%rdx, %rax
// 0x1421b3378  0f83e10e0000                   jae	0x1421b425f <.text+0x21b325f>
// Frames: F24 -> F36
// 0x1421b337e  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F27 -> F24 -> F36
// 0x1421b3382  f3410f107ccc04                 movss	0x4(%r12,%rcx,8), %xmm7
// 0x1421b3389  f3450f1064cc0c                 movss	0xc(%r12,%rcx,8), %xmm12
// Frames: F37
// 0x1421b3390  4889d9                         movq	%rbx, %rcx
// 0x1421b3393  e8e80373fe                     callq	0x1408e3780 <.text+0x8e2780>
// Frames: F32 -> F33 -> F27 -> F24 -> F36
// 0x1421b3398  410f28d6                       movaps	%xmm14, %xmm2
// 0x1421b339c  f3410f5cd3                     subss	%xmm11, %xmm2
// Frames: F34 -> F38 -> F32 -> F33 -> F27 -> F24 -> F36
// 0x1421b33a1  f30f59d7                       mulss	%xmm7, %xmm2
// 0x1421b33a5  f3450f59e3                     mulss	%xmm11, %xmm12
// Frames: F39 -> F40 -> F32 -> F33 -> F27 -> F24 -> F36
// 0x1421b33aa  f3440f58e2                     addss	%xmm2, %xmm12
// Frames: F34 -> F35 -> F37
// 0x1421b33af  f30f1295e0000000               movsldup	0xe0(%rbp), %xmm2       # xmm2 = mem[0,0,2,2]
// 0x1421b33b7  0f14c1                         unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
// 0x1421b33ba  0f59c2                         mulps	%xmm2, %xmm0
// Frames: F39 -> F41 -> F37
// 0x1421b33bd  0f584520                       addps	0x20(%rbp), %xmm0
// Frames: F37
// 0x1421b33c1  0f138590010000                 movlps	%xmm0, 0x190(%rbp)
// 0x1421b33c8  488d8db0000000                 leaq	0xb0(%rbp), %rcx
// 0x1421b33cf  488d9590010000                 leaq	0x190(%rbp), %rdx
// 0x1421b33d6  410f28d4                       movaps	%xmm12, %xmm2
// 0x1421b33da  e8019aaefe                     callq	0x140c9cde0 <.text+0xc9bde0>
// Frames: F42
// 0x1421b33df  f3440f58d6                     addss	%xmm6, %xmm10
// 0x1421b33e4  f30f10854c010000               movss	0x14c(%rbp), %xmm0
// 0x1421b33ec  f30f11442420                   movss	%xmm0, 0x20(%rsp)
// 0x1421b33f2  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421b33f5  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421b33f8  488d8dd0000000                 leaq	0xd0(%rbp), %rcx
// 0x1421b33ff  410f28da                       movaps	%xmm10, %xmm3
// 0x1421b3403  e85841aefe                     callq	0x140c97560 <.text+0xc96560>
// Frames: F43
// 0x1421b3408  f30f10057c948c00               movss	0x8c947c(%rip), %xmm0   # 0x142a7c88c
// 0x1421b3410  f30f100dc0247700               movss	0x7724c0(%rip), %xmm1   # 0x1429258d8
// 0x1421b3418  410f28d2                       movaps	%xmm10, %xmm2
// 0x1421b341c  e87fd0adfe                     callq	0x140c904a0 <.text+0xc8f4a0>
// Frames: F44 -> F43
// 0x1421b3421  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421b3424  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F45 -> F43
// 0x1421b3428  410f28f6                       movaps	%xmm14, %xmm6
// 0x1421b342c  f30f5df1                       minss	%xmm1, %xmm6
// Frames: F34 -> F46 -> F47
// 0x1421b3430  440f28d6                       movaps	%xmm6, %xmm10
// 0x1421b3434  f3440f591533247700             mulss	0x772433(%rip), %xmm10  # 0x142925870
// Frames: F48
// 0x1421b343d  488d8dd0000000                 leaq	0xd0(%rbp), %rcx
// 0x1421b3444  e82745aefe                     callq	0x140c97970 <.text+0xc96970>
// Frames: F49
// 0x1421b3449  440f28d8                       movaps	%xmm0, %xmm11
// 0x1421b344d  488d8d58010000                 leaq	0x158(%rbp), %rcx
// 0x1421b3454  e827bfaffe                     callq	0x140caf380 <.text+0xcae380>
// Frames: F50
// 0x1421b3459  440f28e8                       movaps	%xmm0, %xmm13
// 0x1421b345d  410f28c6                       movaps	%xmm14, %xmm0
// 0x1421b3461  f30f5cc6                       subss	%xmm6, %xmm0
// 0x1421b3465  f3440f58d0                     addss	%xmm0, %xmm10
// 0x1421b346a  410f28ca                       movaps	%xmm10, %xmm1
// 0x1421b346e  f30f590dfa237700               mulss	0x7723fa(%rip), %xmm1   # 0x142925870
// Frames: F51
// 0x1421b3476  410f28c3                       movaps	%xmm11, %xmm0
// 0x1421b347a  f30f5ec1                       divss	%xmm1, %xmm0
// Frames: F9 -> F51
// 0x1421b347e  e8dd097700                     callq	0x142923e60 <.text+0x2922e60>
// Frames: F10 -> F51
// 0x1421b3483  f30f5f05fd247700               maxss	0x7724fd(%rip), %xmm0   # 0x142925988
// Frames: F51
// 0x1421b348b  f30f2cf0                       cvttss2si	%xmm0, %esi
// 0x1421b348f  0f2e0526157f00                 ucomiss	0x7f1526(%rip), %xmm0   # 0x1429a49bc
// 0x1421b3496  b8ffffff7f                     movl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
// 0x1421b349b  0f47f0                         cmoval	%eax, %esi
// 0x1421b349e  0f2ec0                         ucomiss	%xmm0, %xmm0
// 0x1421b34a1  b800000000                     movl	$0x0, %eax
// 0x1421b34a6  0f4af0                         cmovpl	%eax, %esi
// Frames: F49
// 0x1421b34a9  f3440f592dc2117800             mulss	0x7811c2(%rip), %xmm13  # 0x142934674
// Frames: F52 -> F53 -> F54 -> F55 -> F49
// 0x1421b34b2  410f28c5                       movaps	%xmm13, %xmm0
// 0x1421b34b6  e8150b7700                     callq	0x142923fd0 <.text+0x2922fd0>
// 0x1421b34bb  440f28e0                       movaps	%xmm0, %xmm12
// Frames: F56 -> F53 -> F54 -> F55 -> F49
// 0x1421b34bf  410f28c5                       movaps	%xmm13, %xmm0
// 0x1421b34c3  e8b8097700                     callq	0x142923e80 <.text+0x2922e80>
// 0x1421b34c8  440f28e8                       movaps	%xmm0, %xmm13
// Frames: F57
// 0x1421b34cc  4863d6                         movslq	%esi, %rdx
// 0x1421b34cf  48ffc2                         incq	%rdx
// Frames: F58
// 0x1421b34d2  f3450f59d0                     mulss	%xmm8, %xmm10
// 0x1421b34d7  f3450f5ed3                     divss	%xmm11, %xmm10
// Frames: F59
// 0x1421b34dc  488d053550e400                 leaq	0xe45035(%rip), %rax    # 0x142ff8518
// 0x1421b34e3  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421b34e8  488d8d20010000                 leaq	0x120(%rbp), %rcx
// 0x1421b34ef  410f28d2                       movaps	%xmm10, %xmm2
// 0x1421b34f3  4c8d8d58010000                 leaq	0x158(%rbp), %r9
// 0x1421b34fa  e851ceadfe                     callq	0x140c90350 <.text+0xc8f350>
// Frames: F60
// 0x1421b34ff  48899dc8000000                 movq	%rbx, 0xc8(%rbp)
// 0x1421b3506  488b9d30010000                 movq	0x130(%rbp), %rbx
// 0x1421b350d  4883fb01                       cmpq	$0x1, %rbx
// 0x1421b3511  0f86ba0c0000                   jbe	0x1421b41d1 <.text+0x21b31d1>
// Frames: F61
// 0x1421b3517  4489e8                         movl	%r13d, %eax
// 0x1421b351a  f7d8                           negl	%eax
// 0x1421b351c  898554010000                   movl	%eax, 0x154(%rbp)
// 0x1421b3522  410f28c5                       movaps	%xmm13, %xmm0
// 0x1421b3526  410f14c4                       unpcklps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0],xmm0[1],xmm12[1]
// 0x1421b352a  440f5725be237700               xorps	0x7723be(%rip), %xmm12  # 0x1429258f0
// 0x1421b3532  450f14e5                       unpcklps	%xmm13, %xmm12          # xmm12 = xmm12[0],xmm13[0],xmm12[1],xmm13[1]
// 0x1421b3536  410f16c4                       movlhps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0]
// 0x1421b353a  0f294540                       movaps	%xmm0, 0x40(%rbp)
// 0x1421b353e  488d43ff                       leaq	-0x1(%rbx), %rax
// 0x1421b3542  488985c0000000                 movq	%rax, 0xc0(%rbp)
// 0x1421b3549  4183fdff                       cmpl	$-0x1, %r13d
// 0x1421b354d  0f94c0                         sete	%al
// 0x1421b3550  4488b5d7010000                 movb	%r14b, 0x1d7(%rbp)
// 0x1421b3557  4408f0                         orb	%r14b, %al
// 0x1421b355a  0fb6c0                         movzbl	%al, %eax
// 0x1421b355d  ffc0                           incl	%eax
// 0x1421b355f  898544010000                   movl	%eax, 0x144(%rbp)
// 0x1421b3565  4531f6                         xorl	%r14d, %r14d
// 0x1421b3568  eb08                           jmp	0x1421b3572 <.text+0x21b2572>
// 0x1421b356a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F62
// 0x1421b3570  ffc7                           incl	%edi
// Frames: F63 -> F12 -> F13 -> F64 -> F65
// 0x1421b3572  4c3bb5c0000000                 cmpq	0xc0(%rbp), %r14
// Frames: F12 -> F13 -> F64 -> F65
// 0x1421b3579  0f8301090000                   jae	0x1421b3e80 <.text+0x21b2e80>
// Frames: F28 -> F29 -> F30 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b357f  4c39f3                         cmpq	%r14, %rbx
// 0x1421b3582  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421b3585  0f86630c0000                   jbe	0x1421b41ee <.text+0x21b31ee>
// Frames: F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b358b  488b8528010000                 movq	0x128(%rbp), %rax
// 0x1421b3592  f3460f101cb0                   movss	(%rax,%r14,4), %xmm11
// 0x1421b3598  f3440f119d78010000             movss	%xmm11, 0x178(%rbp)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35a1  440f2ed8                       ucomiss	%xmm0, %xmm11
// Frames: F21 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35a5  0f82ee090000                   jb	0x1421b3f99 <.text+0x21b2f99>
// 0x1421b35ab  450f2ef3                       ucomiss	%xmm11, %xmm14
// 0x1421b35af  0f82e4090000                   jb	0x1421b3f99 <.text+0x21b2f99>
// Frames: F22 -> F23 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35b5  450f2edb                       ucomiss	%xmm11, %xmm11
// Frames: F23 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35b9  0f8a4b0a0000                   jp	0x1421b400a <.text+0x21b300a>
// Frames: F64 -> F65
// 0x1421b35bf  49ffc6                         incq	%r14
// Frames: F28 -> F29 -> F30 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35c2  4939de                         cmpq	%rbx, %r14
// 0x1421b35c5  0f83370c0000                   jae	0x1421b4202 <.text+0x21b3202>
// Frames: F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35cb  f3460f1024b0                   movss	(%rax,%r14,4), %xmm12
// 0x1421b35d1  f3440f11a578010000             movss	%xmm12, 0x178(%rbp)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35da  440f2ee0                       ucomiss	%xmm0, %xmm12
// Frames: F21 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35de  0f82430a0000                   jb	0x1421b4027 <.text+0x21b3027>
// 0x1421b35e4  450f2ef4                       ucomiss	%xmm12, %xmm14
// 0x1421b35e8  0f82390a0000                   jb	0x1421b4027 <.text+0x21b3027>
// Frames: F22 -> F23 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35ee  450f2ee4                       ucomiss	%xmm12, %xmm12
// Frames: F23 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b35f2  0f8aa00a0000                   jp	0x1421b4098 <.text+0x21b3098>
// Frames: F69
// 0x1421b35f8  488d8dd0000000                 leaq	0xd0(%rbp), %rcx
// 0x1421b35ff  410f28cb                       movaps	%xmm11, %xmm1
// 0x1421b3603  e8983faefe                     callq	0x140c975a0 <.text+0xc965a0>
// Frames: F70
// 0x1421b3608  440f28d0                       movaps	%xmm0, %xmm10
// 0x1421b360c  440f28e9                       movaps	%xmm1, %xmm13
// 0x1421b3610  488d8dd0000000                 leaq	0xd0(%rbp), %rcx
// 0x1421b3617  410f28cc                       movaps	%xmm12, %xmm1
// 0x1421b361b  e8803faefe                     callq	0x140c975a0 <.text+0xc965a0>
// Frames: F71 -> F72 -> F70
// 0x1421b3620  450fc6d500                     shufps	$0x0, %xmm13, %xmm10    # xmm10 = xmm10[0,0],xmm13[0,0]
// 0x1421b3625  0f285540                       movaps	0x40(%rbp), %xmm2
// 0x1421b3629  440f59d2                       mulps	%xmm2, %xmm10
// 0x1421b362d  450f28c2                       movaps	%xmm10, %xmm8
// 0x1421b3631  66450f15c2                     unpckhpd	%xmm10, %xmm8           # xmm8 = xmm8[1],xmm10[1]
// 0x1421b3636  450f58c2                       addps	%xmm10, %xmm8
// 0x1421b363a  0fc6c100                       shufps	$0x0, %xmm1, %xmm0      # xmm0 = xmm0[0,0],xmm1[0,0]
// Frames: F73 -> F74 -> F70
// 0x1421b363e  0f59c2                         mulps	%xmm2, %xmm0
// Frames: F75 -> F76 -> F70
// 0x1421b3641  0f28f0                         movaps	%xmm0, %xmm6
// 0x1421b3644  660f15f0                       unpckhpd	%xmm0, %xmm6            # xmm6 = xmm6[1],xmm0[1]
// Frames: F77 -> F78 -> F70
// 0x1421b3648  0f58f0                         addps	%xmm0, %xmm6
// Frames: F39 -> F41 -> F79
// 0x1421b364b  410f28c0                       movaps	%xmm8, %xmm0
// 0x1421b364f  0f58c6                         addps	%xmm6, %xmm0
// Frames: F80 -> F81 -> F79
// 0x1421b3652  0f590587227700                 mulps	0x772287(%rip), %xmm0   # 0x1429258e0
// Frames: F79
// 0x1421b3659  0f138578010000                 movlps	%xmm0, 0x178(%rbp)
// Frames: F82
// 0x1421b3660  488d8d90010000                 leaq	0x190(%rbp), %rcx
// 0x1421b3667  488d9578010000                 leaq	0x178(%rbp), %rdx
// 0x1421b366e  e84d97aefe                     callq	0x140c9cdc0 <.text+0xc9bdc0>
// 0x1421b3673  f30f1085b8000000               movss	0xb8(%rbp), %xmm0
// 0x1421b367b  f20f10bdb0000000               movsd	0xb0(%rbp), %xmm7
// Frames: F83 -> F82
// 0x1421b3683  f20f108d90010000               movsd	0x190(%rbp), %xmm1
// 0x1421b368b  0f294d00                       movaps	%xmm1, (%rbp)
// Frames: F39 -> F84 -> F82
// 0x1421b368f  f30f588598010000               addss	0x198(%rbp), %xmm0
// 0x1421b3697  f30f11858c010000               movss	%xmm0, 0x18c(%rbp)
// Frames: F85
// 0x1421b369f  488d8d58010000                 leaq	0x158(%rbp), %rcx
// 0x1421b36a6  e8d5bcaffe                     callq	0x140caf380 <.text+0xcae380>
// 0x1421b36ab  440f28d0                       movaps	%xmm0, %xmm10
// Frames: F86 -> F46 -> F85
// 0x1421b36af  450f28ce                       movaps	%xmm14, %xmm9
// 0x1421b36b3  f3440f5cc8                     subss	%xmm0, %xmm9
// Frames: F34 -> F46 -> F85
// 0x1421b36b8  f3440f590df7358600             mulss	0x8635f7(%rip), %xmm9   # 0x142a16cb8
// 0x1421b36c1  f3440f59155e9a8100             mulss	0x819a5e(%rip), %xmm10  # 0x1429cd128
// Frames: F87 -> F88 -> F89
// 0x1421b36ca  488d4d58                       leaq	0x58(%rbp), %rcx
// 0x1421b36ce  0f288de0000000                 movaps	0xe0(%rbp), %xmm1
// 0x1421b36d5  e8363ffeff                     callq	0x142197610 <.text+0x2196610>
// Frames: F26 -> F87 -> F88 -> F89
// 0x1421b36da  89c0                           movl	%eax, %eax
// Frames: F90 -> F88 -> F89
// 0x1421b36dc  488d4801                       leaq	0x1(%rax), %rcx
// Frames: F28 -> F29 -> F30 -> F91 -> F89
// 0x1421b36e0  483b8d80010000                 cmpq	0x180(%rbp), %rcx
// 0x1421b36e7  0f83290b0000                   jae	0x1421b4216 <.text+0x21b3216>
// Frames: F91 -> F89
// 0x1421b36ed  f3410f104cc408                 movss	0x8(%r12,%rax,8), %xmm1
// 0x1421b36f4  f3410f1054c40c                 movss	0xc(%r12,%rax,8), %xmm2
// Frames: F86 -> F92 -> F91 -> F89
// 0x1421b36fb  f3410f5c0cc4                   subss	(%r12,%rax,8), %xmm1
// Frames: F86 -> F93 -> F91 -> F89
// 0x1421b3701  f3410f5c54c404                 subss	0x4(%r12,%rax,8), %xmm2
// Frames: F94 -> F95 -> F96 -> F97 -> F98 -> F99 -> F89
// 0x1421b3708  0f28c1                         movaps	%xmm1, %xmm0
// 0x1421b370b  f30f59c1                       mulss	%xmm1, %xmm0
// 0x1421b370f  0f28da                         movaps	%xmm2, %xmm3
// 0x1421b3712  f30f59da                       mulss	%xmm2, %xmm3
// 0x1421b3716  f30f58d8                       addss	%xmm0, %xmm3
// Frames: F100 -> F101 -> F95 -> F96 -> F97 -> F98 -> F99 -> F89
// 0x1421b371a  f30f51db                       sqrtss	%xmm3, %xmm3
// Frames: F102 -> F96 -> F97 -> F98 -> F99 -> F89
// 0x1421b371e  410f28c6                       movaps	%xmm14, %xmm0
// 0x1421b3722  f30f5ec3                       divss	%xmm3, %xmm0
// Frames: F34 -> F103 -> F97 -> F98 -> F99 -> F89
// 0x1421b3726  f30f59c8                       mulss	%xmm0, %xmm1
// Frames: F34 -> F104 -> F97 -> F98 -> F99 -> F89
// 0x1421b372a  f30f59c2                       mulss	%xmm2, %xmm0
// Frames: F105
// 0x1421b372e  f30f5ec1                       divss	%xmm1, %xmm0
// Frames: F106 -> F105
// 0x1421b3732  e8f9067700                     callq	0x142923e30 <.text+0x2922e30>
// 0x1421b3737  440f28e8                       movaps	%xmm0, %xmm13
// Frames: F107
// 0x1421b373b  f3450f58dc                     addss	%xmm12, %xmm11
// 0x1421b3740  f3440f591d27217700             mulss	0x772127(%rip), %xmm11  # 0x142925870
// 0x1421b3749  f3440f119d38010000             movss	%xmm11, 0x138(%rbp)
// 0x1421b3752  0f57c0                         xorps	%xmm0, %xmm0
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F107
// 0x1421b3755  440f2ed8                       ucomiss	%xmm0, %xmm11
// Frames: F21 -> F107
// 0x1421b3759  0f8256090000                   jb	0x1421b40b5 <.text+0x21b30b5>
// 0x1421b375f  450f2ef3                       ucomiss	%xmm11, %xmm14
// 0x1421b3763  0f824c090000                   jb	0x1421b40b5 <.text+0x21b30b5>
// Frames: F22 -> F23 -> F107
// 0x1421b3769  450f2edb                       ucomiss	%xmm11, %xmm11
// Frames: F23 -> F107
// 0x1421b376d  0f8ab3090000                   jp	0x1421b4126 <.text+0x21b3126>
// Frames: F108
// 0x1421b3773  488d8dd0000000                 leaq	0xd0(%rbp), %rcx
// 0x1421b377a  410f28cb                       movaps	%xmm11, %xmm1
// 0x1421b377e  e85d41aefe                     callq	0x140c978e0 <.text+0xc968e0>
// Frames: F71 -> F72 -> F108
// 0x1421b3783  0fc6c100                       shufps	$0x0, %xmm1, %xmm0      # xmm0 = xmm0[0,0],xmm1[0,0]
// Frames: F73 -> F74 -> F108
// 0x1421b3787  0f594540                       mulps	0x40(%rbp), %xmm0
// Frames: F75 -> F76 -> F108
// 0x1421b378b  0f28c8                         movaps	%xmm0, %xmm1
// 0x1421b378e  660f15c8                       unpckhpd	%xmm0, %xmm1            # xmm1 = xmm1[1],xmm0[1]
// Frames: F77 -> F78 -> F108
// 0x1421b3792  0f58c8                         addps	%xmm0, %xmm1
// Frames: F108
// 0x1421b3795  0f138d90010000                 movlps	%xmm1, 0x190(%rbp)
// 0x1421b379c  488d8d94000000                 leaq	0x94(%rbp), %rcx
// 0x1421b37a3  488d9590010000                 leaq	0x190(%rbp), %rdx
// 0x1421b37aa  e81196aefe                     callq	0x140c9cdc0 <.text+0xc9bdc0>
// Frames: F109
// 0x1421b37af  0f29bdf0000000                 movaps	%xmm7, 0xf0(%rbp)
// 0x1421b37b6  f3440f101de14be400             movss	0xe44be1(%rip), %xmm11  # 0x142ff83a0
// 0x1421b37bf  f3450f5cdd                     subss	%xmm13, %xmm11
// 0x1421b37c4  f3440f10b594000000             movss	0x94(%rbp), %xmm14
// 0x1421b37cd  f2440f10ad98000000             movsd	0x98(%rbp), %xmm13
// Frames: F110 -> F109
// 0x1421b37d6  450f28fe                       movaps	%xmm14, %xmm15
// 0x1421b37da  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421b37dd  f3440f59f8                     mulss	%xmm0, %xmm15
// Frames: F111 -> F112
// 0x1421b37e2  f3440f591dfd687a00             mulss	0x7a68fd(%rip), %xmm11  # 0x14295a0e8
// Frames: F52 -> F53 -> F54 -> F111 -> F112
// 0x1421b37eb  410f28c3                       movaps	%xmm11, %xmm0
// 0x1421b37ef  e8dc077700                     callq	0x142923fd0 <.text+0x2922fd0>
// 0x1421b37f4  440f28e0                       movaps	%xmm0, %xmm12
// Frames: F56 -> F53 -> F54 -> F111 -> F112
// 0x1421b37f8  410f28c3                       movaps	%xmm11, %xmm0
// 0x1421b37fc  e87f067700                     callq	0x142923e80 <.text+0x2922e80>
// Frames: F113 -> F109
// 0x1421b3801  450f28dd                       movaps	%xmm13, %xmm11
// 0x1421b3805  440f591d93217700               mulps	0x772193(%rip), %xmm11  # 0x1429259a0
// 0x1421b380d  450fc6fdd4                     shufps	$0xd4, %xmm13, %xmm15   # xmm15 = xmm15[0,1],xmm13[1,3]
// 0x1421b3812  450fc6fdf2                     shufps	$0xf2, %xmm13, %xmm15   # xmm15 = xmm15[2,0],xmm13[3,3]
// Frames: F114 -> F109
// 0x1421b3817  410f28fe                       movaps	%xmm14, %xmm7
// 0x1421b381b  f3410f5cfb                     subss	%xmm11, %xmm7
// Frames: F113 -> F109
// 0x1421b3820  450f5cdf                       subps	%xmm15, %xmm11
// 0x1421b3824  f3410f16cb                     movshdup	%xmm11, %xmm1           # xmm1 = xmm11[1,1,3,3]
// 0x1421b3829  0f294d30                       movaps	%xmm1, 0x30(%rbp)
// Frames: F34 -> F115 -> F116 -> F112
// 0x1421b382d  f3450f12fc                     movsldup	%xmm12, %xmm15          # xmm15 = xmm12[0,0,2,2]
// 0x1421b3832  450f59fb                       mulps	%xmm11, %xmm15
// Frames: F34 -> F117 -> F116 -> F112
// 0x1421b3836  f3440f59e7                     mulss	%xmm7, %xmm12
// Frames: F118 -> F119 -> F112
// 0x1421b383b  410f16c4                       movlhps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0]
// 0x1421b383f  450fc6e730                     shufps	$0x30, %xmm15, %xmm12   # xmm12 = xmm12[0,0],xmm15[3,0]
// 0x1421b3844  410f28cf                       movaps	%xmm15, %xmm1
// 0x1421b3848  410fc6cc84                     shufps	$0x84, %xmm12, %xmm1    # xmm1 = xmm1[0,1],xmm12[0,2]
// 0x1421b384d  440fc6f824                     shufps	$0x24, %xmm0, %xmm15    # xmm15 = xmm15[0,1],xmm0[2,0]
// Frames: F120 -> F121 -> F122 -> F123 -> F124 -> F112
// 0x1421b3852  450f16f5                       movlhps	%xmm13, %xmm14          # xmm14 = xmm14[0],xmm13[0]
// 0x1421b3856  410f28d6                       movaps	%xmm14, %xmm2
// 0x1421b385a  410fc6d5d8                     shufps	$0xd8, %xmm13, %xmm2    # xmm2 = xmm2[0,2],xmm13[1,3]
// 0x1421b385f  450fc6f558                     shufps	$0x58, %xmm13, %xmm14   # xmm14 = xmm14[0,2],xmm13[1,1]
// Frames: F73 -> F125 -> F126 -> F127 -> F123 -> F124 -> F112
// 0x1421b3864  410f28c7                       movaps	%xmm15, %xmm0
// 0x1421b3868  410f59c7                       mulps	%xmm15, %xmm0
// Frames: F75 -> F128 -> F127 -> F123 -> F124 -> F112
// 0x1421b386c  f30f16d8                       movshdup	%xmm0, %xmm3            # xmm3 = xmm0[1,1,3,3]
// 0x1421b3870  f30f58d8                       addss	%xmm0, %xmm3
// 0x1421b3874  0f28e0                         movaps	%xmm0, %xmm4
// 0x1421b3877  660f15e0                       unpckhpd	%xmm0, %xmm4            # xmm4 = xmm4[1],xmm0[1]
// 0x1421b387b  f30f58e3                       addss	%xmm3, %xmm4
// 0x1421b387f  0fc6e400                       shufps	$0x0, %xmm4, %xmm4      # xmm4 = xmm4[0,0,0,0]
// Frames: F73 -> F129 -> F123 -> F124 -> F112
// 0x1421b3883  0fc6c0ff                       shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
// Frames: F130 -> F129 -> F123 -> F124 -> F112
// 0x1421b3887  0f5cc4                         subps	%xmm4, %xmm0
// Frames: F73 -> F129 -> F123 -> F124 -> F112
// 0x1421b388a  410f59c6                       mulps	%xmm14, %xmm0
// 0x1421b388e  450f57f6                       xorps	%xmm14, %xmm14
// Frames: F75 -> F131 -> F132 -> F123 -> F124 -> F112
// 0x1421b3892  0f28d9                         movaps	%xmm1, %xmm3
// 0x1421b3895  0fc6d9d2                       shufps	$0xd2, %xmm1, %xmm3     # xmm3 = xmm3[2,0],xmm1[1,3]
// Frames: F75 -> F133 -> F132 -> F123 -> F124 -> F112
// 0x1421b3899  0f28e2                         movaps	%xmm2, %xmm4
// Frames: F73 -> F134 -> F132 -> F123 -> F124 -> F112
// 0x1421b389c  0f59da                         mulps	%xmm2, %xmm3
// Frames: F73 -> F125 -> F126 -> F135 -> F123 -> F124 -> F112
// 0x1421b389f  0f59d1                         mulps	%xmm1, %xmm2
// Frames: F136 -> F137 -> F126 -> F135 -> F123 -> F124 -> F112
// 0x1421b38a2  f30f16ea                       movshdup	%xmm2, %xmm5            # xmm5 = xmm2[1,1,3,3]
// 0x1421b38a6  f30f58ea                       addss	%xmm2, %xmm5
// 0x1421b38aa  0f12d2                         movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
// 0x1421b38ad  f30f58d5                       addss	%xmm5, %xmm2
// 0x1421b38b1  0f28ef                         movaps	%xmm7, %xmm5
// 0x1421b38b4  0f287d30                       movaps	0x30(%rbp), %xmm7
// Frames: F73 -> F135 -> F123 -> F124 -> F112
// 0x1421b38b8  f30f58d2                       addss	%xmm2, %xmm2
// 0x1421b38bc  0fc6d200                       shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
// 0x1421b38c0  410f59d7                       mulps	%xmm15, %xmm2
// Frames: F77 -> F138 -> F123 -> F124 -> F112
// 0x1421b38c4  0f58d0                         addps	%xmm0, %xmm2
// Frames: F75 -> F133 -> F132 -> F123 -> F124 -> F112
// 0x1421b38c7  0fc6e4d2                       shufps	$0xd2, %xmm4, %xmm4     # xmm4 = xmm4[2,0,1,3]
// Frames: F73 -> F139 -> F132 -> F123 -> F124 -> F112
// 0x1421b38cb  0f59e1                         mulps	%xmm1, %xmm4
// Frames: F130 -> F140 -> F132 -> F123 -> F124 -> F112
// 0x1421b38ce  0f5cdc                         subps	%xmm4, %xmm3
// Frames: F75 -> F141 -> F132 -> F123 -> F124 -> F112
// 0x1421b38d1  0fc6dbd2                       shufps	$0xd2, %xmm3, %xmm3     # xmm3 = xmm3[2,0,1,3]
// Frames: F73 -> F132 -> F123 -> F124 -> F112
// 0x1421b38d5  450f58ff                       addps	%xmm15, %xmm15
// 0x1421b38d9  450fc6ffff                     shufps	$0xff, %xmm15, %xmm15   # xmm15 = xmm15[3,3,3,3]
// 0x1421b38de  440f59fb                       mulps	%xmm3, %xmm15
// Frames: F77 -> F142 -> F123 -> F124 -> F112
// 0x1421b38e2  440f58fa                       addps	%xmm2, %xmm15
// Frames: F113 -> F143
// 0x1421b38e6  450f28e7                       movaps	%xmm15, %xmm12
// 0x1421b38ea  66450f15e7                     unpckhpd	%xmm15, %xmm12          # xmm12 = xmm12[1],xmm15[1]
// 0x1421b38ef  0f28c7                         movaps	%xmm7, %xmm0
// 0x1421b38f2  f3410f59c4                     mulss	%xmm12, %xmm0
// 0x1421b38f7  f3410f16e7                     movshdup	%xmm15, %xmm4           # xmm4 = xmm15[1,1,3,3]
// 0x1421b38fc  0f28cd                         movaps	%xmm5, %xmm1
// 0x1421b38ff  f30f59cc                       mulss	%xmm4, %xmm1
// 0x1421b3903  f30f5cc1                       subss	%xmm1, %xmm0
// Frames: F110 -> F143
// 0x1421b3907  0f28cd                         movaps	%xmm5, %xmm1
// 0x1421b390a  f3410f59cf                     mulss	%xmm15, %xmm1
// 0x1421b390f  410f28d4                       movaps	%xmm12, %xmm2
// 0x1421b3913  f3410f59d3                     mulss	%xmm11, %xmm2
// 0x1421b3918  f30f5cca                       subss	%xmm2, %xmm1
// Frames: F114 -> F143
// 0x1421b391c  0f28d4                         movaps	%xmm4, %xmm2
// 0x1421b391f  f3410f59d3                     mulss	%xmm11, %xmm2
// 0x1421b3924  0f28df                         movaps	%xmm7, %xmm3
// 0x1421b3927  f3410f59df                     mulss	%xmm15, %xmm3
// 0x1421b392c  f30f5cd3                       subss	%xmm3, %xmm2
// Frames: F144 -> F145 -> F146
// 0x1421b3930  440f2ef2                       ucomiss	%xmm2, %xmm14
// 0x1421b3934  0f296d10                       movaps	%xmm5, 0x10(%rbp)
// 0x1421b3938  7366                           jae	0x1421b39a0 <.text+0x21b29a0>
// Frames: F147 -> F145 -> F146
// 0x1421b393a  450f28eb                       movaps	%xmm11, %xmm13
// 0x1421b393e  f3440f58ec                     addss	%xmm4, %xmm13
// Frames: F148 -> F145 -> F146
// 0x1421b3943  f30f5815b91f7700               addss	0x771fb9(%rip), %xmm2   # 0x142925904
// Frames: F149 -> F145 -> F146
// 0x1421b394b  450f2ef5                       ucomiss	%xmm13, %xmm14
// 0x1421b394f  0f83bb000000                   jae	0x1421b3a10 <.text+0x21b2a10>
// Frames: F150 -> F145 -> F146
// 0x1421b3955  f3440f58ea                     addss	%xmm2, %xmm13
// Frames: F100 -> F151 -> F152 -> F145 -> F146
// 0x1421b395a  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421b395d  f3410f51d5                     sqrtss	%xmm13, %xmm2
// 0x1421b3962  0f28e5                         movaps	%xmm5, %xmm4
// Frames: F152 -> F145 -> F146
// 0x1421b3965  f30f102d031f7700               movss	0x771f03(%rip), %xmm5   # 0x142925870
// 0x1421b396d  f30f5eea                       divss	%xmm2, %xmm5
// Frames: F153 -> F145 -> F146
// 0x1421b3971  410f28dc                       movaps	%xmm12, %xmm3
// 0x1421b3975  f30f5cd9                       subss	%xmm1, %xmm3
// 0x1421b3979  f30f59dd                       mulss	%xmm5, %xmm3
// Frames: F154 -> F145 -> F146
// 0x1421b397d  f30f5cc4                       subss	%xmm4, %xmm0
// 0x1421b3981  f30f59c5                       mulss	%xmm5, %xmm0
// Frames: F155 -> F145 -> F146
// 0x1421b3985  0f28d7                         movaps	%xmm7, %xmm2
// 0x1421b3988  f3410f5cd7                     subss	%xmm15, %xmm2
// 0x1421b398d  f30f59d5                       mulss	%xmm5, %xmm2
// Frames: F156 -> F145 -> F146
// 0x1421b3991  f3410f59ed                     mulss	%xmm13, %xmm5
// 0x1421b3996  e905010000                     jmp	0x1421b3aa0 <.text+0x21b2aa0>
// 0x1421b399b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F157 -> F145 -> F146
// 0x1421b39a0  f3410f5ce3                     subss	%xmm11, %xmm4
// Frames: F158 -> F145 -> F146
// 0x1421b39a5  f3440f102d561f7700             movss	0x771f56(%rip), %xmm13  # 0x142925904
// 0x1421b39ae  f3440f5cea                     subss	%xmm2, %xmm13
// Frames: F159 -> F145 -> F146
// 0x1421b39b3  440f2ef4                       ucomiss	%xmm4, %xmm14
// 0x1421b39b7  0f28df                         movaps	%xmm7, %xmm3
// 0x1421b39ba  f3410f58df                     addss	%xmm15, %xmm3
// 0x1421b39bf  0f839b000000                   jae	0x1421b3a60 <.text+0x21b2a60>
// Frames: F160 -> F145 -> F146
// 0x1421b39c5  f3410f58e5                     addss	%xmm13, %xmm4
// Frames: F100 -> F151 -> F161 -> F145 -> F146
// 0x1421b39ca  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421b39cd  f30f51d4                       sqrtss	%xmm4, %xmm2
// Frames: F161 -> F145 -> F146
// 0x1421b39d1  f3440f102d961e7700             movss	0x771e96(%rip), %xmm13  # 0x142925870
// 0x1421b39da  f3440f5eea                     divss	%xmm2, %xmm13
// Frames: F162 -> F145 -> F146
// 0x1421b39df  f3410f59dd                     mulss	%xmm13, %xmm3
// Frames: F163 -> F145 -> F146
// 0x1421b39e4  f3410f59e5                     mulss	%xmm13, %xmm4
// Frames: F164 -> F145 -> F146
// 0x1421b39e9  f3410f58cc                     addss	%xmm12, %xmm1
// 0x1421b39ee  f3410f59cd                     mulss	%xmm13, %xmm1
// Frames: F165 -> F145 -> F146
// 0x1421b39f3  f30f5cc5                       subss	%xmm5, %xmm0
// 0x1421b39f7  0f28e8                         movaps	%xmm0, %xmm5
// 0x1421b39fa  f3410f59ed                     mulss	%xmm13, %xmm5
// 0x1421b39ff  0f28c4                         movaps	%xmm4, %xmm0
// 0x1421b3a02  0f28d1                         movaps	%xmm1, %xmm2
// 0x1421b3a05  e996000000                     jmp	0x1421b3aa0 <.text+0x21b2aa0>
// 0x1421b3a0a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F166 -> F145 -> F146
// 0x1421b3a10  f3410f5cd5                     subss	%xmm13, %xmm2
// Frames: F100 -> F151 -> F167 -> F145 -> F146
// 0x1421b3a15  0f57db                         xorps	%xmm3, %xmm3
// 0x1421b3a18  f30f51da                       sqrtss	%xmm2, %xmm3
// Frames: F167 -> F145 -> F146
// 0x1421b3a1c  f30f10254c1e7700               movss	0x771e4c(%rip), %xmm4   # 0x142925870
// 0x1421b3a24  f30f5ee3                       divss	%xmm3, %xmm4
// Frames: F168 -> F145 -> F146
// 0x1421b3a28  f30f58c5                       addss	%xmm5, %xmm0
// 0x1421b3a2c  f30f59c4                       mulss	%xmm4, %xmm0
// Frames: F169 -> F145 -> F146
// 0x1421b3a30  f3410f58cc                     addss	%xmm12, %xmm1
// 0x1421b3a35  f30f59cc                       mulss	%xmm4, %xmm1
// Frames: F170 -> F145 -> F146
// 0x1421b3a39  f30f59d4                       mulss	%xmm4, %xmm2
// Frames: F171 -> F145 -> F146
// 0x1421b3a3d  0f28ef                         movaps	%xmm7, %xmm5
// 0x1421b3a40  f3410f5cef                     subss	%xmm15, %xmm5
// 0x1421b3a45  f30f59ec                       mulss	%xmm4, %xmm5
// 0x1421b3a49  0f28d8                         movaps	%xmm0, %xmm3
// 0x1421b3a4c  0f28c1                         movaps	%xmm1, %xmm0
// 0x1421b3a4f  eb4f                           jmp	0x1421b3aa0 <.text+0x21b2aa0>
// 0x1421b3a51  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F172 -> F145 -> F146
// 0x1421b3a60  f3440f5cec                     subss	%xmm4, %xmm13
// Frames: F100 -> F151 -> F173 -> F145 -> F146
// 0x1421b3a65  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421b3a68  f3410f51d5                     sqrtss	%xmm13, %xmm2
// Frames: F173 -> F145 -> F146
// 0x1421b3a6d  f30f1025fb1d7700               movss	0x771dfb(%rip), %xmm4   # 0x142925870
// 0x1421b3a75  f30f5ee2                       divss	%xmm2, %xmm4
// Frames: F174 -> F145 -> F146
// 0x1421b3a79  f3440f59ec                     mulss	%xmm4, %xmm13
// Frames: F175 -> F145 -> F146
// 0x1421b3a7e  f30f59dc                       mulss	%xmm4, %xmm3
// Frames: F176 -> F145 -> F146
// 0x1421b3a82  f30f58c5                       addss	%xmm5, %xmm0
// 0x1421b3a86  0f28d0                         movaps	%xmm0, %xmm2
// 0x1421b3a89  f30f59d4                       mulss	%xmm4, %xmm2
// Frames: F177 -> F145 -> F146
// 0x1421b3a8d  410f28ec                       movaps	%xmm12, %xmm5
// 0x1421b3a91  f30f5ce9                       subss	%xmm1, %xmm5
// 0x1421b3a95  f30f59ec                       mulss	%xmm4, %xmm5
// 0x1421b3a99  0f28c3                         movaps	%xmm3, %xmm0
// 0x1421b3a9c  410f28dd                       movaps	%xmm13, %xmm3
// Frames: F118 -> F178 -> F145 -> F146
// 0x1421b3aa0  f3440f10355b1e7700             movss	0x771e5b(%rip), %xmm14  # 0x142925904
// 0x1421b3aa9  0f28bdf0000000                 movaps	0xf0(%rbp), %xmm7
// 0x1421b3ab0  0f14d8                         unpcklps	%xmm0, %xmm3            # xmm3 = xmm3[0],xmm0[0],xmm3[1],xmm0[1]
// 0x1421b3ab3  0f14d5                         unpcklps	%xmm5, %xmm2            # xmm2 = xmm2[0],xmm5[0],xmm2[1],xmm5[1]
// 0x1421b3ab6  0f16da                         movlhps	%xmm2, %xmm3            # xmm3 = xmm3[0],xmm2[0]
// Frames: F146
// 0x1421b3ab9  0f295df0                       movaps	%xmm3, -0x10(%rbp)
// 0x1421b3abd  488d4df0                       leaq	-0x10(%rbp), %rcx
// 0x1421b3ac1  e8ea36e5ff                     callq	0x1420071b0 <.text+0x20061b0>
// Frames: F179
// 0x1421b3ac6  4989c7                         movq	%rax, %r15
// 0x1421b3ac9  440f5cc6                       subps	%xmm6, %xmm8
// 0x1421b3acd  450f59c0                       mulps	%xmm8, %xmm8
// 0x1421b3ad1  f3410f16c0                     movshdup	%xmm8, %xmm0            # xmm0 = xmm8[1,1,3,3]
// 0x1421b3ad6  f3410f58c0                     addss	%xmm8, %xmm0
// 0x1421b3adb  f3450f58d1                     addss	%xmm9, %xmm10
// 0x1421b3ae0  450f57c0                       xorps	%xmm8, %xmm8
// 0x1421b3ae4  f3440f51c0                     sqrtss	%xmm0, %xmm8
// 0x1421b3ae9  f3440f599540010000             mulss	0x140(%rbp), %xmm10
// 0x1421b3af2  f3440f118590010000             movss	%xmm8, 0x190(%rbp)
// 0x1421b3afb  f3440f119594010000             movss	%xmm10, 0x194(%rbp)
// 0x1421b3b04  c78598010000cdcccc3d           movl	$0x3dcccccd, 0x198(%rbp) # imm = 0x3DCCCCCD
// 0x1421b3b0e  488d8d90010000                 leaq	0x190(%rbp), %rcx
// 0x1421b3b15  e896c4adfe                     callq	0x140c8ffb0 <.text+0xc8efb0>
// Frames: F180
// 0x1421b3b1a  4889c6                         movq	%rax, %rsi
// 0x1421b3b1d  488d8d58010000                 leaq	0x158(%rbp), %rcx
// 0x1421b3b24  e857b8affe                     callq	0x140caf380 <.text+0xcae380>
// 0x1421b3b29  f30f590537958100               mulss	0x819537(%rip), %xmm0   # 0x1429cd068
// 0x1421b3b31  f30f5885e0000000               addss	0xe0(%rbp), %xmm0
// Frames: F44 -> F180
// 0x1421b3b39  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421b3b3c  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F45 -> F180
// 0x1421b3b40  410f28c6                       movaps	%xmm14, %xmm0
// 0x1421b3b44  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F180
// 0x1421b3b48  e8d36370fe                     callq	0x1408b9f20 <.text+0x8b8f20>
// Frames: F181
// 0x1421b3b4d  0f587d00                       addps	(%rbp), %xmm7
// 0x1421b3b51  0f13bd90010000                 movlps	%xmm7, 0x190(%rbp)
// 0x1421b3b58  f30f10858c010000               movss	0x18c(%rbp), %xmm0
// 0x1421b3b60  f30f118598010000               movss	%xmm0, 0x198(%rbp)
// 0x1421b3b68  c7859c01000000000000           movl	$0x0, 0x19c(%rbp)
// 0x1421b3b72  48b90000003c0000ff7b           movabsq	$0x7bff00003c000000, %rcx # imm = 0x7BFF00003C000000
// 0x1421b3b7c  48898da0010000                 movq	%rcx, 0x1a0(%rbp)
// 0x1421b3b83  89b5a8010000                   movl	%esi, 0x1a8(%rbp)
// 0x1421b3b89  48c1ee20                       shrq	$0x20, %rsi
// 0x1421b3b8d  6689b5ac010000                 movw	%si, 0x1ac(%rbp)
// 0x1421b3b94  668985ae010000                 movw	%ax, 0x1ae(%rbp)
// 0x1421b3b9b  4c89bdb0010000                 movq	%r15, 0x1b0(%rbp)
// 0x1421b3ba2  8b8574010000                   movl	0x174(%rbp), %eax
// 0x1421b3ba8  8985b8010000                   movl	%eax, 0x1b8(%rbp)
// 0x1421b3bae  48c785bc01000000000000         movq	$0x0, 0x1bc(%rbp)
// 0x1421b3bb9  89bdc4010000                   movl	%edi, 0x1c4(%rbp)
// 0x1421b3bbf  c785c801000001000000           movl	$0x1, 0x1c8(%rbp)
// 0x1421b3bc9  8b8544010000                   movl	0x144(%rbp), %eax
// 0x1421b3bcf  8985cc010000                   movl	%eax, 0x1cc(%rbp)
// 0x1421b3bd5  488b8d18010000                 movq	0x118(%rbp), %rcx
// 0x1421b3bdc  488d9590010000                 leaq	0x190(%rbp), %rdx
// 0x1421b3be3  e828090000                     callq	0x1421b4510 <.text+0x21b3510>
// Frames: F182
// 0x1421b3be8  4183fdff                       cmpl	$-0x1, %r13d
// 0x1421b3bec  0f857ef9ffff                   jne	0x1421b3570 <.text+0x21b2570>
// Frames: F34 -> F115 -> F183
// 0x1421b3bf2  f3410f12c2                     movsldup	%xmm10, %xmm0           # xmm0 = xmm10[0,0,2,2]
// 0x1421b3bf7  410f59c7                       mulps	%xmm15, %xmm0
// Frames: F34 -> F117 -> F183
// 0x1421b3bfb  f3450f59e2                     mulss	%xmm10, %xmm12
// 0x1421b3c00  f3440f5925671c7700             mulss	0x771c67(%rip), %xmm12  # 0x142925870
// Frames: F34 -> F115 -> F183
// 0x1421b3c09  0f5905d01c7700                 mulps	0x771cd0(%rip), %xmm0   # 0x1429258e0
// Frames: F39 -> F83 -> F183
// 0x1421b3c10  0f28f7                         movaps	%xmm7, %xmm6
// 0x1421b3c13  0f58f0                         addps	%xmm0, %xmm6
// 0x1421b3c16  f30f16d6                       movshdup	%xmm6, %xmm2            # xmm2 = xmm6[1,1,3,3]
// Frames: F86 -> F184 -> F185
// 0x1421b3c1a  f30f5cf8                       subss	%xmm0, %xmm7
// 0x1421b3c1e  f30f10858c010000               movss	0x18c(%rbp), %xmm0
// Frames: F86 -> F186 -> F185
// 0x1421b3c26  f3410f5cc4                     subss	%xmm12, %xmm0
// Frames: F185
// 0x1421b3c2b  f30f11bd90010000               movss	%xmm7, 0x190(%rbp)
// 0x1421b3c33  f30f118594010000               movss	%xmm0, 0x194(%rbp)
// 0x1421b3c3b  488d8da0000000                 leaq	0xa0(%rbp), %rcx
// 0x1421b3c42  488d9590010000                 leaq	0x190(%rbp), %rdx
// 0x1421b3c49  e89291aefe                     callq	0x140c9cde0 <.text+0xc9bde0>
// Frames: F187
// 0x1421b3c4e  f30f108d8c010000               movss	0x18c(%rbp), %xmm1
// 0x1421b3c56  f3410f58cc                     addss	%xmm12, %xmm1
// 0x1421b3c5b  f2440f108da0000000             movsd	0xa0(%rbp), %xmm9
// 0x1421b3c64  f30f10bda8000000               movss	0xa8(%rbp), %xmm7
// Frames: F86 -> F186 -> F188 -> F187
// 0x1421b3c6c  0f28c7                         movaps	%xmm7, %xmm0
// 0x1421b3c6f  440f28d1                       movaps	%xmm1, %xmm10
// 0x1421b3c73  f30f5cc1                       subss	%xmm1, %xmm0
// Frames: F86 -> F184 -> F188 -> F187
// 0x1421b3c77  410f28c9                       movaps	%xmm9, %xmm1
// 0x1421b3c7b  0f5cce                         subps	%xmm6, %xmm1
// Frames: F189 -> F190 -> F188 -> F187
// 0x1421b3c7e  0f59c9                         mulps	%xmm1, %xmm1
// 0x1421b3c81  f30f16d1                       movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
// 0x1421b3c85  f30f58d1                       addss	%xmm1, %xmm2
// 0x1421b3c89  f30f59c0                       mulss	%xmm0, %xmm0
// 0x1421b3c8d  f30f58c2                       addss	%xmm2, %xmm0
// Frames: F100 -> F191 -> F190 -> F188 -> F187
// 0x1421b3c91  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421b3c94  f30f51c8                       sqrtss	%xmm0, %xmm1
// Frames: F187
// 0x1421b3c98  0f2e0de51c7700                 ucomiss	0x771ce5(%rip), %xmm1   # 0x142925984
// 0x1421b3c9f  0f86cbf8ffff                   jbe	0x1421b3570 <.text+0x21b2570>
// Frames: F113 -> F192
// 0x1421b3ca5  440f286d30                     movaps	0x30(%rbp), %xmm13
// 0x1421b3caa  410f28dd                       movaps	%xmm13, %xmm3
// 0x1421b3cae  0f57ed                         xorps	%xmm5, %xmm5
// 0x1421b3cb1  f30f59dd                       mulss	%xmm5, %xmm3
// Frames: F114 -> F192
// 0x1421b3cb5  0f28c3                         movaps	%xmm3, %xmm0
// 0x1421b3cb8  f3410f5cc3                     subss	%xmm11, %xmm0
// Frames: F110 -> F192
// 0x1421b3cbd  410f28e3                       movaps	%xmm11, %xmm4
// 0x1421b3cc1  f30f59e5                       mulss	%xmm5, %xmm4
// 0x1421b3cc5  440f286510                     movaps	0x10(%rbp), %xmm12
// 0x1421b3cca  410f28d4                       movaps	%xmm12, %xmm2
// 0x1421b3cce  f30f59d5                       mulss	%xmm5, %xmm2
// 0x1421b3cd2  f30f5ce2                       subss	%xmm2, %xmm4
// Frames: F193 -> F194
// 0x1421b3cd6  f3440f11850c010000             movss	%xmm8, 0x10c(%rbp)
// 0x1421b3cdf  f30f118d10010000               movss	%xmm1, 0x110(%rbp)
// 0x1421b3ce7  c78514010000cdcccc3d           movl	$0x3dcccccd, 0x114(%rbp) # imm = 0x3DCCCCCD
// Frames: F113 -> F192
// 0x1421b3cf1  410f28d4                       movaps	%xmm12, %xmm2
// 0x1421b3cf5  0f14d4                         unpcklps	%xmm4, %xmm2            # xmm2 = xmm2[0],xmm4[0],xmm2[1],xmm4[1]
// 0x1421b3cf8  410f14db                       unpcklps	%xmm11, %xmm3           # xmm3 = xmm3[0],xmm11[0],xmm3[1],xmm11[1]
// 0x1421b3cfc  0f5cd3                         subps	%xmm3, %xmm2
// 0x1421b3cff  f30f16da                       movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
// Frames: F159 -> F145 -> F195
// 0x1421b3d03  0f2eeb                         ucomiss	%xmm3, %xmm5
// 0x1421b3d06  734c                           jae	0x1421b3d54 <.text+0x21b2d54>
// Frames: F162 -> F145 -> F195
// 0x1421b3d08  410f28cb                       movaps	%xmm11, %xmm1
// 0x1421b3d0c  0fc60d9c08780011               shufps	$0x11, 0x78089c(%rip), %xmm1 # xmm1 = xmm1[1,0],mem[1,0]
// 0x1421b3d14  410fc6cbe8                     shufps	$0xe8, %xmm11, %xmm1    # xmm1 = xmm1[0,2],xmm11[2,3]
// 0x1421b3d19  0f58ca                         addps	%xmm2, %xmm1
// Frames: F100 -> F151 -> F161 -> F145 -> F195
// 0x1421b3d1c  f30f16d1                       movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
// 0x1421b3d20  f30f51d2                       sqrtss	%xmm2, %xmm2
// Frames: F161 -> F145 -> F195
// 0x1421b3d24  f30f101d441b7700               movss	0x771b44(%rip), %xmm3   # 0x142925870
// 0x1421b3d2c  f30f5eda                       divss	%xmm2, %xmm3
// Frames: F162 -> F145 -> F195
// 0x1421b3d30  f30f12db                       movsldup	%xmm3, %xmm3            # xmm3 = xmm3[0,0,2,2]
// 0x1421b3d34  0f59cb                         mulps	%xmm3, %xmm1
// Frames: F164 -> F145 -> F195
// 0x1421b3d37  f3410f12e4                     movsldup	%xmm12, %xmm4           # xmm4 = xmm12[0,0,2,2]
// 0x1421b3d3c  f3410f58c6                     addss	%xmm14, %xmm0
// 0x1421b3d41  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421b3d44  0f5cd4                         subps	%xmm4, %xmm2
// 0x1421b3d47  0fc6d001                       shufps	$0x1, %xmm0, %xmm2      # xmm2 = xmm2[1,0],xmm0[0,0]
// 0x1421b3d4b  0fc6d0e2                       shufps	$0xe2, %xmm0, %xmm2     # xmm2 = xmm2[2,0],xmm0[2,3]
// 0x1421b3d4f  0f59d3                         mulps	%xmm3, %xmm2
// Frames: F159 -> F145 -> F195
// 0x1421b3d52  eb40                           jmp	0x1421b3d94 <.text+0x21b2d94>
// Frames: F172 -> F145 -> F195
// 0x1421b3d54  410f28ce                       movaps	%xmm14, %xmm1
// 0x1421b3d58  f30f5ccb                       subss	%xmm3, %xmm1
// Frames: F100 -> F151 -> F173 -> F145 -> F195
// 0x1421b3d5c  0f57db                         xorps	%xmm3, %xmm3
// 0x1421b3d5f  f30f51d9                       sqrtss	%xmm1, %xmm3
// Frames: F173 -> F145 -> F195
// 0x1421b3d63  f30f1025051b7700               movss	0x771b05(%rip), %xmm4   # 0x142925870
// 0x1421b3d6b  f30f5ee3                       divss	%xmm3, %xmm4
// Frames: F175 -> F145 -> F195
// 0x1421b3d6f  f3410f58d5                     addss	%xmm13, %xmm2
// Frames: F174 -> F145 -> F195
// 0x1421b3d74  0f14ca                         unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
// 0x1421b3d77  f30f12d4                       movsldup	%xmm4, %xmm2            # xmm2 = xmm4[0,0,2,2]
// 0x1421b3d7b  0f59ca                         mulps	%xmm2, %xmm1
// Frames: F176 -> F145 -> F195
// 0x1421b3d7e  410f16c4                       movlhps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0]
// 0x1421b3d82  410fc6c4e2                     shufps	$0xe2, %xmm12, %xmm0    # xmm0 = xmm0[2,0],xmm12[2,3]
// 0x1421b3d87  0f5805324a8d00                 addps	0x8d4a32(%rip), %xmm0   # 0x142a887c0
// 0x1421b3d8e  0f59c2                         mulps	%xmm2, %xmm0
// 0x1421b3d91  0f28d0                         movaps	%xmm0, %xmm2
// Frames: F118 -> F178 -> F145 -> F195
// 0x1421b3d94  0f16ca                         movlhps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0]
// Frames: F195
// 0x1421b3d97  0f294de0                       movaps	%xmm1, -0x20(%rbp)
// 0x1421b3d9b  488d4de0                       leaq	-0x20(%rbp), %rcx
// 0x1421b3d9f  e80c34e5ff                     callq	0x1420071b0 <.text+0x20061b0>
// Frames: F196
// 0x1421b3da4  4989c7                         movq	%rax, %r15
// 0x1421b3da7  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x1421b3dae  e8fdc1adfe                     callq	0x140c8ffb0 <.text+0xc8efb0>
// Frames: F197
// 0x1421b3db3  4889c6                         movq	%rax, %rsi
// 0x1421b3db6  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421b3db9  e8626170fe                     callq	0x1408b9f20 <.text+0x8b8f20>
// Frames: F198
// 0x1421b3dbe  0f280d1b1b7700                 movaps	0x771b1b(%rip), %xmm1   # 0x1429258e0
// 0x1421b3dc5  440f59c9                       mulps	%xmm1, %xmm9
// 0x1421b3dc9  f30f10059f1a7700               movss	0x771a9f(%rip), %xmm0   # 0x142925870
// 0x1421b3dd1  f30f59f8                       mulss	%xmm0, %xmm7
// 0x1421b3dd5  0f59f1                         mulps	%xmm1, %xmm6
// 0x1421b3dd8  410f58f1                       addps	%xmm9, %xmm6
// 0x1421b3ddc  f3440f59d0                     mulss	%xmm0, %xmm10
// 0x1421b3de1  f3440f58d7                     addss	%xmm7, %xmm10
// 0x1421b3de6  0f13b590010000                 movlps	%xmm6, 0x190(%rbp)
// 0x1421b3ded  f3440f119598010000             movss	%xmm10, 0x198(%rbp)
// 0x1421b3df6  c7859c01000000000000           movl	$0x0, 0x19c(%rbp)
// 0x1421b3e00  48b90000003c0000ff7b           movabsq	$0x7bff00003c000000, %rcx # imm = 0x7BFF00003C000000
// 0x1421b3e0a  48898da0010000                 movq	%rcx, 0x1a0(%rbp)
// 0x1421b3e11  89b5a8010000                   movl	%esi, 0x1a8(%rbp)
// 0x1421b3e17  48c1ee20                       shrq	$0x20, %rsi
// 0x1421b3e1b  6689b5ac010000                 movw	%si, 0x1ac(%rbp)
// 0x1421b3e22  668985ae010000                 movw	%ax, 0x1ae(%rbp)
// 0x1421b3e29  4c89bdb0010000                 movq	%r15, 0x1b0(%rbp)
// 0x1421b3e30  8b8574010000                   movl	0x174(%rbp), %eax
// 0x1421b3e36  8985b8010000                   movl	%eax, 0x1b8(%rbp)
// 0x1421b3e3c  48c785bc01000000000000         movq	$0x0, 0x1bc(%rbp)
// 0x1421b3e47  89bdc4010000                   movl	%edi, 0x1c4(%rbp)
// 0x1421b3e4d  48b80100000002000000           movabsq	$0x200000001, %rax # imm = 0x200000001
// 0x1421b3e57  488985c8010000                 movq	%rax, 0x1c8(%rbp)
// 0x1421b3e5e  488b8d18010000                 movq	0x118(%rbp), %rcx
// 0x1421b3e65  488d9590010000                 leaq	0x190(%rbp), %rdx
// 0x1421b3e6c  e89f060000                     callq	0x1421b4510 <.text+0x21b3510>
// 0x1421b3e71  e9faf6ffff                     jmp	0x1421b3570 <.text+0x21b2570>
// 0x1421b3e76  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F199 -> F200 -> F201 -> F201 -> F202
// 0x1421b3e80  488b9520010000                 movq	0x120(%rbp), %rdx
// Frames: F203 -> F199 -> F200 -> F201 -> F201 -> F202
// 0x1421b3e87  4885d2                         testq	%rdx, %rdx
// 0x1421b3e8a  450f57c0                       xorps	%xmm8, %xmm8
// 0x1421b3e8e  7416                           je	0x1421b3ea6 <.text+0x21b2ea6>
// Frames: F199 -> F200 -> F201 -> F201 -> F202
// 0x1421b3e90  488b8d28010000                 movq	0x128(%rbp), %rcx
// Frames: F204 -> F205 -> F199 -> F200 -> F201 -> F201 -> F202
// 0x1421b3e97  48c1e202                       shlq	$0x2, %rdx
// Frames: F206 -> F207 -> F208 -> F200 -> F201 -> F201 -> F202
// 0x1421b3e9b  41b804000000                   movl	$0x4, %r8d
// 0x1421b3ea1  e87afd45fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F11
// 0x1421b3ea6  440fb6b5d7010000               movzbl	0x1d7(%rbp), %r14d
// 0x1421b3eae  488b9dc8000000                 movq	0xc8(%rbp), %rbx
// 0x1421b3eb5  8b8d50010000                   movl	0x150(%rbp), %ecx
// 0x1421b3ebb  448bad54010000                 movl	0x154(%rbp), %r13d
// 0x1421b3ec2  e9d0f3ffff                     jmp	0x1421b3297 <.text+0x21b2297>
// Frames: F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ec7  488b5558                       movq	0x58(%rbp), %rdx
// Frames: F203 -> F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ecb  4885d2                         testq	%rdx, %rdx
// 0x1421b3ece  7413                           je	0x1421b3ee3 <.text+0x21b2ee3>
// Frames: F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ed0  488b4d60                       movq	0x60(%rbp), %rcx
// Frames: F204 -> F205 -> F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ed4  48c1e203                       shlq	$0x3, %rdx
// Frames: F206 -> F207 -> F208 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ed8  41b804000000                   movl	$0x4, %r8d
// 0x1421b3ede  e83dfd45fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ee3  488b5570                       movq	0x70(%rbp), %rdx
// Frames: F203 -> F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ee7  4885d2                         testq	%rdx, %rdx
// 0x1421b3eea  7413                           je	0x1421b3eff <.text+0x21b2eff>
// Frames: F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3eec  488b4d78                       movq	0x78(%rbp), %rcx
// Frames: F204 -> F205 -> F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ef0  48c1e202                       shlq	$0x2, %rdx
// Frames: F206 -> F207 -> F208 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3ef4  41b804000000                   movl	$0x4, %r8d
// 0x1421b3efa  e821fd45fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3eff  488b55a8                       movq	-0x58(%rbp), %rdx
// Frames: F203 -> F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f03  4885d2                         testq	%rdx, %rdx
// 0x1421b3f06  7413                           je	0x1421b3f1b <.text+0x21b2f1b>
// Frames: F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f08  488b4db0                       movq	-0x50(%rbp), %rcx
// Frames: F204 -> F205 -> F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f0c  48c1e203                       shlq	$0x3, %rdx
// Frames: F206 -> F207 -> F208 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f10  41b804000000                   movl	$0x4, %r8d
// 0x1421b3f16  e805fd45fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f1b  488b55c0                       movq	-0x40(%rbp), %rdx
// Frames: F203 -> F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f1f  4885d2                         testq	%rdx, %rdx
// 0x1421b3f22  7413                           je	0x1421b3f37 <.text+0x21b2f37>
// Frames: F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f24  488b4dc8                       movq	-0x38(%rbp), %rcx
// Frames: F204 -> F205 -> F199 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f28  48c1e202                       shlq	$0x2, %rdx
// Frames: F206 -> F207 -> F208 -> F200 -> F201 -> F201 -> F201 -> F209
// 0x1421b3f2c  41b804000000                   movl	$0x4, %r8d
// 0x1421b3f32  e8e9fc45fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F209
// 0x1421b3f37  0f28b5e0010000                 movaps	0x1e0(%rbp), %xmm6
// 0x1421b3f3e  0f28bdf0010000                 movaps	0x1f0(%rbp), %xmm7
// 0x1421b3f45  440f288500020000               movaps	0x200(%rbp), %xmm8
// 0x1421b3f4d  440f288d10020000               movaps	0x210(%rbp), %xmm9
// 0x1421b3f55  440f289520020000               movaps	0x220(%rbp), %xmm10
// 0x1421b3f5d  440f289d30020000               movaps	0x230(%rbp), %xmm11
// 0x1421b3f65  440f28a540020000               movaps	0x240(%rbp), %xmm12
// 0x1421b3f6d  440f28ad50020000               movaps	0x250(%rbp), %xmm13
// 0x1421b3f75  440f28b560020000               movaps	0x260(%rbp), %xmm14
// 0x1421b3f7d  440f28bd70020000               movaps	0x270(%rbp), %xmm15
// 0x1421b3f85  4881c408030000                 addq	$0x308, %rsp            # imm = 0x308
// 0x1421b3f8c  5b                             popq	%rbx
// 0x1421b3f8d  5f                             popq	%rdi
// 0x1421b3f8e  5e                             popq	%rsi
// 0x1421b3f8f  415c                           popq	%r12
// 0x1421b3f91  415d                           popq	%r13
// 0x1421b3f93  415e                           popq	%r14
// 0x1421b3f95  415f                           popq	%r15
// 0x1421b3f97  5d                             popq	%rbp
// 0x1421b3f98  c3                             retq
// Frames: F210 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b3f99  488d8578010000                 leaq	0x178(%rbp), %rax
// 0x1421b3fa0  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x1421b3fa7  488d056289bdfe                 leaq	-0x142769e(%rip), %rax  # 0x140d8c910 <.text+0xd8b910>
// 0x1421b3fae  48898568010000                 movq	%rax, 0x168(%rbp)
// Frames: F211 -> F210 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b3fb5  488d057c05e400                 leaq	0xe4057c(%rip), %rax    # 0x142ff4538
// 0x1421b3fbc  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x1421b3fc3  48c7859801000001000000         movq	$0x1, 0x198(%rbp)
// 0x1421b3fce  48c785b001000000000000         movq	$0x0, 0x1b0(%rbp)
// 0x1421b3fd9  488d8560010000                 leaq	0x160(%rbp), %rax
// 0x1421b3fe0  488985a0010000                 movq	%rax, 0x1a0(%rbp)
// 0x1421b3fe7  48c785a801000001000000         movq	$0x1, 0x1a8(%rbp)
// Frames: F210 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b3ff2  488d15a745e400                 leaq	0xe445a7(%rip), %rdx    # 0x142ff85a0
// 0x1421b3ff9  488d8d90010000                 leaq	0x190(%rbp), %rcx
// 0x1421b4000  e88b537200                     callq	0x1428d9390 <.text+0x28d8390>
// 0x1421b4005  e96b020000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F23 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b400a  488d0d3705e400                 leaq	0xe40537(%rip), %rcx    # 0x142ff4548
// 0x1421b4011  4c8d058845e400                 leaq	0xe44588(%rip), %r8     # 0x142ff85a0
// 0x1421b4018  ba1d000000                     movl	$0x1d, %edx
// 0x1421b401d  e80e547200                     callq	0x1428d9430 <.text+0x28d8430>
// 0x1421b4022  e94e020000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F210 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b4027  488d8578010000                 leaq	0x178(%rbp), %rax
// 0x1421b402e  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x1421b4035  488d05d488bdfe                 leaq	-0x142772c(%rip), %rax  # 0x140d8c910 <.text+0xd8b910>
// 0x1421b403c  48898568010000                 movq	%rax, 0x168(%rbp)
// Frames: F211 -> F210 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b4043  488d05ee04e400                 leaq	0xe404ee(%rip), %rax    # 0x142ff4538
// 0x1421b404a  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x1421b4051  48c7859801000001000000         movq	$0x1, 0x198(%rbp)
// 0x1421b405c  48c785b001000000000000         movq	$0x0, 0x1b0(%rbp)
// 0x1421b4067  488d8560010000                 leaq	0x160(%rbp), %rax
// 0x1421b406e  488985a0010000                 movq	%rax, 0x1a0(%rbp)
// 0x1421b4075  48c785a801000001000000         movq	$0x1, 0x1a8(%rbp)
// Frames: F210 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b4080  488d154945e400                 leaq	0xe44549(%rip), %rdx    # 0x142ff85d0
// 0x1421b4087  488d8d90010000                 leaq	0x190(%rbp), %rcx
// 0x1421b408e  e8fd527200                     callq	0x1428d9390 <.text+0x28d8390>
// 0x1421b4093  e9dd010000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F23 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b4098  488d0da904e400                 leaq	0xe404a9(%rip), %rcx    # 0x142ff4548
// 0x1421b409f  4c8d052a45e400                 leaq	0xe4452a(%rip), %r8     # 0x142ff85d0
// 0x1421b40a6  ba1d000000                     movl	$0x1d, %edx
// 0x1421b40ab  e880537200                     callq	0x1428d9430 <.text+0x28d8430>
// 0x1421b40b0  e9c0010000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F210 -> F107
// 0x1421b40b5  488d8538010000                 leaq	0x138(%rbp), %rax
// 0x1421b40bc  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x1421b40c3  488d054688bdfe                 leaq	-0x14277ba(%rip), %rax  # 0x140d8c910 <.text+0xd8b910>
// 0x1421b40ca  48898568010000                 movq	%rax, 0x168(%rbp)
// Frames: F211 -> F210 -> F107
// 0x1421b40d1  488d056004e400                 leaq	0xe40460(%rip), %rax    # 0x142ff4538
// 0x1421b40d8  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x1421b40df  48c7859801000001000000         movq	$0x1, 0x198(%rbp)
// 0x1421b40ea  48c785b001000000000000         movq	$0x0, 0x1b0(%rbp)
// 0x1421b40f5  488d8560010000                 leaq	0x160(%rbp), %rax
// 0x1421b40fc  488985a0010000                 movq	%rax, 0x1a0(%rbp)
// 0x1421b4103  48c785a801000001000000         movq	$0x1, 0x1a8(%rbp)
// Frames: F210 -> F107
// 0x1421b410e  488d155b44e400                 leaq	0xe4445b(%rip), %rdx    # 0x142ff8570
// 0x1421b4115  488d8d90010000                 leaq	0x190(%rbp), %rcx
// 0x1421b411c  e86f527200                     callq	0x1428d9390 <.text+0x28d8390>
// 0x1421b4121  e94f010000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F23 -> F107
// 0x1421b4126  488d0d1b04e400                 leaq	0xe4041b(%rip), %rcx    # 0x142ff4548
// 0x1421b412d  4c8d053c44e400                 leaq	0xe4443c(%rip), %r8     # 0x142ff8570
// 0x1421b4134  ba1d000000                     movl	$0x1d, %edx
// 0x1421b4139  e8f2527200                     callq	0x1428d9430 <.text+0x28d8430>
// 0x1421b413e  e932010000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F210 -> F16
// 0x1421b4143  488d8560010000                 leaq	0x160(%rbp), %rax
// 0x1421b414a  48898520010000                 movq	%rax, 0x120(%rbp)
// 0x1421b4151  488d05b887bdfe                 leaq	-0x1427848(%rip), %rax  # 0x140d8c910 <.text+0xd8b910>
// 0x1421b4158  48898528010000                 movq	%rax, 0x128(%rbp)
// Frames: F211 -> F210 -> F16
// 0x1421b415f  488d05d203e400                 leaq	0xe403d2(%rip), %rax    # 0x142ff4538
// 0x1421b4166  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x1421b416d  48c7859801000001000000         movq	$0x1, 0x198(%rbp)
// 0x1421b4178  48c785b001000000000000         movq	$0x0, 0x1b0(%rbp)
// 0x1421b4183  488d8520010000                 leaq	0x120(%rbp), %rax
// 0x1421b418a  488985a0010000                 movq	%rax, 0x1a0(%rbp)
// 0x1421b4191  48c785a801000001000000         movq	$0x1, 0x1a8(%rbp)
// Frames: F210 -> F16
// 0x1421b419c  488d155d43e400                 leaq	0xe4435d(%rip), %rdx    # 0x142ff8500
// 0x1421b41a3  488d8d90010000                 leaq	0x190(%rbp), %rcx
// 0x1421b41aa  e8e1517200                     callq	0x1428d9390 <.text+0x28d8390>
// 0x1421b41af  e9c1000000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F23 -> F16
// 0x1421b41b4  488d0d8d03e400                 leaq	0xe4038d(%rip), %rcx    # 0x142ff4548
// 0x1421b41bb  4c8d053e43e400                 leaq	0xe4433e(%rip), %r8     # 0x142ff8500
// 0x1421b41c2  ba1d000000                     movl	$0x1d, %edx
// 0x1421b41c7  e864527200                     callq	0x1428d9430 <.text+0x28d8430>
// 0x1421b41cc  e9a4000000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F60
// 0x1421b41d1  488d0d5843e400                 leaq	0xe44358(%rip), %rcx    # 0x142ff8530
// 0x1421b41d8  4c8d057943e400                 leaq	0xe44379(%rip), %r8     # 0x142ff8558
// 0x1421b41df  ba23000000                     movl	$0x23, %edx
// 0x1421b41e4  e847527200                     callq	0x1428d9430 <.text+0x28d8430>
// 0x1421b41e9  e987000000                     jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F28 -> F29 -> F30 -> F66 -> F67 -> F68 -> F64 -> F65
// 0x1421b41ee  4c8d059343e400                 leaq	0xe44393(%rip), %r8     # 0x142ff8588
// 0x1421b41f5  4c89f1                         movq	%r14, %rcx
// 0x1421b41f8  4889da                         movq	%rbx, %rdx
// 0x1421b41fb  e818537200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421b4200  eb73                           jmp	0x1421b4275 <.text+0x21b3275>
// 0x1421b4202  4c8d05af43e400                 leaq	0xe443af(%rip), %r8     # 0x142ff85b8
// 0x1421b4209  4c89f1                         movq	%r14, %rcx
// 0x1421b420c  4889da                         movq	%rbx, %rdx
// 0x1421b420f  e804537200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421b4214  eb5f                           jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F28 -> F29 -> F30 -> F91 -> F89
// 0x1421b4216  4c8d054b02e400                 leaq	0xe4024b(%rip), %r8     # 0x142ff4468
// 0x1421b421d  488b9580010000                 movq	0x180(%rbp), %rdx
// 0x1421b4224  e8ef527200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421b4229  eb4a                           jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F25
// 0x1421b422b  4c8d050602e400                 leaq	0xe40206(%rip), %r8     # 0x142ff4438
// 0x1421b4232  e8e1527200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421b4237  eb3c                           jmp	0x1421b4275 <.text+0x21b3275>
// 0x1421b4239  4c8d051002e400                 leaq	0xe40210(%rip), %r8     # 0x142ff4450
// 0x1421b4240  4889c1                         movq	%rax, %rcx
// 0x1421b4243  488b9580010000                 movq	0x180(%rbp), %rdx
// 0x1421b424a  e8c9527200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421b424f  eb24                           jmp	0x1421b4275 <.text+0x21b3275>
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F36
// 0x1421b4251  4c8d05e001e400                 leaq	0xe401e0(%rip), %r8     # 0x142ff4438
// 0x1421b4258  e8bb527200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421b425d  eb16                           jmp	0x1421b4275 <.text+0x21b3275>
// 0x1421b425f  4c8d05ea01e400                 leaq	0xe401ea(%rip), %r8     # 0x142ff4450
// 0x1421b4266  4889c1                         movq	%rax, %rcx
// 0x1421b4269  488b9580010000                 movq	0x180(%rbp), %rdx
// 0x1421b4270  e8a3527200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421b4275  0f0b                           ud2
// 0x1421b4277  660f1f840000000000             nopw	(%rax,%rax)
// 0x1421b4280  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b4285  55                             pushq	%rbp
// 0x1421b4286  4157                           pushq	%r15
// 0x1421b4288  4156                           pushq	%r14
// 0x1421b428a  4155                           pushq	%r13
// 0x1421b428c  4154                           pushq	%r12
// 0x1421b428e  56                             pushq	%rsi
// 0x1421b428f  57                             pushq	%rdi
// 0x1421b4290  53                             pushq	%rbx
// 0x1421b4291  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1421b4298  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421b429f  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x1421b42a5  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x1421b42ab  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x1421b42b1  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x1421b42b7  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x1421b42bd  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x1421b42c3  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x1421b42cc  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x1421b42d5  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x1421b42dd  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F199 -> F200 -> F201 -> F201 -> F202
// 0x1421b42e5  488b9520010000                 movq	0x120(%rbp), %rdx
// Frames: F203 -> F199 -> F200 -> F201 -> F201 -> F202
// 0x1421b42ec  4885d2                         testq	%rdx, %rdx
// 0x1421b42ef  7416                           je	0x1421b4307 <.text+0x21b3307>
// Frames: F199 -> F200 -> F201 -> F201 -> F202
// 0x1421b42f1  488b8d28010000                 movq	0x128(%rbp), %rcx
// Frames: F204 -> F205 -> F199 -> F200 -> F201 -> F201 -> F202
// 0x1421b42f8  48c1e202                       shlq	$0x2, %rdx
// Frames: F206 -> F207 -> F208 -> F200 -> F201 -> F201 -> F202
// 0x1421b42fc  41b804000000                   movl	$0x4, %r8d
// 0x1421b4302  e819f945fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F202
// 0x1421b4307  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x1421b430f  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x1421b4317  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x1421b4320  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1421b4329  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x1421b432f  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x1421b4335  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x1421b433b  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x1421b4341  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x1421b4347  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x1421b434d  4881c4c8000000                 addq	$0xc8, %rsp
// 0x1421b4354  5b                             popq	%rbx
// 0x1421b4355  5f                             popq	%rdi
// 0x1421b4356  5e                             popq	%rsi
// 0x1421b4357  415c                           popq	%r12
// 0x1421b4359  415d                           popq	%r13
// 0x1421b435b  415e                           popq	%r14
// 0x1421b435d  415f                           popq	%r15
// 0x1421b435f  5d                             popq	%rbp
// 0x1421b4360  c3                             retq
// 0x1421b4361  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// 0x1421b4370  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b4375  55                             pushq	%rbp
// 0x1421b4376  4157                           pushq	%r15
// 0x1421b4378  4156                           pushq	%r14
// 0x1421b437a  4155                           pushq	%r13
// 0x1421b437c  4154                           pushq	%r12
// 0x1421b437e  56                             pushq	%rsi
// 0x1421b437f  57                             pushq	%rdi
// 0x1421b4380  53                             pushq	%rbx
// 0x1421b4381  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1421b4388  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421b438f  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x1421b4395  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x1421b439b  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x1421b43a1  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x1421b43a7  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x1421b43ad  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x1421b43b3  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x1421b43bc  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x1421b43c5  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x1421b43cd  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// 0x1421b43d5  488d4da8                       leaq	-0x58(%rbp), %rcx
// Frames: F209
// 0x1421b43d9  e812e141fe                     callq	0x1405d24f0 <.text+0x5d14f0>
// 0x1421b43de  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x1421b43e6  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x1421b43ee  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x1421b43f7  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1421b4400  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x1421b4406  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x1421b440c  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x1421b4412  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x1421b4418  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x1421b441e  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x1421b4424  4881c4c8000000                 addq	$0xc8, %rsp
// 0x1421b442b  5b                             popq	%rbx
// 0x1421b442c  5f                             popq	%rdi
// 0x1421b442d  5e                             popq	%rsi
// 0x1421b442e  415c                           popq	%r12
// 0x1421b4430  415d                           popq	%r13
// 0x1421b4432  415e                           popq	%r14
// 0x1421b4434  415f                           popq	%r15
// 0x1421b4436  5d                             popq	%rbp
// 0x1421b4437  c3                             retq
// 0x1421b4438  0f1f840000000000               nopl	(%rax,%rax)
// 0x1421b4440  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b4445  55                             pushq	%rbp
// 0x1421b4446  4157                           pushq	%r15
// 0x1421b4448  4156                           pushq	%r14
// 0x1421b444a  4155                           pushq	%r13
// 0x1421b444c  4154                           pushq	%r12
// 0x1421b444e  56                             pushq	%rsi
// 0x1421b444f  57                             pushq	%rdi
// 0x1421b4450  53                             pushq	%rbx
// 0x1421b4451  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1421b4458  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421b445f  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x1421b4465  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x1421b446b  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x1421b4471  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x1421b4477  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x1421b447d  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x1421b4483  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x1421b448c  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x1421b4495  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x1421b449d  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// 0x1421b44a5  488d4d58                       leaq	0x58(%rbp), %rcx
// 0x1421b44a9  e842e041fe                     callq	0x1405d24f0 <.text+0x5d14f0>
// 0x1421b44ae  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x1421b44b6  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x1421b44be  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x1421b44c7  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1421b44d0  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x1421b44d6  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x1421b44dc  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x1421b44e2  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x1421b44e8  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x1421b44ee  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x1421b44f4  4881c4c8000000                 addq	$0xc8, %rsp
// 0x1421b44fb  5b                             popq	%rbx
// 0x1421b44fc  5f                             popq	%rdi
// 0x1421b44fd  5e                             popq	%rsi
// 0x1421b44fe  415c                           popq	%r12
// 0x1421b4500  415d                           popq	%r13
// 0x1421b4502  415e                           popq	%r14
// 0x1421b4504  415f                           popq	%r15
// 0x1421b4506  5d                             popq	%rbp
// 0x1421b4507  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/utils/assemble_circular_roof.rs:26:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F2: crates/systems/roof/src/utils/assemble_circular_roof.rs:32:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F3: crates/systems/roof/src/utils/assemble_circular_roof.rs:35:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F4: crates/systems/roof/src/utils/assemble_circular_roof.rs:40:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F5: crates/systems/roof/src/utils/assemble_circular_roof.rs:42:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F6: crates/utils/src/curve/mod.rs:170:0 utils::curve::Curve::length_ws
// F7: crates/systems/roof/src/utils/assemble_circular_roof.rs:47:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F8: crates/systems/roof/src/utils/assemble_circular_roof.rs:48:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:71:0 std::f32::impl$0::ceil
// F10: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:891:0 core::f32::impl$0::max
// F11: crates/systems/roof/src/utils/assemble_circular_roof.rs:56:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/range.rs:764:0 core::iter::range::impl$5::spec_next
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/range.rs:849:0 core::iter::range::impl$6::next
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1824:0 core::cmp::impls::impl$76::lt
// F15: crates/systems/roof/src/utils/assemble_circular_roof.rs:58:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F16: crates/systems/roof/src/utils/assemble_circular_roof.rs:62:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1788:0 core::cmp::impls::impl$53::le
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1944:0 core::cmp::impls::impl$10::le
// F19: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/range.rs:823:0 core::ops::range::RangeBounds::contains
// F20: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/range.rs:514:0 core::ops::range::RangeInclusive::contains
// F21: crates/utils/src/curve/curve_u.rs:91:0 utils::curve::curve_u::CurveU::new
// F22: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:525:0 core::f32::impl$0::is_nan
// F23: crates/utils/src/curve/curve_u.rs:95:0 utils::curve::curve_u::CurveU::new
// F24: crates/utils/src/curve/mod.rs:740:0 utils::curve::Curve::get_pos_at_u
// F25: crates/systems/roof/src/utils/assemble_circular_roof.rs:64:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F26: crates/utils/src/curve/mod.rs:1333:0 utils::curve::CurveInternalCoord::i0
// F27: crates/utils/src/curve/mod.rs:800:0 utils::curve::Curve::get_pos_at_coord
// F28: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:274:0 core::slice::index::impl$2::index
// F29: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:16:0 core::slice::index::impl$0::index
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3358:0 alloc::vec::impl$13::index
// F31: crates/utils/src/curve/mod.rs:1338:0 utils::curve::CurveInternalCoord::i1
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:694:0 glam::f32::vec2::Vec2::lerp
// F33: crates/utils/src/lib.rs:79:0 utils::impl$0::lerp
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:344:0 core::ops::arith::impl$141::mul
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1200:0 glam::f32::vec2::impl$24::mul
// F36: crates/systems/roof/src/utils/assemble_circular_roof.rs:65:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F37: crates/systems/roof/src/utils/assemble_circular_roof.rs:67:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1201:0 glam::f32::vec2::impl$24::mul
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1286:0 glam::f32::vec2::impl$34::add
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1285:0 glam::f32::vec2::impl$34::add
// F42: crates/systems/roof/src/utils/assemble_circular_roof.rs:73:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F43: crates/systems/roof/src/utils/assemble_circular_roof.rs:75:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1418:0 core::f32::impl$0::clamp
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1421:0 core::f32::impl$0::clamp
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F47: crates/systems/roof/src/utils/assemble_circular_roof.rs:76:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F48: crates/systems/roof/src/utils/assemble_circular_roof.rs:79:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F49: crates/systems/roof/src/utils/assemble_circular_roof.rs:89:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F50: crates/systems/roof/src/utils/assemble_circular_roof.rs:77:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F51: crates/systems/roof/src/utils/assemble_circular_roof.rs:80:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F52: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:652:0 std::f32::impl$0::sin
// F53: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:861:0 std::f32::impl$0::sin_cos
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs:168:0 glam::f32::math::std_math::sin_cos
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs:124:0 glam::f32::sse2::mat2::Mat2::from_angle
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:676:0 std::f32::impl$0::sin
// F57: crates/systems/roof/src/utils/assemble_circular_roof.rs:91:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F58: crates/systems/roof/src/utils/assemble_circular_roof.rs:92:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F59: crates/systems/roof/src/utils/assemble_circular_roof.rs:90:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F60: crates/systems/roof/src/utils/assemble_circular_roof.rs:96:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F61: crates/systems/roof/src/utils/assemble_circular_roof.rs:99:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F62: crates/systems/roof/src/utils/assemble_circular_roof.rs:210:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F63: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1824:0 core::cmp::impls::impl$58::lt
// F64: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/map.rs:107:0 core::iter::adapters::map::impl$2::next
// F65: crates/systems/roof/src/utils/assemble_circular_roof.rs:105:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F66: crates/systems/roof/src/utils/assemble_circular_roof.rs:99:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// F67: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:305:0 core::ops::function::impls::impl$4::call_once
// F68: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F69: crates/systems/roof/src/utils/assemble_circular_roof.rs:106:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F70: crates/systems/roof/src/utils/assemble_circular_roof.rs:107:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F71: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:949:0 core::core_arch::x86::sse::_mm_set_ps
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs:323:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F73: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:82:0 core::core_arch::x86::sse::_mm_mul_ps
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs:324:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F75: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/macros.rs:124:0 core::core_arch::x86::sse::_mm_shuffle_ps
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs:325:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F77: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:34:0 core::core_arch::x86::sse::_mm_add_ps
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs:326:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F79: crates/systems/roof/src/utils/assemble_circular_roof.rs:109:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F80: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:502:0 core::ops::arith::impl$241::div
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1065:0 glam::f32::vec2::impl$8::div
// F82: crates/systems/roof/src/utils/assemble_circular_roof.rs:110:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1335:0 glam::f32::vec3::impl$34::add
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1337:0 glam::f32::vec3::impl$34::add
// F85: crates/systems/roof/src/utils/assemble_circular_roof.rs:118:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F86: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$77::sub
// F87: crates/utils/src/curve/mod.rs:682:0 utils::curve::Curve::get_curve_segment_from_u
// F88: crates/utils/src/curve/mod.rs:793:0 utils::curve::Curve::get_tangent_at_u
// F89: crates/systems/roof/src/utils/assemble_circular_roof.rs:121:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F90: crates/utils/src/curve/mod.rs:683:0 utils::curve::Curve::get_curve_segment_from_u
// F91: crates/utils/src/curve/mod.rs:794:0 utils::curve::Curve::get_tangent_at_u
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1420:0 glam::f32::vec2::impl$50::sub
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1421:0 glam::f32::vec2::impl$50::sub
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:167:0 glam::f32::vec2::Vec2::dot
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:397:0 glam::f32::vec2::Vec2::length
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:416:0 glam::f32::vec2::Vec2::length_recip
// F97: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:468:0 glam::f32::vec2::Vec2::normalize
// F98: crates/utils/src/lib.rs:69:0 utils::impl$0::normalize
// F99: crates/utils/src/curve/mod.rs:796:0 utils::curve::Curve::get_tangent_at_u
// F100: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:370:0 std::f32::impl$0::sqrt
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs:178:0 glam::f32::vec2::Vec2::dot
// F102: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:830:0 glam::f32::vec2::Vec2::length
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1200:0 glam::f32::vec2::Vec2::length_recip
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1201:0 glam::f32::vec2::Vec2::length_recip
// F105: crates/systems/roof/src/utils/assemble_circular_roof.rs:122:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F106: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:790:0 std::f32::impl$0::atan
// F107: crates/systems/roof/src/utils/assemble_circular_roof.rs:125:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F108: crates/systems/roof/src/utils/assemble_circular_roof.rs:127:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F109: crates/systems/roof/src/utils/assemble_circular_roof.rs:128:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:220:0 glam::f32::vec3::Vec3::cross
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:138:0 glam::f32::sse2::quat::Quat::from_axis_angle
// F112: crates/systems/roof/src/utils/assemble_circular_roof.rs:131:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:219:0 glam::f32::vec3::Vec3::cross
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:221:0 glam::f32::vec3::Vec3::cross
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1247:0 glam::f32::vec3::impl$24::mul
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:139:0 glam::f32::sse2::quat::Quat::from_axis_angle
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1249:0 glam::f32::vec3::impl$24::mul
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:72:0 glam::f32::sse2::quat::Quat::from_xyzw
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:140:0 glam::f32::sse2::quat::Quat::from_axis_angle
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs:90:0 glam::f32::sse2::vec3a::Vec3A::new
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs:1869:0 glam::f32::sse2::vec3a::impl$101::from
// F122: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/convert/mod.rs:761:0 core::convert::impl$3::into
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:714:0 glam::f32::sse2::quat::Quat::mul_vec3
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:924:0 glam::f32::sse2::quat::impl$9::mul
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs:43:0 glam::sse2::dot3_in_x
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs:67:0 glam::sse2::dot3_into_m128
// F127: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:793:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs:68:0 glam::sse2::dot3_into_m128
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:796:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F130: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:58:0 core::core_arch::x86::sse::_mm_sub_ps
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs:236:0 glam::f32::sse2::vec3a::Vec3A::cross
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:799:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs:237:0 glam::f32::sse2::vec3a::Vec3A::cross
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs:238:0 glam::f32::sse2::vec3a::Vec3A::cross
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:797:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F136: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:22:0 core::core_arch::x86::sse::_mm_add_ss
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs:47:0 glam::sse2::dot3_in_x
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:795:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs:239:0 glam::f32::sse2::vec3a::Vec3A::cross
// F140: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs:240:0 glam::f32::sse2::vec3a::Vec3A::cross
// F141: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs:241:0 glam::f32::sse2::vec3a::Vec3A::cross
// F142: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:794:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F143: crates/systems/roof/src/utils/assemble_circular_roof.rs:132:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:204:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:268:0 glam::f32::sse2::quat::Quat::from_mat3
// F146: crates/systems/roof/src/utils/assemble_circular_roof.rs:140:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:231:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:232:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F149: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:233:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:245:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F151: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs:178:0 glam::f32::math::std_math::sqrt
// F152: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:246:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F153: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:248:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F154: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:249:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:250:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:251:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F157: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:206:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F158: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:207:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F159: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:208:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F160: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:220:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:221:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:223:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F163: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:224:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F164: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:225:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F165: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:226:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F166: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:235:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:236:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F168: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:238:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F169: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:239:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F170: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:240:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F171: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:241:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F172: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:210:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F173: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:211:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F174: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:213:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F175: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:214:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F176: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:215:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F177: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:216:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F178: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs:0:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F179: crates/systems/roof/src/utils/assemble_circular_roof.rs:142:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F180: crates/systems/roof/src/utils/assemble_circular_roof.rs:144:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F181: crates/systems/roof/src/utils/assemble_circular_roof.rs:138:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F182: crates/systems/roof/src/utils/assemble_circular_roof.rs:167:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F183: crates/systems/roof/src/utils/assemble_circular_roof.rs:168:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F184: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1475:0 glam::f32::vec3::impl$50::sub
// F185: crates/systems/roof/src/utils/assemble_circular_roof.rs:169:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F186: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1477:0 glam::f32::vec3::impl$50::sub
// F187: crates/systems/roof/src/utils/assemble_circular_roof.rs:173:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F188: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:478:0 glam::f32::vec3::Vec3::distance
// F189: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:204:0 glam::f32::vec3::Vec3::dot
// F190: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:452:0 glam::f32::vec3::impl$50::sub
// F191: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs:178:0 glam::f32::vec3::Vec3::dot
// F192: crates/systems/roof/src/utils/assemble_circular_roof.rs:175:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F193: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:76:0 glam::f32::vec3::Vec3::new
// F194: crates/systems/roof/src/utils/assemble_circular_roof.rs:179:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F195: crates/systems/roof/src/utils/assemble_circular_roof.rs:194:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F196: crates/systems/roof/src/utils/assemble_circular_roof.rs:196:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F197: crates/systems/roof/src/utils/assemble_circular_roof.rs:197:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F198: crates/systems/roof/src/utils/assemble_circular_roof.rs:192:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F199: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:751:0 alloc::raw_vec::RawVecInner::deallocate
// F200: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:402:0 alloc::raw_vec::impl$3::drop
// F201: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:523:0 core::ptr::drop_in_place
// F202: crates/systems/roof/src/utils/assemble_circular_roof.rs:212:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F203: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:523:0 alloc::raw_vec::RawVecInner::current_memory
// F204: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:945:0 core::num::impl$11::unchecked_mul
// F205: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:531:0 alloc::raw_vec::RawVecInner::current_memory
// F206: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:116:0 alloc::alloc::dealloc
// F207: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:269:0 alloc::alloc::impl$1::deallocate
// F208: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:753:0 alloc::raw_vec::RawVecInner::deallocate
// F209: crates/systems/roof/src/utils/assemble_circular_roof.rs:213:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hd5806efb2ba59b33
// F210: crates/utils/src/curve/curve_u.rs:90:0 utils::curve::curve_u::CurveU::new
// F211: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/mod.rs:613:0 core::fmt::Arguments::new_v1
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/utils/assemble_circular_roof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 54aa41bfd109274f9f5b9f325464e5921591aeb79304c08ac72c40fd63c826ea
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: assemble_circular_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3475
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3475,"record_offset":2160,"rva":35336480,"va":5404045600}
// 编译实例 RVAs: 0x21b3120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29236974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e812e141fe","size":5,"soft_fail":false,"target_va":5374813424,"va":5404050393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28967558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e87afd45fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5404049057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9f20
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26188845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8d36370fe","size":5,"soft_fail":false,"target_va":5377859360,"va":5404048200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8ffb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22166378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e896c4adfe","size":5,"soft_fail":false,"target_va":5381881776,"va":5404048149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22163329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e87fd0adfe","size":5,"soft_fail":false,"target_va":5381883040,"va":5404046364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22134440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e85841aefe","size":5,"soft_fail":false,"target_va":5381911904,"va":5404046339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22112435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e84d97aefe","size":5,"soft_fail":false,"target_va":5381934528,"va":5404046958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22111743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8019aaefe","size":5,"soft_fail":false,"target_va":5381934560,"va":5404046298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420071b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1755414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8ea36e5ff","size":5,"soft_fail":false,"target_va":5402292656,"va":5404048065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142197610
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-113918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e80243feff","size":5,"soft_fail":false,"target_va":5403932176,"va":5404046089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7800569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8f9067700","size":5,"soft_fail":false,"target_va":5411847728,"va":5404047154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7801935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e84f0c7700","size":5,"soft_fail":false,"target_va":5411847776,"va":5404045836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7801272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8b8097700","size":5,"soft_fail":false,"target_va":5411847808,"va":5404046531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7801621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8150b7700","size":5,"soft_fail":false,"target_va":5411848144,"va":5404046518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t7492622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e80e547200","size":5,"soft_fail":false,"target_va":5411542064,"va":5404049437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t7492376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e818537200","size":5,"soft_fail":false,"target_va":5411542296,"va":5404049915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7492491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e88b537200","size":5,"soft_fail":false,"target_va":5411541904,"va":5404049408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26017664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8800073fe","size":5,"soft_fail":false,"target_va":5378028128,"va":5404045787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26017208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8480273fe","size":5,"soft_fail":false,"target_va":5378028576,"va":5404045779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26016792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8e80373fe","size":5,"soft_fail":false,"target_va":5378029440,"va":5404046227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::profile_curve_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26014905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8470b73fe","size":5,"soft_fail":false,"target_va":5378030912,"va":5404045812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_circle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24293318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e83a508dfe","size":5,"soft_fail":false,"target_va":5379752448,"va":5404045761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24293630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8024f8dfe","size":5,"soft_fail":false,"target_va":5379752128,"va":5404045753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22036697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e827bfaffe","size":5,"soft_fail":false,"target_va":5382009728,"va":5404046420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e828090000","size":5,"soft_fail":false,"target_va":5404050704,"va":5404048355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22134435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e85d41aefe","size":5,"soft_fail":false,"target_va":5381912800,"va":5404047230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22134888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e8983faefe","size":5,"soft_fail":false,"target_va":5381911968,"va":5404046851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22133465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e82745aefe","size":5,"soft_fail":false,"target_va":5381912944,"va":5404046404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22163887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404045600,"raw_hex":"e851ceadfe","size":5,"soft_fail":false,"target_va":5381882704,"va":5404046586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3475,"record_offset":3976,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3475,"record_offset":2544,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3475,"record_offset":4236,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3475,"record_offset":2572,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3475,"record_offset":8916,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":3475,"record_offset":4120,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":32466,"source_line":123,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":3475,"record_offset":5104,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3475,"record_offset":6508,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":24914,"source_line":136,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3475,"record_offset":7688,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3475,"record_offset":6748,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":25347,"source_line":923,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":3864,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":3920,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":5312,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":6468,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":8432,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":8656,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":8112,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":5368,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":8344,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3475,"record_offset":4008,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3475,"record_offset":6076,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":33090,"source_line":789,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3475,"record_offset":2516,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3475,"record_offset":2868,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3475,"record_offset":5540,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3475,"record_offset":2488,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3475,"record_offset":2656,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3475,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":2160,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3475,"record_offset":9080,"parent_record_offset":8988,"ancestor_record_offset":2160,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3475,"record_offset":9016,"parent_record_offset":8988,"ancestor_record_offset":2160,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3475,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":2160,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3475,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":2160,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3475,"record_offset":2956,"parent_record_offset":2896,"ancestor_record_offset":2160,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3475,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":2160,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3475,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":2160,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3475,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":2160,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3475,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":2160,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3475,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":2160,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3475,"record_offset":5224,"parent_record_offset":5104,"ancestor_record_offset":2160,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3475,"record_offset":7436,"parent_record_offset":7380,"ancestor_record_offset":2160,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3475,"record_offset":5164,"parent_record_offset":5104,"ancestor_record_offset":2160,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3475,"record_offset":5136,"parent_record_offset":5104,"ancestor_record_offset":2160,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3475,"record_offset":5192,"parent_record_offset":5104,"ancestor_record_offset":2160,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3475,"record_offset":7072,"parent_record_offset":6880,"ancestor_record_offset":2160,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3475,"record_offset":2808,"parent_record_offset":2656,"ancestor_record_offset":2160,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3475,"record_offset":2836,"parent_record_offset":2656,"ancestor_record_offset":2160,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3475,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":2160,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3475,"record_offset":4272,"parent_record_offset":4236,"ancestor_record_offset":2160,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3475,"record_offset":9048,"parent_record_offset":9016,"ancestor_record_offset":2160,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3475,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":2160,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3475,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":2160,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3475,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":2160,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3475,"record_offset":5452,"parent_record_offset":5428,"ancestor_record_offset":2160,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3475,"record_offset":4396,"parent_record_offset":4356,"ancestor_record_offset":2160,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3475,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":2160,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3475,"record_offset":2696,"parent_record_offset":2656,"ancestor_record_offset":2160,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3475,"record_offset":4356,"parent_record_offset":4236,"ancestor_record_offset":2160,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3475,"record_offset":8940,"parent_record_offset":8916,"ancestor_record_offset":2160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3475,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2160,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3475,"record_offset":3012,"parent_record_offset":2984,"ancestor_record_offset":2160,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":3475,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":2160,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":3475,"record_offset":7820,"parent_record_offset":7712,"ancestor_record_offset":2160,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3475,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":2160,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3475,"record_offset":6716,"parent_record_offset":6508,"ancestor_record_offset":2160,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3475,"record_offset":6772,"parent_record_offset":6748,"ancestor_record_offset":2160,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3475,"record_offset":6880,"parent_record_offset":6772,"ancestor_record_offset":2160,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3475,"record_offset":7128,"parent_record_offset":6880,"ancestor_record_offset":2160,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3475,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":2160,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3475,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":2160,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":5936,"parent_record_offset":5904,"ancestor_record_offset":2160,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":2160,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":2160,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2160,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":2160,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2160,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":3792,"parent_record_offset":3684,"ancestor_record_offset":2160,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3475,"record_offset":5736,"parent_record_offset":5540,"ancestor_record_offset":2160,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":8544,"parent_record_offset":8456,"ancestor_record_offset":2160,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":6628,"parent_record_offset":6508,"ancestor_record_offset":2160,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3475,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":2160,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":3475,"record_offset":6952,"parent_record_offset":6924,"ancestor_record_offset":2160,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":3475,"record_offset":6924,"parent_record_offset":6880,"ancestor_record_offset":2160,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3475,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":2160,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3475,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":2160,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3475,"record_offset":5968,"parent_record_offset":5936,"ancestor_record_offset":2160,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/utils/assemble_circular_roof.rs
//     evidence={"module":3475,"record_offset":4436,"parent_record_offset":4396,"ancestor_record_offset":2160,"inlinee":116744,"source_line":99,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3475,"record_offset":5580,"parent_record_offset":5540,"ancestor_record_offset":2160,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3475,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":2160,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3475,"record_offset":2928,"parent_record_offset":2896,"ancestor_record_offset":2160,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3475,"record_offset":3052,"parent_record_offset":2896,"ancestor_record_offset":2160,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3475,"record_offset":4568,"parent_record_offset":4436,"ancestor_record_offset":2160,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3475,"record_offset":3176,"parent_record_offset":2896,"ancestor_record_offset":2160,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3475,"record_offset":5824,"parent_record_offset":5540,"ancestor_record_offset":2160,"inlinee":25387,"source_line":68,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C808
// 首条位置: {"module":3475,"inlinee":116744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3475,"record_offset":4472,"parent_record_offset":4436,"ancestor_record_offset":4436,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3475,"record_offset":4568,"parent_record_offset":4436,"ancestor_record_offset":4436,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3475,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":4436,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3475,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4436,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3475,"record_offset":4720,"parent_record_offset":4568,"ancestor_record_offset":4436,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3475,"record_offset":4748,"parent_record_offset":4568,"ancestor_record_offset":4436,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3475,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":4436,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3475,"record_offset":4608,"parent_record_offset":4568,"ancestor_record_offset":4436,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3475,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":4436,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3475,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4436,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03475.jsonl.gz"}
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
// 完整结构及原始类型记录: assemble_circular_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: assemble_circular_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: assemble_circular_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// 信息 ID: 45546ab97446c60fc35e497c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::utils::assemble_circular_roof::assemble_circular_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>; module=3475, inline_record=4436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>
// 信息 ID: c3bf8d93e90d11bf05a5fb11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_roof::utils::assemble_circular_roof::assemble_circular_roof -> ["system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles @ 0x1a36efb -> 0x21b3120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
