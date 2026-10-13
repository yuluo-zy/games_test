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
// Inline site 754:4464 -> host 754:2364 / rva_8e0400_len_1411; exact_symbolizer_function_frame
//   Function-frame RVA span [0x8e0843, 0x8e0888), 12 instructions
//   Function-frame RVA span [0x8e088b, 0x8e08c1), 10 instructions
//   Function-frame RVA span [0x8e123a, 0x8e1268), 10 instructions
//   Function-frame RVA span [0x8e12d1, 0x8e13ed), 40 instructions
//
// FUNCTION system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 754:2364 -> rva_8e0400_len_1411
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_8e0400_len_1411: RVA 0x8e0400, 5137 PDB bytes; 1013 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_8e0400_len_1411.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 754:2364: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>
// Frames: F1
// 0x1408e0400  55                             pushq	%rbp
// 0x1408e0401  4157                           pushq	%r15
// 0x1408e0403  4156                           pushq	%r14
// 0x1408e0405  4155                           pushq	%r13
// 0x1408e0407  4154                           pushq	%r12
// 0x1408e0409  56                             pushq	%rsi
// 0x1408e040a  57                             pushq	%rdi
// 0x1408e040b  53                             pushq	%rbx
// 0x1408e040c  4881ecf8020000                 subq	$0x2f8, %rsp            # imm = 0x2F8
// 0x1408e0413  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1408e041b  440f29bd60020000               movaps	%xmm15, 0x260(%rbp)
// 0x1408e0423  440f29b550020000               movaps	%xmm14, 0x250(%rbp)
// 0x1408e042b  440f29ad40020000               movaps	%xmm13, 0x240(%rbp)
// 0x1408e0433  440f29a530020000               movaps	%xmm12, 0x230(%rbp)
// 0x1408e043b  440f299d20020000               movaps	%xmm11, 0x220(%rbp)
// 0x1408e0443  440f299510020000               movaps	%xmm10, 0x210(%rbp)
// 0x1408e044b  440f298d00020000               movaps	%xmm9, 0x200(%rbp)
// 0x1408e0453  440f2985f0010000               movaps	%xmm8, 0x1f0(%rbp)
// 0x1408e045b  0f29bde0010000                 movaps	%xmm7, 0x1e0(%rbp)
// 0x1408e0462  0f29b5d0010000                 movaps	%xmm6, 0x1d0(%rbp)
// 0x1408e0469  48c785c8010000feffffff         movq	$-0x2, 0x1c8(%rbp)
// 0x1408e0474  4c898d00010000                 movq	%r9, 0x100(%rbp)
// 0x1408e047b  4588c6                         movb	%r8b, %r14b
// 0x1408e047e  4889d3                         movq	%rdx, %rbx
// 0x1408e0481  4889cf                         movq	%rcx, %rdi
// Frames: F2
// 0x1408e0484  48c7855001000000000000         movq	$0x0, 0x150(%rbp)
// 0x1408e048f  488db580010000                 leaq	0x180(%rbp), %rsi
// Frames: F3
// 0x1408e0496  4889f1                         movq	%rsi, %rcx
// 0x1408e0499  e8421dddff                     callq	0x1406b21e0 <.text+0x6b11e0>
// 0x1408e049e  4889f1                         movq	%rsi, %rcx
// 0x1408e04a1  e87a1eddff                     callq	0x1406b2320 <.text+0x6b1320>
// 0x1408e04a6  0f28f0                         movaps	%xmm0, %xmm6
// 0x1408e04a9  0f28f9                         movaps	%xmm1, %xmm7
// 0x1408e04ac  488d4d08                       leaq	0x8(%rbp), %rcx
// Frames: F4
// 0x1408e04b0  4889da                         movq	%rbx, %rdx
// 0x1408e04b3  e838a0ddff                     callq	0x1406ba4f0 <.text+0x6b94f0>
// Frames: F5
// 0x1408e04b8  4889d9                         movq	%rbx, %rcx
// 0x1408e04bb  e8709fddff                     callq	0x1406ba430 <.text+0x6b9430>
// 0x1408e04c0  f30f11442420                   movss	%xmm0, 0x20(%rsp)
// 0x1408e04c6  488d8da8000000                 leaq	0xa8(%rbp), %rcx
// 0x1408e04cd  488d5508                       leaq	0x8(%rbp), %rdx
// 0x1408e04d1  0f28d6                         movaps	%xmm6, %xmm2
// 0x1408e04d4  0f28df                         movaps	%xmm7, %xmm3
// 0x1408e04d7  e8b4a9ddff                     callq	0x1406bae90 <.text+0x6b9e90>
// Frames: F6 -> F7
// 0x1408e04dc  f30f10b5d8000000               movss	0xd8(%rbp), %xmm6
// Frames: F8
// 0x1408e04e4  0f28c6                         movaps	%xmm6, %xmm0
// 0x1408e04e7  f30f5e05d59c9f01               divss	0x19f9cd5(%rip), %xmm0  # 0x1422da1c4
// Frames: F9 -> F8
// 0x1408e04ef  e8fc478601                     callq	0x142144cf0 <.text+0x2143cf0>
// Frames: F10 -> F8
// 0x1408e04f4  f30f5f05f4c08a01               maxss	0x18ac0f4(%rip), %xmm0  # 0x14218c5f0
// Frames: F11
// 0x1408e04fc  f30f2cc8                       cvttss2si	%xmm0, %ecx
// 0x1408e0500  0f2e05993c8a01                 ucomiss	0x18a3c99(%rip), %xmm0  # 0x1421841a0
// 0x1408e0507  b8ffffff7f                     movl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
// 0x1408e050c  0f47c8                         cmoval	%eax, %ecx
// 0x1408e050f  31c0                           xorl	%eax, %eax
// 0x1408e0511  0f2ec0                         ucomiss	%xmm0, %xmm0
// 0x1408e0514  0f4ac8                         cmovpl	%eax, %ecx
// 0x1408e0517  f30f5ef0                       divss	%xmm0, %xmm6
// 0x1408e051b  f30f580595768e01               addss	0x18e7695(%rip), %xmm0  # 0x1421c7bb8
// 0x1408e0523  f30f118544010000               movss	%xmm0, 0x144(%rbp)
// 0x1408e052b  f20f1007                       movsd	(%rdi), %xmm0
// 0x1408e052f  0f2945f0                       movaps	%xmm0, -0x10(%rbp)
// 0x1408e0533  f30f59358d9c9f01               mulss	0x19f9c8d(%rip), %xmm6  # 0x1422da1c8
// 0x1408e053b  f30f11b53c010000               movss	%xmm6, 0x13c(%rbp)
// 0x1408e0543  8b03                           movl	(%rbx), %eax
// 0x1408e0545  898574010000                   movl	%eax, 0x174(%rbp)
// 0x1408e054b  89c8                           movl	%ecx, %eax
// 0x1408e054d  f7d8                           negl	%eax
// 0x1408e054f  898538010000                   movl	%eax, 0x138(%rbp)
// 0x1408e0555  450f57f6                       xorps	%xmm14, %xmm14
// 0x1408e0559  f3440f10255e828601             movss	0x186825e(%rip), %xmm12 # 0x1421487c0
// 0x1408e0562  4531ed                         xorl	%r13d, %r13d
// 0x1408e0565  31ff                           xorl	%edi, %edi
// 0x1408e0567  898d48010000                   movl	%ecx, 0x148(%rbp)
// Frames: F12 -> F13 -> F11
// 0x1408e056d  4439e9                         cmpl	%r13d, %ecx
// 0x1408e0570  4489e8                         movl	%r13d, %eax
// 0x1408e0573  0f4fc1                         cmovgl	%ecx, %eax
// 0x1408e0576  41f7dd                         negl	%r13d
// 0x1408e0579  f7d8                           negl	%eax
// 0x1408e057b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F14 -> F12 -> F13 -> F11
// 0x1408e0580  4439e8                         cmpl	%r13d, %eax
// Frames: F12 -> F13 -> F11
// 0x1408e0583  0f84d30b0000                   je	0x1408e115c <.text+0x8e015c>
// Frames: F15
// 0x1408e0589  41ffcd                         decl	%r13d
// 0x1408e058c  4439ad38010000                 cmpl	%r13d, 0x138(%rbp)
// 0x1408e0593  74eb                           je	0x1408e0580 <.text+0x8df580>
// Frames: F16
// 0x1408e0595  4489e8                         movl	%r13d, %eax
// 0x1408e0598  f7d0                           notl	%eax
// 0x1408e059a  0f57c9                         xorps	%xmm1, %xmm1
// 0x1408e059d  f30f2ac8                       cvtsi2ss	%eax, %xmm1
// 0x1408e05a1  f30f5e8d44010000               divss	0x144(%rbp), %xmm1
// 0x1408e05a9  f30f118d60010000               movss	%xmm1, 0x160(%rbp)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F16
// 0x1408e05b1  410f2ece                       ucomiss	%xmm14, %xmm1
// Frames: F21 -> F16
// 0x1408e05b5  0f82c00e0000                   jb	0x1408e147b <.text+0x8e047b>
// 0x1408e05bb  440f2ee1                       ucomiss	%xmm1, %xmm12
// 0x1408e05bf  0f82b60e0000                   jb	0x1408e147b <.text+0x8e047b>
// Frames: F22 -> F23 -> F16
// 0x1408e05c5  0f2ec9                         ucomiss	%xmm1, %xmm1
// Frames: F23 -> F16
// 0x1408e05c8  0f8a1b0f0000                   jp	0x1408e14e9 <.text+0x8e04e9>
// Frames: F24 -> F25
// 0x1408e05ce  488d8da8000000                 leaq	0xa8(%rbp), %rcx
// 0x1408e05d5  0f298de0000000                 movaps	%xmm1, 0xe0(%rbp)
// 0x1408e05dc  e83f889e00                     callq	0x1412c8e20 <.text+0x12c7e20>
// 0x1408e05e1  488b95b8000000                 movq	0xb8(%rbp), %rdx
// Frames: F26 -> F27 -> F24 -> F25
// 0x1408e05e8  89c1                           movl	%eax, %ecx
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F25
// 0x1408e05ea  4839ca                         cmpq	%rcx, %rdx
// 0x1408e05ed  0f868d0c0000                   jbe	0x1408e1280 <.text+0x8e0280>
// Frames: F31 -> F27 -> F24 -> F25
// 0x1408e05f3  488d4101                       leaq	0x1(%rcx), %rax
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F25
// 0x1408e05f7  4839d0                         cmpq	%rdx, %rax
// 0x1408e05fa  0f83910c0000                   jae	0x1408e1291 <.text+0x8e0291>
// Frames: F24 -> F25
// 0x1408e0600  440f28d0                       movaps	%xmm0, %xmm10
// 0x1408e0604  48899558010000                 movq	%rdx, 0x158(%rbp)
// 0x1408e060b  4c8ba5b0000000                 movq	0xb0(%rbp), %r12
// Frames: F27 -> F24 -> F25
// 0x1408e0612  f3410f1034cc                   movss	(%r12,%rcx,8), %xmm6
// 0x1408e0618  f3410f107ccc08                 movss	0x8(%r12,%rcx,8), %xmm7
// Frames: F24 -> F32
// 0x1408e061f  488d8da8000000                 leaq	0xa8(%rbp), %rcx
// 0x1408e0626  0f288de0000000                 movaps	0xe0(%rbp), %xmm1
// 0x1408e062d  e8ee879e00                     callq	0x1412c8e20 <.text+0x12c7e20>
// Frames: F26 -> F27 -> F24 -> F32
// 0x1408e0632  89c1                           movl	%eax, %ecx
// 0x1408e0634  488b9558010000                 movq	0x158(%rbp), %rdx
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F32
// 0x1408e063b  4839ca                         cmpq	%rcx, %rdx
// 0x1408e063e  0f86610c0000                   jbe	0x1408e12a5 <.text+0x8e02a5>
// Frames: F31 -> F27 -> F24 -> F32
// 0x1408e0644  488d4101                       leaq	0x1(%rcx), %rax
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F32
// 0x1408e0648  4839d0                         cmpq	%rdx, %rax
// 0x1408e064b  0f83650c0000                   jae	0x1408e12b6 <.text+0x8e02b6>
// Frames: F24 -> F32
// 0x1408e0651  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F27 -> F24 -> F32
// 0x1408e0655  f3450f1044cc04                 movss	0x4(%r12,%rcx,8), %xmm8
// 0x1408e065c  f3450f104ccc0c                 movss	0xc(%r12,%rcx,8), %xmm9
// Frames: F33
// 0x1408e0663  4889d9                         movq	%rbx, %rcx
// 0x1408e0666  e8e5a1ddff                     callq	0x1406ba850 <.text+0x6b9850>
// Frames: F34 -> F35 -> F36 -> F37 -> F27 -> F24 -> F32
// 0x1408e066b  f3450f5cc8                     subss	%xmm8, %xmm9
// Frames: F38 -> F39 -> F36 -> F37 -> F27 -> F24 -> F32
// 0x1408e0670  f3450f59d9                     mulss	%xmm9, %xmm11
// Frames: F40 -> F41 -> F36 -> F37 -> F27 -> F24 -> F32
// 0x1408e0675  f3450f58d8                     addss	%xmm8, %xmm11
// Frames: F38 -> F42 -> F33
// 0x1408e067a  f30f1295e0000000               movsldup	0xe0(%rbp), %xmm2       # xmm2 = mem[0,0,2,2]
// 0x1408e0682  0f14c1                         unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
// 0x1408e0685  0f59c2                         mulps	%xmm2, %xmm0
// Frames: F40 -> F43 -> F33
// 0x1408e0688  0f5845f0                       addps	-0x10(%rbp), %xmm0
// Frames: F33
// 0x1408e068c  0f138580010000                 movlps	%xmm0, 0x180(%rbp)
// 0x1408e0693  488d4d70                       leaq	0x70(%rbp), %rcx
// 0x1408e0697  488d9580010000                 leaq	0x180(%rbp), %rdx
// 0x1408e069e  410f28d3                       movaps	%xmm11, %xmm2
// 0x1408e06a2  e8d9a0efff                     callq	0x1407da780 <.text+0x7d9780>
// Frames: F44
// 0x1408e06a7  f30f5cfe                       subss	%xmm6, %xmm7
// 0x1408e06ab  f3440f59d7                     mulss	%xmm7, %xmm10
// 0x1408e06b0  f3440f58d6                     addss	%xmm6, %xmm10
// 0x1408e06b5  0f57c9                         xorps	%xmm1, %xmm1
// 0x1408e06b8  0f57d2                         xorps	%xmm2, %xmm2
// 0x1408e06bb  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x1408e06c2  410f28da                       movaps	%xmm10, %xmm3
// 0x1408e06c6  e82540ffff                     callq	0x1408d46f0 <.text+0x8d36f0>
// Frames: F45
// 0x1408e06cb  f30f1005d1359501               movss	0x19535d1(%rip), %xmm0  # 0x142233ca4
// 0x1408e06d3  f30f100d55169501               movss	0x1951655(%rip), %xmm1  # 0x142231d30
// 0x1408e06db  410f28d2                       movaps	%xmm10, %xmm2
// 0x1408e06df  e89c90efff                     callq	0x1407d9780 <.text+0x7d8780>
// Frames: F46 -> F45
// 0x1408e06e4  0f57c9                         xorps	%xmm1, %xmm1
// 0x1408e06e7  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F47 -> F45
// 0x1408e06eb  410f28f4                       movaps	%xmm12, %xmm6
// 0x1408e06ef  f30f5df1                       minss	%xmm1, %xmm6
// Frames: F48 -> F49 -> F50
// 0x1408e06f3  440f28d6                       movaps	%xmm6, %xmm10
// 0x1408e06f7  f3440f5915049d8601             mulss	0x1869d04(%rip), %xmm10 # 0x14214a404
// Frames: F51
// 0x1408e0700  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x1408e0707  e84442ffff                     callq	0x1408d4950 <.text+0x8d3950>
// Frames: F52
// 0x1408e070c  440f28d8                       movaps	%xmm0, %xmm11
// 0x1408e0710  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x1408e0717  e894857700                     callq	0x141058cb0 <.text+0x1057cb0>
// Frames: F53
// 0x1408e071c  440f28e8                       movaps	%xmm0, %xmm13
// 0x1408e0720  410f28c4                       movaps	%xmm12, %xmm0
// 0x1408e0724  f30f5cc6                       subss	%xmm6, %xmm0
// 0x1408e0728  f3440f58d0                     addss	%xmm0, %xmm10
// 0x1408e072d  410f28ca                       movaps	%xmm10, %xmm1
// 0x1408e0731  f30f590dcb9c8601               mulss	0x1869ccb(%rip), %xmm1  # 0x14214a404
// Frames: F54
// 0x1408e0739  410f28c3                       movaps	%xmm11, %xmm0
// 0x1408e073d  f30f5ec1                       divss	%xmm1, %xmm0
// Frames: F9 -> F54
// 0x1408e0741  e8aa458601                     callq	0x142144cf0 <.text+0x2143cf0>
// Frames: F10 -> F54
// 0x1408e0746  f30f5f05ca2b9501               maxss	0x1952bca(%rip), %xmm0  # 0x142233318
// Frames: F54
// 0x1408e074e  f30f2cf0                       cvttss2si	%xmm0, %esi
// 0x1408e0752  0f2e05473a8a01                 ucomiss	0x18a3a47(%rip), %xmm0  # 0x1421841a0
// 0x1408e0759  b8ffffff7f                     movl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
// 0x1408e075e  0f47f0                         cmoval	%eax, %esi
// 0x1408e0761  0f2ec0                         ucomiss	%xmm0, %xmm0
// 0x1408e0764  b800000000                     movl	$0x0, %eax
// 0x1408e0769  0f4af0                         cmovpl	%eax, %esi
// Frames: F52
// 0x1408e076c  f3440f592d2bb28601             mulss	0x186b22b(%rip), %xmm13 # 0x14214b9a0
// Frames: F55 -> F56 -> F57 -> F58 -> F52
// 0x1408e0775  410f28c5                       movaps	%xmm13, %xmm0
// 0x1408e0779  e8e2468601                     callq	0x142144e60 <.text+0x2143e60>
// 0x1408e077e  440f28e0                       movaps	%xmm0, %xmm12
// Frames: F59 -> F56 -> F57 -> F58 -> F52
// 0x1408e0782  410f28c5                       movaps	%xmm13, %xmm0
// 0x1408e0786  e885458601                     callq	0x142144d10 <.text+0x2143d10>
// 0x1408e078b  440f28e8                       movaps	%xmm0, %xmm13
// Frames: F60
// 0x1408e078f  4863d6                         movslq	%esi, %rdx
// 0x1408e0792  48ffc2                         incq	%rdx
// Frames: F61
// 0x1408e0795  f3450f59d6                     mulss	%xmm14, %xmm10
// 0x1408e079a  f3450f5ed3                     divss	%xmm11, %xmm10
// Frames: F62
// 0x1408e079f  488d050a9f9f01                 leaq	0x19f9f0a(%rip), %rax   # 0x1422da6b0
// 0x1408e07a6  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1408e07ab  488d8d18010000                 leaq	0x118(%rbp), %rcx
// 0x1408e07b2  410f28d2                       movaps	%xmm10, %xmm2
// 0x1408e07b6  4c8d8d50010000                 leaq	0x150(%rbp), %r9
// 0x1408e07bd  e89e8eefff                     callq	0x1407d9660 <.text+0x7d8660>
// Frames: F63
// 0x1408e07c2  48899d88000000                 movq	%rbx, 0x88(%rbp)
// 0x1408e07c9  488b9d28010000                 movq	0x128(%rbp), %rbx
// 0x1408e07d0  4883fb01                       cmpq	$0x1, %rbx
// 0x1408e07d4  0f86290d0000                   jbe	0x1408e1503 <.text+0x8e0503>
// Frames: F64
// 0x1408e07da  4489e8                         movl	%r13d, %eax
// 0x1408e07dd  f7d8                           negl	%eax
// 0x1408e07df  89854c010000                   movl	%eax, 0x14c(%rbp)
// 0x1408e07e5  410f28c5                       movaps	%xmm13, %xmm0
// 0x1408e07e9  410f14c4                       unpcklps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0],xmm0[1],xmm12[1]
// 0x1408e07ed  440f5725db398a01               xorps	0x18a39db(%rip), %xmm12 # 0x1421841d0
// 0x1408e07f5  450f14e5                       unpcklps	%xmm13, %xmm12          # xmm12 = xmm12[0],xmm13[0],xmm12[1],xmm13[1]
// 0x1408e07f9  410f16c4                       movlhps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0]
// 0x1408e07fd  0f294540                       movaps	%xmm0, 0x40(%rbp)
// 0x1408e0801  488d43ff                       leaq	-0x1(%rbx), %rax
// 0x1408e0805  48898580000000                 movq	%rax, 0x80(%rbp)
// 0x1408e080c  4183fdff                       cmpl	$-0x1, %r13d
// 0x1408e0810  0f94c0                         sete	%al
// 0x1408e0813  4488b5c7010000                 movb	%r14b, 0x1c7(%rbp)
// 0x1408e081a  4408f0                         orb	%r14b, %al
// 0x1408e081d  0fb6c0                         movzbl	%al, %eax
// 0x1408e0820  ffc0                           incl	%eax
// 0x1408e0822  898540010000                   movl	%eax, 0x140(%rbp)
// 0x1408e0828  4531f6                         xorl	%r14d, %r14d
// 0x1408e082b  eb09                           jmp	0x1408e0836 <.text+0x8df836>
// 0x1408e082d  0f1f00                         nopl	(%rax)
// Frames: F65
// 0x1408e0830  ffc7                           incl	%edi
// 0x1408e0832  450f57f6                       xorps	%xmm14, %xmm14
// Frames: F66 -> F12 -> F13 -> F67 -> F68
// 0x1408e0836  4c3bb580000000                 cmpq	0x80(%rbp), %r14
// Frames: F12 -> F13 -> F67 -> F68
// 0x1408e083d  0f83cd080000                   jae	0x1408e1110 <.text+0x8e0110>
// Frames: F28 -> F29 -> F30 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e0843  4c39f3                         cmpq	%r14, %rbx
// 0x1408e0846  f30f1005727f8601               movss	0x1867f72(%rip), %xmm0  # 0x1421487c0
// 0x1408e084e  0f86e6090000                   jbe	0x1408e123a <.text+0x8e023a>
// Frames: F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e0854  488b8520010000                 movq	0x120(%rbp), %rax
// 0x1408e085b  f3460f101cb0                   movss	(%rax,%r14,4), %xmm11
// 0x1408e0861  f3440f119d78010000             movss	%xmm11, 0x178(%rbp)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e086a  450f2ede                       ucomiss	%xmm14, %xmm11
// Frames: F21 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e086e  0f827a0a0000                   jb	0x1408e12ee <.text+0x8e02ee>
// 0x1408e0874  410f2ec3                       ucomiss	%xmm11, %xmm0
// 0x1408e0878  0f82700a0000                   jb	0x1408e12ee <.text+0x8e02ee>
// Frames: F22 -> F23 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e087e  450f2edb                       ucomiss	%xmm11, %xmm11
// Frames: F23 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e0882  0f8a490a0000                   jp	0x1408e12d1 <.text+0x8e02d1>
// Frames: F67 -> F68
// 0x1408e0888  49ffc6                         incq	%r14
// Frames: F28 -> F29 -> F30 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e088b  4939de                         cmpq	%rbx, %r14
// 0x1408e088e  0f83bd090000                   jae	0x1408e1251 <.text+0x8e0251>
// Frames: F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e0894  f3460f1024b0                   movss	(%rax,%r14,4), %xmm12
// 0x1408e089a  f3440f11a578010000             movss	%xmm12, 0x178(%rbp)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e08a3  450f2ee6                       ucomiss	%xmm14, %xmm12
// Frames: F21 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e08a7  0f82b20a0000                   jb	0x1408e135f <.text+0x8e035f>
// 0x1408e08ad  410f2ec4                       ucomiss	%xmm12, %xmm0
// 0x1408e08b1  0f82a80a0000                   jb	0x1408e135f <.text+0x8e035f>
// Frames: F22 -> F23 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e08b7  450f2ee4                       ucomiss	%xmm12, %xmm12
// Frames: F23 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e08bb  0f8a0f0b0000                   jp	0x1408e13d0 <.text+0x8e03d0>
// Frames: F72
// 0x1408e08c1  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x1408e08c8  410f28cb                       movaps	%xmm11, %xmm1
// 0x1408e08cc  e85f3effff                     callq	0x1408d4730 <.text+0x8d3730>
// Frames: F73
// 0x1408e08d1  440f28d0                       movaps	%xmm0, %xmm10
// 0x1408e08d5  440f28e9                       movaps	%xmm1, %xmm13
// 0x1408e08d9  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x1408e08e0  410f28cc                       movaps	%xmm12, %xmm1
// 0x1408e08e4  e8473effff                     callq	0x1408d4730 <.text+0x8d3730>
// Frames: F74 -> F75 -> F73
// 0x1408e08e9  450fc6d500                     shufps	$0x0, %xmm13, %xmm10    # xmm10 = xmm10[0,0],xmm13[0,0]
// 0x1408e08ee  0f285540                       movaps	0x40(%rbp), %xmm2
// 0x1408e08f2  440f59d2                       mulps	%xmm2, %xmm10
// 0x1408e08f6  410f28fa                       movaps	%xmm10, %xmm7
// 0x1408e08fa  66410f15fa                     unpckhpd	%xmm10, %xmm7           # xmm7 = xmm7[1],xmm10[1]
// 0x1408e08ff  410f58fa                       addps	%xmm10, %xmm7
// 0x1408e0903  0fc6c100                       shufps	$0x0, %xmm1, %xmm0      # xmm0 = xmm0[0,0],xmm1[0,0]
// Frames: F76 -> F77 -> F73
// 0x1408e0907  0f59c2                         mulps	%xmm2, %xmm0
// Frames: F78 -> F79 -> F73
// 0x1408e090a  440f28c0                       movaps	%xmm0, %xmm8
// 0x1408e090e  66440f15c0                     unpckhpd	%xmm0, %xmm8            # xmm8 = xmm8[1],xmm0[1]
// Frames: F80 -> F81 -> F73
// 0x1408e0913  440f58c0                       addps	%xmm0, %xmm8
// Frames: F40 -> F43 -> F82
// 0x1408e0917  0f28c7                         movaps	%xmm7, %xmm0
// 0x1408e091a  410f58c0                       addps	%xmm8, %xmm0
// Frames: F83 -> F84 -> F82
// 0x1408e091e  0f5905cb728e01                 mulps	0x18e72cb(%rip), %xmm0  # 0x1421c7bf0
// Frames: F82
// 0x1408e0925  0f138578010000                 movlps	%xmm0, 0x178(%rbp)
// Frames: F85
// 0x1408e092c  488d8d80010000                 leaq	0x180(%rbp), %rcx
// 0x1408e0933  488d9578010000                 leaq	0x178(%rbp), %rdx
// 0x1408e093a  e8219eefff                     callq	0x1407da760 <.text+0x7d9760>
// 0x1408e093f  f3440f107578                   movss	0x78(%rbp), %xmm14
// 0x1408e0945  f2440f107d70                   movsd	0x70(%rbp), %xmm15
// Frames: F86 -> F85
// 0x1408e094b  f20f108580010000               movsd	0x180(%rbp), %xmm0
// 0x1408e0953  0f2945d0                       movaps	%xmm0, -0x30(%rbp)
// Frames: F40 -> F87 -> F85
// 0x1408e0957  f3440f58b588010000             addss	0x188(%rbp), %xmm14
// Frames: F88
// 0x1408e0960  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x1408e0967  e844837700                     callq	0x141058cb0 <.text+0x1057cb0>
// 0x1408e096c  440f28d0                       movaps	%xmm0, %xmm10
// Frames: F89 -> F49 -> F88
// 0x1408e0970  f30f1035487e8601               movss	0x1867e48(%rip), %xmm6  # 0x1421487c0
// 0x1408e0978  f30f5cf0                       subss	%xmm0, %xmm6
// Frames: F48 -> F49 -> F88
// 0x1408e097c  f30f593584038f01               mulss	0x18f0384(%rip), %xmm6  # 0x1421d0d08
// 0x1408e0984  f3440f591507bb9601             mulss	0x196bb07(%rip), %xmm10 # 0x14224c494
// Frames: F90 -> F91 -> F92
// 0x1408e098d  488d8da8000000                 leaq	0xa8(%rbp), %rcx
// 0x1408e0994  0f288de0000000                 movaps	0xe0(%rbp), %xmm1
// 0x1408e099b  e880849e00                     callq	0x1412c8e20 <.text+0x12c7e20>
// Frames: F26 -> F90 -> F91 -> F92
// 0x1408e09a0  89c0                           movl	%eax, %eax
// Frames: F93 -> F91 -> F92
// 0x1408e09a2  488d4801                       leaq	0x1(%rax), %rcx
// Frames: F28 -> F29 -> F30 -> F94 -> F92
// 0x1408e09a6  483b8d58010000                 cmpq	0x158(%rbp), %rcx
// 0x1408e09ad  0f83b5080000                   jae	0x1408e1268 <.text+0x8e0268>
// Frames: F94 -> F92
// 0x1408e09b3  f3410f104cc408                 movss	0x8(%r12,%rax,8), %xmm1
// 0x1408e09ba  f3410f1054c40c                 movss	0xc(%r12,%rax,8), %xmm2
// Frames: F34 -> F95 -> F94 -> F92
// 0x1408e09c1  f3410f5c0cc4                   subss	(%r12,%rax,8), %xmm1
// Frames: F34 -> F35 -> F94 -> F92
// 0x1408e09c7  f3410f5c54c404                 subss	0x4(%r12,%rax,8), %xmm2
// Frames: F96 -> F97 -> F98 -> F99 -> F100 -> F101 -> F92
// 0x1408e09ce  0f28c1                         movaps	%xmm1, %xmm0
// 0x1408e09d1  f30f59c1                       mulss	%xmm1, %xmm0
// 0x1408e09d5  0f28da                         movaps	%xmm2, %xmm3
// 0x1408e09d8  f30f59da                       mulss	%xmm2, %xmm3
// 0x1408e09dc  f30f58d8                       addss	%xmm0, %xmm3
// Frames: F102 -> F103 -> F97 -> F98 -> F99 -> F100 -> F101 -> F92
// 0x1408e09e0  f30f51db                       sqrtss	%xmm3, %xmm3
// 0x1408e09e4  f3440f100dd37d8601             movss	0x1867dd3(%rip), %xmm9  # 0x1421487c0
// Frames: F104 -> F98 -> F99 -> F100 -> F101 -> F92
// 0x1408e09ed  410f28c1                       movaps	%xmm9, %xmm0
// 0x1408e09f1  f30f5ec3                       divss	%xmm3, %xmm0
// Frames: F38 -> F105 -> F99 -> F100 -> F101 -> F92
// 0x1408e09f5  f30f59c8                       mulss	%xmm0, %xmm1
// Frames: F38 -> F106 -> F99 -> F100 -> F101 -> F92
// 0x1408e09f9  f30f59c2                       mulss	%xmm2, %xmm0
// Frames: F107
// 0x1408e09fd  f30f5ec1                       divss	%xmm1, %xmm0
// Frames: F108 -> F107
// 0x1408e0a01  e8ba428601                     callq	0x142144cc0 <.text+0x2143cc0>
// Frames: F109
// 0x1408e0a06  f3450f58dc                     addss	%xmm12, %xmm11
// 0x1408e0a0b  f3440f591df0998601             mulss	0x18699f0(%rip), %xmm11 # 0x14214a404
// 0x1408e0a14  f3440f119d34010000             movss	%xmm11, 0x134(%rbp)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F109
// 0x1408e0a1d  440f2e1deb718e01               ucomiss	0x18e71eb(%rip), %xmm11 # 0x1421c7c10
// Frames: F21 -> F109
// 0x1408e0a25  0f82df090000                   jb	0x1408e140a <.text+0x8e040a>
// 0x1408e0a2b  450f2ecb                       ucomiss	%xmm11, %xmm9
// 0x1408e0a2f  0f82d5090000                   jb	0x1408e140a <.text+0x8e040a>
// Frames: F22 -> F23 -> F109
// 0x1408e0a35  450f2edb                       ucomiss	%xmm11, %xmm11
// Frames: F23 -> F109
// 0x1408e0a39  0f8aae090000                   jp	0x1408e13ed <.text+0x8e03ed>
// Frames: F110
// 0x1408e0a3f  440f28e8                       movaps	%xmm0, %xmm13
// 0x1408e0a43  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x1408e0a4a  410f28cb                       movaps	%xmm11, %xmm1
// 0x1408e0a4e  e8ad3effff                     callq	0x1408d4900 <.text+0x8d3900>
// Frames: F74 -> F75 -> F110
// 0x1408e0a53  0fc6c100                       shufps	$0x0, %xmm1, %xmm0      # xmm0 = xmm0[0,0],xmm1[0,0]
// Frames: F76 -> F77 -> F110
// 0x1408e0a57  0f594540                       mulps	0x40(%rbp), %xmm0
// Frames: F78 -> F79 -> F110
// 0x1408e0a5b  0f28c8                         movaps	%xmm0, %xmm1
// 0x1408e0a5e  660f15c8                       unpckhpd	%xmm0, %xmm1            # xmm1 = xmm1[1],xmm0[1]
// Frames: F80 -> F81 -> F110
// 0x1408e0a62  0f58c8                         addps	%xmm0, %xmm1
// Frames: F110
// 0x1408e0a65  0f138d80010000                 movlps	%xmm1, 0x180(%rbp)
// 0x1408e0a6c  488d4d60                       leaq	0x60(%rbp), %rcx
// 0x1408e0a70  488d9580010000                 leaq	0x180(%rbp), %rdx
// 0x1408e0a77  e8e49cefff                     callq	0x1407da760 <.text+0x7d9760>
// Frames: F111
// 0x1408e0a7c  f3440f1025239b9f01             movss	0x19f9b23(%rip), %xmm12 # 0x1422da5a8
// 0x1408e0a85  f3450f5ce5                     subss	%xmm13, %xmm12
// 0x1408e0a8a  f3440f106d68                   movss	0x68(%rbp), %xmm13
// Frames: F112 -> F111
// 0x1408e0a90  450f28cd                       movaps	%xmm13, %xmm9
// 0x1408e0a94  0f57c0                         xorps	%xmm0, %xmm0
// 0x1408e0a97  f3440f59c8                     mulss	%xmm0, %xmm9
// Frames: F113 -> F114
// 0x1408e0a9c  f3440f59259b4c8601             mulss	0x1864c9b(%rip), %xmm12 # 0x142145740
// Frames: F55 -> F56 -> F57 -> F113 -> F114
// 0x1408e0aa5  410f28c4                       movaps	%xmm12, %xmm0
// 0x1408e0aa9  e8b2438601                     callq	0x142144e60 <.text+0x2143e60>
// 0x1408e0aae  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F59 -> F56 -> F57 -> F113 -> F114
// 0x1408e0ab2  410f28c4                       movaps	%xmm12, %xmm0
// 0x1408e0ab6  e855428601                     callq	0x142144d10 <.text+0x2143d10>
// Frames: F111
// 0x1408e0abb  f20f105560                     movsd	0x60(%rbp), %xmm2
// Frames: F112 -> F111
// 0x1408e0ac0  0f28ca                         movaps	%xmm2, %xmm1
// 0x1408e0ac3  0f590d56718e01                 mulps	0x18e7156(%rip), %xmm1  # 0x1421c7c20
// Frames: F115 -> F111
// 0x1408e0aca  f30f16e1                       movshdup	%xmm1, %xmm4            # xmm4 = xmm1[1,1,3,3]
// 0x1408e0ace  f3410f5ce5                     subss	%xmm13, %xmm4
// Frames: F112 -> F111
// 0x1408e0ad3  f3440f12e2                     movsldup	%xmm2, %xmm12           # xmm12 = xmm2[0,0,2,2]
// 0x1408e0ad8  f3450f10e1                     movss	%xmm9, %xmm12           # xmm12 = xmm9[0],xmm12[1,2,3]
// 0x1408e0add  440f5ce1                       subps	%xmm1, %xmm12
// Frames: F38 -> F116 -> F117 -> F114
// 0x1408e0ae1  f3410f12db                     movsldup	%xmm11, %xmm3           # xmm3 = xmm11[0,0,2,2]
// Frames: F38 -> F118 -> F117 -> F114
// 0x1408e0ae6  f3440f59dc                     mulss	%xmm4, %xmm11
// Frames: F38 -> F116 -> F117 -> F114
// 0x1408e0aeb  410f59dc                       mulps	%xmm12, %xmm3
// Frames: F119 -> F120 -> F114
// 0x1408e0aef  440f16db                       movlhps	%xmm3, %xmm11           # xmm11 = xmm11[0],xmm3[0]
// 0x1408e0af3  410f28cb                       movaps	%xmm11, %xmm1
// 0x1408e0af7  0fc6cbd8                       shufps	$0xd8, %xmm3, %xmm1     # xmm1 = xmm1[0,2],xmm3[1,3]
// 0x1408e0afb  0fc6c3d4                       shufps	$0xd4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[1,3]
// 0x1408e0aff  440fc6d828                     shufps	$0x28, %xmm0, %xmm11    # xmm11 = xmm11[0,2],xmm0[2,0]
// Frames: F121 -> F122 -> F123 -> F124 -> F125 -> F114
// 0x1408e0b04  410f28c5                       movaps	%xmm13, %xmm0
// 0x1408e0b08  0fc6c230                       shufps	$0x30, %xmm2, %xmm0     # xmm0 = xmm0[0,0],xmm2[3,0]
// 0x1408e0b0c  0f28da                         movaps	%xmm2, %xmm3
// 0x1408e0b0f  0fc6d884                       shufps	$0x84, %xmm0, %xmm3     # xmm3 = xmm3[0,1],xmm0[0,2]
// 0x1408e0b13  410fc6d504                     shufps	$0x4, %xmm13, %xmm2     # xmm2 = xmm2[0,1],xmm13[0,0]
// 0x1408e0b18  440f28ec                       movaps	%xmm4, %xmm13
// Frames: F76 -> F126 -> F127 -> F128 -> F124 -> F125 -> F114
// 0x1408e0b1c  410f28c3                       movaps	%xmm11, %xmm0
// 0x1408e0b20  410f59c3                       mulps	%xmm11, %xmm0
// Frames: F78 -> F129 -> F128 -> F124 -> F125 -> F114
// 0x1408e0b24  f30f16e0                       movshdup	%xmm0, %xmm4            # xmm4 = xmm0[1,1,3,3]
// 0x1408e0b28  f30f58e0                       addss	%xmm0, %xmm4
// 0x1408e0b2c  0f28e8                         movaps	%xmm0, %xmm5
// 0x1408e0b2f  660f15e8                       unpckhpd	%xmm0, %xmm5            # xmm5 = xmm5[1],xmm0[1]
// 0x1408e0b33  f30f58ec                       addss	%xmm4, %xmm5
// 0x1408e0b37  0fc6ed00                       shufps	$0x0, %xmm5, %xmm5      # xmm5 = xmm5[0,0,0,0]
// Frames: F76 -> F130 -> F124 -> F125 -> F114
// 0x1408e0b3b  0fc6c0ff                       shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
// Frames: F131 -> F130 -> F124 -> F125 -> F114
// 0x1408e0b3f  0f5cc5                         subps	%xmm5, %xmm0
// Frames: F76 -> F130 -> F124 -> F125 -> F114
// 0x1408e0b42  0f59c2                         mulps	%xmm2, %xmm0
// Frames: F78 -> F132 -> F133 -> F124 -> F125 -> F114
// 0x1408e0b45  0f28d1                         movaps	%xmm1, %xmm2
// 0x1408e0b48  0fc6d1d2                       shufps	$0xd2, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[1,3]
// Frames: F78 -> F134 -> F133 -> F124 -> F125 -> F114
// 0x1408e0b4c  0f28e3                         movaps	%xmm3, %xmm4
// Frames: F76 -> F135 -> F133 -> F124 -> F125 -> F114
// 0x1408e0b4f  0f59d3                         mulps	%xmm3, %xmm2
// Frames: F76 -> F126 -> F127 -> F136 -> F124 -> F125 -> F114
// 0x1408e0b52  0f59d9                         mulps	%xmm1, %xmm3
// Frames: F137 -> F138 -> F127 -> F136 -> F124 -> F125 -> F114
// 0x1408e0b55  f30f16eb                       movshdup	%xmm3, %xmm5            # xmm5 = xmm3[1,1,3,3]
// 0x1408e0b59  f30f58eb                       addss	%xmm3, %xmm5
// 0x1408e0b5d  0f12db                         movhlps	%xmm3, %xmm3            # xmm3 = xmm3[1,1]
// 0x1408e0b60  f30f58dd                       addss	%xmm5, %xmm3
// Frames: F76 -> F136 -> F124 -> F125 -> F114
// 0x1408e0b64  f30f58db                       addss	%xmm3, %xmm3
// 0x1408e0b68  0fc6db00                       shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
// 0x1408e0b6c  410f59db                       mulps	%xmm11, %xmm3
// Frames: F80 -> F139 -> F124 -> F125 -> F114
// 0x1408e0b70  0f58d8                         addps	%xmm0, %xmm3
// Frames: F78 -> F134 -> F133 -> F124 -> F125 -> F114
// 0x1408e0b73  0fc6e4d2                       shufps	$0xd2, %xmm4, %xmm4     # xmm4 = xmm4[2,0,1,3]
// Frames: F76 -> F140 -> F133 -> F124 -> F125 -> F114
// 0x1408e0b77  0f59e1                         mulps	%xmm1, %xmm4
// Frames: F131 -> F141 -> F133 -> F124 -> F125 -> F114
// 0x1408e0b7a  0f5cd4                         subps	%xmm4, %xmm2
// Frames: F78 -> F142 -> F133 -> F124 -> F125 -> F114
// 0x1408e0b7d  0fc6d2d2                       shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
// Frames: F76 -> F133 -> F124 -> F125 -> F114
// 0x1408e0b81  450f58db                       addps	%xmm11, %xmm11
// 0x1408e0b85  450fc6dbff                     shufps	$0xff, %xmm11, %xmm11   # xmm11 = xmm11[3,3,3,3]
// 0x1408e0b8a  440f59da                       mulps	%xmm2, %xmm11
// Frames: F80 -> F143 -> F124 -> F125 -> F114
// 0x1408e0b8e  440f58db                       addps	%xmm3, %xmm11
// Frames: F115 -> F144
// 0x1408e0b92  450f28cb                       movaps	%xmm11, %xmm9
// 0x1408e0b96  66450f15cb                     unpckhpd	%xmm11, %xmm9           # xmm9 = xmm9[1],xmm11[1]
// 0x1408e0b9b  f3410f16d3                     movshdup	%xmm11, %xmm2           # xmm2 = xmm11[1,1,3,3]
// 0x1408e0ba0  410f28c3                       movaps	%xmm11, %xmm0
// 0x1408e0ba4  410fc6c3e2                     shufps	$0xe2, %xmm11, %xmm0    # xmm0 = xmm0[2,0],xmm11[2,3]
// 0x1408e0ba9  410f59c4                       mulps	%xmm12, %xmm0
// 0x1408e0bad  f3410f16ec                     movshdup	%xmm12, %xmm5           # xmm5 = xmm12[1,1,3,3]
// 0x1408e0bb2  410f28cd                       movaps	%xmm13, %xmm1
// 0x1408e0bb6  410fc6ccd4                     shufps	$0xd4, %xmm12, %xmm1    # xmm1 = xmm1[0,1],xmm12[1,3]
// 0x1408e0bbb  410fc6ccf2                     shufps	$0xf2, %xmm12, %xmm1    # xmm1 = xmm1[2,0],xmm12[3,3]
// 0x1408e0bc0  410f28db                       movaps	%xmm11, %xmm3
// 0x1408e0bc4  410fc6dbe9                     shufps	$0xe9, %xmm11, %xmm3    # xmm3 = xmm3[1,2],xmm11[2,3]
// 0x1408e0bc9  0f59cb                         mulps	%xmm3, %xmm1
// 0x1408e0bcc  0f5cc1                         subps	%xmm1, %xmm0
// Frames: F145 -> F144
// 0x1408e0bcf  0f28ca                         movaps	%xmm2, %xmm1
// 0x1408e0bd2  440f29ad90000000               movaps	%xmm13, 0x90(%rbp)
// 0x1408e0bda  f3410f59cd                     mulss	%xmm13, %xmm1
// 0x1408e0bdf  410f28e4                       movaps	%xmm12, %xmm4
// 0x1408e0be3  f3410f59e3                     mulss	%xmm11, %xmm4
// 0x1408e0be8  f30f5ccc                       subss	%xmm4, %xmm1
// 0x1408e0bec  450f57ed                       xorps	%xmm13, %xmm13
// Frames: F146 -> F147 -> F148
// 0x1408e0bf0  0f57e4                         xorps	%xmm4, %xmm4
// 0x1408e0bf3  0f2ee1                         ucomiss	%xmm1, %xmm4
// 0x1408e0bf6  0f296de0                       movaps	%xmm5, -0x20(%rbp)
// 0x1408e0bfa  7374                           jae	0x1408e0c70 <.text+0x8dfc70>
// Frames: F149 -> F147 -> F148
// 0x1408e0bfc  0f28a590000000                 movaps	0x90(%rbp), %xmm4
// 0x1408e0c03  f30f58e2                       addss	%xmm2, %xmm4
// Frames: F150 -> F147 -> F148
// 0x1408e0c07  f30f1015b17b8601               movss	0x1867bb1(%rip), %xmm2  # 0x1421487c0
// 0x1408e0c0f  f30f58ca                       addss	%xmm2, %xmm1
// Frames: F151 -> F147 -> F148
// 0x1408e0c13  440f2eec                       ucomiss	%xmm4, %xmm13
// 0x1408e0c17  0f83c3000000                   jae	0x1408e0ce0 <.text+0x8dfce0>
// Frames: F152 -> F147 -> F148
// 0x1408e0c1d  f30f58e1                       addss	%xmm1, %xmm4
// Frames: F102 -> F153 -> F154 -> F147 -> F148
// 0x1408e0c21  0f57c9                         xorps	%xmm1, %xmm1
// 0x1408e0c24  f30f51cc                       sqrtss	%xmm4, %xmm1
// Frames: F154 -> F147 -> F148
// 0x1408e0c28  f30f101dd4978601               movss	0x18697d4(%rip), %xmm3  # 0x14214a404
// 0x1408e0c30  f30f5ed9                       divss	%xmm1, %xmm3
// Frames: F155 -> F147 -> F148
// 0x1408e0c34  410f28c9                       movaps	%xmm9, %xmm1
// 0x1408e0c38  0f14c8                         unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
// 0x1408e0c3b  410f28d4                       movaps	%xmm12, %xmm2
// 0x1408e0c3f  0fc6d011                       shufps	$0x11, %xmm0, %xmm2     # xmm2 = xmm2[1,0],xmm0[1,0]
// 0x1408e0c43  0fc6d0e2                       shufps	$0xe2, %xmm0, %xmm2     # xmm2 = xmm2[2,0],xmm0[2,3]
// 0x1408e0c47  0f5cca                         subps	%xmm2, %xmm1
// 0x1408e0c4a  f30f12d3                       movsldup	%xmm3, %xmm2            # xmm2 = xmm3[0,0,2,2]
// 0x1408e0c4e  0f59d1                         mulps	%xmm1, %xmm2
// Frames: F156 -> F147 -> F148
// 0x1408e0c51  410f28cc                       movaps	%xmm12, %xmm1
// 0x1408e0c55  f3410f5ccb                     subss	%xmm11, %xmm1
// 0x1408e0c5a  f30f59cb                       mulss	%xmm3, %xmm1
// 0x1408e0c5e  e9fe000000                     jmp	0x1408e0d61 <.text+0x8dfd61>
// 0x1408e0c63  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F157 -> F147 -> F148
// 0x1408e0c70  f30f5c9590000000               subss	0x90(%rbp), %xmm2
// Frames: F158 -> F147 -> F148
// 0x1408e0c78  f30f101d407b8601               movss	0x1867b40(%rip), %xmm3  # 0x1421487c0
// 0x1408e0c80  f30f5cd9                       subss	%xmm1, %xmm3
// Frames: F159 -> F147 -> F148
// 0x1408e0c84  440f2eea                       ucomiss	%xmm2, %xmm13
// 0x1408e0c88  0f8392000000                   jae	0x1408e0d20 <.text+0x8dfd20>
// Frames: F160 -> F147 -> F148
// 0x1408e0c8e  410f16d4                       movlhps	%xmm12, %xmm2           # xmm2 = xmm2[0],xmm12[0]
// 0x1408e0c92  410fc6d4e2                     shufps	$0xe2, %xmm12, %xmm2    # xmm2 = xmm2[2,0],xmm12[2,3]
// 0x1408e0c97  410f16db                       movlhps	%xmm11, %xmm3           # xmm3 = xmm3[0],xmm11[0]
// 0x1408e0c9b  410fc6dbe2                     shufps	$0xe2, %xmm11, %xmm3    # xmm3 = xmm3[2,0],xmm11[2,3]
// 0x1408e0ca0  0f58da                         addps	%xmm2, %xmm3
// Frames: F102 -> F153 -> F161 -> F147 -> F148
// 0x1408e0ca3  f30f16cb                       movshdup	%xmm3, %xmm1            # xmm1 = xmm3[1,1,3,3]
// 0x1408e0ca7  f30f51c9                       sqrtss	%xmm1, %xmm1
// Frames: F161 -> F147 -> F148
// 0x1408e0cab  f30f102551978601               movss	0x1869751(%rip), %xmm4  # 0x14214a404
// 0x1408e0cb3  f30f5ee1                       divss	%xmm1, %xmm4
// Frames: F160 -> F147 -> F148
// 0x1408e0cb7  f30f12d4                       movsldup	%xmm4, %xmm2            # xmm2 = xmm4[0,0,2,2]
// 0x1408e0cbb  0f59d3                         mulps	%xmm3, %xmm2
// Frames: F162 -> F147 -> F148
// 0x1408e0cbe  f30f16c8                       movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
// 0x1408e0cc2  f3410f58c9                     addss	%xmm9, %xmm1
// 0x1408e0cc7  f30f59cc                       mulss	%xmm4, %xmm1
// Frames: F163 -> F147 -> F148
// 0x1408e0ccb  f30f5cc5                       subss	%xmm5, %xmm0
// 0x1408e0ccf  f30f59c4                       mulss	%xmm4, %xmm0
// 0x1408e0cd3  0f28d8                         movaps	%xmm0, %xmm3
// 0x1408e0cd6  e98a000000                     jmp	0x1408e0d65 <.text+0x8dfd65>
// 0x1408e0cdb  0f1f440000                     nopl	(%rax,%rax)
// Frames: F164 -> F147 -> F148
// 0x1408e0ce0  f30f5ccc                       subss	%xmm4, %xmm1
// Frames: F102 -> F153 -> F165 -> F147 -> F148
// 0x1408e0ce4  0f57d2                         xorps	%xmm2, %xmm2
// 0x1408e0ce7  f30f51d1                       sqrtss	%xmm1, %xmm2
// Frames: F165 -> F147 -> F148
// 0x1408e0ceb  f30f102511978601               movss	0x1869711(%rip), %xmm4  # 0x14214a404
// 0x1408e0cf3  f30f5ee2                       divss	%xmm2, %xmm4
// Frames: F166 -> F147 -> F148
// 0x1408e0cf7  410fc6dc11                     shufps	$0x11, %xmm12, %xmm3    # xmm3 = xmm3[1,0],xmm12[1,0]
// 0x1408e0cfc  410fc6dce2                     shufps	$0xe2, %xmm12, %xmm3    # xmm3 = xmm3[2,0],xmm12[2,3]
// 0x1408e0d01  0f58d8                         addps	%xmm0, %xmm3
// 0x1408e0d04  f30f12d4                       movsldup	%xmm4, %xmm2            # xmm2 = xmm4[0,0,2,2]
// 0x1408e0d08  0f59d3                         mulps	%xmm3, %xmm2
// Frames: F167 -> F147 -> F148
// 0x1408e0d0b  f30f59cc                       mulss	%xmm4, %xmm1
// Frames: F168 -> F147 -> F148
// 0x1408e0d0f  410f28dc                       movaps	%xmm12, %xmm3
// 0x1408e0d13  f3410f5cdb                     subss	%xmm11, %xmm3
// 0x1408e0d18  eb47                           jmp	0x1408e0d61 <.text+0x8dfd61>
// 0x1408e0d1a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F169 -> F147 -> F148
// 0x1408e0d20  f30f5cda                       subss	%xmm2, %xmm3
// Frames: F102 -> F153 -> F170 -> F147 -> F148
// 0x1408e0d24  0f57c9                         xorps	%xmm1, %xmm1
// 0x1408e0d27  f30f51cb                       sqrtss	%xmm3, %xmm1
// Frames: F170 -> F147 -> F148
// 0x1408e0d2b  f30f1025d1968601               movss	0x18696d1(%rip), %xmm4  # 0x14214a404
// 0x1408e0d33  f30f5ee1                       divss	%xmm1, %xmm4
// Frames: F171 -> F147 -> F148
// 0x1408e0d37  410f28cc                       movaps	%xmm12, %xmm1
// 0x1408e0d3b  f3410f58cb                     addss	%xmm11, %xmm1
// Frames: F172 -> F147 -> F148
// 0x1408e0d40  0f14d9                         unpcklps	%xmm1, %xmm3            # xmm3 = xmm3[0],xmm1[0],xmm3[1],xmm1[1]
// 0x1408e0d43  f30f12d4                       movsldup	%xmm4, %xmm2            # xmm2 = xmm4[0,0,2,2]
// 0x1408e0d47  0f59d3                         mulps	%xmm3, %xmm2
// Frames: F173 -> F147 -> F148
// 0x1408e0d4a  0f28cd                         movaps	%xmm5, %xmm1
// 0x1408e0d4d  f30f58c8                       addss	%xmm0, %xmm1
// 0x1408e0d51  f30f59cc                       mulss	%xmm4, %xmm1
// Frames: F174 -> F147 -> F148
// 0x1408e0d55  f30f16c0                       movshdup	%xmm0, %xmm0            # xmm0 = xmm0[1,1,3,3]
// 0x1408e0d59  410f28d9                       movaps	%xmm9, %xmm3
// 0x1408e0d5d  f30f5cd8                       subss	%xmm0, %xmm3
// 0x1408e0d61  f30f59dc                       mulss	%xmm4, %xmm3
// Frames: F119 -> F175 -> F147 -> F148
// 0x1408e0d65  0f16d9                         movlhps	%xmm1, %xmm3            # xmm3 = xmm3[0],xmm1[0]
// 0x1408e0d68  0fc6d324                       shufps	$0x24, %xmm3, %xmm2     # xmm2 = xmm2[0,1],xmm3[2,0]
// Frames: F148
// 0x1408e0d6c  0f2955c0                       movaps	%xmm2, -0x40(%rbp)
// 0x1408e0d70  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1408e0d74  e867809e00                     callq	0x1412c8de0 <.text+0x12c7de0>
// Frames: F176
// 0x1408e0d79  4989c7                         movq	%rax, %r15
// 0x1408e0d7c  410f5cf8                       subps	%xmm8, %xmm7
// 0x1408e0d80  0f59ff                         mulps	%xmm7, %xmm7
// 0x1408e0d83  f30f16c7                       movshdup	%xmm7, %xmm0            # xmm0 = xmm7[1,1,3,3]
// 0x1408e0d87  f30f58c7                       addss	%xmm7, %xmm0
// 0x1408e0d8b  0f57ff                         xorps	%xmm7, %xmm7
// 0x1408e0d8e  f30f51f8                       sqrtss	%xmm0, %xmm7
// 0x1408e0d92  f3440f58d6                     addss	%xmm6, %xmm10
// 0x1408e0d97  f3440f59953c010000             mulss	0x13c(%rbp), %xmm10
// 0x1408e0da0  f30f11bd80010000               movss	%xmm7, 0x180(%rbp)
// 0x1408e0da8  f3440f119584010000             movss	%xmm10, 0x184(%rbp)
// 0x1408e0db1  c78588010000cdcccc3d           movl	$0x3dcccccd, 0x188(%rbp) # imm = 0x3DCCCCCD
// 0x1408e0dbb  488d8d80010000                 leaq	0x180(%rbp), %rcx
// 0x1408e0dc2  e889ce3d00                     callq	0x140cbdc50 <.text+0xcbcc50>
// Frames: F177
// 0x1408e0dc7  4889c6                         movq	%rax, %rsi
// 0x1408e0dca  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x1408e0dd1  e8da7e7700                     callq	0x141058cb0 <.text+0x1057cb0>
// 0x1408e0dd6  f30f590522a08d01               mulss	0x18da022(%rip), %xmm0  # 0x1421bae00
// 0x1408e0dde  f30f5885e0000000               addss	0xe0(%rbp), %xmm0
// Frames: F46 -> F177
// 0x1408e0de6  0f57c9                         xorps	%xmm1, %xmm1
// 0x1408e0de9  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F47 -> F177
// 0x1408e0ded  f30f1005cb798601               movss	0x18679cb(%rip), %xmm0  # 0x1421487c0
// 0x1408e0df5  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F177
// 0x1408e0df9  e8a26b9000                     callq	0x1411e79a0 <.text+0x11e69a0>
// Frames: F178
// 0x1408e0dfe  440f587dd0                     addps	-0x30(%rbp), %xmm15
// 0x1408e0e03  440f13bd80010000               movlps	%xmm15, 0x180(%rbp)
// 0x1408e0e0b  f3440f11b588010000             movss	%xmm14, 0x188(%rbp)
// 0x1408e0e14  c7858c01000000000000           movl	$0x0, 0x18c(%rbp)
// 0x1408e0e1e  48b90000003c0000ff7b           movabsq	$0x7bff00003c000000, %rcx # imm = 0x7BFF00003C000000
// 0x1408e0e28  48898d90010000                 movq	%rcx, 0x190(%rbp)
// 0x1408e0e2f  89b598010000                   movl	%esi, 0x198(%rbp)
// 0x1408e0e35  48c1ee20                       shrq	$0x20, %rsi
// 0x1408e0e39  6689b59c010000                 movw	%si, 0x19c(%rbp)
// 0x1408e0e40  6689859e010000                 movw	%ax, 0x19e(%rbp)
// 0x1408e0e47  4c89bda0010000                 movq	%r15, 0x1a0(%rbp)
// 0x1408e0e4e  8b8574010000                   movl	0x174(%rbp), %eax
// 0x1408e0e54  8985a8010000                   movl	%eax, 0x1a8(%rbp)
// 0x1408e0e5a  48c785ac01000000000000         movq	$0x0, 0x1ac(%rbp)
// 0x1408e0e65  89bdb4010000                   movl	%edi, 0x1b4(%rbp)
// 0x1408e0e6b  c785b801000001000000           movl	$0x1, 0x1b8(%rbp)
// 0x1408e0e75  8b8540010000                   movl	0x140(%rbp), %eax
// 0x1408e0e7b  8985bc010000                   movl	%eax, 0x1bc(%rbp)
// 0x1408e0e81  488b8d00010000                 movq	0x100(%rbp), %rcx
// 0x1408e0e88  488d9580010000                 leaq	0x180(%rbp), %rdx
// 0x1408e0e8f  e88c090000                     callq	0x1408e1820 <.text+0x8e0820>
// Frames: F179
// 0x1408e0e94  4183fdff                       cmpl	$-0x1, %r13d
// 0x1408e0e98  0f8592f9ffff                   jne	0x1408e0830 <.text+0x8df830>
// Frames: F38 -> F118 -> F180
// 0x1408e0e9e  f3410f12c2                     movsldup	%xmm10, %xmm0           # xmm0 = xmm10[0,0,2,2]
// 0x1408e0ea3  410f59c3                       mulps	%xmm11, %xmm0
// Frames: F38 -> F181 -> F180
// 0x1408e0ea7  f3450f59ca                     mulss	%xmm10, %xmm9
// 0x1408e0eac  f3440f590d4f958601             mulss	0x186954f(%rip), %xmm9  # 0x14214a404
// Frames: F38 -> F118 -> F180
// 0x1408e0eb5  0f5905346d8e01                 mulps	0x18e6d34(%rip), %xmm0  # 0x1421c7bf0
// Frames: F40 -> F86 -> F180
// 0x1408e0ebc  410f28f7                       movaps	%xmm15, %xmm6
// 0x1408e0ec0  0f58f0                         addps	%xmm0, %xmm6
// 0x1408e0ec3  f30f16d6                       movshdup	%xmm6, %xmm2            # xmm2 = xmm6[1,1,3,3]
// Frames: F34 -> F182 -> F183
// 0x1408e0ec7  f3440f5cf8                     subss	%xmm0, %xmm15
// Frames: F34 -> F184 -> F183
// 0x1408e0ecc  410f28c6                       movaps	%xmm14, %xmm0
// 0x1408e0ed0  f3410f5cc1                     subss	%xmm9, %xmm0
// Frames: F183
// 0x1408e0ed5  f3440f11bd80010000             movss	%xmm15, 0x180(%rbp)
// 0x1408e0ede  f30f118584010000               movss	%xmm0, 0x184(%rbp)
// 0x1408e0ee6  488d4d50                       leaq	0x50(%rbp), %rcx
// 0x1408e0eea  488d9580010000                 leaq	0x180(%rbp), %rdx
// 0x1408e0ef1  e88a98efff                     callq	0x1407da780 <.text+0x7d9780>
// Frames: F185
// 0x1408e0ef6  f3450f58f1                     addss	%xmm9, %xmm14
// 0x1408e0efb  f2440f104d50                   movsd	0x50(%rbp), %xmm9
// 0x1408e0f01  f3440f104558                   movss	0x58(%rbp), %xmm8
// Frames: F34 -> F184 -> F186 -> F185
// 0x1408e0f07  410f28c0                       movaps	%xmm8, %xmm0
// 0x1408e0f0b  f3410f5cc6                     subss	%xmm14, %xmm0
// Frames: F34 -> F182 -> F186 -> F185
// 0x1408e0f10  410f28c9                       movaps	%xmm9, %xmm1
// 0x1408e0f14  0f5cce                         subps	%xmm6, %xmm1
// Frames: F187 -> F188 -> F186 -> F185
// 0x1408e0f17  0f59c9                         mulps	%xmm1, %xmm1
// 0x1408e0f1a  f30f16d1                       movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
// 0x1408e0f1e  f30f58d1                       addss	%xmm1, %xmm2
// 0x1408e0f22  f30f59c0                       mulss	%xmm0, %xmm0
// 0x1408e0f26  f30f58c2                       addss	%xmm2, %xmm0
// Frames: F102 -> F189 -> F188 -> F186 -> F185
// 0x1408e0f2a  f30f51d8                       sqrtss	%xmm0, %xmm3
// Frames: F185
// 0x1408e0f2e  0f2e1dd39e8d01                 ucomiss	0x18d9ed3(%rip), %xmm3  # 0x1421bae08
// 0x1408e0f35  0f86f5f8ffff                   jbe	0x1408e0830 <.text+0x8df830>
// Frames: F115 -> F190
// 0x1408e0f3b  410f28d4                       movaps	%xmm12, %xmm2
// 0x1408e0f3f  0f57ed                         xorps	%xmm5, %xmm5
// 0x1408e0f42  f30f59d5                       mulss	%xmm5, %xmm2
// 0x1408e0f46  440f285de0                     movaps	-0x20(%rbp), %xmm11
// 0x1408e0f4b  410f28c3                       movaps	%xmm11, %xmm0
// 0x1408e0f4f  f30f5cc2                       subss	%xmm2, %xmm0
// 0x1408e0f53  440f28ad90000000               movaps	0x90(%rbp), %xmm13
// Frames: F112 -> F190
// 0x1408e0f5b  410f28cd                       movaps	%xmm13, %xmm1
// 0x1408e0f5f  f30f59cd                       mulss	%xmm5, %xmm1
// 0x1408e0f63  410f28e3                       movaps	%xmm11, %xmm4
// 0x1408e0f67  f30f59e5                       mulss	%xmm5, %xmm4
// 0x1408e0f6b  f30f5ccc                       subss	%xmm4, %xmm1
// Frames: F145 -> F190
// 0x1408e0f6f  f3410f5cd5                     subss	%xmm13, %xmm2
// Frames: F191 -> F192
// 0x1408e0f74  f30f11bdf4000000               movss	%xmm7, 0xf4(%rbp)
// 0x1408e0f7c  f30f119df8000000               movss	%xmm3, 0xf8(%rbp)
// 0x1408e0f84  c785fc000000cdcccc3d           movl	$0x3dcccccd, 0xfc(%rbp) # imm = 0x3DCCCCCD
// Frames: F34 -> F182 -> F193 -> F194
// 0x1408e0f8e  410f5cf1                       subps	%xmm9, %xmm6
// Frames: F34 -> F184 -> F193 -> F194
// 0x1408e0f92  f3450f5cf0                     subss	%xmm8, %xmm14
// Frames: F38 -> F195 -> F193 -> F194
// 0x1408e0f97  0f5935526c8e01                 mulps	0x18e6c52(%rip), %xmm6  # 0x1421c7bf0
// 0x1408e0f9e  f3440f10155d948601             movss	0x186945d(%rip), %xmm10 # 0x14214a404
// Frames: F38 -> F196 -> F193 -> F194
// 0x1408e0fa7  f3450f59f2                     mulss	%xmm10, %xmm14
// Frames: F157 -> F147 -> F197
// 0x1408e0fac  f3410f5ccd                     subss	%xmm13, %xmm1
// Frames: F159 -> F147 -> F197
// 0x1408e0fb1  0f2ee9                         ucomiss	%xmm1, %xmm5
// 0x1408e0fb4  734b                           jae	0x1408e1001 <.text+0x8e0001>
// Frames: F198 -> F147 -> F197
// 0x1408e0fb6  f30f103d02788601               movss	0x1867802(%rip), %xmm7  # 0x1421487c0
// 0x1408e0fbe  f30f58cf                       addss	%xmm7, %xmm1
// Frames: F102 -> F153 -> F161 -> F147 -> F197
// 0x1408e0fc2  0f57db                         xorps	%xmm3, %xmm3
// 0x1408e0fc5  f30f51d9                       sqrtss	%xmm1, %xmm3
// Frames: F161 -> F147 -> F197
// 0x1408e0fc9  410f28e2                       movaps	%xmm10, %xmm4
// 0x1408e0fcd  f30f5ee3                       divss	%xmm3, %xmm4
// Frames: F160 -> F147 -> F197
// 0x1408e0fd1  f3410f58c4                     addss	%xmm12, %xmm0
// 0x1408e0fd6  f30f59c4                       mulss	%xmm4, %xmm0
// Frames: F199 -> F147 -> F197
// 0x1408e0fda  f30f59cc                       mulss	%xmm4, %xmm1
// Frames: F162 -> F147 -> F197
// 0x1408e0fde  0f57ed                         xorps	%xmm5, %xmm5
// 0x1408e0fe1  410f5cec                       subps	%xmm12, %xmm5
// 0x1408e0fe5  f3440f10e2                     movss	%xmm2, %xmm12           # xmm12 = xmm2[0],xmm12[1,2,3]
// 0x1408e0fea  440f58e7                       addps	%xmm7, %xmm12
// 0x1408e0fee  410fc6ec01                     shufps	$0x1, %xmm12, %xmm5     # xmm5 = xmm5[1,0],xmm12[0,0]
// 0x1408e0ff3  410fc6ece2                     shufps	$0xe2, %xmm12, %xmm5    # xmm5 = xmm5[2,0],xmm12[2,3]
// 0x1408e0ff8  f30f12dc                       movsldup	%xmm4, %xmm3            # xmm3 = xmm4[0,0,2,2]
// 0x1408e0ffc  0f59dd                         mulps	%xmm5, %xmm3
// 0x1408e0fff  eb45                           jmp	0x1408e1046 <.text+0x8e0046>
// Frames: F169 -> F147 -> F197
// 0x1408e1001  f30f1025b7778601               movss	0x18677b7(%rip), %xmm4  # 0x1421487c0
// 0x1408e1009  f30f5ce1                       subss	%xmm1, %xmm4
// Frames: F102 -> F153 -> F170 -> F147 -> F197
// 0x1408e100d  0f57c9                         xorps	%xmm1, %xmm1
// 0x1408e1010  f30f51cc                       sqrtss	%xmm4, %xmm1
// Frames: F170 -> F147 -> F197
// 0x1408e1014  410f28da                       movaps	%xmm10, %xmm3
// 0x1408e1018  f30f5ed9                       divss	%xmm1, %xmm3
// Frames: F172 -> F147 -> F197
// 0x1408e101c  f30f59e3                       mulss	%xmm3, %xmm4
// Frames: F171 -> F147 -> F197
// 0x1408e1020  f3410f58c4                     addss	%xmm12, %xmm0
// 0x1408e1025  0f28c8                         movaps	%xmm0, %xmm1
// 0x1408e1028  f30f59cb                       mulss	%xmm3, %xmm1
// Frames: F173 -> F147 -> F197
// 0x1408e102c  410f16d3                       movlhps	%xmm11, %xmm2           # xmm2 = xmm2[0],xmm11[0]
// 0x1408e1030  410fc6d3e2                     shufps	$0xe2, %xmm11, %xmm2    # xmm2 = xmm2[2,0],xmm11[2,3]
// 0x1408e1035  0f581574919f01                 addps	0x19f9174(%rip), %xmm2  # 0x1422da1b0
// 0x1408e103c  f30f12db                       movsldup	%xmm3, %xmm3            # xmm3 = xmm3[0,0,2,2]
// 0x1408e1040  0f59da                         mulps	%xmm2, %xmm3
// 0x1408e1043  0f28c4                         movaps	%xmm4, %xmm0
// Frames: F119 -> F175 -> F147 -> F197
// 0x1408e1046  0f14c1                         unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
// 0x1408e1049  0f16c3                         movlhps	%xmm3, %xmm0            # xmm0 = xmm0[0],xmm3[0]
// Frames: F197
// 0x1408e104c  0f2945b0                       movaps	%xmm0, -0x50(%rbp)
// 0x1408e1050  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x1408e1054  e8877d9e00                     callq	0x1412c8de0 <.text+0x12c7de0>
// Frames: F200
// 0x1408e1059  4889c6                         movq	%rax, %rsi
// 0x1408e105c  488d8df4000000                 leaq	0xf4(%rbp), %rcx
// 0x1408e1063  e8e8cb3d00                     callq	0x140cbdc50 <.text+0xcbcc50>
// Frames: F201
// 0x1408e1068  4989c7                         movq	%rax, %r15
// 0x1408e106b  0f57c0                         xorps	%xmm0, %xmm0
// 0x1408e106e  e82d699000                     callq	0x1411e79a0 <.text+0x11e69a0>
// Frames: F202
// 0x1408e1073  440f58ce                       addps	%xmm6, %xmm9
// 0x1408e1077  f3450f58c6                     addss	%xmm14, %xmm8
// 0x1408e107c  440f138d80010000               movlps	%xmm9, 0x180(%rbp)
// 0x1408e1084  f3440f118588010000             movss	%xmm8, 0x188(%rbp)
// 0x1408e108d  c7858c01000000000000           movl	$0x0, 0x18c(%rbp)
// 0x1408e1097  48b90000003c0000ff7b           movabsq	$0x7bff00003c000000, %rcx # imm = 0x7BFF00003C000000
// 0x1408e10a1  48898d90010000                 movq	%rcx, 0x190(%rbp)
// 0x1408e10a8  4489bd98010000                 movl	%r15d, 0x198(%rbp)
// 0x1408e10af  49c1ef20                       shrq	$0x20, %r15
// 0x1408e10b3  664489bd9c010000               movw	%r15w, 0x19c(%rbp)
// 0x1408e10bb  6689859e010000                 movw	%ax, 0x19e(%rbp)
// 0x1408e10c2  4889b5a0010000                 movq	%rsi, 0x1a0(%rbp)
// 0x1408e10c9  8b8574010000                   movl	0x174(%rbp), %eax
// 0x1408e10cf  8985a8010000                   movl	%eax, 0x1a8(%rbp)
// 0x1408e10d5  48c785ac01000000000000         movq	$0x0, 0x1ac(%rbp)
// 0x1408e10e0  89bdb4010000                   movl	%edi, 0x1b4(%rbp)
// 0x1408e10e6  48b80100000002000000           movabsq	$0x200000001, %rax # imm = 0x200000001
// 0x1408e10f0  488985b8010000                 movq	%rax, 0x1b8(%rbp)
// 0x1408e10f7  488b8d00010000                 movq	0x100(%rbp), %rcx
// 0x1408e10fe  488d9580010000                 leaq	0x180(%rbp), %rdx
// 0x1408e1105  e816070000                     callq	0x1408e1820 <.text+0x8e0820>
// 0x1408e110a  e921f7ffff                     jmp	0x1408e0830 <.text+0x8df830>
// 0x1408e110f  90                             nop
// Frames: F203 -> F204
// 0x1408e1110  488b9518010000                 movq	0x118(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F204
// 0x1408e1117  4885d2                         testq	%rdx, %rdx
// 0x1408e111a  f3440f10259d768601             movss	0x186769d(%rip), %xmm12 # 0x1421487c0
// 0x1408e1123  7416                           je	0x1408e113b <.text+0x8e013b>
// Frames: F203 -> F204
// 0x1408e1125  488b8d20010000                 movq	0x120(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F204
// 0x1408e112c  48c1e202                       shlq	$0x2, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F204
// 0x1408e1130  41b804000000                   movl	$0x4, %r8d
// 0x1408e1136  e85596afff                     callq	0x1403da790 <.text+0x3d9790>
// 0x1408e113b  440fb6b5c7010000               movzbl	0x1c7(%rbp), %r14d
// 0x1408e1143  488b9d88000000                 movq	0x88(%rbp), %rbx
// 0x1408e114a  8b8d48010000                   movl	0x148(%rbp), %ecx
// 0x1408e1150  448bad4c010000                 movl	0x14c(%rbp), %r13d
// 0x1408e1157  e911f4ffff                     jmp	0x1408e056d <.text+0x8df56d>
// Frames: F203 -> F203 -> F212
// 0x1408e115c  488b95a8000000                 movq	0xa8(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e1163  4885d2                         testq	%rdx, %rdx
// 0x1408e1166  7416                           je	0x1408e117e <.text+0x8e017e>
// Frames: F203 -> F203 -> F212
// 0x1408e1168  488b8db0000000                 movq	0xb0(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e116f  48c1e203                       shlq	$0x3, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F203 -> F212
// 0x1408e1173  41b804000000                   movl	$0x4, %r8d
// 0x1408e1179  e81296afff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F203 -> F203 -> F212
// 0x1408e117e  488b95c0000000                 movq	0xc0(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e1185  4885d2                         testq	%rdx, %rdx
// 0x1408e1188  7416                           je	0x1408e11a0 <.text+0x8e01a0>
// Frames: F203 -> F203 -> F212
// 0x1408e118a  488b8dc8000000                 movq	0xc8(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e1191  48c1e202                       shlq	$0x2, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F203 -> F212
// 0x1408e1195  41b804000000                   movl	$0x4, %r8d
// 0x1408e119b  e8f095afff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F203 -> F203 -> F212
// 0x1408e11a0  488b5508                       movq	0x8(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e11a4  4885d2                         testq	%rdx, %rdx
// 0x1408e11a7  7413                           je	0x1408e11bc <.text+0x8e01bc>
// Frames: F203 -> F203 -> F212
// 0x1408e11a9  488b4d10                       movq	0x10(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e11ad  48c1e203                       shlq	$0x3, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F203 -> F212
// 0x1408e11b1  41b804000000                   movl	$0x4, %r8d
// 0x1408e11b7  e8d495afff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F203 -> F203 -> F212
// 0x1408e11bc  488b5520                       movq	0x20(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e11c0  4885d2                         testq	%rdx, %rdx
// 0x1408e11c3  7413                           je	0x1408e11d8 <.text+0x8e01d8>
// Frames: F203 -> F203 -> F212
// 0x1408e11c5  488b4d28                       movq	0x28(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e11c9  48c1e202                       shlq	$0x2, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F203 -> F212
// 0x1408e11cd  41b804000000                   movl	$0x4, %r8d
// 0x1408e11d3  e8b895afff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F212
// 0x1408e11d8  0f28b5d0010000                 movaps	0x1d0(%rbp), %xmm6
// 0x1408e11df  0f28bde0010000                 movaps	0x1e0(%rbp), %xmm7
// 0x1408e11e6  440f2885f0010000               movaps	0x1f0(%rbp), %xmm8
// 0x1408e11ee  440f288d00020000               movaps	0x200(%rbp), %xmm9
// 0x1408e11f6  440f289510020000               movaps	0x210(%rbp), %xmm10
// 0x1408e11fe  440f289d20020000               movaps	0x220(%rbp), %xmm11
// 0x1408e1206  440f28a530020000               movaps	0x230(%rbp), %xmm12
// 0x1408e120e  440f28ad40020000               movaps	0x240(%rbp), %xmm13
// 0x1408e1216  440f28b550020000               movaps	0x250(%rbp), %xmm14
// 0x1408e121e  440f28bd60020000               movaps	0x260(%rbp), %xmm15
// 0x1408e1226  4881c4f8020000                 addq	$0x2f8, %rsp            # imm = 0x2F8
// 0x1408e122d  5b                             popq	%rbx
// 0x1408e122e  5f                             popq	%rdi
// 0x1408e122f  5e                             popq	%rsi
// 0x1408e1230  415c                           popq	%r12
// 0x1408e1232  415d                           popq	%r13
// 0x1408e1234  415e                           popq	%r14
// 0x1408e1236  415f                           popq	%r15
// 0x1408e1238  5d                             popq	%rbp
// 0x1408e1239  c3                             retq
// Frames: F28 -> F29 -> F30 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e123a  4c8d05df949f01                 leaq	0x19f94df(%rip), %r8    # 0x1422da720
// 0x1408e1241  4c89f1                         movq	%r14, %rcx
// 0x1408e1244  4889da                         movq	%rbx, %rdx
// 0x1408e1247  e8c4d97c01                     callq	0x1420aec10 <.text+0x20adc10>
// 0x1408e124c  e9ca020000                     jmp	0x1408e151b <.text+0x8e051b>
// 0x1408e1251  4c8d05f8949f01                 leaq	0x19f94f8(%rip), %r8    # 0x1422da750
// 0x1408e1258  4c89f1                         movq	%r14, %rcx
// 0x1408e125b  4889da                         movq	%rbx, %rdx
// 0x1408e125e  e8add97c01                     callq	0x1420aec10 <.text+0x20adc10>
// 0x1408e1263  e9b3020000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F28 -> F29 -> F30 -> F94 -> F92
// 0x1408e1268  4c8d05c1f5cc01                 leaq	0x1ccf5c1(%rip), %r8    # 0x1425b0830
// 0x1408e126f  488b9558010000                 movq	0x158(%rbp), %rdx
// 0x1408e1276  e895d97c01                     callq	0x1420aec10 <.text+0x20adc10>
// 0x1408e127b  e99b020000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F25
// 0x1408e1280  4c8d0579f5cc01                 leaq	0x1ccf579(%rip), %r8    # 0x1425b0800
// 0x1408e1287  e884d97c01                     callq	0x1420aec10 <.text+0x20adc10>
// 0x1408e128c  e98a020000                     jmp	0x1408e151b <.text+0x8e051b>
// 0x1408e1291  4c8d0580f5cc01                 leaq	0x1ccf580(%rip), %r8    # 0x1425b0818
// 0x1408e1298  4889c1                         movq	%rax, %rcx
// 0x1408e129b  e870d97c01                     callq	0x1420aec10 <.text+0x20adc10>
// 0x1408e12a0  e976020000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F28 -> F29 -> F30 -> F27 -> F24 -> F32
// 0x1408e12a5  4c8d0554f5cc01                 leaq	0x1ccf554(%rip), %r8    # 0x1425b0800
// 0x1408e12ac  e85fd97c01                     callq	0x1420aec10 <.text+0x20adc10>
// 0x1408e12b1  e965020000                     jmp	0x1408e151b <.text+0x8e051b>
// 0x1408e12b6  4c8d055bf5cc01                 leaq	0x1ccf55b(%rip), %r8    # 0x1425b0818
// 0x1408e12bd  4889c1                         movq	%rax, %rcx
// 0x1408e12c0  488b9558010000                 movq	0x158(%rbp), %rdx
// 0x1408e12c7  e844d97c01                     callq	0x1420aec10 <.text+0x20adc10>
// 0x1408e12cc  e94a020000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F23 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e12d1  488d0d98859f01                 leaq	0x19f8598(%rip), %rcx   # 0x1422d9870
// 0x1408e12d8  4c8d0559949f01                 leaq	0x19f9459(%rip), %r8    # 0x1422da738
// 0x1408e12df  ba1d000000                     movl	$0x1d, %edx
// 0x1408e12e4  e867d87c01                     callq	0x1420aeb50 <.text+0x20adb50>
// 0x1408e12e9  e92d020000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F213 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e12ee  488d8578010000                 leaq	0x178(%rbp), %rax
// 0x1408e12f5  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x1408e12fc  488d055dacf9ff                 leaq	-0x653a3(%rip), %rax    # 0x14087bf60 <.text+0x87af60>
// 0x1408e1303  48898568010000                 movq	%rax, 0x168(%rbp)
// Frames: F214 -> F213 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e130a  488d054f859f01                 leaq	0x19f854f(%rip), %rax   # 0x1422d9860
// 0x1408e1311  48898580010000                 movq	%rax, 0x180(%rbp)
// 0x1408e1318  48c7858801000001000000         movq	$0x1, 0x188(%rbp)
// 0x1408e1323  48c785a001000000000000         movq	$0x0, 0x1a0(%rbp)
// 0x1408e132e  488d8560010000                 leaq	0x160(%rbp), %rax
// 0x1408e1335  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x1408e133c  48c7859801000001000000         movq	$0x1, 0x198(%rbp)
// Frames: F213 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e1347  488d15ea939f01                 leaq	0x19f93ea(%rip), %rdx   # 0x1422da738
// 0x1408e134e  488d8d80010000                 leaq	0x180(%rbp), %rcx
// 0x1408e1355  e826d77c01                     callq	0x1420aea80 <.text+0x20ada80>
// 0x1408e135a  e9bc010000                     jmp	0x1408e151b <.text+0x8e051b>
// 0x1408e135f  488d8578010000                 leaq	0x178(%rbp), %rax
// 0x1408e1366  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x1408e136d  488d05ecabf9ff                 leaq	-0x65414(%rip), %rax    # 0x14087bf60 <.text+0x87af60>
// 0x1408e1374  48898568010000                 movq	%rax, 0x168(%rbp)
// Frames: F214 -> F213 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e137b  488d05de849f01                 leaq	0x19f84de(%rip), %rax   # 0x1422d9860
// 0x1408e1382  48898580010000                 movq	%rax, 0x180(%rbp)
// 0x1408e1389  48c7858801000001000000         movq	$0x1, 0x188(%rbp)
// 0x1408e1394  48c785a001000000000000         movq	$0x0, 0x1a0(%rbp)
// 0x1408e139f  488d8560010000                 leaq	0x160(%rbp), %rax
// 0x1408e13a6  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x1408e13ad  48c7859801000001000000         movq	$0x1, 0x198(%rbp)
// Frames: F213 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e13b8  488d15a9939f01                 leaq	0x19f93a9(%rip), %rdx   # 0x1422da768
// 0x1408e13bf  488d8d80010000                 leaq	0x180(%rbp), %rcx
// 0x1408e13c6  e8b5d67c01                     callq	0x1420aea80 <.text+0x20ada80>
// 0x1408e13cb  e94b010000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F23 -> F69 -> F70 -> F71 -> F67 -> F68
// 0x1408e13d0  488d0d99849f01                 leaq	0x19f8499(%rip), %rcx   # 0x1422d9870
// 0x1408e13d7  4c8d058a939f01                 leaq	0x19f938a(%rip), %r8    # 0x1422da768
// 0x1408e13de  ba1d000000                     movl	$0x1d, %edx
// 0x1408e13e3  e868d77c01                     callq	0x1420aeb50 <.text+0x20adb50>
// 0x1408e13e8  e92e010000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F23 -> F109
// 0x1408e13ed  488d0d7c849f01                 leaq	0x19f847c(%rip), %rcx   # 0x1422d9870
// 0x1408e13f4  4c8d050d939f01                 leaq	0x19f930d(%rip), %r8    # 0x1422da708
// 0x1408e13fb  ba1d000000                     movl	$0x1d, %edx
// 0x1408e1400  e84bd77c01                     callq	0x1420aeb50 <.text+0x20adb50>
// 0x1408e1405  e911010000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F213 -> F109
// 0x1408e140a  488d8534010000                 leaq	0x134(%rbp), %rax
// 0x1408e1411  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x1408e1418  488d0541abf9ff                 leaq	-0x654bf(%rip), %rax    # 0x14087bf60 <.text+0x87af60>
// 0x1408e141f  48898568010000                 movq	%rax, 0x168(%rbp)
// Frames: F214 -> F213 -> F109
// 0x1408e1426  488d0533849f01                 leaq	0x19f8433(%rip), %rax   # 0x1422d9860
// 0x1408e142d  48898580010000                 movq	%rax, 0x180(%rbp)
// 0x1408e1434  48c7858801000001000000         movq	$0x1, 0x188(%rbp)
// 0x1408e143f  48c785a001000000000000         movq	$0x0, 0x1a0(%rbp)
// 0x1408e144a  488d8560010000                 leaq	0x160(%rbp), %rax
// 0x1408e1451  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x1408e1458  48c7859801000001000000         movq	$0x1, 0x198(%rbp)
// Frames: F213 -> F109
// 0x1408e1463  488d159e929f01                 leaq	0x19f929e(%rip), %rdx   # 0x1422da708
// 0x1408e146a  488d8d80010000                 leaq	0x180(%rbp), %rcx
// 0x1408e1471  e80ad67c01                     callq	0x1420aea80 <.text+0x20ada80>
// 0x1408e1476  e9a0000000                     jmp	0x1408e151b <.text+0x8e051b>
// Frames: F213 -> F16
// 0x1408e147b  488d8560010000                 leaq	0x160(%rbp), %rax
// 0x1408e1482  48898518010000                 movq	%rax, 0x118(%rbp)
// 0x1408e1489  488d05d0aaf9ff                 leaq	-0x65530(%rip), %rax    # 0x14087bf60 <.text+0x87af60>
// 0x1408e1490  48898520010000                 movq	%rax, 0x120(%rbp)
// Frames: F214 -> F213 -> F16
// 0x1408e1497  488d05c2839f01                 leaq	0x19f83c2(%rip), %rax   # 0x1422d9860
// 0x1408e149e  48898580010000                 movq	%rax, 0x180(%rbp)
// 0x1408e14a5  48c7858801000001000000         movq	$0x1, 0x188(%rbp)
// 0x1408e14b0  48c785a001000000000000         movq	$0x0, 0x1a0(%rbp)
// 0x1408e14bb  488d8518010000                 leaq	0x118(%rbp), %rax
// 0x1408e14c2  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x1408e14c9  48c7859801000001000000         movq	$0x1, 0x198(%rbp)
// Frames: F213 -> F16
// 0x1408e14d4  488d15bd919f01                 leaq	0x19f91bd(%rip), %rdx   # 0x1422da698
// 0x1408e14db  488d8d80010000                 leaq	0x180(%rbp), %rcx
// 0x1408e14e2  e899d57c01                     callq	0x1420aea80 <.text+0x20ada80>
// 0x1408e14e7  eb32                           jmp	0x1408e151b <.text+0x8e051b>
// Frames: F23 -> F16
// 0x1408e14e9  488d0d80839f01                 leaq	0x19f8380(%rip), %rcx   # 0x1422d9870
// 0x1408e14f0  4c8d05a1919f01                 leaq	0x19f91a1(%rip), %r8    # 0x1422da698
// 0x1408e14f7  ba1d000000                     movl	$0x1d, %edx
// 0x1408e14fc  e84fd67c01                     callq	0x1420aeb50 <.text+0x20adb50>
// 0x1408e1501  eb18                           jmp	0x1408e151b <.text+0x8e051b>
// Frames: F63
// 0x1408e1503  488d0dbe919f01                 leaq	0x19f91be(%rip), %rcx   # 0x1422da6c8
// 0x1408e150a  4c8d05df919f01                 leaq	0x19f91df(%rip), %r8    # 0x1422da6f0
// 0x1408e1511  ba23000000                     movl	$0x23, %edx
// 0x1408e1516  e835d67c01                     callq	0x1420aeb50 <.text+0x20adb50>
// 0x1408e151b  0f0b                           ud2
// 0x1408e151d  0f1f00                         nopl	(%rax)
// 0x1408e1520  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e1525  55                             pushq	%rbp
// 0x1408e1526  4157                           pushq	%r15
// 0x1408e1528  4156                           pushq	%r14
// 0x1408e152a  4155                           pushq	%r13
// 0x1408e152c  4154                           pushq	%r12
// 0x1408e152e  56                             pushq	%rsi
// 0x1408e152f  57                             pushq	%rdi
// 0x1408e1530  53                             pushq	%rbx
// 0x1408e1531  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1408e1538  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1408e153f  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x1408e1545  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x1408e154b  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x1408e1551  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x1408e1557  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x1408e155d  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x1408e1563  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x1408e156c  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x1408e1575  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x1408e157d  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F203 -> F203 -> F212
// 0x1408e1585  488b5508                       movq	0x8(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e1589  4885d2                         testq	%rdx, %rdx
// 0x1408e158c  7413                           je	0x1408e15a1 <.text+0x8e05a1>
// Frames: F203 -> F203 -> F212
// 0x1408e158e  488b4d10                       movq	0x10(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e1592  48c1e203                       shlq	$0x3, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F203 -> F212
// 0x1408e1596  41b804000000                   movl	$0x4, %r8d
// 0x1408e159c  e8ef91afff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F203 -> F203 -> F212
// 0x1408e15a1  488b5520                       movq	0x20(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e15a5  4885d2                         testq	%rdx, %rdx
// 0x1408e15a8  7413                           je	0x1408e15bd <.text+0x8e05bd>
// Frames: F203 -> F203 -> F212
// 0x1408e15aa  488b4d28                       movq	0x28(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e15ae  48c1e202                       shlq	$0x2, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F203 -> F212
// 0x1408e15b2  41b804000000                   movl	$0x4, %r8d
// 0x1408e15b8  e8d391afff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F212
// 0x1408e15bd  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x1408e15c5  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x1408e15cd  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x1408e15d6  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1408e15df  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x1408e15e5  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x1408e15eb  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x1408e15f1  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x1408e15f7  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x1408e15fd  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x1408e1603  4881c4c8000000                 addq	$0xc8, %rsp
// 0x1408e160a  5b                             popq	%rbx
// 0x1408e160b  5f                             popq	%rdi
// 0x1408e160c  5e                             popq	%rsi
// 0x1408e160d  415c                           popq	%r12
// 0x1408e160f  415d                           popq	%r13
// 0x1408e1611  415e                           popq	%r14
// 0x1408e1613  415f                           popq	%r15
// 0x1408e1615  5d                             popq	%rbp
// 0x1408e1616  c3                             retq
// 0x1408e1617  660f1f840000000000             nopw	(%rax,%rax)
// 0x1408e1620  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e1625  55                             pushq	%rbp
// 0x1408e1626  4157                           pushq	%r15
// 0x1408e1628  4156                           pushq	%r14
// 0x1408e162a  4155                           pushq	%r13
// 0x1408e162c  4154                           pushq	%r12
// 0x1408e162e  56                             pushq	%rsi
// 0x1408e162f  57                             pushq	%rdi
// 0x1408e1630  53                             pushq	%rbx
// 0x1408e1631  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1408e1638  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1408e163f  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x1408e1645  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x1408e164b  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x1408e1651  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x1408e1657  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x1408e165d  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x1408e1663  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x1408e166c  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x1408e1675  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x1408e167d  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F203 -> F203 -> F212
// 0x1408e1685  488b95a8000000                 movq	0xa8(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e168c  4885d2                         testq	%rdx, %rdx
// 0x1408e168f  7416                           je	0x1408e16a7 <.text+0x8e06a7>
// Frames: F203 -> F203 -> F212
// 0x1408e1691  488b8db0000000                 movq	0xb0(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e1698  48c1e203                       shlq	$0x3, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F203 -> F212
// 0x1408e169c  41b804000000                   movl	$0x4, %r8d
// 0x1408e16a2  e8e990afff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F203 -> F203 -> F212
// 0x1408e16a7  488b95c0000000                 movq	0xc0(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e16ae  4885d2                         testq	%rdx, %rdx
// 0x1408e16b1  7416                           je	0x1408e16c9 <.text+0x8e06c9>
// Frames: F203 -> F203 -> F212
// 0x1408e16b3  488b8dc8000000                 movq	0xc8(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F203 -> F212
// 0x1408e16ba  48c1e202                       shlq	$0x2, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F203 -> F212
// 0x1408e16be  41b804000000                   movl	$0x4, %r8d
// 0x1408e16c4  e8c790afff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F212
// 0x1408e16c9  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x1408e16d1  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x1408e16d9  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x1408e16e2  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1408e16eb  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x1408e16f1  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x1408e16f7  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x1408e16fd  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x1408e1703  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x1408e1709  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x1408e170f  4881c4c8000000                 addq	$0xc8, %rsp
// 0x1408e1716  5b                             popq	%rbx
// 0x1408e1717  5f                             popq	%rdi
// 0x1408e1718  5e                             popq	%rsi
// 0x1408e1719  415c                           popq	%r12
// 0x1408e171b  415d                           popq	%r13
// 0x1408e171d  415e                           popq	%r14
// 0x1408e171f  415f                           popq	%r15
// 0x1408e1721  5d                             popq	%rbp
// 0x1408e1722  c3                             retq
// 0x1408e1723  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1408e1730  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e1735  55                             pushq	%rbp
// 0x1408e1736  4157                           pushq	%r15
// 0x1408e1738  4156                           pushq	%r14
// 0x1408e173a  4155                           pushq	%r13
// 0x1408e173c  4154                           pushq	%r12
// 0x1408e173e  56                             pushq	%rsi
// 0x1408e173f  57                             pushq	%rdi
// 0x1408e1740  53                             pushq	%rbx
// 0x1408e1741  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1408e1748  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1408e174f  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x1408e1755  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x1408e175b  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x1408e1761  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x1408e1767  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x1408e176d  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x1408e1773  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x1408e177c  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x1408e1785  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x1408e178d  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F203 -> F204
// 0x1408e1795  488b9518010000                 movq	0x118(%rbp), %rdx
// Frames: F205 -> F206 -> F203 -> F203 -> F204
// 0x1408e179c  4885d2                         testq	%rdx, %rdx
// 0x1408e179f  7416                           je	0x1408e17b7 <.text+0x8e07b7>
// Frames: F203 -> F204
// 0x1408e17a1  488b8d20010000                 movq	0x120(%rbp), %rcx
// Frames: F207 -> F208 -> F206 -> F203 -> F203 -> F204
// 0x1408e17a8  48c1e202                       shlq	$0x2, %rdx
// Frames: F209 -> F210 -> F211 -> F203 -> F203 -> F204
// 0x1408e17ac  41b804000000                   movl	$0x4, %r8d
// 0x1408e17b2  e8d98fafff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F204
// 0x1408e17b7  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x1408e17bf  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x1408e17c7  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x1408e17d0  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1408e17d9  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x1408e17df  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x1408e17e5  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x1408e17eb  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x1408e17f1  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x1408e17f7  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x1408e17fd  4881c4c8000000                 addq	$0xc8, %rsp
// 0x1408e1804  5b                             popq	%rbx
// 0x1408e1805  5f                             popq	%rdi
// 0x1408e1806  5e                             popq	%rsi
// 0x1408e1807  415c                           popq	%r12
// 0x1408e1809  415d                           popq	%r13
// 0x1408e180b  415e                           popq	%r14
// 0x1408e180d  415f                           popq	%r15
// 0x1408e180f  5d                             popq	%rbp
// 0x1408e1810  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/utils/assemble_circular_roof.rs:26:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F2: crates/systems/roof/src/utils/assemble_circular_roof.rs:32:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F3: crates/systems/roof/src/utils/assemble_circular_roof.rs:35:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F4: crates/systems/roof/src/utils/assemble_circular_roof.rs:40:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F5: crates/systems/roof/src/utils/assemble_circular_roof.rs:42:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F6: crates/utils/src/curve/mod.rs:146:0 utils::curve::Curve::length_ws
// F7: crates/systems/roof/src/utils/assemble_circular_roof.rs:47:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F8: crates/systems/roof/src/utils/assemble_circular_roof.rs:48:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:74:0 std::f32::impl$0::ceil
// F10: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:925:0 core::f32::impl$0::max
// F11: crates/systems/roof/src/utils/assemble_circular_roof.rs:56:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/range.rs:754:0 core::iter::range::impl$5::spec_next
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/range.rs:845:0 core::iter::range::impl$6::next
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1553:0 core::cmp::impls::impl$72::lt
// F15: crates/systems/roof/src/utils/assemble_circular_roof.rs:58:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F16: crates/systems/roof/src/utils/assemble_circular_roof.rs:62:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F17: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1517:0 core::cmp::impls::impl$50::le
// F18: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1677:0 core::cmp::impls::impl$10::le
// F19: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/range.rs:823:0 core::ops::range::RangeBounds::contains
// F20: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/range.rs:514:0 core::ops::range::RangeInclusive::contains
// F21: crates/utils/src/curve/curve_u.rs:89:0 utils::curve::curve_u::CurveU::new
// F22: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:503:0 core::f32::impl$0::is_nan
// F23: crates/utils/src/curve/curve_u.rs:93:0 utils::curve::curve_u::CurveU::new
// F24: crates/utils/src/curve/mod.rs:563:0 utils::curve::Curve::get_pos_at_u
// F25: crates/systems/roof/src/utils/assemble_circular_roof.rs:64:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F26: crates/utils/src/curve/mod.rs:1109:0 utils::curve::CurveInternalCoord::i0
// F27: crates/utils/src/curve/mod.rs:623:0 utils::curve::Curve::get_pos_at_coord
// F28: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:264:0 core::slice::index::impl$2::index
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:18:0 core::slice::index::impl$0::index
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2829:0 alloc::vec::impl$12::index
// F31: crates/utils/src/curve/mod.rs:1114:0 utils::curve::CurveInternalCoord::i1
// F32: crates/systems/roof/src/utils/assemble_circular_roof.rs:65:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F33: crates/systems/roof/src/utils/assemble_circular_roof.rs:67:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F34: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:221:0 core::ops::arith::impl$68::sub
// F35: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:1054:0 glam::f32::vec2::impl$17::sub
// F36: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:676:0 glam::f32::vec2::Vec2::lerp
// F37: crates/utils/src/lib.rs:76:0 utils::impl$0::lerp
// F38: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:351:0 core::ops::arith::impl$124::mul
// F39: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:975:0 glam::f32::vec2::impl$17::sub
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:112:0 core::ops::arith::impl$12::add
// F41: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:1005:0 glam::f32::vec2::impl$17::sub
// F42: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:974:0 glam::f32::vec2::impl$9::mul
// F43: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:1004:0 glam::f32::vec2::impl$12::add
// F44: crates/systems/roof/src/utils/assemble_circular_roof.rs:73:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F45: crates/systems/roof/src/utils/assemble_circular_roof.rs:75:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1521:0 core::f32::impl$0::clamp
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1524:0 core::f32::impl$0::clamp
// F48: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:344:0 core::ops::arith::impl$124::mul
// F49: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F50: crates/systems/roof/src/utils/assemble_circular_roof.rs:76:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F51: crates/systems/roof/src/utils/assemble_circular_roof.rs:79:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F52: crates/systems/roof/src/utils/assemble_circular_roof.rs:89:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F53: crates/systems/roof/src/utils/assemble_circular_roof.rs:77:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F54: crates/systems/roof/src/utils/assemble_circular_roof.rs:80:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F55: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:714:0 std::f32::impl$0::sin
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:916:0 std::f32::impl$0::sin_cos
// F57: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs:178:0 glam::f32::math::std_math::sin_cos
// F58: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs:125:0 glam::f32::sse2::mat2::Mat2::from_angle
// F59: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:737:0 std::f32::impl$0::sin
// F60: crates/systems/roof/src/utils/assemble_circular_roof.rs:91:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F61: crates/systems/roof/src/utils/assemble_circular_roof.rs:92:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F62: crates/systems/roof/src/utils/assemble_circular_roof.rs:90:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F63: crates/systems/roof/src/utils/assemble_circular_roof.rs:96:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F64: crates/systems/roof/src/utils/assemble_circular_roof.rs:99:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F65: crates/systems/roof/src/utils/assemble_circular_roof.rs:210:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F66: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1553:0 core::cmp::impls::impl$54::lt
// F67: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/map.rs:108:0 core::iter::adapters::map::impl$2::next
// F68: crates/systems/roof/src/utils/assemble_circular_roof.rs:105:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F69: crates/systems/roof/src/utils/assemble_circular_roof.rs:99:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// F70: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:305:0 core::ops::function::impls::impl$4::call_once
// F71: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F72: crates/systems/roof/src/utils/assemble_circular_roof.rs:106:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F73: crates/systems/roof/src/utils/assemble_circular_roof.rs:107:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:944:0 core::core_arch::x86::sse::_mm_set_ps
// F75: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs:278:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F76: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:78:0 core::core_arch::x86::sse::_mm_mul_ps
// F77: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs:279:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F78: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1013:0 core::core_arch::x86::sse::_mm_shuffle_ps
// F79: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs:280:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F80: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:32:0 core::core_arch::x86::sse::_mm_add_ps
// F81: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs:281:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F82: crates/systems/roof/src/utils/assemble_circular_roof.rs:109:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F83: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:509:0 core::ops::arith::impl$216::div
// F84: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:925:0 glam::f32::vec2::impl$4::div
// F85: crates/systems/roof/src/utils/assemble_circular_roof.rs:110:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F86: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1083:0 glam::f32::vec3::impl$12::add
// F87: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1085:0 glam::f32::vec3::impl$12::add
// F88: crates/systems/roof/src/utils/assemble_circular_roof.rs:118:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F89: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$68::sub
// F90: crates/utils/src/curve/mod.rs:520:0 utils::curve::Curve::get_curve_segment_from_u
// F91: crates/utils/src/curve/mod.rs:616:0 utils::curve::Curve::get_tangent_at_u
// F92: crates/systems/roof/src/utils/assemble_circular_roof.rs:121:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F93: crates/utils/src/curve/mod.rs:521:0 utils::curve::Curve::get_curve_segment_from_u
// F94: crates/utils/src/curve/mod.rs:617:0 utils::curve::Curve::get_tangent_at_u
// F95: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:1053:0 glam::f32::vec2::impl$17::sub
// F96: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:158:0 glam::f32::vec2::Vec2::dot
// F97: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:381:0 glam::f32::vec2::Vec2::length
// F98: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:400:0 glam::f32::vec2::Vec2::length_recip
// F99: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:452:0 glam::f32::vec2::Vec2::normalize
// F100: crates/utils/src/lib.rs:66:0 utils::impl$0::normalize
// F101: crates/utils/src/curve/mod.rs:619:0 utils::curve::Curve::get_tangent_at_u
// F102: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:442:0 std::f32::impl$0::sqrt
// F103: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs:188:0 glam::f32::vec2::Vec2::dot
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:868:0 glam::f32::vec2::Vec2::length
// F105: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:974:0 glam::f32::vec2::Vec2::length_recip
// F106: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:975:0 glam::f32::vec2::Vec2::length_recip
// F107: crates/systems/roof/src/utils/assemble_circular_roof.rs:122:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F108: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:847:0 std::f32::impl$0::atan
// F109: crates/systems/roof/src/utils/assemble_circular_roof.rs:125:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F110: crates/systems/roof/src/utils/assemble_circular_roof.rs:127:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F111: crates/systems/roof/src/utils/assemble_circular_roof.rs:128:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F112: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:212:0 glam::f32::vec3::Vec3::cross
// F113: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:139:0 glam::f32::sse2::quat::Quat::from_axis_angle
// F114: crates/systems/roof/src/utils/assemble_circular_roof.rs:131:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F115: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:211:0 glam::f32::vec3::Vec3::cross
// F116: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1051:0 glam::f32::vec3::impl$9::mul
// F117: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:140:0 glam::f32::sse2::quat::Quat::from_axis_angle
// F118: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1050:0 glam::f32::vec3::impl$9::mul
// F119: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:73:0 glam::f32::sse2::quat::Quat::from_xyzw
// F120: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:141:0 glam::f32::sse2::quat::Quat::from_axis_angle
// F121: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs:91:0 glam::f32::sse2::vec3a::Vec3A::new
// F122: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs:1359:0 glam::f32::sse2::vec3a::impl$45::from
// F123: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/convert/mod.rs:759:0 core::convert::impl$3::into
// F124: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:662:0 glam::f32::sse2::quat::Quat::mul_vec3
// F125: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:876:0 glam::f32::sse2::quat::impl$9::mul
// F126: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs:43:0 glam::sse2::dot3_in_x
// F127: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs:67:0 glam::sse2::dot3_into_m128
// F128: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:736:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F129: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs:68:0 glam::sse2::dot3_into_m128
// F130: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:739:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F131: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:55:0 core::core_arch::x86::sse::_mm_sub_ps
// F132: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs:227:0 glam::f32::sse2::vec3a::Vec3A::cross
// F133: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:742:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F134: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs:228:0 glam::f32::sse2::vec3a::Vec3A::cross
// F135: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs:229:0 glam::f32::sse2::vec3a::Vec3A::cross
// F136: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:740:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F137: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:21:0 core::core_arch::x86::sse::_mm_add_ss
// F138: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs:47:0 glam::sse2::dot3_in_x
// F139: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:738:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F140: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs:230:0 glam::f32::sse2::vec3a::Vec3A::cross
// F141: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs:231:0 glam::f32::sse2::vec3a::Vec3A::cross
// F142: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs:232:0 glam::f32::sse2::vec3a::Vec3A::cross
// F143: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:737:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F144: crates/systems/roof/src/utils/assemble_circular_roof.rs:132:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F145: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:213:0 glam::f32::vec3::Vec3::cross
// F146: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:197:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F147: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:254:0 glam::f32::sse2::quat::Quat::from_mat3
// F148: crates/systems/roof/src/utils/assemble_circular_roof.rs:140:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F149: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:224:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F150: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:225:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F151: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:226:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F152: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:238:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F153: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs:188:0 glam::f32::math::std_math::sqrt
// F154: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:239:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F155: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:241:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F156: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:243:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F157: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:199:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F158: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:200:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F159: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:201:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F160: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:216:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F161: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:214:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F162: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:218:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F163: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:219:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F164: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:228:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F165: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:229:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F166: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:231:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F167: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:233:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F168: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:234:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F169: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:203:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F170: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:204:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F171: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:207:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F172: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:206:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F173: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:208:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F174: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:209:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F175: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:0:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F176: crates/systems/roof/src/utils/assemble_circular_roof.rs:142:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F177: crates/systems/roof/src/utils/assemble_circular_roof.rs:144:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F178: crates/systems/roof/src/utils/assemble_circular_roof.rs:138:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F179: crates/systems/roof/src/utils/assemble_circular_roof.rs:167:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F180: crates/systems/roof/src/utils/assemble_circular_roof.rs:168:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F181: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1052:0 glam::f32::vec3::impl$9::mul
// F182: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1137:0 glam::f32::vec3::impl$17::sub
// F183: crates/systems/roof/src/utils/assemble_circular_roof.rs:169:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F184: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1139:0 glam::f32::vec3::impl$17::sub
// F185: crates/systems/roof/src/utils/assemble_circular_roof.rs:173:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F186: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:463:0 glam::f32::vec3::Vec3::distance
// F187: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:196:0 glam::f32::vec3::Vec3::dot
// F188: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:437:0 glam::f32::vec3::impl$17::sub
// F189: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs:188:0 glam::f32::vec3::Vec3::dot
// F190: crates/systems/roof/src/utils/assemble_circular_roof.rs:175:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F191: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:77:0 glam::f32::vec3::Vec3::new
// F192: crates/systems/roof/src/utils/assemble_circular_roof.rs:179:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F193: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:743:0 glam::f32::vec3::Vec3::lerp
// F194: crates/systems/roof/src/utils/assemble_circular_roof.rs:193:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F195: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1050:0 glam::f32::vec3::impl$17::sub
// F196: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1052:0 glam::f32::vec3::impl$17::sub
// F197: crates/systems/roof/src/utils/assemble_circular_roof.rs:194:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F198: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:213:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F199: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs:217:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F200: crates/systems/roof/src/utils/assemble_circular_roof.rs:196:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F201: crates/systems/roof/src/utils/assemble_circular_roof.rs:197:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F202: crates/systems/roof/src/utils/assemble_circular_roof.rs:192:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F203: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:515:0 core::ptr::drop_in_place
// F204: crates/systems/roof/src/utils/assemble_circular_roof.rs:212:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F205: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:280:0 alloc::raw_vec::RawVec::current_memory
// F206: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:554:0 alloc::raw_vec::impl$4::drop
// F207: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::unchecked_mul
// F208: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:290:0 alloc::raw_vec::RawVec::current_memory
// F209: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:119:0 alloc::alloc::dealloc
// F210: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:256:0 alloc::alloc::impl$1::deallocate
// F211: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:555:0 alloc::raw_vec::impl$4::drop
// F212: crates/systems/roof/src/utils/assemble_circular_roof.rs:213:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::h16108a54f77bfabc
// F213: crates/utils/src/curve/curve_u.rs:88:0 utils::curve::curve_u::CurveU::new
// F214: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:343:0 core::fmt::Arguments::new_v1
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/utils/assemble_circular_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e6253919730aa04373a6844f6965e00290a5183ff3aeff5c07a313a307e92ca2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: assemble_circular_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 754
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":754,"record_offset":2364,"rva":9307136,"va":5378016256}
// 编译实例 RVAs: 0x8e0400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5269931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e85596afff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378019638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1077092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e89c90efff","size":5,"soft_fail":false,"target_va":5376939904,"va":5378016991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1073631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8219eefff","size":5,"soft_fail":false,"target_va":5376943968,"va":5378017594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1072935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8d9a0efff","size":5,"soft_fail":false,"target_va":5376944000,"va":5378016930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbdc50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4050569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e889ce3d00","size":5,"soft_fail":false,"target_va":5382069328,"va":5378018754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e79a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9464738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8a26b9000","size":5,"soft_fail":false,"target_va":5387483552,"va":5378018809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c8de0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10387559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e867809e00","size":5,"soft_fail":false,"target_va":5388406240,"va":5378018676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c8e20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10389567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e83f889e00","size":5,"soft_fail":false,"target_va":5388406304,"va":5378016732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25576122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8ba428601","size":5,"soft_fail":false,"target_va":5403593920,"va":5378017793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25577468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8fc478601","size":5,"soft_fail":false,"target_va":5403593968,"va":5378016495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25576837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e885458601","size":5,"soft_fail":false,"target_va":5403594000,"va":5378017158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25577186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8e2468601","size":5,"soft_fail":false,"target_va":5403594336,"va":5378017145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t24959079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e867d87c01","size":5,"soft_fail":false,"target_va":5402979152,"va":5378020068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t24959428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8c4d97c01","size":5,"soft_fail":false,"target_va":5402979344,"va":5378019911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t24958758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e826d77c01","size":5,"soft_fail":false,"target_va":5402978944,"va":5378020181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2252944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8709fddff","size":5,"soft_fail":false,"target_va":5375763504,"va":5378016443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2252744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e838a0ddff","size":5,"soft_fail":false,"target_va":5375763696,"va":5378016435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2252315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8e5a1ddff","size":5,"soft_fail":false,"target_va":5375764560,"va":5378016870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::profile_curve_ws
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2250316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8b4a9ddff","size":5,"soft_fail":false,"target_va":5375766160,"va":5378016471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_circle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2285958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e87a1eddff","size":5,"soft_fail":false,"target_va":5375730464,"va":5378016417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2286270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8421dddff","size":5,"soft_fail":false,"target_va":5375730144,"va":5378016409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7832980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e894857700","size":5,"soft_fail":false,"target_va":5385850032,"va":5378017047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e88c090000","size":5,"soft_fail":false,"target_va":5378021408,"va":5378018959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-49491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e8ad3effff","size":5,"soft_fail":false,"target_va":5377968384,"va":5378017870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-49569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e85f3effff","size":5,"soft_fail":false,"target_va":5377967920,"va":5378017484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-49115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e82540ffff","size":5,"soft_fail":false,"target_va":5377967856,"va":5378016966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-48572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e84442ffff","size":5,"soft_fail":false,"target_va":5377968464,"va":5378017031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1077602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378016256,"raw_hex":"e89e8eefff","size":5,"soft_fail":false,"target_va":5376939616,"va":5378017213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":4004,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":2752,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":754,"record_offset":4264,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":754,"record_offset":2780,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":9080,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":754,"record_offset":4148,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":23090,"source_line":124,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":754,"record_offset":5128,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":6516,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14670,"source_line":137,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":7680,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":6760,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14675,"source_line":875,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":3948,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5332,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":3892,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":6480,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":8424,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":8672,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":8644,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":5388,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":8336,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":8104,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":754,"record_offset":4036,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":754,"record_offset":6096,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":45968,"source_line":846,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":754,"record_offset":2724,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":754,"record_offset":3076,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":754,"record_offset":5560,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":754,"record_offset":2696,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":754,"record_offset":2864,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":2364,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":754,"record_offset":9232,"parent_record_offset":9132,"ancestor_record_offset":2364,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":754,"record_offset":9164,"parent_record_offset":9132,"ancestor_record_offset":2364,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":754,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":2364,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":3164,"parent_record_offset":3104,"ancestor_record_offset":2364,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":754,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":2364,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":754,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":2364,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":754,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":2364,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":754,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2364,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":754,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":2364,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":5244,"parent_record_offset":5128,"ancestor_record_offset":2364,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":7432,"parent_record_offset":7376,"ancestor_record_offset":2364,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":5188,"parent_record_offset":5128,"ancestor_record_offset":2364,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":5160,"parent_record_offset":5128,"ancestor_record_offset":2364,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":5216,"parent_record_offset":5128,"ancestor_record_offset":2364,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":754,"record_offset":7080,"parent_record_offset":6892,"ancestor_record_offset":2364,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":3016,"parent_record_offset":2864,"ancestor_record_offset":2364,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":754,"record_offset":3044,"parent_record_offset":2864,"ancestor_record_offset":2364,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":754,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":2364,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":754,"record_offset":4300,"parent_record_offset":4264,"ancestor_record_offset":2364,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":754,"record_offset":9196,"parent_record_offset":9164,"ancestor_record_offset":2364,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":3816,"parent_record_offset":3756,"ancestor_record_offset":2364,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":3844,"parent_record_offset":3756,"ancestor_record_offset":2364,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":5356,"parent_record_offset":5332,"ancestor_record_offset":2364,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":754,"record_offset":3788,"parent_record_offset":3756,"ancestor_record_offset":2364,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":754,"record_offset":4424,"parent_record_offset":4384,"ancestor_record_offset":2364,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":754,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":2364,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":754,"record_offset":2904,"parent_record_offset":2864,"ancestor_record_offset":2364,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":754,"record_offset":4384,"parent_record_offset":4264,"ancestor_record_offset":2364,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":754,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":2364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":754,"record_offset":3192,"parent_record_offset":3164,"ancestor_record_offset":2364,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":754,"record_offset":3220,"parent_record_offset":3192,"ancestor_record_offset":2364,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":754,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":2364,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":754,"record_offset":7812,"parent_record_offset":7704,"ancestor_record_offset":2364,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":7704,"parent_record_offset":7680,"ancestor_record_offset":2364,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":6728,"parent_record_offset":6516,"ancestor_record_offset":2364,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":2364,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":754,"record_offset":6892,"parent_record_offset":6784,"ancestor_record_offset":2364,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":7136,"parent_record_offset":6892,"ancestor_record_offset":2364,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":2364,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":754,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":2364,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5956,"parent_record_offset":5924,"ancestor_record_offset":2364,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5924,"parent_record_offset":5892,"ancestor_record_offset":2364,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":2364,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":2364,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":2364,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":754,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":2364,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":8536,"parent_record_offset":8448,"ancestor_record_offset":2364,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":8448,"parent_record_offset":8424,"ancestor_record_offset":2364,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":754,"record_offset":6636,"parent_record_offset":6516,"ancestor_record_offset":2364,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":754,"record_offset":6964,"parent_record_offset":6936,"ancestor_record_offset":2364,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":754,"record_offset":6936,"parent_record_offset":6892,"ancestor_record_offset":2364,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":754,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":2364,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":754,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":2364,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":754,"record_offset":5988,"parent_record_offset":5956,"ancestor_record_offset":2364,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/utils/assemble_circular_roof.rs
//     evidence={"module":754,"record_offset":4464,"parent_record_offset":4424,"ancestor_record_offset":2364,"inlinee":45975,"source_line":99,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":754,"record_offset":5600,"parent_record_offset":5560,"ancestor_record_offset":2364,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":754,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":2364,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":754,"record_offset":3136,"parent_record_offset":3104,"ancestor_record_offset":2364,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":754,"record_offset":3260,"parent_record_offset":3104,"ancestor_record_offset":2364,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":754,"record_offset":4596,"parent_record_offset":4464,"ancestor_record_offset":2364,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":3708,"parent_record_offset":3424,"ancestor_record_offset":2364,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":754,"record_offset":5844,"parent_record_offset":5560,"ancestor_record_offset":2364,"inlinee":22391,"source_line":65,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB397
// 首条位置: {"module":754,"inlinee":45975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":754,"record_offset":4500,"parent_record_offset":4464,"ancestor_record_offset":4464,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":754,"record_offset":4596,"parent_record_offset":4464,"ancestor_record_offset":4464,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":754,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4464,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":754,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":4464,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":754,"record_offset":4744,"parent_record_offset":4596,"ancestor_record_offset":4464,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":754,"record_offset":4772,"parent_record_offset":4596,"ancestor_record_offset":4464,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":754,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":4464,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":754,"record_offset":4632,"parent_record_offset":4596,"ancestor_record_offset":4464,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":754,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4464,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00754.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":754,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":4464,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00754.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// 信息 ID: 51aa7c11941bae4886be9dd1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::utils::assemble_circular_roof::assemble_circular_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>; module=754, inline_record=4464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>
// 信息 ID: ae89bf3f7841b8b11d749c28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_roof::utils::assemble_circular_roof::assemble_circular_roof -> ["system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles @ 0x3f06f9 -> 0x8e0400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
