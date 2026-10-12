// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: place_spires.rs.assembly.json.gz

// FUNCTION system_roof::visual::place_spires::place_roof_spires::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 1894:16096 -> host 1894:15756 / rva_12bf010_len_151; exact_symbolizer_function_frame
//   Function-frame RVA span [0x12bf032, 0x12bf140), 62 instructions
//
// FUNCTION system_roof::visual::place_spires::place_roof_spires
// Candidate interfaces: cd898a0c41c18ea683f31317
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3174:4176 -> rva_197e1b0_len_6a1
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_12bf010_len_151: RVA 0x12bf010, 337 PDB bytes; 81 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_12bf010_len_151.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 1894:15756: core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>
// Frames: F1
// 0x1412bf010  56                             pushq	%rsi
// 0x1412bf011  57                             pushq	%rdi
// 0x1412bf012  4883ec78                       subq	$0x78, %rsp
// 0x1412bf016  440f29442460                   movaps	%xmm8, 0x60(%rsp)
// 0x1412bf01c  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1412bf021  0f29742440                     movaps	%xmm6, 0x40(%rsp)
// 0x1412bf026  4c89c7                         movq	%r8, %rdi
// 0x1412bf029  4889ce                         movq	%rcx, %rsi
// Frames: F2
// 0x1412bf02c  f3410f107104                   movss	0x4(%r9), %xmm6
// Frames: F3 -> F2
// 0x1412bf032  4c89c1                         movq	%r8, %rcx
// 0x1412bf035  e8e6b33fff                     callq	0x1406ba420 <.text+0x6b9420>
// 0x1412bf03a  31c9                           xorl	%ecx, %ecx
// 0x1412bf03c  ba00000000                     movl	$0x0, %edx
// 0x1412bf041  84c0                           testb	%al, %al
// 0x1412bf043  0f84f7000000                   je	0x1412bf140 <.text+0x12be140>
// Frames: F4 -> F2
// 0x1412bf049  4889f9                         movq	%rdi, %rcx
// 0x1412bf04c  e87fc62dff                     callq	0x14059b6d0 <.text+0x59a6d0>
// 0x1412bf051  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1412bf056  488d4c2420                     leaq	0x20(%rsp), %rcx
// 0x1412bf05b  e8d0e9e4fe                     callq	0x14010da30 <.text+0x10ca30>
// 0x1412bf060  660f6ec0                       movd	%eax, %xmm0
// 0x1412bf064  48c1e820                       shrq	$0x20, %rax
// 0x1412bf068  660f6ec8                       movd	%eax, %xmm1
// Frames: F5 -> F2
// 0x1412bf06c  660f6fd0                       movdqa	%xmm0, %xmm2
// 0x1412bf070  f30f5ed1                       divss	%xmm1, %xmm2
// 0x1412bf074  f30f5ec8                       divss	%xmm0, %xmm1
// Frames: F6 -> F5 -> F2
// 0x1412bf078  0f28c2                         movaps	%xmm2, %xmm0
// 0x1412bf07b  f30fc2c203                     cmpunordss	%xmm2, %xmm0
// 0x1412bf080  0f28d8                         movaps	%xmm0, %xmm3
// 0x1412bf083  0f54d9                         andps	%xmm1, %xmm3
// 0x1412bf086  f30f5fca                       maxss	%xmm2, %xmm1
// 0x1412bf08a  0f55c1                         andnps	%xmm1, %xmm0
// 0x1412bf08d  0f56c3                         orps	%xmm3, %xmm0
// 0x1412bf090  31c9                           xorl	%ecx, %ecx
// 0x1412bf092  f30f100d0ef32e01               movss	0x12ef30e(%rip), %xmm1  # 0x1425ae3a8
// Frames: F7 -> F2
// 0x1412bf09a  0f2ec8                         ucomiss	%xmm0, %xmm1
// 0x1412bf09d  ba00000000                     movl	$0x0, %edx
// 0x1412bf0a2  0f8698000000                   jbe	0x1412bf140 <.text+0x12be140>
// Frames: F8 -> F2
// 0x1412bf0a8  488d4c2434                     leaq	0x34(%rsp), %rcx
// 0x1412bf0ad  4889fa                         movq	%rdi, %rdx
// 0x1412bf0b0  e83bb33fff                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F9 -> F2
// 0x1412bf0b5  4889f9                         movq	%rdi, %rcx
// 0x1412bf0b8  e893b73fff                     callq	0x1406ba850 <.text+0x6b9850>
// 0x1412bf0bd  f30f1144242c                   movss	%xmm0, 0x2c(%rsp)
// 0x1412bf0c3  f30f114c2430                   movss	%xmm1, 0x30(%rsp)
// 0x1412bf0c9  488d4c2420                     leaq	0x20(%rsp), %rcx
// 0x1412bf0ce  488d54242c                     leaq	0x2c(%rsp), %rdx
// 0x1412bf0d3  e888b651ff                     callq	0x1407da760 <.text+0x7d9760>
// Frames: F10 -> F8 -> F2
// 0x1412bf0d8  f30f107c2434                   movss	0x34(%rsp), %xmm7
// Frames: F11 -> F10 -> F8 -> F2
// 0x1412bf0de  f30f587c2420                   addss	0x20(%rsp), %xmm7
// Frames: F12 -> F8 -> F2
// 0x1412bf0e4  f20f10442438                   movsd	0x38(%rsp), %xmm0
// 0x1412bf0ea  f2440f10442424                 movsd	0x24(%rsp), %xmm8
// Frames: F11 -> F12 -> F8 -> F2
// 0x1412bf0f1  440f58c0                       addps	%xmm0, %xmm8
// Frames: F13 -> F2
// 0x1412bf0f5  4889f9                         movq	%rdi, %rcx
// 0x1412bf0f8  e833b33fff                     callq	0x1406ba430 <.text+0x6b9430>
// 0x1412bf0fd  0f57c9                         xorps	%xmm1, %xmm1
// Frames: F14 -> F15 -> F13 -> F2
// 0x1412bf100  f30f59c8                       mulss	%xmm0, %xmm1
// Frames: F11 -> F10 -> F8 -> F2
// 0x1412bf104  f30f58f9                       addss	%xmm1, %xmm7
// Frames: F11 -> F12 -> F8 -> F2
// 0x1412bf108  0f14c1                         unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
// 0x1412bf10b  410f58c0                       addps	%xmm8, %xmm0
// Frames: F16 -> F17 -> F18 -> F2
// 0x1412bf10f  0f57c9                         xorps	%xmm1, %xmm1
// 0x1412bf112  f30f10cf                       movss	%xmm7, %xmm1            # xmm1 = xmm7[0],xmm1[1,2,3]
// 0x1412bf116  0fc6c84c                       shufps	$0x4c, %xmm0, %xmm1     # xmm1 = xmm1[0,3],xmm0[0,1]
// 0x1412bf11a  0fc6c978                       shufps	$0x78, %xmm1, %xmm1     # xmm1 = xmm1[0,2,3,1]
// Frames: F19 -> F2
// 0x1412bf11e  f30f104724                     movss	0x24(%rdi), %xmm0
// Frames: F20 -> F2
// 0x1412bf123  0f294e10                       movaps	%xmm1, 0x10(%rsi)
// 0x1412bf127  f30f117620                     movss	%xmm6, 0x20(%rsi)
// 0x1412bf12c  f30f114624                     movss	%xmm0, 0x24(%rsi)
// 0x1412bf131  48c746280000803e               movq	$0x3e800000, 0x28(%rsi) # imm = 0x3E800000
// 0x1412bf139  b901000000                     movl	$0x1, %ecx
// 0x1412bf13e  31d2                           xorl	%edx, %edx
// Frames: F21
// 0x1412bf140  48890e                         movq	%rcx, (%rsi)
// 0x1412bf143  48895608                       movq	%rdx, 0x8(%rsi)
// 0x1412bf147  4889f0                         movq	%rsi, %rax
// 0x1412bf14a  0f28742440                     movaps	0x40(%rsp), %xmm6
// 0x1412bf14f  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1412bf154  440f28442460                   movaps	0x60(%rsp), %xmm8
// 0x1412bf15a  4883c478                       addq	$0x78, %rsp
// 0x1412bf15e  5f                             popq	%rdi
// 0x1412bf15f  5e                             popq	%rsi
// 0x1412bf160  c3                             retq
//
// RANGE rva_197e1b0_len_6a1: RVA 0x197e1b0, 1697 PDB bytes; 424 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_197e1b0_len_6a1.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3174:4176: system_roof::visual::place_spires::place_roof_spires
// Frames: F22
// 0x14197e1b0  55                             pushq	%rbp
// 0x14197e1b1  4157                           pushq	%r15
// 0x14197e1b3  4156                           pushq	%r14
// 0x14197e1b5  4155                           pushq	%r13
// 0x14197e1b7  4154                           pushq	%r12
// 0x14197e1b9  56                             pushq	%rsi
// 0x14197e1ba  57                             pushq	%rdi
// 0x14197e1bb  53                             pushq	%rbx
// 0x14197e1bc  4881ec48010000                 subq	$0x148, %rsp            # imm = 0x148
// 0x14197e1c3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14197e1cb  48c785c0000000feffffff         movq	$-0x2, 0xc0(%rbp)
// 0x14197e1d6  4c89c3                         movq	%r8, %rbx
// 0x14197e1d9  4889d7                         movq	%rdx, %rdi
// 0x14197e1dc  4889ce                         movq	%rcx, %rsi
// Frames: F23
// 0x14197e1df  e84ccddcfe                     callq	0x14074af30 <.text+0x749f30>
// 0x14197e1e4  84c0                           testb	%al, %al
// 0x14197e1e6  745d                           je	0x14197e245 <.text+0x197d245>
// Frames: F24 -> F23
// 0x14197e1e8  41be34000000                   movl	$0x34, %r14d
// 0x14197e1ee  b934000000                     movl	$0x34, %ecx
// 0x14197e1f3  e818fbffff                     callq	0x14197dd10 <.text+0x197cd10>
// 0x14197e1f8  4c8d3dd93fd700                 leaq	0xd73fd9(%rip), %r15    # 0x1426f21d8
// 0x14197e1ff  4883f801                       cmpq	$0x1, %rax
// 0x14197e203  0f85af000000                   jne	0x14197e2b8 <.text+0x197d2b8>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x14197e209  4989d1                         movq	%rdx, %r9
// 0x14197e20c  4885d2                         testq	%rdx, %rdx
// 0x14197e20f  7444                           je	0x14197e255 <.text+0x197d255>
// Frames: F30 -> F31 -> F32 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x14197e211  4983f934                       cmpq	$0x34, %r9
// Frames: F32 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x14197e215  733c                           jae	0x14197e253 <.text+0x197d253>
// Frames: F30 -> F31 -> F32 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x14197e217  488d05ba3fd700                 leaq	0xd73fba(%rip), %rax    # 0x1426f21d8
// Frames: F33 -> F34 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x14197e21e  41803c01bf                     cmpb	$-0x41, (%r9,%rax)
// Frames: F26 -> F27 -> F28 -> F29 -> F23
// 0x14197e223  7f30                           jg	0x14197e255 <.text+0x197d255>
// Frames: F35 -> F28 -> F29 -> F23
// 0x14197e225  488d052441d700                 leaq	0xd74124(%rip), %rax    # 0x1426f2350
// 0x14197e22c  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14197e231  488d0da03fd700                 leaq	0xd73fa0(%rip), %rcx    # 0x1426f21d8
// 0x14197e238  ba34000000                     movl	$0x34, %edx
// 0x14197e23d  4531c0                         xorl	%r8d, %r8d
// 0x14197e240  e8eb0e7300                     callq	0x1420af130 <.text+0x20ae130>
// 0x14197e245  31c0                           xorl	%eax, %eax
// 0x14197e247  488985b8000000                 movq	%rax, 0xb8(%rbp)
// 0x14197e24e  e90c020000                     jmp	0x14197e45f <.text+0x197d45f>
// Frames: F26 -> F27 -> F28 -> F29 -> F23
// 0x14197e253  75d0                           jne	0x14197e225 <.text+0x197d225>
// Frames: F29 -> F23
// 0x14197e255  4c89c9                         movq	%r9, %rcx
// 0x14197e258  e8b3faffff                     callq	0x14197dd10 <.text+0x197cd10>
// 0x14197e25d  4883f801                       cmpq	$0x1, %rax
// 0x14197e261  7555                           jne	0x14197e2b8 <.text+0x197d2b8>
// Frames: F25 -> F36 -> F37 -> F28 -> F38 -> F23
// 0x14197e263  4989d0                         movq	%rdx, %r8
// 0x14197e266  4983c002                       addq	$0x2, %r8
// 0x14197e26a  7439                           je	0x14197e2a5 <.text+0x197d2a5>
// Frames: F30 -> F31 -> F32 -> F36 -> F37 -> F28 -> F38 -> F23
// 0x14197e26c  4983f834                       cmpq	$0x34, %r8
// Frames: F32 -> F36 -> F37 -> F28 -> F38 -> F23
// 0x14197e270  7331                           jae	0x14197e2a3 <.text+0x197d2a3>
// Frames: F30 -> F31 -> F32 -> F36 -> F37 -> F28 -> F38 -> F23
// 0x14197e272  488d055f3fd700                 leaq	0xd73f5f(%rip), %rax    # 0x1426f21d8
// Frames: F33 -> F34 -> F36 -> F37 -> F28 -> F38 -> F23
// 0x14197e279  41803c00bf                     cmpb	$-0x41, (%r8,%rax)
// Frames: F36 -> F37 -> F28 -> F38 -> F23
// 0x14197e27e  7f25                           jg	0x14197e2a5 <.text+0x197d2a5>
// Frames: F39 -> F28 -> F38 -> F23
// 0x14197e280  488d05e140d700                 leaq	0xd740e1(%rip), %rax    # 0x1426f2368
// 0x14197e287  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14197e28c  488d0d453fd700                 leaq	0xd73f45(%rip), %rcx    # 0x1426f21d8
// 0x14197e293  ba34000000                     movl	$0x34, %edx
// 0x14197e298  41b934000000                   movl	$0x34, %r9d
// 0x14197e29e  e88d0e7300                     callq	0x1420af130 <.text+0x20ae130>
// Frames: F36 -> F37 -> F28 -> F38 -> F23
// 0x14197e2a3  75db                           jne	0x14197e280 <.text+0x197d280>
// Frames: F40 -> F41 -> F42 -> F43 -> F37 -> F28 -> F38 -> F23
// 0x14197e2a5  4c8d3d2c3fd700                 leaq	0xd73f2c(%rip), %r15    # 0x1426f21d8
// 0x14197e2ac  4d01c7                         addq	%r8, %r15
// 0x14197e2af  41be32000000                   movl	$0x32, %r14d
// Frames: F44 -> F42 -> F43 -> F37 -> F28 -> F38 -> F23
// 0x14197e2b5  4929d6                         subq	%rdx, %r14
// 0x14197e2b8  488d053f41d700                 leaq	0xd7413f(%rip), %rax    # 0x1426f23fe
// 0x14197e2bf  4c8d250a41d700                 leaq	0xd7410a(%rip), %r12    # 0x1426f23d0
// 0x14197e2c6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2d0  0fb648ff                       movzbl	-0x1(%rax), %ecx
// 0x14197e2d4  84c9                           testb	%cl, %cl
// 0x14197e2d6  7808                           js	0x14197e2e0 <.text+0x197d2e0>
// 0x14197e2d8  48ffc8                         decq	%rax
// 0x14197e2db  0fb6c9                         movzbl	%cl, %ecx
// 0x14197e2de  eb5c                           jmp	0x14197e33c <.text+0x197d33c>
// Frames: F53 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2e0  0fb650fe                       movzbl	-0x2(%rax), %edx
// Frames: F54 -> F55 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2e4  80fac0                         cmpb	$-0x40, %dl
// Frames: F55 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2e7  7d25                           jge	0x14197e30e <.text+0x197d30e>
// Frames: F56 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2e9  440fb640fd                     movzbl	-0x3(%rax), %r8d
// Frames: F54 -> F57 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2ee  4180f8bf                       cmpb	$-0x41, %r8b
// Frames: F57 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2f2  7f23                           jg	0x14197e317 <.text+0x197d317>
// Frames: F58 -> F59 -> F60 -> F61 -> F62 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2f4  440fb648fc                     movzbl	-0x4(%rax), %r9d
// 0x14197e2f9  4883c0fc                       addq	$-0x4, %rax
// Frames: F63 -> F64 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e2fd  4183e107                       andl	$0x7, %r9d
// Frames: F65 -> F66 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e301  41c1e106                       shll	$0x6, %r9d
// 0x14197e305  4183e03f                       andl	$0x3f, %r8d
// 0x14197e309  4509c8                         orl	%r9d, %r8d
// 0x14197e30c  eb11                           jmp	0x14197e31f <.text+0x197d31f>
// 0x14197e30e  4883c0fe                       addq	$-0x2, %rax
// 0x14197e312  83e21f                         andl	$0x1f, %edx
// 0x14197e315  eb12                           jmp	0x14197e329 <.text+0x197d329>
// 0x14197e317  4883c0fd                       addq	$-0x3, %rax
// 0x14197e31b  4183e00f                       andl	$0xf, %r8d
// Frames: F65 -> F67 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e31f  41c1e006                       shll	$0x6, %r8d
// 0x14197e323  83e23f                         andl	$0x3f, %edx
// 0x14197e326  4409c2                         orl	%r8d, %edx
// Frames: F65 -> F68 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e329  c1e206                         shll	$0x6, %edx
// 0x14197e32c  80e13f                         andb	$0x3f, %cl
// 0x14197e32f  0fb6c9                         movzbl	%cl, %ecx
// 0x14197e332  09d1                           orl	%edx, %ecx
// Frames: F69 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e334  81f900001100                   cmpl	$0x110000, %ecx         # imm = 0x110000
// 0x14197e33a  740f                           je	0x14197e34b <.text+0x197d34b>
// Frames: F70 -> F71 -> F72 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e33c  83f95c                         cmpl	$0x5c, %ecx
// 0x14197e33f  745a                           je	0x14197e39b <.text+0x197d39b>
// 0x14197e341  83f92f                         cmpl	$0x2f, %ecx
// 0x14197e344  7455                           je	0x14197e39b <.text+0x197d39b>
// Frames: F73 -> F74 -> F75 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e346  4c39e0                         cmpq	%r12, %rax
// Frames: F76 -> F75 -> F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F23
// 0x14197e349  7585                           jne	0x14197e2d0 <.text+0x197d2d0>
// Frames: F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x14197e34b  41bd2e000000                   movl	$0x2e, %r13d
// 0x14197e351  e81adddcfe                     callq	0x14074c070 <.text+0x74b070>
// Frames: F82 -> F83 -> F84 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x14197e356  48833800                       cmpq	$0x0, (%rax)
// 0x14197e35a  0f859a000000                   jne	0x14197e3fa <.text+0x197d3fa>
// Frames: F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x14197e360  4889c1                         movq	%rax, %rcx
// 0x14197e363  31d2                           xorl	%edx, %edx
// 0x14197e365  e83623f5fe                     callq	0x1408d06a0 <.text+0x8cf6a0>
// Frames: F86 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x14197e36a  4885c0                         testq	%rax, %rax
// 0x14197e36d  0f858b000000                   jne	0x14197e3fe <.text+0x197d3fe>
// Frames: F87 -> F79 -> F80 -> F81 -> F23
// 0x14197e373  488d052e3fd700                 leaq	0xd73f2e(%rip), %rax    # 0x1426f22a8
// 0x14197e37a  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14197e37f  488d0d893ed700                 leaq	0xd73e89(%rip), %rcx    # 0x1426f220f
// 0x14197e386  4c8d0ddb3dd700                 leaq	0xd73ddb(%rip), %r9     # 0x1426f2168
// 0x14197e38d  4c8d4518                       leaq	0x18(%rbp), %r8
// 0x14197e391  ba46000000                     movl	$0x46, %edx
// 0x14197e396  e8850b7300                     callq	0x1420aef20 <.text+0x20adf20>
// Frames: F88 -> F23
// 0x14197e39b  4c8d252e40d700                 leaq	0xd7402e(%rip), %r12    # 0x1426f23d0
// 0x14197e3a2  4c29e0                         subq	%r12, %rax
// 0x14197e3a5  4989c0                         movq	%rax, %r8
// 0x14197e3a8  49ffc0                         incq	%r8
// Frames: F25 -> F36 -> F37 -> F28 -> F88 -> F23
// 0x14197e3ab  7432                           je	0x14197e3df <.text+0x197d3df>
// Frames: F30 -> F31 -> F32 -> F36 -> F37 -> F28 -> F88 -> F23
// 0x14197e3ad  4983f82e                       cmpq	$0x2e, %r8
// Frames: F32 -> F36 -> F37 -> F28 -> F88 -> F23
// 0x14197e3b1  732a                           jae	0x14197e3dd <.text+0x197d3dd>
// Frames: F33 -> F34 -> F36 -> F37 -> F28 -> F88 -> F23
// 0x14197e3b3  43803c20bf                     cmpb	$-0x41, (%r8,%r12)
// Frames: F36 -> F37 -> F28 -> F88 -> F23
// 0x14197e3b8  7f25                           jg	0x14197e3df <.text+0x197d3df>
// Frames: F39 -> F28 -> F88 -> F23
// 0x14197e3ba  488d056f3fd700                 leaq	0xd73f6f(%rip), %rax    # 0x1426f2330
// 0x14197e3c1  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14197e3c6  488d0d0340d700                 leaq	0xd74003(%rip), %rcx    # 0x1426f23d0
// 0x14197e3cd  ba2e000000                     movl	$0x2e, %edx
// 0x14197e3d2  41b92e000000                   movl	$0x2e, %r9d
// 0x14197e3d8  e8530d7300                     callq	0x1420af130 <.text+0x20ae130>
// Frames: F36 -> F37 -> F28 -> F88 -> F23
// 0x14197e3dd  75db                           jne	0x14197e3ba <.text+0x197d3ba>
// Frames: F40 -> F41 -> F42 -> F43 -> F37 -> F28 -> F88 -> F23
// 0x14197e3df  4d01c4                         addq	%r8, %r12
// 0x14197e3e2  41bd2d000000                   movl	$0x2d, %r13d
// Frames: F44 -> F42 -> F43 -> F37 -> F28 -> F88 -> F23
// 0x14197e3e8  4929c5                         subq	%rax, %r13
// Frames: F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x14197e3eb  e880dcdcfe                     callq	0x14074c070 <.text+0x74b070>
// Frames: F82 -> F83 -> F84 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x14197e3f0  48833800                       cmpq	$0x0, (%rax)
// 0x14197e3f4  0f8466ffffff                   je	0x14197e360 <.text+0x197d360>
// Frames: F89 -> F83 -> F84 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x14197e3fa  4883c008                       addq	$0x8, %rax
// Frames: F90 -> F91 -> F92 -> F93 -> F94 -> F79 -> F80 -> F81 -> F23
// 0x14197e3fe  48833800                       cmpq	$0x0, (%rax)
// 0x14197e402  0f85a2020000                   jne	0x14197e6aa <.text+0x197d6aa>
// Frames: F95 -> F96 -> F97 -> F98 -> F99 -> F91 -> F92 -> F93 -> F94 -> F79 -> F80 -> F81 -> F23
// 0x14197e408  48c700ffffffff                 movq	$-0x1, (%rax)
// 0x14197e40f  488985b8000000                 movq	%rax, 0xb8(%rbp)
// Frames: F100 -> F101 -> F92 -> F93 -> F94 -> F79 -> F80 -> F81 -> F23
// 0x14197e416  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F102 -> F93 -> F94 -> F79 -> F80 -> F81 -> F23
// 0x14197e41a  488d05973cd700                 leaq	0xd73c97(%rip), %rax    # 0x1426f20b8
// 0x14197e421  4889442428                     movq	%rax, 0x28(%rsp)
// 0x14197e426  4c896c2420                     movq	%r13, 0x20(%rsp)
// 0x14197e42b  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x14197e434  4c89fa                         movq	%r15, %rdx
// 0x14197e437  4d89f0                         movq	%r14, %r8
// 0x14197e43a  4d89e1                         movq	%r12, %r9
// 0x14197e43d  e8eecddcfe                     callq	0x14074b230 <.text+0x74a230>
// 0x14197e442  48898598000000                 movq	%rax, 0x98(%rbp)
// Frames: F95 -> F96 -> F97 -> F98 -> F103 -> F104 -> F104 -> F93 -> F94 -> F79 -> F80 -> F81 -> F23
// 0x14197e449  488b85b8000000                 movq	0xb8(%rbp), %rax
// 0x14197e450  48ff00                         incq	(%rax)
// 0x14197e453  b801000000                     movl	$0x1, %eax
// 0x14197e458  488985b8000000                 movq	%rax, 0xb8(%rbp)
// Frames: F105
// 0x14197e45f  488b0b                         movq	(%rbx), %rcx
// Frames: F106 -> F107 -> F105
// 0x14197e462  488d1557b29500                 leaq	0x95b257(%rip), %rdx    # 0x1422d96c0
// 0x14197e469  e8026ef5fe                     callq	0x1408d5270 <.text+0x8d4270>
// 0x14197e46e  4889c3                         movq	%rax, %rbx
// 0x14197e471  4885c0                         testq	%rax, %rax
// 0x14197e474  0f8481010000                   je	0x14197e5fb <.text+0x197d5fb>
// Frames: F108 -> F107 -> F105
// 0x14197e47a  488d057cb39500                 leaq	0x95b37c(%rip), %rax    # 0x1422d97fd
// 0x14197e481  488985a0000000                 movq	%rax, 0xa0(%rbp)
// 0x14197e488  48c785a800000036000000         movq	$0x36, 0xa8(%rbp)
// 0x14197e493  4989de                         movq	%rbx, %r14
// 0x14197e496  4983c610                       addq	$0x10, %r14
// Frames: F109 -> F110 -> F111 -> F112 -> F111 -> F108 -> F107 -> F105
// 0x14197e49a  48837b1836                     cmpq	$0x36, 0x18(%rbx)
// 0x14197e49f  0f85d7010000                   jne	0x14197e67c <.text+0x197d67c>
// Frames: F108 -> F107 -> F105
// 0x14197e4a5  498b16                         movq	(%r14), %rdx
// Frames: F113 -> F110 -> F111 -> F112 -> F111 -> F108 -> F107 -> F105
// 0x14197e4a8  488d0d4eb39500                 leaq	0x95b34e(%rip), %rcx    # 0x1422d97fd
// 0x14197e4af  41b836000000                   movl	$0x36, %r8d
// 0x14197e4b5  e8c6657c00                     callq	0x142144a80 <.text+0x2143a80>
// 0x14197e4ba  85c0                           testl	%eax, %eax
// Frames: F108 -> F107 -> F105
// 0x14197e4bc  0f85ba010000                   jne	0x14197e67c <.text+0x197d67c>
// Frames: F114 -> F107 -> F105
// 0x14197e4c2  488b0b                         movq	(%rbx), %rcx
// 0x14197e4c5  488b4308                       movq	0x8(%rbx), %rax
// Frames: F107 -> F105
// 0x14197e4c9  ff5018                         callq	*0x18(%rax)
// Frames: F105
// 0x14197e4cc  4889c3                         movq	%rax, %rbx
// 0x14197e4cf  4885c0                         testq	%rax, %rax
// 0x14197e4d2  0f8423010000                   je	0x14197e5fb <.text+0x197d5fb>
// Frames: F115 -> F116
// 0x14197e4d8  488b07                         movq	(%rdi), %rax
// Frames: F117 -> F116
// 0x14197e4db  488b4f08                       movq	0x8(%rdi), %rcx
// Frames: F118 -> F119 -> F120 -> F117 -> F116
// 0x14197e4df  488b90c8000000                 movq	0xc8(%rax), %rdx
// 0x14197e4e6  4c8b80d0000000                 movq	0xd0(%rax), %r8
// Frames: F121 -> F122 -> F123 -> F118 -> F119 -> F120 -> F117 -> F116
// 0x14197e4ed  4e8d0482                       leaq	(%rdx,%r8,4), %r8
// Frames: F124 -> F119 -> F120 -> F117 -> F116
// 0x14197e4f1  0f57c0                         xorps	%xmm0, %xmm0
// 0x14197e4f4  0f114578                       movups	%xmm0, 0x78(%rbp)
// 0x14197e4f8  0f114568                       movups	%xmm0, 0x68(%rbp)
// 0x14197e4fc  0f114558                       movups	%xmm0, 0x58(%rbp)
// 0x14197e500  48c7858800000000000000         movq	$0x0, 0x88(%rbp)
// Frames: F125 -> F120 -> F117 -> F116
// 0x14197e50b  4c8d8998000000                 leaq	0x98(%rcx), %r9
// Frames: F126 -> F120 -> F117 -> F116
// 0x14197e512  4c8d9120010000                 leaq	0x120(%rcx), %r10
// Frames: F127 -> F120 -> F117 -> F116
// 0x14197e519  48895530                       movq	%rdx, 0x30(%rbp)
// 0x14197e51d  4c894538                       movq	%r8, 0x38(%rbp)
// 0x14197e521  488d15e8a39500                 leaq	0x95a3e8(%rip), %rdx    # 0x1422d8910
// 0x14197e528  48895540                       movq	%rdx, 0x40(%rbp)
// 0x14197e52c  48c7454800000000               movq	$0x0, 0x48(%rbp)
// 0x14197e534  48895550                       movq	%rdx, 0x50(%rbp)
// Frames: F128 -> F129 -> F130 -> F116
// 0x14197e538  4c895518                       movq	%r10, 0x18(%rbp)
// 0x14197e53c  4c894d20                       movq	%r9, 0x20(%rbp)
// 0x14197e540  48894528                       movq	%rax, 0x28(%rbp)
// 0x14197e544  48898d90000000                 movq	%rcx, 0x90(%rbp)
// Frames: F129 -> F130 -> F116
// 0x14197e54b  488d8da0000000                 leaq	0xa0(%rbp), %rcx
// 0x14197e552  488d5518                       leaq	0x18(%rbp), %rdx
// 0x14197e556  e825d0f4fe                     callq	0x1408cb580 <.text+0x8ca580>
// Frames: F131
// 0x14197e55b  488b0e                         movq	(%rsi), %rcx
// 0x14197e55e  e8bdc7cdfe                     callq	0x14065ad20 <.text+0x659d20>
// 0x14197e563  488bb5b0000000                 movq	0xb0(%rbp), %rsi
// Frames: F132 -> F133 -> F131
// 0x14197e56a  4885f6                         testq	%rsi, %rsi
// Frames: F133 -> F131
// 0x14197e56d  7465                           je	0x14197e5d4 <.text+0x197d5d4>
// Frames: F134 -> F135 -> F136 -> F137 -> F131
// 0x14197e56f  4889f1                         movq	%rsi, %rcx
// 0x14197e572  48c1e105                       shlq	$0x5, %rcx
// Frames: F138 -> F136 -> F137 -> F131
// 0x14197e576  483b4b20                       cmpq	0x20(%rbx), %rcx
// 0x14197e57a  0f8736010000                   ja	0x14197e6b6 <.text+0x197d6b6>
// Frames: F131
// 0x14197e580  4c8b8da8000000                 movq	0xa8(%rbp), %r9
// Frames: F137 -> F131
// 0x14197e587  4989d8                         movq	%rbx, %r8
// 0x14197e58a  4983c008                       addq	$0x8, %r8
// Frames: F139 -> F136 -> F137 -> F131
// 0x14197e58e  48c7450800000000               movq	$0x0, 0x8(%rbp)
// 0x14197e596  48894d10                       movq	%rcx, 0x10(%rbp)
// Frames: F140 -> F136 -> F137 -> F131
// 0x14197e59a  488d5508                       leaq	0x8(%rbp), %rdx
// 0x14197e59e  4889542428                     movq	%rdx, 0x28(%rsp)
// 0x14197e5a3  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x14197e5a8  48c744243800000000             movq	$0x0, 0x38(%rsp)
// 0x14197e5b1  48c744243001000000             movq	$0x1, 0x30(%rsp)
// 0x14197e5ba  488d4d18                       leaq	0x18(%rbp), %rcx
// 0x14197e5be  4889c2                         movq	%rax, %rdx
// 0x14197e5c1  e88ab8dbfe                     callq	0x140739e50 <.text+0x738e50>
// Frames: F141 -> F140 -> F136 -> F137 -> F131
// 0x14197e5c6  488b4518                       movq	0x18(%rbp), %rax
// 0x14197e5ca  4883f80b                       cmpq	$0xb, %rax
// 0x14197e5ce  0f85e9000000                   jne	0x14197e6bd <.text+0x197d6bd>
// Frames: F142 -> F143 -> F144 -> F131
// 0x14197e5d4  488933                         movq	%rsi, (%rbx)
// Frames: F104 -> F145
// 0x14197e5d7  488b95a0000000                 movq	0xa0(%rbp), %rdx
// Frames: F146 -> F147 -> F104 -> F104 -> F145
// 0x14197e5de  4885d2                         testq	%rdx, %rdx
// 0x14197e5e1  7471                           je	0x14197e654 <.text+0x197d654>
// Frames: F104 -> F145
// 0x14197e5e3  488b8da8000000                 movq	0xa8(%rbp), %rcx
// Frames: F148 -> F149 -> F147 -> F104 -> F104 -> F145
// 0x14197e5ea  48c1e205                       shlq	$0x5, %rdx
// Frames: F150 -> F151 -> F152 -> F104 -> F104 -> F145
// 0x14197e5ee  41b810000000                   movl	$0x10, %r8d
// 0x14197e5f4  e897c1a5fe                     callq	0x1403da790 <.text+0x3d9790>
// 0x14197e5f9  eb59                           jmp	0x14197e654 <.text+0x197d654>
// Frames: F153 -> F154 -> F155 -> F156
// 0x14197e5fb  488b05263d2f01                 movq	0x12f3d26(%rip), %rax   # 0x142c72328
// 0x14197e602  488b00                         movq	(%rax), %rax
// Frames: F157 -> F156
// 0x14197e605  4883f802                       cmpq	$0x2, %rax
// Frames: F156
// 0x14197e609  7249                           jb	0x14197e654 <.text+0x197d654>
// Frames: F158 -> F156
// 0x14197e60b  488d05363ed700                 leaq	0xd73e36(%rip), %rax    # 0x1426f2448
// 0x14197e612  48894518                       movq	%rax, 0x18(%rbp)
// 0x14197e616  48c7452001000000               movq	$0x1, 0x20(%rbp)
// 0x14197e61e  488d05933ad700                 leaq	0xd73a93(%rip), %rax    # 0x1426f20b8
// 0x14197e625  48894528                       movq	%rax, 0x28(%rbp)
// 0x14197e629  0f57c0                         xorps	%xmm0, %xmm0
// 0x14197e62c  0f114530                       movups	%xmm0, 0x30(%rbp)
// Frames: F159 -> F156
// 0x14197e630  48c744242000000000             movq	$0x0, 0x20(%rsp)
// 0x14197e639  4c8d05403ed700                 leaq	0xd73e40(%rip), %r8     # 0x1426f2480
// 0x14197e640  488d4d18                       leaq	0x18(%rbp), %rcx
// 0x14197e644  ba02000000                     movl	$0x2, %edx
// 0x14197e649  41b93a000000                   movl	$0x3a, %r9d
// 0x14197e64f  e8dc01ecfe                     callq	0x14083e830 <.text+0x83d830>
// Frames: F160
// 0x14197e654  488b8db8000000                 movq	0xb8(%rbp), %rcx
// 0x14197e65b  488b9598000000                 movq	0x98(%rbp), %rdx
// 0x14197e662  e8e9f5ffff                     callq	0x14197dc50 <.text+0x197cc50>
// 0x14197e667  90                             nop
// 0x14197e668  4881c448010000                 addq	$0x148, %rsp            # imm = 0x148
// 0x14197e66f  5b                             popq	%rbx
// 0x14197e670  5f                             popq	%rdi
// 0x14197e671  5e                             popq	%rsi
// 0x14197e672  415c                           popq	%r12
// 0x14197e674  415d                           popq	%r13
// 0x14197e676  415e                           popq	%r14
// 0x14197e678  415f                           popq	%r15
// 0x14197e67a  5d                             popq	%rbp
// 0x14197e67b  c3                             retq
// Frames: F108 -> F107 -> F105
// 0x14197e67c  48c7451800000000               movq	$0x0, 0x18(%rbp)
// 0x14197e684  488d053db09500                 leaq	0x95b03d(%rip), %rax    # 0x1422d96c8
// 0x14197e68b  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14197e690  488d95a0000000                 leaq	0xa0(%rbp), %rdx
// 0x14197e697  4c8d4d18                       leaq	0x18(%rbp), %r9
// 0x14197e69b  31c9                           xorl	%ecx, %ecx
// 0x14197e69d  4d89f0                         movq	%r14, %r8
// 0x14197e6a0  e83b307300                     callq	0x1420b16e0 <.text+0x20b06e0>
// 0x14197e6a5  e98f000000                     jmp	0x14197e739 <.text+0x197d739>
// Frames: F161 -> F93 -> F94 -> F79 -> F80 -> F81 -> F23
// 0x14197e6aa  488d0d673cd700                 leaq	0xd73c67(%rip), %rcx    # 0x1426f2318
// 0x14197e6b1  e8ca027300                     callq	0x1420ae980 <.text+0x20ad980>
// 0x14197e6b6  b80b000000                     movl	$0xb, %eax
// 0x14197e6bb  eb28                           jmp	0x14197e6e5 <.text+0x197d6e5>
// Frames: F162 -> F140 -> F136 -> F137 -> F131
// 0x14197e6bd  488b4d60                       movq	0x60(%rbp), %rcx
// 0x14197e6c1  48894d00                       movq	%rcx, (%rbp)
// 0x14197e6c5  0f104520                       movups	0x20(%rbp), %xmm0
// 0x14197e6c9  0f104d30                       movups	0x30(%rbp), %xmm1
// 0x14197e6cd  0f105540                       movups	0x40(%rbp), %xmm2
// 0x14197e6d1  0f105d50                       movups	0x50(%rbp), %xmm3
// 0x14197e6d5  0f295df0                       movaps	%xmm3, -0x10(%rbp)
// 0x14197e6d9  0f2955e0                       movaps	%xmm2, -0x20(%rbp)
// 0x14197e6dd  0f294dd0                       movaps	%xmm1, -0x30(%rbp)
// 0x14197e6e1  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// Frames: F163 -> F131
// 0x14197e6e5  48894518                       movq	%rax, 0x18(%rbp)
// 0x14197e6e9  0f2845c0                       movaps	-0x40(%rbp), %xmm0
// 0x14197e6ed  0f284dd0                       movaps	-0x30(%rbp), %xmm1
// 0x14197e6f1  0f2855e0                       movaps	-0x20(%rbp), %xmm2
// 0x14197e6f5  0f285df0                       movaps	-0x10(%rbp), %xmm3
// 0x14197e6f9  0f114520                       movups	%xmm0, 0x20(%rbp)
// 0x14197e6fd  0f114d30                       movups	%xmm1, 0x30(%rbp)
// 0x14197e701  0f115540                       movups	%xmm2, 0x40(%rbp)
// 0x14197e705  0f115d50                       movups	%xmm3, 0x50(%rbp)
// 0x14197e709  488b4500                       movq	(%rbp), %rax
// 0x14197e70d  48894560                       movq	%rax, 0x60(%rbp)
// 0x14197e711  488d05e83cd700                 leaq	0xd73ce8(%rip), %rax    # 0x1426f2400
// 0x14197e718  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14197e71d  488d0d643ad700                 leaq	0xd73a64(%rip), %rcx    # 0x1426f2188
// 0x14197e724  4c8d0d8d3ad700                 leaq	0xd73a8d(%rip), %r9     # 0x1426f21b8
// 0x14197e72b  4c8d4518                       leaq	0x18(%rbp), %r8
// 0x14197e72f  ba2b000000                     movl	$0x2b, %edx
// 0x14197e734  e8e7077300                     callq	0x1420aef20 <.text+0x20adf20>
// 0x14197e739  0f0b                           ud2
// 0x14197e73b  0f1f440000                     nopl	(%rax,%rax)
// 0x14197e740  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14197e745  55                             pushq	%rbp
// 0x14197e746  4157                           pushq	%r15
// 0x14197e748  4156                           pushq	%r14
// 0x14197e74a  4155                           pushq	%r13
// 0x14197e74c  4154                           pushq	%r12
// 0x14197e74e  56                             pushq	%rsi
// 0x14197e74f  57                             pushq	%rdi
// 0x14197e750  53                             pushq	%rbx
// 0x14197e751  4883ec48                       subq	$0x48, %rsp
// 0x14197e755  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14197e75c  488d4d18                       leaq	0x18(%rbp), %rcx
// 0x14197e760  e8cbefa3fe                     callq	0x1403bd730 <.text+0x3bc730>
// 0x14197e765  90                             nop
// 0x14197e766  4883c448                       addq	$0x48, %rsp
// 0x14197e76a  5b                             popq	%rbx
// 0x14197e76b  5f                             popq	%rdi
// 0x14197e76c  5e                             popq	%rsi
// 0x14197e76d  415c                           popq	%r12
// 0x14197e76f  415d                           popq	%r13
// 0x14197e771  415e                           popq	%r14
// 0x14197e773  415f                           popq	%r15
// 0x14197e775  5d                             popq	%rbp
// 0x14197e776  c3                             retq
// 0x14197e777  660f1f840000000000             nopw	(%rax,%rax)
// 0x14197e780  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14197e785  55                             pushq	%rbp
// 0x14197e786  4157                           pushq	%r15
// 0x14197e788  4156                           pushq	%r14
// 0x14197e78a  4155                           pushq	%r13
// 0x14197e78c  4154                           pushq	%r12
// 0x14197e78e  56                             pushq	%rsi
// 0x14197e78f  57                             pushq	%rdi
// 0x14197e790  53                             pushq	%rbx
// 0x14197e791  4883ec48                       subq	$0x48, %rsp
// 0x14197e795  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14197e79c  488b85b8000000                 movq	0xb8(%rbp), %rax
// Frames: F95 -> F96 -> F97 -> F98 -> F103 -> F104 -> F104 -> F93 -> F94 -> F79 -> F80 -> F81 -> F23
// 0x14197e7a3  48ff00                         incq	(%rax)
// Frames: F93 -> F94 -> F79 -> F80 -> F81 -> F23
// 0x14197e7a6  4883c448                       addq	$0x48, %rsp
// 0x14197e7aa  5b                             popq	%rbx
// 0x14197e7ab  5f                             popq	%rdi
// 0x14197e7ac  5e                             popq	%rsi
// 0x14197e7ad  415c                           popq	%r12
// 0x14197e7af  415d                           popq	%r13
// 0x14197e7b1  415e                           popq	%r14
// 0x14197e7b3  415f                           popq	%r15
// 0x14197e7b5  5d                             popq	%rbp
// 0x14197e7b6  c3                             retq
// 0x14197e7b7  660f1f840000000000             nopw	(%rax,%rax)
// 0x14197e7c0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14197e7c5  55                             pushq	%rbp
// 0x14197e7c6  4157                           pushq	%r15
// 0x14197e7c8  4156                           pushq	%r14
// 0x14197e7ca  4155                           pushq	%r13
// 0x14197e7cc  4154                           pushq	%r12
// 0x14197e7ce  56                             pushq	%rsi
// 0x14197e7cf  57                             pushq	%rdi
// 0x14197e7d0  53                             pushq	%rbx
// 0x14197e7d1  4883ec48                       subq	$0x48, %rsp
// 0x14197e7d5  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F104 -> F145
// 0x14197e7dc  488b95a0000000                 movq	0xa0(%rbp), %rdx
// Frames: F146 -> F147 -> F104 -> F104 -> F145
// 0x14197e7e3  4885d2                         testq	%rdx, %rdx
// 0x14197e7e6  7416                           je	0x14197e7fe <.text+0x197d7fe>
// Frames: F104 -> F145
// 0x14197e7e8  488b8da8000000                 movq	0xa8(%rbp), %rcx
// Frames: F148 -> F149 -> F147 -> F104 -> F104 -> F145
// 0x14197e7ef  48c1e205                       shlq	$0x5, %rdx
// Frames: F150 -> F151 -> F152 -> F104 -> F104 -> F145
// 0x14197e7f3  41b810000000                   movl	$0x10, %r8d
// 0x14197e7f9  e892bfa5fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F145
// 0x14197e7fe  90                             nop
// 0x14197e7ff  4883c448                       addq	$0x48, %rsp
// 0x14197e803  5b                             popq	%rbx
// 0x14197e804  5f                             popq	%rdi
// 0x14197e805  5e                             popq	%rsi
// 0x14197e806  415c                           popq	%r12
// 0x14197e808  415d                           popq	%r13
// 0x14197e80a  415e                           popq	%r14
// 0x14197e80c  415f                           popq	%r15
// 0x14197e80e  5d                             popq	%rbp
// 0x14197e80f  c3                             retq
// 0x14197e810  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14197e815  55                             pushq	%rbp
// 0x14197e816  4157                           pushq	%r15
// 0x14197e818  4156                           pushq	%r14
// 0x14197e81a  4155                           pushq	%r13
// 0x14197e81c  4154                           pushq	%r12
// 0x14197e81e  56                             pushq	%rsi
// 0x14197e81f  57                             pushq	%rdi
// 0x14197e820  53                             pushq	%rbx
// 0x14197e821  4883ec48                       subq	$0x48, %rsp
// 0x14197e825  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14197e82c  488b8db8000000                 movq	0xb8(%rbp), %rcx
// 0x14197e833  488b9598000000                 movq	0x98(%rbp), %rdx
// Frames: F160
// 0x14197e83a  e811f4ffff                     callq	0x14197dc50 <.text+0x197cc50>
// Frames: F22
// 0x14197e83f  90                             nop
// 0x14197e840  4883c448                       addq	$0x48, %rsp
// 0x14197e844  5b                             popq	%rbx
// 0x14197e845  5f                             popq	%rdi
// 0x14197e846  5e                             popq	%rsi
// 0x14197e847  415c                           popq	%r12
// 0x14197e849  415d                           popq	%r13
// 0x14197e84b  415e                           popq	%r14
// 0x14197e84d  415f                           popq	%r15
// 0x14197e84f  5d                             popq	%rbp
// 0x14197e850  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:293:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::haae2e1b180f1ca12
// F2: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:294:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::haae2e1b180f1ca12
// F3: crates/systems/roof/src/visual/place_spires.rs:29:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F4: crates/systems/roof/src/visual/place_spires.rs:30:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F5: crates/systems/roof/src/visual/place_spires.rs:32:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F6: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:925:0 core::f32::impl$0::max
// F7: crates/systems/roof/src/visual/place_spires.rs:35:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F8: crates/systems/roof/src/visual/place_spires.rs:36:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F9: crates/systems/roof/src/visual/place_spires.rs:37:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F10: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1083:0 glam::f32::vec3::impl$12::add
// F11: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:112:0 core::ops::arith::impl$12::add
// F12: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1084:0 glam::f32::vec3::impl$12::add
// F13: crates/systems/roof/src/visual/place_spires.rs:38:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:351:0 core::ops::arith::impl$124::mul
// F15: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1050:0 glam::f32::vec3::impl$9::mul
// F16: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs:93:0 glam::f32::sse2::vec4::Vec4::new
// F17: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:155:0 glam::f32::vec3::Vec3::extend
// F18: crates/systems/roof/src/visual/place_spires.rs:40:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F19: crates/systems/roof/src/visual/place_spires.rs:42:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F20: crates/systems/roof/src/visual/place_spires.rs:39:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:295:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::haae2e1b180f1ca12
// F22: crates/systems/roof/src/visual/place_spires.rs:18:0 system_roof::visual::place_spires::place_roof_spires::h29310966e9450a1c
// F23: crates/systems/roof/src/visual/place_spires.rs:23:0 system_roof::visual::place_spires::place_roof_spires::h29310966e9450a1c
// F24: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:222:0 core::str::impl$0::is_char_boundary
// F26: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:334:0 core::str::traits::impl$9::get
// F27: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:365:0 core::str::traits::impl$9::index
// F28: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:61:0 core::str::traits::impl$4::index
// F29: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F31: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:226:0 core::str::impl$0::is_char_boundary
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1103:0 core::num::impl$6::is_utf8_char_boundary
// F34: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:238:0 core::str::impl$0::is_char_boundary
// F35: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:367:0 core::str::traits::impl$9::index
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:403:0 core::str::traits::impl$10::get
// F37: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:436:0 core::str::traits::impl$10::index
// F38: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:438:0 core::str::traits::impl$10::index
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:964:0 core::ptr::const_ptr::impl$0::add
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:213:0 core::str::traits::impl$7::get_unchecked
// F42: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:425:0 core::str::traits::impl$10::get_unchecked
// F43: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:406:0 core::str::traits::impl$10::get
// F44: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:214:0 core::str::traits::impl$7::get_unchecked
// F45: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:85:0 core::str::validations::next_code_point_reverse
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/iter.rs:134:0 core::str::iter::impl$2::next_back
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/iter.rs:222:0 core::str::iter::impl$6::next_back
// F48: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:695:0 core::str::pattern::impl$11::next_back
// F49: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:303:0 core::str::pattern::ReverseSearcher::next_match_back
// F50: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:780:0 core::str::pattern::impl$22::next_match_back
// F51: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:1314:0 core::str::impl$0::rfind
// F52: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F53: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:94:0 core::str::validations::next_code_point_reverse
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:25:0 core::str::validations::utf8_is_cont_byte
// F55: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:96:0 core::str::validations::next_code_point_reverse
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:99:0 core::str::validations::next_code_point_reverse
// F57: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:101:0 core::str::validations::next_code_point_reverse
// F58: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:539:0 core::ptr::non_null::NonNull::offset
// F59: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:720:0 core::ptr::non_null::NonNull::sub
// F60: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:131:0 core::slice::iter::Iter::pre_dec_end
// F61: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:401:0 core::slice::iter::impl$182::next_back
// F62: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F63: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:12:0 core::str::validations::utf8_first_byte
// F64: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F65: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:18:0 core::str::validations::utf8_acc_cont_byte
// F66: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:106:0 core::str::validations::next_code_point_reverse
// F67: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:108:0 core::str::validations::next_code_point_reverse
// F68: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:110:0 core::str::validations::next_code_point_reverse
// F69: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::option::Option::map
// F70: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:285:0 core::slice::iter::impl$181::any
// F71: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:641:0 core::str::pattern::impl$8::matches
// F72: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:698:0 core::str::pattern::impl$11::next_back
// F73: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:1798:0 core::ptr::non_null::impl$12::eq
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$182::next_back
// F75: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F76: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:398:0 core::slice::iter::impl$182::next_back
// F77: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:93:0 puffin::impl$6::call::THREAD_PROFILER::__getit
// F78: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:283:0 std::thread::local::LocalKey::try_with
// F79: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:260:0 std::thread::local::LocalKey::with
// F80: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F81: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F82: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F83: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/mod.rs:46:0 std::sys::thread_local::lazy::LazyKeyInner::get
// F84: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:174:0 std::sys::thread_local::fast_local::Key::get
// F85: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:176:0 std::sys::thread_local::fast_local::Key::get
// F86: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1207:0 core::option::Option::ok_or
// F87: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1034:0 core::result::Result::expect
// F88: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F89: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:684:0 core::option::Option::as_ref
// F90: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1787:0 core::cell::BorrowRefMut::new
// F91: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1091:0 core::cell::RefCell::try_borrow_mut
// F92: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1059:0 core::cell::RefCell::borrow_mut
// F93: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F94: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:284:0 std::thread::local::LocalKey::try_with
// F95: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F96: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:922:0 core::mem::replace
// F97: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:482:0 core::cell::Cell::replace
// F98: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:411:0 core::cell::Cell::set
// F99: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1789:0 core::cell::BorrowRefMut::new
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:2126:0 core::cell::UnsafeCell::get
// F101: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1099:0 core::cell::RefCell::try_borrow_mut
// F102: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F103: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1776:0 core::cell::impl$42::drop
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:515:0 core::ptr::drop_in_place
// F105: crates/systems/roof/src/visual/place_spires.rs:25:0 system_roof::visual::place_spires::place_roof_spires::h29310966e9450a1c
// F106: crates/country-core/src/asset_libraries/ssbo_library.rs:203:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F107: crates/country-core/src/asset_libraries/ssbo_library.rs:239:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
// F108: crates/country-core/src/asset_libraries/ssbo_library.rs:204:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F109: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/cmp.rs:84:0 core::slice::cmp::impl$5::equal
// F110: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/cmp.rs:16:0 core::slice::cmp::impl$0::eq
// F111: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1655:0 core::cmp::impls::impl$9::eq
// F112: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:30:0 core::str::traits::impl$1::eq
// F113: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/cmp.rs:92:0 core::slice::cmp::impl$5::equal
// F114: crates/country-core/src/asset_libraries/ssbo_library.rs:205:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F115: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:440:0 bevy_ecs::system::query::Query::iter
// F116: crates/systems/roof/src/visual/place_spires.rs:26:0 system_roof::visual::place_spires::place_roof_spires::h29310966e9450a1c
// F117: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:442:0 bevy_ecs::system::query::Query::iter
// F118: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1533:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F119: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:49:0 bevy_ecs::query::iter::QueryIter::new
// F120: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:1246:0 bevy_ecs::query::state::QueryState::iter_unchecked_manual
// F121: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F122: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F123: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F124: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1528:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F125: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:47:0 bevy_ecs::query::iter::QueryIter::new
// F126: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:46:0 bevy_ecs::query::iter::QueryIter::new
// F127: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:42:0 bevy_ecs::query::iter::QueryIter::new
// F128: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/traits/collect.rs:322:0 core::iter::traits::collect::impl$0::into_iter
// F129: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2894:0 alloc::vec::impl$14::from_iter
// F130: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/traits/iterator.rs:2003:0 core::iter::traits::iterator::Iterator::collect
// F131: crates/systems/roof/src/visual/place_spires.rs:55:0 system_roof::visual::place_spires::place_roof_spires::h29310966e9450a1c
// F132: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:158:0 core::slice::impl$0::is_empty
// F133: crates/country-core/src/render/ssbo.rs:357:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F134: crates/rhapsody/src/raw/bytes.rs:13:0 rhapsody::raw::bytes::slice_as_bytes
// F135: crates/country-core/src/render/ssbo.rs:218:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F136: crates/country-core/src/render/ssbo.rs:161:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write
// F137: crates/country-core/src/render/ssbo.rs:358:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F138: crates/country-core/src/render/ssbo.rs:222:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F139: crates/country-core/src/render/ssbo.rs:224:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F140: crates/country-core/src/render/ssbo.rs:223:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F141: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:827:0 core::result::Result::map_err
// F142: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3292:0 core::sync::atomic::atomic_store
// F143: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:2442:0 core::sync::atomic::AtomicUsize::store
// F144: crates/country-core/src/render/ssbo.rs:361:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F145: crates/systems/roof/src/visual/place_spires.rs:56:0 system_roof::visual::place_spires::place_roof_spires::h29310966e9450a1c
// F146: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:280:0 alloc::raw_vec::RawVec::current_memory
// F147: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:554:0 alloc::raw_vec::impl$4::drop
// F148: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::unchecked_mul
// F149: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:290:0 alloc::raw_vec::RawVec::current_memory
// F150: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:119:0 alloc::alloc::dealloc
// F151: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:256:0 alloc::alloc::impl$1::deallocate
// F152: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:555:0 alloc::raw_vec::impl$4::drop
// F153: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3307:0 core::sync::atomic::atomic_load
// F154: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3268:0 core::sync::atomic::AtomicUsize::load
// F155: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs:1276:0 log::max_level
// F156: crates/systems/roof/src/visual/place_spires.rs:58:0 system_roof::visual::place_spires::place_roof_spires::h29310966e9450a1c
// F157: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1189:0 core::cmp::PartialOrd::le
// F158: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:333:0 core::fmt::Arguments::new_const
// F159: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs:73:0 log::__private_api::log
// F160: crates/systems/roof/src/visual/place_spires.rs:60:0 system_roof::visual::place_spires::place_roof_spires::h29310966e9450a1c
// F161: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1061:0 core::cell::RefCell::borrow_mut
// F162: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:829:0 core::result::Result::map_err
// F163: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1077:0 core::result::Result::unwrap
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/visual/place_spires.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fc4b22dfc9b1321a61b518260f885d9435fa1253b0612961653d3261ee8a6d08
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: place_spires.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1894, 3174
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::place_spires::place_roof_spires
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3174,"record_offset":4176,"rva":26730928,"va":5395440048}
// 编译实例 RVAs: 0x197e1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22810677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8cbefa3fe","size":5,"soft_fail":false,"target_va":5372630832,"va":5395441504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22691433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e897c1a5fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395441140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20068419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8bdc7cdfe","size":5,"soft_fail":false,"target_va":5375372576,"va":5395440990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19154806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e88ab8dbfe","size":5,"soft_fail":false,"target_va":5376286288,"va":5395441089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cb580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17510363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e825d0f4fe","size":5,"soft_fail":false,"target_va":5377930624,"va":5395440982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408d5270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17469950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8026ef5fe","size":5,"soft_fail":false,"target_va":5377970800,"va":5395440745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8152518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8c6657c00","size":5,"soft_fail":false,"target_va":5403593344,"va":5395440821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7537354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8ca027300","size":5,"soft_fail":false,"target_va":5402978688,"va":5395441329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7548987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e83b307300","size":5,"soft_fail":false,"target_va":5402990304,"va":5395441312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8e9f5ffff","size":5,"soft_fail":false,"target_va":5395438672,"va":5395441250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7539589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8850b7300","size":5,"soft_fail":false,"target_va":5402980128,"va":5395440534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e818fbffff","size":5,"soft_fail":false,"target_va":5395438864,"va":5395440115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7540459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8eb0e7300","size":5,"soft_fail":false,"target_va":5402980656,"va":5395440192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18087460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8dc01ecfe","size":5,"soft_fail":false,"target_va":5377353776,"va":5395441231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19083794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e8eecddcfe","size":5,"soft_fail":false,"target_va":5376356912,"va":5395440701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19079910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e81adddcfe","size":5,"soft_fail":false,"target_va":5376360560,"va":5395440465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19083956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e84ccddcfe","size":5,"soft_fail":false,"target_va":5376356144,"va":5395440095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h8ca862e57c4aa143 (.llvm.3516157794931125432)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17489098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395440048,"raw_hex":"e83623f5fe","size":5,"soft_fail":false,"target_va":5377951392,"va":5395440485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5395440048,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5395440841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3174,"record_offset":7256,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3174,"record_offset":8132,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3174,"record_offset":8160,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3174,"record_offset":7472,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3174,"record_offset":7828,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3174,"record_offset":8216,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3174,"record_offset":7016,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3174,"record_offset":7556,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":14705,"source_line":356,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3174,"record_offset":8188,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3174,"record_offset":8048,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3174,"record_offset":5948,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3174,"record_offset":4360,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3174,"record_offset":4936,"parent_record_offset":4176,"ancestor_record_offset":4176,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3174,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":4176,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3174,"record_offset":7980,"parent_record_offset":7880,"ancestor_record_offset":4176,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3174,"record_offset":7912,"parent_record_offset":7880,"ancestor_record_offset":4176,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3174,"record_offset":7880,"parent_record_offset":7852,"ancestor_record_offset":4176,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3174,"record_offset":7496,"parent_record_offset":7472,"ancestor_record_offset":4176,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3174,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":4176,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3174,"record_offset":7344,"parent_record_offset":7308,"ancestor_record_offset":4176,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3174,"record_offset":7284,"parent_record_offset":7256,"ancestor_record_offset":4176,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3174,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":4176,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3174,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":4176,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3174,"record_offset":6396,"parent_record_offset":6368,"ancestor_record_offset":4176,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3174,"record_offset":6308,"parent_record_offset":6276,"ancestor_record_offset":4176,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3174,"record_offset":6340,"parent_record_offset":6308,"ancestor_record_offset":4176,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3174,"record_offset":6512,"parent_record_offset":6340,"ancestor_record_offset":4176,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3174,"record_offset":6624,"parent_record_offset":6600,"ancestor_record_offset":4176,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3174,"record_offset":7084,"parent_record_offset":7044,"ancestor_record_offset":4176,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3174,"record_offset":7520,"parent_record_offset":7496,"ancestor_record_offset":4176,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3174,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":4176,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3174,"record_offset":7944,"parent_record_offset":7912,"ancestor_record_offset":4176,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3174,"record_offset":4572,"parent_record_offset":4476,"ancestor_record_offset":4176,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3174,"record_offset":6200,"parent_record_offset":6172,"ancestor_record_offset":4176,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3174,"record_offset":5532,"parent_record_offset":5072,"ancestor_record_offset":4176,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3174,"record_offset":6248,"parent_record_offset":6068,"ancestor_record_offset":4176,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3174,"record_offset":4884,"parent_record_offset":4856,"ancestor_record_offset":4176,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3174,"record_offset":6576,"parent_record_offset":6276,"ancestor_record_offset":4176,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3174,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":4176,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3174,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":4176,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3174,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":4176,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3174,"record_offset":5492,"parent_record_offset":5460,"ancestor_record_offset":4176,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3174,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":4176,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3174,"record_offset":6976,"parent_record_offset":6028,"ancestor_record_offset":4176,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3174,"record_offset":7724,"parent_record_offset":7656,"ancestor_record_offset":4176,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3174,"record_offset":7168,"parent_record_offset":7140,"ancestor_record_offset":4176,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3174,"record_offset":7196,"parent_record_offset":7168,"ancestor_record_offset":4176,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3174,"record_offset":4508,"parent_record_offset":4476,"ancestor_record_offset":4176,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3174,"record_offset":7596,"parent_record_offset":7556,"ancestor_record_offset":4176,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3174,"record_offset":7372,"parent_record_offset":7344,"ancestor_record_offset":4176,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3174,"record_offset":4536,"parent_record_offset":4508,"ancestor_record_offset":4176,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3174,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":4176,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3174,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":4176,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3174,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":4176,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3174,"record_offset":5236,"parent_record_offset":5128,"ancestor_record_offset":4176,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3174,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":4176,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3174,"record_offset":4968,"parent_record_offset":4936,"ancestor_record_offset":4176,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3174,"record_offset":5100,"parent_record_offset":5072,"ancestor_record_offset":4176,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3174,"record_offset":5072,"parent_record_offset":5040,"ancestor_record_offset":4176,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3174,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":4176,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3174,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":4176,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3174,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":4176,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3174,"record_offset":5564,"parent_record_offset":5040,"ancestor_record_offset":4176,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3174,"record_offset":4672,"parent_record_offset":4640,"ancestor_record_offset":4176,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3174,"record_offset":4832,"parent_record_offset":4672,"ancestor_record_offset":4176,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3174,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":4176,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3174,"record_offset":7112,"parent_record_offset":7084,"ancestor_record_offset":4176,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3174,"record_offset":4392,"parent_record_offset":4360,"ancestor_record_offset":4176,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3174,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":4176,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3174,"record_offset":4448,"parent_record_offset":4416,"ancestor_record_offset":4176,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3174,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":4176,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3174,"record_offset":5128,"parent_record_offset":5100,"ancestor_record_offset":4176,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3174,"record_offset":5376,"parent_record_offset":5128,"ancestor_record_offset":4176,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3174,"record_offset":5348,"parent_record_offset":5128,"ancestor_record_offset":4176,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3174,"record_offset":5180,"parent_record_offset":5128,"ancestor_record_offset":4176,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3174,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":4176,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3174,"record_offset":7768,"parent_record_offset":7556,"ancestor_record_offset":4176,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3174,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":4176,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3174,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":4176,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3174,"record_offset":7044,"parent_record_offset":7016,"ancestor_record_offset":4176,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3174,"record_offset":7624,"parent_record_offset":7556,"ancestor_record_offset":4176,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3174,"record_offset":7656,"parent_record_offset":7624,"ancestor_record_offset":4176,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3174,"record_offset":5988,"parent_record_offset":5948,"ancestor_record_offset":4176,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3174,"record_offset":6112,"parent_record_offset":6068,"ancestor_record_offset":4176,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3174,"record_offset":6276,"parent_record_offset":6068,"ancestor_record_offset":4176,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3174,"record_offset":6548,"parent_record_offset":6276,"ancestor_record_offset":4176,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":3174,"record_offset":7696,"parent_record_offset":7656,"ancestor_record_offset":4176,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3174,"record_offset":6140,"parent_record_offset":6112,"ancestor_record_offset":4176,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3174,"record_offset":6172,"parent_record_offset":6140,"ancestor_record_offset":4176,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3174,"record_offset":6068,"parent_record_offset":6028,"ancestor_record_offset":4176,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3174,"record_offset":6028,"parent_record_offset":5988,"ancestor_record_offset":4176,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03174.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::place_spires::place_roof_spires::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12D1C
// 首条位置: {"module":1894,"inlinee":77084}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1894,"record_offset":16152,"parent_record_offset":16096,"ancestor_record_offset":16096,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01894.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1894,"record_offset":16412,"parent_record_offset":16096,"ancestor_record_offset":16096,"inlinee":14702,"source_line":154,"archive":"_pdb/A/modules/01894.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1894,"record_offset":16180,"parent_record_offset":16096,"ancestor_record_offset":16096,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01894.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1894,"record_offset":16268,"parent_record_offset":16096,"ancestor_record_offset":16096,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01894.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1894,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":16096,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01894.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1894,"record_offset":16208,"parent_record_offset":16180,"ancestor_record_offset":16096,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01894.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1894,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":16096,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/01894.jsonl.gz"}
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
// 完整结构及原始类型记录: place_spires.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: place_spires.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: place_spires.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>

// ------------------------------------------------------------------
// 函数: system_roof::visual::place_spires::place_roof_spires
// 信息 ID: 772c4f7ceb6e7d26c77e9e67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cd898a0c41c18ea683f31317: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> @ 0x12bee9e -> 0x197e1b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x12d89e8 -> 0x197e1b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::place_spires::place_roof_spires::closure$0
// 信息 ID: 2130372e34083fc0e4d6605d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::visual::place_spires::place_roof_spires", "captures": "unknown"}
// [闭包环境类型引用] system_roof::visual::place_spires::place_roof_spires::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::place_spires::place_roof_spires::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>; module=1894, inline_record=16096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
