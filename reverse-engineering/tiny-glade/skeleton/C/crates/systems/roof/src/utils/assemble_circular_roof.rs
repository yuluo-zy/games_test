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
// Inline site 3786:2540 -> host 3786:268 / rva_282f9f0_len_1278; exact_symbolizer_function_frame
//   Function-frame RVA span [0x282fe48, 0x282fe7b), 9 instructions
//   Function-frame RVA span [0x282fe7e, 0x282feaa), 8 instructions
//   Function-frame RVA span [0x2830835, 0x28308ad), 20 instructions
//   Function-frame RVA span [0x2830943, 0x283096d), 12 instructions
//
// FUNCTION system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3786:268 -> rva_282f9f0_len_1278
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_282f9f0_len_1278: RVA 0x282f9f0, 4728 PDB bytes; 989 instructions
// Evidence: _assembly/roof-v1/C/pack-003/rva_282f9f0_len_1278.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3786:268: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>
// Frames: F1
// 0x14282f9f0  55                             pushq	%rbp
// 0x14282f9f1  4157                           pushq	%r15
// 0x14282f9f3  4156                           pushq	%r14
// 0x14282f9f5  4155                           pushq	%r13
// 0x14282f9f7  4154                           pushq	%r12
// 0x14282f9f9  56                             pushq	%rsi
// 0x14282f9fa  57                             pushq	%rdi
// 0x14282f9fb  53                             pushq	%rbx
// 0x14282f9fc  4881ecb8020000                 subq	$0x2b8, %rsp            # imm = 0x2B8
// 0x14282fa03  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14282fa0b  440f29bd20020000               movaps	%xmm15, 0x220(%rbp)
// 0x14282fa13  440f29b510020000               movaps	%xmm14, 0x210(%rbp)
// 0x14282fa1b  440f29ad00020000               movaps	%xmm13, 0x200(%rbp)
// 0x14282fa23  440f29a5f0010000               movaps	%xmm12, 0x1f0(%rbp)
// 0x14282fa2b  440f299de0010000               movaps	%xmm11, 0x1e0(%rbp)
// 0x14282fa33  440f2995d0010000               movaps	%xmm10, 0x1d0(%rbp)
// 0x14282fa3b  440f298dc0010000               movaps	%xmm9, 0x1c0(%rbp)
// 0x14282fa43  440f2985b0010000               movaps	%xmm8, 0x1b0(%rbp)
// 0x14282fa4b  0f29bda0010000                 movaps	%xmm7, 0x1a0(%rbp)
// 0x14282fa52  0f29b590010000                 movaps	%xmm6, 0x190(%rbp)
// 0x14282fa59  48c78588010000feffffff         movq	$-0x2, 0x188(%rbp)
// 0x14282fa64  4488cb                         movb	%r9b, %bl
// 0x14282fa67  4489852c010000                 movl	%r8d, 0x12c(%rbp)
// 0x14282fa6e  4989d6                         movq	%rdx, %r14
// 0x14282fa71  4889cf                         movq	%rcx, %rdi
// Frames: F2
// 0x14282fa74  48c7852001000000000000         movq	$0x0, 0x120(%rbp)
// 0x14282fa7f  488db540010000                 leaq	0x140(%rbp), %rsi
// Frames: F3
// 0x14282fa86  4889f1                         movq	%rsi, %rcx
// 0x14282fa89  e8921d1bfe                     callq	0x1409e1820 <.text+0x9e0820>
// 0x14282fa8e  4889f1                         movq	%rsi, %rcx
// 0x14282fa91  e84a1d1bfe                     callq	0x1409e17e0 <.text+0x9e07e0>
// 0x14282fa96  0f28f0                         movaps	%xmm0, %xmm6
// 0x14282fa99  0f28f9                         movaps	%xmm1, %xmm7
// 0x14282fa9c  488d4da8                       leaq	-0x58(%rbp), %rcx
// Frames: F4
// 0x14282faa0  4c89f2                         movq	%r14, %rdx
// 0x14282faa3  e8688b3bfe                     callq	0x140be8610 <.text+0xbe7610>
// 0x14282faa8  90                             nop
// Frames: F5
// 0x14282faa9  4c89f1                         movq	%r14, %rcx
// 0x14282faac  e88f923bfe                     callq	0x140be8d40 <.text+0xbe7d40>
// 0x14282fab1  90                             nop
// 0x14282fab2  f30f11442420                   movss	%xmm0, 0x20(%rsp)
// 0x14282fab8  488d4d30                       leaq	0x30(%rbp), %rcx
// 0x14282fabc  488d55a8                       leaq	-0x58(%rbp), %rdx
// 0x14282fac0  0f28d6                         movaps	%xmm6, %xmm2
// 0x14282fac3  0f28df                         movaps	%xmm7, %xmm3
// 0x14282fac6  e825813bfe                     callq	0x140be7bf0 <.text+0xbe6bf0>
// 0x14282facb  90                             nop
// Frames: F6 -> F7
// 0x14282facc  f30f107560                     movss	0x60(%rbp), %xmm6
// Frames: F8
// 0x14282fad1  0f28c6                         movaps	%xmm6, %xmm0
// 0x14282fad4  f30f5e05dc93d500               divss	0xd593dc(%rip), %xmm0   # 0x143588eb8
// Frames: F9 -> F10 -> F11
// 0x14282fadc  e89fc25e00                     callq	0x142e1bd80 <.text+0x2e1ad80>
// Frames: F12 -> F13
// 0x14282fae1  f30f5f05eb2e6f00               maxss	0x6f2eeb(%rip), %xmm0   # 0x142f229d4
// Frames: F14
// 0x14282fae9  f3440f2cf8                     cvttss2si	%xmm0, %r15d
// 0x14282faee  0f2e054bf46d00                 ucomiss	0x6df44b(%rip), %xmm0   # 0x142f0ef40
// 0x14282faf5  b8ffffff7f                     movl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
// 0x14282fafa  440f47f8                       cmoval	%eax, %r15d
// 0x14282fafe  31c0                           xorl	%eax, %eax
// 0x14282fb00  0f2ec0                         ucomiss	%xmm0, %xmm0
// 0x14282fb03  440f4af8                       cmovpl	%eax, %r15d
// 0x14282fb07  f30f5ef0                       divss	%xmm0, %xmm6
// 0x14282fb0b  f30f58056d1b6f00               addss	0x6f1b6d(%rip), %xmm0   # 0x142f21680
// 0x14282fb13  f30f118510010000               movss	%xmm0, 0x110(%rbp)
// 0x14282fb1b  f20f1007                       movsd	(%rdi), %xmm0
// 0x14282fb1f  0f294510                       movaps	%xmm0, 0x10(%rbp)
// 0x14282fb23  f30f10470c                     movss	0xc(%rdi), %xmm0
// 0x14282fb28  f30f118514010000               movss	%xmm0, 0x114(%rbp)
// 0x14282fb30  f30f59358493d500               mulss	0xd59384(%rip), %xmm6   # 0x143588ebc
// 0x14282fb38  f30f11b508010000               movss	%xmm6, 0x108(%rbp)
// 0x14282fb40  4489f8                         movl	%r15d, %eax
// 0x14282fb43  f7d8                           negl	%eax
// 0x14282fb45  898504010000                   movl	%eax, 0x104(%rbp)
// 0x14282fb4b  450f57f6                       xorps	%xmm14, %xmm14
// 0x14282fb4f  f3440f1005cc4e6d00             movss	0x6d4ecc(%rip), %xmm8   # 0x142f04a24
// 0x14282fb58  4531e4                         xorl	%r12d, %r12d
// 0x14282fb5b  31ff                           xorl	%edi, %edi
// Frames: F15 -> F16 -> F14
// 0x14282fb5d  4539e7                         cmpl	%r12d, %r15d
// 0x14282fb60  4489e0                         movl	%r12d, %eax
// 0x14282fb63  410f4fc7                       cmovgl	%r15d, %eax
// 0x14282fb67  41f7dc                         negl	%r12d
// 0x14282fb6a  f7d8                           negl	%eax
// 0x14282fb6c  0f1f4000                       nopl	(%rax)
// Frames: F17 -> F15 -> F16 -> F14
// 0x14282fb70  4439e0                         cmpl	%r12d, %eax
// Frames: F15 -> F16 -> F14
// 0x14282fb73  0f84ea0b0000                   je	0x142830763 <.text+0x282f763>
// Frames: F18
// 0x14282fb79  41ffcc                         decl	%r12d
// 0x14282fb7c  4439a504010000                 cmpl	%r12d, 0x104(%rbp)
// 0x14282fb83  74eb                           je	0x14282fb70 <.text+0x282eb70>
// Frames: F19
// 0x14282fb85  4489e0                         movl	%r12d, %eax
// 0x14282fb88  f7d0                           notl	%eax
// 0x14282fb8a  0f57c9                         xorps	%xmm1, %xmm1
// 0x14282fb8d  f30f2ac8                       cvtsi2ss	%eax, %xmm1
// 0x14282fb91  f30f5e8d10010000               divss	0x110(%rbp), %xmm1
// 0x14282fb99  f30f118de8000000               movss	%xmm1, 0xe8(%rbp)
// Frames: F20 -> F21 -> F22 -> F23 -> F24 -> F19
// 0x14282fba1  410f2ece                       ucomiss	%xmm14, %xmm1
// Frames: F24 -> F19
// 0x14282fba5  0f823e0d0000                   jb	0x1428308e9 <.text+0x282f8e9>
// 0x14282fbab  440f2ec1                       ucomiss	%xmm1, %xmm8
// 0x14282fbaf  0f82340d0000                   jb	0x1428308e9 <.text+0x282f8e9>
// Frames: F25 -> F26
// 0x14282fbb5  488d4d30                       leaq	0x30(%rbp), %rcx
// 0x14282fbb9  0f298da0000000                 movaps	%xmm1, 0xa0(%rbp)
// 0x14282fbc0  e8ebb8feff                     callq	0x14281b4b0 <.text+0x281a4b0>
// 0x14282fbc5  90                             nop
// 0x14282fbc6  488b5540                       movq	0x40(%rbp), %rdx
// Frames: F27 -> F28 -> F25 -> F26
// 0x14282fbca  89c1                           movl	%eax, %ecx
// Frames: F29 -> F30 -> F31 -> F28 -> F25 -> F26
// 0x14282fbcc  4839ca                         cmpq	%rcx, %rdx
// 0x14282fbcf  0f86ae0d0000                   jbe	0x142830983 <.text+0x282f983>
// Frames: F32 -> F28 -> F25 -> F26
// 0x14282fbd5  488d4101                       leaq	0x1(%rcx), %rax
// 0x14282fbd9  48899538010000                 movq	%rdx, 0x138(%rbp)
// Frames: F29 -> F30 -> F31 -> F28 -> F25 -> F26
// 0x14282fbe0  4839d0                         cmpq	%rdx, %rax
// 0x14282fbe3  0f83a90d0000                   jae	0x142830992 <.text+0x282f992>
// Frames: F25 -> F26
// 0x14282fbe9  4c8b6d38                       movq	0x38(%rbp), %r13
// Frames: F33 -> F34 -> F28 -> F25 -> F26
// 0x14282fbed  450f28d0                       movaps	%xmm8, %xmm10
// 0x14282fbf1  f3440f5cd0                     subss	%xmm0, %xmm10
// Frames: F35 -> F36 -> F33 -> F34 -> F28 -> F25 -> F26
// 0x14282fbf6  f3450f5954cd00                 mulss	(%r13,%rcx,8), %xmm10
// 0x14282fbfd  f3410f1074cd08                 movss	0x8(%r13,%rcx,8), %xmm6
// 0x14282fc04  f30f59f0                       mulss	%xmm0, %xmm6
// Frames: F25 -> F37
// 0x14282fc08  488d4d30                       leaq	0x30(%rbp), %rcx
// 0x14282fc0c  0f288da0000000                 movaps	0xa0(%rbp), %xmm1
// 0x14282fc13  e898b8feff                     callq	0x14281b4b0 <.text+0x281a4b0>
// 0x14282fc18  90                             nop
// Frames: F27 -> F28 -> F25 -> F37
// 0x14282fc19  89c1                           movl	%eax, %ecx
// 0x14282fc1b  488b9538010000                 movq	0x138(%rbp), %rdx
// Frames: F29 -> F30 -> F31 -> F28 -> F25 -> F37
// 0x14282fc22  4839ca                         cmpq	%rcx, %rdx
// 0x14282fc25  0f86800d0000                   jbe	0x1428309ab <.text+0x282f9ab>
// Frames: F32 -> F28 -> F25 -> F37
// 0x14282fc2b  488d4101                       leaq	0x1(%rcx), %rax
// Frames: F29 -> F30 -> F31 -> F28 -> F25 -> F37
// 0x14282fc2f  4839d0                         cmpq	%rdx, %rax
// 0x14282fc32  0f83820d0000                   jae	0x1428309ba <.text+0x282f9ba>
// Frames: F25 -> F37
// 0x14282fc38  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F28 -> F25 -> F37
// 0x14282fc3c  f3410f107ccd04                 movss	0x4(%r13,%rcx,8), %xmm7
// 0x14282fc43  f3450f1064cd0c                 movss	0xc(%r13,%rcx,8), %xmm12
// Frames: F38
// 0x14282fc4a  4c89f1                         movq	%r14, %rcx
// 0x14282fc4d  e8de883bfe                     callq	0x140be8530 <.text+0xbe7530>
// 0x14282fc52  90                             nop
// Frames: F33 -> F34 -> F28 -> F25 -> F37
// 0x14282fc53  410f28d0                       movaps	%xmm8, %xmm2
// 0x14282fc57  f3410f5cd3                     subss	%xmm11, %xmm2
// Frames: F35 -> F39 -> F33 -> F34 -> F28 -> F25 -> F37
// 0x14282fc5c  f30f59d7                       mulss	%xmm7, %xmm2
// 0x14282fc60  f3450f59e3                     mulss	%xmm11, %xmm12
// Frames: F40 -> F41 -> F33 -> F34 -> F28 -> F25 -> F37
// 0x14282fc65  f3440f58e2                     addss	%xmm2, %xmm12
// Frames: F35 -> F36 -> F38
// 0x14282fc6a  f30f1295a0000000               movsldup	0xa0(%rbp), %xmm2       # xmm2 = mem[0,0,2,2]
// 0x14282fc72  0f14c1                         unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
// 0x14282fc75  0f59c2                         mulps	%xmm2, %xmm0
// Frames: F40 -> F42 -> F38
// 0x14282fc78  0f584510                       addps	0x10(%rbp), %xmm0
// Frames: F38
// 0x14282fc7c  0f138540010000                 movlps	%xmm0, 0x140(%rbp)
// 0x14282fc83  488d4d78                       leaq	0x78(%rbp), %rcx
// 0x14282fc87  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x14282fc8e  410f28d4                       movaps	%xmm12, %xmm2
// 0x14282fc92  e8e9085cfe                     callq	0x140df0580 <.text+0xdef580>
// 0x14282fc97  90                             nop
// Frames: F43
// 0x14282fc98  f3440f58d6                     addss	%xmm6, %xmm10
// 0x14282fc9d  f30f108514010000               movss	0x114(%rbp), %xmm0
// 0x14282fca5  f30f11442420                   movss	%xmm0, 0x20(%rsp)
// 0x14282fcab  0f57c9                         xorps	%xmm1, %xmm1
// 0x14282fcae  0f57d2                         xorps	%xmm2, %xmm2
// 0x14282fcb1  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x14282fcb8  410f28da                       movaps	%xmm10, %xmm3
// 0x14282fcbc  e81fb65bfe                     callq	0x140deb2e0 <.text+0xdea2e0>
// 0x14282fcc1  90                             nop
// Frames: F44
// 0x14282fcc2  f30f10059e087c00               movss	0x7c089e(%rip), %xmm0   # 0x142ff0568
// 0x14282fcca  f30f100dee706f00               movss	0x6f70ee(%rip), %xmm1   # 0x142f26dc0
// 0x14282fcd2  410f28d2                       movaps	%xmm10, %xmm2
// 0x14282fcd6  e815415bfe                     callq	0x140de3df0 <.text+0xde2df0>
// 0x14282fcdb  90                             nop
// Frames: F45 -> F44
// 0x14282fcdc  0f57c9                         xorps	%xmm1, %xmm1
// 0x14282fcdf  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F46 -> F44
// 0x14282fce3  410f28f0                       movaps	%xmm8, %xmm6
// 0x14282fce7  f30f5df1                       minss	%xmm1, %xmm6
// Frames: F35 -> F47 -> F48
// 0x14282fceb  440f28d6                       movaps	%xmm6, %xmm10
// 0x14282fcef  f3440f59157c696d00             mulss	0x6d697c(%rip), %xmm10  # 0x142f06674
// Frames: F49
// 0x14282fcf8  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x14282fcff  e86cc25bfe                     callq	0x140debf70 <.text+0xdeaf70>
// 0x14282fd04  90                             nop
// Frames: F50
// 0x14282fd05  440f28d8                       movaps	%xmm0, %xmm11
// 0x14282fd09  488d8d20010000                 leaq	0x120(%rbp), %rcx
// 0x14282fd10  e85beb5cfe                     callq	0x140dfe870 <.text+0xdfd870>
// 0x14282fd15  90                             nop
// Frames: F51
// 0x14282fd16  440f28e8                       movaps	%xmm0, %xmm13
// 0x14282fd1a  410f28c0                       movaps	%xmm8, %xmm0
// 0x14282fd1e  f30f5cc6                       subss	%xmm6, %xmm0
// 0x14282fd22  f3440f58d0                     addss	%xmm0, %xmm10
// 0x14282fd27  410f28ca                       movaps	%xmm10, %xmm1
// 0x14282fd2b  f30f590d41696d00               mulss	0x6d6941(%rip), %xmm1   # 0x142f06674
// Frames: F52
// 0x14282fd33  410f28c3                       movaps	%xmm11, %xmm0
// 0x14282fd37  f30f5ec1                       divss	%xmm1, %xmm0
// Frames: F9 -> F10 -> F53
// 0x14282fd3b  e840c05e00                     callq	0x142e1bd80 <.text+0x2e1ad80>
// Frames: F12 -> F54
// 0x14282fd40  f30f5f05d88b6f00               maxss	0x6f8bd8(%rip), %xmm0   # 0x142f28920
// Frames: F52
// 0x14282fd48  f30f2cf0                       cvttss2si	%xmm0, %esi
// 0x14282fd4c  0f2e05edf16d00                 ucomiss	0x6df1ed(%rip), %xmm0   # 0x142f0ef40
// 0x14282fd53  b8ffffff7f                     movl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
// 0x14282fd58  0f47f0                         cmoval	%eax, %esi
// 0x14282fd5b  0f2ec0                         ucomiss	%xmm0, %xmm0
// 0x14282fd5e  b800000000                     movl	$0x0, %eax
// 0x14282fd63  0f4af0                         cmovpl	%eax, %esi
// Frames: F50
// 0x14282fd66  f3440f592d6d2c6f00             mulss	0x6f2c6d(%rip), %xmm13  # 0x142f229dc
// Frames: F55 -> F56 -> F57 -> F58 -> F50
// 0x14282fd6f  410f28c5                       movaps	%xmm13, %xmm0
// 0x14282fd73  e878c15e00                     callq	0x142e1bef0 <.text+0x2e1aef0>
// 0x14282fd78  440f28e0                       movaps	%xmm0, %xmm12
// Frames: F59 -> F56 -> F57 -> F58 -> F50
// 0x14282fd7c  410f28c5                       movaps	%xmm13, %xmm0
// 0x14282fd80  e81bc05e00                     callq	0x142e1bda0 <.text+0x2e1ada0>
// 0x14282fd85  440f28e8                       movaps	%xmm0, %xmm13
// Frames: F60
// 0x14282fd89  4863d6                         movslq	%esi, %rdx
// 0x14282fd8c  48ffc2                         incq	%rdx
// Frames: F61
// 0x14282fd8f  f3450f59d6                     mulss	%xmm14, %xmm10
// 0x14282fd94  f3450f5ed3                     divss	%xmm11, %xmm10
// Frames: F62
// 0x14282fd99  488d059095d500                 leaq	0xd59590(%rip), %rax    # 0x143589330
// 0x14282fda0  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14282fda5  488d8de8000000                 leaq	0xe8(%rbp), %rcx
// 0x14282fdac  410f28d2                       movaps	%xmm10, %xmm2
// 0x14282fdb0  4c8d8d20010000                 leaq	0x120(%rbp), %r9
// 0x14282fdb7  e8a4405bfe                     callq	0x140de3e60 <.text+0xde2e60>
// 0x14282fdbc  90                             nop
// Frames: F63
// 0x14282fdbd  4489bd18010000                 movl	%r15d, 0x118(%rbp)
// 0x14282fdc4  4c8bbdf8000000                 movq	0xf8(%rbp), %r15
// 0x14282fdcb  4983ff01                       cmpq	$0x1, %r15
// 0x14282fdcf  0f86500b0000                   jbe	0x142830925 <.text+0x282f925>
// 0x14282fdd5  4c89b588000000                 movq	%r14, 0x88(%rbp)
// 0x14282fddc  4489e0                         movl	%r12d, %eax
// 0x14282fddf  f7d8                           negl	%eax
// 0x14282fde1  89851c010000                   movl	%eax, 0x11c(%rbp)
// 0x14282fde7  410f28c5                       movaps	%xmm13, %xmm0
// 0x14282fdeb  410f14c4                       unpcklps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0],xmm0[1],xmm12[1]
// 0x14282fdef  440f572539e46c00               xorps	0x6ce439(%rip), %xmm12  # 0x142efe230
// 0x14282fdf7  450f14e5                       unpcklps	%xmm13, %xmm12          # xmm12 = xmm12[0],xmm13[0],xmm12[1],xmm13[1]
// 0x14282fdfb  410f16c4                       movlhps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0]
// 0x14282fdff  0f294520                       movaps	%xmm0, 0x20(%rbp)
// 0x14282fe03  4183fcff                       cmpl	$-0x1, %r12d
// 0x14282fe07  0f94c0                         sete	%al
// 0x14282fe0a  889d87010000                   movb	%bl, 0x187(%rbp)
// 0x14282fe10  08d8                           orb	%bl, %al
// 0x14282fe12  0fb6c0                         movzbl	%al, %eax
// 0x14282fe15  ffc0                           incl	%eax
// 0x14282fe17  89850c010000                   movl	%eax, 0x10c(%rbp)
// 0x14282fe1d  4531f6                         xorl	%r14d, %r14d
// 0x14282fe20  eb19                           jmp	0x14282fe3b <.text+0x282ee3b>
// 0x14282fe22  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F64
// 0x14282fe30  ffc7                           incl	%edi
// 0x14282fe32  f3440f1005e94b6d00             movss	0x6d4be9(%rip), %xmm8   # 0x142f04a24
// Frames: F65 -> F15 -> F16 -> F66 -> F67
// 0x14282fe3b  498d47ff                       leaq	-0x1(%r15), %rax
// 0x14282fe3f  4939c6                         cmpq	%rax, %r14
// Frames: F15 -> F16 -> F66 -> F67
// 0x14282fe42  0f83d8080000                   jae	0x142830720 <.text+0x282f720>
// Frames: F29 -> F30 -> F31 -> F68 -> F69 -> F70 -> F66 -> F67
// 0x14282fe48  4d39fe                         cmpq	%r15, %r14
// 0x14282fe4b  0f83f20a0000                   jae	0x142830943 <.text+0x282f943>
// Frames: F68 -> F69 -> F70 -> F66 -> F67
// 0x14282fe51  488b85f0000000                 movq	0xf0(%rbp), %rax
// 0x14282fe58  f3460f101cb0                   movss	(%rax,%r14,4), %xmm11
// 0x14282fe5e  f3440f119d30010000             movss	%xmm11, 0x130(%rbp)
// Frames: F20 -> F21 -> F22 -> F23 -> F24 -> F68 -> F69 -> F70 -> F66 -> F67
// 0x14282fe67  450f2ede                       ucomiss	%xmm14, %xmm11
// Frames: F24 -> F68 -> F69 -> F70 -> F66 -> F67
// 0x14282fe6b  0f82c4090000                   jb	0x142830835 <.text+0x282f835>
// 0x14282fe71  450f2ec3                       ucomiss	%xmm11, %xmm8
// 0x14282fe75  0f82ba090000                   jb	0x142830835 <.text+0x282f835>
// Frames: F66 -> F67
// 0x14282fe7b  49ffc6                         incq	%r14
// Frames: F29 -> F30 -> F31 -> F68 -> F69 -> F70 -> F66 -> F67
// 0x14282fe7e  4d39fe                         cmpq	%r15, %r14
// 0x14282fe81  0f83d10a0000                   jae	0x142830958 <.text+0x282f958>
// Frames: F68 -> F69 -> F70 -> F66 -> F67
// 0x14282fe87  f3460f1024b0                   movss	(%rax,%r14,4), %xmm12
// 0x14282fe8d  f3440f11a530010000             movss	%xmm12, 0x130(%rbp)
// Frames: F20 -> F21 -> F22 -> F23 -> F24 -> F68 -> F69 -> F70 -> F66 -> F67
// 0x14282fe96  450f2ee6                       ucomiss	%xmm14, %xmm12
// Frames: F24 -> F68 -> F69 -> F70 -> F66 -> F67
// 0x14282fe9a  0f82d1090000                   jb	0x142830871 <.text+0x282f871>
// 0x14282fea0  450f2ec4                       ucomiss	%xmm12, %xmm8
// 0x14282fea4  0f82c7090000                   jb	0x142830871 <.text+0x282f871>
// Frames: F71
// 0x14282feaa  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x14282feb1  410f28cb                       movaps	%xmm11, %xmm1
// 0x14282feb5  e846b45bfe                     callq	0x140deb300 <.text+0xdea300>
// 0x14282feba  90                             nop
// Frames: F72
// 0x14282febb  440f28d0                       movaps	%xmm0, %xmm10
// 0x14282febf  440f28e9                       movaps	%xmm1, %xmm13
// 0x14282fec3  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x14282feca  410f28cc                       movaps	%xmm12, %xmm1
// 0x14282fece  e82db45bfe                     callq	0x140deb300 <.text+0xdea300>
// 0x14282fed3  90                             nop
// Frames: F73 -> F74 -> F72
// 0x14282fed4  450fc6d500                     shufps	$0x0, %xmm13, %xmm10    # xmm10 = xmm10[0,0],xmm13[0,0]
// 0x14282fed9  0f285520                       movaps	0x20(%rbp), %xmm2
// 0x14282fedd  440f59d2                       mulps	%xmm2, %xmm10
// 0x14282fee1  410f28f2                       movaps	%xmm10, %xmm6
// 0x14282fee5  66410f15f2                     unpckhpd	%xmm10, %xmm6           # xmm6 = xmm6[1],xmm10[1]
// 0x14282feea  410f58f2                       addps	%xmm10, %xmm6
// 0x14282feee  0fc6c100                       shufps	$0x0, %xmm1, %xmm0      # xmm0 = xmm0[0,0],xmm1[0,0]
// Frames: F75 -> F76 -> F72
// 0x14282fef2  0f59c2                         mulps	%xmm2, %xmm0
// Frames: F77 -> F78 -> F72
// 0x14282fef5  440f28c0                       movaps	%xmm0, %xmm8
// 0x14282fef9  66440f15c0                     unpckhpd	%xmm0, %xmm8            # xmm8 = xmm8[1],xmm0[1]
// Frames: F79 -> F80 -> F72
// 0x14282fefe  440f58c0                       addps	%xmm0, %xmm8
// Frames: F40 -> F42 -> F81
// 0x14282ff02  0f28c6                         movaps	%xmm6, %xmm0
// 0x14282ff05  410f58c0                       addps	%xmm8, %xmm0
// Frames: F82 -> F83 -> F81
// 0x14282ff09  0f5905304b6d00                 mulps	0x6d4b30(%rip), %xmm0   # 0x142f04a40
// Frames: F81
// 0x14282ff10  0f138530010000                 movlps	%xmm0, 0x130(%rbp)
// Frames: F84
// 0x14282ff17  488d8d40010000                 leaq	0x140(%rbp), %rcx
// 0x14282ff1e  488d9530010000                 leaq	0x130(%rbp), %rdx
// 0x14282ff25  e836065cfe                     callq	0x140df0560 <.text+0xdef560>
// 0x14282ff2a  90                             nop
// 0x14282ff2b  f20f104578                     movsd	0x78(%rbp), %xmm0
// 0x14282ff30  0f2945e0                       movaps	%xmm0, -0x20(%rbp)
// Frames: F85 -> F84
// 0x14282ff34  f20f10bd40010000               movsd	0x140(%rbp), %xmm7
// Frames: F40 -> F85 -> F84
// 0x14282ff3c  f30f108580000000               movss	0x80(%rbp), %xmm0
// 0x14282ff44  0f294500                       movaps	%xmm0, (%rbp)
// 0x14282ff48  f30f108548010000               movss	0x148(%rbp), %xmm0
// 0x14282ff50  0f2945f0                       movaps	%xmm0, -0x10(%rbp)
// Frames: F86
// 0x14282ff54  488d8d20010000                 leaq	0x120(%rbp), %rcx
// 0x14282ff5b  e810e95cfe                     callq	0x140dfe870 <.text+0xdfd870>
// 0x14282ff60  90                             nop
// 0x14282ff61  440f28d0                       movaps	%xmm0, %xmm10
// Frames: F87 -> F47 -> F86
// 0x14282ff65  f3440f100db64a6d00             movss	0x6d4ab6(%rip), %xmm9   # 0x142f04a24
// 0x14282ff6e  f3440f5cc8                     subss	%xmm0, %xmm9
// Frames: F35 -> F47 -> F86
// 0x14282ff73  f3440f590d54b17400             mulss	0x74b154(%rip), %xmm9   # 0x142f7b0d0
// 0x14282ff7c  f3440f5915938a6f00             mulss	0x6f8a93(%rip), %xmm10  # 0x142f28a18
// Frames: F88 -> F89 -> F90
// 0x14282ff85  488d4d30                       leaq	0x30(%rbp), %rcx
// 0x14282ff89  0f288da0000000                 movaps	0xa0(%rbp), %xmm1
// 0x14282ff90  e81bb5feff                     callq	0x14281b4b0 <.text+0x281a4b0>
// 0x14282ff95  90                             nop
// Frames: F27 -> F88 -> F89 -> F90
// 0x14282ff96  89c0                           movl	%eax, %eax
// Frames: F91 -> F89 -> F90
// 0x14282ff98  488d4801                       leaq	0x1(%rax), %rcx
// Frames: F29 -> F30 -> F31 -> F92 -> F90
// 0x14282ff9c  483b8d38010000                 cmpq	0x138(%rbp), %rcx
// 0x14282ffa3  0f83c4090000                   jae	0x14283096d <.text+0x282f96d>
// Frames: F92 -> F90
// 0x14282ffa9  f3410f104cc508                 movss	0x8(%r13,%rax,8), %xmm1
// 0x14282ffb0  f3410f1054c50c                 movss	0xc(%r13,%rax,8), %xmm2
// Frames: F87 -> F93 -> F92 -> F90
// 0x14282ffb7  f3410f5c4cc500                 subss	(%r13,%rax,8), %xmm1
// Frames: F87 -> F94 -> F92 -> F90
// 0x14282ffbe  f3410f5c54c504                 subss	0x4(%r13,%rax,8), %xmm2
// Frames: F95 -> F96 -> F97 -> F98 -> F99 -> F100 -> F90
// 0x14282ffc5  0f28c1                         movaps	%xmm1, %xmm0
// 0x14282ffc8  f30f59c1                       mulss	%xmm1, %xmm0
// 0x14282ffcc  0f28da                         movaps	%xmm2, %xmm3
// 0x14282ffcf  f30f59da                       mulss	%xmm2, %xmm3
// 0x14282ffd3  f30f58d8                       addss	%xmm0, %xmm3
// Frames: F101 -> F102 -> F103 -> F96 -> F97 -> F98 -> F99 -> F100 -> F90
// 0x14282ffd7  f30f51db                       sqrtss	%xmm3, %xmm3
// 0x14282ffdb  f3440f103d404a6d00             movss	0x6d4a40(%rip), %xmm15  # 0x142f04a24
// Frames: F104 -> F97 -> F98 -> F99 -> F100 -> F90
// 0x14282ffe4  410f28c7                       movaps	%xmm15, %xmm0
// 0x14282ffe8  f30f5ec3                       divss	%xmm3, %xmm0
// Frames: F35 -> F105 -> F98 -> F99 -> F100 -> F90
// 0x14282ffec  f30f59c8                       mulss	%xmm0, %xmm1
// Frames: F35 -> F106 -> F98 -> F99 -> F100 -> F90
// 0x14282fff0  f30f59c2                       mulss	%xmm2, %xmm0
// Frames: F107
// 0x14282fff4  f30f5ec1                       divss	%xmm1, %xmm0
// Frames: F108 -> F107
// 0x14282fff8  e853bd5e00                     callq	0x142e1bd50 <.text+0x2e1ad50>
// Frames: F109
// 0x14282fffd  f3450f58dc                     addss	%xmm12, %xmm11
// 0x142830002  f3440f591d69666d00             mulss	0x6d6669(%rip), %xmm11  # 0x142f06674
// 0x14283000b  f3440f119d00010000             movss	%xmm11, 0x100(%rbp)
// Frames: F20 -> F21 -> F22 -> F23 -> F24 -> F109
// 0x142830014  450f2ede                       ucomiss	%xmm14, %xmm11
// Frames: F24 -> F109
// 0x142830018  0f828f080000                   jb	0x1428308ad <.text+0x282f8ad>
// 0x14283001e  450f2efb                       ucomiss	%xmm11, %xmm15
// 0x142830022  0f8285080000                   jb	0x1428308ad <.text+0x282f8ad>
// Frames: F110
// 0x142830028  440f28e8                       movaps	%xmm0, %xmm13
// 0x14283002c  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x142830033  410f28cb                       movaps	%xmm11, %xmm1
// 0x142830037  e8e4b55bfe                     callq	0x140deb620 <.text+0xdea620>
// 0x14283003c  90                             nop
// Frames: F73 -> F74 -> F110
// 0x14283003d  0fc6c100                       shufps	$0x0, %xmm1, %xmm0      # xmm0 = xmm0[0,0],xmm1[0,0]
// Frames: F75 -> F76 -> F110
// 0x142830041  0f594520                       mulps	0x20(%rbp), %xmm0
// Frames: F110
// 0x142830045  0f28c8                         movaps	%xmm0, %xmm1
// 0x142830048  660f15c8                       unpckhpd	%xmm0, %xmm1            # xmm1 = xmm1[1],xmm0[1]
// 0x14283004c  0f58c8                         addps	%xmm0, %xmm1
// 0x14283004f  0f138d40010000                 movlps	%xmm1, 0x140(%rbp)
// 0x142830056  488d4d6c                       leaq	0x6c(%rbp), %rcx
// 0x14283005a  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x142830061  e8fa045cfe                     callq	0x140df0560 <.text+0xdef560>
// 0x142830066  90                             nop
// Frames: F111
// 0x142830067  0f29bdb0000000                 movaps	%xmm7, 0xb0(%rbp)
// 0x14283006e  f3440f101d6192d500             movss	0xd59261(%rip), %xmm11  # 0x1435892d8
// 0x142830077  f3450f5cdd                     subss	%xmm13, %xmm11
// 0x14283007c  f3440f106d6c                   movss	0x6c(%rbp), %xmm13
// 0x142830082  f2440f107d70                   movsd	0x70(%rbp), %xmm15
// Frames: F112 -> F111
// 0x142830088  410f28ff                       movaps	%xmm15, %xmm7
// 0x14283008c  0f57c0                         xorps	%xmm0, %xmm0
// 0x14283008f  0f59f8                         mulps	%xmm0, %xmm7
// 0x142830092  0f28c7                         movaps	%xmm7, %xmm0
// 0x142830095  0fc6c7e1                       shufps	$0xe1, %xmm7, %xmm0     # xmm0 = xmm0[1,0],xmm7[2,3]
// 0x142830099  450f28f7                       movaps	%xmm15, %xmm14
// 0x14283009d  f3450f10f5                     movss	%xmm13, %xmm14          # xmm14 = xmm13[0],xmm14[1,2,3]
// 0x1428300a2  440f5935666d6f00               mulps	0x6f6d66(%rip), %xmm14  # 0x142f26e10
// 0x1428300aa  440f58f0                       addps	%xmm0, %xmm14
// Frames: F113 -> F114
// 0x1428300ae  f3440f591d4df06d00             mulss	0x6df04d(%rip), %xmm11  # 0x142f0f104
// Frames: F55 -> F56 -> F57 -> F113 -> F114
// 0x1428300b7  410f28c3                       movaps	%xmm11, %xmm0
// 0x1428300bb  e830be5e00                     callq	0x142e1bef0 <.text+0x2e1aef0>
// 0x1428300c0  440f28e0                       movaps	%xmm0, %xmm12
// Frames: F59 -> F56 -> F57 -> F113 -> F114
// 0x1428300c4  410f28c3                       movaps	%xmm11, %xmm0
// 0x1428300c8  e8d3bc5e00                     callq	0x142e1bda0 <.text+0x2e1ada0>
// Frames: F115 -> F116 -> F117 -> F118 -> F119 -> F114
// 0x1428300cd  410f28e5                       movaps	%xmm13, %xmm4
// 0x1428300d1  410f16e7                       movlhps	%xmm15, %xmm4           # xmm4 = xmm4[0],xmm15[0]
// 0x1428300d5  0f28cc                         movaps	%xmm4, %xmm1
// 0x1428300d8  410fc6cfd8                     shufps	$0xd8, %xmm15, %xmm1    # xmm1 = xmm1[0,2],xmm15[1,3]
// 0x1428300dd  410fc6e758                     shufps	$0x58, %xmm15, %xmm4    # xmm4 = xmm4[0,2],xmm15[1,1]
// Frames: F77 -> F120 -> F121 -> F118 -> F119 -> F114
// 0x1428300e2  0f28d9                         movaps	%xmm1, %xmm3
// 0x1428300e5  0fc6d9d2                       shufps	$0xd2, %xmm1, %xmm3     # xmm3 = xmm3[2,0],xmm1[1,3]
// Frames: F122 -> F111
// 0x1428300e9  f3440f5cef                     subss	%xmm7, %xmm13
// 0x1428300ee  450f28fe                       movaps	%xmm14, %xmm15
// 0x1428300f2  450f57f6                       xorps	%xmm14, %xmm14
// Frames: F123 -> F124 -> F114
// 0x1428300f6  450fc6e400                     shufps	$0x0, %xmm12, %xmm12    # xmm12 = xmm12[0,0,0,0]
// 0x1428300fb  410f28ed                       movaps	%xmm13, %xmm5
// 0x1428300ff  410fc6eff4                     shufps	$0xf4, %xmm15, %xmm5    # xmm5 = xmm5[0,1],xmm15[3,3]
// 0x142830104  410f28d7                       movaps	%xmm15, %xmm2
// 0x142830108  0fc6d581                       shufps	$0x81, %xmm5, %xmm2     # xmm2 = xmm2[1,0],xmm5[0,2]
// 0x14283010c  410f59d4                       mulps	%xmm12, %xmm2
// 0x142830110  0fc6c2e4                       shufps	$0xe4, %xmm2, %xmm0     # xmm0 = xmm0[0,1],xmm2[2,3]
// 0x142830114  440f28da                       movaps	%xmm2, %xmm11
// 0x142830118  440fc6d824                     shufps	$0x24, %xmm0, %xmm11    # xmm11 = xmm11[0,1],xmm0[2,0]
// Frames: F75 -> F125 -> F126 -> F127 -> F118 -> F119 -> F114
// 0x14283011d  410f28eb                       movaps	%xmm11, %xmm5
// 0x142830121  410f59eb                       mulps	%xmm11, %xmm5
// Frames: F77 -> F128 -> F127 -> F118 -> F119 -> F114
// 0x142830125  0f28c5                         movaps	%xmm5, %xmm0
// 0x142830128  0fc6c5aa                       shufps	$0xaa, %xmm5, %xmm0     # xmm0 = xmm0[2,2],xmm5[2,2]
// 0x14283012c  f3440f16e5                     movshdup	%xmm5, %xmm12           # xmm12 = xmm5[1,1,3,3]
// 0x142830131  f3440f58e5                     addss	%xmm5, %xmm12
// 0x142830136  450fc6e400                     shufps	$0x0, %xmm12, %xmm12    # xmm12 = xmm12[0,0,0,0]
// 0x14283013b  440f58e0                       addps	%xmm0, %xmm12
// Frames: F75 -> F129 -> F118 -> F119 -> F114
// 0x14283013f  0fc6edff                       shufps	$0xff, %xmm5, %xmm5     # xmm5 = xmm5[3,3,3,3]
// Frames: F130 -> F129 -> F118 -> F119 -> F114
// 0x142830143  410f5cec                       subps	%xmm12, %xmm5
// Frames: F75 -> F129 -> F118 -> F119 -> F114
// 0x142830147  0f59ec                         mulps	%xmm4, %xmm5
// Frames: F75 -> F125 -> F126 -> F131 -> F118 -> F119 -> F114
// 0x14283014a  0f28c1                         movaps	%xmm1, %xmm0
// 0x14283014d  0f59c2                         mulps	%xmm2, %xmm0
// Frames: F132 -> F133 -> F126 -> F131 -> F118 -> F119 -> F114
// 0x142830150  f30f16e0                       movshdup	%xmm0, %xmm4            # xmm4 = xmm0[1,1,3,3]
// 0x142830154  f30f58e0                       addss	%xmm0, %xmm4
// 0x142830158  0f12c0                         movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
// 0x14283015b  f30f58c4                       addss	%xmm4, %xmm0
// Frames: F75 -> F131 -> F118 -> F119 -> F114
// 0x14283015f  f30f58c0                       addss	%xmm0, %xmm0
// 0x142830163  0fc6c000                       shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
// 0x142830167  410f59c3                       mulps	%xmm11, %xmm0
// Frames: F79 -> F134 -> F118 -> F119 -> F114
// 0x14283016b  0f58c5                         addps	%xmm5, %xmm0
// Frames: F75 -> F135 -> F121 -> F118 -> F119 -> F114
// 0x14283016e  0f59da                         mulps	%xmm2, %xmm3
// Frames: F77 -> F136 -> F121 -> F118 -> F119 -> F114
// 0x142830171  0fc6d2d2                       shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
// Frames: F75 -> F137 -> F121 -> F118 -> F119 -> F114
// 0x142830175  0f59d1                         mulps	%xmm1, %xmm2
// Frames: F130 -> F138 -> F121 -> F118 -> F119 -> F114
// 0x142830178  0f5cd3                         subps	%xmm3, %xmm2
// Frames: F77 -> F139 -> F121 -> F118 -> F119 -> F114
// 0x14283017b  0fc6d2d2                       shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
// Frames: F75 -> F121 -> F118 -> F119 -> F114
// 0x14283017f  450f58db                       addps	%xmm11, %xmm11
// 0x142830183  450fc6dbff                     shufps	$0xff, %xmm11, %xmm11   # xmm11 = xmm11[3,3,3,3]
// 0x142830188  440f59da                       mulps	%xmm2, %xmm11
// Frames: F79 -> F140 -> F118 -> F119 -> F114
// 0x14283018c  440f58d8                       addps	%xmm0, %xmm11
// Frames: F141 -> F142
// 0x142830190  410f28c3                       movaps	%xmm11, %xmm0
// 0x142830194  66410f15c3                     unpckhpd	%xmm11, %xmm0           # xmm0 = xmm0[1],xmm11[1]
// 0x142830199  f3410f16d3                     movshdup	%xmm11, %xmm2           # xmm2 = xmm11[1,1,3,3]
// 0x14283019e  410f28dd                       movaps	%xmm13, %xmm3
// 0x1428301a2  410f16df                       movlhps	%xmm15, %xmm3           # xmm3 = xmm3[0],xmm15[0]
// 0x1428301a6  410fc6dfe2                     shufps	$0xe2, %xmm15, %xmm3    # xmm3 = xmm3[2,0],xmm15[2,3]
// 0x1428301ab  410f28cb                       movaps	%xmm11, %xmm1
// 0x1428301af  410fc6cbe2                     shufps	$0xe2, %xmm11, %xmm1    # xmm1 = xmm1[2,0],xmm11[2,3]
// 0x1428301b4  0f59cb                         mulps	%xmm3, %xmm1
// 0x1428301b7  410f28e7                       movaps	%xmm15, %xmm4
// 0x1428301bb  f3410f10e5                     movss	%xmm13, %xmm4           # xmm4 = xmm13[0],xmm4[1,2,3]
// 0x1428301c0  410f28db                       movaps	%xmm11, %xmm3
// 0x1428301c4  410fc6dbe9                     shufps	$0xe9, %xmm11, %xmm3    # xmm3 = xmm3[1,2],xmm11[2,3]
// 0x1428301c9  0f59e3                         mulps	%xmm3, %xmm4
// 0x1428301cc  0f5ccc                         subps	%xmm4, %xmm1
// Frames: F122 -> F142
// 0x1428301cf  410f28ef                       movaps	%xmm15, %xmm5
// 0x1428301d3  410f59eb                       mulps	%xmm11, %xmm5
// 0x1428301d7  f30f16e5                       movshdup	%xmm5, %xmm4            # xmm4 = xmm5[1,1,3,3]
// 0x1428301db  f30f5ce5                       subss	%xmm5, %xmm4
// Frames: F143 -> F144 -> F145
// 0x1428301df  440f2ef4                       ucomiss	%xmm4, %xmm14
// 0x1428301e3  f3450f16e7                     movshdup	%xmm15, %xmm12          # xmm12 = xmm15[1,1,3,3]
// 0x1428301e8  7366                           jae	0x142830250 <.text+0x282f250>
// Frames: F146 -> F144 -> F145
// 0x1428301ea  f3410f58d4                     addss	%xmm12, %xmm2
// Frames: F147 -> F144 -> F145
// 0x1428301ef  f30f102d2d486d00               movss	0x6d482d(%rip), %xmm5   # 0x142f04a24
// 0x1428301f7  f30f58e5                       addss	%xmm5, %xmm4
// Frames: F148 -> F144 -> F145
// 0x1428301fb  440f2ef2                       ucomiss	%xmm2, %xmm14
// 0x1428301ff  0f83bb000000                   jae	0x1428302c0 <.text+0x282f2c0>
// Frames: F149 -> F144 -> F145
// 0x142830205  f30f58d4                       addss	%xmm4, %xmm2
// Frames: F150 -> F144 -> F145
// 0x142830209  f30f16d9                       movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
// 0x14283020d  410f14dd                       unpcklps	%xmm13, %xmm3           # xmm3 = xmm3[0],xmm13[0],xmm3[1],xmm13[1]
// Frames: F101 -> F102 -> F151 -> F152 -> F144 -> F145
// 0x142830211  0f57e4                         xorps	%xmm4, %xmm4
// 0x142830214  f30f51e2                       sqrtss	%xmm2, %xmm4
// Frames: F152 -> F144 -> F145
// 0x142830218  f30f102d54646d00               movss	0x6d6454(%rip), %xmm5   # 0x142f06674
// 0x142830220  f30f5eec                       divss	%xmm4, %xmm5
// Frames: F153 -> F144 -> F145
// 0x142830224  0fc6ed00                       shufps	$0x0, %xmm5, %xmm5      # xmm5 = xmm5[0,0,0,0]
// Frames: F150 -> F144 -> F145
// 0x142830228  0f14c1                         unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
// 0x14283022b  410f28e7                       movaps	%xmm15, %xmm4
// 0x14283022f  0fc6e1d4                       shufps	$0xd4, %xmm1, %xmm4     # xmm4 = xmm4[0,1],xmm1[1,3]
// 0x142830233  0fc6c484                       shufps	$0x84, %xmm4, %xmm0     # xmm0 = xmm0[0,1],xmm4[0,2]
// 0x142830237  410f16db                       movlhps	%xmm11, %xmm3           # xmm3 = xmm3[0],xmm11[0]
// 0x14283023b  0f5cc3                         subps	%xmm3, %xmm0
// 0x14283023e  0fc6d0e4                       shufps	$0xe4, %xmm0, %xmm2     # xmm2 = xmm2[0,1],xmm0[2,3]
// 0x142830242  0fc6c224                       shufps	$0x24, %xmm2, %xmm0     # xmm0 = xmm0[0,1],xmm2[2,0]
// 0x142830246  0f59c5                         mulps	%xmm5, %xmm0
// 0x142830249  e9f2000000                     jmp	0x142830340 <.text+0x282f340>
// 0x14283024e  6690                           nop
// Frames: F154 -> F144 -> F145
// 0x142830250  f3410f5cd4                     subss	%xmm12, %xmm2
// Frames: F155 -> F144 -> F145
// 0x142830255  f30f101dc7476d00               movss	0x6d47c7(%rip), %xmm3   # 0x142f04a24
// 0x14283025d  f30f5cdc                       subss	%xmm4, %xmm3
// Frames: F156 -> F144 -> F145
// 0x142830261  440f2ef2                       ucomiss	%xmm2, %xmm14
// 0x142830265  0f8395000000                   jae	0x142830300 <.text+0x282f300>
// Frames: F157 -> F144 -> F145
// 0x14283026b  410f28c7                       movaps	%xmm15, %xmm0
// 0x14283026f  0f14c2                         unpcklps	%xmm2, %xmm0            # xmm0 = xmm0[0],xmm2[0],xmm0[1],xmm2[1]
// 0x142830272  0fc6c114                       shufps	$0x14, %xmm1, %xmm0     # xmm0 = xmm0[0,1],xmm1[1,0]
// 0x142830276  410f28cb                       movaps	%xmm11, %xmm1
// 0x14283027a  0f14cb                         unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
// 0x14283027d  410f28d3                       movaps	%xmm11, %xmm2
// 0x142830281  f2410f10d5                     movsd	%xmm13, %xmm2           # xmm2 = xmm13[0],xmm2[1]
// 0x142830286  0fc6ca24                       shufps	$0x24, %xmm2, %xmm1     # xmm1 = xmm1[0,1],xmm2[2,0]
// 0x14283028a  0f28d0                         movaps	%xmm0, %xmm2
// 0x14283028d  0f58d1                         addps	%xmm1, %xmm2
// 0x142830290  0f5cc1                         subps	%xmm1, %xmm0
// 0x142830293  0fc6c223                       shufps	$0x23, %xmm2, %xmm0     # xmm0 = xmm0[3,0],xmm2[2,0]
// Frames: F101 -> F102 -> F151 -> F158 -> F144 -> F145
// 0x142830297  f30f16ca                       movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
// Frames: F157 -> F144 -> F145
// 0x14283029b  0fc6d024                       shufps	$0x24, %xmm0, %xmm2     # xmm2 = xmm2[0,1],xmm0[2,0]
// Frames: F101 -> F102 -> F151 -> F158 -> F144 -> F145
// 0x14283029f  f30f51c9                       sqrtss	%xmm1, %xmm1
// Frames: F158 -> F144 -> F145
// 0x1428302a3  f30f1005c9636d00               movss	0x6d63c9(%rip), %xmm0   # 0x142f06674
// 0x1428302ab  f30f5ec1                       divss	%xmm1, %xmm0
// Frames: F159 -> F144 -> F145
// 0x1428302af  0fc6c000                       shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
// Frames: F157 -> F144 -> F145
// 0x1428302b3  0f59c2                         mulps	%xmm2, %xmm0
// 0x1428302b6  e985000000                     jmp	0x142830340 <.text+0x282f340>
// 0x1428302bb  0f1f440000                     nopl	(%rax,%rax)
// Frames: F160 -> F144 -> F145
// 0x1428302c0  f30f5ce2                       subss	%xmm2, %xmm4
// Frames: F101 -> F102 -> F151 -> F161 -> F144 -> F145
// 0x1428302c4  0f57d2                         xorps	%xmm2, %xmm2
// 0x1428302c7  f30f51d4                       sqrtss	%xmm4, %xmm2
// Frames: F161 -> F144 -> F145
// 0x1428302cb  f30f1005a1636d00               movss	0x6d63a1(%rip), %xmm0   # 0x142f06674
// 0x1428302d3  f30f5ec2                       divss	%xmm2, %xmm0
// Frames: F162 -> F144 -> F145
// 0x1428302d7  f3410f10dd                     movss	%xmm13, %xmm3           # xmm3 = xmm13[0],xmm3[1,2,3]
// 0x1428302dc  0f58d9                         addps	%xmm1, %xmm3
// Frames: F163 -> F144 -> F145
// 0x1428302df  410f28cf                       movaps	%xmm15, %xmm1
// 0x1428302e3  f3410f5ccb                     subss	%xmm11, %xmm1
// 0x1428302e8  0f16c8                         movlhps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0]
// 0x1428302eb  0fc6d924                       shufps	$0x24, %xmm1, %xmm3     # xmm3 = xmm3[0,1],xmm1[2,0]
// 0x1428302ef  0f14c4                         unpcklps	%xmm4, %xmm0            # xmm0 = xmm0[0],xmm4[0],xmm0[1],xmm4[1]
// 0x1428302f2  0fc6c010                       shufps	$0x10, %xmm0, %xmm0     # xmm0 = xmm0[0,0,1,0]
// Frames: F162 -> F144 -> F145
// 0x1428302f6  0f59c3                         mulps	%xmm3, %xmm0
// 0x1428302f9  eb45                           jmp	0x142830340 <.text+0x282f340>
// 0x1428302fb  0f1f440000                     nopl	(%rax,%rax)
// Frames: F164 -> F144 -> F145
// 0x142830300  f30f16e1                       movshdup	%xmm1, %xmm4            # xmm4 = xmm1[1,1,3,3]
// 0x142830304  f30f5cc4                       subss	%xmm4, %xmm0
// Frames: F165 -> F144 -> F145
// 0x142830308  f3410f58cd                     addss	%xmm13, %xmm1
// Frames: F166 -> F144 -> F145
// 0x14283030d  0f14c8                         unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
// Frames: F167 -> F144 -> F145
// 0x142830310  410f28c7                       movaps	%xmm15, %xmm0
// 0x142830314  f3410f58c3                     addss	%xmm11, %xmm0
// Frames: F168 -> F144 -> F145
// 0x142830319  f30f5cda                       subss	%xmm2, %xmm3
// Frames: F101 -> F102 -> F151 -> F169 -> F144 -> F145
// 0x14283031d  0f57d2                         xorps	%xmm2, %xmm2
// 0x142830320  f30f51d3                       sqrtss	%xmm3, %xmm2
// Frames: F169 -> F144 -> F145
// 0x142830324  f30f102548636d00               movss	0x6d6348(%rip), %xmm4   # 0x142f06674
// 0x14283032c  f30f5ee2                       divss	%xmm2, %xmm4
// Frames: F164 -> F144 -> F145
// 0x142830330  0fc6e400                       shufps	$0x0, %xmm4, %xmm4      # xmm4 = xmm4[0,0,0,0]
// Frames: F166 -> F144 -> F145
// 0x142830334  0f14d8                         unpcklps	%xmm0, %xmm3            # xmm3 = xmm3[0],xmm0[0],xmm3[1],xmm0[1]
// 0x142830337  0f16d9                         movlhps	%xmm1, %xmm3            # xmm3 = xmm3[0],xmm1[0]
// 0x14283033a  0f59dc                         mulps	%xmm4, %xmm3
// 0x14283033d  0f28c3                         movaps	%xmm3, %xmm0
// Frames: F145
// 0x142830340  0f28bdb0000000                 movaps	0xb0(%rbp), %xmm7
// 0x142830347  0f298540010000                 movaps	%xmm0, 0x140(%rbp)
// 0x14283034e  488d8d40010000                 leaq	0x140(%rbp), %rcx
// 0x142830355  e816c8cdff                     callq	0x14250cb70 <.text+0x250bb70>
// 0x14283035a  90                             nop
// Frames: F170
// 0x14283035b  4889c3                         movq	%rax, %rbx
// 0x14283035e  410f5cf0                       subps	%xmm8, %xmm6
// 0x142830362  0f59f6                         mulps	%xmm6, %xmm6
// 0x142830365  f30f16c6                       movshdup	%xmm6, %xmm0            # xmm0 = xmm6[1,1,3,3]
// 0x142830369  f30f58c6                       addss	%xmm6, %xmm0
// 0x14283036d  f3440f51c0                     sqrtss	%xmm0, %xmm8
// 0x142830372  f3450f58d1                     addss	%xmm9, %xmm10
// 0x142830377  f3440f599508010000             mulss	0x108(%rbp), %xmm10
// 0x142830380  f3440f118540010000             movss	%xmm8, 0x140(%rbp)
// 0x142830389  f3440f119544010000             movss	%xmm10, 0x144(%rbp)
// 0x142830392  c78548010000cdcccc3d           movl	$0x3dcccccd, 0x148(%rbp) # imm = 0x3DCCCCCD
// 0x14283039c  488d8d40010000                 leaq	0x140(%rbp), %rcx
// 0x1428303a3  e8f8015cfe                     callq	0x140df05a0 <.text+0xdef5a0>
// 0x1428303a8  90                             nop
// Frames: F171
// 0x1428303a9  4889c6                         movq	%rax, %rsi
// 0x1428303ac  488d8d20010000                 leaq	0x120(%rbp), %rcx
// 0x1428303b3  e8b8e45cfe                     callq	0x140dfe870 <.text+0xdfd870>
// 0x1428303b8  90                             nop
// 0x1428303b9  f30f5905f79f6d00               mulss	0x6d9ff7(%rip), %xmm0   # 0x142f0a3b8
// 0x1428303c1  f30f5885a0000000               addss	0xa0(%rbp), %xmm0
// Frames: F45 -> F171
// 0x1428303c9  0f57c9                         xorps	%xmm1, %xmm1
// 0x1428303cc  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F46 -> F171
// 0x1428303d0  f30f10054c466d00               movss	0x6d464c(%rip), %xmm0   # 0x142f04a24
// 0x1428303d8  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F171
// 0x1428303dc  e88f5a60ff                     callq	0x141e35e70 <.text+0x1e34e70>
// 0x1428303e1  90                             nop
// Frames: F172
// 0x1428303e2  0f284d00                       movaps	(%rbp), %xmm1
// 0x1428303e6  0fc60d42086d0030               shufps	$0x30, 0x6d0842(%rip), %xmm1 # xmm1 = xmm1[0,0],mem[3,0]
// 0x1428303ee  0f2845e0                       movaps	-0x20(%rbp), %xmm0
// 0x1428303f2  0fc6c184                       shufps	$0x84, %xmm1, %xmm0     # xmm0 = xmm0[0,1],xmm1[0,2]
// 0x1428303f6  0f284df0                       movaps	-0x10(%rbp), %xmm1
// 0x1428303fa  0fc60d1e397c0030               shufps	$0x30, 0x7c391e(%rip), %xmm1 # xmm1 = xmm1[0,0],mem[3,0]
// 0x142830402  0fc6f984                       shufps	$0x84, %xmm1, %xmm7     # xmm7 = xmm7[0,1],xmm1[0,2]
// 0x142830406  0f58f8                         addps	%xmm0, %xmm7
// 0x142830409  0f13bd40010000                 movlps	%xmm7, 0x140(%rbp)
// 0x142830410  0f28c7                         movaps	%xmm7, %xmm0
// 0x142830413  660f15c7                       unpckhpd	%xmm7, %xmm0            # xmm0 = xmm0[1],xmm7[1]
// 0x142830417  f30f118548010000               movss	%xmm0, 0x148(%rbp)
// 0x14283041f  c7854c01000000000000           movl	$0x0, 0x14c(%rbp)
// 0x142830429  48b90000003c0000ff7b           movabsq	$0x7bff00003c000000, %rcx # imm = 0x7BFF00003C000000
// 0x142830433  48898d50010000                 movq	%rcx, 0x150(%rbp)
// 0x14283043a  89b558010000                   movl	%esi, 0x158(%rbp)
// 0x142830440  48c1ee20                       shrq	$0x20, %rsi
// 0x142830444  6689b55c010000                 movw	%si, 0x15c(%rbp)
// 0x14283044b  6689855e010000                 movw	%ax, 0x15e(%rbp)
// 0x142830452  48899d60010000                 movq	%rbx, 0x160(%rbp)
// 0x142830459  8b852c010000                   movl	0x12c(%rbp), %eax
// 0x14283045f  898568010000                   movl	%eax, 0x168(%rbp)
// 0x142830465  48c7856c01000000000000         movq	$0x0, 0x16c(%rbp)
// 0x142830470  89bd74010000                   movl	%edi, 0x174(%rbp)
// 0x142830476  c7857801000001000000           movl	$0x1, 0x178(%rbp)
// 0x142830480  8b850c010000                   movl	0x10c(%rbp), %eax
// 0x142830486  89857c010000                   movl	%eax, 0x17c(%rbp)
// 0x14283048c  488b8da0020000                 movq	0x2a0(%rbp), %rcx
// 0x142830493  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x14283049a  e8d1070000                     callq	0x142830c70 <.text+0x282fc70>
// 0x14283049f  90                             nop
// Frames: F173
// 0x1428304a0  4183fcff                       cmpl	$-0x1, %r12d
// 0x1428304a4  0f8586f9ffff                   jne	0x14282fe30 <.text+0x282ee30>
// Frames: F35 -> F174 -> F175
// 0x1428304aa  0f2805ef846f00                 movaps	0x6f84ef(%rip), %xmm0   # 0x142f289a0
// 0x1428304b1  f3410f10c2                     movss	%xmm10, %xmm0           # xmm0 = xmm10[0],xmm0[1,2,3]
// 0x1428304b6  0fc6c040                       shufps	$0x40, %xmm0, %xmm0     # xmm0 = xmm0[0,0,0,1]
// 0x1428304ba  410f28cb                       movaps	%xmm11, %xmm1
// 0x1428304be  f30f10155e456d00               movss	0x6d455e(%rip), %xmm2   # 0x142f04a24
// 0x1428304c6  f20f10ca                       movsd	%xmm2, %xmm1            # xmm1 = xmm2[0],xmm1[1]
// 0x1428304ca  440fc6d924                     shufps	$0x24, %xmm1, %xmm11    # xmm11 = xmm11[0,1],xmm1[2,0]
// 0x1428304cf  440f59d8                       mulps	%xmm0, %xmm11
// 0x1428304d3  440f591d05e1ac00               mulps	0xace105(%rip), %xmm11  # 0x1432fe5e0
// Frames: F40 -> F85 -> F175
// 0x1428304db  0f28f7                         movaps	%xmm7, %xmm6
// 0x1428304de  410f58f3                       addps	%xmm11, %xmm6
// Frames: F87 -> F176 -> F177
// 0x1428304e2  410f5cfb                       subps	%xmm11, %xmm7
// Frames: F177
// 0x1428304e6  0fc6ffe8                       shufps	$0xe8, %xmm7, %xmm7     # xmm7 = xmm7[0,2,2,3]
// 0x1428304ea  0f13bd40010000                 movlps	%xmm7, 0x140(%rbp)
// Frames: F178
// 0x1428304f1  f3440f16d6                     movshdup	%xmm6, %xmm10           # xmm10 = xmm6[1,1,3,3]
// 0x1428304f6  488d8dd8000000                 leaq	0xd8(%rbp), %rcx
// 0x1428304fd  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x142830504  410f28d2                       movaps	%xmm10, %xmm2
// 0x142830508  e873005cfe                     callq	0x140df0580 <.text+0xdef580>
// 0x14283050d  90                             nop
// Frames: F179
// 0x14283050e  f20f10bdd8000000               movsd	0xd8(%rbp), %xmm7
// 0x142830516  f30f108ddc000000               movss	0xdc(%rbp), %xmm1
// 0x14283051e  f30f1085e0000000               movss	0xe0(%rbp), %xmm0
// Frames: F87 -> F180 -> F181 -> F179
// 0x142830526  f3410f5cca                     subss	%xmm10, %xmm1
// Frames: F182 -> F183 -> F181 -> F179
// 0x14283052b  f30f59c9                       mulss	%xmm1, %xmm1
// Frames: F87 -> F176 -> F181 -> F179
// 0x14283052f  0f28d0                         movaps	%xmm0, %xmm2
// 0x142830532  0f16d7                         movlhps	%xmm7, %xmm2            # xmm2 = xmm2[0],xmm7[0]
// 0x142830535  0fc6d7e2                       shufps	$0xe2, %xmm7, %xmm2     # xmm2 = xmm2[2,0],xmm7[2,3]
// 0x142830539  0f28de                         movaps	%xmm6, %xmm3
// 0x14283053c  0fc6dee8                       shufps	$0xe8, %xmm6, %xmm3     # xmm3 = xmm3[0,2],xmm6[2,3]
// 0x142830540  0f5cd3                         subps	%xmm3, %xmm2
// Frames: F182 -> F183 -> F181 -> F179
// 0x142830543  0f59d2                         mulps	%xmm2, %xmm2
// 0x142830546  f30f58ca                       addss	%xmm2, %xmm1
// 0x14283054a  f30f16d2                       movshdup	%xmm2, %xmm2            # xmm2 = xmm2[1,1,3,3]
// 0x14283054e  f30f58d1                       addss	%xmm1, %xmm2
// Frames: F101 -> F102 -> F184 -> F183 -> F181 -> F179
// 0x142830552  0f57c9                         xorps	%xmm1, %xmm1
// 0x142830555  f30f51ca                       sqrtss	%xmm2, %xmm1
// Frames: F179
// 0x142830559  0f2e0dd4246f00                 ucomiss	0x6f24d4(%rip), %xmm1   # 0x142f22a34
// 0x142830560  0f86caf8ffff                   jbe	0x14282fe30 <.text+0x282ee30>
// Frames: F112 -> F185
// 0x142830566  410f28d4                       movaps	%xmm12, %xmm2
// 0x14283056a  f3410f59d6                     mulss	%xmm14, %xmm2
// 0x14283056f  410f28dd                       movaps	%xmm13, %xmm3
// 0x142830573  f30f591d85836f00               mulss	0x6f8385(%rip), %xmm3   # 0x142f28900
// 0x14283057b  f30f58da                       addss	%xmm2, %xmm3
// Frames: F186 -> F187
// 0x14283057f  f3440f1185cc000000             movss	%xmm8, 0xcc(%rbp)
// 0x142830588  f30f118dd0000000               movss	%xmm1, 0xd0(%rbp)
// 0x142830590  c785d4000000cdcccc3d           movl	$0x3dcccccd, 0xd4(%rbp) # imm = 0x3DCCCCCD
// Frames: F35 -> F174 -> F188 -> F189
// 0x14283059a  f30f100d82446d00               movss	0x6d4482(%rip), %xmm1   # 0x142f04a24
// 0x1428305a2  0f16c8                         movlhps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0]
// 0x1428305a5  0fc6f924                       shufps	$0x24, %xmm1, %xmm7     # xmm7 = xmm7[0,1],xmm1[2,0]
// 0x1428305a9  0f593d30e0ac00                 mulps	0xace030(%rip), %xmm7   # 0x1432fe5e0
// 0x1428305b0  0f5935e9e87e00                 mulps	0x7ee8e9(%rip), %xmm6   # 0x14301eea0
// Frames: F141 -> F185
// 0x1428305b7  410f28c7                       movaps	%xmm15, %xmm0
// 0x1428305bb  0f5905de836f00                 mulps	0x6f83de(%rip), %xmm0   # 0x142f289a0
// 0x1428305c2  410f28cd                       movaps	%xmm13, %xmm1
// 0x1428305c6  0f14cb                         unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
// 0x1428305c9  0f5cc8                         subps	%xmm0, %xmm1
// Frames: F122 -> F185
// 0x1428305cc  f3410f5cc4                     subss	%xmm12, %xmm0
// Frames: F141 -> F185
// 0x1428305d1  f30f16d1                       movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
// Frames: F156 -> F144 -> F190
// 0x1428305d5  440f2ef2                       ucomiss	%xmm2, %xmm14
// 0x1428305d9  734a                           jae	0x142830625 <.text+0x282f625>
// Frames: F157 -> F144 -> F190
// 0x1428305db  f30f101541446d00               movss	0x6d4441(%rip), %xmm2   # 0x142f04a24
// 0x1428305e3  0f28da                         movaps	%xmm2, %xmm3
// 0x1428305e6  410f16df                       movlhps	%xmm15, %xmm3           # xmm3 = xmm3[0],xmm15[0]
// 0x1428305ea  410fc6dfe2                     shufps	$0xe2, %xmm15, %xmm3    # xmm3 = xmm3[2,0],xmm15[2,3]
// 0x1428305ef  0f58d9                         addps	%xmm1, %xmm3
// Frames: F101 -> F102 -> F151 -> F158 -> F144 -> F190
// 0x1428305f2  f30f16cb                       movshdup	%xmm3, %xmm1            # xmm1 = xmm3[1,1,3,3]
// 0x1428305f6  f30f51c9                       sqrtss	%xmm1, %xmm1
// Frames: F191 -> F144 -> F190
// 0x1428305fa  f30f58c2                       addss	%xmm2, %xmm0
// Frames: F158 -> F144 -> F190
// 0x1428305fe  f30f10156e606d00               movss	0x6d606e(%rip), %xmm2   # 0x142f06674
// 0x142830606  f30f5ed1                       divss	%xmm1, %xmm2
// Frames: F159 -> F144 -> F190
// 0x14283060a  0f57c9                         xorps	%xmm1, %xmm1
// 0x14283060d  f3410f5ccd                     subss	%xmm13, %xmm1
// 0x142830612  0f16ca                         movlhps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0]
// 0x142830615  0fc6d924                       shufps	$0x24, %xmm1, %xmm3     # xmm3 = xmm3[0,1],xmm1[2,0]
// Frames: F157 -> F144 -> F190
// 0x142830619  0f14d0                         unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
// 0x14283061c  0fc6d210                       shufps	$0x10, %xmm2, %xmm2     # xmm2 = xmm2[0,0,1,0]
// 0x142830620  0f59d3                         mulps	%xmm3, %xmm2
// Frames: F156 -> F144 -> F190
// 0x142830623  eb3e                           jmp	0x142830663 <.text+0x282f663>
// Frames: F168 -> F144 -> F190
// 0x142830625  0f281504d3a200                 movaps	0xa2d304(%rip), %xmm2   # 0x14325d930
// 0x14283062c  440f16fa                       movlhps	%xmm2, %xmm15           # xmm15 = xmm15[0],xmm2[0]
// 0x142830630  440fc6fae2                     shufps	$0xe2, %xmm2, %xmm15    # xmm15 = xmm15[2,0],xmm2[2,3]
// 0x142830635  410f16c5                       movlhps	%xmm13, %xmm0           # xmm0 = xmm0[0],xmm13[0]
// 0x142830639  0fc6c821                       shufps	$0x21, %xmm0, %xmm1     # xmm1 = xmm1[1,0],xmm0[2,0]
// 0x14283063d  410f28c7                       movaps	%xmm15, %xmm0
// 0x142830641  0f5cc1                         subps	%xmm1, %xmm0
// 0x142830644  410f58cf                       addps	%xmm15, %xmm1
// 0x142830648  f30f10c8                       movss	%xmm0, %xmm1            # xmm1 = xmm0[0],xmm1[1,2,3]
// Frames: F101 -> F102 -> F151 -> F169 -> F144 -> F190
// 0x14283064c  f30f51c0                       sqrtss	%xmm0, %xmm0
// Frames: F169 -> F144 -> F190
// 0x142830650  f30f10151c606d00               movss	0x6d601c(%rip), %xmm2   # 0x142f06674
// 0x142830658  f30f5ed0                       divss	%xmm0, %xmm2
// Frames: F164 -> F144 -> F190
// 0x14283065c  0fc6d200                       shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
// Frames: F166 -> F144 -> F190
// 0x142830660  0f59d1                         mulps	%xmm1, %xmm2
// Frames: F190
// 0x142830663  0f299540010000                 movaps	%xmm2, 0x140(%rbp)
// 0x14283066a  488d8d40010000                 leaq	0x140(%rbp), %rcx
// 0x142830671  e8fac4cdff                     callq	0x14250cb70 <.text+0x250bb70>
// 0x142830676  90                             nop
// Frames: F192
// 0x142830677  4889c6                         movq	%rax, %rsi
// 0x14283067a  488d8dcc000000                 leaq	0xcc(%rbp), %rcx
// 0x142830681  e81aff5bfe                     callq	0x140df05a0 <.text+0xdef5a0>
// 0x142830686  90                             nop
// Frames: F193
// 0x142830687  4889c3                         movq	%rax, %rbx
// 0x14283068a  0f57c0                         xorps	%xmm0, %xmm0
// 0x14283068d  e8de5760ff                     callq	0x141e35e70 <.text+0x1e34e70>
// 0x142830692  90                             nop
// Frames: F194
// 0x142830693  0f58f7                         addps	%xmm7, %xmm6
// 0x142830696  0f29b540010000                 movaps	%xmm6, 0x140(%rbp)
// 0x14283069d  48b90000003c0000ff7b           movabsq	$0x7bff00003c000000, %rcx # imm = 0x7BFF00003C000000
// 0x1428306a7  48898d50010000                 movq	%rcx, 0x150(%rbp)
// 0x1428306ae  899d58010000                   movl	%ebx, 0x158(%rbp)
// 0x1428306b4  48c1eb20                       shrq	$0x20, %rbx
// 0x1428306b8  66899d5c010000                 movw	%bx, 0x15c(%rbp)
// 0x1428306bf  6689855e010000                 movw	%ax, 0x15e(%rbp)
// 0x1428306c6  4889b560010000                 movq	%rsi, 0x160(%rbp)
// 0x1428306cd  8b852c010000                   movl	0x12c(%rbp), %eax
// 0x1428306d3  898568010000                   movl	%eax, 0x168(%rbp)
// 0x1428306d9  48c7856c01000000000000         movq	$0x0, 0x16c(%rbp)
// 0x1428306e4  89bd74010000                   movl	%edi, 0x174(%rbp)
// 0x1428306ea  48b80100000002000000           movabsq	$0x200000001, %rax # imm = 0x200000001
// 0x1428306f4  48898578010000                 movq	%rax, 0x178(%rbp)
// 0x1428306fb  488b8da0020000                 movq	0x2a0(%rbp), %rcx
// 0x142830702  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x142830709  e862050000                     callq	0x142830c70 <.text+0x282fc70>
// 0x14283070e  90                             nop
// 0x14283070f  e91cf7ffff                     jmp	0x14282fe30 <.text+0x282ee30>
// 0x142830714  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F195 -> F196 -> F197 -> F197 -> F198
// 0x142830720  488b95e8000000                 movq	0xe8(%rbp), %rdx
// Frames: F199 -> F195 -> F196 -> F197 -> F197 -> F198
// 0x142830727  4885d2                         testq	%rdx, %rdx
// 0x14283072a  7416                           je	0x142830742 <.text+0x282f742>
// Frames: F195 -> F196 -> F197 -> F197 -> F198
// 0x14283072c  488b8df0000000                 movq	0xf0(%rbp), %rcx
// Frames: F200 -> F201 -> F195 -> F196 -> F197 -> F197 -> F198
// 0x142830733  48c1e202                       shlq	$0x2, %rdx
// Frames: F202 -> F203 -> F204 -> F205 -> F206 -> F196 -> F197 -> F197 -> F198
// 0x142830737  41b804000000                   movl	$0x4, %r8d
// 0x14283073d  e8ae51e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F14
// 0x142830742  0fb69d87010000                 movzbl	0x187(%rbp), %ebx
// 0x142830749  4c8bb588000000                 movq	0x88(%rbp), %r14
// 0x142830750  448bbd18010000                 movl	0x118(%rbp), %r15d
// 0x142830757  448ba51c010000                 movl	0x11c(%rbp), %r12d
// 0x14283075e  e9faf3ffff                     jmp	0x14282fb5d <.text+0x282eb5d>
// Frames: F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x142830763  488b5530                       movq	0x30(%rbp), %rdx
// Frames: F199 -> F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x142830767  4885d2                         testq	%rdx, %rdx
// 0x14283076a  7413                           je	0x14283077f <.text+0x282f77f>
// Frames: F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x14283076c  488b4d38                       movq	0x38(%rbp), %rcx
// Frames: F200 -> F201 -> F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x142830770  48c1e203                       shlq	$0x3, %rdx
// Frames: F202 -> F203 -> F204 -> F205 -> F206 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x142830774  41b804000000                   movl	$0x4, %r8d
// 0x14283077a  e87151e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x14283077f  488b5548                       movq	0x48(%rbp), %rdx
// Frames: F199 -> F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x142830783  4885d2                         testq	%rdx, %rdx
// 0x142830786  7413                           je	0x14283079b <.text+0x282f79b>
// Frames: F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x142830788  488b4d50                       movq	0x50(%rbp), %rcx
// Frames: F200 -> F201 -> F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x14283078c  48c1e202                       shlq	$0x2, %rdx
// Frames: F202 -> F203 -> F204 -> F205 -> F206 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x142830790  41b804000000                   movl	$0x4, %r8d
// 0x142830796  e85551e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x14283079b  488b55a8                       movq	-0x58(%rbp), %rdx
// Frames: F199 -> F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x14283079f  4885d2                         testq	%rdx, %rdx
// 0x1428307a2  7413                           je	0x1428307b7 <.text+0x282f7b7>
// Frames: F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x1428307a4  488b4db0                       movq	-0x50(%rbp), %rcx
// Frames: F200 -> F201 -> F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x1428307a8  48c1e203                       shlq	$0x3, %rdx
// Frames: F202 -> F203 -> F204 -> F205 -> F206 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x1428307ac  41b804000000                   movl	$0x4, %r8d
// 0x1428307b2  e83951e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x1428307b7  488b55c0                       movq	-0x40(%rbp), %rdx
// Frames: F199 -> F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x1428307bb  4885d2                         testq	%rdx, %rdx
// 0x1428307be  7413                           je	0x1428307d3 <.text+0x282f7d3>
// Frames: F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x1428307c0  488b4dc8                       movq	-0x38(%rbp), %rcx
// Frames: F200 -> F201 -> F195 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x1428307c4  48c1e202                       shlq	$0x2, %rdx
// Frames: F202 -> F203 -> F204 -> F205 -> F206 -> F196 -> F197 -> F197 -> F197 -> F207
// 0x1428307c8  41b804000000                   movl	$0x4, %r8d
// 0x1428307ce  e81d51e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F207
// 0x1428307d3  0f28b590010000                 movaps	0x190(%rbp), %xmm6
// 0x1428307da  0f28bda0010000                 movaps	0x1a0(%rbp), %xmm7
// 0x1428307e1  440f2885b0010000               movaps	0x1b0(%rbp), %xmm8
// 0x1428307e9  440f288dc0010000               movaps	0x1c0(%rbp), %xmm9
// 0x1428307f1  440f2895d0010000               movaps	0x1d0(%rbp), %xmm10
// 0x1428307f9  440f289de0010000               movaps	0x1e0(%rbp), %xmm11
// 0x142830801  440f28a5f0010000               movaps	0x1f0(%rbp), %xmm12
// 0x142830809  440f28ad00020000               movaps	0x200(%rbp), %xmm13
// 0x142830811  440f28b510020000               movaps	0x210(%rbp), %xmm14
// 0x142830819  440f28bd20020000               movaps	0x220(%rbp), %xmm15
// 0x142830821  4881c4b8020000                 addq	$0x2b8, %rsp            # imm = 0x2B8
// 0x142830828  5b                             popq	%rbx
// 0x142830829  5f                             popq	%rdi
// 0x14283082a  5e                             popq	%rsi
// 0x14283082b  415c                           popq	%r12
// 0x14283082d  415d                           popq	%r13
// 0x14283082f  415e                           popq	%r14
// 0x142830831  415f                           popq	%r15
// 0x142830833  5d                             popq	%rbp
// 0x142830834  c3                             retq
// Frames: F208 -> F68 -> F69 -> F70 -> F66 -> F67
// 0x142830835  488d8530010000                 leaq	0x130(%rbp), %rax
// 0x14283083c  48898540010000                 movq	%rax, 0x140(%rbp)
// 0x142830843  488d05e67273fe                 leaq	-0x18c8d1a(%rip), %rax  # 0x140f67b30 <.text+0xf66b30>
// 0x14283084a  48898548010000                 movq	%rax, 0x148(%rbp)
// 0x142830851  488d0df46dd500                 leaq	0xd56df4(%rip), %rcx    # 0x14358764c
// 0x142830858  4c8d05598bd500                 leaq	0xd58b59(%rip), %r8     # 0x1435893b8
// 0x14283085f  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x142830866  e835105800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x14283086b  90                             nop
// 0x14283086c  e960010000                     jmp	0x1428309d1 <.text+0x282f9d1>
// 0x142830871  488d8530010000                 leaq	0x130(%rbp), %rax
// 0x142830878  48898540010000                 movq	%rax, 0x140(%rbp)
// 0x14283087f  488d05aa7273fe                 leaq	-0x18c8d56(%rip), %rax  # 0x140f67b30 <.text+0xf66b30>
// 0x142830886  48898548010000                 movq	%rax, 0x148(%rbp)
// 0x14283088d  488d0db86dd500                 leaq	0xd56db8(%rip), %rcx    # 0x14358764c
// 0x142830894  4c8d054d8bd500                 leaq	0xd58b4d(%rip), %r8     # 0x1435893e8
// 0x14283089b  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x1428308a2  e8f90f5800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x1428308a7  90                             nop
// 0x1428308a8  e924010000                     jmp	0x1428309d1 <.text+0x282f9d1>
// Frames: F208 -> F109
// 0x1428308ad  488d8500010000                 leaq	0x100(%rbp), %rax
// 0x1428308b4  48898540010000                 movq	%rax, 0x140(%rbp)
// 0x1428308bb  488d056e7273fe                 leaq	-0x18c8d92(%rip), %rax  # 0x140f67b30 <.text+0xf66b30>
// 0x1428308c2  48898548010000                 movq	%rax, 0x148(%rbp)
// 0x1428308c9  488d0d7c6dd500                 leaq	0xd56d7c(%rip), %rcx    # 0x14358764c
// 0x1428308d0  4c8d05b18ad500                 leaq	0xd58ab1(%rip), %r8     # 0x143589388
// 0x1428308d7  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x1428308de  e8bd0f5800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x1428308e3  90                             nop
// 0x1428308e4  e9e8000000                     jmp	0x1428309d1 <.text+0x282f9d1>
// Frames: F208 -> F19
// 0x1428308e9  488d85e8000000                 leaq	0xe8(%rbp), %rax
// 0x1428308f0  48898540010000                 movq	%rax, 0x140(%rbp)
// 0x1428308f7  488d05327273fe                 leaq	-0x18c8dce(%rip), %rax  # 0x140f67b30 <.text+0xf66b30>
// 0x1428308fe  48898548010000                 movq	%rax, 0x148(%rbp)
// 0x142830905  488d0d406dd500                 leaq	0xd56d40(%rip), %rcx    # 0x14358764c
// 0x14283090c  4c8d05058ad500                 leaq	0xd58a05(%rip), %r8     # 0x143589318
// 0x142830913  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x14283091a  e8810f5800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x14283091f  90                             nop
// 0x142830920  e9ac000000                     jmp	0x1428309d1 <.text+0x282f9d1>
// Frames: F63
// 0x142830925  488d0d1c8ad500                 leaq	0xd58a1c(%rip), %rcx    # 0x143589348
// 0x14283092c  4c8d053d8ad500                 leaq	0xd58a3d(%rip), %r8     # 0x143589370
// 0x142830933  ba23000000                     movl	$0x23, %edx
// 0x142830938  e8430f5800                     callq	0x142db1880 <.text+0x2db0880>
// 0x14283093d  90                             nop
// 0x14283093e  e98e000000                     jmp	0x1428309d1 <.text+0x282f9d1>
// Frames: F29 -> F30 -> F31 -> F68 -> F69 -> F70 -> F66 -> F67
// 0x142830943  4c8d05568ad500                 leaq	0xd58a56(%rip), %r8     # 0x1435893a0
// 0x14283094a  4c89f1                         movq	%r14, %rcx
// 0x14283094d  4c89fa                         movq	%r15, %rdx
// 0x142830950  e86d0d5800                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x142830955  90                             nop
// 0x142830956  eb79                           jmp	0x1428309d1 <.text+0x282f9d1>
// 0x142830958  4c8d05718ad500                 leaq	0xd58a71(%rip), %r8     # 0x1435893d0
// 0x14283095f  4c89f1                         movq	%r14, %rcx
// 0x142830962  4c89fa                         movq	%r15, %rdx
// 0x142830965  e8580d5800                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x14283096a  90                             nop
// 0x14283096b  eb64                           jmp	0x1428309d1 <.text+0x282f9d1>
// Frames: F29 -> F30 -> F31 -> F92 -> F90
// 0x14283096d  4c8d054c6cd500                 leaq	0xd56c4c(%rip), %r8     # 0x1435875c0
// 0x142830974  488b9538010000                 movq	0x138(%rbp), %rdx
// 0x14283097b  e8420d5800                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x142830980  90                             nop
// 0x142830981  eb4e                           jmp	0x1428309d1 <.text+0x282f9d1>
// Frames: F29 -> F30 -> F31 -> F28 -> F25 -> F26
// 0x142830983  4c8d05066cd500                 leaq	0xd56c06(%rip), %r8     # 0x143587590
// 0x14283098a  e8330d5800                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x14283098f  90                             nop
// 0x142830990  eb3f                           jmp	0x1428309d1 <.text+0x282f9d1>
// 0x142830992  4c8d050f6cd500                 leaq	0xd56c0f(%rip), %r8     # 0x1435875a8
// 0x142830999  4889c1                         movq	%rax, %rcx
// 0x14283099c  488b9538010000                 movq	0x138(%rbp), %rdx
// 0x1428309a3  e81a0d5800                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x1428309a8  90                             nop
// 0x1428309a9  eb26                           jmp	0x1428309d1 <.text+0x282f9d1>
// Frames: F29 -> F30 -> F31 -> F28 -> F25 -> F37
// 0x1428309ab  4c8d05de6bd500                 leaq	0xd56bde(%rip), %r8     # 0x143587590
// 0x1428309b2  e80b0d5800                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x1428309b7  90                             nop
// 0x1428309b8  eb17                           jmp	0x1428309d1 <.text+0x282f9d1>
// 0x1428309ba  4c8d05e76bd500                 leaq	0xd56be7(%rip), %r8     # 0x1435875a8
// 0x1428309c1  4889c1                         movq	%rax, %rcx
// 0x1428309c4  488b9538010000                 movq	0x138(%rbp), %rdx
// 0x1428309cb  e8f20c5800                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x1428309d0  90                             nop
// 0x1428309d1  0f0b                           ud2
// 0x1428309d3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1428309e0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1428309e5  55                             pushq	%rbp
// 0x1428309e6  4157                           pushq	%r15
// 0x1428309e8  4156                           pushq	%r14
// 0x1428309ea  4155                           pushq	%r13
// 0x1428309ec  4154                           pushq	%r12
// 0x1428309ee  56                             pushq	%rsi
// 0x1428309ef  57                             pushq	%rdi
// 0x1428309f0  53                             pushq	%rbx
// 0x1428309f1  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1428309f8  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1428309ff  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x142830a05  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x142830a0b  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x142830a11  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x142830a17  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x142830a1d  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x142830a23  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x142830a2c  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x142830a35  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x142830a3d  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F195 -> F196 -> F197 -> F197 -> F198
// 0x142830a45  488b95e8000000                 movq	0xe8(%rbp), %rdx
// Frames: F199 -> F195 -> F196 -> F197 -> F197 -> F198
// 0x142830a4c  4885d2                         testq	%rdx, %rdx
// 0x142830a4f  7416                           je	0x142830a67 <.text+0x282fa67>
// Frames: F195 -> F196 -> F197 -> F197 -> F198
// 0x142830a51  488b8df0000000                 movq	0xf0(%rbp), %rcx
// Frames: F200 -> F201 -> F195 -> F196 -> F197 -> F197 -> F198
// 0x142830a58  48c1e202                       shlq	$0x2, %rdx
// Frames: F202 -> F203 -> F204 -> F205 -> F206 -> F196 -> F197 -> F197 -> F198
// 0x142830a5c  41b804000000                   movl	$0x4, %r8d
// 0x142830a62  e8894ee7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F198
// 0x142830a67  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x142830a6f  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x142830a77  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x142830a80  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x142830a89  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x142830a8f  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x142830a95  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x142830a9b  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x142830aa1  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x142830aa7  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x142830aad  4881c4c8000000                 addq	$0xc8, %rsp
// 0x142830ab4  5b                             popq	%rbx
// 0x142830ab5  5f                             popq	%rdi
// 0x142830ab6  5e                             popq	%rsi
// 0x142830ab7  415c                           popq	%r12
// 0x142830ab9  415d                           popq	%r13
// 0x142830abb  415e                           popq	%r14
// 0x142830abd  415f                           popq	%r15
// 0x142830abf  5d                             popq	%rbp
// 0x142830ac0  c3                             retq
// 0x142830ac1  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// 0x142830ad0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142830ad5  55                             pushq	%rbp
// 0x142830ad6  4157                           pushq	%r15
// 0x142830ad8  4156                           pushq	%r14
// 0x142830ada  4155                           pushq	%r13
// 0x142830adc  4154                           pushq	%r12
// 0x142830ade  56                             pushq	%rsi
// 0x142830adf  57                             pushq	%rdi
// 0x142830ae0  53                             pushq	%rbx
// 0x142830ae1  4881ecc8000000                 subq	$0xc8, %rsp
// 0x142830ae8  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x142830aef  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x142830af5  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x142830afb  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x142830b01  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x142830b07  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x142830b0d  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x142830b13  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x142830b1c  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x142830b25  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x142830b2d  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// 0x142830b35  488d4da8                       leaq	-0x58(%rbp), %rcx
// Frames: F207
// 0x142830b39  e8921085fd                     callq	0x140081bd0 <.text+0x80bd0>
// 0x142830b3e  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x142830b46  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x142830b4e  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x142830b57  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x142830b60  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x142830b66  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x142830b6c  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x142830b72  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x142830b78  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x142830b7e  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x142830b84  4881c4c8000000                 addq	$0xc8, %rsp
// 0x142830b8b  5b                             popq	%rbx
// 0x142830b8c  5f                             popq	%rdi
// 0x142830b8d  5e                             popq	%rsi
// 0x142830b8e  415c                           popq	%r12
// 0x142830b90  415d                           popq	%r13
// 0x142830b92  415e                           popq	%r14
// 0x142830b94  415f                           popq	%r15
// 0x142830b96  5d                             popq	%rbp
// 0x142830b97  c3                             retq
// 0x142830b98  0f1f840000000000               nopl	(%rax,%rax)
// 0x142830ba0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142830ba5  55                             pushq	%rbp
// 0x142830ba6  4157                           pushq	%r15
// 0x142830ba8  4156                           pushq	%r14
// 0x142830baa  4155                           pushq	%r13
// 0x142830bac  4154                           pushq	%r12
// 0x142830bae  56                             pushq	%rsi
// 0x142830baf  57                             pushq	%rdi
// 0x142830bb0  53                             pushq	%rbx
// 0x142830bb1  4881ecc8000000                 subq	$0xc8, %rsp
// 0x142830bb8  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x142830bbf  440f297c2420                   movaps	%xmm15, 0x20(%rsp)
// 0x142830bc5  440f29742430                   movaps	%xmm14, 0x30(%rsp)
// 0x142830bcb  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x142830bd1  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x142830bd7  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x142830bdd  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x142830be3  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x142830bec  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x142830bf5  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x142830bfd  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// 0x142830c05  488d4d30                       leaq	0x30(%rbp), %rcx
// 0x142830c09  e8c20f85fd                     callq	0x140081bd0 <.text+0x80bd0>
// 0x142830c0e  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x142830c16  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x142830c1e  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x142830c27  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x142830c30  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x142830c36  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x142830c3c  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x142830c42  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x142830c48  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x142830c4e  440f287c2420                   movaps	0x20(%rsp), %xmm15
// 0x142830c54  4881c4c8000000                 addq	$0xc8, %rsp
// 0x142830c5b  5b                             popq	%rbx
// 0x142830c5c  5f                             popq	%rdi
// 0x142830c5d  5e                             popq	%rsi
// 0x142830c5e  415c                           popq	%r12
// 0x142830c60  415d                           popq	%r13
// 0x142830c62  415e                           popq	%r14
// 0x142830c64  415f                           popq	%r15
// 0x142830c66  5d                             popq	%rbp
// 0x142830c67  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/utils/assemble_circular_roof.rs:26:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F2: crates/systems/roof/src/utils/assemble_circular_roof.rs:33:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F3: crates/systems/roof/src/utils/assemble_circular_roof.rs:36:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F4: crates/systems/roof/src/utils/assemble_circular_roof.rs:41:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F5: crates/systems/roof/src/utils/assemble_circular_roof.rs:43:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F6: crates/utils/src/curve/mod.rs:170:0 utils::curve::Curve::length_ws
// F7: crates/systems/roof/src/utils/assemble_circular_roof.rs:48:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F8: crates/systems/roof/src/utils/assemble_circular_roof.rs:49:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F9: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1750:0 core::f32::math::ceil
// F10: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:73:0 std::f32::impl$0::ceil
// F11: crates/systems/roof/src/utils/assemble_circular_roof.rs:50:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:992:0 core::f32::impl$0::max
// F13: crates/systems/roof/src/utils/assemble_circular_roof.rs:52:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F14: crates/systems/roof/src/utils/assemble_circular_roof.rs:57:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:772:0 core::iter::range::impl$5::spec_next
// F16: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:857:0 core::iter::range::impl$6::next
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:1916:0 core::cmp::impls::impl$76::lt
// F18: crates/systems/roof/src/utils/assemble_circular_roof.rs:59:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F19: crates/systems/roof/src/utils/assemble_circular_roof.rs:63:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F20: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:1918:0 core::cmp::impls::impl$53::le
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:2146:0 core::cmp::impls::impl$10::le
// F22: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/range.rs:874:0 core::ops::range::RangeBounds::contains
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/range.rs:527:0 core::ops::range::RangeInclusive::contains
// F24: crates/utils/src/curve/curve_u.rs:91:0 utils::curve::curve_u::CurveU::new
// F25: crates/utils/src/curve/mod.rs:740:0 utils::curve::Curve::get_pos_at_u
// F26: crates/systems/roof/src/utils/assemble_circular_roof.rs:65:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F27: crates/utils/src/curve/mod.rs:1333:0 utils::curve::CurveInternalCoord::i0
// F28: crates/utils/src/curve/mod.rs:800:0 utils::curve::Curve::get_pos_at_coord
// F29: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:272:0 core::slice::index::impl$2::index
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:0 core::slice::index::impl$0::index
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3804:0 alloc::vec::impl$14::index
// F32: crates/utils/src/curve/mod.rs:1338:0 utils::curve::CurveInternalCoord::i1
// F33: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:797:0 glam::f32::vec2::Vec2::lerp
// F34: crates/utils/src/lib.rs:79:0 utils::impl$0::lerp
// F35: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:352:0 core::ops::arith::impl$141::mul
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1311:0 glam::f32::vec2::impl$24::mul
// F37: crates/systems/roof/src/utils/assemble_circular_roof.rs:66:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F38: crates/systems/roof/src/utils/assemble_circular_roof.rs:68:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1312:0 glam::f32::vec2::impl$24::mul
// F40: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1397:0 glam::f32::vec2::impl$34::add
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1396:0 glam::f32::vec2::impl$34::add
// F43: crates/systems/roof/src/utils/assemble_circular_roof.rs:74:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F44: crates/systems/roof/src/utils/assemble_circular_roof.rs:76:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1512:0 core::f32::impl$0::clamp
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1515:0 core::f32::impl$0::clamp
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F48: crates/systems/roof/src/utils/assemble_circular_roof.rs:77:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F49: crates/systems/roof/src/utils/assemble_circular_roof.rs:80:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F50: crates/systems/roof/src/utils/assemble_circular_roof.rs:90:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F51: crates/systems/roof/src/utils/assemble_circular_roof.rs:78:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F52: crates/systems/roof/src/utils/assemble_circular_roof.rs:81:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F53: crates/systems/roof/src/utils/assemble_circular_roof.rs:82:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F54: crates/systems/roof/src/utils/assemble_circular_roof.rs:83:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F55: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:700:0 std::f32::impl$0::sin
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:911:0 std::f32::impl$0::sin_cos
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs:183:0 glam::f32::math::std_math::sin_cos
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs:132:0 glam::f32::sse2::mat2::Mat2::from_angle
// F59: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:724:0 std::f32::impl$0::sin
// F60: crates/systems/roof/src/utils/assemble_circular_roof.rs:92:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F61: crates/systems/roof/src/utils/assemble_circular_roof.rs:93:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F62: crates/systems/roof/src/utils/assemble_circular_roof.rs:91:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F63: crates/systems/roof/src/utils/assemble_circular_roof.rs:97:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F64: crates/systems/roof/src/utils/assemble_circular_roof.rs:211:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F65: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:1916:0 core::cmp::impls::impl$58::lt
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:0 core::iter::adapters::map::impl$2::next
// F67: crates/systems/roof/src/utils/assemble_circular_roof.rs:106:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F68: crates/systems/roof/src/utils/assemble_circular_roof.rs:100:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// F69: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:0 core::ops::function::impls::impl$4::call_once
// F70: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F71: crates/systems/roof/src/utils/assemble_circular_roof.rs:107:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F72: crates/systems/roof/src/utils/assemble_circular_roof.rs:108:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F73: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:975:0 core::core_arch::x86::sse::_mm_set_ps
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs:331:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F75: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:88:0 core::core_arch::x86::sse::_mm_mul_ps
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs:332:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F77: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/macros.rs:157:0 core::core_arch::x86::sse::_mm_shuffle_ps
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs:333:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F79: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:36:0 core::core_arch::x86::sse::_mm_add_ps
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs:334:0 glam::f32::sse2::mat2::Mat2::mul_vec2
// F81: crates/systems/roof/src/utils/assemble_circular_roof.rs:110:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F82: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:517:0 core::ops::arith::impl$241::div
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1176:0 glam::f32::vec2::impl$8::div
// F84: crates/systems/roof/src/utils/assemble_circular_roof.rs:111:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1591:0 glam::f32::vec3::impl$34::add
// F86: crates/systems/roof/src/utils/assemble_circular_roof.rs:119:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F87: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:218:0 core::ops::arith::impl$77::sub
// F88: crates/utils/src/curve/mod.rs:682:0 utils::curve::Curve::get_curve_segment_from_u
// F89: crates/utils/src/curve/mod.rs:793:0 utils::curve::Curve::get_tangent_at_u
// F90: crates/systems/roof/src/utils/assemble_circular_roof.rs:122:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F91: crates/utils/src/curve/mod.rs:683:0 utils::curve::Curve::get_curve_segment_from_u
// F92: crates/utils/src/curve/mod.rs:794:0 utils::curve::Curve::get_tangent_at_u
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1531:0 glam::f32::vec2::impl$50::sub
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1532:0 glam::f32::vec2::impl$50::sub
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:186:0 glam::f32::vec2::Vec2::dot
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:462:0 glam::f32::vec2::Vec2::length
// F97: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:481:0 glam::f32::vec2::Vec2::length_recip
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:533:0 glam::f32::vec2::Vec2::normalize
// F99: crates/utils/src/lib.rs:69:0 utils::impl$0::normalize
// F100: crates/utils/src/curve/mod.rs:796:0 utils::curve::Curve::get_tangent_at_u
// F101: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:2041:0 core::f32::math::sqrt
// F102: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:385:0 std::f32::impl$0::sqrt
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs:193:0 glam::f32::vec2::Vec2::dot
// F104: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:908:0 glam::f32::vec2::Vec2::length
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1311:0 glam::f32::vec2::Vec2::length_recip
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1312:0 glam::f32::vec2::Vec2::length_recip
// F107: crates/systems/roof/src/utils/assemble_circular_roof.rs:123:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F108: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:838:0 std::f32::impl$0::atan
// F109: crates/systems/roof/src/utils/assemble_circular_roof.rs:126:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F110: crates/systems/roof/src/utils/assemble_circular_roof.rs:128:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F111: crates/systems/roof/src/utils/assemble_circular_roof.rs:129:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:265:0 glam::f32::vec3::Vec3::cross
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:148:0 glam::f32::sse2::quat::Quat::from_axis_angle
// F114: crates/systems/roof/src/utils/assemble_circular_roof.rs:132:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs:109:0 glam::f32::sse2::vec3a::Vec3A::new
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs:2115:0 glam::f32::sse2::vec3a::impl$101::from
// F117: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:778:0 core::convert::impl$3::into
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:791:0 glam::f32::sse2::quat::Quat::mul_vec3
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:1175:0 glam::f32::sse2::quat::impl$33::mul
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs:282:0 glam::f32::sse2::vec3a::Vec3A::cross
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:876:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F122: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:266:0 glam::f32::vec3::Vec3::cross
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:82:0 glam::f32::sse2::quat::Quat::from_xyzw
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:150:0 glam::f32::sse2::quat::Quat::from_axis_angle
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs:43:0 glam::sse2::dot3_in_x
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs:67:0 glam::sse2::dot3_into_m128
// F127: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:870:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs:68:0 glam::sse2::dot3_into_m128
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:873:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F130: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:62:0 core::core_arch::x86::sse::_mm_sub_ps
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:874:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F132: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:23:0 core::core_arch::x86::sse::_mm_add_ss
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs:47:0 glam::sse2::dot3_in_x
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:872:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs:284:0 glam::f32::sse2::vec3a::Vec3A::cross
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs:281:0 glam::f32::sse2::vec3a::Vec3A::cross
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs:283:0 glam::f32::sse2::vec3a::Vec3A::cross
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs:285:0 glam::f32::sse2::vec3a::Vec3A::cross
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs:286:0 glam::f32::sse2::vec3a::Vec3A::cross
// F140: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:871:0 glam::f32::sse2::quat::Quat::mul_vec3a
// F141: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:264:0 glam::f32::vec3::Vec3::cross
// F142: crates/systems/roof/src/utils/assemble_circular_roof.rs:133:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F143: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:214:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:278:0 glam::f32::sse2::quat::Quat::from_mat3
// F145: crates/systems/roof/src/utils/assemble_circular_roof.rs:141:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F146: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:241:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:242:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:243:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F149: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:255:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:258:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F151: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs:193:0 glam::f32::math::std_math::sqrt
// F152: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:256:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F153: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:261:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F154: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:216:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:217:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:218:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F157: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:233:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F158: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:231:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F159: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:236:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F160: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:245:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:246:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:248:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F163: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:251:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F164: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:226:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F165: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:225:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F166: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:223:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:224:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F168: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:220:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F169: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:221:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F170: crates/systems/roof/src/utils/assemble_circular_roof.rs:143:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F171: crates/systems/roof/src/utils/assemble_circular_roof.rs:145:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F172: crates/systems/roof/src/utils/assemble_circular_roof.rs:139:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F173: crates/systems/roof/src/utils/assemble_circular_roof.rs:168:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F174: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1503:0 glam::f32::vec3::impl$24::mul
// F175: crates/systems/roof/src/utils/assemble_circular_roof.rs:169:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F176: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1731:0 glam::f32::vec3::impl$50::sub
// F177: crates/systems/roof/src/utils/assemble_circular_roof.rs:170:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F178: crates/systems/roof/src/utils/assemble_circular_roof.rs:172:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F179: crates/systems/roof/src/utils/assemble_circular_roof.rs:174:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F180: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1732:0 glam::f32::vec3::impl$50::sub
// F181: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:579:0 glam::f32::vec3::Vec3::distance
// F182: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:249:0 glam::f32::vec3::Vec3::dot
// F183: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:553:0 glam::f32::vec3::impl$50::sub
// F184: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs:193:0 glam::f32::vec3::Vec3::dot
// F185: crates/systems/roof/src/utils/assemble_circular_roof.rs:176:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F186: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:95:0 glam::f32::vec3::Vec3::new
// F187: crates/systems/roof/src/utils/assemble_circular_roof.rs:180:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F188: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:899:0 glam::f32::vec3::Vec3::lerp
// F189: crates/systems/roof/src/utils/assemble_circular_roof.rs:194:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F190: crates/systems/roof/src/utils/assemble_circular_roof.rs:195:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F191: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs:235:0 glam::f32::sse2::quat::Quat::from_rotation_axes
// F192: crates/systems/roof/src/utils/assemble_circular_roof.rs:197:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F193: crates/systems/roof/src/utils/assemble_circular_roof.rs:198:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F194: crates/systems/roof/src/utils/assemble_circular_roof.rs:193:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F195: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:872:0 alloc::raw_vec::RawVecInner::deallocate
// F196: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:423:0 alloc::raw_vec::impl$3::drop
// F197: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:805:0 core::ptr::drop_in_place
// F198: crates/systems/roof/src/utils/assemble_circular_roof.rs:213:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F199: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:632:0 alloc::raw_vec::RawVecInner::current_memory
// F200: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:1232:0 core::num::impl$11::unchecked_mul
// F201: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:640:0 alloc::raw_vec::RawVecInner::current_memory
// F202: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:115:0 alloc::alloc::dealloc
// F203: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:209:0 alloc::alloc::Global::deallocate_impl_runtime
// F204: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:324:0 alloc::alloc::Global::deallocate_impl
// F205: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:442:0 alloc::alloc::impl$1::deallocate
// F206: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:874:0 alloc::raw_vec::RawVecInner::deallocate
// F207: crates/systems/roof/src/utils/assemble_circular_roof.rs:214:0 system_roof::utils::assemble_circular_roof::assemble_circular_roof::hdface6f6486c6c46
// F208: crates/utils/src/curve/curve_u.rs:90:0 utils::curve::curve_u::CurveU::new
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/utils/assemble_circular_roof.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ba230cf2c0372d1f33d7b12e567af1b1ecb2841b741d3d6e483e091a2d93896e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: assemble_circular_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3786
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 函数符号: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3786,"record_offset":268,"rva":42138096,"va":5410847216}
// 编译实例 RVAs: 0x282f9f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-35171922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8ae51e7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5410850621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41611118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8921085fd","size":5,"soft_fail":false,"target_va":5369240528,"va":5410851641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27573995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e815415bfe","size":5,"soft_fail":false,"target_va":5383273968,"va":5410847958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deb2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27544033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e81fb65bfe","size":5,"soft_fail":false,"target_va":5383303904,"va":5410847932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27523530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e836065cfe","size":5,"soft_fail":false,"target_va":5383325024,"va":5410848549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27522839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8e9085cfe","size":5,"soft_fail":false,"target_va":5383325056,"va":5410847890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df05a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27524616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8f8015cfe","size":5,"soft_fail":false,"target_va":5383325088,"va":5410849699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e35e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10462577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e88f5a60ff","size":5,"soft_fail":false,"target_va":5400387184,"va":5410849756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14250cb70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3291114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e816c8cdff","size":5,"soft_fail":false,"target_va":5407558512,"va":5410849621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14281b4b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-83733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8ebb8feff","size":5,"soft_fail":false,"target_va":5410763952,"va":5410847680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6208851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e853bd5e00","size":5,"soft_fail":false,"target_va":5417057616,"va":5410848760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6210207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e89fc25e00","size":5,"soft_fail":false,"target_va":5417057664,"va":5410847452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6209563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e81bc05e00","size":5,"soft_fail":false,"target_va":5417057696,"va":5410848128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_circular_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6209912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e878c15e00","size":5,"soft_fail":false,"target_va":5417058032,"va":5410848115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5771075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8430f5800","size":5,"soft_fail":false,"target_va":5416622208,"va":5410851128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t5770605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e86d0d5800","size":5,"soft_fail":false,"target_va":5416621762,"va":5410851152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t5771317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e835105800","size":5,"soft_fail":false,"target_va":5416622240,"va":5410850918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29650289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e88f923bfe","size":5,"soft_fail":false,"target_va":5381197120,"va":5410847404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29652120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8688b3bfe","size":5,"soft_fail":false,"target_va":5381195280,"va":5410847395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29652770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8de883bfe","size":5,"soft_fail":false,"target_va":5381195056,"va":5410847821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::profile_curve_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29654747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e825813bfe","size":5,"soft_fail":false,"target_va":5381192688,"va":5410847430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_circle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31777462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e84a1d1bfe","size":5,"soft_fail":false,"target_va":5379069920,"va":5410847377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31777390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8921d1bfe","size":5,"soft_fail":false,"target_va":5379069984,"va":5410847369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27464869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e85beb5cfe","size":5,"soft_fail":false,"target_va":5383383152,"va":5410848016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8d1070000","size":5,"soft_fail":false,"target_va":5410851952,"va":5410849946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27544092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8e4b55bfe","size":5,"soft_fail":false,"target_va":5383304736,"va":5410848823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27544506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e846b45bfe","size":5,"soft_fail":false,"target_va":5383303936,"va":5410848437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27540884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e86cc25bfe","size":5,"soft_fail":false,"target_va":5383307120,"va":5410847999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27574108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410847216,"raw_hex":"e8a4405bfe","size":5,"soft_fail":false,"target_va":5383274080,"va":5410848183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3786,"record_offset":2052,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3786,"record_offset":684,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3786,"record_offset":2340,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3786,"record_offset":712,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3786,"record_offset":6888,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":3786,"record_offset":2224,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":35242,"source_line":131,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":3786,"record_offset":3080,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3786,"record_offset":4376,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":26854,"source_line":146,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3786,"record_offset":5472,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":16667,"source_line":277,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$33::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3786,"record_offset":4528,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11718,"source_line":1174,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":1940,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":1996,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":3288,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":4340,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":6220,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":6512,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":6484,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":5996,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":3344,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":6164,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3786,"record_offset":2084,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3786,"record_offset":4076,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":50265,"source_line":837,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3786,"record_offset":628,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3786,"record_offset":944,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3786,"record_offset":3512,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3786,"record_offset":600,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3786,"record_offset":796,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3786,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3786,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3786,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3786,"record_offset":7052,"parent_record_offset":6960,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3786,"record_offset":6988,"parent_record_offset":6960,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3786,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3786,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3786,"record_offset":1032,"parent_record_offset":972,"ancestor_record_offset":268,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3786,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":268,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3786,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3786,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":268,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3786,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":268,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3786,"record_offset":4584,"parent_record_offset":4556,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3786,"record_offset":3200,"parent_record_offset":3080,"ancestor_record_offset":268,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3786,"record_offset":5224,"parent_record_offset":5168,"ancestor_record_offset":268,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3786,"record_offset":3140,"parent_record_offset":3080,"ancestor_record_offset":268,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3786,"record_offset":3112,"parent_record_offset":3080,"ancestor_record_offset":268,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3786,"record_offset":3168,"parent_record_offset":3080,"ancestor_record_offset":268,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3786,"record_offset":4876,"parent_record_offset":4716,"ancestor_record_offset":268,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3786,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":268,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3786,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":268,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3786,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":268,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3786,"record_offset":2376,"parent_record_offset":2340,"ancestor_record_offset":268,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3786,"record_offset":7020,"parent_record_offset":6988,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3786,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":268,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3786,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":268,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3786,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":268,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3786,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3786,"record_offset":2500,"parent_record_offset":2460,"ancestor_record_offset":268,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3786,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":268,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3786,"record_offset":828,"parent_record_offset":796,"ancestor_record_offset":268,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3786,"record_offset":2460,"parent_record_offset":2340,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3786,"record_offset":6912,"parent_record_offset":6888,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3786,"record_offset":1060,"parent_record_offset":1032,"ancestor_record_offset":268,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3786,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":268,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":3786,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":268,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":3786,"record_offset":5608,"parent_record_offset":5496,"ancestor_record_offset":268,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3786,"record_offset":5496,"parent_record_offset":5472,"ancestor_record_offset":268,"inlinee":16668,"source_line":208,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3786,"record_offset":4496,"parent_record_offset":4376,"ancestor_record_offset":268,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3786,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":268,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":3786,"record_offset":4668,"parent_record_offset":4556,"ancestor_record_offset":268,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3786,"record_offset":4716,"parent_record_offset":4668,"ancestor_record_offset":268,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3786,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":268,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3786,"record_offset":4608,"parent_record_offset":4584,"ancestor_record_offset":268,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":268,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":3876,"parent_record_offset":3844,"ancestor_record_offset":268,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":268,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":268,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":268,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":268,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":1868,"parent_record_offset":1760,"ancestor_record_offset":268,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3786,"record_offset":3708,"parent_record_offset":3512,"ancestor_record_offset":268,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":6312,"parent_record_offset":6244,"ancestor_record_offset":268,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":268,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3786,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":268,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":3786,"record_offset":4968,"parent_record_offset":4940,"ancestor_record_offset":268,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":3786,"record_offset":4940,"parent_record_offset":4668,"ancestor_record_offset":268,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3786,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":268,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3786,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":268,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3786,"record_offset":3940,"parent_record_offset":3908,"ancestor_record_offset":268,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/utils/assemble_circular_roof.rs
//     evidence={"module":3786,"record_offset":2540,"parent_record_offset":2500,"ancestor_record_offset":268,"inlinee":133080,"source_line":100,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3786,"record_offset":3552,"parent_record_offset":3512,"ancestor_record_offset":268,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3786,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":268,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3786,"record_offset":1004,"parent_record_offset":972,"ancestor_record_offset":268,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3786,"record_offset":1128,"parent_record_offset":972,"ancestor_record_offset":268,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3786,"record_offset":2672,"parent_record_offset":2540,"ancestor_record_offset":268,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3786,"record_offset":1252,"parent_record_offset":972,"ancestor_record_offset":268,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3786,"record_offset":3796,"parent_record_offset":3512,"ancestor_record_offset":268,"inlinee":27588,"source_line":68,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x207D8
// 首条位置: {"module":3786,"inlinee":133080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3786,"record_offset":2576,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3786,"record_offset":2672,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3786,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2540,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3786,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2540,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3786,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":2540,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3786,"record_offset":2704,"parent_record_offset":2672,"ancestor_record_offset":2540,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3786,"record_offset":2604,"parent_record_offset":2576,"ancestor_record_offset":2540,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03786.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3786,"record_offset":2632,"parent_record_offset":2604,"ancestor_record_offset":2540,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/03786.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_roof::utils::assemble_circular_roof::assemble_circular_roof::closure$0
// 信息 ID: ca87e377a0557c397dcf6cf5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::utils::assemble_circular_roof::assemble_circular_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>; module=3786, inline_record=2540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>
// 信息 ID: 74c2b78218c85cc5e7a39ab5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_roof::utils::assemble_circular_roof::assemble_circular_roof -> ["system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles @ 0x1f3dfc5 -> 0x282f9f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
