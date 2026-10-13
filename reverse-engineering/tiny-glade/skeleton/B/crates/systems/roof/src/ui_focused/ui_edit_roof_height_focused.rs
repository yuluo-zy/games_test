// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: ui_edit_roof_height_focused.rs.assembly.json.gz

// FUNCTION system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3470:9180 -> host 3470:2728 / rva_21ad440_len_1511; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x21ade27, 0x21adedc), 29 instructions
//   Function-frame RVA span [0x21ae03e, 0x21ae0f6), 30 instructions
// Inline site 3470:9720 -> host 3470:2728 / rva_21ad440_len_1511; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x21ade27, 0x21adedc), 29 instructions
//   Function-frame RVA span [0x21ae03e, 0x21ae0f6), 30 instructions
//
// FUNCTION system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// Candidate interfaces: df6c077194d4ee581fe377dd
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3470:2728 -> rva_21ad440_len_1511
//
// FUNCTION system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3470:13160 -> rva_21ae960_len_1d9
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21ad440_len_1511: RVA 0x21ad440, 5393 PDB bytes; 1164 instructions
// Evidence: _assembly/roof-v1/B/pack-002/rva_21ad440_len_1511.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3470:2728: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// Frames: F1
// 0x1421ad440  55                             pushq	%rbp
// 0x1421ad441  4157                           pushq	%r15
// 0x1421ad443  4156                           pushq	%r14
// 0x1421ad445  4155                           pushq	%r13
// 0x1421ad447  4154                           pushq	%r12
// 0x1421ad449  56                             pushq	%rsi
// 0x1421ad44a  57                             pushq	%rdi
// 0x1421ad44b  53                             pushq	%rbx
// 0x1421ad44c  4881ecd8030000                 subq	$0x3d8, %rsp            # imm = 0x3D8
// 0x1421ad453  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1421ad45b  440f298d40030000               movaps	%xmm9, 0x340(%rbp)
// 0x1421ad463  440f298530030000               movaps	%xmm8, 0x330(%rbp)
// 0x1421ad46b  0f29bd20030000                 movaps	%xmm7, 0x320(%rbp)
// 0x1421ad472  0f29b510030000                 movaps	%xmm6, 0x310(%rbp)
// 0x1421ad479  48c78508030000feffffff         movq	$-0x2, 0x308(%rbp)
// 0x1421ad484  4d89ce                         movq	%r9, %r14
// 0x1421ad487  4d89c4                         movq	%r8, %r12
// 0x1421ad48a  4889d6                         movq	%rdx, %rsi
// 0x1421ad48d  48898da0020000                 movq	%rcx, 0x2a0(%rbp)
// 0x1421ad494  488b85e0030000                 movq	0x3e0(%rbp), %rax
// Frames: F2
// 0x1421ad49b  4c8b38                         movq	(%rax), %r15
// 0x1421ad49e  488b4820                       movq	0x20(%rax), %rcx
// 0x1421ad4a2  48898df8020000                 movq	%rcx, 0x2f8(%rbp)
// 0x1421ad4a9  488b4040                       movq	0x40(%rax), %rax
// 0x1421ad4ad  48898598020000                 movq	%rax, 0x298(%rbp)
// Frames: F3
// 0x1421ad4b4  e84795a6fe                     callq	0x140c16a00 <.text+0xc15a00>
// 0x1421ad4b9  89c3                           movl	%eax, %ebx
// 0x1421ad4bb  84c0                           testb	%al, %al
// 0x1421ad4bd  745c                           je	0x1421ad51b <.text+0x21ac51b>
// Frames: F4 -> F3
// 0x1421ad4bf  bf5b000000                     movl	$0x5b, %edi
// 0x1421ad4c4  b95b000000                     movl	$0x5b, %ecx
// 0x1421ad4c9  e842faffff                     callq	0x1421acf10 <.text+0x21abf10>
// 0x1421ad4ce  4c8d2dcba4e400                 leaq	0xe4a4cb(%rip), %r13    # 0x142ff79a0
// 0x1421ad4d5  4883f801                       cmpq	$0x1, %rax
// 0x1421ad4d9  0f85a7000000                   jne	0x1421ad586 <.text+0x21ac586>
// Frames: F5 -> F6 -> F7 -> F8 -> F9 -> F3
// 0x1421ad4df  4989d1                         movq	%rdx, %r9
// 0x1421ad4e2  4885d2                         testq	%rdx, %rdx
// 0x1421ad4e5  743d                           je	0x1421ad524 <.text+0x21ac524>
// Frames: F10 -> F6 -> F7 -> F8 -> F9 -> F3
// 0x1421ad4e7  4983f95b                       cmpq	$0x5b, %r9
// 0x1421ad4eb  7335                           jae	0x1421ad522 <.text+0x21ac522>
// Frames: F11 -> F6 -> F7 -> F8 -> F9 -> F3
// 0x1421ad4ed  488d05aca4e400                 leaq	0xe4a4ac(%rip), %rax    # 0x142ff79a0
// Frames: F12 -> F11 -> F6 -> F7 -> F8 -> F9 -> F3
// 0x1421ad4f4  41803c01bf                     cmpb	$-0x41, (%r9,%rax)
// Frames: F6 -> F7 -> F8 -> F9 -> F3
// 0x1421ad4f9  7f29                           jg	0x1421ad524 <.text+0x21ac524>
// Frames: F13 -> F8 -> F9 -> F3
// 0x1421ad4fb  488d057ea5e400                 leaq	0xe4a57e(%rip), %rax    # 0x142ff7a80
// 0x1421ad502  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421ad507  488d0d92a4e400                 leaq	0xe4a492(%rip), %rcx    # 0x142ff79a0
// 0x1421ad50e  ba5b000000                     movl	$0x5b, %edx
// 0x1421ad513  4531c0                         xorl	%r8d, %r8d
// 0x1421ad516  e825c47200                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F3
// 0x1421ad51b  31c0                           xorl	%eax, %eax
// 0x1421ad51d  e9f6010000                     jmp	0x1421ad718 <.text+0x21ac718>
// Frames: F6 -> F7 -> F8 -> F9 -> F3
// 0x1421ad522  75d7                           jne	0x1421ad4fb <.text+0x21ac4fb>
// Frames: F9 -> F3
// 0x1421ad524  4c89c9                         movq	%r9, %rcx
// 0x1421ad527  e8e4f9ffff                     callq	0x1421acf10 <.text+0x21abf10>
// 0x1421ad52c  4883f801                       cmpq	$0x1, %rax
// 0x1421ad530  7554                           jne	0x1421ad586 <.text+0x21ac586>
// Frames: F5 -> F14 -> F15 -> F8 -> F16 -> F3
// 0x1421ad532  4989d0                         movq	%rdx, %r8
// 0x1421ad535  4983c002                       addq	$0x2, %r8
// 0x1421ad539  7439                           je	0x1421ad574 <.text+0x21ac574>
// Frames: F10 -> F14 -> F15 -> F8 -> F16 -> F3
// 0x1421ad53b  4983f85b                       cmpq	$0x5b, %r8
// 0x1421ad53f  7331                           jae	0x1421ad572 <.text+0x21ac572>
// Frames: F11 -> F14 -> F15 -> F8 -> F16 -> F3
// 0x1421ad541  488d0558a4e400                 leaq	0xe4a458(%rip), %rax    # 0x142ff79a0
// Frames: F12 -> F11 -> F14 -> F15 -> F8 -> F16 -> F3
// 0x1421ad548  41803c00bf                     cmpb	$-0x41, (%r8,%rax)
// Frames: F14 -> F15 -> F8 -> F16 -> F3
// 0x1421ad54d  7f25                           jg	0x1421ad574 <.text+0x21ac574>
// Frames: F17 -> F8 -> F16 -> F3
// 0x1421ad54f  488d0542a5e400                 leaq	0xe4a542(%rip), %rax    # 0x142ff7a98
// 0x1421ad556  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421ad55b  488d0d3ea4e400                 leaq	0xe4a43e(%rip), %rcx    # 0x142ff79a0
// 0x1421ad562  ba5b000000                     movl	$0x5b, %edx
// 0x1421ad567  41b95b000000                   movl	$0x5b, %r9d
// 0x1421ad56d  e8cec37200                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F14 -> F15 -> F8 -> F16 -> F3
// 0x1421ad572  75db                           jne	0x1421ad54f <.text+0x21ac54f>
// Frames: F18 -> F19 -> F20 -> F15 -> F8 -> F16 -> F3
// 0x1421ad574  bf59000000                     movl	$0x59, %edi
// 0x1421ad579  4829d7                         subq	%rdx, %rdi
// Frames: F21 -> F22 -> F19 -> F20 -> F15 -> F8 -> F16 -> F3
// 0x1421ad57c  4c8d2d1da4e400                 leaq	0xe4a41d(%rip), %r13    # 0x142ff79a0
// 0x1421ad583  4d01c5                         addq	%r8, %r13
// 0x1421ad586  4c89bdf0020000                 movq	%r15, 0x2f0(%rbp)
// 0x1421ad58d  4c8d0565a6e400                 leaq	0xe4a665(%rip), %r8     # 0x142ff7bf9
// 0x1421ad594  4c8d3d1da6e400                 leaq	0xe4a61d(%rip), %r15    # 0x142ff7bb8
// 0x1421ad59b  4889b5e0020000                 movq	%rsi, 0x2e0(%rbp)
// 0x1421ad5a2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5b0  410fbe40ff                     movsbl	-0x1(%r8), %eax
// 0x1421ad5b5  85c0                           testl	%eax, %eax
// 0x1421ad5b7  7817                           js	0x1421ad5d0 <.text+0x21ac5d0>
// Frames: F31 -> F32 -> F33 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5b9  49ffc8                         decq	%r8
// 0x1421ad5bc  83f85c                         cmpl	$0x5c, %eax
// 0x1421ad5bf  7563                           jne	0x1421ad624 <.text+0x21ac624>
// 0x1421ad5c1  eb78                           jmp	0x1421ad63b <.text+0x21ac63b>
// 0x1421ad5c3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F34 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5d0  410fb648fe                     movzbl	-0x2(%r8), %ecx
// Frames: F35 -> F36 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5d5  80f9c0                         cmpb	$-0x40, %cl
// Frames: F36 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5d8  7d23                           jge	0x1421ad5fd <.text+0x21ac5fd>
// Frames: F37 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5da  410fb650fd                     movzbl	-0x3(%r8), %edx
// Frames: F35 -> F38 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5df  80fac0                         cmpb	$-0x40, %dl
// Frames: F38 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5e2  7d22                           jge	0x1421ad606 <.text+0x21ac606>
// Frames: F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5e4  450fb648fc                     movzbl	-0x4(%r8), %r9d
// 0x1421ad5e9  4983c0fc                       addq	$-0x4, %r8
// Frames: F45 -> F46 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5ed  4183e107                       andl	$0x7, %r9d
// Frames: F47 -> F48 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5f1  41c1e106                       shll	$0x6, %r9d
// 0x1421ad5f5  83e23f                         andl	$0x3f, %edx
// 0x1421ad5f8  4409ca                         orl	%r9d, %edx
// 0x1421ad5fb  eb10                           jmp	0x1421ad60d <.text+0x21ac60d>
// Frames: F36 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad5fd  4983c0fe                       addq	$-0x2, %r8
// 0x1421ad601  83e11f                         andl	$0x1f, %ecx
// 0x1421ad604  eb0f                           jmp	0x1421ad615 <.text+0x21ac615>
// 0x1421ad606  4983c0fd                       addq	$-0x3, %r8
// 0x1421ad60a  83e20f                         andl	$0xf, %edx
// Frames: F47 -> F49 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad60d  c1e206                         shll	$0x6, %edx
// 0x1421ad610  83e13f                         andl	$0x3f, %ecx
// 0x1421ad613  09d1                           orl	%edx, %ecx
// Frames: F47 -> F50 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad615  c1e106                         shll	$0x6, %ecx
// 0x1421ad618  243f                           andb	$0x3f, %al
// 0x1421ad61a  0fb6c0                         movzbl	%al, %eax
// 0x1421ad61d  09c8                           orl	%ecx, %eax
// Frames: F31 -> F32 -> F33 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad61f  83f85c                         cmpl	$0x5c, %eax
// 0x1421ad622  7417                           je	0x1421ad63b <.text+0x21ac63b>
// 0x1421ad624  83f82f                         cmpl	$0x2f, %eax
// 0x1421ad627  7412                           je	0x1421ad63b <.text+0x21ac63b>
// Frames: F51 -> F52 -> F53 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad629  4d39f8                         cmpq	%r15, %r8
// Frames: F54 -> F53 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F3
// 0x1421ad62c  7582                           jne	0x1421ad5b0 <.text+0x21ac5b0>
// 0x1421ad62e  4c89f6                         movq	%r14, %rsi
// 0x1421ad631  4189de                         movl	%ebx, %r14d
// 0x1421ad634  bb41000000                     movl	$0x41, %ebx
// 0x1421ad639  eb52                           jmp	0x1421ad68d <.text+0x21ac68d>
// Frames: F55 -> F3
// 0x1421ad63b  4c8d3d76a5e400                 leaq	0xe4a576(%rip), %r15    # 0x142ff7bb8
// 0x1421ad642  4d29f8                         subq	%r15, %r8
// Frames: F5 -> F14 -> F15 -> F8 -> F55 -> F3
// 0x1421ad645  49ffc0                         incq	%r8
// 0x1421ad648  7432                           je	0x1421ad67c <.text+0x21ac67c>
// Frames: F10 -> F14 -> F15 -> F8 -> F55 -> F3
// 0x1421ad64a  4983f841                       cmpq	$0x41, %r8
// 0x1421ad64e  732a                           jae	0x1421ad67a <.text+0x21ac67a>
// Frames: F12 -> F11 -> F14 -> F15 -> F8 -> F55 -> F3
// 0x1421ad650  43803c38bf                     cmpb	$-0x41, (%r8,%r15)
// Frames: F14 -> F15 -> F8 -> F55 -> F3
// 0x1421ad655  7f25                           jg	0x1421ad67c <.text+0x21ac67c>
// Frames: F17 -> F8 -> F55 -> F3
// 0x1421ad657  488d0502a4e400                 leaq	0xe4a402(%rip), %rax    # 0x142ff7a60
// 0x1421ad65e  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421ad663  488d0d4ea5e400                 leaq	0xe4a54e(%rip), %rcx    # 0x142ff7bb8
// 0x1421ad66a  ba41000000                     movl	$0x41, %edx
// 0x1421ad66f  41b941000000                   movl	$0x41, %r9d
// 0x1421ad675  e8c6c27200                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F14 -> F15 -> F8 -> F55 -> F3
// 0x1421ad67a  75db                           jne	0x1421ad657 <.text+0x21ac657>
// Frames: F18 -> F19 -> F20 -> F15 -> F8 -> F55 -> F3
// 0x1421ad67c  4c89f6                         movq	%r14, %rsi
// 0x1421ad67f  4189de                         movl	%ebx, %r14d
// 0x1421ad682  bb41000000                     movl	$0x41, %ebx
// 0x1421ad687  4c29c3                         subq	%r8, %rbx
// Frames: F21 -> F22 -> F19 -> F20 -> F15 -> F8 -> F55 -> F3
// 0x1421ad68a  4d01c7                         addq	%r8, %r15
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F61 -> F3
// 0x1421ad68d  e89eaaa6fe                     callq	0x140c18130 <.text+0xc17130>
// 0x1421ad692  4889c1                         movq	%rax, %rcx
// Frames: F62 -> F56 -> F57 -> F58 -> F59 -> F60 -> F61 -> F3
// 0x1421ad695  488b00                         movq	(%rax), %rax
// 0x1421ad698  4883f801                       cmpq	$0x1, %rax
// 0x1421ad69c  0f85820c0000                   jne	0x1421ae324 <.text+0x21ad324>
// Frames: F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F61 -> F3
// 0x1421ad6a2  4883c108                       addq	$0x8, %rcx
// Frames: F64 -> F65 -> F66 -> F67 -> F68 -> F59 -> F60 -> F61 -> F3
// 0x1421ad6a6  48833900                       cmpq	$0x0, (%rcx)
// 0x1421ad6aa  0f85160e0000                   jne	0x1421ae4c6 <.text+0x21ad4c6>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F65 -> F66 -> F67 -> F68 -> F59 -> F60 -> F61 -> F3
// 0x1421ad6b0  48c701ffffffff                 movq	$-0x1, (%rcx)
// 0x1421ad6b7  48898de8020000                 movq	%rcx, 0x2e8(%rbp)
// Frames: F74 -> F75 -> F66 -> F67 -> F68 -> F59 -> F60 -> F61 -> F3
// 0x1421ad6be  4883c108                       addq	$0x8, %rcx
// Frames: F76 -> F67 -> F68 -> F59 -> F60 -> F61 -> F3
// 0x1421ad6c2  48895c2420                     movq	%rbx, 0x20(%rsp)
// 0x1421ad6c7  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x1421ad6d0  48c744242801000000             movq	$0x1, 0x28(%rsp)
// 0x1421ad6d9  4c89ea                         movq	%r13, %rdx
// 0x1421ad6dc  4989f8                         movq	%rdi, %r8
// 0x1421ad6df  4d89f9                         movq	%r15, %r9
// 0x1421ad6e2  e85996a6fe                     callq	0x140c16d40 <.text+0xc15d40>
// Frames: F69 -> F70 -> F71 -> F72 -> F77 -> F78 -> F78 -> F67 -> F68 -> F59 -> F60 -> F61 -> F3
// 0x1421ad6e7  488b8de8020000                 movq	0x2e8(%rbp), %rcx
// 0x1421ad6ee  48ff01                         incq	(%rcx)
// 0x1421ad6f1  488985e8020000                 movq	%rax, 0x2e8(%rbp)
// Frames: F3
// 0x1421ad6f8  48898568010000                 movq	%rax, 0x168(%rbp)
// 0x1421ad6ff  b801000000                     movl	$0x1, %eax
// 0x1421ad704  4489f3                         movl	%r14d, %ebx
// 0x1421ad707  4c8bbdf0020000                 movq	0x2f0(%rbp), %r15
// 0x1421ad70e  4989f6                         movq	%rsi, %r14
// 0x1421ad711  488bb5e0020000                 movq	0x2e0(%rbp), %rsi
// 0x1421ad718  48898560010000                 movq	%rax, 0x160(%rbp)
// Frames: F79
// 0x1421ad71f  488b06                         movq	(%rsi), %rax
// 0x1421ad722  8b08                           movl	(%rax), %ecx
// 0x1421ad724  83f905                         cmpl	$0x5, %ecx
// 0x1421ad727  0f87e20c0000                   ja	0x1421ae40f <.text+0x21ad40f>
// 0x1421ad72d  83f902                         cmpl	$0x2, %ecx
// 0x1421ad730  7409                           je	0x1421ad73b <.text+0x21ac73b>
// 0x1421ad732  83f901                         cmpl	$0x1, %ecx
// 0x1421ad735  0f85d40c0000                   jne	0x1421ae40f <.text+0x21ad40f>
// 0x1421ad73b  889df0020000                   movb	%bl, 0x2f0(%rbp)
// 0x1421ad741  488b5008                       movq	0x8(%rax), %rdx
// 0x1421ad745  488995c8010000                 movq	%rdx, 0x1c8(%rbp)
// Frames: F80
// 0x1421ad74c  498b0c24                       movq	(%r12), %rcx
// 0x1421ad750  e85b4b8efe                     callq	0x140a922b0 <.text+0xa912b0>
// 0x1421ad755  4889c2                         movq	%rax, %rdx
// 0x1421ad758  48c1ea20                       shrq	$0x20, %rdx
// 0x1421ad75c  7553                           jne	0x1421ad7b1 <.text+0x21ac7b1>
// Frames: F78 -> F81
// 0x1421ad75e  80bdf002000000                 cmpb	$0x0, 0x2f0(%rbp)
// 0x1421ad765  0f84ef0c0000                   je	0x1421ae45a <.text+0x21ad45a>
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ad76b  e8c0a9a6fe                     callq	0x140c18130 <.text+0xc17130>
// 0x1421ad770  4889c1                         movq	%rax, %rcx
// Frames: F62 -> F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ad773  488b00                         movq	(%rax), %rax
// 0x1421ad776  4883f801                       cmpq	$0x1, %rax
// 0x1421ad77a  0f85220d0000                   jne	0x1421ae4a2 <.text+0x21ad4a2>
// Frames: F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ad780  4883c108                       addq	$0x8, %rcx
// Frames: F64 -> F65 -> F66 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ad784  48833900                       cmpq	$0x0, (%rcx)
// 0x1421ad788  0f85380d0000                   jne	0x1421ae4c6 <.text+0x21ad4c6>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F65 -> F66 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ad78e  48c701ffffffff                 movq	$-0x1, (%rcx)
// 0x1421ad795  48898df8020000                 movq	%rcx, 0x2f8(%rbp)
// Frames: F74 -> F75 -> F66 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ad79c  4883c108                       addq	$0x8, %rcx
// Frames: F83 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ad7a0  488b95e8020000                 movq	0x2e8(%rbp), %rdx
// 0x1421ad7a7  e84499a6fe                     callq	0x140c170f0 <.text+0xc160f0>
// 0x1421ad7ac  e99f0c0000                     jmp	0x1421ae450 <.text+0x21ad450>
// Frames: F84 -> F85 -> F86 -> F87 -> F88
// 0x1421ad7b1  498b4e08                       movq	0x8(%r14), %rcx
// Frames: F89 -> F90 -> F87 -> F88
// 0x1421ad7b5  4189c1                         movl	%eax, %r9d
// Frames: F91 -> F92 -> F89 -> F90 -> F87 -> F88
// 0x1421ad7b8  4c3b4910                       cmpq	0x10(%rcx), %r9
// 0x1421ad7bc  0f83100d0000                   jae	0x1421ae4d2 <.text+0x21ad4d2>
// Frames: F93 -> F94 -> F95 -> F96 -> F97 -> F98 -> F89 -> F90 -> F87 -> F88
// 0x1421ad7c2  4c8b4108                       movq	0x8(%rcx), %r8
// 0x1421ad7c6  4889c2                         movq	%rax, %rdx
// 0x1421ad7c9  48c1ea20                       shrq	$0x20, %rdx
// Frames: F99 -> F91 -> F92 -> F89 -> F90 -> F87 -> F88
// 0x1421ad7cd  4f8d1489                       leaq	(%r9,%r9,4), %r10
// Frames: F100 -> F101 -> F102 -> F90 -> F87 -> F88
// 0x1421ad7d1  43391490                       cmpl	%edx, (%r8,%r10,4)
// 0x1421ad7d5  0f85f70c0000                   jne	0x1421ae4d2 <.text+0x21ad4d2>
// Frames: F103 -> F104 -> F90 -> F87 -> F88
// 0x1421ad7db  4f8d1490                       leaq	(%r8,%r10,4), %r10
// 0x1421ad7df  458b4204                       movl	0x4(%r10), %r8d
// 0x1421ad7e3  41bbffffffff                   movl	$0xffffffff, %r11d      # imm = 0xFFFFFFFF
// 0x1421ad7e9  4d39d8                         cmpq	%r11, %r8
// Frames: F104 -> F90 -> F87 -> F88
// 0x1421ad7ec  0f84e00c0000                   je	0x1421ae4d2 <.text+0x21ad4d2>
// Frames: F87 -> F88
// 0x1421ad7f2  498b16                         movq	(%r14), %rdx
// 0x1421ad7f5  4531c9                         xorl	%r9d, %r9d
// Frames: F105 -> F106 -> F87 -> F88
// 0x1421ad7f8  4c394238                       cmpq	%r8, 0x38(%rdx)
// 0x1421ad7fc  0f86e00c0000                   jbe	0x1421ae4e2 <.text+0x21ad4e2>
// Frames: F107 -> F108 -> F109 -> F110 -> F111 -> F106 -> F87 -> F88
// 0x1421ad802  4c8b5a28                       movq	0x28(%rdx), %r11
// Frames: F112 -> F107 -> F108 -> F109 -> F110 -> F111 -> F106 -> F87 -> F88
// 0x1421ad806  4489c7                         movl	%r8d, %edi
// 0x1421ad809  c1ef06                         shrl	$0x6, %edi
// Frames: F113 -> F108 -> F109 -> F110 -> F111 -> F106 -> F87 -> F88
// 0x1421ad80c  4d8b1cfb                       movq	(%r11,%rdi,8), %r11
// Frames: F108 -> F109 -> F110 -> F111 -> F106 -> F87 -> F88
// 0x1421ad810  4d0fa3c3                       btq	%r8, %r11
// Frames: F114 -> F87 -> F88
// 0x1421ad814  0f83c80c0000                   jae	0x1421ae4e2 <.text+0x21ad4e2>
// Frames: F93 -> F94 -> F95 -> F96 -> F97 -> F115 -> F116 -> F117 -> F87 -> F88
// 0x1421ad81a  458b4a0c                       movl	0xc(%r10), %r9d
// 0x1421ad81e  418b4210                       movl	0x10(%r10), %eax
// 0x1421ad822  4c8b81a8010000                 movq	0x1a8(%rcx), %r8
// Frames: F99 -> F91 -> F118 -> F116 -> F117 -> F87 -> F88
// 0x1421ad829  4f8d0cc9                       leaq	(%r9,%r9,8), %r9
// Frames: F119 -> F87 -> F88
// 0x1421ad82d  488b8a10010000                 movq	0x110(%rdx), %rcx
// Frames: F91 -> F92 -> F120 -> F121 -> F122 -> F123 -> F124 -> F125 -> F119 -> F87 -> F88
// 0x1421ad834  4b394cc840                     cmpq	%rcx, 0x40(%r8,%r9,8)
// 0x1421ad839  7627                           jbe	0x1421ad862 <.text+0x21ac862>
// Frames: F121 -> F122 -> F123 -> F124 -> F125 -> F119 -> F87 -> F88
// 0x1421ad83b  4b8d14c8                       leaq	(%r8,%r9,8), %rdx
// 0x1421ad83f  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F126 -> F57 -> F127 -> F120 -> F121 -> F122 -> F123 -> F124 -> F125 -> F119 -> F87 -> F88
// 0x1421ad843  498b0cc8                       movq	(%r8,%rcx,8), %rcx
// 0x1421ad847  4885c9                         testq	%rcx, %rcx
// Frames: F127 -> F120 -> F121 -> F122 -> F123 -> F124 -> F125 -> F119 -> F87 -> F88
// 0x1421ad84a  7416                           je	0x1421ad862 <.text+0x21ac862>
// Frames: F121 -> F122 -> F123 -> F124 -> F125 -> F119 -> F87 -> F88
// 0x1421ad84c  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F128 -> F129 -> F130 -> F121 -> F122 -> F123 -> F124 -> F125 -> F119 -> F87 -> F88
// 0x1421ad850  48f7d1                         notq	%rcx
// Frames: F131 -> F132 -> F133 -> F130 -> F134 -> F124 -> F125 -> F119 -> F87 -> F88
// 0x1421ad853  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1421ad857  48c1e104                       shlq	$0x4, %rcx
// 0x1421ad85b  488b4c0a10                     movq	0x10(%rdx,%rcx), %rcx
// 0x1421ad860  eb02                           jmp	0x1421ad864 <.text+0x21ac864>
// 0x1421ad862  31c9                           xorl	%ecx, %ecx
// Frames: F112 -> F135 -> F136 -> F137 -> F138 -> F139 -> F87 -> F88
// 0x1421ad864  4c6bf058                       imulq	$0x58, %rax, %r14
// 0x1421ad868  4901ce                         addq	%rcx, %r14
// Frames: F140
// 0x1421ad86b  4c89f1                         movq	%r14, %rcx
// 0x1421ad86e  e81d4e73fe                     callq	0x1408e2690 <.text+0x8e1690>
// 0x1421ad873  488985e0010000                 movq	%rax, 0x1e0(%rbp)
// 0x1421ad87a  488d8d9c010000                 leaq	0x19c(%rbp), %rcx
// 0x1421ad881  488d95e0010000                 leaq	0x1e0(%rbp), %rdx
// 0x1421ad888  4d89f0                         movq	%r14, %r8
// 0x1421ad88b  e8c04e73fe                     callq	0x1408e2750 <.text+0x8e1750>
// Frames: F141
// 0x1421ad890  f20f10bda0010000               movsd	0x1a0(%rbp), %xmm7
// 0x1421ad898  f3440f1085a8010000             movss	0x1a8(%rbp), %xmm8
// 0x1421ad8a1  83bd9c01000001                 cmpl	$0x1, 0x19c(%rbp)
// 0x1421ad8a8  7524                           jne	0x1421ad8ce <.text+0x21ac8ce>
// Frames: F142
// 0x1421ad8aa  f20f1085ac010000               movsd	0x1ac(%rbp), %xmm0
// Frames: F143 -> F144 -> F142
// 0x1421ad8b2  0f58f8                         addps	%xmm0, %xmm7
// Frames: F143 -> F145 -> F142
// 0x1421ad8b5  f3440f5885b4010000             addss	0x1b4(%rbp), %xmm8
// Frames: F146 -> F147 -> F142
// 0x1421ad8be  0f593d1b807700                 mulps	0x77801b(%rip), %xmm7   # 0x1429258e0
// Frames: F146 -> F148 -> F142
// 0x1421ad8c5  f3440f5905a27f7700             mulss	0x777fa2(%rip), %xmm8   # 0x142925870
// Frames: F143 -> F144 -> F149
// 0x1421ad8ce  f20f10058a438d00               movsd	0x8d438a(%rip), %xmm0   # 0x142a81c60
// 0x1421ad8d6  0f58c7                         addps	%xmm7, %xmm0
// 0x1421ad8d9  0f57c9                         xorps	%xmm1, %xmm1
// Frames: F143 -> F145 -> F149
// 0x1421ad8dc  f3410f58c8                     addss	%xmm8, %xmm1
// Frames: F150 -> F149
// 0x1421ad8e1  0f1385d0020000                 movlps	%xmm0, 0x2d0(%rbp)
// 0x1421ad8e8  f30f118dd8020000               movss	%xmm1, 0x2d8(%rbp)
// Frames: F151
// 0x1421ad8f0  488dbde0010000                 leaq	0x1e0(%rbp), %rdi
// 0x1421ad8f7  4889f9                         movq	%rdi, %rcx
// 0x1421ad8fa  4c89fa                         movq	%r15, %rdx
// 0x1421ad8fd  e8ee94aefe                     callq	0x140c96df0 <.text+0xc95df0>
// 0x1421ad902  f30f1085e0010000               movss	0x1e0(%rbp), %xmm0
// 0x1421ad90a  f30f1095e8010000               movss	0x1e8(%rbp), %xmm2
// 0x1421ad912  0f280dd77f7700                 movaps	0x777fd7(%rip), %xmm1   # 0x1429258f0
// Frames: F152 -> F151
// 0x1421ad919  0f57ca                         xorps	%xmm2, %xmm1
// Frames: F153 -> F154 -> F155 -> F156 -> F151
// 0x1421ad91c  f30f59d2                       mulss	%xmm2, %xmm2
// 0x1421ad920  0f28d8                         movaps	%xmm0, %xmm3
// 0x1421ad923  f30f59d8                       mulss	%xmm0, %xmm3
// 0x1421ad927  f30f58da                       addss	%xmm2, %xmm3
// Frames: F157 -> F158 -> F154 -> F155 -> F156 -> F151
// 0x1421ad92b  f30f51db                       sqrtss	%xmm3, %xmm3
// 0x1421ad92f  f30f1015cd7f7700               movss	0x777fcd(%rip), %xmm2   # 0x142925904
// Frames: F159 -> F155 -> F156 -> F151
// 0x1421ad937  f30f5ed3                       divss	%xmm3, %xmm2
// Frames: F160 -> F161 -> F156 -> F151
// 0x1421ad93b  f30f59ca                       mulss	%xmm2, %xmm1
// Frames: F160 -> F162 -> F156 -> F151
// 0x1421ad93f  f30f59d0                       mulss	%xmm0, %xmm2
// Frames: F151
// 0x1421ad943  4c8dad70010000                 leaq	0x170(%rbp), %r13
// 0x1421ad94a  4c89e9                         movq	%r13, %rcx
// 0x1421ad94d  e87e36aefe                     callq	0x140c90fd0 <.text+0xc8ffd0>
// Frames: F143 -> F145 -> F163
// 0x1421ad952  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421ad955  f30f5885d8020000               addss	0x2d8(%rbp), %xmm0
// Frames: F163
// 0x1421ad95d  f20f108dd0020000               movsd	0x2d0(%rbp), %xmm1
// Frames: F143 -> F144 -> F163
// 0x1421ad965  0f580d74338d00                 addps	0x8d3374(%rip), %xmm1   # 0x142a80ce0
// Frames: F164
// 0x1421ad96c  0f138df0010000                 movlps	%xmm1, 0x1f0(%rbp)
// 0x1421ad973  f30f1185f8010000               movss	%xmm0, 0x1f8(%rbp)
// 0x1421ad97b  0f288570010000                 movaps	0x170(%rbp), %xmm0
// 0x1421ad982  0f2985e0010000                 movaps	%xmm0, 0x1e0(%rbp)
// 0x1421ad989  48b80000803f0000803f           movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
// 0x1421ad993  488985fc010000                 movq	%rax, 0x1fc(%rbp)
// 0x1421ad99a  c785040200000000803f           movl	$0x3f800000, 0x204(%rbp) # imm = 0x3F800000
// Frames: F165
// 0x1421ad9a4  48897c2420                     movq	%rdi, 0x20(%rsp)
// 0x1421ad9a9  488d8d80010000                 leaq	0x180(%rbp), %rcx
// 0x1421ad9b0  f30f100dd8018400               movss	0x8401d8(%rip), %xmm1   # 0x1429edb90
// 0x1421ad9b8  f30f1035447f7700               movss	0x777f44(%rip), %xmm6   # 0x142925904
// 0x1421ad9c0  f30f101da87e7700               movss	0x777ea8(%rip), %xmm3   # 0x142925870
// 0x1421ad9c8  0f28d6                         movaps	%xmm6, %xmm2
// 0x1421ad9cb  e810c792fe                     callq	0x140ada0e0 <.text+0xad90e0>
// Frames: F166
// 0x1421ad9d0  4c89fb                         movq	%r15, %rbx
// 0x1421ad9d3  488b85e8030000                 movq	0x3e8(%rbp), %rax
// 0x1421ad9da  488b00                         movq	(%rax), %rax
// 0x1421ad9dd  8b4810                         movl	0x10(%rax), %ecx
// 0x1421ad9e0  f30f104814                     movss	0x14(%rax), %xmm1
// 0x1421ad9e5  c6850703000001                 movb	$0x1, 0x307(%rbp)
// 0x1421ad9ec  e87f2079fe                     callq	0x14093fa70 <.text+0x93ea70>
// Frames: F167
// 0x1421ad9f1  84c0                           testb	%al, %al
// 0x1421ad9f3  7412                           je	0x1421ada07 <.text+0x21aca07>
// Frames: F168
// 0x1421ad9f5  c6850703000001                 movb	$0x1, 0x307(%rbp)
// 0x1421ad9fc  488d4db8                       leaq	-0x48(%rbp), %rcx
// 0x1421ada00  e8abb57ffe                     callq	0x1409a8fb0 <.text+0x9a7fb0>
// 0x1421ada05  eb46                           jmp	0x1421ada4d <.text+0x21aca4d>
// Frames: F169
// 0x1421ada07  c6850703000001                 movb	$0x1, 0x307(%rbp)
// 0x1421ada0e  488d8de0010000                 leaq	0x1e0(%rbp), %rcx
// 0x1421ada15  e866b57ffe                     callq	0x1409a8f80 <.text+0x9a7f80>
// 0x1421ada1a  48c7859800000002000000         movq	$0x2, 0x98(%rbp)
// 0x1421ada25  c7859000000001000000           movl	$0x1, 0x90(%rbp)
// 0x1421ada2f  c6850703000001                 movb	$0x1, 0x307(%rbp)
// 0x1421ada36  488d4db8                       leaq	-0x48(%rbp), %rcx
// 0x1421ada3a  488d95e0010000                 leaq	0x1e0(%rbp), %rdx
// 0x1421ada41  4c8d8590000000                 leaq	0x90(%rbp), %r8
// 0x1421ada48  e883b57ffe                     callq	0x1409a8fd0 <.text+0x9a7fd0>
// Frames: F170
// 0x1421ada4d  488b85c8030000                 movq	0x3c8(%rbp), %rax
// 0x1421ada54  488b38                         movq	(%rax), %rdi
// Frames: F171
// 0x1421ada57  4c89b590000000                 movq	%r14, 0x90(%rbp)
// 0x1421ada5e  4c8d25dbf8ffff                 leaq	-0x725(%rip), %r12      # 0x1421ad340 <.text+0x21ac340>
// 0x1421ada65  4c89a598000000                 movq	%r12, 0x98(%rbp)
// Frames: F172 -> F173 -> F174 -> F171
// 0x1421ada6c  488d05b5a1e400                 leaq	0xe4a1b5(%rip), %rax    # 0x142ff7c28
// 0x1421ada73  488985e0010000                 movq	%rax, 0x1e0(%rbp)
// 0x1421ada7a  48c785e801000001000000         movq	$0x1, 0x1e8(%rbp)
// 0x1421ada85  488d8590000000                 leaq	0x90(%rbp), %rax
// 0x1421ada8c  488985f0010000                 movq	%rax, 0x1f0(%rbp)
// 0x1421ada93  48c785f801000001000000         movq	$0x1, 0x1f8(%rbp)
// 0x1421ada9e  48c7850002000000000000         movq	$0x0, 0x200(%rbp)
// 0x1421adaa9  c6850703000001                 movb	$0x1, 0x307(%rbp)
// 0x1421adab0  488d4d18                       leaq	0x18(%rbp), %rcx
// 0x1421adab4  488d95e0010000                 leaq	0x1e0(%rbp), %rdx
// 0x1421adabb  e8d0c4bcfe                     callq	0x140d79f90 <.text+0xd78f90>
// Frames: F175 -> F171
// 0x1421adac0  0f104518                       movups	0x18(%rbp), %xmm0
// 0x1421adac4  0f2985b0020000                 movaps	%xmm0, 0x2b0(%rbp)
// 0x1421adacb  488b4528                       movq	0x28(%rbp), %rax
// 0x1421adacf  488985c0020000                 movq	%rax, 0x2c0(%rbp)
// Frames: F176
// 0x1421adad6  488b8590010000                 movq	0x190(%rbp), %rax
// 0x1421adadd  488985a8000000                 movq	%rax, 0xa8(%rbp)
// 0x1421adae4  0f108580010000                 movups	0x180(%rbp), %xmm0
// 0x1421adaeb  0f118598000000                 movups	%xmm0, 0x98(%rbp)
// Frames: F177
// 0x1421adaf2  c6859000000002                 movb	$0x2, 0x90(%rbp)
// Frames: F178
// 0x1421adaf9  c6453000                       movb	$0x0, 0x30(%rbp)
// 0x1421adafd  c6850603000001                 movb	$0x1, 0x306(%rbp)
// Frames: F179
// 0x1421adb04  488d8de0010000                 leaq	0x1e0(%rbp), %rcx
// 0x1421adb0b  488d9590000000                 leaq	0x90(%rbp), %rdx
// 0x1421adb12  4c8d4530                       leaq	0x30(%rbp), %r8
// 0x1421adb16  e8f5b27ffe                     callq	0x1409a8e10 <.text+0x9a7e10>
// Frames: F180
// 0x1421adb1b  0f104508                       movups	0x8(%rbp), %xmm0
// 0x1421adb1f  0f298580000000                 movaps	%xmm0, 0x80(%rbp)
// 0x1421adb26  0f1045f8                       movups	-0x8(%rbp), %xmm0
// 0x1421adb2a  0f294570                       movaps	%xmm0, 0x70(%rbp)
// 0x1421adb2e  0f1045b8                       movups	-0x48(%rbp), %xmm0
// 0x1421adb32  0f104dc8                       movups	-0x38(%rbp), %xmm1
// 0x1421adb36  0f1055d8                       movups	-0x28(%rbp), %xmm2
// 0x1421adb3a  0f105de8                       movups	-0x18(%rbp), %xmm3
// 0x1421adb3e  0f295d60                       movaps	%xmm3, 0x60(%rbp)
// 0x1421adb42  0f295550                       movaps	%xmm2, 0x50(%rbp)
// 0x1421adb46  0f294d40                       movaps	%xmm1, 0x40(%rbp)
// 0x1421adb4a  0f294530                       movaps	%xmm0, 0x30(%rbp)
// 0x1421adb4e  c6850603000001                 movb	$0x1, 0x306(%rbp)
// Frames: F179
// 0x1421adb55  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x1421adb5c  488d95e0010000                 leaq	0x1e0(%rbp), %rdx
// 0x1421adb63  4c8d4530                       leaq	0x30(%rbp), %r8
// 0x1421adb67  e814b37ffe                     callq	0x1409a8e80 <.text+0x9a7e80>
// 0x1421adb6c  c6850603000001                 movb	$0x1, 0x306(%rbp)
// 0x1421adb73  488d8de0010000                 leaq	0x1e0(%rbp), %rcx
// 0x1421adb7a  488d9590000000                 leaq	0x90(%rbp), %rdx
// 0x1421adb81  4531c0                         xorl	%r8d, %r8d
// 0x1421adb84  e857b37ffe                     callq	0x1409a8ee0 <.text+0x9a7ee0>
// Frames: F170
// 0x1421adb89  c6850603000000                 movb	$0x0, 0x306(%rbp)
// 0x1421adb90  488d95b0020000                 leaq	0x2b0(%rbp), %rdx
// 0x1421adb97  4c8d85e0010000                 leaq	0x1e0(%rbp), %r8
// 0x1421adb9e  4889f9                         movq	%rdi, %rcx
// 0x1421adba1  e87ad708ff                     callq	0x14123b320 <.text+0x123a320>
// 0x1421adba6  488985d0010000                 movq	%rax, 0x1d0(%rbp)
// 0x1421adbad  488995d8010000                 movq	%rdx, 0x1d8(%rbp)
// 0x1421adbb4  c6850703000000                 movb	$0x0, 0x307(%rbp)
// Frames: F181
// 0x1421adbbb  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1421adbc2  e8590a78fe                     callq	0x14092e620 <.text+0x92d620>
// 0x1421adbc7  8885e0010000                   movb	%al, 0x1e0(%rbp)
// 0x1421adbcd  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421adbd4  488dbde0010000                 leaq	0x1e0(%rbp), %rdi
// 0x1421adbdb  4889f9                         movq	%rdi, %rcx
// 0x1421adbde  e89d7a71fe                     callq	0x1408c5680 <.text+0x8c4680>
// 0x1421adbe3  84c0                           testb	%al, %al
// 0x1421adbe5  7517                           jne	0x1421adbfe <.text+0x21acbfe>
// 0x1421adbe7  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421adbee  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1421adbf5  e8160378fe                     callq	0x14092df10 <.text+0x92cf10>
// 0x1421adbfa  3c02                           cmpb	$0x2, %al
// 0x1421adbfc  7408                           je	0x1421adc06 <.text+0x21acc06>
// 0x1421adbfe  f30f103522f58100               movss	0x81f522(%rip), %xmm6   # 0x1429cd128
// Frames: F182
// 0x1421adc06  488b85f0030000                 movq	0x3f0(%rbp), %rax
// 0x1421adc0d  f30f11b5dc020000               movss	%xmm6, 0x2dc(%rbp)
// 0x1421adc15  4c8b38                         movq	(%rax), %r15
// 0x1421adc18  488d85c8010000                 leaq	0x1c8(%rbp), %rax
// Frames: F183
// 0x1421adc1f  488985e0010000                 movq	%rax, 0x1e0(%rbp)
// 0x1421adc26  4c89a5e8010000                 movq	%r12, 0x1e8(%rbp)
// 0x1421adc2d  4c89adf0010000                 movq	%r13, 0x1f0(%rbp)
// 0x1421adc34  488d0555d3b9fe                 leaq	-0x1462cab(%rip), %rax  # 0x140d4af90 <.text+0xd49f90>
// 0x1421adc3b  488985f8010000                 movq	%rax, 0x1f8(%rbp)
// 0x1421adc42  488d85d0020000                 leaq	0x2d0(%rbp), %rax
// 0x1421adc49  48898500020000                 movq	%rax, 0x200(%rbp)
// 0x1421adc50  488d05a9ddb9fe                 leaq	-0x1462257(%rip), %rax  # 0x140d4ba00 <.text+0xd4aa00>
// 0x1421adc57  48898508020000                 movq	%rax, 0x208(%rbp)
// 0x1421adc5e  488d85dc020000                 leaq	0x2dc(%rbp), %rax
// 0x1421adc65  48898510020000                 movq	%rax, 0x210(%rbp)
// 0x1421adc6c  488d059decbdfe                 leaq	-0x1421363(%rip), %rax  # 0x140d8c910 <.text+0xd8b910>
// 0x1421adc73  48898518020000                 movq	%rax, 0x218(%rbp)
// Frames: F172 -> F173 -> F174 -> F183
// 0x1421adc7a  488d05c79fe400                 leaq	0xe49fc7(%rip), %rax    # 0x142ff7c48
// 0x1421adc81  48898590000000                 movq	%rax, 0x90(%rbp)
// 0x1421adc88  48c7859800000004000000         movq	$0x4, 0x98(%rbp)
// 0x1421adc93  4889bda0000000                 movq	%rdi, 0xa0(%rbp)
// 0x1421adc9a  48c785a800000004000000         movq	$0x4, 0xa8(%rbp)
// 0x1421adca5  48c785b000000000000000         movq	$0x0, 0xb0(%rbp)
// 0x1421adcb0  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421adcb7  488d8d48010000                 leaq	0x148(%rbp), %rcx
// 0x1421adcbe  488d9590000000                 leaq	0x90(%rbp), %rdx
// 0x1421adcc5  e8c6c2bcfe                     callq	0x140d79f90 <.text+0xd78f90>
// Frames: F175 -> F183
// 0x1421adcca  488b8548010000                 movq	0x148(%rbp), %rax
// 0x1421adcd1  488985e0020000                 movq	%rax, 0x2e0(%rbp)
// 0x1421adcd8  488bb550010000                 movq	0x150(%rbp), %rsi
// 0x1421adcdf  4c8bad58010000                 movq	0x158(%rbp), %r13
// Frames: F184
// 0x1421adce6  0f28b570010000                 movaps	0x170(%rbp), %xmm6
// 0x1421adced  f3440f108ddc020000             movss	0x2dc(%rbp), %xmm9
// 0x1421adcf6  488b85d0020000                 movq	0x2d0(%rbp), %rax
// 0x1421adcfd  488985e0010000                 movq	%rax, 0x1e0(%rbp)
// 0x1421add04  8b85d8020000                   movl	0x2d8(%rbp), %eax
// 0x1421add0a  8985e8010000                   movl	%eax, 0x1e8(%rbp)
// Frames: F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F184
// 0x1421add10  0fb605f26e4801                 movzbl	0x1486ef2(%rip), %eax   # 0x143634c09
// Frames: F192 -> F187 -> F188 -> F189 -> F190 -> F191 -> F184
// 0x1421add17  b930000000                     movl	$0x30, %ecx
// 0x1421add1c  ba10000000                     movl	$0x10, %edx
// 0x1421add21  e8ea5e46fe                     callq	0x140613c10 <.text+0x612c10>
// Frames: F189 -> F190 -> F191 -> F184
// 0x1421add26  4885c0                         testq	%rax, %rax
// 0x1421add29  4889b5a8020000                 movq	%rsi, 0x2a8(%rbp)
// 0x1421add30  0f84ee070000                   je	0x1421ae524 <.text+0x21ad524>
// Frames: F190 -> F191 -> F184
// 0x1421add36  48c70001000000                 movq	$0x1, (%rax)
// 0x1421add3d  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x1421add45  0f297010                       movaps	%xmm6, 0x10(%rax)
// 0x1421add49  f3440f114820                   movss	%xmm9, 0x20(%rax)
// 0x1421add4f  488b8de0010000                 movq	0x1e0(%rbp), %rcx
// 0x1421add56  48894824                       movq	%rcx, 0x24(%rax)
// 0x1421add5a  8b8de8010000                   movl	0x1e8(%rbp), %ecx
// 0x1421add60  89482c                         movl	%ecx, 0x2c(%rax)
// Frames: F193
// 0x1421add63  488d0d1e9fe400                 leaq	0xe49f1e(%rip), %rcx    # 0x142ff7c88
// 0x1421add6a  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x1421add6f  4889f9                         movq	%rdi, %rcx
// 0x1421add72  4889f2                         movq	%rsi, %rdx
// 0x1421add75  4d89e8                         movq	%r13, %r8
// 0x1421add78  4989c1                         movq	%rax, %r9
// 0x1421add7b  e830fb6efe                     callq	0x14089d8b0 <.text+0x89c8b0>
// Frames: F182
// 0x1421add80  488b8500020000                 movq	0x200(%rbp), %rax
// 0x1421add87  488985b0000000                 movq	%rax, 0xb0(%rbp)
// 0x1421add8e  0f1085e0010000                 movups	0x1e0(%rbp), %xmm0
// 0x1421add95  0f108df0010000                 movups	0x1f0(%rbp), %xmm1
// 0x1421add9c  0f298da0000000                 movaps	%xmm1, 0xa0(%rbp)
// 0x1421adda3  0f298590000000                 movaps	%xmm0, 0x90(%rbp)
// 0x1421addaa  488d9590000000                 leaq	0x90(%rbp), %rdx
// 0x1421addb1  4c89f9                         movq	%r15, %rcx
// 0x1421addb4  e8d7f197fe                     callq	0x140b2cf90 <.text+0xb2bf90>
// Frames: F194 -> F195 -> F196 -> F78 -> F78 -> F78 -> F197
// 0x1421addb9  488b95e0020000                 movq	0x2e0(%rbp), %rdx
// 0x1421addc0  4885d2                         testq	%rdx, %rdx
// 0x1421addc3  7412                           je	0x1421addd7 <.text+0x21acdd7>
// Frames: F198 -> F199 -> F200 -> F196 -> F78 -> F78 -> F78 -> F197
// 0x1421addc5  41b801000000                   movl	$0x1, %r8d
// 0x1421addcb  488b8da8020000                 movq	0x2a8(%rbp), %rcx
// 0x1421addd2  e8495e46fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F201
// 0x1421addd7  f3410f10764c                   movss	0x4c(%r14), %xmm6
// 0x1421adddd  c6850703000000                 movb	$0x0, 0x307(%rbp)
// Frames: F202
// 0x1421adde4  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1421addeb  e8200178fe                     callq	0x14092df10 <.text+0x92cf10>
// 0x1421addf0  4c8bad18040000                 movq	0x418(%rbp), %r13
// 0x1421addf7  488b9510040000                 movq	0x410(%rbp), %rdx
// 0x1421addfe  4c8bbdd8030000                 movq	0x3d8(%rbp), %r15
// 0x1421ade05  04fe                           addb	$-0x2, %al
// 0x1421ade07  0fb6c0                         movzbl	%al, %eax
// 0x1421ade0a  3c03                           cmpb	$0x3, %al
// 0x1421ade0c  b903000000                     movl	$0x3, %ecx
// 0x1421ade11  0f42c8                         cmovbl	%eax, %ecx
// 0x1421ade14  0fb6c1                         movzbl	%cl, %eax
// 0x1421ade17  488d0d0e9fe400                 leaq	0xe49f0e(%rip), %rcx    # 0x142ff7d2c
// 0x1421ade1e  48630481                       movslq	(%rcx,%rax,4), %rax
// 0x1421ade22  4801c8                         addq	%rcx, %rax
// 0x1421ade25  ffe0                           jmpq	*%rax
// Frames: F203 -> F204
// 0x1421ade27  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421ade2e  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x1421ade35  488b9598020000                 movq	0x298(%rbp), %rdx
// 0x1421ade3c  e8dfa591fe                     callq	0x140ac8420 <.text+0xac7420>
// Frames: F205 -> F204
// 0x1421ade41  488b85f8020000                 movq	0x2f8(%rbp), %rax
// 0x1421ade48  f30f1008                       movss	(%rax), %xmm1
// 0x1421ade4c  f30f105004                     movss	0x4(%rax), %xmm2
// 0x1421ade51  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421ade58  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x1421ade5f  e8eca591fe                     callq	0x140ac8450 <.text+0xac7450>
// Frames: F206 -> F204
// 0x1421ade64  0f28d1                         movaps	%xmm1, %xmm2
// 0x1421ade67  f30f11bde0010000               movss	%xmm7, 0x1e0(%rbp)
// 0x1421ade6f  f30f11b5e4010000               movss	%xmm6, 0x1e4(%rbp)
// 0x1421ade77  f3440f1185e8010000             movss	%xmm8, 0x1e8(%rbp)
// 0x1421ade80  48b8000000000000803f           movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
// 0x1421ade8a  488985ec010000                 movq	%rax, 0x1ec(%rbp)
// 0x1421ade91  c785f401000000000000           movl	$0x0, 0x1f4(%rbp)
// Frames: F207 -> F206 -> F204
// 0x1421ade9b  f20f100d7dec9000               movsd	0x90ec7d(%rip), %xmm1   # 0x142abcb20
// 0x1421adea3  f20f114d30                     movsd	%xmm1, 0x30(%rbp)
// 0x1421adea8  c6453800                       movb	$0x0, 0x38(%rbp)
// 0x1421adeac  c6850703000000                 movb	$0x0, 0x307(%rbp)
// Frames: F208 -> F204
// 0x1421adeb3  488d4530                       leaq	0x30(%rbp), %rax
// 0x1421adeb7  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1421adebc  48897c2428                     movq	%rdi, 0x28(%rsp)
// 0x1421adec1  48895c2420                     movq	%rbx, 0x20(%rsp)
// 0x1421adec6  488d8db0020000                 leaq	0x2b0(%rbp), %rcx
// 0x1421adecd  4c8d8d90000000                 leaq	0x90(%rbp), %r9
// 0x1421aded4  0f28c8                         movaps	%xmm0, %xmm1
// 0x1421aded7  e8c4a591fe                     callq	0x140ac84a0 <.text+0xac74a0>
// Frames: F209 -> F210 -> F211 -> F204
// 0x1421adedc  488b85f8030000                 movq	0x3f8(%rbp), %rax
// 0x1421adee3  f30f1085b0020000               movss	0x2b0(%rbp), %xmm0
// Frames: F209 -> F212 -> F211 -> F204
// 0x1421adeeb  f30f108db4020000               movss	0x2b4(%rbp), %xmm1
// Frames: F209 -> F213 -> F211 -> F204
// 0x1421adef3  f30f1095b8020000               movss	0x2b8(%rbp), %xmm2
// Frames: F209 -> F210 -> F211 -> F204
// 0x1421adefb  f30f12df                       movsldup	%xmm7, %xmm3            # xmm3 = xmm7[0,0,2,2]
// 0x1421adeff  0f28e3                         movaps	%xmm3, %xmm4
// 0x1421adf02  f30f10e0                       movss	%xmm0, %xmm4            # xmm4 = xmm0[0],xmm4[1,2,3]
// 0x1421adf06  0f5ce3                         subps	%xmm3, %xmm4
// Frames: F209 -> F212 -> F211 -> F204
// 0x1421adf09  f30f10f9                       movss	%xmm1, %xmm7            # xmm7 = xmm1[0],xmm7[1,2,3]
// 0x1421adf0d  f30f12c6                       movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
// 0x1421adf11  0f5cf8                         subps	%xmm0, %xmm7
// Frames: F209 -> F213 -> F211 -> F204
// 0x1421adf14  410f14d0                       unpcklps	%xmm8, %xmm2            # xmm2 = xmm2[0],xmm8[0],xmm2[1],xmm8[1]
// 0x1421adf18  f3410f12c0                     movsldup	%xmm8, %xmm0            # xmm0 = xmm8[0,0,2,2]
// 0x1421adf1d  0f5cd0                         subps	%xmm0, %xmm2
// Frames: F214 -> F215 -> F211 -> F204
// 0x1421adf20  0f59e4                         mulps	%xmm4, %xmm4
// 0x1421adf23  0f59ff                         mulps	%xmm7, %xmm7
// 0x1421adf26  0f58fc                         addps	%xmm4, %xmm7
// 0x1421adf29  0f59d2                         mulps	%xmm2, %xmm2
// 0x1421adf2c  0f58d7                         addps	%xmm7, %xmm2
// Frames: F157 -> F216 -> F215 -> F211 -> F204
// 0x1421adf2f  0f51c2                         sqrtps	%xmm2, %xmm0
// Frames: F217
// 0x1421adf32  f30f16c8                       movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
// 0x1421adf36  f30f5cc1                       subss	%xmm1, %xmm0
// 0x1421adf3a  488b8da0020000                 movq	0x2a0(%rbp), %rcx
// 0x1421adf41  f30f1101                       movss	%xmm0, (%rcx)
// Frames: F218 -> F219 -> F220
// 0x1421adf45  448b601c                       movl	0x1c(%rax), %r12d
// 0x1421adf49  4c8b6810                       movq	0x10(%rax), %r13
// 0x1421adf4d  45896500                       movl	%r12d, (%r13)
// Frames: F221 -> F220
// 0x1421adf51  488b38                         movq	(%rax), %rdi
// Frames: F222
// 0x1421adf54  498b0e                         movq	(%r14), %rcx
// 0x1421adf57  c6850703000000                 movb	$0x0, 0x307(%rbp)
// Frames: F223 -> F222
// 0x1421adf5e  e8bde570fe                     callq	0x1408bc520 <.text+0x8bb520>
// Frames: F224
// 0x1421adf63  4989d0                         movq	%rdx, %r8
// 0x1421adf66  488b8d00040000                 movq	0x400(%rbp), %rcx
// 0x1421adf6d  488b95c0030000                 movq	0x3c0(%rbp), %rdx
// 0x1421adf74  488b19                         movq	(%rcx), %rbx
// 0x1421adf77  c6850703000000                 movb	$0x0, 0x307(%rbp)
// Frames: F220
// 0x1421adf7e  488d0d339de400                 leaq	0xe49d33(%rip), %rcx    # 0x142ff7cb8
// 0x1421adf85  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x1421adf8a  4889542420                     movq	%rdx, 0x20(%rsp)
// 0x1421adf8f  48c744243009000000             movq	$0x9, 0x30(%rsp)
// 0x1421adf98  4889f9                         movq	%rdi, %rcx
// 0x1421adf9b  4889c2                         movq	%rax, %rdx
// 0x1421adf9e  4989d9                         movq	%rbx, %r9
// 0x1421adfa1  e8ca24ffff                     callq	0x1421a0470 <.text+0x219f470>
// Frames: F218 -> F219 -> F225
// 0x1421adfa6  45896500                       movl	%r12d, (%r13)
// Frames: F225
// 0x1421adfaa  498b16                         movq	(%r14), %rdx
// 0x1421adfad  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421adfb4  4889f9                         movq	%rdi, %rcx
// 0x1421adfb7  4989d8                         movq	%rbx, %r8
// 0x1421adfba  e841dc97fe                     callq	0x140b2bc00 <.text+0xb2ac00>
// Frames: F226
// 0x1421adfbf  488b8510040000                 movq	0x410(%rbp), %rax
// 0x1421adfc6  488b08                         movq	(%rax), %rcx
// 0x1421adfc9  498b16                         movq	(%r14), %rdx
// 0x1421adfcc  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421adfd3  e888c56afe                     callq	0x14085a560 <.text+0x859560>
// Frames: F227 -> F226
// 0x1421adfd8  4885c0                         testq	%rax, %rax
// 0x1421adfdb  0f8454050000                   je	0x1421ae535 <.text+0x21ad535>
// Frames: F226
// 0x1421adfe1  f30f104054                     movss	0x54(%rax), %xmm0
// 0x1421adfe6  488b8518040000                 movq	0x418(%rbp), %rax
// 0x1421adfed  f30f1100                       movss	%xmm0, (%rax)
// Frames: F228
// 0x1421adff1  498b06                         movq	(%r14), %rax
// Frames: F229
// 0x1421adff4  410f104630                     movups	0x30(%r14), %xmm0
// 0x1421adff9  0f1185ec010000                 movups	%xmm0, 0x1ec(%rbp)
// 0x1421ae000  410f104624                     movups	0x24(%r14), %xmm0
// 0x1421ae005  0f2985e0010000                 movaps	%xmm0, 0x1e0(%rbp)
// Frames: F218 -> F219 -> F230
// 0x1421ae00c  418b4f1c                       movl	0x1c(%r15), %ecx
// 0x1421ae010  498b5710                       movq	0x10(%r15), %rdx
// 0x1421ae014  890a                           movl	%ecx, (%rdx)
// Frames: F221 -> F230
// 0x1421ae016  498b0f                         movq	(%r15), %rcx
// Frames: F230
// 0x1421ae019  c70100000000                   movl	$0x0, (%rcx)
// 0x1421ae01f  48894110                       movq	%rax, 0x10(%rcx)
// 0x1421ae023  0f2885e0010000                 movaps	0x1e0(%rbp), %xmm0
// 0x1421ae02a  0f114118                       movups	%xmm0, 0x18(%rcx)
// 0x1421ae02e  0f1085ec010000                 movups	0x1ec(%rbp), %xmm0
// 0x1421ae035  0f114124                       movups	%xmm0, 0x24(%rcx)
// 0x1421ae039  e9ca030000                     jmp	0x1421ae408 <.text+0x21ad408>
// Frames: F203 -> F231
// 0x1421ae03e  4989d7                         movq	%rdx, %r15
// 0x1421ae041  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421ae048  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x1421ae04f  488b9598020000                 movq	0x298(%rbp), %rdx
// 0x1421ae056  e8c5a391fe                     callq	0x140ac8420 <.text+0xac7420>
// Frames: F205 -> F231
// 0x1421ae05b  488b85f8020000                 movq	0x2f8(%rbp), %rax
// 0x1421ae062  f30f1008                       movss	(%rax), %xmm1
// 0x1421ae066  f30f105004                     movss	0x4(%rax), %xmm2
// 0x1421ae06b  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421ae072  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x1421ae079  e8d2a391fe                     callq	0x140ac8450 <.text+0xac7450>
// Frames: F206 -> F231
// 0x1421ae07e  0f28d1                         movaps	%xmm1, %xmm2
// 0x1421ae081  f30f11bde0010000               movss	%xmm7, 0x1e0(%rbp)
// 0x1421ae089  f30f11b5e4010000               movss	%xmm6, 0x1e4(%rbp)
// 0x1421ae091  f3440f1185e8010000             movss	%xmm8, 0x1e8(%rbp)
// 0x1421ae09a  48b8000000000000803f           movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
// 0x1421ae0a4  488985ec010000                 movq	%rax, 0x1ec(%rbp)
// 0x1421ae0ab  c785f401000000000000           movl	$0x0, 0x1f4(%rbp)
// Frames: F207 -> F206 -> F231
// 0x1421ae0b5  f20f100d63ea9000               movsd	0x90ea63(%rip), %xmm1   # 0x142abcb20
// 0x1421ae0bd  f20f114d30                     movsd	%xmm1, 0x30(%rbp)
// 0x1421ae0c2  c6453800                       movb	$0x0, 0x38(%rbp)
// 0x1421ae0c6  c6850703000000                 movb	$0x0, 0x307(%rbp)
// Frames: F208 -> F231
// 0x1421ae0cd  488d4530                       leaq	0x30(%rbp), %rax
// 0x1421ae0d1  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1421ae0d6  48897c2428                     movq	%rdi, 0x28(%rsp)
// 0x1421ae0db  48895c2420                     movq	%rbx, 0x20(%rsp)
// 0x1421ae0e0  488d8db0020000                 leaq	0x2b0(%rbp), %rcx
// 0x1421ae0e7  4c8d8d90000000                 leaq	0x90(%rbp), %r9
// 0x1421ae0ee  0f28c8                         movaps	%xmm0, %xmm1
// 0x1421ae0f1  e8aaa391fe                     callq	0x140ac84a0 <.text+0xac74a0>
// Frames: F211 -> F231
// 0x1421ae0f6  f30f1085b0020000               movss	0x2b0(%rbp), %xmm0
// 0x1421ae0fe  f30f108db4020000               movss	0x2b4(%rbp), %xmm1
// 0x1421ae106  f30f1095b8020000               movss	0x2b8(%rbp), %xmm2
// Frames: F209 -> F210 -> F211 -> F231
// 0x1421ae10e  f30f5cc7                       subss	%xmm7, %xmm0
// Frames: F209 -> F212 -> F211 -> F231
// 0x1421ae112  f30f5cce                       subss	%xmm6, %xmm1
// Frames: F209 -> F213 -> F211 -> F231
// 0x1421ae116  f3410f5cd0                     subss	%xmm8, %xmm2
// Frames: F214 -> F215 -> F211 -> F231
// 0x1421ae11b  f30f59c0                       mulss	%xmm0, %xmm0
// 0x1421ae11f  f30f59c9                       mulss	%xmm1, %xmm1
// 0x1421ae123  f30f58c8                       addss	%xmm0, %xmm1
// 0x1421ae127  f30f59d2                       mulss	%xmm2, %xmm2
// 0x1421ae12b  f30f58d1                       addss	%xmm1, %xmm2
// Frames: F157 -> F216 -> F215 -> F211 -> F231
// 0x1421ae12f  f30f51d2                       sqrtss	%xmm2, %xmm2
// 0x1421ae133  488b85a0020000                 movq	0x2a0(%rbp), %rax
// Frames: F231
// 0x1421ae13a  f30f5c10                       subss	(%rax), %xmm2
// 0x1421ae13e  c6850703000000                 movb	$0x0, 0x307(%rbp)
// Frames: F232
// 0x1421ae145  f30f100523787700               movss	0x777823(%rip), %xmm0   # 0x142925970
// 0x1421ae14d  f30f100d7ff18100               movss	0x81f17f(%rip), %xmm1   # 0x1429cd2d4
// 0x1421ae155  e84623aefe                     callq	0x140c904a0 <.text+0xc8f4a0>
// Frames: F233 -> F234
// 0x1421ae15a  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421ae15d  f30f5fc8                       maxss	%xmm0, %xmm1
// 0x1421ae161  f30f10359b777700               movss	0x77779b(%rip), %xmm6   # 0x142925904
// Frames: F235 -> F234
// 0x1421ae169  f30f5df1                       minss	%xmm1, %xmm6
// Frames: F218 -> F219 -> F236
// 0x1421ae16d  418b471c                       movl	0x1c(%r15), %eax
// 0x1421ae171  498b4f10                       movq	0x10(%r15), %rcx
// 0x1421ae175  8901                           movl	%eax, (%rcx)
// Frames: F237 -> F238
// 0x1421ae177  f3410f104500                   movss	(%r13), %xmm0
// 0x1421ae17d  f30f100d271b8d00               movss	0x8d1b27(%rip), %xmm1   # 0x142a7fcac
// 0x1421ae185  0f2ec8                         ucomiss	%xmm0, %xmm1
// 0x1421ae188  0f86f4010000                   jbe	0x1421ae382 <.text+0x21ad382>
// Frames: F239
// 0x1421ae18e  498b0f                         movq	(%r15), %rcx
// 0x1421ae191  f3410f105e30                   movss	0x30(%r14), %xmm3
// 0x1421ae197  0f28d6                         movaps	%xmm6, %xmm2
// 0x1421ae19a  f30f5cd3                       subss	%xmm3, %xmm2
// Frames: F240 -> F238
// 0x1421ae19e  f30f58d3                       addss	%xmm3, %xmm2
// Frames: F241 -> F238
// 0x1421ae1a2  0f2e1ddb777700                 ucomiss	0x7777db(%rip), %xmm3   # 0x142925984
// 0x1421ae1a9  0f8696010000                   jbe	0x1421ae345 <.text+0x21ad345>
// 0x1421ae1af  f30f1025cd777700               movss	0x7777cd(%rip), %xmm4   # 0x142925984
// 0x1421ae1b7  0f2ee2                         ucomiss	%xmm2, %xmm4
// 0x1421ae1ba  0f83a3010000                   jae	0x1421ae363 <.text+0x21ad363>
// Frames: F242 -> F238
// 0x1421ae1c0  0f2ee3                         ucomiss	%xmm3, %xmm4
// 0x1421ae1c3  0f8389010000                   jae	0x1421ae352 <.text+0x21ad352>
// 0x1421ae1c9  e9b4010000                     jmp	0x1421ae382 <.text+0x21ad382>
// Frames: F243
// 0x1421ae1ce  498b07                         movq	(%r15), %rax
// Frames: F126 -> F243
// 0x1421ae1d1  833802                         cmpl	$0x2, (%rax)
// Frames: F243
// 0x1421ae1d4  0f842e020000                   je	0x1421ae408 <.text+0x21ad408>
// Frames: F244 -> F245 -> F246
// 0x1421ae1da  f30f104020                     movss	0x20(%rax), %xmm0
// 0x1421ae1df  410f2e462c                     ucomiss	0x2c(%r14), %xmm0
// 0x1421ae1e4  7560                           jne	0x1421ae246 <.text+0x21ad246>
// 0x1421ae1e6  7a5e                           jp	0x1421ae246 <.text+0x21ad246>
// Frames: F247 -> F245 -> F246
// 0x1421ae1e8  f30f104024                     movss	0x24(%rax), %xmm0
// 0x1421ae1ed  410f2e4630                     ucomiss	0x30(%r14), %xmm0
// 0x1421ae1f2  7552                           jne	0x1421ae246 <.text+0x21ad246>
// 0x1421ae1f4  7a50                           jp	0x1421ae246 <.text+0x21ad246>
// Frames: F248 -> F245 -> F246
// 0x1421ae1f6  f30f104018                     movss	0x18(%rax), %xmm0
// Frames: F249 -> F248 -> F245 -> F246
// 0x1421ae1fb  410f2e4624                     ucomiss	0x24(%r14), %xmm0
// Frames: F248 -> F245 -> F246
// 0x1421ae200  7544                           jne	0x1421ae246 <.text+0x21ad246>
// 0x1421ae202  7a42                           jp	0x1421ae246 <.text+0x21ad246>
// 0x1421ae204  f30f10401c                     movss	0x1c(%rax), %xmm0
// 0x1421ae209  410f2e4628                     ucomiss	0x28(%r14), %xmm0
// 0x1421ae20e  7536                           jne	0x1421ae246 <.text+0x21ad246>
// 0x1421ae210  7a34                           jp	0x1421ae246 <.text+0x21ad246>
// Frames: F250 -> F245 -> F246
// 0x1421ae212  f30f104028                     movss	0x28(%rax), %xmm0
// 0x1421ae217  410f2e4634                     ucomiss	0x34(%r14), %xmm0
// 0x1421ae21c  7528                           jne	0x1421ae246 <.text+0x21ad246>
// 0x1421ae21e  7a26                           jp	0x1421ae246 <.text+0x21ad246>
// Frames: F251 -> F245 -> F246
// 0x1421ae220  0fb64830                       movzbl	0x30(%rax), %ecx
// Frames: F252 -> F251 -> F245 -> F246
// 0x1421ae224  413a4e3c                       cmpb	0x3c(%r14), %cl
// Frames: F251 -> F245 -> F246
// 0x1421ae228  751c                           jne	0x1421ae246 <.text+0x21ad246>
// Frames: F253 -> F245 -> F246
// 0x1421ae22a  f30f10402c                     movss	0x2c(%rax), %xmm0
// 0x1421ae22f  410f2e4638                     ucomiss	0x38(%r14), %xmm0
// 0x1421ae234  7510                           jne	0x1421ae246 <.text+0x21ad246>
// 0x1421ae236  7a0e                           jp	0x1421ae246 <.text+0x21ad246>
// Frames: F254 -> F245 -> F246
// 0x1421ae238  0fb64831                       movzbl	0x31(%rax), %ecx
// Frames: F255 -> F254 -> F245 -> F246
// 0x1421ae23c  413a4e3d                       cmpb	0x3d(%r14), %cl
// Frames: F246
// 0x1421ae240  0f84c2010000                   je	0x1421ae408 <.text+0x21ad408>
// Frames: F218 -> F219 -> F256
// 0x1421ae246  488b8dd0030000                 movq	0x3d0(%rbp), %rcx
// 0x1421ae24d  8b591c                         movl	0x1c(%rcx), %ebx
// 0x1421ae250  4c8b7910                       movq	0x10(%rcx), %r15
// 0x1421ae254  41891f                         movl	%ebx, (%r15)
// Frames: F221 -> F256
// 0x1421ae257  488b39                         movq	(%rcx), %rdi
// Frames: F257
// 0x1421ae25a  0f107010                       movups	0x10(%rax), %xmm6
// Frames: F258
// 0x1421ae25e  440f104020                     movups	0x20(%rax), %xmm8
// Frames: F259
// 0x1421ae263  4d8b6624                       movq	0x24(%r14), %r12
// Frames: F260
// 0x1421ae267  410f107e2c                     movups	0x2c(%r14), %xmm7
// Frames: F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F256
// 0x1421ae26c  0fb60596694801                 movzbl	0x1486996(%rip), %eax   # 0x143634c09
// Frames: F192 -> F187 -> F188 -> F189 -> F190 -> F191 -> F256
// 0x1421ae273  b950000000                     movl	$0x50, %ecx
// 0x1421ae278  ba08000000                     movl	$0x8, %edx
// 0x1421ae27d  e88e5946fe                     callq	0x140613c10 <.text+0x612c10>
// Frames: F189 -> F190 -> F191 -> F256
// 0x1421ae282  4885c0                         testq	%rax, %rax
// 0x1421ae285  0f84bf020000                   je	0x1421ae54a <.text+0x21ad54a>
// Frames: F190 -> F191 -> F256
// 0x1421ae28b  48c70001000000                 movq	$0x1, (%rax)
// 0x1421ae292  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x1421ae29a  c6401002                       movb	$0x2, 0x10(%rax)
// 0x1421ae29e  450fc6c0e1                     shufps	$0xe1, %xmm8, %xmm8     # xmm8 = xmm8[1,0,2,3]
// 0x1421ae2a3  440f114014                     movups	%xmm8, 0x14(%rax)
// 0x1421ae2a8  0fc6ffe1                       shufps	$0xe1, %xmm7, %xmm7     # xmm7 = xmm7[1,0,2,3]
// 0x1421ae2ac  0f117824                       movups	%xmm7, 0x24(%rax)
// 0x1421ae2b0  0f117038                       movups	%xmm6, 0x38(%rax)
// 0x1421ae2b4  4c896048                       movq	%r12, 0x48(%rax)
// Frames: F256
// 0x1421ae2b8  488985e8010000                 movq	%rax, 0x1e8(%rbp)
// 0x1421ae2bf  48c785e001000008000000         movq	$0x8, 0x1e0(%rbp)
// 0x1421ae2ca  4c8d050f9ae400                 leaq	0xe49a0f(%rip), %r8     # 0x142ff7ce0
// 0x1421ae2d1  488d95e0010000                 leaq	0x1e0(%rbp), %rdx
// 0x1421ae2d8  4889f9                         movq	%rdi, %rcx
// 0x1421ae2db  e8e06b8dfe                     callq	0x140a84ec0 <.text+0xa83ec0>
// 0x1421ae2e0  488b8510040000                 movq	0x410(%rbp), %rax
// Frames: F261
// 0x1421ae2e7  488b08                         movq	(%rax), %rcx
// 0x1421ae2ea  498b16                         movq	(%r14), %rdx
// 0x1421ae2ed  e86ec26afe                     callq	0x14085a560 <.text+0x859560>
// Frames: F227 -> F261
// 0x1421ae2f2  4885c0                         testq	%rax, %rax
// 0x1421ae2f5  0f8460020000                   je	0x1421ae55b <.text+0x21ad55b>
// Frames: F261
// 0x1421ae2fb  f30f105854                     movss	0x54(%rax), %xmm3
// Frames: F262
// 0x1421ae300  f3410f105500                   movss	(%r13), %xmm2
// 0x1421ae306  0f2ed3                         ucomiss	%xmm3, %xmm2
// 0x1421ae309  7506                           jne	0x1421ae311 <.text+0x21ad311>
// 0x1421ae30b  0f8bf7000000                   jnp	0x1421ae408 <.text+0x21ad408>
// Frames: F218 -> F219 -> F263
// 0x1421ae311  41891f                         movl	%ebx, (%r15)
// Frames: F263
// 0x1421ae314  498b16                         movq	(%r14), %rdx
// 0x1421ae317  4889f9                         movq	%rdi, %rcx
// 0x1421ae31a  e8615c8dfe                     callq	0x140a83f80 <.text+0xa82f80>
// 0x1421ae31f  e9e4000000                     jmp	0x1421ae408 <.text+0x21ad408>
// Frames: F62 -> F56 -> F57 -> F58 -> F59 -> F60 -> F61 -> F3
// 0x1421ae324  4885c0                         testq	%rax, %rax
// 0x1421ae327  0f858d010000                   jne	0x1421ae4ba <.text+0x21ad4ba>
// Frames: F264 -> F56 -> F57 -> F58 -> F59 -> F60 -> F61 -> F3
// 0x1421ae32d  31d2                           xorl	%edx, %edx
// 0x1421ae32f  e80ca07600                     callq	0x142918340 <.text+0x2917340>
// 0x1421ae334  4889c1                         movq	%rax, %rcx
// Frames: F265 -> F58 -> F59 -> F60 -> F61 -> F3
// 0x1421ae337  4885c0                         testq	%rax, %rax
// 0x1421ae33a  0f8566f3ffff                   jne	0x1421ad6a6 <.text+0x21ac6a6>
// 0x1421ae340  e975010000                     jmp	0x1421ae4ba <.text+0x21ad4ba>
// Frames: F242 -> F238
// 0x1421ae345  f30f100d37767700               movss	0x777637(%rip), %xmm1   # 0x142925984
// 0x1421ae34d  0f2ecb                         ucomiss	%xmm3, %xmm1
// 0x1421ae350  7230                           jb	0x1421ae382 <.text+0x21ad382>
// 0x1421ae352  0f2e152b767700                 ucomiss	0x77762b(%rip), %xmm2   # 0x142925984
// 0x1421ae359  7627                           jbe	0x1421ae382 <.text+0x21ad382>
// 0x1421ae35b  f30f100dcdec8100               movss	0x81eccd(%rip), %xmm1   # 0x1429cd030
// Frames: F266 -> F238
// 0x1421ae363  498b16                         movq	(%r14), %rdx
// Frames: F233 -> F267 -> F238
// 0x1421ae366  f30f5fc8                       maxss	%xmm0, %xmm1
// 0x1421ae36a  f30f10158efa8c00               movss	0x8cfa8e(%rip), %xmm2   # 0x142a7de00
// Frames: F235 -> F267 -> F238
// 0x1421ae372  f30f5dd1                       minss	%xmm1, %xmm2
// 0x1421ae376  c6850703000000                 movb	$0x0, 0x307(%rbp)
// Frames: F268 -> F238
// 0x1421ae37d  e8dec96afe                     callq	0x14085ad60 <.text+0x859d60>
// Frames: F269
// 0x1421ae382  488b8508040000                 movq	0x408(%rbp), %rax
// 0x1421ae389  4d8b36                         movq	(%r14), %r14
// Frames: F218 -> F219 -> F270 -> F271
// 0x1421ae38c  8b481c                         movl	0x1c(%rax), %ecx
// 0x1421ae38f  488b5010                       movq	0x10(%rax), %rdx
// 0x1421ae393  890a                           movl	%ecx, (%rdx)
// Frames: F221 -> F270 -> F271
// 0x1421ae395  488b38                         movq	(%rax), %rdi
// Frames: F272 -> F273 -> F274 -> F270 -> F271
// 0x1421ae398  488b5f30                       movq	0x30(%rdi), %rbx
// Frames: F275 -> F274 -> F270 -> F271
// 0x1421ae39c  4c8b7f40                       movq	0x40(%rdi), %r15
// Frames: F276 -> F273 -> F274 -> F270 -> F271
// 0x1421ae3a0  483b5f20                       cmpq	0x20(%rdi), %rbx
// 0x1421ae3a4  7510                           jne	0x1421ae3b6 <.text+0x21ad3b6>
// Frames: F273 -> F274 -> F270 -> F271
// 0x1421ae3a6  488d4f20                       leaq	0x20(%rdi), %rcx
// Frames: F277 -> F273 -> F274 -> F270 -> F271
// 0x1421ae3aa  488d15bf499c00                 leaq	0x9c49bf(%rip), %rdx    # 0x142b72d70
// 0x1421ae3b1  e8fa433bfe                     callq	0x1405627b0 <.text+0x5617b0>
// Frames: F93 -> F94 -> F95 -> F278 -> F279 -> F273 -> F274 -> F270 -> F271
// 0x1421ae3b6  488b4728                       movq	0x28(%rdi), %rax
// Frames: F280 -> F279 -> F273 -> F274 -> F270 -> F271
// 0x1421ae3ba  4889d9                         movq	%rbx, %rcx
// 0x1421ae3bd  48c1e106                       shlq	$0x6, %rcx
// Frames: F69 -> F281 -> F273 -> F274 -> F270 -> F271
// 0x1421ae3c1  c7040800000000                 movl	$0x0, (%rax,%rcx)
// 0x1421ae3c8  c744080801000000               movl	$0x1, 0x8(%rax,%rcx)
// 0x1421ae3d0  f30f1174080c                   movss	%xmm6, 0xc(%rax,%rcx)
// 0x1421ae3d6  c744081000000000               movl	$0x0, 0x10(%rax,%rcx)
// 0x1421ae3de  c744081800000000               movl	$0x0, 0x18(%rax,%rcx)
// 0x1421ae3e6  4c89740820                     movq	%r14, 0x20(%rax,%rcx)
// 0x1421ae3eb  c744082800000000               movl	$0x0, 0x28(%rax,%rcx)
// 0x1421ae3f3  c644083400                     movb	$0x0, 0x34(%rax,%rcx)
// 0x1421ae3f8  4c897c0838                     movq	%r15, 0x38(%rax,%rcx)
// Frames: F282 -> F273 -> F274 -> F270 -> F271
// 0x1421ae3fd  48ffc3                         incq	%rbx
// 0x1421ae400  48895f30                       movq	%rbx, 0x30(%rdi)
// Frames: F283 -> F274 -> F270 -> F271
// 0x1421ae404  48ff4740                       incq	0x40(%rdi)
// 0x1421ae408  0fb69df0020000                 movzbl	0x2f0(%rbp), %ebx
// Frames: F78 -> F81
// 0x1421ae40f  84db                           testb	%bl, %bl
// 0x1421ae411  7447                           je	0x1421ae45a <.text+0x21ad45a>
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae413  e8189da6fe                     callq	0x140c18130 <.text+0xc17130>
// 0x1421ae418  4889c1                         movq	%rax, %rcx
// Frames: F62 -> F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae41b  488b00                         movq	(%rax), %rax
// 0x1421ae41e  4883f801                       cmpq	$0x1, %rax
// 0x1421ae422  7568                           jne	0x1421ae48c <.text+0x21ad48c>
// Frames: F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae424  4883c108                       addq	$0x8, %rcx
// Frames: F64 -> F65 -> F66 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae428  48833900                       cmpq	$0x0, (%rcx)
// 0x1421ae42c  0f8594000000                   jne	0x1421ae4c6 <.text+0x21ad4c6>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F65 -> F66 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae432  48c701ffffffff                 movq	$-0x1, (%rcx)
// 0x1421ae439  48898df8020000                 movq	%rcx, 0x2f8(%rbp)
// Frames: F74 -> F75 -> F66 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae440  4883c108                       addq	$0x8, %rcx
// Frames: F83 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae444  488b95e8020000                 movq	0x2e8(%rbp), %rdx
// 0x1421ae44b  e8a08ca6fe                     callq	0x140c170f0 <.text+0xc160f0>
// Frames: F69 -> F70 -> F71 -> F72 -> F77 -> F78 -> F78 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae450  488b85f8020000                 movq	0x2f8(%rbp), %rax
// 0x1421ae457  48ff00                         incq	(%rax)
// Frames: F81
// 0x1421ae45a  0f28b510030000                 movaps	0x310(%rbp), %xmm6
// 0x1421ae461  0f28bd20030000                 movaps	0x320(%rbp), %xmm7
// 0x1421ae468  440f288530030000               movaps	0x330(%rbp), %xmm8
// 0x1421ae470  440f288d40030000               movaps	0x340(%rbp), %xmm9
// 0x1421ae478  4881c4d8030000                 addq	$0x3d8, %rsp            # imm = 0x3D8
// 0x1421ae47f  5b                             popq	%rbx
// 0x1421ae480  5f                             popq	%rdi
// 0x1421ae481  5e                             popq	%rsi
// 0x1421ae482  415c                           popq	%r12
// 0x1421ae484  415d                           popq	%r13
// 0x1421ae486  415e                           popq	%r14
// 0x1421ae488  415f                           popq	%r15
// 0x1421ae48a  5d                             popq	%rbp
// 0x1421ae48b  c3                             retq
// Frames: F62 -> F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae48c  4885c0                         testq	%rax, %rax
// 0x1421ae48f  7529                           jne	0x1421ae4ba <.text+0x21ad4ba>
// Frames: F264 -> F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae491  31d2                           xorl	%edx, %edx
// 0x1421ae493  e8a89e7600                     callq	0x142918340 <.text+0x2917340>
// 0x1421ae498  4889c1                         movq	%rax, %rcx
// Frames: F265 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae49b  4885c0                         testq	%rax, %rax
// 0x1421ae49e  7588                           jne	0x1421ae428 <.text+0x21ad428>
// 0x1421ae4a0  eb18                           jmp	0x1421ae4ba <.text+0x21ad4ba>
// Frames: F62 -> F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae4a2  4885c0                         testq	%rax, %rax
// 0x1421ae4a5  7513                           jne	0x1421ae4ba <.text+0x21ad4ba>
// Frames: F264 -> F56 -> F57 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae4a7  31d2                           xorl	%edx, %edx
// 0x1421ae4a9  e8929e7600                     callq	0x142918340 <.text+0x2917340>
// 0x1421ae4ae  4889c1                         movq	%rax, %rcx
// Frames: F265 -> F58 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae4b1  4885c0                         testq	%rax, %rax
// 0x1421ae4b4  0f85caf2ffff                   jne	0x1421ad784 <.text+0x21ac784>
// 0x1421ae4ba  488d0d2780e400                 leaq	0xe48027(%rip), %rcx    # 0x142ff64e8
// 0x1421ae4c1  e80a9c7200                     callq	0x1428d80d0 <.text+0x28d70d0>
// 0x1421ae4c6  488d0d8b80e400                 leaq	0xe4808b(%rip), %rcx    # 0x142ff6558
// 0x1421ae4cd  e87ead7200                     callq	0x1428d9250 <.text+0x28d8250>
// Frames: F284 -> F90 -> F87 -> F88
// 0x1421ae4d2  48c1e220                       shlq	$0x20, %rdx
// 0x1421ae4d6  4c09ca                         orq	%r9, %rdx
// 0x1421ae4d9  41b901000000                   movl	$0x1, %r9d
// 0x1421ae4df  4889d0                         movq	%rdx, %rax
// Frames: F285 -> F88
// 0x1421ae4e2  44898de0010000                 movl	%r9d, 0x1e0(%rbp)
// 0x1421ae4e9  448985e4010000                 movl	%r8d, 0x1e4(%rbp)
// 0x1421ae4f0  488985e8010000                 movq	%rax, 0x1e8(%rbp)
// 0x1421ae4f7  488d050297e400                 leaq	0xe49702(%rip), %rax    # 0x142ff7c00
// 0x1421ae4fe  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421ae503  488d0d0694e400                 leaq	0xe49406(%rip), %rcx    # 0x142ff7910
// 0x1421ae50a  4c8d0ddf93e400                 leaq	0xe493df(%rip), %r9     # 0x142ff78f0
// 0x1421ae511  4c8d85e0010000                 leaq	0x1e0(%rbp), %r8
// 0x1421ae518  ba2b000000                     movl	$0x2b, %edx
// 0x1421ae51d  e83eb27200                     callq	0x1428d9760 <.text+0x28d8760>
// 0x1421ae522  eb43                           jmp	0x1421ae567 <.text+0x21ad567>
// Frames: F286 -> F190 -> F191 -> F184
// 0x1421ae524  b910000000                     movl	$0x10, %ecx
// 0x1421ae529  ba30000000                     movl	$0x30, %edx
// 0x1421ae52e  e8cdaa7200                     callq	0x1428d9000 <.text+0x28d8000>
// 0x1421ae533  eb32                           jmp	0x1421ae567 <.text+0x21ad567>
// Frames: F287 -> F226
// 0x1421ae535  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421ae53c  488d0d8597e400                 leaq	0xe49785(%rip), %rcx    # 0x142ff7cc8
// 0x1421ae543  e8c8ad7200                     callq	0x1428d9310 <.text+0x28d8310>
// 0x1421ae548  eb1d                           jmp	0x1421ae567 <.text+0x21ad567>
// Frames: F286 -> F190 -> F191 -> F256
// 0x1421ae54a  b908000000                     movl	$0x8, %ecx
// 0x1421ae54f  ba50000000                     movl	$0x50, %edx
// 0x1421ae554  e8a7aa7200                     callq	0x1428d9000 <.text+0x28d8000>
// 0x1421ae559  eb0c                           jmp	0x1421ae567 <.text+0x21ad567>
// Frames: F287 -> F261
// 0x1421ae55b  488d0d9697e400                 leaq	0xe49796(%rip), %rcx    # 0x142ff7cf8
// 0x1421ae562  e8a9ad7200                     callq	0x1428d9310 <.text+0x28d8310>
// 0x1421ae567  0f0b                           ud2
// 0x1421ae569  0f1f8000000000                 nopl	(%rax)
// 0x1421ae570  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ae575  55                             pushq	%rbp
// 0x1421ae576  4157                           pushq	%r15
// 0x1421ae578  4156                           pushq	%r14
// 0x1421ae57a  4155                           pushq	%r13
// 0x1421ae57c  4154                           pushq	%r12
// 0x1421ae57e  56                             pushq	%rsi
// 0x1421ae57f  57                             pushq	%rdi
// 0x1421ae580  53                             pushq	%rbx
// 0x1421ae581  4883ec78                       subq	$0x78, %rsp
// 0x1421ae585  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ae58c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x1421ae592  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x1421ae598  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1421ae59d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// 0x1421ae5a2  488b85f8020000                 movq	0x2f8(%rbp), %rax
// Frames: F69 -> F70 -> F71 -> F72 -> F77 -> F78 -> F78 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae5a9  48ff00                         incq	(%rax)
// 0x1421ae5ac  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae5b1  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae5b6  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae5bc  440f284c2430                   movaps	0x30(%rsp), %xmm9
// Frames: F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae5c2  4883c478                       addq	$0x78, %rsp
// 0x1421ae5c6  5b                             popq	%rbx
// 0x1421ae5c7  5f                             popq	%rdi
// 0x1421ae5c8  5e                             popq	%rsi
// 0x1421ae5c9  415c                           popq	%r12
// 0x1421ae5cb  415d                           popq	%r13
// 0x1421ae5cd  415e                           popq	%r14
// 0x1421ae5cf  415f                           popq	%r15
// 0x1421ae5d1  5d                             popq	%rbp
// 0x1421ae5d2  c3                             retq
// 0x1421ae5d3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1421ae5e0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ae5e5  55                             pushq	%rbp
// 0x1421ae5e6  4157                           pushq	%r15
// 0x1421ae5e8  4156                           pushq	%r14
// 0x1421ae5ea  4155                           pushq	%r13
// 0x1421ae5ec  4154                           pushq	%r12
// 0x1421ae5ee  56                             pushq	%rsi
// 0x1421ae5ef  57                             pushq	%rdi
// 0x1421ae5f0  53                             pushq	%rbx
// 0x1421ae5f1  4883ec78                       subq	$0x78, %rsp
// 0x1421ae5f5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ae5fc  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x1421ae602  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x1421ae608  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1421ae60d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// 0x1421ae612  488b85f8020000                 movq	0x2f8(%rbp), %rax
// Frames: F69 -> F70 -> F71 -> F72 -> F77 -> F78 -> F78 -> F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae619  48ff00                         incq	(%rax)
// 0x1421ae61c  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae621  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae626  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae62c  440f284c2430                   movaps	0x30(%rsp), %xmm9
// Frames: F67 -> F68 -> F59 -> F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae632  4883c478                       addq	$0x78, %rsp
// 0x1421ae636  5b                             popq	%rbx
// 0x1421ae637  5f                             popq	%rdi
// 0x1421ae638  5e                             popq	%rsi
// 0x1421ae639  415c                           popq	%r12
// 0x1421ae63b  415d                           popq	%r13
// 0x1421ae63d  415e                           popq	%r14
// 0x1421ae63f  415f                           popq	%r15
// 0x1421ae641  5d                             popq	%rbp
// 0x1421ae642  c3                             retq
// 0x1421ae643  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1421ae650  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ae655  55                             pushq	%rbp
// 0x1421ae656  4157                           pushq	%r15
// 0x1421ae658  4156                           pushq	%r14
// 0x1421ae65a  4155                           pushq	%r13
// 0x1421ae65c  4154                           pushq	%r12
// 0x1421ae65e  56                             pushq	%rsi
// 0x1421ae65f  57                             pushq	%rdi
// 0x1421ae660  53                             pushq	%rbx
// 0x1421ae661  4883ec78                       subq	$0x78, %rsp
// 0x1421ae665  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ae66c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x1421ae672  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x1421ae678  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1421ae67d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// 0x1421ae682  488b85e8020000                 movq	0x2e8(%rbp), %rax
// Frames: F69 -> F70 -> F71 -> F72 -> F77 -> F78 -> F78 -> F67 -> F68 -> F59 -> F60 -> F61 -> F3
// 0x1421ae689  48ff00                         incq	(%rax)
// 0x1421ae68c  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae691  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae696  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae69c  440f284c2430                   movaps	0x30(%rsp), %xmm9
// Frames: F67 -> F68 -> F59 -> F60 -> F61 -> F3
// 0x1421ae6a2  4883c478                       addq	$0x78, %rsp
// 0x1421ae6a6  5b                             popq	%rbx
// 0x1421ae6a7  5f                             popq	%rdi
// 0x1421ae6a8  5e                             popq	%rsi
// 0x1421ae6a9  415c                           popq	%r12
// 0x1421ae6ab  415d                           popq	%r13
// 0x1421ae6ad  415e                           popq	%r14
// 0x1421ae6af  415f                           popq	%r15
// 0x1421ae6b1  5d                             popq	%rbp
// 0x1421ae6b2  c3                             retq
// 0x1421ae6b3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1421ae6c0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ae6c5  55                             pushq	%rbp
// 0x1421ae6c6  4157                           pushq	%r15
// 0x1421ae6c8  4156                           pushq	%r14
// 0x1421ae6ca  4155                           pushq	%r13
// 0x1421ae6cc  4154                           pushq	%r12
// 0x1421ae6ce  56                             pushq	%rsi
// 0x1421ae6cf  57                             pushq	%rdi
// 0x1421ae6d0  53                             pushq	%rbx
// 0x1421ae6d1  4883ec78                       subq	$0x78, %rsp
// 0x1421ae6d5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ae6dc  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x1421ae6e2  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x1421ae6e8  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1421ae6ed  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F194 -> F195 -> F196 -> F78 -> F78 -> F78 -> F197
// 0x1421ae6f2  4883bde002000000               cmpq	$0x0, 0x2e0(%rbp)
// 0x1421ae6fa  7419                           je	0x1421ae715 <.text+0x21ad715>
// Frames: F198 -> F199 -> F200 -> F196 -> F78 -> F78 -> F78 -> F197
// 0x1421ae6fc  41b801000000                   movl	$0x1, %r8d
// 0x1421ae702  488b8da8020000                 movq	0x2a8(%rbp), %rcx
// 0x1421ae709  488b95e0020000                 movq	0x2e0(%rbp), %rdx
// 0x1421ae710  e80b5546fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F197
// 0x1421ae715  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421ae71c  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae721  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae726  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae72c  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x1421ae732  4883c478                       addq	$0x78, %rsp
// 0x1421ae736  5b                             popq	%rbx
// 0x1421ae737  5f                             popq	%rdi
// 0x1421ae738  5e                             popq	%rsi
// 0x1421ae739  415c                           popq	%r12
// 0x1421ae73b  415d                           popq	%r13
// 0x1421ae73d  415e                           popq	%r14
// 0x1421ae73f  415f                           popq	%r15
// 0x1421ae741  5d                             popq	%rbp
// 0x1421ae742  c3                             retq
// 0x1421ae743  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1421ae750  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ae755  55                             pushq	%rbp
// 0x1421ae756  4157                           pushq	%r15
// 0x1421ae758  4156                           pushq	%r14
// 0x1421ae75a  4155                           pushq	%r13
// 0x1421ae75c  4154                           pushq	%r12
// 0x1421ae75e  56                             pushq	%rsi
// 0x1421ae75f  57                             pushq	%rdi
// 0x1421ae760  53                             pushq	%rbx
// 0x1421ae761  4883ec78                       subq	$0x78, %rsp
// 0x1421ae765  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ae76c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x1421ae772  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x1421ae778  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1421ae77d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F288
// 0x1421ae782  80bd0603000000                 cmpb	$0x0, 0x306(%rbp)
// 0x1421ae789  744c                           je	0x1421ae7d7 <.text+0x21ad7d7>
// Frames: F195 -> F196 -> F78 -> F78 -> F78 -> F288
// 0x1421ae78b  488b95b0020000                 movq	0x2b0(%rbp), %rdx
// Frames: F194 -> F195 -> F196 -> F78 -> F78 -> F78 -> F288
// 0x1421ae792  4885d2                         testq	%rdx, %rdx
// 0x1421ae795  7412                           je	0x1421ae7a9 <.text+0x21ad7a9>
// Frames: F195 -> F196 -> F78 -> F78 -> F78 -> F288
// 0x1421ae797  488b8db8020000                 movq	0x2b8(%rbp), %rcx
// Frames: F198 -> F199 -> F200 -> F196 -> F78 -> F78 -> F78 -> F288
// 0x1421ae79e  41b801000000                   movl	$0x1, %r8d
// 0x1421ae7a4  e8775446fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F288
// 0x1421ae7a9  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421ae7b0  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae7b5  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae7ba  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae7c0  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x1421ae7c6  4883c478                       addq	$0x78, %rsp
// 0x1421ae7ca  5b                             popq	%rbx
// 0x1421ae7cb  5f                             popq	%rdi
// 0x1421ae7cc  5e                             popq	%rsi
// 0x1421ae7cd  415c                           popq	%r12
// 0x1421ae7cf  415d                           popq	%r13
// 0x1421ae7d1  415e                           popq	%r14
// 0x1421ae7d3  415f                           popq	%r15
// 0x1421ae7d5  5d                             popq	%rbp
// 0x1421ae7d6  c3                             retq
// 0x1421ae7d7  c6850703000000                 movb	$0x0, 0x307(%rbp)
// 0x1421ae7de  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae7e3  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae7e8  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae7ee  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x1421ae7f4  4883c478                       addq	$0x78, %rsp
// 0x1421ae7f8  5b                             popq	%rbx
// 0x1421ae7f9  5f                             popq	%rdi
// 0x1421ae7fa  5e                             popq	%rsi
// 0x1421ae7fb  415c                           popq	%r12
// 0x1421ae7fd  415d                           popq	%r13
// 0x1421ae7ff  415e                           popq	%r14
// 0x1421ae801  415f                           popq	%r15
// 0x1421ae803  5d                             popq	%rbp
// 0x1421ae804  c3                             retq
// 0x1421ae805  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x1421ae810  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ae815  55                             pushq	%rbp
// 0x1421ae816  4157                           pushq	%r15
// 0x1421ae818  4156                           pushq	%r14
// 0x1421ae81a  4155                           pushq	%r13
// 0x1421ae81c  4154                           pushq	%r12
// 0x1421ae81e  56                             pushq	%rsi
// 0x1421ae81f  57                             pushq	%rdi
// 0x1421ae820  53                             pushq	%rbx
// 0x1421ae821  4883ec78                       subq	$0x78, %rsp
// 0x1421ae825  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ae82c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x1421ae832  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x1421ae838  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1421ae83d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F289
// 0x1421ae842  80bd0703000000                 cmpb	$0x0, 0x307(%rbp)
// 0x1421ae849  744d                           je	0x1421ae898 <.text+0x21ad898>
// Frames: F195 -> F196 -> F78 -> F78 -> F289
// 0x1421ae84b  488b8580010000                 movq	0x180(%rbp), %rax
// Frames: F194 -> F195 -> F196 -> F78 -> F78 -> F289
// 0x1421ae852  4885c0                         testq	%rax, %rax
// 0x1421ae855  741a                           je	0x1421ae871 <.text+0x21ad871>
// Frames: F195 -> F196 -> F78 -> F78 -> F289
// 0x1421ae857  488b8d88010000                 movq	0x188(%rbp), %rcx
// Frames: F290 -> F291 -> F195 -> F196 -> F78 -> F78 -> F289
// 0x1421ae85e  48c1e002                       shlq	$0x2, %rax
// 0x1421ae862  488d14c0                       leaq	(%rax,%rax,8), %rdx
// Frames: F198 -> F199 -> F200 -> F196 -> F78 -> F78 -> F289
// 0x1421ae866  41b804000000                   movl	$0x4, %r8d
// 0x1421ae86c  e8af5346fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F289
// 0x1421ae871  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae876  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae87b  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae881  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x1421ae887  4883c478                       addq	$0x78, %rsp
// 0x1421ae88b  5b                             popq	%rbx
// 0x1421ae88c  5f                             popq	%rdi
// 0x1421ae88d  5e                             popq	%rsi
// 0x1421ae88e  415c                           popq	%r12
// 0x1421ae890  415d                           popq	%r13
// 0x1421ae892  415e                           popq	%r14
// 0x1421ae894  415f                           popq	%r15
// 0x1421ae896  5d                             popq	%rbp
// 0x1421ae897  c3                             retq
// 0x1421ae898  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae89d  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae8a2  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae8a8  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x1421ae8ae  4883c478                       addq	$0x78, %rsp
// 0x1421ae8b2  5b                             popq	%rbx
// 0x1421ae8b3  5f                             popq	%rdi
// 0x1421ae8b4  5e                             popq	%rsi
// 0x1421ae8b5  415c                           popq	%r12
// 0x1421ae8b7  415d                           popq	%r13
// 0x1421ae8b9  415e                           popq	%r14
// 0x1421ae8bb  415f                           popq	%r15
// 0x1421ae8bd  5d                             popq	%rbp
// 0x1421ae8be  c3                             retq
// 0x1421ae8bf  90                             nop
// 0x1421ae8c0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ae8c5  55                             pushq	%rbp
// 0x1421ae8c6  4157                           pushq	%r15
// 0x1421ae8c8  4156                           pushq	%r14
// 0x1421ae8ca  4155                           pushq	%r13
// 0x1421ae8cc  4154                           pushq	%r12
// 0x1421ae8ce  56                             pushq	%rsi
// 0x1421ae8cf  57                             pushq	%rdi
// 0x1421ae8d0  53                             pushq	%rbx
// 0x1421ae8d1  4883ec78                       subq	$0x78, %rsp
// 0x1421ae8d5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ae8dc  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x1421ae8e2  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x1421ae8e8  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1421ae8ed  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F78 -> F81
// 0x1421ae8f2  80bdf002000000                 cmpb	$0x0, 0x2f0(%rbp)
// 0x1421ae8f9  742f                           je	0x1421ae92a <.text+0x21ad92a>
// 0x1421ae8fb  488d8568010000                 leaq	0x168(%rbp), %rax
// 0x1421ae902  488985c0010000                 movq	%rax, 0x1c0(%rbp)
// 0x1421ae909  488d85c0010000                 leaq	0x1c0(%rbp), %rax
// 0x1421ae910  488985b8010000                 movq	%rax, 0x1b8(%rbp)
// Frames: F60 -> F82 -> F78 -> F78 -> F81
// 0x1421ae917  488d0de290e400                 leaq	0xe490e2(%rip), %rcx    # 0x142ff7a00
// 0x1421ae91e  488d95b8010000                 leaq	0x1b8(%rbp), %rdx
// 0x1421ae925  e8b61affff                     callq	0x1421a03e0 <.text+0x219f3e0>
// Frames: F1
// 0x1421ae92a  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421ae92f  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1421ae934  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x1421ae93a  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x1421ae940  4883c478                       addq	$0x78, %rsp
// 0x1421ae944  5b                             popq	%rbx
// 0x1421ae945  5f                             popq	%rdi
// 0x1421ae946  5e                             popq	%rsi
// 0x1421ae947  415c                           popq	%r12
// 0x1421ae949  415d                           popq	%r13
// 0x1421ae94b  415e                           popq	%r14
// 0x1421ae94d  415f                           popq	%r15
// 0x1421ae94f  5d                             popq	%rbp
// 0x1421ae950  c3                             retq
//
// RANGE rva_21ae960_len_1d9: RVA 0x21ae960, 473 PDB bytes; 109 instructions
// Evidence: _assembly/roof-v1/B/pack-002/rva_21ae960_len_1d9.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3470:13160: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// Frames: F292
// 0x1421ae960  55                             pushq	%rbp
// 0x1421ae961  4156                           pushq	%r14
// 0x1421ae963  56                             pushq	%rsi
// 0x1421ae964  57                             pushq	%rdi
// 0x1421ae965  53                             pushq	%rbx
// 0x1421ae966  4881ec80010000                 subq	$0x180, %rsp            # imm = 0x180
// 0x1421ae96d  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1421ae975  48c785f8000000feffffff         movq	$-0x2, 0xf8(%rbp)
// 0x1421ae980  4c89cf                         movq	%r9, %rdi
// 0x1421ae983  4c89c3                         movq	%r8, %rbx
// 0x1421ae986  4989d6                         movq	%rdx, %r14
// 0x1421ae989  4889ce                         movq	%rcx, %rsi
// Frames: F185 -> F186 -> F187 -> F188 -> F189 -> F293
// 0x1421ae98c  0fb60576624801                 movzbl	0x1486276(%rip), %eax   # 0x143634c09
// Frames: F192 -> F187 -> F188 -> F189 -> F293
// 0x1421ae993  b908000000                     movl	$0x8, %ecx
// 0x1421ae998  ba08000000                     movl	$0x8, %edx
// 0x1421ae99d  e86e5246fe                     callq	0x140613c10 <.text+0x612c10>
// Frames: F189 -> F293
// 0x1421ae9a2  4885c0                         testq	%rax, %rax
// 0x1421ae9a5  0f842e010000                   je	0x1421aead9 <.text+0x21adad9>
// Frames: F192 -> F187 -> F188 -> F294 -> F295 -> F296 -> F297 -> F298 -> F299 -> F300 -> F301 -> F302 -> F303 -> F304 -> F223 -> F305
// 0x1421ae9ab  488985f0000000                 movq	%rax, 0xf0(%rbp)
// 0x1421ae9b2  0fb60550624801                 movzbl	0x1486250(%rip), %eax   # 0x143634c09
// 0x1421ae9b9  b91a000000                     movl	$0x1a, %ecx
// 0x1421ae9be  ba01000000                     movl	$0x1, %edx
// 0x1421ae9c3  e8485246fe                     callq	0x140613c10 <.text+0x612c10>
// Frames: F306 -> F295 -> F296 -> F297 -> F298 -> F299 -> F300 -> F301 -> F302 -> F303 -> F304 -> F223 -> F305
// 0x1421ae9c8  4885c0                         testq	%rax, %rax
// 0x1421ae9cb  0f8417010000                   je	0x1421aeae8 <.text+0x21adae8>
// Frames: F307 -> F308 -> F309 -> F299 -> F300 -> F301 -> F302 -> F303 -> F304 -> F223 -> F305
// 0x1421ae9d1  0f10054293e400                 movups	0xe49342(%rip), %xmm0   # 0x142ff7d1a
// 0x1421ae9d8  0f11400a                       movups	%xmm0, 0xa(%rax)
// 0x1421ae9dc  0f10052d93e400                 movups	0xe4932d(%rip), %xmm0   # 0x142ff7d10
// 0x1421ae9e3  0f1100                         movups	%xmm0, (%rax)
// Frames: F310
// 0x1421ae9e6  48c745481a000000               movq	$0x1a, 0x48(%rbp)
// 0x1421ae9ee  48894550                       movq	%rax, 0x50(%rbp)
// 0x1421ae9f2  48c745581a000000               movq	$0x1a, 0x58(%rbp)
// 0x1421ae9fa  48b80000000000000080           movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
// 0x1421aea04  48894540                       movq	%rax, 0x40(%rbp)
// 0x1421aea08  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x1421aea0c  488d5540                       leaq	0x40(%rbp), %rdx
// 0x1421aea10  e89bef6efe                     callq	0x14089d9b0 <.text+0x89c9b0>
// 0x1421aea15  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x1421aea19  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x1421aea1d  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421aea20  e86bf26efe                     callq	0x14089dc90 <.text+0x89cc90>
// Frames: F311
// 0x1421aea25  410f2806                       movaps	(%r14), %xmm0
// Frames: F310
// 0x1421aea29  0f2985d0000000                 movaps	%xmm0, 0xd0(%rbp)
// 0x1421aea30  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x1421aea34  488d5540                       leaq	0x40(%rbp), %rdx
// 0x1421aea38  4c8d85d0000000                 leaq	0xd0(%rbp), %r8
// 0x1421aea3f  e8ecf16efe                     callq	0x14089dc30 <.text+0x89cc30>
// Frames: F312
// 0x1421aea44  f3410f105610                   movss	0x10(%r14), %xmm2
// 0x1421aea4a  f30f59157e6e7700               mulss	0x776e7e(%rip), %xmm2   # 0x1429258d0
// Frames: F310
// 0x1421aea52  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x1421aea56  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x1421aea5a  e801f26efe                     callq	0x14089dc60 <.text+0x89cc60>
// Frames: F313
// 0x1421aea5f  418b461c                       movl	0x1c(%r14), %eax
// 0x1421aea63  8985e8000000                   movl	%eax, 0xe8(%rbp)
// 0x1421aea69  498b4614                       movq	0x14(%r14), %rax
// 0x1421aea6d  488985e0000000                 movq	%rax, 0xe0(%rbp)
// Frames: F310
// 0x1421aea74  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x1421aea78  488d5540                       leaq	0x40(%rbp), %rdx
// 0x1421aea7c  4c8d85e0000000                 leaq	0xe0(%rbp), %r8
// 0x1421aea83  e878f16efe                     callq	0x14089dc00 <.text+0x89cc00>
// 0x1421aea88  488b8550010000                 movq	0x150(%rbp), %rax
// 0x1421aea8f  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421aea94  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x1421aea98  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x1421aea9c  4989d8                         movq	%rbx, %r8
// 0x1421aea9f  4989f9                         movq	%rdi, %r9
// 0x1421aeaa2  e8a9f26efe                     callq	0x14089dd50 <.text+0x89cd50>
// 0x1421aeaa7  488b4560                       movq	0x60(%rbp), %rax
// 0x1421aeaab  488b8df0000000                 movq	0xf0(%rbp), %rcx
// Frames: F293
// 0x1421aeab2  488901                         movq	%rax, (%rcx)
// Frames: F314 -> F315 -> F316 -> F293
// 0x1421aeab5  48c70601000000                 movq	$0x1, (%rsi)
// 0x1421aeabc  48894e08                       movq	%rcx, 0x8(%rsi)
// 0x1421aeac0  48c7461001000000               movq	$0x1, 0x10(%rsi)
// Frames: F317
// 0x1421aeac8  4889f0                         movq	%rsi, %rax
// 0x1421aeacb  4881c480010000                 addq	$0x180, %rsp            # imm = 0x180
// 0x1421aead2  5b                             popq	%rbx
// 0x1421aead3  5f                             popq	%rdi
// 0x1421aead4  5e                             popq	%rsi
// 0x1421aead5  415e                           popq	%r14
// 0x1421aead7  5d                             popq	%rbp
// 0x1421aead8  c3                             retq
// Frames: F286 -> F293
// 0x1421aead9  b908000000                     movl	$0x8, %ecx
// 0x1421aeade  ba08000000                     movl	$0x8, %edx
// 0x1421aeae3  e818a57200                     callq	0x1428d9000 <.text+0x28d8000>
// Frames: F318 -> F296 -> F297 -> F298 -> F299 -> F300 -> F301 -> F302 -> F303 -> F304 -> F223 -> F305
// 0x1421aeae8  4c8d05998ee400                 leaq	0xe48e99(%rip), %r8     # 0x142ff7988
// 0x1421aeaef  b901000000                     movl	$0x1, %ecx
// 0x1421aeaf4  ba1a000000                     movl	$0x1a, %edx
// 0x1421aeaf9  e8e5a47200                     callq	0x1428d8fe3 <.text+0x28d7fe3>
// 0x1421aeafe  0f0b                           ud2
// 0x1421aeb00  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421aeb05  55                             pushq	%rbp
// 0x1421aeb06  4156                           pushq	%r14
// 0x1421aeb08  56                             pushq	%rsi
// 0x1421aeb09  57                             pushq	%rdi
// 0x1421aeb0a  53                             pushq	%rbx
// 0x1421aeb0b  4883ec30                       subq	$0x30, %rsp
// 0x1421aeb0f  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F198 -> F199 -> F319 -> F78 -> F293
// 0x1421aeb16  ba08000000                     movl	$0x8, %edx
// 0x1421aeb1b  41b808000000                   movl	$0x8, %r8d
// 0x1421aeb21  488b8df0000000                 movq	0xf0(%rbp), %rcx
// 0x1421aeb28  e8f35046fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F292
// 0x1421aeb2d  90                             nop
// 0x1421aeb2e  4883c430                       addq	$0x30, %rsp
// 0x1421aeb32  5b                             popq	%rbx
// 0x1421aeb33  5f                             popq	%rdi
// 0x1421aeb34  5e                             popq	%rsi
// 0x1421aeb35  415e                           popq	%r14
// 0x1421aeb37  5d                             popq	%rbp
// 0x1421aeb38  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:41:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F2: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:54:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F3: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:72:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F5: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:371:0 core::str::impl$0::is_char_boundary
// F6: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:434:0 core::str::traits::impl$10::get
// F7: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:465:0 core::str::traits::impl$10::index
// F8: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:60:0 core::str::traits::impl$4::index
// F9: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F10: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:375:0 core::str::impl$0::is_char_boundary
// F11: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:387:0 core::str::impl$0::is_char_boundary
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/mod.rs:1046:0 core::num::impl$6::is_utf8_char_boundary
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:467:0 core::str::traits::impl$10::index
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:502:0 core::str::traits::impl$11::get
// F15: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:535:0 core::str::traits::impl$11::index
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:537:0 core::str::traits::impl$11::index
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:211:0 core::str::traits::impl$7::get_unchecked
// F19: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:524:0 core::str::traits::impl$11::get_unchecked
// F20: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:505:0 core::str::traits::impl$11::get
// F21: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:967:0 core::ptr::const_ptr::impl$0::add
// F22: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:212:0 core::str::traits::impl$7::get_unchecked
// F23: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:85:0 core::str::validations::next_code_point_reverse
// F24: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/iter.rs:131:0 core::str::iter::impl$2::next_back
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/iter.rs:219:0 core::str::iter::impl$6::next_back
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:714:0 core::str::pattern::impl$11::next_back
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:317:0 core::str::pattern::ReverseSearcher::next_match_back
// F28: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:799:0 core::str::pattern::impl$22::next_match_back
// F29: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:1492:0 core::str::impl$0::rfind
// F30: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F31: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:285:0 core::slice::iter::impl$182::any
// F32: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:660:0 core::str::pattern::impl$8::matches
// F33: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:717:0 core::str::pattern::impl$11::next_back
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:94:0 core::str::validations::next_code_point_reverse
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:25:0 core::str::validations::utf8_is_cont_byte
// F36: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:96:0 core::str::validations::next_code_point_reverse
// F37: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:99:0 core::str::validations::next_code_point_reverse
// F38: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:101:0 core::str::validations::next_code_point_reverse
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:541:0 core::ptr::non_null::NonNull::offset
// F40: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:697:0 core::ptr::non_null::NonNull::sub
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:131:0 core::slice::iter::Iter::pre_dec_end
// F42: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:83:0 core::slice::iter::Iter::next_back_unchecked
// F43: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:401:0 core::slice::iter::impl$183::next_back
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:12:0 core::str::validations::utf8_first_byte
// F46: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F47: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:18:0 core::str::validations::utf8_acc_cont_byte
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:106:0 core::str::validations::next_code_point_reverse
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:108:0 core::str::validations::next_code_point_reverse
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:110:0 core::str::validations::next_code_point_reverse
// F51: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:1623:0 core::ptr::non_null::impl$14::eq
// F52: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$183::next_back
// F53: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F54: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$183::next_back
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/mod.rs:94:0 puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
// F57: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F58: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:309:0 std::thread::local::LocalKey::try_with
// F59: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:274:0 std::thread::local::LocalKey::with
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F62: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:53:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F63: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:54:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F64: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1822:0 core::cell::BorrowRefMut::new
// F65: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1122:0 core::cell::RefCell::try_borrow_mut
// F66: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1090:0 core::cell::RefCell::borrow_mut
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F68: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:310:0 std::thread::local::LocalKey::try_with
// F69: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F70: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:870:0 core::mem::replace
// F71: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:503:0 core::cell::Cell::replace
// F72: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:429:0 core::cell::Cell::set
// F73: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1824:0 core::cell::BorrowRefMut::new
// F74: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:2198:0 core::cell::UnsafeCell::get
// F75: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1130:0 core::cell::RefCell::try_borrow_mut
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F77: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1811:0 core::cell::impl$44::drop
// F78: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:523:0 core::ptr::drop_in_place
// F79: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:74:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F80: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:75:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F81: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:268:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop::closure$0
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:640:0 bevy_ecs::system::query::Query::copy_unsafe
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:622:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:545:0 bevy_ecs::system::query::Query::as_readonly
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1386:0 bevy_ecs::system::query::Query::get
// F88: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:79:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:843:0 bevy_ecs::entity::Entities::get
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1595:0 bevy_ecs::system::query::Query::get_inner
// F91: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F92: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F93: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F94: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F95: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F96: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F97: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F98: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 core::slice::impl$0::get
// F99: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F100: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1742:0 core::cmp::impls::impl$24::ne
// F101: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:201:0 core::num::nonzero::impl$2::ne
// F102: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:844:0 bevy_ecs::entity::Entities::get
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:76:0 bevy_ecs::archetype::impl$25::eq
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:845:0 bevy_ecs::entity::Entities::get
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1603:0 bevy_ecs::system::query::Query::get_inner
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F110: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/bool.rs:62:0 core::bool::impl$0::then
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F112: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F113: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/internal_macros.rs:36:0 fixedbitset::FixedBitSet::get_unchecked
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1600:0 bevy_ecs::system::query::Query::get_inner
// F115: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1632:0 bevy_ecs::system::query::Query::get_inner
// F118: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 alloc::vec::impl$8::deref
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1634:0 bevy_ecs::system::query::Query::get_inner
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F122: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:367:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1181:0 bevy_ecs::query::fetch::impl$26::set_table
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1169:0 bevy_ecs::query::fetch::impl$26::set_archetype
// F126: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F127: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::slice::impl$0::get
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F130: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:304:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1239:0 bevy_ecs::query::fetch::impl$27::fetch::closure$0
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1234:0 bevy_ecs::query::fetch::impl$27::fetch
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1638:0 bevy_ecs::system::query::Query::get_inner
// F140: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:80:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F141: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:101:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F142: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:103:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F143: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1335:0 glam::f32::vec3::impl$34::add
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1337:0 glam::f32::vec3::impl$34::add
// F146: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:502:0 core::ops::arith::impl$241::div
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1107:0 glam::f32::vec3::impl$8::div
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1109:0 glam::f32::vec3::impl$8::div
// F149: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:106:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1334:0 glam::f32::vec3::impl$34::add
// F151: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:109:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F152: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:893:0 glam::f32::vec2::Vec2::perp
// F153: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:167:0 glam::f32::vec2::Vec2::dot
// F154: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:397:0 glam::f32::vec2::Vec2::length
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:416:0 glam::f32::vec2::Vec2::length_recip
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:468:0 glam::f32::vec2::Vec2::normalize
// F157: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:370:0 std::f32::impl$0::sqrt
// F158: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs:178:0 glam::f32::vec2::Vec2::dot
// F159: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:830:0 glam::f32::vec2::Vec2::length
// F160: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:344:0 core::ops::arith::impl$141::mul
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1200:0 glam::f32::vec2::Vec2::length_recip
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1201:0 glam::f32::vec2::Vec2::length_recip
// F163: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:116:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F164: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:115:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F165: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:111:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F166: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:122:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F167: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:125:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F168: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:126:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F169: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:128:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F170: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:131:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F171: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:132:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F172: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/fmt.rs:654:0 alloc::fmt::format::closure$0
// F173: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1225:0 core::option::Option::map_or_else
// F174: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/fmt.rs:654:0 alloc::fmt::format
// F175: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/hint.rs:600:0 core::hint::must_use
// F176: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:135:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F177: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:134:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F178: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:137:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F179: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:133:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F180: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:139:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F181: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:144:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F182: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:150:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F183: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:152:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F184: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:156:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F185: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1752:0 core::ptr::read_volatile
// F186: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:94:0 alloc::alloc::alloc
// F187: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:192:0 alloc::alloc::Global::alloc_impl
// F188: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:254:0 alloc::alloc::impl$1::allocate
// F189: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:349:0 alloc::alloc::exchange_malloc
// F190: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/boxed.rs:274:0 alloc::boxed::impl$0::new
// F191: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:389:0 alloc::sync::Arc::new
// F192: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:96:0 alloc::alloc::alloc
// F193: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:151:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F194: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:523:0 alloc::raw_vec::RawVecInner::current_memory
// F195: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:751:0 alloc::raw_vec::RawVecInner::deallocate
// F196: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:402:0 alloc::raw_vec::impl$3::drop
// F197: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:170:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F198: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:116:0 alloc::alloc::dealloc
// F199: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:269:0 alloc::alloc::impl$1::deallocate
// F200: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:753:0 alloc::raw_vec::RawVecInner::deallocate
// F201: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:173:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F202: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:175:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F203: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:89:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F204: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:180:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F205: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:90:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F206: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:93:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F207: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/range.rs:379:0 core::ops::range::RangeInclusive::new
// F208: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:92:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F209: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$77::sub
// F210: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1475:0 glam::f32::vec3::impl$50::sub
// F211: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:478:0 glam::f32::vec3::Vec3::distance
// F212: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1476:0 glam::f32::vec3::impl$50::sub
// F213: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1477:0 glam::f32::vec3::impl$50::sub
// F214: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:204:0 glam::f32::vec3::Vec3::dot
// F215: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:452:0 glam::f32::vec3::impl$50::sub
// F216: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs:178:0 glam::f32::vec3::Vec3::dot
// F217: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:183:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F218: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F219: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F220: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:185:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F221: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:437:0 bevy_ecs::change_detection::impl$38::deref_mut
// F222: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:186:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F223: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/convert/mod.rs:761:0 core::convert::impl$3::into
// F224: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:187:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F225: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:192:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F226: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:194:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F227: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:976:0 core::option::Option::unwrap
// F228: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:236:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F229: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:237:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F230: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:235:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F231: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:198:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F232: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:201:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F233: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1418:0 core::f32::impl$0::clamp
// F234: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:202:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F235: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1421:0 core::f32::impl$0::clamp
// F236: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:207:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F237: crates/systems/roof/src/ui_edit_roof.rs:245:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F238: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:206:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F239: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:210:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F240: crates/systems/roof/src/ui_edit_roof.rs:247:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F241: crates/systems/roof/src/ui_edit_roof.rs:249:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F242: crates/systems/roof/src/ui_edit_roof.rs:251:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F243: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:244:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F244: crates/country-core/src/resources/roofs/roof_shape.rs:314:0 country_core::resources::roofs::roof_shape::impl$43::eq
// F245: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:262:0 core::cmp::PartialEq::ne
// F246: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:245:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F247: crates/country-core/src/resources/roofs/roof_shape.rs:316:0 country_core::resources::roofs::roof_shape::impl$43::eq
// F248: crates/country-core/src/resources/roofs/roof_shape.rs:321:0 country_core::resources::roofs::roof_shape::impl$43::eq
// F249: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:22:0 glam::f32::vec2::impl$102::eq
// F250: crates/country-core/src/resources/roofs/roof_shape.rs:326:0 country_core::resources::roofs::roof_shape::impl$43::eq
// F251: crates/country-core/src/resources/roofs/roof_shape.rs:328:0 country_core::resources::roofs::roof_shape::impl$43::eq
// F252: crates/country-core/src/resources/roofs/roof_shape.rs:336:0 country_core::resources::roofs::roof_shape::impl$48::eq
// F253: crates/country-core/src/resources/roofs/roof_shape.rs:331:0 country_core::resources::roofs::roof_shape::impl$43::eq
// F254: crates/country-core/src/resources/roofs/roof_shape.rs:333:0 country_core::resources::roofs::roof_shape::impl$43::eq
// F255: crates/country-core/src/resources/roofs/roof_shape.rs:123:0 country_core::resources::roofs::roof_shape::impl$35::eq
// F256: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:246:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F257: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:247:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F258: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:249:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F259: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:253:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F260: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:252:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F261: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:260:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F262: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:261:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F263: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:262:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F264: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:56:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F265: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:299:0 core::ptr::const_ptr::impl$0::as_ref
// F266: crates/systems/roof/src/ui_edit_roof.rs:258:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F267: crates/systems/roof/src/ui_edit_roof.rs:259:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F268: crates/systems/roof/src/ui_edit_roof.rs:257:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F269: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:222:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F270: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs:75:0 bevy_ecs::event::writer::EventWriter::write
// F271: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:221:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F272: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2419:0 alloc::vec::Vec::push
// F273: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:139:0 bevy_ecs::event::collections::Events::send_with_caller
// F274: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::change_detection::impl$38::deref_mut
// F275: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:130:0 bevy_ecs::event::collections::Events::send_with_caller
// F276: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2422:0 alloc::vec::Vec::push
// F277: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2423:0 alloc::vec::Vec::push
// F278: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1734:0 alloc::vec::Vec::as_mut_ptr
// F279: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2426:0 alloc::vec::Vec::push
// F280: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 alloc::vec::Vec::as_mut_ptr
// F281: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2427:0 alloc::vec::Vec::push
// F282: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2428:0 alloc::vec::Vec::push
// F283: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:140:0 bevy_ecs::event::collections::Events::send_with_caller
// F284: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:2014:0 core::result::impl$27::from_residual
// F285: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1109:0 core::result::Result::unwrap
// F286: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:351:0 alloc::alloc::exchange_malloc
// F287: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:978:0 core::option::Option::unwrap
// F288: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:141:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F289: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:217:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h49e59ba96f924c86
// F290: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:945:0 core::num::impl$11::unchecked_mul
// F291: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:531:0 alloc::raw_vec::RawVecInner::current_memory
// F292: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:156:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F293: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:157:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F294: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:472:0 alloc::raw_vec::RawVecInner::try_allocate_in
// F295: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:418:0 alloc::raw_vec::RawVecInner::with_capacity_in
// F296: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:186:0 alloc::raw_vec::RawVec::with_capacity_in
// F297: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:803:0 alloc::vec::Vec::with_capacity_in
// F298: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:162:0 alloc::slice::hack::impl$1::to_vec
// F299: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:111:0 alloc::slice::hack::to_vec
// F300: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:505:0 alloc::slice::impl$0::to_vec_in
// F301: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:480:0 alloc::slice::impl$0::to_vec
// F302: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:888:0 alloc::slice::impl$9::to_owned
// F303: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/str.rs:211:0 alloc::str::impl$4::to_owned
// F304: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/string.rs:2957:0 alloc::string::impl$47::from
// F305: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:159:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F306: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:476:0 alloc::raw_vec::RawVecInner::try_allocate_in
// F307: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:4441:0 core::intrinsics::copy_nonoverlapping
// F308: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:1356:0 core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
// F309: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:167:0 alloc::slice::hack::impl$1::to_vec
// F310: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:158:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F311: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:162:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F312: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:163:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F313: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:164:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F314: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:935:0 alloc::vec::Vec::from_raw_parts_in
// F315: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:103:0 alloc::slice::hack::into_vec
// F316: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:528:0 alloc::slice::impl$0::into_vec
// F317: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:168:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F318: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:426:0 alloc::raw_vec::RawVecInner::with_capacity_in
// F319: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/boxed.rs:1684:0 alloc::boxed::impl$8::drop
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f95441868a2ee08b7a5ac7d84fe03f2574c1e6360ca921e78639e4812cb891a4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_edit_roof_height_focused.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3470
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
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3470,"record_offset":2728,"rva":35312704,"va":5404021824}
// 编译实例 RVAs: 0x21ad440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28942614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8ea5e46fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5404024097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28942775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8495e46fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5404024274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26155587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8bde570fe","size":5,"soft_fail":false,"target_va":5377869088,"va":5404024670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5680
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26117475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e89d7a71fe","size":5,"soft_fail":false,"target_va":5377906304,"va":5404023774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14092df10
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25689322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8160378fe","size":5,"soft_fail":false,"target_va":5378334480,"va":5404023797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a83f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24290207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8615c8dfe","size":5,"soft_fail":false,"target_va":5379735424,"va":5404025626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22142138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e84623aefe","size":5,"soft_fail":false,"target_va":5381883040,"va":5404025173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14123b320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16197766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e87ad708ff","size":5,"soft_fail":false,"target_va":5387825952,"va":5404023713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421a0470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-56118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8ca24ffff","size":5,"soft_fail":false,"target_va":5403968624,"va":5404024737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7514829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8cdaa7200","size":5,"soft_fail":false,"target_va":5411540992,"va":5404026158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-21183280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8d0c4bcfe","size":5,"soft_fail":false,"target_va":5382840208,"va":5404023483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-29670406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8fa433bfe","size":5,"soft_fail":false,"target_va":5374355376,"va":5404025777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7515518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e87ead7200","size":5,"soft_fail":false,"target_va":5411541584,"va":5404026061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7515592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8c8ad7200","size":5,"soft_fail":false,"target_va":5411541776,"va":5404026179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7516734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e83eb27200","size":5,"soft_fail":false,"target_va":5411542880,"va":5404026141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e842faffff","size":5,"soft_fail":false,"target_va":5404020496,"va":5404021961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7521317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e825c47200","size":5,"soft_fail":false,"target_va":5411543360,"va":5404022038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::pressed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25616257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e87f2079fe","size":5,"soft_fail":false,"target_va":5378407024,"va":5404023276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24286240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8e06b8dfe","size":5,"soft_fail":false,"target_va":5379739328,"va":5404025563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25997795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e81d4e73fe","size":5,"soft_fail":false,"target_va":5378025104,"va":5404022894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25997632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8c04e73fe","size":5,"soft_fail":false,"target_va":5378025296,"va":5404022923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23602111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e841dc97fe","size":5,"soft_fail":false,"target_va":5380422656,"va":5404024762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26557048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e888c56afe","size":5,"soft_fail":false,"target_va":5377467744,"va":5404024787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26555938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8dec96afe","size":5,"soft_fail":false,"target_va":5377469792,"va":5404025725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24229029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e85b4b8efe","size":5,"soft_fail":false,"target_va":5379793584,"va":5404022608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25687463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8590a78fe","size":5,"soft_fail":false,"target_va":5378336288,"va":5404023746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::boop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25184922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e866b57ffe","size":5,"soft_fail":false,"target_va":5378838400,"va":5404023317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25184853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8abb57ffe","size":5,"soft_fail":false,"target_va":5378838448,"va":5404023296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25184893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e883b57ffe","size":5,"soft_fail":false,"target_va":5378838480,"va":5404023368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25185547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8f5b27ffe","size":5,"soft_fail":false,"target_va":5378838032,"va":5404023574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25185449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e857b37ffe","size":5,"soft_fail":false,"target_va":5378838240,"va":5404023684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25185516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e814b37ffe","size":5,"soft_fail":false,"target_va":5378838144,"va":5404023655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26281168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e830fb6efe","size":5,"soft_fail":false,"target_va":5377743024,"va":5404024187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23596585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8d7f197fe","size":5,"soft_fail":false,"target_va":5380427664,"va":5404024244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23935216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e810c792fe","size":5,"soft_fail":false,"target_va":5380088032,"va":5404023243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24009249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8dfa591fe","size":5,"soft_fail":false,"target_va":5380015136,"va":5404024380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24009236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8eca591fe","size":5,"soft_fail":false,"target_va":5380015184,"va":5404024415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24009276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8c4a591fe","size":5,"soft_fail":false,"target_va":5380015264,"va":5404024535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22636967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e85996a6fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5404022498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22631778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e89eaaa6fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5404022413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22636220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e84499a6fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5404022695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22637241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e84795a6fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5404021940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7774220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e80ca07600","size":5,"soft_fail":false,"target_va":5411799872,"va":5404025647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-58698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8b61affff","size":5,"soft_fail":false,"target_va":5403968480,"va":5404027173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7511050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e80a9c7200","size":5,"soft_fail":false,"target_va":5411537104,"va":5404026049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_right
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22113042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e8ee94aefe","size":5,"soft_fail":false,"target_va":5381910000,"va":5404023037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22137218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404021824,"raw_hex":"e87e36aefe","size":5,"soft_fail":false,"target_va":5381885904,"va":5404023117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3470,"record_offset":8512,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3470,"record_offset":8736,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3470,"record_offset":9480,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3470,"record_offset":10728,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3470,"record_offset":6268,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3470,"record_offset":10204,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3470,"record_offset":9540,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3470,"record_offset":10000,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":3470,"record_offset":8596,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3470,"record_offset":10176,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3470,"record_offset":9624,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3470,"record_offset":5408,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3470,"record_offset":12296,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3470,"record_offset":8204,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3470,"record_offset":8176,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3470,"record_offset":9248,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3470,"record_offset":7912,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3470,"record_offset":8000,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3470,"record_offset":4424,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3470,"record_offset":2992,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3470,"record_offset":3448,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] system_roof::ui_edit_roof::update_wall_height_if_needed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/ui_edit_roof.rs
//     evidence={"module":3470,"record_offset":10088,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":93237,"source_line":239,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs
//     evidence={"module":3470,"record_offset":9180,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":116721,"source_line":88,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":8848,"parent_record_offset":8824,"ancestor_record_offset":2728,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":2728,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":2728,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":8792,"parent_record_offset":8764,"ancestor_record_offset":2728,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":8824,"parent_record_offset":8792,"ancestor_record_offset":2728,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":9104,"parent_record_offset":9048,"ancestor_record_offset":2728,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3470,"record_offset":8764,"parent_record_offset":8736,"ancestor_record_offset":2728,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3470,"record_offset":8560,"parent_record_offset":8536,"ancestor_record_offset":2728,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":2728,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":9076,"parent_record_offset":9048,"ancestor_record_offset":2728,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":2728,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":2728,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":2728,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":2728,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3470,"record_offset":10896,"parent_record_offset":10852,"ancestor_record_offset":2728,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3470,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":2728,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3470,"record_offset":6568,"parent_record_offset":6468,"ancestor_record_offset":2728,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3470,"record_offset":10852,"parent_record_offset":10816,"ancestor_record_offset":2728,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3470,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":2728,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3470,"record_offset":6768,"parent_record_offset":6436,"ancestor_record_offset":2728,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3470,"record_offset":9508,"parent_record_offset":9480,"ancestor_record_offset":2728,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3470,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":2728,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3470,"record_offset":6436,"parent_record_offset":6384,"ancestor_record_offset":2728,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3470,"record_offset":10816,"parent_record_offset":10752,"ancestor_record_offset":2728,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3470,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":2728,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3470,"record_offset":7240,"parent_record_offset":6384,"ancestor_record_offset":2728,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3470,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":2728,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3470,"record_offset":7736,"parent_record_offset":6384,"ancestor_record_offset":2728,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3470,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":2728,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3470,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":2728,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3470,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":2728,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3470,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":2728,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3470,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":2728,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3470,"record_offset":7316,"parent_record_offset":7288,"ancestor_record_offset":2728,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3470,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":2728,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3470,"record_offset":6980,"parent_record_offset":6384,"ancestor_record_offset":2728,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3470,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":2728,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3470,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":2728,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3470,"record_offset":6300,"parent_record_offset":6268,"ancestor_record_offset":2728,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3470,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":2728,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3470,"record_offset":6384,"parent_record_offset":6268,"ancestor_record_offset":2728,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3470,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":2728,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3470,"record_offset":7808,"parent_record_offset":7784,"ancestor_record_offset":2728,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3470,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":2728,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3470,"record_offset":4760,"parent_record_offset":4732,"ancestor_record_offset":2728,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3470,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":2728,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3470,"record_offset":4788,"parent_record_offset":4760,"ancestor_record_offset":2728,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3470,"record_offset":4708,"parent_record_offset":4680,"ancestor_record_offset":2728,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3470,"record_offset":4732,"parent_record_offset":4708,"ancestor_record_offset":2728,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3470,"record_offset":4904,"parent_record_offset":4732,"ancestor_record_offset":2728,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3470,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":2728,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3470,"record_offset":6736,"parent_record_offset":6712,"ancestor_record_offset":2728,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3470,"record_offset":10140,"parent_record_offset":10088,"ancestor_record_offset":2728,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3470,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":2728,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3470,"record_offset":12912,"parent_record_offset":12880,"ancestor_record_offset":2728,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3470,"record_offset":3148,"parent_record_offset":3116,"ancestor_record_offset":2728,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3470,"record_offset":6712,"parent_record_offset":6436,"ancestor_record_offset":2728,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3470,"record_offset":7940,"parent_record_offset":7912,"ancestor_record_offset":2728,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3470,"record_offset":8360,"parent_record_offset":8228,"ancestor_record_offset":2728,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3470,"record_offset":8028,"parent_record_offset":8000,"ancestor_record_offset":2728,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3470,"record_offset":9312,"parent_record_offset":9272,"ancestor_record_offset":2728,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3470,"record_offset":4576,"parent_record_offset":4532,"ancestor_record_offset":2728,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3470,"record_offset":9216,"parent_record_offset":9180,"ancestor_record_offset":2728,"inlinee":7576,"source_line":378,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3470,"record_offset":7480,"parent_record_offset":7456,"ancestor_record_offset":2728,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3470,"record_offset":7520,"parent_record_offset":7340,"ancestor_record_offset":2728,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3470,"record_offset":8536,"parent_record_offset":8512,"ancestor_record_offset":2728,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3470,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":2728,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3470,"record_offset":5364,"parent_record_offset":4532,"ancestor_record_offset":2728,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3470,"record_offset":4968,"parent_record_offset":4680,"ancestor_record_offset":2728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3470,"record_offset":6932,"parent_record_offset":6900,"ancestor_record_offset":2728,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3470,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":2728,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3470,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":2728,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3470,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":2728,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3470,"record_offset":8900,"parent_record_offset":8872,"ancestor_record_offset":2728,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3470,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":2728,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3470,"record_offset":7876,"parent_record_offset":6384,"ancestor_record_offset":2728,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3470,"record_offset":6468,"parent_record_offset":6436,"ancestor_record_offset":2728,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3470,"record_offset":6536,"parent_record_offset":6508,"ancestor_record_offset":2728,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3470,"record_offset":6508,"parent_record_offset":6468,"ancestor_record_offset":2728,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3470,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":2728,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3470,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":2728,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3470,"record_offset":4116,"parent_record_offset":4088,"ancestor_record_offset":2728,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3470,"record_offset":3764,"parent_record_offset":3648,"ancestor_record_offset":2728,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3470,"record_offset":3116,"parent_record_offset":3088,"ancestor_record_offset":2728,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3470,"record_offset":3476,"parent_record_offset":3448,"ancestor_record_offset":2728,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3470,"record_offset":3616,"parent_record_offset":3584,"ancestor_record_offset":2728,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3470,"record_offset":3584,"parent_record_offset":3548,"ancestor_record_offset":2728,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3470,"record_offset":3524,"parent_record_offset":3500,"ancestor_record_offset":2728,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3470,"record_offset":3548,"parent_record_offset":3524,"ancestor_record_offset":2728,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3470,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":2728,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3470,"record_offset":4088,"parent_record_offset":3548,"ancestor_record_offset":2728,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3470,"record_offset":3088,"parent_record_offset":3056,"ancestor_record_offset":2728,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3470,"record_offset":3056,"parent_record_offset":3028,"ancestor_record_offset":2728,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3470,"record_offset":3248,"parent_record_offset":3216,"ancestor_record_offset":2728,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3470,"record_offset":3344,"parent_record_offset":3248,"ancestor_record_offset":2728,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3470,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":2728,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3470,"record_offset":3028,"parent_record_offset":2992,"ancestor_record_offset":2728,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3470,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":2728,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3470,"record_offset":3648,"parent_record_offset":3616,"ancestor_record_offset":2728,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3470,"record_offset":3932,"parent_record_offset":3648,"ancestor_record_offset":2728,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3470,"record_offset":3904,"parent_record_offset":3648,"ancestor_record_offset":2728,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3470,"record_offset":3708,"parent_record_offset":3648,"ancestor_record_offset":2728,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$35::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3470,"record_offset":10324,"parent_record_offset":10228,"ancestor_record_offset":2728,"inlinee":25203,"source_line":123,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3470,"record_offset":10228,"parent_record_offset":10204,"ancestor_record_offset":2728,"inlinee":25199,"source_line":310,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$48::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3470,"record_offset":10296,"parent_record_offset":10228,"ancestor_record_offset":2728,"inlinee":25201,"source_line":336,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3470,"record_offset":6800,"parent_record_offset":6384,"ancestor_record_offset":2728,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3470,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":2728,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3470,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":2728,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3470,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":2728,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3470,"record_offset":8292,"parent_record_offset":8260,"ancestor_record_offset":2728,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3470,"record_offset":8260,"parent_record_offset":8228,"ancestor_record_offset":2728,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3470,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":2728,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3470,"record_offset":10268,"parent_record_offset":10228,"ancestor_record_offset":2728,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3470,"record_offset":9408,"parent_record_offset":9272,"ancestor_record_offset":2728,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3470,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":2728,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3470,"record_offset":7568,"parent_record_offset":7544,"ancestor_record_offset":2728,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3470,"record_offset":4460,"parent_record_offset":4424,"ancestor_record_offset":2728,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3470,"record_offset":5472,"parent_record_offset":5440,"ancestor_record_offset":2728,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3470,"record_offset":5988,"parent_record_offset":5728,"ancestor_record_offset":2728,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3470,"record_offset":4604,"parent_record_offset":4576,"ancestor_record_offset":2728,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3470,"record_offset":4680,"parent_record_offset":4532,"ancestor_record_offset":2728,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3470,"record_offset":4940,"parent_record_offset":4680,"ancestor_record_offset":2728,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3470,"record_offset":8324,"parent_record_offset":8292,"ancestor_record_offset":2728,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3470,"record_offset":4632,"parent_record_offset":4604,"ancestor_record_offset":2728,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3470,"record_offset":4532,"parent_record_offset":4496,"ancestor_record_offset":2728,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3470,"record_offset":4496,"parent_record_offset":4460,"ancestor_record_offset":2728,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C7F1
// 首条位置: {"module":3470,"inlinee":116721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3470,"record_offset":9216,"parent_record_offset":9180,"ancestor_record_offset":9180,"inlinee":7576,"source_line":378,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3470,"record_offset":13160,"rva":35318112,"va":5404027232}
// 编译实例 RVAs: 0x21ae960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28945810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e86e5246fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5404027293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28946189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e8f35046fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5404027688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7513368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e818a57200","size":5,"soft_fail":false,"target_va":5411540992,"va":5404027619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7513317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e8e5a47200","size":5,"soft_fail":false,"target_va":5411540963,"va":5404027641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26283413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e86bf26efe","size":5,"soft_fail":false,"target_va":5377744016,"va":5404027424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26283351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e8a9f26efe","size":5,"soft_fail":false,"target_va":5377744208,"va":5404027554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26284133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e89bef6efe","size":5,"soft_fail":false,"target_va":5377743280,"va":5404027408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26283656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e878f16efe","size":5,"soft_fail":false,"target_va":5377743872,"va":5404027523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26283540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e8ecf16efe","size":5,"soft_fail":false,"target_va":5377743920,"va":5404027455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26283519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404027232,"raw_hex":"e801f26efe","size":5,"soft_fail":false,"target_va":5377743968,"va":5404027482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":13360,"parent_record_offset":13160,"ancestor_record_offset":13160,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3470,"record_offset":14044,"parent_record_offset":13160,"ancestor_record_offset":13160,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3470,"record_offset":13508,"parent_record_offset":13160,"ancestor_record_offset":13160,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3470,"record_offset":14128,"parent_record_offset":13160,"ancestor_record_offset":13160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":13416,"parent_record_offset":13392,"ancestor_record_offset":13160,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":13440,"parent_record_offset":13416,"ancestor_record_offset":13160,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":14200,"parent_record_offset":14176,"ancestor_record_offset":13160,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":13392,"parent_record_offset":13360,"ancestor_record_offset":13160,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3470,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":13160,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3470,"record_offset":14152,"parent_record_offset":14128,"ancestor_record_offset":13160,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":13768,"parent_record_offset":13740,"ancestor_record_offset":13160,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":13828,"parent_record_offset":13796,"ancestor_record_offset":13160,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3470,"record_offset":13796,"parent_record_offset":13768,"ancestor_record_offset":13160,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3470,"record_offset":13704,"parent_record_offset":13676,"ancestor_record_offset":13160,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3470,"record_offset":14068,"parent_record_offset":14044,"ancestor_record_offset":13160,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3470,"record_offset":13676,"parent_record_offset":13648,"ancestor_record_offset":13160,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3470,"record_offset":13620,"parent_record_offset":13592,"ancestor_record_offset":13160,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3470,"record_offset":13648,"parent_record_offset":13620,"ancestor_record_offset":13160,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3470,"record_offset":13592,"parent_record_offset":13564,"ancestor_record_offset":13160,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3470,"record_offset":13564,"parent_record_offset":13536,"ancestor_record_offset":13160,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3470,"record_offset":13536,"parent_record_offset":13508,"ancestor_record_offset":13160,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3470,"record_offset":14092,"parent_record_offset":14068,"ancestor_record_offset":13160,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3470,"record_offset":13740,"parent_record_offset":13704,"ancestor_record_offset":13160,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3470,"record_offset":13980,"parent_record_offset":13956,"ancestor_record_offset":13160,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3470,"record_offset":13956,"parent_record_offset":13704,"ancestor_record_offset":13160,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3470,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":13160,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03470.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_edit_roof_height_focused.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof::DoUiEditRoof>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof::DoUiEditRoof>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof::DoUiEditRoof>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof::DoUiEditRoof>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof::DoUiEditRoof>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof::DoUiEditRoof>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > > >", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred", "arguments": ["tuple$<void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof::DoUiEditRoof>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof::DoUiEditRoof>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::config::impl$4::into_configs", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_edit_roof_height_focused.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_edit_roof_height_focused.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// 信息 ID: 1aaac17d6ef504b9a38cd72a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] df6c077194d4ee581fe377dd: void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > @ 0x1a1d74e -> 0x21ad440 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > > @ 0x1a291e6 -> 0x21ad440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// 信息 ID: e27500a84d42424e9a87d6c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused; module=3470, inline_record=9180; 不是独立调用边。
// [PDB 内联宿主] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused; module=3470, inline_record=9720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// 信息 ID: c84ebece3238db69c091e4e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused", "captures": "unknown"}
// [闭包环境类型引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1 <- core::ops::function::FnOnce::call_once<system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x21acf02 -> 0x21ae960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
