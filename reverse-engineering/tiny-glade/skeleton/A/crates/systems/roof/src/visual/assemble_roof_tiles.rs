// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: assemble_roof_tiles.rs.assembly.json.gz

// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 266:9852 -> host 266:3304 / rva_3eff70_len_a0b; exact_symbolizer_function_frame
//   Function-frame RVA span [0x3f0697, 0x3f06a5), 3 instructions
//
// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// Candidate interfaces: fce0cd045cae57e7be2d3064
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 266:3304 -> rva_3eff70_len_a0b
//
// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 7; complete records in enriched/assembly sidecars.
// Compiled procedure 753:32852 -> rva_8dfff0_len_353
// Compiled procedure 754:11396 -> rva_8e1820_len_353
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_3eff70_len_a0b: RVA 0x3eff70, 2571 PDB bytes; 638 instructions
// Evidence: _assembly/roof-v1/A/pack-000/rva_3eff70_len_a0b.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 266:3304: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// Frames: F1
// 0x1403eff70  55                             pushq	%rbp
// 0x1403eff71  4157                           pushq	%r15
// 0x1403eff73  4156                           pushq	%r14
// 0x1403eff75  4155                           pushq	%r13
// 0x1403eff77  4154                           pushq	%r12
// 0x1403eff79  56                             pushq	%rsi
// 0x1403eff7a  57                             pushq	%rdi
// 0x1403eff7b  53                             pushq	%rbx
// 0x1403eff7c  4881ec78010000                 subq	$0x178, %rsp            # imm = 0x178
// 0x1403eff83  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1403eff8b  660f7fb5e0000000               movdqa	%xmm6, 0xe0(%rbp)
// 0x1403eff93  48c785d8000000feffffff         movq	$-0x2, 0xd8(%rbp)
// 0x1403eff9e  4d89cd                         movq	%r9, %r13
// 0x1403effa1  4c89c6                         movq	%r8, %rsi
// 0x1403effa4  4989d7                         movq	%rdx, %r15
// 0x1403effa7  4889cf                         movq	%rcx, %rdi
// Frames: F2
// 0x1403effaa  e881af3500                     callq	0x14074af30 <.text+0x749f30>
// 0x1403effaf  84c0                           testb	%al, %al
// 0x1403effb1  745d                           je	0x1403f0010 <.text+0x3ef010>
// Frames: F3 -> F2
// 0x1403effb3  41be3d000000                   movl	$0x3d, %r14d
// 0x1403effb9  b93d000000                     movl	$0x3d, %ecx
// 0x1403effbe  e8ddfaffff                     callq	0x1403efaa0 <.text+0x3eeaa0>
// 0x1403effc3  4c8d25867ddd01                 leaq	0x1dd7d86(%rip), %r12   # 0x1421c7d50
// 0x1403effca  4883f801                       cmpq	$0x1, %rax
// 0x1403effce  0f85af000000                   jne	0x1403f0083 <.text+0x3ef083>
// Frames: F4 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x1403effd4  4989d1                         movq	%rdx, %r9
// 0x1403effd7  4885d2                         testq	%rdx, %rdx
// 0x1403effda  7444                           je	0x1403f0020 <.text+0x3ef020>
// Frames: F9 -> F10 -> F11 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x1403effdc  4983f93d                       cmpq	$0x3d, %r9
// Frames: F11 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x1403effe0  733c                           jae	0x1403f001e <.text+0x3ef01e>
// Frames: F9 -> F10 -> F11 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x1403effe2  488d05677ddd01                 leaq	0x1dd7d67(%rip), %rax   # 0x1421c7d50
// Frames: F12 -> F13 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x1403effe9  41803c01bf                     cmpb	$-0x41, (%r9,%rax)
// Frames: F5 -> F6 -> F7 -> F8 -> F2
// 0x1403effee  7f30                           jg	0x1403f0020 <.text+0x3ef020>
// Frames: F14 -> F7 -> F8 -> F2
// 0x1403efff0  488d05d97edd01                 leaq	0x1dd7ed9(%rip), %rax   # 0x1421c7ed0
// 0x1403efff7  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1403efffc  488d0d4d7ddd01                 leaq	0x1dd7d4d(%rip), %rcx   # 0x1421c7d50
// 0x1403f0003  ba3d000000                     movl	$0x3d, %edx
// 0x1403f0008  4531c0                         xorl	%r8d, %r8d
// 0x1403f000b  e820f1cb01                     callq	0x1420af130 <.text+0x20ae130>
// 0x1403f0010  31c0                           xorl	%eax, %eax
// 0x1403f0012  488985c0000000                 movq	%rax, 0xc0(%rbp)
// 0x1403f0019  e916020000                     jmp	0x1403f0234 <.text+0x3ef234>
// Frames: F5 -> F6 -> F7 -> F8 -> F2
// 0x1403f001e  75d0                           jne	0x1403efff0 <.text+0x3eeff0>
// Frames: F8 -> F2
// 0x1403f0020  4c89c9                         movq	%r9, %rcx
// 0x1403f0023  e878faffff                     callq	0x1403efaa0 <.text+0x3eeaa0>
// 0x1403f0028  4883f801                       cmpq	$0x1, %rax
// 0x1403f002c  7555                           jne	0x1403f0083 <.text+0x3ef083>
// Frames: F4 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x1403f002e  4989d0                         movq	%rdx, %r8
// 0x1403f0031  4983c002                       addq	$0x2, %r8
// 0x1403f0035  7439                           je	0x1403f0070 <.text+0x3ef070>
// Frames: F9 -> F10 -> F11 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x1403f0037  4983f83d                       cmpq	$0x3d, %r8
// Frames: F11 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x1403f003b  7331                           jae	0x1403f006e <.text+0x3ef06e>
// Frames: F9 -> F10 -> F11 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x1403f003d  488d050c7ddd01                 leaq	0x1dd7d0c(%rip), %rax   # 0x1421c7d50
// Frames: F12 -> F13 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x1403f0044  41803c00bf                     cmpb	$-0x41, (%r8,%rax)
// Frames: F15 -> F16 -> F7 -> F17 -> F2
// 0x1403f0049  7f25                           jg	0x1403f0070 <.text+0x3ef070>
// Frames: F18 -> F7 -> F17 -> F2
// 0x1403f004b  488d05967edd01                 leaq	0x1dd7e96(%rip), %rax   # 0x1421c7ee8
// 0x1403f0052  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1403f0057  488d0df27cdd01                 leaq	0x1dd7cf2(%rip), %rcx   # 0x1421c7d50
// 0x1403f005e  ba3d000000                     movl	$0x3d, %edx
// 0x1403f0063  41b93d000000                   movl	$0x3d, %r9d
// 0x1403f0069  e8c2f0cb01                     callq	0x1420af130 <.text+0x20ae130>
// Frames: F15 -> F16 -> F7 -> F17 -> F2
// 0x1403f006e  75db                           jne	0x1403f004b <.text+0x3ef04b>
// Frames: F19 -> F20 -> F21 -> F22 -> F16 -> F7 -> F17 -> F2
// 0x1403f0070  4c8d25d97cdd01                 leaq	0x1dd7cd9(%rip), %r12   # 0x1421c7d50
// 0x1403f0077  4d01c4                         addq	%r8, %r12
// 0x1403f007a  41be3b000000                   movl	$0x3b, %r14d
// Frames: F23 -> F21 -> F22 -> F16 -> F7 -> F17 -> F2
// 0x1403f0080  4929d6                         subq	%rdx, %r14
// 0x1403f0083  4c89add0000000                 movq	%r13, 0xd0(%rbp)
// 0x1403f008a  488d05147fdd01                 leaq	0x1dd7f14(%rip), %rax   # 0x1421c7fa5
// 0x1403f0091  4c8d2dd87edd01                 leaq	0x1dd7ed8(%rip), %r13   # 0x1421c7f70
// 0x1403f0098  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00a0  0fb648ff                       movzbl	-0x1(%rax), %ecx
// 0x1403f00a4  84c9                           testb	%cl, %cl
// 0x1403f00a6  7808                           js	0x1403f00b0 <.text+0x3ef0b0>
// 0x1403f00a8  48ffc8                         decq	%rax
// 0x1403f00ab  0fb6c9                         movzbl	%cl, %ecx
// 0x1403f00ae  eb5c                           jmp	0x1403f010c <.text+0x3ef10c>
// Frames: F32 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00b0  0fb650fe                       movzbl	-0x2(%rax), %edx
// Frames: F33 -> F34 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00b4  80fac0                         cmpb	$-0x40, %dl
// Frames: F34 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00b7  7d25                           jge	0x1403f00de <.text+0x3ef0de>
// Frames: F35 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00b9  440fb640fd                     movzbl	-0x3(%rax), %r8d
// Frames: F33 -> F36 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00be  4180f8bf                       cmpb	$-0x41, %r8b
// Frames: F36 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00c2  7f23                           jg	0x1403f00e7 <.text+0x3ef0e7>
// Frames: F37 -> F38 -> F39 -> F40 -> F41 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00c4  440fb648fc                     movzbl	-0x4(%rax), %r9d
// 0x1403f00c9  4883c0fc                       addq	$-0x4, %rax
// Frames: F42 -> F43 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00cd  4183e107                       andl	$0x7, %r9d
// Frames: F44 -> F45 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00d1  41c1e106                       shll	$0x6, %r9d
// 0x1403f00d5  4183e03f                       andl	$0x3f, %r8d
// 0x1403f00d9  4509c8                         orl	%r9d, %r8d
// 0x1403f00dc  eb11                           jmp	0x1403f00ef <.text+0x3ef0ef>
// 0x1403f00de  4883c0fe                       addq	$-0x2, %rax
// 0x1403f00e2  83e21f                         andl	$0x1f, %edx
// 0x1403f00e5  eb12                           jmp	0x1403f00f9 <.text+0x3ef0f9>
// 0x1403f00e7  4883c0fd                       addq	$-0x3, %rax
// 0x1403f00eb  4183e00f                       andl	$0xf, %r8d
// Frames: F44 -> F46 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00ef  41c1e006                       shll	$0x6, %r8d
// 0x1403f00f3  83e23f                         andl	$0x3f, %edx
// 0x1403f00f6  4409c2                         orl	%r8d, %edx
// Frames: F44 -> F47 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f00f9  c1e206                         shll	$0x6, %edx
// 0x1403f00fc  80e13f                         andb	$0x3f, %cl
// 0x1403f00ff  0fb6c9                         movzbl	%cl, %ecx
// 0x1403f0102  09d1                           orl	%edx, %ecx
// Frames: F48 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f0104  81f900001100                   cmpl	$0x110000, %ecx         # imm = 0x110000
// 0x1403f010a  740f                           je	0x1403f011b <.text+0x3ef11b>
// Frames: F49 -> F50 -> F51 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f010c  83f95c                         cmpl	$0x5c, %ecx
// 0x1403f010f  7459                           je	0x1403f016a <.text+0x3ef16a>
// 0x1403f0111  83f92f                         cmpl	$0x2f, %ecx
// 0x1403f0114  7454                           je	0x1403f016a <.text+0x3ef16a>
// Frames: F52 -> F53 -> F54 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f0116  4c39e8                         cmpq	%r13, %rax
// Frames: F55 -> F54 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x1403f0119  7585                           jne	0x1403f00a0 <.text+0x3ef0a0>
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x1403f011b  bb35000000                     movl	$0x35, %ebx
// 0x1403f0120  e84bbf3500                     callq	0x14074c070 <.text+0x74b070>
// Frames: F61 -> F62 -> F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x1403f0125  48833800                       cmpq	$0x0, (%rax)
// 0x1403f0129  0f8599000000                   jne	0x1403f01c8 <.text+0x3ef1c8>
// Frames: F64 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x1403f012f  4889c1                         movq	%rax, %rcx
// 0x1403f0132  31d2                           xorl	%edx, %edx
// 0x1403f0134  e867054e00                     callq	0x1408d06a0 <.text+0x8cf6a0>
// Frames: F65 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x1403f0139  4885c0                         testq	%rax, %rax
// 0x1403f013c  0f858a000000                   jne	0x1403f01cc <.text+0x3ef1cc>
// Frames: F66 -> F58 -> F59 -> F60 -> F2
// 0x1403f0142  488d05df7cdd01                 leaq	0x1dd7cdf(%rip), %rax   # 0x1421c7e28
// 0x1403f0149  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1403f014e  488d0d3b7cdd01                 leaq	0x1dd7c3b(%rip), %rcx   # 0x1421c7d90
// 0x1403f0155  4c8d0d847bdd01                 leaq	0x1dd7b84(%rip), %r9    # 0x1421c7ce0
// 0x1403f015c  4c8d45d8                       leaq	-0x28(%rbp), %r8
// 0x1403f0160  ba46000000                     movl	$0x46, %edx
// 0x1403f0165  e8b6edcb01                     callq	0x1420aef20 <.text+0x20adf20>
// Frames: F67 -> F2
// 0x1403f016a  4c8d2dff7ddd01                 leaq	0x1dd7dff(%rip), %r13   # 0x1421c7f70
// 0x1403f0171  4c29e8                         subq	%r13, %rax
// 0x1403f0174  4989c0                         movq	%rax, %r8
// 0x1403f0177  49ffc0                         incq	%r8
// Frames: F4 -> F15 -> F16 -> F7 -> F67 -> F2
// 0x1403f017a  7432                           je	0x1403f01ae <.text+0x3ef1ae>
// Frames: F9 -> F10 -> F11 -> F15 -> F16 -> F7 -> F67 -> F2
// 0x1403f017c  4983f835                       cmpq	$0x35, %r8
// Frames: F11 -> F15 -> F16 -> F7 -> F67 -> F2
// 0x1403f0180  732a                           jae	0x1403f01ac <.text+0x3ef1ac>
// Frames: F12 -> F13 -> F15 -> F16 -> F7 -> F67 -> F2
// 0x1403f0182  43803c28bf                     cmpb	$-0x41, (%r8,%r13)
// Frames: F15 -> F16 -> F7 -> F67 -> F2
// 0x1403f0187  7f25                           jg	0x1403f01ae <.text+0x3ef1ae>
// Frames: F18 -> F7 -> F67 -> F2
// 0x1403f0189  488d05207ddd01                 leaq	0x1dd7d20(%rip), %rax   # 0x1421c7eb0
// 0x1403f0190  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1403f0195  488d0dd47ddd01                 leaq	0x1dd7dd4(%rip), %rcx   # 0x1421c7f70
// 0x1403f019c  ba35000000                     movl	$0x35, %edx
// 0x1403f01a1  41b935000000                   movl	$0x35, %r9d
// 0x1403f01a7  e884efcb01                     callq	0x1420af130 <.text+0x20ae130>
// Frames: F15 -> F16 -> F7 -> F67 -> F2
// 0x1403f01ac  75db                           jne	0x1403f0189 <.text+0x3ef189>
// Frames: F19 -> F20 -> F21 -> F22 -> F16 -> F7 -> F67 -> F2
// 0x1403f01ae  4d01c5                         addq	%r8, %r13
// 0x1403f01b1  bb34000000                     movl	$0x34, %ebx
// Frames: F23 -> F21 -> F22 -> F16 -> F7 -> F67 -> F2
// 0x1403f01b6  4829c3                         subq	%rax, %rbx
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x1403f01b9  e8b2be3500                     callq	0x14074c070 <.text+0x74b070>
// Frames: F61 -> F62 -> F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x1403f01be  48833800                       cmpq	$0x0, (%rax)
// 0x1403f01c2  0f8467ffffff                   je	0x1403f012f <.text+0x3ef12f>
// Frames: F68 -> F62 -> F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x1403f01c8  4883c008                       addq	$0x8, %rax
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x1403f01cc  48833800                       cmpq	$0x0, (%rax)
// 0x1403f01d0  0f8534060000                   jne	0x1403f080a <.text+0x3ef80a>
// Frames: F74 -> F75 -> F76 -> F77 -> F78 -> F70 -> F71 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x1403f01d6  48c700ffffffff                 movq	$-0x1, (%rax)
// 0x1403f01dd  488985c8000000                 movq	%rax, 0xc8(%rbp)
// Frames: F79 -> F80 -> F71 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x1403f01e4  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F81 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x1403f01e8  488d05417add01                 leaq	0x1dd7a41(%rip), %rax   # 0x1421c7c30
// 0x1403f01ef  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1403f01f4  48895c2420                     movq	%rbx, 0x20(%rsp)
// 0x1403f01f9  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x1403f0202  4c89e2                         movq	%r12, %rdx
// 0x1403f0205  4d89f0                         movq	%r14, %r8
// 0x1403f0208  4d89e9                         movq	%r13, %r9
// 0x1403f020b  e820b03500                     callq	0x14074b230 <.text+0x74a230>
// 0x1403f0210  488985a8000000                 movq	%rax, 0xa8(%rbp)
// Frames: F74 -> F75 -> F76 -> F77 -> F82 -> F83 -> F83 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x1403f0217  488b85c8000000                 movq	0xc8(%rbp), %rax
// 0x1403f021e  48ff00                         incq	(%rax)
// 0x1403f0221  b801000000                     movl	$0x1, %eax
// 0x1403f0226  488985c0000000                 movq	%rax, 0xc0(%rbp)
// 0x1403f022d  4c8badd0000000                 movq	0xd0(%rbp), %r13
// Frames: F84 -> F85 -> F86
// 0x1403f0234  4c8b0f                         movq	(%rdi), %r9
// Frames: F87 -> F85 -> F86
// 0x1403f0237  488b4f08                       movq	0x8(%rdi), %rcx
// Frames: F88 -> F89 -> F90 -> F85 -> F86
// 0x1403f023b  498b11                         movq	(%r9), %rdx
// 0x1403f023e  4531d2                         xorl	%r10d, %r10d
// Frames: F91 -> F92 -> F89 -> F90 -> F85 -> F86
// 0x1403f0241  4889d0                         movq	%rdx, %rax
// 0x1403f0244  482b4118                       subq	0x18(%rcx), %rax
// 0x1403f0248  490f42c2                       cmovbq	%r10, %rax
// Frames: F91 -> F93 -> F89 -> F90 -> F85 -> F86
// 0x1403f024c  482b5138                       subq	0x38(%rcx), %rdx
// 0x1403f0250  490f42d2                       cmovbq	%r10, %rdx
// Frames: F94 -> F95 -> F10 -> F96 -> F89 -> F90 -> F85 -> F86
// 0x1403f0254  4889c3                         movq	%rax, %rbx
// 0x1403f0257  48c1e305                       shlq	$0x5, %rbx
// 0x1403f025b  48035908                       addq	0x8(%rcx), %rbx
// Frames: F96 -> F89 -> F90 -> F85 -> F86
// 0x1403f025f  4c8b5910                       movq	0x10(%rcx), %r11
// Frames: F97 -> F96 -> F89 -> F90 -> F85 -> F86
// 0x1403f0263  4929c3                         subq	%rax, %r11
// 0x1403f0266  4d0f42da                       cmovbq	%r10, %r11
// Frames: F98 -> F89 -> F90 -> F85 -> F86
// 0x1403f026a  488b7930                       movq	0x30(%rcx), %rdi
// Frames: F97 -> F96 -> F89 -> F90 -> F85 -> F86
// 0x1403f026e  4c8d053b80ee01                 leaq	0x1ee803b(%rip), %r8    # 0x1422d82b0
// 0x1403f0275  490f42d8                       cmovbq	%r8, %rbx
// Frames: F94 -> F95 -> F10 -> F98 -> F89 -> F90 -> F85 -> F86
// 0x1403f0279  4889d0                         movq	%rdx, %rax
// 0x1403f027c  48c1e005                       shlq	$0x5, %rax
// 0x1403f0280  48034128                       addq	0x28(%rcx), %rax
// 0x1403f0284  4839fa                         cmpq	%rdi, %rdx
// Frames: F97 -> F98 -> F89 -> F90 -> F85 -> F86
// 0x1403f0287  490f47c0                       cmovaq	%r8, %rax
// 0x1403f028b  4829d7                         subq	%rdx, %rdi
// 0x1403f028e  490f42fa                       cmovbq	%r10, %rdi
// 0x1403f0292  488b9560010000                 movq	0x160(%rbp), %rdx
// Frames: F99 -> F89 -> F90 -> F85 -> F86
// 0x1403f0299  488b4940                       movq	0x40(%rcx), %rcx
// 0x1403f029d  4829f9                         subq	%rdi, %rcx
// 0x1403f02a0  4c29d9                         subq	%r11, %rcx
// 0x1403f02a3  4c898dc8000000                 movq	%r9, 0xc8(%rbp)
// 0x1403f02aa  498909                         movq	%rcx, (%r9)
// Frames: F100 -> F101 -> F102 -> F103 -> F89 -> F90 -> F85 -> F86
// 0x1403f02ad  49c1e305                       shlq	$0x5, %r11
// 0x1403f02b1  4901db                         addq	%rbx, %r11
// 0x1403f02b4  4c895d58                       movq	%r11, 0x58(%rbp)
// 0x1403f02b8  48c1e705                       shlq	$0x5, %rdi
// 0x1403f02bc  4801c7                         addq	%rax, %rdi
// 0x1403f02bf  48897d60                       movq	%rdi, 0x60(%rbp)
// 0x1403f02c3  488b0e                         movq	(%rsi), %rcx
// 0x1403f02c6  48894d40                       movq	%rcx, 0x40(%rbp)
// 0x1403f02ca  498b0f                         movq	(%r15), %rcx
// 0x1403f02cd  48894d48                       movq	%rcx, 0x48(%rbp)
// 0x1403f02d1  498b4f08                       movq	0x8(%r15), %rcx
// 0x1403f02d5  48894d50                       movq	%rcx, 0x50(%rbp)
// 0x1403f02d9  418b4f10                       movl	0x10(%r15), %ecx
// 0x1403f02dd  898db0000000                   movl	%ecx, 0xb0(%rbp)
// 0x1403f02e3  418b4f14                       movl	0x14(%r15), %ecx
// 0x1403f02e7  898db4000000                   movl	%ecx, 0xb4(%rbp)
// 0x1403f02ed  498b4d00                       movq	(%r13), %rcx
// 0x1403f02f1  488b12                         movq	(%rdx), %rdx
// 0x1403f02f4  48895570                       movq	%rdx, 0x70(%rbp)
// 0x1403f02f8  660f76f6                       pcmpeqd	%xmm6, %xmm6
// 0x1403f02fc  48894d68                       movq	%rcx, 0x68(%rbp)
// 0x1403f0300  4883c120                       addq	$0x20, %rcx
// 0x1403f0304  48894d78                       movq	%rcx, 0x78(%rbp)
// 0x1403f0308  4c8d75d8                       leaq	-0x28(%rbp), %r14
// 0x1403f030c  eb0c                           jmp	0x1403f031a <.text+0x3ef31a>
// 0x1403f030e  6690                           nop
// 0x1403f0310  488b9dd0000000                 movq	0xd0(%rbp), %rbx
// 0x1403f0317  4c89e8                         movq	%r13, %rax
// Frames: F104 -> F105 -> F106 -> F107 -> F108 -> F86
// 0x1403f031a  4885db                         testq	%rbx, %rbx
// 0x1403f031d  7421                           je	0x1403f0340 <.text+0x3ef340>
// 0x1403f031f  483b5d58                       cmpq	0x58(%rbp), %rbx
// 0x1403f0323  741b                           je	0x1403f0340 <.text+0x3ef340>
// 0x1403f0325  488d4b20                       leaq	0x20(%rbx), %rcx
// 0x1403f0329  48898dd0000000                 movq	%rcx, 0xd0(%rbp)
// 0x1403f0330  4989c5                         movq	%rax, %r13
// 0x1403f0333  eb2e                           jmp	0x1403f0363 <.text+0x3ef363>
// 0x1403f0335  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F104 -> F109 -> F110 -> F106 -> F107 -> F108 -> F86
// 0x1403f0340  4885c0                         testq	%rax, %rax
// 0x1403f0343  0f84d8030000                   je	0x1403f0721 <.text+0x3ef721>
// 0x1403f0349  483b4560                       cmpq	0x60(%rbp), %rax
// 0x1403f034d  0f84ce030000                   je	0x1403f0721 <.text+0x3ef721>
// Frames: F111 -> F112 -> F113 -> F109 -> F110 -> F106 -> F107 -> F108 -> F86
// 0x1403f0353  4c8d6820                       leaq	0x20(%rax), %r13
// 0x1403f0357  31c9                           xorl	%ecx, %ecx
// 0x1403f0359  48898dd0000000                 movq	%rcx, 0xd0(%rbp)
// 0x1403f0360  4889c3                         movq	%rax, %rbx
// Frames: F114 -> F108 -> F86
// 0x1403f0363  488b85c8000000                 movq	0xc8(%rbp), %rax
// 0x1403f036a  48ff00                         incq	(%rax)
// Frames: F115 -> F116 -> F117
// 0x1403f036d  488b4d40                       movq	0x40(%rbp), %rcx
// 0x1403f0371  488d15c092ee01                 leaq	0x1ee92c0(%rip), %rdx   # 0x1422d9638
// 0x1403f0378  e8f34e4e00                     callq	0x1408d5270 <.text+0x8d4270>
// 0x1403f037d  4989c7                         movq	%rax, %r15
// 0x1403f0380  4885c0                         testq	%rax, %rax
// 0x1403f0383  0f84d0030000                   je	0x1403f0759 <.text+0x3ef759>
// Frames: F118 -> F116 -> F117
// 0x1403f0389  488d0d3894ee01                 leaq	0x1ee9438(%rip), %rcx   # 0x1422d97c8
// 0x1403f0390  48898d80000000                 movq	%rcx, 0x80(%rbp)
// 0x1403f0397  48c7858800000035000000         movq	$0x35, 0x88(%rbp)
// 0x1403f03a2  4d89fc                         movq	%r15, %r12
// 0x1403f03a5  4983c410                       addq	$0x10, %r12
// Frames: F119 -> F120 -> F121 -> F122 -> F121 -> F118 -> F116 -> F117
// 0x1403f03a9  49837f1835                     cmpq	$0x35, 0x18(%r15)
// 0x1403f03ae  0f85b6030000                   jne	0x1403f076a <.text+0x3ef76a>
// Frames: F118 -> F116 -> F117
// 0x1403f03b4  498b1424                       movq	(%r12), %rdx
// Frames: F123 -> F120 -> F121 -> F122 -> F121 -> F118 -> F116 -> F117
// 0x1403f03b8  41b835000000                   movl	$0x35, %r8d
// 0x1403f03be  e8bd46d501                     callq	0x142144a80 <.text+0x2143a80>
// 0x1403f03c3  85c0                           testl	%eax, %eax
// Frames: F118 -> F116 -> F117
// 0x1403f03c5  0f859f030000                   jne	0x1403f076a <.text+0x3ef76a>
// Frames: F124 -> F116 -> F117
// 0x1403f03cb  498b0f                         movq	(%r15), %rcx
// 0x1403f03ce  498b4708                       movq	0x8(%r15), %rax
// Frames: F116 -> F117
// 0x1403f03d2  ff5018                         callq	*0x18(%rax)
// Frames: F117
// 0x1403f03d5  4989c4                         movq	%rax, %r12
// Frames: F125 -> F117
// 0x1403f03d8  4885c0                         testq	%rax, %rax
// 0x1403f03db  0f8478030000                   je	0x1403f0759 <.text+0x3ef759>
// Frames: F126
// 0x1403f03e1  4c8b4b08                       movq	0x8(%rbx), %r9
// Frames: F127 -> F126
// 0x1403f03e5  8b85b4000000                   movl	0xb4(%rbp), %eax
// 0x1403f03eb  89442428                       movl	%eax, 0x28(%rsp)
// 0x1403f03ef  8b85b0000000                   movl	0xb0(%rbp), %eax
// 0x1403f03f5  89442420                       movl	%eax, 0x20(%rsp)
// 0x1403f03f9  4c89f1                         movq	%r14, %rcx
// 0x1403f03fc  488b5548                       movq	0x48(%rbp), %rdx
// 0x1403f0400  4c8b4550                       movq	0x50(%rbp), %r8
// 0x1403f0404  e8578a4e00                     callq	0x1408d8e60 <.text+0x8d7e60>
// Frames: F128 -> F126
// 0x1403f0409  4c89f7                         movq	%r14, %rdi
// 0x1403f040c  4c8b75d8                       movq	-0x28(%rbp), %r14
// 0x1403f0410  4d85f6                         testq	%r14, %r14
// 0x1403f0413  0f847c030000                   je	0x1403f0795 <.text+0x3ef795>
// Frames: F129 -> F126
// 0x1403f0419  488b5de8                       movq	-0x18(%rbp), %rbx
// 0x1403f041d  8b75f4                         movl	-0xc(%rbp), %esi
// 0x1403f0420  4c8b7df8                       movq	-0x8(%rbp), %r15
// Frames: F130 -> F131 -> F132
// 0x1403f0424  49833e00                       cmpq	$0x0, (%r14)
// Frames: F132
// 0x1403f0428  0f85c2000000                   jne	0x1403f04f0 <.text+0x3ef4f0>
// Frames: F133 -> F134 -> F135 -> F136 -> F137 -> F138 -> F139 -> F140
// 0x1403f042e  0fb6058ca68802                 movzbl	0x288a68c(%rip), %eax   # 0x142c7aac1
// Frames: F141 -> F135 -> F136 -> F137 -> F138 -> F139 -> F140
// 0x1403f0435  b920000000                     movl	$0x20, %ecx
// 0x1403f043a  ba08000000                     movl	$0x8, %edx
// 0x1403f043f  e83ca3feff                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F137 -> F138 -> F139 -> F140
// 0x1403f0444  4885c0                         testq	%rax, %rax
// 0x1403f0447  0f849e030000                   je	0x1403f07eb <.text+0x3ef7eb>
// Frames: F138 -> F139 -> F140
// 0x1403f044d  4889c2                         movq	%rax, %rdx
// 0x1403f0450  48c70000000000                 movq	$0x0, (%rax)
// 0x1403f0457  48c7400804000000               movq	$0x4, 0x8(%rax)
// 0x1403f045f  48c7401000000000               movq	$0x0, 0x10(%rax)
// 0x1403f0467  c6401800                       movb	$0x0, 0x18(%rax)
// Frames: F142 -> F143 -> F144 -> F145 -> F140
// 0x1403f046b  4d8b442460                     movq	0x60(%r12), %r8
// 0x1403f0470  eb10                           jmp	0x1403f0482 <.text+0x3ef482>
// 0x1403f0472  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F146 -> F147 -> F148 -> F145 -> F140
// 0x1403f0480  f390                           pause
// Frames: F149 -> F150 -> F151 -> F145 -> F140
// 0x1403f0482  498b4008                       movq	0x8(%r8), %rax
// 0x1403f0486  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F152 -> F145 -> F140
// 0x1403f0490  4883f8ff                       cmpq	$-0x1, %rax
// 0x1403f0494  74ea                           je	0x1403f0480 <.text+0x3ef480>
// Frames: F153 -> F145 -> F140
// 0x1403f0496  4885c0                         testq	%rax, %rax
// 0x1403f0499  0f88b0020000                   js	0x1403f074f <.text+0x3ef74f>
// Frames: F154 -> F145 -> F140
// 0x1403f049f  488d4801                       leaq	0x1(%rax), %rcx
// Frames: F155 -> F156 -> F154 -> F145 -> F140
// 0x1403f04a3  f0                             lock
// 0x1403f04a4  490fb14808                     cmpxchgq	%rcx, 0x8(%r8)
// Frames: F154 -> F145 -> F140
// 0x1403f04a9  75e5                           jne	0x1403f0490 <.text+0x3ef490>
// Frames: F157 -> F158 -> F140
// 0x1403f04ab  4c8985a0000000                 movq	%r8, 0xa0(%rbp)
// 0x1403f04b2  48899598000000                 movq	%rdx, 0x98(%rbp)
// 0x1403f04b9  8933                           movl	%esi, (%rbx)
// 0x1403f04bb  4c89b5b8000000                 movq	%r14, 0xb8(%rbp)
// Frames: F83 -> F140
// 0x1403f04c2  49833e00                       cmpq	$0x0, (%r14)
// 0x1403f04c6  740c                           je	0x1403f04d4 <.text+0x3ef4d4>
// 0x1403f04c8  488b8db8000000                 movq	0xb8(%rbp), %rcx
// 0x1403f04cf  e88ccb4d00                     callq	0x1408cd060 <.text+0x8cc060>
// Frames: F140
// 0x1403f04d4  488b85a0000000                 movq	0xa0(%rbp), %rax
// 0x1403f04db  4c8bb5b8000000                 movq	0xb8(%rbp), %r14
// 0x1403f04e2  498906                         movq	%rax, (%r14)
// 0x1403f04e5  488b8598000000                 movq	0x98(%rbp), %rax
// 0x1403f04ec  49894608                       movq	%rax, 0x8(%r14)
// Frames: F157 -> F158 -> F159
// 0x1403f04f0  8933                           movl	%esi, (%rbx)
// Frames: F104 -> F159
// 0x1403f04f2  49833e00                       cmpq	$0x0, (%r14)
// Frames: F125 -> F159
// 0x1403f04f6  0f84d3020000                   je	0x1403f07cf <.text+0x3ef7cf>
// Frames: F160 -> F161
// 0x1403f04fc  498b7608                       movq	0x8(%r14), %rsi
// Frames: F125 -> F160 -> F161
// 0x1403f0500  4885f6                         testq	%rsi, %rsi
// 0x1403f0503  0f84d4020000                   je	0x1403f07dd <.text+0x3ef7dd>
// Frames: F155 -> F162 -> F163 -> F164 -> F161
// 0x1403f0509  31c0                           xorl	%eax, %eax
// 0x1403f050b  b101                           movb	$0x1, %cl
// 0x1403f050d  f0                             lock
// 0x1403f050e  0fb04e18                       cmpxchgb	%cl, 0x18(%rsi)
// 0x1403f0512  4989fe                         movq	%rdi, %r14
// Frames: F165 -> F166 -> F167 -> F164 -> F161
// 0x1403f0515  740f                           je	0x1403f0526 <.text+0x3ef526>
// Frames: F164 -> F161
// 0x1403f0517  488d4e18                       leaq	0x18(%rsi), %rcx
// Frames: F168 -> F164 -> F161
// 0x1403f051b  41b800ca9a3b                   movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
// 0x1403f0521  e87abecb01                     callq	0x1420ac3a0 <.text+0x20ab3a0>
// Frames: F169 -> F161
// 0x1403f0526  488b4610                       movq	0x10(%rsi), %rax
// Frames: F170 -> F169 -> F161
// 0x1403f052a  4885c0                         testq	%rax, %rax
// Frames: F171 -> F169 -> F161
// 0x1403f052d  7411                           je	0x1403f0540 <.text+0x3ef540>
// Frames: F169 -> F161
// 0x1403f052f  488b4e08                       movq	0x8(%rsi), %rcx
// Frames: F172 -> F169 -> F161
// 0x1403f0533  8b4c81fc                       movl	-0x4(%rcx,%rax,4), %ecx
// Frames: F173 -> F161
// 0x1403f0537  498b442428                     movq	0x28(%r12), %rax
// 0x1403f053c  eb13                           jmp	0x1403f0551 <.text+0x3ef551>
// 0x1403f053e  6690                           nop
// 0x1403f0540  498b442428                     movq	0x28(%r12), %rax
// Frames: F174 -> F175 -> F176 -> F177 -> F169 -> F161
// 0x1403f0545  4885c0                         testq	%rax, %rax
// Frames: F125 -> F176 -> F177 -> F169 -> F161
// 0x1403f0548  0f84ae020000                   je	0x1403f07fc <.text+0x3ef7fc>
// Frames: F176 -> F177 -> F169 -> F161
// 0x1403f054e  8d48ff                         leal	-0x1(%rax), %ecx
// Frames: F178 -> F161
// 0x1403f0551  498b542420                     movq	0x20(%r12), %rdx
// Frames: F179 -> F161
// 0x1403f0556  f3410f6f442430                 movdqu	0x30(%r12), %xmm0
// Frames: F180 -> F161
// 0x1403f055d  f3410f6f4c2440                 movdqu	0x40(%r12), %xmm1
// Frames: F181 -> F161
// 0x1403f0564  f3410f6f542450                 movdqu	0x50(%r12), %xmm2
// Frames: F182 -> F161
// 0x1403f056b  894d28                         movl	%ecx, 0x28(%rbp)
// 0x1403f056e  488955d8                       movq	%rdx, -0x28(%rbp)
// 0x1403f0572  488945e0                       movq	%rax, -0x20(%rbp)
// 0x1403f0576  f30f7f45e8                     movdqu	%xmm0, -0x18(%rbp)
// 0x1403f057b  488975f8                       movq	%rsi, -0x8(%rbp)
// 0x1403f057f  f30f7f4d00                     movdqu	%xmm1, (%rbp)
// 0x1403f0584  4c896510                       movq	%r12, 0x10(%rbp)
// 0x1403f0588  f30f7f5518                     movdqu	%xmm2, 0x18(%rbp)
// Frames: F183
// 0x1403f058d  4c89f1                         movq	%r14, %rcx
// 0x1403f0590  e8ebad4e00                     callq	0x1408db380 <.text+0x8da380>
// Frames: F184
// 0x1403f0595  4c89f9                         movq	%r15, %rcx
// 0x1403f0598  e8c39e2c00                     callq	0x1406ba460 <.text+0x6b9460>
// 0x1403f059d  84c0                           testb	%al, %al
// 0x1403f059f  742f                           je	0x1403f05d0 <.text+0x3ef5d0>
// Frames: F83 -> F185
// 0x1403f05a1  488b4df8                       movq	-0x8(%rbp), %rcx
// Frames: F186 -> F187 -> F188 -> F189 -> F83 -> F185
// 0x1403f05a5  b001                           movb	$0x1, %al
// 0x1403f05a7  31d2                           xorl	%edx, %edx
// 0x1403f05a9  f0                             lock
// 0x1403f05aa  0fb05118                       cmpxchgb	%dl, 0x18(%rcx)
// Frames: F165 -> F190 -> F189 -> F83 -> F185
// 0x1403f05ae  0f845cfdffff                   je	0x1403f0310 <.text+0x3ef310>
// Frames: F189 -> F83 -> F185
// 0x1403f05b4  4883c118                       addq	$0x18, %rcx
// Frames: F191 -> F189 -> F83 -> F185
// 0x1403f05b8  31d2                           xorl	%edx, %edx
// 0x1403f05ba  e8f1c4cb01                     callq	0x1420acab0 <.text+0x20abab0>
// 0x1403f05bf  e94cfdffff                     jmp	0x1403f0310 <.text+0x3ef310>
// 0x1403f05c4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F192 -> F193 -> F194 -> F195
// 0x1403f05d0  488b4568                       movq	0x68(%rbp), %rax
// 0x1403f05d4  4883781800                     cmpq	$0x0, 0x18(%rax)
// 0x1403f05d9  0f84f5000000                   je	0x1403f06d4 <.text+0x3ef6d4>
// Frames: F196 -> F197 -> F193 -> F194 -> F195
// 0x1403f05df  488b4d78                       movq	0x78(%rbp), %rcx
// 0x1403f05e3  4c89fa                         movq	%r15, %rdx
// 0x1403f05e6  4889c6                         movq	%rax, %rsi
// 0x1403f05e9  e8e2c6ceff                     callq	0x1400dccd0 <.text+0xdbcd0>
// Frames: F198 -> F193 -> F194 -> F195
// 0x1403f05ee  488b0e                         movq	(%rsi), %rcx
// 0x1403f05f1  488b5608                       movq	0x8(%rsi), %rdx
// Frames: F199 -> F200 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f05f5  4989c0                         movq	%rax, %r8
// 0x1403f05f8  49c1e839                       shrq	$0x39, %r8
// 0x1403f05fc  66410f6ec0                     movd	%r8d, %xmm0
// 0x1403f0601  660f60c0                       punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
// 0x1403f0605  f20f70c000                     pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
// 0x1403f060a  660f70c000                     pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
// 0x1403f060f  4c8d41e0                       leaq	-0x20(%rcx), %r8
// 0x1403f0613  4531c9                         xorl	%r9d, %r9d
// Frames: F203 -> F204 -> F205 -> F206 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0616  4821d0                         andq	%rdx, %rax
// 0x1403f0619  f30f6f0c01                     movdqu	(%rcx,%rax), %xmm1
// Frames: F207 -> F208 -> F209 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f061e  660f6fd1                       movdqa	%xmm1, %xmm2
// 0x1403f0622  660f74d0                       pcmpeqb	%xmm0, %xmm2
// 0x1403f0626  66440fd7d2                     pmovmskb	%xmm2, %r10d
// 0x1403f062b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F210 -> F211 -> F209 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0630  664585d2                       testw	%r10w, %r10w
// 0x1403f0634  743a                           je	0x1403f0670 <.text+0x3ef670>
// Frames: F212 -> F213 -> F214 -> F211 -> F209 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0636  f3450fbcda                     tzcntl	%r10d, %r11d
// Frames: F215 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f063b  4901c3                         addq	%rax, %r11
// 0x1403f063e  4921d3                         andq	%rdx, %r11
// Frames: F216 -> F217 -> F218 -> F219 -> F220 -> F221 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0641  4c89de                         movq	%r11, %rsi
// 0x1403f0644  48c1e605                       shlq	$0x5, %rsi
// 0x1403f0648  4c89c3                         movq	%r8, %rbx
// 0x1403f064b  4829f3                         subq	%rsi, %rbx
// Frames: F121 -> F222 -> F223 -> F220 -> F221 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f064e  498b37                         movq	(%r15), %rsi
// Frames: F224 -> F225 -> F209 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0651  458d62ff                       leal	-0x1(%r10), %r12d
// 0x1403f0655  4521d4                         andl	%r10d, %r12d
// 0x1403f0658  4589e2                         movl	%r12d, %r10d
// Frames: F226 -> F121 -> F222 -> F223 -> F220 -> F221 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f065b  483b33                         cmpq	(%rbx), %rsi
// Frames: F221 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f065e  75d0                           jne	0x1403f0630 <.text+0x3ef630>
// 0x1403f0660  eb2e                           jmp	0x1403f0690 <.text+0x3ef690>
// 0x1403f0662  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F207 -> F208 -> F227 -> F228 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0670  660f74ce                       pcmpeqb	%xmm6, %xmm1
// 0x1403f0674  66440fd7d1                     pmovmskb	%xmm1, %r10d
// Frames: F229 -> F228 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0679  4585d2                         testl	%r10d, %r10d
// Frames: F228 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f067c  7556                           jne	0x1403f06d4 <.text+0x3ef6d4>
// Frames: F230 -> F231 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f067e  4c01c8                         addq	%r9, %rax
// 0x1403f0681  4883c010                       addq	$0x10, %rax
// Frames: F232 -> F231 -> F201 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0685  4983c110                       addq	$0x10, %r9
// 0x1403f0689  eb8b                           jmp	0x1403f0616 <.text+0x3ef616>
// 0x1403f068b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F216 -> F217 -> F233 -> F234 -> F235 -> F202 -> F198 -> F193 -> F194 -> F195
// 0x1403f0690  49f7db                         negq	%r11
// 0x1403f0693  49c1e305                       shlq	$0x5, %r11
// Frames: F236 -> F237 -> F238 -> F195
// 0x1403f0697  4a837c19f800                   cmpq	$0x0, -0x8(%rcx,%r11)
// 0x1403f069d  410f95c0                       setne	%r8b
// 0x1403f06a1  498d470c                       leaq	0xc(%r15), %rax
// Frames: F239
// 0x1403f06a5  41837f0800                     cmpl	$0x0, 0x8(%r15)
// 0x1403f06aa  7436                           je	0x1403f06e2 <.text+0x3ef6e2>
// Frames: F240
// 0x1403f06ac  488b4810                       movq	0x10(%rax), %rcx
// 0x1403f06b0  48894dd0                       movq	%rcx, -0x30(%rbp)
// 0x1403f06b4  f30f6f00                       movdqu	(%rax), %xmm0
// 0x1403f06b8  660f7f45c0                     movdqa	%xmm0, -0x40(%rbp)
// Frames: F241
// 0x1403f06bd  4c89742420                     movq	%r14, 0x20(%rsp)
// 0x1403f06c2  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1403f06c6  4c89fa                         movq	%r15, %rdx
// 0x1403f06c9  4c8b4d70                       movq	0x70(%rbp), %r9
// 0x1403f06cd  e8eeb64e00                     callq	0x1408dbdc0 <.text+0x8dadc0>
// 0x1403f06d2  eb2a                           jmp	0x1403f06fe <.text+0x3ef6fe>
// Frames: F239
// 0x1403f06d4  4531c0                         xorl	%r8d, %r8d
// 0x1403f06d7  498d470c                       leaq	0xc(%r15), %rax
// 0x1403f06db  41837f0800                     cmpl	$0x0, 0x8(%r15)
// 0x1403f06e0  75ca                           jne	0x1403f06ac <.text+0x3ef6ac>
// Frames: F242
// 0x1403f06e2  8b4808                         movl	0x8(%rax), %ecx
// 0x1403f06e5  894d38                         movl	%ecx, 0x38(%rbp)
// 0x1403f06e8  488b00                         movq	(%rax), %rax
// 0x1403f06eb  48894530                       movq	%rax, 0x30(%rbp)
// Frames: F243
// 0x1403f06ef  488d4d30                       leaq	0x30(%rbp), %rcx
// 0x1403f06f3  4c89fa                         movq	%r15, %rdx
// 0x1403f06f6  4d89f1                         movq	%r14, %r9
// 0x1403f06f9  e802fd4e00                     callq	0x1408e0400 <.text+0x8df400>
// Frames: F83 -> F185
// 0x1403f06fe  488b4df8                       movq	-0x8(%rbp), %rcx
// Frames: F186 -> F187 -> F188 -> F189 -> F83 -> F185
// 0x1403f0702  b001                           movb	$0x1, %al
// 0x1403f0704  31d2                           xorl	%edx, %edx
// 0x1403f0706  f0                             lock
// 0x1403f0707  0fb05118                       cmpxchgb	%dl, 0x18(%rcx)
// Frames: F165 -> F190 -> F189 -> F83 -> F185
// 0x1403f070b  0f84fffbffff                   je	0x1403f0310 <.text+0x3ef310>
// Frames: F189 -> F83 -> F185
// 0x1403f0711  4883c118                       addq	$0x18, %rcx
// Frames: F191 -> F189 -> F83 -> F185
// 0x1403f0715  31d2                           xorl	%edx, %edx
// 0x1403f0717  e894c3cb01                     callq	0x1420acab0 <.text+0x20abab0>
// 0x1403f071c  e9effbffff                     jmp	0x1403f0310 <.text+0x3ef310>
// Frames: F244
// 0x1403f0721  488b8dc0000000                 movq	0xc0(%rbp), %rcx
// 0x1403f0728  488b95a8000000                 movq	0xa8(%rbp), %rdx
// 0x1403f072f  e8acf2ffff                     callq	0x1403ef9e0 <.text+0x3ee9e0>
// 0x1403f0734  0f28b5e0000000                 movaps	0xe0(%rbp), %xmm6
// 0x1403f073b  4881c478010000                 addq	$0x178, %rsp            # imm = 0x178
// 0x1403f0742  5b                             popq	%rbx
// 0x1403f0743  5f                             popq	%rdi
// 0x1403f0744  5e                             popq	%rsi
// 0x1403f0745  415c                           popq	%r12
// 0x1403f0747  415d                           popq	%r13
// 0x1403f0749  415e                           popq	%r14
// 0x1403f074b  415f                           popq	%r15
// 0x1403f074d  5d                             popq	%rbp
// 0x1403f074e  c3                             retq
// Frames: F153 -> F145 -> F140
// 0x1403f074f  e8cc10cc01                     callq	0x1420b1820 <.text+0x20b0820>
// 0x1403f0754  e9af000000                     jmp	0x1403f0808 <.text+0x3ef808>
// Frames: F245 -> F117
// 0x1403f0759  488d0d4878dd01                 leaq	0x1dd7848(%rip), %rcx   # 0x1421c7fa8
// 0x1403f0760  e8dbe2cb01                     callq	0x1420aea40 <.text+0x20ada40>
// 0x1403f0765  e99e000000                     jmp	0x1403f0808 <.text+0x3ef808>
// Frames: F118 -> F116 -> F117
// 0x1403f076a  48c745d800000000               movq	$0x0, -0x28(%rbp)
// 0x1403f0772  488d054f8fee01                 leaq	0x1ee8f4f(%rip), %rax   # 0x1422d96c8
// 0x1403f0779  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1403f077e  488d9580000000                 leaq	0x80(%rbp), %rdx
// 0x1403f0785  4c8d4dd8                       leaq	-0x28(%rbp), %r9
// 0x1403f0789  31c9                           xorl	%ecx, %ecx
// 0x1403f078b  4d89e0                         movq	%r12, %r8
// 0x1403f078e  e84d0fcc01                     callq	0x1420b16e0 <.text+0x20b06e0>
// 0x1403f0793  eb73                           jmp	0x1403f0808 <.text+0x3ef808>
// Frames: F246 -> F126
// 0x1403f0795  f30f6f45e0                     movdqu	-0x20(%rbp), %xmm0
// 0x1403f079a  660f7f8580000000               movdqa	%xmm0, 0x80(%rbp)
// 0x1403f07a2  488d051778dd01                 leaq	0x1dd7817(%rip), %rax   # 0x1421c7fc0
// 0x1403f07a9  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1403f07ae  488d0d4b75dd01                 leaq	0x1dd754b(%rip), %rcx   # 0x1421c7d00
// 0x1403f07b5  4c8d0d7475dd01                 leaq	0x1dd7574(%rip), %r9    # 0x1421c7d30
// 0x1403f07bc  4c8d8580000000                 leaq	0x80(%rbp), %r8
// 0x1403f07c3  ba2b000000                     movl	$0x2b, %edx
// 0x1403f07c8  e853e7cb01                     callq	0x1420aef20 <.text+0x20adf20>
// 0x1403f07cd  eb39                           jmp	0x1403f0808 <.text+0x3ef808>
// Frames: F245 -> F159
// 0x1403f07cf  488d0d0278dd01                 leaq	0x1dd7802(%rip), %rcx   # 0x1421c7fd8
// 0x1403f07d6  e865e2cb01                     callq	0x1420aea40 <.text+0x20ada40>
// 0x1403f07db  eb2b                           jmp	0x1403f0808 <.text+0x3ef808>
// Frames: F245 -> F160 -> F161
// 0x1403f07dd  488d0d9c94ee01                 leaq	0x1ee949c(%rip), %rcx   # 0x1422d9c80
// 0x1403f07e4  e857e2cb01                     callq	0x1420aea40 <.text+0x20ada40>
// 0x1403f07e9  eb1d                           jmp	0x1403f0808 <.text+0x3ef808>
// Frames: F247 -> F138 -> F139 -> F140
// 0x1403f07eb  b908000000                     movl	$0x8, %ecx
// 0x1403f07f0  ba20000000                     movl	$0x20, %edx
// 0x1403f07f5  e866dfcb01                     callq	0x1420ae760 <.text+0x20ad760>
// 0x1403f07fa  eb0c                           jmp	0x1403f0808 <.text+0x3ef808>
// Frames: F245 -> F176 -> F177 -> F169 -> F161
// 0x1403f07fc  488d0d9594ee01                 leaq	0x1ee9495(%rip), %rcx   # 0x1422d9c98
// 0x1403f0803  e838e2cb01                     callq	0x1420aea40 <.text+0x20ada40>
// 0x1403f0808  0f0b                           ud2
// Frames: F248 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x1403f080a  488d0d8776dd01                 leaq	0x1dd7687(%rip), %rcx   # 0x1421c7e98
// 0x1403f0811  e86ae1cb01                     callq	0x1420ae980 <.text+0x20ad980>
// 0x1403f0816  cc                             int3
// 0x1403f0817  660f1f840000000000             nopw	(%rax,%rax)
// 0x1403f0820  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0825  55                             pushq	%rbp
// 0x1403f0826  4157                           pushq	%r15
// 0x1403f0828  4156                           pushq	%r14
// 0x1403f082a  4155                           pushq	%r13
// 0x1403f082c  4154                           pushq	%r12
// 0x1403f082e  56                             pushq	%rsi
// 0x1403f082f  57                             pushq	%rdi
// 0x1403f0830  53                             pushq	%rbx
// 0x1403f0831  4883ec48                       subq	$0x48, %rsp
// 0x1403f0835  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f083c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x1403f0842  488b85c8000000                 movq	0xc8(%rbp), %rax
// Frames: F74 -> F75 -> F76 -> F77 -> F82 -> F83 -> F83 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x1403f0849  48ff00                         incq	(%rax)
// 0x1403f084c  0f28742430                     movaps	0x30(%rsp), %xmm6
// Frames: F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x1403f0851  4883c448                       addq	$0x48, %rsp
// 0x1403f0855  5b                             popq	%rbx
// 0x1403f0856  5f                             popq	%rdi
// 0x1403f0857  5e                             popq	%rsi
// 0x1403f0858  415c                           popq	%r12
// 0x1403f085a  415d                           popq	%r13
// 0x1403f085c  415e                           popq	%r14
// 0x1403f085e  415f                           popq	%r15
// 0x1403f0860  5d                             popq	%rbp
// 0x1403f0861  c3                             retq
// 0x1403f0862  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1403f0870  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0875  55                             pushq	%rbp
// 0x1403f0876  4157                           pushq	%r15
// 0x1403f0878  4156                           pushq	%r14
// 0x1403f087a  4155                           pushq	%r13
// 0x1403f087c  4154                           pushq	%r12
// 0x1403f087e  56                             pushq	%rsi
// 0x1403f087f  57                             pushq	%rdi
// 0x1403f0880  53                             pushq	%rbx
// 0x1403f0881  4883ec48                       subq	$0x48, %rsp
// 0x1403f0885  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f088c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x1403f0892  488b85a0000000                 movq	0xa0(%rbp), %rax
// 0x1403f0899  488b8db8000000                 movq	0xb8(%rbp), %rcx
// Frames: F140
// 0x1403f08a0  488901                         movq	%rax, (%rcx)
// 0x1403f08a3  488b8598000000                 movq	0x98(%rbp), %rax
// 0x1403f08aa  48894108                       movq	%rax, 0x8(%rcx)
// 0x1403f08ae  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x1403f08b3  4883c448                       addq	$0x48, %rsp
// 0x1403f08b7  5b                             popq	%rbx
// 0x1403f08b8  5f                             popq	%rdi
// 0x1403f08b9  5e                             popq	%rsi
// 0x1403f08ba  415c                           popq	%r12
// 0x1403f08bc  415d                           popq	%r13
// 0x1403f08be  415e                           popq	%r14
// 0x1403f08c0  415f                           popq	%r15
// 0x1403f08c2  5d                             popq	%rbp
// 0x1403f08c3  c3                             retq
// 0x1403f08c4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// 0x1403f08d0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f08d5  55                             pushq	%rbp
// 0x1403f08d6  4157                           pushq	%r15
// 0x1403f08d8  4156                           pushq	%r14
// 0x1403f08da  4155                           pushq	%r13
// 0x1403f08dc  4154                           pushq	%r12
// 0x1403f08de  56                             pushq	%rsi
// 0x1403f08df  57                             pushq	%rdi
// 0x1403f08e0  53                             pushq	%rbx
// 0x1403f08e1  4883ec48                       subq	$0x48, %rsp
// 0x1403f08e5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f08ec  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// Frames: F83 -> F185
// 0x1403f08f2  488b4df8                       movq	-0x8(%rbp), %rcx
// 0x1403f08f6  31d2                           xorl	%edx, %edx
// Frames: F186 -> F187 -> F188 -> F189 -> F83 -> F185
// 0x1403f08f8  b001                           movb	$0x1, %al
// 0x1403f08fa  f0                             lock
// 0x1403f08fb  0fb05118                       cmpxchgb	%dl, 0x18(%rcx)
// Frames: F165 -> F190 -> F189 -> F83 -> F185
// 0x1403f08ff  740b                           je	0x1403f090c <.text+0x3ef90c>
// Frames: F191 -> F189 -> F83 -> F185
// 0x1403f0901  4883c118                       addq	$0x18, %rcx
// 0x1403f0905  31d2                           xorl	%edx, %edx
// 0x1403f0907  e8a4c1cb01                     callq	0x1420acab0 <.text+0x20abab0>
// Frames: F185
// 0x1403f090c  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x1403f0911  4883c448                       addq	$0x48, %rsp
// 0x1403f0915  5b                             popq	%rbx
// 0x1403f0916  5f                             popq	%rdi
// 0x1403f0917  5e                             popq	%rsi
// 0x1403f0918  415c                           popq	%r12
// 0x1403f091a  415d                           popq	%r13
// 0x1403f091c  415e                           popq	%r14
// 0x1403f091e  415f                           popq	%r15
// 0x1403f0920  5d                             popq	%rbp
// 0x1403f0921  c3                             retq
// 0x1403f0922  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1403f0930  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0935  55                             pushq	%rbp
// 0x1403f0936  4157                           pushq	%r15
// 0x1403f0938  4156                           pushq	%r14
// 0x1403f093a  4155                           pushq	%r13
// 0x1403f093c  4154                           pushq	%r12
// 0x1403f093e  56                             pushq	%rsi
// 0x1403f093f  57                             pushq	%rdi
// 0x1403f0940  53                             pushq	%rbx
// 0x1403f0941  4883ec48                       subq	$0x48, %rsp
// 0x1403f0945  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f094c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x1403f0952  488b8dc0000000                 movq	0xc0(%rbp), %rcx
// 0x1403f0959  488b95a8000000                 movq	0xa8(%rbp), %rdx
// Frames: F244
// 0x1403f0960  e87bf0ffff                     callq	0x1403ef9e0 <.text+0x3ee9e0>
// 0x1403f0965  0f28742430                     movaps	0x30(%rsp), %xmm6
// Frames: F1
// 0x1403f096a  4883c448                       addq	$0x48, %rsp
// 0x1403f096e  5b                             popq	%rbx
// 0x1403f096f  5f                             popq	%rdi
// 0x1403f0970  5e                             popq	%rsi
// 0x1403f0971  415c                           popq	%r12
// 0x1403f0973  415d                           popq	%r13
// 0x1403f0975  415e                           popq	%r14
// 0x1403f0977  415f                           popq	%r15
// 0x1403f0979  5d                             popq	%rbp
// 0x1403f097a  c3                             retq
//
// RANGE rva_8dfff0_len_353: RVA 0x8dfff0, 851 PDB bytes; 201 instructions
// Evidence: _assembly/roof-v1/A/pack-000/rva_8dfff0_len_353.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 753:32852: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// Frames: F249
// 0x1408dfff0  55                             pushq	%rbp
// 0x1408dfff1  56                             pushq	%rsi
// 0x1408dfff2  4881ec98000000                 subq	$0x98, %rsp
// 0x1408dfff9  488dac2480000000               leaq	0x80(%rsp), %rbp
// Frames: F250
// 0x1408e0001  48c74510feffffff               movq	$-0x2, 0x10(%rbp)
// 0x1408e0009  e8a2b0ffff                     callq	0x1408db0b0 <.text+0x8da0b0>
// 0x1408e000e  84c0                           testb	%al, %al
// 0x1408e0010  0f84ad010000                   je	0x1408e01c3 <.text+0x8df1c3>
// Frames: F251 -> F252 -> F253 -> F254 -> F255
// 0x1408e0016  488b05b3aa3902                 movq	0x239aab3(%rip), %rax   # 0x142c7aad0
// Frames: F253 -> F254 -> F255
// 0x1408e001d  4883f803                       cmpq	$0x3, %rax
// Frames: F254 -> F255
// 0x1408e0021  0f85a6010000                   jne	0x1408e01cd <.text+0x8df1cd>
// Frames: F255
// 0x1408e0027  488b159aaa3902                 movq	0x239aa9a(%rip), %rdx   # 0x142c7aac8
// 0x1408e002e  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1408e0032  e8496b9eff                     callq	0x1402c6b80 <.text+0x2c5b80>
// Frames: F256 -> F255
// 0x1408e0037  48837dc000                     cmpq	$0x0, -0x40(%rbp)
// 0x1408e003c  0f85dd010000                   jne	0x1408e021f <.text+0x8df21f>
// Frames: F257 -> F255
// 0x1408e0042  488b45c8                       movq	-0x38(%rbp), %rax
// 0x1408e0046  488945b8                       movq	%rax, -0x48(%rbp)
// 0x1408e004a  0fb645d0                       movzbl	-0x30(%rbp), %eax
// 0x1408e004e  88450f                         movb	%al, 0xf(%rbp)
// Frames: F133 -> F134 -> F135 -> F136 -> F258 -> F259 -> F260 -> F261 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e0051  0fb60569aa3902                 movzbl	0x239aa69(%rip), %eax   # 0x142c7aac1
// Frames: F141 -> F135 -> F136 -> F258 -> F259 -> F260 -> F261 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e0058  b90e000000                     movl	$0xe, %ecx
// 0x1408e005d  ba01000000                     movl	$0x1, %edx
// 0x1408e0062  e819a7afff                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F268 -> F259 -> F260 -> F261 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e0067  4885c0                         testq	%rax, %rax
// 0x1408e006a  0f84e8010000                   je	0x1408e0258 <.text+0x8df258>
// Frames: F203 -> F269 -> F270 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e0070  48b95f70617468212829           movabsq	$0x292821687461705f, %rcx # imm = 0x292821687461705F
// 0x1408e007a  48894806                       movq	%rcx, 0x6(%rax)
// 0x1408e007e  48b96d6f64756c655f70           movabsq	$0x705f656c75646f6d, %rcx # imm = 0x705F656C75646F6D
// 0x1408e0088  488908                         movq	%rcx, (%rax)
// Frames: F271 -> F266 -> F267 -> F255
// 0x1408e008b  48c745c00e000000               movq	$0xe, -0x40(%rbp)
// 0x1408e0093  488945c8                       movq	%rax, -0x38(%rbp)
// 0x1408e0097  48c745d00e000000               movq	$0xe, -0x30(%rbp)
// Frames: F272 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e009f  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1408e00a3  ba0e000000                     movl	$0xe, %edx
// 0x1408e00a8  41b817000000                   movl	$0x17, %r8d
// 0x1408e00ae  e8edff7b01                     callq	0x1420a00a0 <.text+0x209f0a0>
// Frames: F279 -> F280 -> F281 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00b3  488b45c8                       movq	-0x38(%rbp), %rax
// Frames: F282 -> F283 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00b7  488b55d0                       movq	-0x30(%rbp), %rdx
// Frames: F284 -> F281 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00bb  0f100549a49f01                 movups	0x19fa449(%rip), %xmm0  # 0x1422da50b
// 0x1408e00c2  0f110410                       movups	%xmm0, (%rax,%rdx)
// 0x1408e00c6  48b9656c3a3a5761726e           movabsq	$0x6e7261573a3a6c65, %rcx # imm = 0x6E7261573A3A6C65
// 0x1408e00d0  48894c100f                     movq	%rcx, 0xf(%rax,%rdx)
// Frames: F285 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00d5  4883c217                       addq	$0x17, %rdx
// Frames: F286 -> F255
// 0x1408e00d9  48895500                       movq	%rdx, (%rbp)
// 0x1408e00dd  0f1045c0                       movups	-0x40(%rbp), %xmm0
// 0x1408e00e1  0f2945f0                       movaps	%xmm0, -0x10(%rbp)
// Frames: F287 -> F288 -> F289 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00e5  488b45f0                       movq	-0x10(%rbp), %rax
// Frames: F290 -> F288 -> F289 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00e9  4829d0                         subq	%rdx, %rax
// Frames: F288 -> F289 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00ec  4883f819                       cmpq	$0x19, %rax
// Frames: F289 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00f0  7713                           ja	0x1408e0105 <.text+0x8df105>
// Frames: F272 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e00f2  488d4df0                       leaq	-0x10(%rbp), %rcx
// 0x1408e00f6  41b81a000000                   movl	$0x1a, %r8d
// 0x1408e00fc  e89fff7b01                     callq	0x1420a00a0 <.text+0x209f0a0>
// Frames: F282 -> F283 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e0101  488b5500                       movq	(%rbp), %rdx
// Frames: F279 -> F280 -> F281 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e0105  488b45f8                       movq	-0x8(%rbp), %rax
// Frames: F284 -> F281 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e0109  0f10051ca49f01                 movups	0x19fa41c(%rip), %xmm0  # 0x1422da52c
// 0x1408e0110  0f1144100a                     movups	%xmm0, 0xa(%rax,%rdx)
// 0x1408e0115  0f100506a49f01                 movups	0x19fa406(%rip), %xmm0  # 0x1422da522
// 0x1408e011c  0f110410                       movups	%xmm0, (%rax,%rdx)
// Frames: F285 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e0120  4883c21a                       addq	$0x1a, %rdx
// 0x1408e0124  48895500                       movq	%rdx, (%rbp)
// Frames: F286 -> F255
// 0x1408e0128  488955d0                       movq	%rdx, -0x30(%rbp)
// 0x1408e012c  0f2845f0                       movaps	-0x10(%rbp), %xmm0
// 0x1408e0130  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// 0x1408e0134  488b45b8                       movq	-0x48(%rbp), %rax
// Frames: F79 -> F291 -> F255
// 0x1408e0138  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F292 -> F255
// 0x1408e013c  488d55c0                       leaq	-0x40(%rbp), %rdx
// 0x1408e0140  e81b459e00                     callq	0x1412c4660 <.text+0x12c3660>
// Frames: F255
// 0x1408e0145  84c0                           testb	%al, %al
// 0x1408e0147  7556                           jne	0x1408e019f <.text+0x8df19f>
// Frames: F149 -> F150 -> F293 -> F255
// 0x1408e0149  488b05d8213902                 movq	0x23921d8(%rip), %rax   # 0x142c72328
// 0x1408e0150  488b00                         movq	(%rax), %rax
// Frames: F294 -> F255
// 0x1408e0153  4883f802                       cmpq	$0x2, %rax
// Frames: F255
// 0x1408e0157  7246                           jb	0x1408e019f <.text+0x8df19f>
// Frames: F295 -> F255
// 0x1408e0159  488d05e0a39f01                 leaq	0x19fa3e0(%rip), %rax   # 0x1422da540
// 0x1408e0160  488945c0                       movq	%rax, -0x40(%rbp)
// 0x1408e0164  48c745c801000000               movq	$0x1, -0x38(%rbp)
// 0x1408e016c  488d4508                       leaq	0x8(%rbp), %rax
// 0x1408e0170  488945d0                       movq	%rax, -0x30(%rbp)
// 0x1408e0174  0f57c0                         xorps	%xmm0, %xmm0
// 0x1408e0177  0f1145d8                       movups	%xmm0, -0x28(%rbp)
// Frames: F296 -> F255
// 0x1408e017b  48c744242000000000             movq	$0x0, 0x20(%rsp)
// 0x1408e0184  4c8d05eda39f01                 leaq	0x19fa3ed(%rip), %r8    # 0x1422da578
// 0x1408e018b  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1408e018f  ba02000000                     movl	$0x2, %edx
// 0x1408e0194  41b929000000                   movl	$0x29, %r9d
// 0x1408e019a  e891e6f5ff                     callq	0x14083e830 <.text+0x83d830>
// Frames: F297 -> F298 -> F83 -> F255
// 0x1408e019f  807d0f00                       cmpb	$0x0, 0xf(%rbp)
// 0x1408e01a3  488b4db8                       movq	-0x48(%rbp), %rcx
// 0x1408e01a7  7512                           jne	0x1408e01bb <.text+0x8df1bb>
// Frames: F149 -> F150 -> F299 -> F300 -> F301 -> F297 -> F298 -> F83 -> F255
// 0x1408e01a9  488b0548223902                 movq	0x2392248(%rip), %rax   # 0x142c723f8
// 0x1408e01b0  488b00                         movq	(%rax), %rax
// Frames: F299 -> F300 -> F301 -> F297 -> F298 -> F83 -> F255
// 0x1408e01b3  48d1e0                         shlq	%rax
// 0x1408e01b6  4885c0                         testq	%rax, %rax
// 0x1408e01b9  754f                           jne	0x1408e020a <.text+0x8df20a>
// Frames: F302 -> F303 -> F304 -> F305 -> F83 -> F255
// 0x1408e01bb  31c0                           xorl	%eax, %eax
// 0x1408e01bd  8601                           xchgb	%al, (%rcx)
// Frames: F304 -> F305 -> F83 -> F255
// 0x1408e01bf  3c02                           cmpb	$0x2, %al
// 0x1408e01c1  7440                           je	0x1408e0203 <.text+0x8df203>
// Frames: F306
// 0x1408e01c3  4881c498000000                 addq	$0x98, %rsp
// 0x1408e01ca  5e                             popq	%rsi
// 0x1408e01cb  5d                             popq	%rbp
// 0x1408e01cc  c3                             retq
// Frames: F307 -> F255
// 0x1408e01cd  c645f001                       movb	$0x1, -0x10(%rbp)
// 0x1408e01d1  488d45f0                       leaq	-0x10(%rbp), %rax
// Frames: F308 -> F255
// 0x1408e01d5  488945c0                       movq	%rax, -0x40(%rbp)
// 0x1408e01d9  488d0500a39f01                 leaq	0x19fa300(%rip), %rax   # 0x1422da4e0
// 0x1408e01e0  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1408e01e5  488d0de4a83902                 leaq	0x239a8e4(%rip), %rcx   # 0x142c7aad0
// 0x1408e01ec  4c8d0dfd9f9f01                 leaq	0x19f9ffd(%rip), %r9    # 0x1422da1f0
// 0x1408e01f3  4c8d45c0                       leaq	-0x40(%rbp), %r8
// 0x1408e01f7  31d2                           xorl	%edx, %edx
// 0x1408e01f9  e8e2db7c01                     callq	0x1420adde0 <.text+0x20acde0>
// 0x1408e01fe  e924feffff                     jmp	0x1408e0027 <.text+0x8df027>
// Frames: F309 -> F305 -> F83 -> F255
// 0x1408e0203  e8c8db7c01                     callq	0x1420addd0 <.text+0x20acdd0>
// 0x1408e0208  ebb9                           jmp	0x1408e01c3 <.text+0x8df1c3>
// Frames: F310 -> F300 -> F301 -> F297 -> F298 -> F83 -> F255
// 0x1408e020a  4889ce                         movq	%rcx, %rsi
// 0x1408e020d  e84eda7c01                     callq	0x1420adc60 <.text+0x20acc60>
// 0x1408e0212  4889f1                         movq	%rsi, %rcx
// Frames: F297 -> F298 -> F83 -> F255
// 0x1408e0215  84c0                           testb	%al, %al
// 0x1408e0217  75a2                           jne	0x1408e01bb <.text+0x8df1bb>
// Frames: F311 -> F312 -> F313 -> F298 -> F83 -> F255
// 0x1408e0219  c6410101                       movb	$0x1, 0x1(%rcx)
// 0x1408e021d  eb9c                           jmp	0x1408e01bb <.text+0x8df1bb>
// Frames: F66 -> F255
// 0x1408e021f  488b45c8                       movq	-0x38(%rbp), %rax
// 0x1408e0223  0fb64dd0                       movzbl	-0x30(%rbp), %ecx
// 0x1408e0227  488945f0                       movq	%rax, -0x10(%rbp)
// 0x1408e022b  884df8                         movb	%cl, -0x8(%rbp)
// 0x1408e022e  488d05aba29f01                 leaq	0x19fa2ab(%rip), %rax   # 0x1422da4e0
// 0x1408e0235  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1408e023a  488d0db7a29f01                 leaq	0x19fa2b7(%rip), %rcx   # 0x1422da4f8
// 0x1408e0241  4c8d0d38a09f01                 leaq	0x19fa038(%rip), %r9    # 0x1422da280
// 0x1408e0248  4c8d45f0                       leaq	-0x10(%rbp), %r8
// 0x1408e024c  ba13000000                     movl	$0x13, %edx
// 0x1408e0251  e8caec7c01                     callq	0x1420aef20 <.text+0x20adf20>
// 0x1408e0256  eb0f                           jmp	0x1408e0267 <.text+0x8df267>
// Frames: F314 -> F259 -> F260 -> F261 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e0258  b901000000                     movl	$0x1, %ecx
// 0x1408e025d  ba0e000000                     movl	$0xe, %edx
// 0x1408e0262  e8f9e47c01                     callq	0x1420ae760 <.text+0x20ad760>
// 0x1408e0267  0f0b                           ud2
// 0x1408e0269  0f1f8000000000                 nopl	(%rax)
// 0x1408e0270  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e0275  55                             pushq	%rbp
// 0x1408e0276  56                             pushq	%rsi
// 0x1408e0277  4883ec38                       subq	$0x38, %rsp
// 0x1408e027b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1408e0282  488b4db8                       movq	-0x48(%rbp), %rcx
// 0x1408e0286  0fb6550f                       movzbl	0xf(%rbp), %edx
// Frames: F255
// 0x1408e028a  e881b580ff                     callq	0x1400eb810 <.text+0xea810>
// Frames: F249
// 0x1408e028f  90                             nop
// 0x1408e0290  4883c438                       addq	$0x38, %rsp
// 0x1408e0294  5e                             popq	%rsi
// 0x1408e0295  5d                             popq	%rbp
// 0x1408e0296  c3                             retq
// 0x1408e0297  660f1f840000000000             nopw	(%rax,%rax)
// 0x1408e02a0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e02a5  55                             pushq	%rbp
// 0x1408e02a6  56                             pushq	%rsi
// 0x1408e02a7  4883ec38                       subq	$0x38, %rsp
// 0x1408e02ab  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F83 -> F315 -> F255
// 0x1408e02b2  488b55c0                       movq	-0x40(%rbp), %rdx
// Frames: F316 -> F317 -> F83 -> F83 -> F83 -> F315 -> F255
// 0x1408e02b6  4885d2                         testq	%rdx, %rdx
// 0x1408e02b9  740f                           je	0x1408e02ca <.text+0x8df2ca>
// Frames: F83 -> F315 -> F255
// 0x1408e02bb  488b4dc8                       movq	-0x38(%rbp), %rcx
// Frames: F318 -> F319 -> F320 -> F83 -> F83 -> F83 -> F315 -> F255
// 0x1408e02bf  41b801000000                   movl	$0x1, %r8d
// 0x1408e02c5  e8c6a4afff                     callq	0x1403da790 <.text+0x3d9790>
// 0x1408e02ca  90                             nop
// 0x1408e02cb  4883c438                       addq	$0x38, %rsp
// 0x1408e02cf  5e                             popq	%rsi
// 0x1408e02d0  5d                             popq	%rbp
// 0x1408e02d1  c3                             retq
// 0x1408e02d2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1408e02e0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e02e5  55                             pushq	%rbp
// 0x1408e02e6  56                             pushq	%rsi
// 0x1408e02e7  4883ec38                       subq	$0x38, %rsp
// 0x1408e02eb  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F83 -> F315 -> F255
// 0x1408e02f2  488b55f0                       movq	-0x10(%rbp), %rdx
// Frames: F316 -> F317 -> F83 -> F83 -> F83 -> F315 -> F255
// 0x1408e02f6  4885d2                         testq	%rdx, %rdx
// 0x1408e02f9  740f                           je	0x1408e030a <.text+0x8df30a>
// Frames: F83 -> F315 -> F255
// 0x1408e02fb  488b4df8                       movq	-0x8(%rbp), %rcx
// Frames: F318 -> F319 -> F320 -> F83 -> F83 -> F83 -> F315 -> F255
// 0x1408e02ff  41b801000000                   movl	$0x1, %r8d
// 0x1408e0305  e886a4afff                     callq	0x1403da790 <.text+0x3d9790>
// 0x1408e030a  90                             nop
// 0x1408e030b  4883c438                       addq	$0x38, %rsp
// 0x1408e030f  5e                             popq	%rsi
// 0x1408e0310  5d                             popq	%rbp
// 0x1408e0311  c3                             retq
// 0x1408e0312  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1408e0320  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e0325  55                             pushq	%rbp
// 0x1408e0326  56                             pushq	%rsi
// 0x1408e0327  4883ec38                       subq	$0x38, %rsp
// 0x1408e032b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1408e0332  488d4df0                       leaq	-0x10(%rbp), %rcx
// Frames: F66 -> F255
// 0x1408e0336  e8f501a8ff                     callq	0x140360530 <.text+0x35f530>
// Frames: F321 -> F255
// 0x1408e033b  90                             nop
// 0x1408e033c  4883c438                       addq	$0x38, %rsp
// 0x1408e0340  5e                             popq	%rsi
// 0x1408e0341  5d                             popq	%rbp
// 0x1408e0342  c3                             retq
//
// RANGE rva_8e1820_len_353: RVA 0x8e1820, 851 PDB bytes; 201 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_8e1820_len_353.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 754:11396: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// Frames: F249
// 0x1408e1820  55                             pushq	%rbp
// 0x1408e1821  56                             pushq	%rsi
// 0x1408e1822  4881ec98000000                 subq	$0x98, %rsp
// 0x1408e1829  488dac2480000000               leaq	0x80(%rsp), %rbp
// Frames: F250
// 0x1408e1831  48c74510feffffff               movq	$-0x2, 0x10(%rbp)
// 0x1408e1839  e87298ffff                     callq	0x1408db0b0 <.text+0x8da0b0>
// 0x1408e183e  84c0                           testb	%al, %al
// 0x1408e1840  0f84ad010000                   je	0x1408e19f3 <.text+0x8e09f3>
// Frames: F251 -> F252 -> F253 -> F254 -> F255
// 0x1408e1846  488b0583923902                 movq	0x2399283(%rip), %rax   # 0x142c7aad0
// Frames: F253 -> F254 -> F255
// 0x1408e184d  4883f803                       cmpq	$0x3, %rax
// Frames: F254 -> F255
// 0x1408e1851  0f85a6010000                   jne	0x1408e19fd <.text+0x8e09fd>
// Frames: F255
// 0x1408e1857  488b156a923902                 movq	0x239926a(%rip), %rdx   # 0x142c7aac8
// 0x1408e185e  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1408e1862  e819539eff                     callq	0x1402c6b80 <.text+0x2c5b80>
// Frames: F256 -> F255
// 0x1408e1867  48837dc000                     cmpq	$0x0, -0x40(%rbp)
// 0x1408e186c  0f85dd010000                   jne	0x1408e1a4f <.text+0x8e0a4f>
// Frames: F257 -> F255
// 0x1408e1872  488b45c8                       movq	-0x38(%rbp), %rax
// 0x1408e1876  488945b8                       movq	%rax, -0x48(%rbp)
// 0x1408e187a  0fb645d0                       movzbl	-0x30(%rbp), %eax
// 0x1408e187e  88450f                         movb	%al, 0xf(%rbp)
// Frames: F133 -> F134 -> F135 -> F136 -> F258 -> F259 -> F260 -> F261 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e1881  0fb60539923902                 movzbl	0x2399239(%rip), %eax   # 0x142c7aac1
// Frames: F141 -> F135 -> F136 -> F258 -> F259 -> F260 -> F261 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e1888  b90e000000                     movl	$0xe, %ecx
// 0x1408e188d  ba01000000                     movl	$0x1, %edx
// 0x1408e1892  e8e98eafff                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F268 -> F259 -> F260 -> F261 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e1897  4885c0                         testq	%rax, %rax
// 0x1408e189a  0f84e8010000                   je	0x1408e1a88 <.text+0x8e0a88>
// Frames: F203 -> F269 -> F270 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e18a0  48b95f70617468212829           movabsq	$0x292821687461705f, %rcx # imm = 0x292821687461705F
// 0x1408e18aa  48894806                       movq	%rcx, 0x6(%rax)
// 0x1408e18ae  48b96d6f64756c655f70           movabsq	$0x705f656c75646f6d, %rcx # imm = 0x705F656C75646F6D
// 0x1408e18b8  488908                         movq	%rcx, (%rax)
// Frames: F271 -> F266 -> F267 -> F255
// 0x1408e18bb  48c745c00e000000               movq	$0xe, -0x40(%rbp)
// 0x1408e18c3  488945c8                       movq	%rax, -0x38(%rbp)
// 0x1408e18c7  48c745d00e000000               movq	$0xe, -0x30(%rbp)
// Frames: F272 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e18cf  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1408e18d3  ba0e000000                     movl	$0xe, %edx
// 0x1408e18d8  41b817000000                   movl	$0x17, %r8d
// 0x1408e18de  e8bde77b01                     callq	0x1420a00a0 <.text+0x209f0a0>
// Frames: F279 -> F280 -> F281 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e18e3  488b45c8                       movq	-0x38(%rbp), %rax
// Frames: F282 -> F283 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e18e7  488b55d0                       movq	-0x30(%rbp), %rdx
// Frames: F284 -> F281 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e18eb  0f1005f18e9f01                 movups	0x19f8ef1(%rip), %xmm0  # 0x1422da7e3
// 0x1408e18f2  0f110410                       movups	%xmm0, (%rax,%rdx)
// 0x1408e18f6  48b9656c3a3a5761726e           movabsq	$0x6e7261573a3a6c65, %rcx # imm = 0x6E7261573A3A6C65
// 0x1408e1900  48894c100f                     movq	%rcx, 0xf(%rax,%rdx)
// Frames: F285 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1905  4883c217                       addq	$0x17, %rdx
// Frames: F286 -> F255
// 0x1408e1909  48895500                       movq	%rdx, (%rbp)
// 0x1408e190d  0f1045c0                       movups	-0x40(%rbp), %xmm0
// 0x1408e1911  0f2945f0                       movaps	%xmm0, -0x10(%rbp)
// Frames: F287 -> F288 -> F289 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1915  488b45f0                       movq	-0x10(%rbp), %rax
// Frames: F290 -> F288 -> F289 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1919  4829d0                         subq	%rdx, %rax
// Frames: F288 -> F289 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e191c  4883f819                       cmpq	$0x19, %rax
// Frames: F289 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1920  7713                           ja	0x1408e1935 <.text+0x8e0935>
// Frames: F272 -> F273 -> F274 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1922  488d4df0                       leaq	-0x10(%rbp), %rcx
// 0x1408e1926  41b81a000000                   movl	$0x1a, %r8d
// 0x1408e192c  e86fe77b01                     callq	0x1420a00a0 <.text+0x209f0a0>
// Frames: F282 -> F283 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1931  488b5500                       movq	(%rbp), %rdx
// Frames: F279 -> F280 -> F281 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1935  488b45f8                       movq	-0x8(%rbp), %rax
// Frames: F284 -> F281 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1939  0f1005c48e9f01                 movups	0x19f8ec4(%rip), %xmm0  # 0x1422da804
// 0x1408e1940  0f1144100a                     movups	%xmm0, 0xa(%rax,%rdx)
// 0x1408e1945  0f1005ae8e9f01                 movups	0x19f8eae(%rip), %xmm0  # 0x1422da7fa
// 0x1408e194c  0f110410                       movups	%xmm0, (%rax,%rdx)
// Frames: F285 -> F275 -> F276 -> F277 -> F278 -> F255
// 0x1408e1950  4883c21a                       addq	$0x1a, %rdx
// 0x1408e1954  48895500                       movq	%rdx, (%rbp)
// Frames: F286 -> F255
// 0x1408e1958  488955d0                       movq	%rdx, -0x30(%rbp)
// 0x1408e195c  0f2845f0                       movaps	-0x10(%rbp), %xmm0
// 0x1408e1960  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// 0x1408e1964  488b45b8                       movq	-0x48(%rbp), %rax
// Frames: F79 -> F291 -> F255
// 0x1408e1968  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F292 -> F255
// 0x1408e196c  488d55c0                       leaq	-0x40(%rbp), %rdx
// 0x1408e1970  e8eb2c9e00                     callq	0x1412c4660 <.text+0x12c3660>
// Frames: F255
// 0x1408e1975  84c0                           testb	%al, %al
// 0x1408e1977  7556                           jne	0x1408e19cf <.text+0x8e09cf>
// Frames: F149 -> F150 -> F293 -> F255
// 0x1408e1979  488b05a8093902                 movq	0x23909a8(%rip), %rax   # 0x142c72328
// 0x1408e1980  488b00                         movq	(%rax), %rax
// Frames: F294 -> F255
// 0x1408e1983  4883f802                       cmpq	$0x2, %rax
// Frames: F255
// 0x1408e1987  7246                           jb	0x1408e19cf <.text+0x8e09cf>
// Frames: F295 -> F255
// 0x1408e1989  488d05888e9f01                 leaq	0x19f8e88(%rip), %rax   # 0x1422da818
// 0x1408e1990  488945c0                       movq	%rax, -0x40(%rbp)
// 0x1408e1994  48c745c801000000               movq	$0x1, -0x38(%rbp)
// 0x1408e199c  488d4508                       leaq	0x8(%rbp), %rax
// 0x1408e19a0  488945d0                       movq	%rax, -0x30(%rbp)
// 0x1408e19a4  0f57c0                         xorps	%xmm0, %xmm0
// 0x1408e19a7  0f1145d8                       movups	%xmm0, -0x28(%rbp)
// Frames: F296 -> F255
// 0x1408e19ab  48c744242000000000             movq	$0x0, 0x20(%rsp)
// 0x1408e19b4  4c8d05958e9f01                 leaq	0x19f8e95(%rip), %r8    # 0x1422da850
// 0x1408e19bb  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1408e19bf  ba02000000                     movl	$0x2, %edx
// 0x1408e19c4  41b929000000                   movl	$0x29, %r9d
// 0x1408e19ca  e861cef5ff                     callq	0x14083e830 <.text+0x83d830>
// Frames: F297 -> F298 -> F83 -> F255
// 0x1408e19cf  807d0f00                       cmpb	$0x0, 0xf(%rbp)
// 0x1408e19d3  488b4db8                       movq	-0x48(%rbp), %rcx
// 0x1408e19d7  7512                           jne	0x1408e19eb <.text+0x8e09eb>
// Frames: F149 -> F150 -> F299 -> F300 -> F301 -> F297 -> F298 -> F83 -> F255
// 0x1408e19d9  488b05180a3902                 movq	0x2390a18(%rip), %rax   # 0x142c723f8
// 0x1408e19e0  488b00                         movq	(%rax), %rax
// Frames: F299 -> F300 -> F301 -> F297 -> F298 -> F83 -> F255
// 0x1408e19e3  48d1e0                         shlq	%rax
// 0x1408e19e6  4885c0                         testq	%rax, %rax
// 0x1408e19e9  754f                           jne	0x1408e1a3a <.text+0x8e0a3a>
// Frames: F302 -> F303 -> F304 -> F305 -> F83 -> F255
// 0x1408e19eb  31c0                           xorl	%eax, %eax
// 0x1408e19ed  8601                           xchgb	%al, (%rcx)
// Frames: F304 -> F305 -> F83 -> F255
// 0x1408e19ef  3c02                           cmpb	$0x2, %al
// 0x1408e19f1  7440                           je	0x1408e1a33 <.text+0x8e0a33>
// Frames: F306
// 0x1408e19f3  4881c498000000                 addq	$0x98, %rsp
// 0x1408e19fa  5e                             popq	%rsi
// 0x1408e19fb  5d                             popq	%rbp
// 0x1408e19fc  c3                             retq
// Frames: F307 -> F255
// 0x1408e19fd  c645f001                       movb	$0x1, -0x10(%rbp)
// 0x1408e1a01  488d45f0                       leaq	-0x10(%rbp), %rax
// Frames: F308 -> F255
// 0x1408e1a05  488945c0                       movq	%rax, -0x40(%rbp)
// 0x1408e1a09  488d05a88d9f01                 leaq	0x19f8da8(%rip), %rax   # 0x1422da7b8
// 0x1408e1a10  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1408e1a15  488d0db4903902                 leaq	0x23990b4(%rip), %rcx   # 0x142c7aad0
// 0x1408e1a1c  4c8d0d8d8b9f01                 leaq	0x19f8b8d(%rip), %r9    # 0x1422da5b0
// 0x1408e1a23  4c8d45c0                       leaq	-0x40(%rbp), %r8
// 0x1408e1a27  31d2                           xorl	%edx, %edx
// 0x1408e1a29  e8b2c37c01                     callq	0x1420adde0 <.text+0x20acde0>
// 0x1408e1a2e  e924feffff                     jmp	0x1408e1857 <.text+0x8e0857>
// Frames: F309 -> F305 -> F83 -> F255
// 0x1408e1a33  e898c37c01                     callq	0x1420addd0 <.text+0x20acdd0>
// 0x1408e1a38  ebb9                           jmp	0x1408e19f3 <.text+0x8e09f3>
// Frames: F310 -> F300 -> F301 -> F297 -> F298 -> F83 -> F255
// 0x1408e1a3a  4889ce                         movq	%rcx, %rsi
// 0x1408e1a3d  e81ec27c01                     callq	0x1420adc60 <.text+0x20acc60>
// 0x1408e1a42  4889f1                         movq	%rsi, %rcx
// Frames: F297 -> F298 -> F83 -> F255
// 0x1408e1a45  84c0                           testb	%al, %al
// 0x1408e1a47  75a2                           jne	0x1408e19eb <.text+0x8e09eb>
// Frames: F311 -> F312 -> F313 -> F298 -> F83 -> F255
// 0x1408e1a49  c6410101                       movb	$0x1, 0x1(%rcx)
// 0x1408e1a4d  eb9c                           jmp	0x1408e19eb <.text+0x8e09eb>
// Frames: F66 -> F255
// 0x1408e1a4f  488b45c8                       movq	-0x38(%rbp), %rax
// 0x1408e1a53  0fb64dd0                       movzbl	-0x30(%rbp), %ecx
// 0x1408e1a57  488945f0                       movq	%rax, -0x10(%rbp)
// 0x1408e1a5b  884df8                         movb	%cl, -0x8(%rbp)
// 0x1408e1a5e  488d05538d9f01                 leaq	0x19f8d53(%rip), %rax   # 0x1422da7b8
// 0x1408e1a65  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1408e1a6a  488d0d5f8d9f01                 leaq	0x19f8d5f(%rip), %rcx   # 0x1422da7d0
// 0x1408e1a71  4c8d0dc88b9f01                 leaq	0x19f8bc8(%rip), %r9    # 0x1422da640
// 0x1408e1a78  4c8d45f0                       leaq	-0x10(%rbp), %r8
// 0x1408e1a7c  ba13000000                     movl	$0x13, %edx
// 0x1408e1a81  e89ad47c01                     callq	0x1420aef20 <.text+0x20adf20>
// 0x1408e1a86  eb0f                           jmp	0x1408e1a97 <.text+0x8e0a97>
// Frames: F314 -> F259 -> F260 -> F261 -> F262 -> F263 -> F264 -> F265 -> F266 -> F267 -> F255
// 0x1408e1a88  b901000000                     movl	$0x1, %ecx
// 0x1408e1a8d  ba0e000000                     movl	$0xe, %edx
// 0x1408e1a92  e8c9cc7c01                     callq	0x1420ae760 <.text+0x20ad760>
// 0x1408e1a97  0f0b                           ud2
// 0x1408e1a99  0f1f8000000000                 nopl	(%rax)
// 0x1408e1aa0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e1aa5  55                             pushq	%rbp
// 0x1408e1aa6  56                             pushq	%rsi
// 0x1408e1aa7  4883ec38                       subq	$0x38, %rsp
// 0x1408e1aab  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1408e1ab2  488b4db8                       movq	-0x48(%rbp), %rcx
// 0x1408e1ab6  0fb6550f                       movzbl	0xf(%rbp), %edx
// Frames: F255
// 0x1408e1aba  e8519d80ff                     callq	0x1400eb810 <.text+0xea810>
// Frames: F249
// 0x1408e1abf  90                             nop
// 0x1408e1ac0  4883c438                       addq	$0x38, %rsp
// 0x1408e1ac4  5e                             popq	%rsi
// 0x1408e1ac5  5d                             popq	%rbp
// 0x1408e1ac6  c3                             retq
// 0x1408e1ac7  660f1f840000000000             nopw	(%rax,%rax)
// 0x1408e1ad0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e1ad5  55                             pushq	%rbp
// 0x1408e1ad6  56                             pushq	%rsi
// 0x1408e1ad7  4883ec38                       subq	$0x38, %rsp
// 0x1408e1adb  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F83 -> F315 -> F255
// 0x1408e1ae2  488b55c0                       movq	-0x40(%rbp), %rdx
// Frames: F316 -> F317 -> F83 -> F83 -> F83 -> F315 -> F255
// 0x1408e1ae6  4885d2                         testq	%rdx, %rdx
// 0x1408e1ae9  740f                           je	0x1408e1afa <.text+0x8e0afa>
// Frames: F83 -> F315 -> F255
// 0x1408e1aeb  488b4dc8                       movq	-0x38(%rbp), %rcx
// Frames: F318 -> F319 -> F320 -> F83 -> F83 -> F83 -> F315 -> F255
// 0x1408e1aef  41b801000000                   movl	$0x1, %r8d
// 0x1408e1af5  e8968cafff                     callq	0x1403da790 <.text+0x3d9790>
// 0x1408e1afa  90                             nop
// 0x1408e1afb  4883c438                       addq	$0x38, %rsp
// 0x1408e1aff  5e                             popq	%rsi
// 0x1408e1b00  5d                             popq	%rbp
// 0x1408e1b01  c3                             retq
// 0x1408e1b02  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1408e1b10  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e1b15  55                             pushq	%rbp
// 0x1408e1b16  56                             pushq	%rsi
// 0x1408e1b17  4883ec38                       subq	$0x38, %rsp
// 0x1408e1b1b  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F83 -> F315 -> F255
// 0x1408e1b22  488b55f0                       movq	-0x10(%rbp), %rdx
// Frames: F316 -> F317 -> F83 -> F83 -> F83 -> F315 -> F255
// 0x1408e1b26  4885d2                         testq	%rdx, %rdx
// 0x1408e1b29  740f                           je	0x1408e1b3a <.text+0x8e0b3a>
// Frames: F83 -> F315 -> F255
// 0x1408e1b2b  488b4df8                       movq	-0x8(%rbp), %rcx
// Frames: F318 -> F319 -> F320 -> F83 -> F83 -> F83 -> F315 -> F255
// 0x1408e1b2f  41b801000000                   movl	$0x1, %r8d
// 0x1408e1b35  e8568cafff                     callq	0x1403da790 <.text+0x3d9790>
// 0x1408e1b3a  90                             nop
// 0x1408e1b3b  4883c438                       addq	$0x38, %rsp
// 0x1408e1b3f  5e                             popq	%rsi
// 0x1408e1b40  5d                             popq	%rbp
// 0x1408e1b41  c3                             retq
// 0x1408e1b42  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1408e1b50  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1408e1b55  55                             pushq	%rbp
// 0x1408e1b56  56                             pushq	%rsi
// 0x1408e1b57  4883ec38                       subq	$0x38, %rsp
// 0x1408e1b5b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1408e1b62  488d4df0                       leaq	-0x10(%rbp), %rcx
// Frames: F66 -> F255
// 0x1408e1b66  e8c5e9a7ff                     callq	0x140360530 <.text+0x35f530>
// Frames: F321 -> F255
// 0x1408e1b6b  90                             nop
// 0x1408e1b6c  4883c438                       addq	$0x38, %rsp
// 0x1408e1b70  5e                             popq	%rsi
// 0x1408e1b71  5d                             popq	%rbp
// 0x1408e1b72  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/assemble_roof_tiles.rs:11:0 assemble_roof_tiles
// F2: crates/systems/roof/src/visual/assemble_roof_tiles.rs:20:0 assemble_roof_tiles
// F3: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F4: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:222:0 core::str::impl$0::is_char_boundary
// F5: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:334:0 core::str::traits::impl$9::get
// F6: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:365:0 core::str::traits::impl$9::index
// F7: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:61:0 core::str::traits::impl$4::index
// F8: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F10: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F11: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:226:0 core::str::impl$0::is_char_boundary
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1103:0 core::num::impl$6::is_utf8_char_boundary
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:238:0 core::str::impl$0::is_char_boundary
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:367:0 core::str::traits::impl$9::index
// F15: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:403:0 core::str::traits::impl$10::get
// F16: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:436:0 core::str::traits::impl$10::index
// F17: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F18: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:438:0 core::str::traits::impl$10::index
// F19: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:964:0 core::ptr::const_ptr::impl$0::add
// F20: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:213:0 core::str::traits::impl$7::get_unchecked
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:425:0 core::str::traits::impl$10::get_unchecked
// F22: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:406:0 core::str::traits::impl$10::get
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:214:0 core::str::traits::impl$7::get_unchecked
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:85:0 core::str::validations::next_code_point_reverse
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/iter.rs:134:0 core::str::iter::impl$2::next_back
// F26: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/iter.rs:222:0 core::str::iter::impl$6::next_back
// F27: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:695:0 core::str::pattern::impl$11::next_back
// F28: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:303:0 core::str::pattern::ReverseSearcher::next_match_back
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:780:0 core::str::pattern::impl$22::next_match_back
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:1314:0 core::str::impl$0::rfind
// F31: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:94:0 core::str::validations::next_code_point_reverse
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:25:0 core::str::validations::utf8_is_cont_byte
// F34: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:96:0 core::str::validations::next_code_point_reverse
// F35: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:99:0 core::str::validations::next_code_point_reverse
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:101:0 core::str::validations::next_code_point_reverse
// F37: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:539:0 core::ptr::non_null::NonNull::offset
// F38: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:720:0 core::ptr::non_null::NonNull::sub
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:131:0 core::slice::iter::Iter::pre_dec_end
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:401:0 core::slice::iter::impl$182::next_back
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F42: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:12:0 core::str::validations::utf8_first_byte
// F43: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F44: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:18:0 core::str::validations::utf8_acc_cont_byte
// F45: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:106:0 core::str::validations::next_code_point_reverse
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:108:0 core::str::validations::next_code_point_reverse
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:110:0 core::str::validations::next_code_point_reverse
// F48: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::option::Option::map
// F49: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:285:0 core::slice::iter::impl$181::any
// F50: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:641:0 core::str::pattern::impl$8::matches
// F51: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:698:0 core::str::pattern::impl$11::next_back
// F52: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:1798:0 core::ptr::non_null::impl$12::eq
// F53: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$182::next_back
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F55: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:398:0 core::slice::iter::impl$182::next_back
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:93:0 puffin::impl$6::call::THREAD_PROFILER::__getit
// F57: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:283:0 std::thread::local::LocalKey::try_with
// F58: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:260:0 std::thread::local::LocalKey::with
// F59: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F60: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F61: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F62: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/mod.rs:46:0 std::sys::thread_local::lazy::LazyKeyInner::get
// F63: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:174:0 std::sys::thread_local::fast_local::Key::get
// F64: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:176:0 std::sys::thread_local::fast_local::Key::get
// F65: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1207:0 core::option::Option::ok_or
// F66: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1034:0 core::result::Result::expect
// F67: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F68: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:684:0 core::option::Option::as_ref
// F69: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1787:0 core::cell::BorrowRefMut::new
// F70: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1091:0 core::cell::RefCell::try_borrow_mut
// F71: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1059:0 core::cell::RefCell::borrow_mut
// F72: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F73: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:284:0 std::thread::local::LocalKey::try_with
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F75: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:922:0 core::mem::replace
// F76: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:482:0 core::cell::Cell::replace
// F77: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:411:0 core::cell::Cell::set
// F78: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1789:0 core::cell::BorrowRefMut::new
// F79: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:2126:0 core::cell::UnsafeCell::get
// F80: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1099:0 core::cell::RefCell::try_borrow_mut
// F81: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F82: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1776:0 core::cell::impl$42::drop
// F83: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:515:0 core::ptr::drop_in_place
// F84: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:806:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F85: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:469:0 bevy_ecs::event::EventReader::read
// F86: crates/systems/roof/src/visual/assemble_roof_tiles.rs:22:0 assemble_roof_tiles
// F87: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:547:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F88: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:811:0 bevy_ecs::event::EventIteratorWithId::new
// F89: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:723:0 bevy_ecs::event::ManualEventReader::read_with_id
// F90: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:718:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F91: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::saturating_sub
// F92: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:813:0 bevy_ecs::event::EventIteratorWithId::new
// F93: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:816:0 bevy_ecs::event::EventIteratorWithId::new
// F94: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:354:0 core::slice::index::impl$4::get
// F95: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:480:0 core::slice::index::impl$6::get
// F96: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:817:0 bevy_ecs::event::EventIteratorWithId::new
// F97: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1007:0 core::slice::impl$0::get
// F98: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:818:0 bevy_ecs::event::EventIteratorWithId::new
// F99: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:823:0 bevy_ecs::event::EventIteratorWithId::new
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F101: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F102: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F103: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:825:0 bevy_ecs::event::EventIteratorWithId::new
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:705:0 core::option::Option::as_mut
// F105: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:300:0 core::iter::adapters::chain::and_then_or_clear
// F106: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next
// F107: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:845:0 bevy_ecs::event::impl$23::next
// F108: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:771:0 bevy_ecs::event::impl$20::next
// F109: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next::closure$0
// F110: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1513:0 core::option::Option::or_else
// F111: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F112: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F113: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F114: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:850:0 bevy_ecs::event::impl$23::next
// F115: crates/country-core/src/asset_libraries/ssbo_library.rs:203:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F116: crates/country-core/src/asset_libraries/ssbo_library.rs:239:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
// F117: crates/systems/roof/src/visual/assemble_roof_tiles.rs:23:0 assemble_roof_tiles
// F118: crates/country-core/src/asset_libraries/ssbo_library.rs:204:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F119: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/cmp.rs:84:0 core::slice::cmp::impl$5::equal
// F120: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/cmp.rs:16:0 core::slice::cmp::impl$0::eq
// F121: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1655:0 core::cmp::impls::impl$9::eq
// F122: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:30:0 core::str::traits::impl$1::eq
// F123: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/cmp.rs:92:0 core::slice::cmp::impl$5::equal
// F124: crates/country-core/src/asset_libraries/ssbo_library.rs:205:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F125: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:931:0 core::option::Option::unwrap
// F126: crates/systems/roof/src/visual/assemble_roof_tiles.rs:25:0 assemble_roof_tiles
// F127: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:967:0 bevy_ecs::system::query::Query::get_mut
// F128: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1075:0 core::result::Result::unwrap
// F129: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1076:0 core::result::Result::unwrap
// F130: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:608:0 core::option::Option::is_some
// F131: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:652:0 core::option::Option::is_none
// F132: crates/systems/roof/src/visual/assemble_roof_tiles.rs:27:0 assemble_roof_tiles
// F133: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1665:0 core::ptr::read_volatile
// F134: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:98:0 alloc::alloc::alloc
// F135: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:183:0 alloc::alloc::Global::alloc_impl
// F136: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:243:0 alloc::alloc::impl$1::allocate
// F137: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:332:0 alloc::alloc::exchange_malloc
// F138: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/boxed.rs:219:0 alloc::boxed::impl$0::new
// F139: crates/country-core/src/render/sparse_instance_buffer.rs:121:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
// F140: crates/systems/roof/src/visual/assemble_roof_tiles.rs:28:0 assemble_roof_tiles
// F141: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:100:0 alloc::alloc::alloc
// F142: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:402:0 core::ptr::non_null::NonNull::as_ref
// F143: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:1796:0 alloc::sync::Arc::inner
// F144: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:1619:0 alloc::sync::Arc::downgrade
// F145: crates/country-core/src/render/sparse_instance_buffer.rs:129:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
// F146: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:24:0 core::core_arch::x86::sse2::_mm_pause
// F147: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/hint.rs:226:0 core::hint::spin_loop
// F148: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:1624:0 alloc::sync::Arc::downgrade
// F149: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3307:0 core::sync::atomic::atomic_load
// F150: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3268:0 core::sync::atomic::AtomicUsize::load
// F151: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:0:0 alloc::sync::Arc::downgrade
// F152: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:1623:0 alloc::sync::Arc::downgrade
// F153: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:1630:0 alloc::sync::Arc::downgrade
// F154: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:1639:0 alloc::sync::Arc::downgrade
// F155: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3417:0 core::sync::atomic::atomic_compare_exchange_weak
// F156: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3268:0 core::sync::atomic::AtomicUsize::compare_exchange_weak
// F157: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$49::set_changed
// F158: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$50::deref_mut
// F159: crates/systems/roof/src/visual/assemble_roof_tiles.rs:31:0 assemble_roof_tiles
// F160: crates/country-core/src/render/sparse_instance_buffer.rs:166:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F161: crates/systems/roof/src/visual/assemble_roof_tiles.rs:32:0 assemble_roof_tiles
// F162: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3043:0 core::sync::atomic::AtomicU8::compare_exchange_weak
// F163: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs:69:0 parking_lot::raw_mutex::impl$0::lock
// F164: crates/country-core/src/render/sparse_instance_buffer.rs:167:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F165: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:539:0 core::result::Result::is_ok
// F166: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:582:0 core::result::Result::is_err
// F167: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs:70:0 parking_lot::raw_mutex::impl$0::lock
// F168: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs:72:0 parking_lot::raw_mutex::impl$0::lock
// F169: crates/country-core/src/render/sparse_instance_buffer.rs:170:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F170: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:303:0 core::slice::impl$0::last
// F171: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1869:0 core::option::Option::copied
// F172: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1870:0 core::option::Option::copied
// F173: crates/country-core/src/render/sparse_instance_buffer.rs:175:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F174: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/uint_macros.rs:2075:0 core::num::impl$11::overflowing_sub
// F175: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/uint_macros.rs:605:0 core::num::impl$11::checked_sub
// F176: crates/country-core/src/render/sparse_instance_buffer.rs:175:0 country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
// F177: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:978:0 core::option::Option::unwrap_or_else
// F178: crates/country-core/src/render/sparse_instance_buffer.rs:176:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F179: crates/country-core/src/render/sparse_instance_buffer.rs:177:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F180: crates/country-core/src/render/sparse_instance_buffer.rs:179:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F181: crates/country-core/src/render/sparse_instance_buffer.rs:181:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F182: crates/country-core/src/render/sparse_instance_buffer.rs:169:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F183: crates/systems/roof/src/visual/assemble_roof_tiles.rs:33:0 assemble_roof_tiles
// F184: crates/systems/roof/src/visual/assemble_roof_tiles.rs:35:0 assemble_roof_tiles
// F185: crates/systems/roof/src/visual/assemble_roof_tiles.rs:63:0 assemble_roof_tiles
// F186: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3385:0 core::sync::atomic::atomic_compare_exchange
// F187: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3043:0 core::sync::atomic::AtomicU8::compare_exchange
// F188: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs:104:0 parking_lot::raw_mutex::impl$0::unlock
// F189: crates/country-core/src/render/sparse_instance_buffer.rs:436:0 country_core::render::sparse_instance_buffer::impl$5::drop
// F190: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs:105:0 parking_lot::raw_mutex::impl$0::unlock
// F191: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs:109:0 parking_lot::raw_mutex::impl$0::unlock
// F192: /rust/deps/hashbrown-0.14.3/src/map.rs:1355:0 hashbrown::map::HashMap::get_inner
// F193: /rust/deps/hashbrown-0.14.3/src/map.rs:1313:0 hashbrown::map::HashMap::get
// F194: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/collections/hash/map.rs:882:0 std::collections::hash::map::HashMap::get
// F195: crates/systems/roof/src/visual/assemble_roof_tiles.rs:46:0 assemble_roof_tiles
// F196: /rust/deps/hashbrown-0.14.3/src/map.rs:262:0 hashbrown::map::make_hash
// F197: /rust/deps/hashbrown-0.14.3/src/map.rs:1358:0 hashbrown::map::HashMap::get_inner
// F198: /rust/deps/hashbrown-0.14.3/src/map.rs:1359:0 hashbrown::map::HashMap::get_inner
// F199: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:155:0 hashbrown::raw::h2
// F200: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:2169:0 hashbrown::raw::RawTableInner::find_inner
// F201: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:1464:0 hashbrown::raw::RawTable::find
// F202: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:1481:0 hashbrown::raw::RawTable::get
// F203: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2987:0 core::intrinsics::copy_nonoverlapping
// F204: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1240:0 core::core_arch::x86::sse2::_mm_loadu_si128
// F205: /rust/deps/hashbrown-0.14.3/src/raw/sse2.rs:53:0 hashbrown::raw::sse2::Group::load
// F206: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:2187:0 hashbrown::raw::RawTableInner::find_inner
// F207: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1445:0 core::core_arch::x86::sse2::_mm_movemask_epi8
// F208: /rust/deps/hashbrown-0.14.3/src/raw/sse2.rs:90:0 hashbrown::raw::sse2::Group::match_byte
// F209: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:2189:0 hashbrown::raw::RawTableInner::find_inner
// F210: /rust/deps/hashbrown-0.14.3/src/raw/bitmask.rs:50:0 hashbrown::raw::bitmask::BitMask::lowest_set_bit
// F211: /rust/deps/hashbrown-0.14.3/src/raw/bitmask.rs:129:0 hashbrown::raw::bitmask::impl$3::next
// F212: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/nonzero.rs:1565:0 core::num::nonzero::NonZero::trailing_zeros
// F213: /rust/deps/hashbrown-0.14.3/src/raw/bitmask.rs:80:0 hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
// F214: /rust/deps/hashbrown-0.14.3/src/raw/bitmask.rs:51:0 hashbrown::raw::bitmask::BitMask::lowest_set_bit
// F215: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:2192:0 hashbrown::raw::RawTableInner::find_inner
// F216: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:488:0 core::ptr::mut_ptr::impl$0::offset
// F217: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1141:0 core::ptr::mut_ptr::impl$0::sub
// F218: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:519:0 hashbrown::raw::Bucket::as_ptr
// F219: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:683:0 hashbrown::raw::Bucket::as_ref
// F220: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:1466:0 hashbrown::raw::impl$8::find::closure$0
// F221: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:2194:0 hashbrown::raw::RawTableInner::find_inner
// F222: /rust/deps/hashbrown-0.14.3/src/lib.rs:171:0 hashbrown::impl$0::equivalent
// F223: /rust/deps/hashbrown-0.14.3/src/map.rs:229:0 hashbrown::map::equivalent_key::closure$0
// F224: /rust/deps/hashbrown-0.14.3/src/raw/bitmask.rs:38:0 hashbrown::raw::bitmask::BitMask::remove_lowest_bit
// F225: /rust/deps/hashbrown-0.14.3/src/raw/bitmask.rs:130:0 hashbrown::raw::bitmask::impl$3::next
// F226: crates/country-core/src/resources/walls/mod.rs:66:0 country_core::resources::walls::impl$9::eq
// F227: /rust/deps/hashbrown-0.14.3/src/raw/sse2.rs:98:0 hashbrown::raw::sse2::Group::match_empty
// F228: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:2199:0 hashbrown::raw::RawTableInner::find_inner
// F229: /rust/deps/hashbrown-0.14.3/src/raw/bitmask.rs:44:0 hashbrown::raw::sse2::Group::match_empty
// F230: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:183:0 hashbrown::raw::ProbeSeq::move_next
// F231: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:2203:0 hashbrown::raw::RawTableInner::find_inner
// F232: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:182:0 hashbrown::raw::ProbeSeq::move_next
// F233: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:382:0 hashbrown::raw::Bucket::from_base_index
// F234: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:1024:0 hashbrown::raw::RawTable::bucket
// F235: /rust/deps/hashbrown-0.14.3/src/raw/mod.rs:1471:0 hashbrown::raw::RawTable::find
// F236: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2219:0 alloc::vec::Vec::is_empty
// F237: crates/systems/roof/src/visual/assemble_roof_tiles.rs:46:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// F238: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1132:0 core::option::Option::map_or
// F239: crates/systems/roof/src/visual/assemble_roof_tiles.rs:49:0 assemble_roof_tiles
// F240: crates/systems/roof/src/visual/assemble_roof_tiles.rs:53:0 assemble_roof_tiles
// F241: crates/systems/roof/src/visual/assemble_roof_tiles.rs:54:0 assemble_roof_tiles
// F242: crates/systems/roof/src/visual/assemble_roof_tiles.rs:50:0 assemble_roof_tiles
// F243: crates/systems/roof/src/visual/assemble_roof_tiles.rs:51:0 assemble_roof_tiles
// F244: crates/systems/roof/src/visual/assemble_roof_tiles.rs:64:0 assemble_roof_tiles
// F245: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:933:0 core::option::Option::unwrap
// F246: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1077:0 core::result::Result::unwrap
// F247: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:334:0 alloc::alloc::exchange_malloc
// F248: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1061:0 core::cell::RefCell::borrow_mut
// F249: crates/systems/roof/src/visual/assemble_roof_tiles.rs:39:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// F250: crates/systems/roof/src/visual/assemble_roof_tiles.rs:40:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// F251: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3308:0 core::sync::atomic::atomic_load
// F252: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:1433:0 core::sync::atomic::AtomicPtr::load
// F253: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/sync/once/queue.rs:122:0 std::sys::sync::once::queue::Once::is_completed
// F254: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sync/once.rs:144:0 std::sync::once::Once::call_once
// F255: crates/systems/roof/src/visual/assemble_roof_tiles.rs:41:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// F256: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1032:0 core::result::Result::expect
// F257: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1033:0 core::result::Result::expect
// F258: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:221:0 alloc::raw_vec::RawVec::try_allocate_in
// F259: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:155:0 alloc::raw_vec::RawVec::with_capacity_in
// F260: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:688:0 alloc::vec::Vec::with_capacity_in
// F261: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:162:0 alloc::slice::hack::impl$1::to_vec
// F262: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:111:0 alloc::slice::hack::to_vec
// F263: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:441:0 alloc::slice::impl$0::to_vec_in
// F264: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:416:0 alloc::slice::impl$0::to_vec
// F265: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:823:0 alloc::slice::impl$9::to_owned
// F266: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/str.rs:211:0 alloc::str::impl$4::to_owned
// F267: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/string.rs:2706:0 alloc::string::impl$44::from
// F268: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:225:0 alloc::raw_vec::RawVec::try_allocate_in
// F269: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:1336:0 core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
// F270: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:167:0 alloc::slice::hack::impl$1::to_vec
// F271: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/string.rs:994:0 alloc::slice::impl$9::to_owned
// F272: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:333:0 alloc::raw_vec::RawVec::reserve
// F273: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:945:0 alloc::vec::Vec::reserve
// F274: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2092:0 alloc::vec::Vec::append_elements
// F275: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/spec_extend.rs:55:0 alloc::vec::spec_extend::impl$4::spec_extend
// F276: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2538:0 alloc::vec::Vec::extend_from_slice
// F277: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/string.rs:1067:0 alloc::string::String::push_str
// F278: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/string.rs:2431:0 alloc::string::impl$24::add
// F279: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F280: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F281: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2094:0 alloc::vec::Vec::append_elements
// F282: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2203:0 alloc::vec::Vec::len
// F283: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2093:0 alloc::vec::Vec::append_elements
// F284: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2987:0 alloc::vec::Vec::as_mut_ptr
// F285: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2095:0 alloc::vec::Vec::append_elements
// F286: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/string.rs:2432:0 alloc::string::impl$24::add
// F287: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:271:0 alloc::raw_vec::RawVec::capacity
// F288: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:415:0 alloc::raw_vec::RawVec::needs_to_grow
// F289: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:332:0 alloc::raw_vec::RawVec::reserve
// F290: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 alloc::raw_vec::RawVec::capacity
// F291: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sync/mutex.rs:541:0 std::sync::mutex::impl$13::deref_mut
// F292: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/collections/btree/set.rs:906:0 alloc::collections::btree::set::BTreeSet::insert
// F293: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs:1276:0 log::max_level
// F294: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1189:0 core::cmp::PartialOrd::le
// F295: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:343:0 core::fmt::Arguments::new_v1
// F296: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs:73:0 log::__private_api::log
// F297: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sync/poison.rs:53:0 std::sync::poison::Flag::done
// F298: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sync/mutex.rs:550:0 std::sync::mutex::impl$14::drop
// F299: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:436:0 std::panicking::panic_count::count_is_zero
// F300: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:587:0 std::panicking::panicking
// F301: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/mod.rs:797:0 std::thread::panicking
// F302: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3325:0 core::sync::atomic::atomic_swap
// F303: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:2470:0 core::sync::atomic::AtomicU8::swap
// F304: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/sync/mutex/futex.rs:95:0 std::sys::sync::mutex::futex::Mutex::unlock
// F305: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sync/mutex.rs:551:0 std::sync::mutex::impl$14::drop
// F306: crates/systems/roof/src/visual/assemble_roof_tiles.rs:43:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// F307: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sync/once.rs:148:0 std::sync::once::Once::call_once
// F308: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sync/once.rs:149:0 std::sync::once::Once::call_once
// F309: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/sync/mutex/futex.rs:100:0 std::sys::sync::mutex::futex::Mutex::unlock
// F310: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:448:0 std::panicking::panic_count::count_is_zero
// F311: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3292:0 core::sync::atomic::atomic_store
// F312: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:641:0 core::sync::atomic::AtomicBool::store
// F313: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sync/poison.rs:54:0 std::sync::poison::Flag::done
// F314: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:567:0 alloc::raw_vec::handle_reserve
// F315: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/string.rs:2433:0 alloc::string::impl$24::add
// F316: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:280:0 alloc::raw_vec::RawVec::current_memory
// F317: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:554:0 alloc::raw_vec::impl$4::drop
// F318: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:119:0 alloc::alloc::dealloc
// F319: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:256:0 alloc::alloc::impl$1::deallocate
// F320: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:555:0 alloc::raw_vec::impl$4::drop
// F321: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1028:0 core::result::Result::expect
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/visual/assemble_roof_tiles.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 07c17f77ad5c445fc6bac8569bb4566d7aee8525a4370f48898c316ee95c0daf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: assemble_roof_tiles.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 266, 753, 754
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":266,"record_offset":3304,"rva":4128624,"va":5372837744}
// 编译实例 RVAs: 0x3eff70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3225886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8e2c6ceff","size":5,"soft_fail":false,"target_va":5369613520,"va":5372839401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-89284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e83ca3feff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372838975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cd060
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5098380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e88ccb4d00","size":5,"soft_fail":false,"target_va":5377937504,"va":5372839119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408d5270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5132019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8f34e4e00","size":5,"soft_fail":false,"target_va":5377970800,"va":5372838776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408db380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5156331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8ebad4e00","size":5,"soft_fail":false,"target_va":5377995648,"va":5372839312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408dbdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5158638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8eeb64e00","size":5,"soft_fail":false,"target_va":5377998272,"va":5372839629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e0400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5176578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e802fd4e00","size":5,"soft_fail":false,"target_va":5378016256,"va":5372839673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30754493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8bd46d501","size":5,"soft_fail":false,"target_va":5403593344,"va":5372838846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30138214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e866dfcb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372839925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::downgrade::panic_cold_display::h39b8543fa60cfa49 (.llvm.12996154446813914949)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30150860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8cc10cc01","size":5,"soft_fail":false,"target_va":5402990624,"va":5372839759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5147223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8578a4e00","size":5,"soft_fail":false,"target_va":5377986144,"va":5372838916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30138730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e86ae1cb01","size":5,"soft_fail":false,"target_va":5402978688,"va":5372839953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30139099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8dbe2cb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5372839776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30150477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e84d0fcc01","size":5,"soft_fail":false,"target_va":5402990304,"va":5372839822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8acf2ffff","size":5,"soft_fail":false,"target_va":5372836320,"va":5372839727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30141878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8b6edcb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372838245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8ddfaffff","size":5,"soft_fail":false,"target_va":5372836512,"va":5372837822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30142752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e820f1cb01","size":5,"soft_fail":false,"target_va":5402980656,"va":5372837899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2924227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8c39e2c00","size":5,"soft_fail":false,"target_va":5375763552,"va":5372839320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30129786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e87abecb01","size":5,"soft_fail":false,"target_va":5402968992,"va":5372839201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t30131441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e8f1c4cb01","size":5,"soft_fail":false,"target_va":5402970800,"va":5372839354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3518496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e820b03500","size":5,"soft_fail":false,"target_va":5376356912,"va":5372838411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3522379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e84bbf3500","size":5,"soft_fail":false,"target_va":5376360560,"va":5372838176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3518337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e881af3500","size":5,"soft_fail":false,"target_va":5376356144,"va":5372837802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h8ca862e57c4aa143 (.llvm.3516157794931125432)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5113191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372837744,"raw_hex":"e867054e00","size":5,"soft_fail":false,"target_va":5377951392,"va":5372838196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372837744,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5372838866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":266,"record_offset":7920,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":266,"record_offset":6152,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":266,"record_offset":6700,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":266,"record_offset":7288,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8060,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":7356,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":9828,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":7252,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":266,"record_offset":7976,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":266,"record_offset":7316,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":266,"record_offset":7012,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":266,"record_offset":7412,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":14712,"source_line":117,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":266,"record_offset":8124,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":266,"record_offset":5104,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":266,"record_offset":3516,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":266,"record_offset":4092,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":266,"record_offset":8772,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":266,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":3304,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":266,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":3304,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":266,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":3304,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":266,"record_offset":7512,"parent_record_offset":7480,"ancestor_record_offset":3304,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":266,"record_offset":7452,"parent_record_offset":7412,"ancestor_record_offset":3304,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":266,"record_offset":7636,"parent_record_offset":7412,"ancestor_record_offset":3304,"inlinee":11196,"source_line":1613,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":266,"record_offset":7688,"parent_record_offset":7636,"ancestor_record_offset":3304,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":266,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":3304,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":266,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":3304,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":266,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":3304,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":266,"record_offset":6216,"parent_record_offset":6176,"ancestor_record_offset":3304,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":266,"record_offset":6724,"parent_record_offset":6700,"ancestor_record_offset":3304,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":266,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":3304,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":266,"record_offset":5504,"parent_record_offset":5476,"ancestor_record_offset":3304,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":266,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":3304,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":266,"record_offset":5532,"parent_record_offset":5504,"ancestor_record_offset":3304,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":266,"record_offset":5444,"parent_record_offset":5416,"ancestor_record_offset":3304,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":266,"record_offset":5476,"parent_record_offset":5444,"ancestor_record_offset":3304,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":266,"record_offset":5648,"parent_record_offset":5476,"ancestor_record_offset":3304,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":266,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":3304,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":266,"record_offset":7080,"parent_record_offset":7040,"ancestor_record_offset":3304,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":266,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":3304,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":266,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":3304,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":266,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":3304,"inlinee":8944,"source_line":21,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hint.rs
//     evidence={"module":266,"record_offset":7744,"parent_record_offset":7636,"ancestor_record_offset":3304,"inlinee":8943,"source_line":216,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":266,"record_offset":9056,"parent_record_offset":9032,"ancestor_record_offset":3304,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":266,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":3304,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":266,"record_offset":6752,"parent_record_offset":6724,"ancestor_record_offset":3304,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":266,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":3304,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":266,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":3304,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":266,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":3304,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":266,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":3304,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":266,"record_offset":6288,"parent_record_offset":6240,"ancestor_record_offset":3304,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":266,"record_offset":3728,"parent_record_offset":3632,"ancestor_record_offset":3304,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":266,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":3304,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":3304,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":3304,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8400,"parent_record_offset":8124,"ancestor_record_offset":3304,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":3304,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":4688,"parent_record_offset":4228,"ancestor_record_offset":3304,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":5388,"parent_record_offset":5212,"ancestor_record_offset":3304,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":6832,"parent_record_offset":6752,"ancestor_record_offset":3304,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8188,"parent_record_offset":8124,"ancestor_record_offset":3304,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8432,"parent_record_offset":8124,"ancestor_record_offset":3304,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":266,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":3304,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":266,"record_offset":5712,"parent_record_offset":5416,"ancestor_record_offset":3304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":266,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":3304,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":266,"record_offset":9400,"parent_record_offset":9376,"ancestor_record_offset":3304,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":266,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":3304,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":266,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":3304,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":266,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":3304,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":266,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":3304,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":266,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":3304,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":266,"record_offset":4648,"parent_record_offset":4616,"ancestor_record_offset":3304,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":266,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":3304,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":266,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":3304,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":266,"record_offset":6112,"parent_record_offset":5176,"ancestor_record_offset":3304,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":266,"record_offset":8312,"parent_record_offset":8224,"ancestor_record_offset":3304,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":266,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":3304,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":266,"record_offset":7164,"parent_record_offset":7136,"ancestor_record_offset":3304,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":266,"record_offset":7192,"parent_record_offset":7164,"ancestor_record_offset":3304,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":266,"record_offset":3664,"parent_record_offset":3632,"ancestor_record_offset":3304,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":266,"record_offset":6544,"parent_record_offset":6240,"ancestor_record_offset":3304,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":266,"record_offset":8372,"parent_record_offset":8124,"ancestor_record_offset":3304,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":266,"record_offset":3692,"parent_record_offset":3664,"ancestor_record_offset":3304,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":266,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":3304,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":266,"record_offset":6392,"parent_record_offset":6352,"ancestor_record_offset":3304,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":266,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":3304,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":266,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":3304,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":266,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":3304,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":266,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":3304,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":266,"record_offset":6908,"parent_record_offset":6856,"ancestor_record_offset":3304,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":266,"record_offset":4392,"parent_record_offset":4284,"ancestor_record_offset":3304,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":266,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":3304,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":266,"record_offset":4124,"parent_record_offset":4092,"ancestor_record_offset":3304,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":266,"record_offset":4256,"parent_record_offset":4228,"ancestor_record_offset":3304,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":266,"record_offset":4228,"parent_record_offset":4196,"ancestor_record_offset":3304,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":266,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3304,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":266,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":3304,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":266,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":3304,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":266,"record_offset":4720,"parent_record_offset":4196,"ancestor_record_offset":3304,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":266,"record_offset":3828,"parent_record_offset":3796,"ancestor_record_offset":3304,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":266,"record_offset":3988,"parent_record_offset":3828,"ancestor_record_offset":3304,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":266,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3304,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":266,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":3304,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":266,"record_offset":3548,"parent_record_offset":3516,"ancestor_record_offset":3304,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":266,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":3304,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":266,"record_offset":3604,"parent_record_offset":3572,"ancestor_record_offset":3304,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":266,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":3304,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":266,"record_offset":4284,"parent_record_offset":4256,"ancestor_record_offset":3304,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":266,"record_offset":4532,"parent_record_offset":4284,"ancestor_record_offset":3304,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":266,"record_offset":4504,"parent_record_offset":4284,"ancestor_record_offset":3304,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":266,"record_offset":4336,"parent_record_offset":4284,"ancestor_record_offset":3304,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":8676,"parent_record_offset":8640,"ancestor_record_offset":3304,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":8256,"parent_record_offset":8224,"ancestor_record_offset":3304,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":7856,"parent_record_offset":7636,"ancestor_record_offset":3304,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":7800,"parent_record_offset":7636,"ancestor_record_offset":3304,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":3304,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":3304,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":7824,"parent_record_offset":7800,"ancestor_record_offset":3304,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":266,"record_offset":7040,"parent_record_offset":7012,"ancestor_record_offset":3304,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":266,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":3304,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":266,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":3304,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":266,"record_offset":9524,"parent_record_offset":9496,"ancestor_record_offset":3304,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":266,"record_offset":9468,"parent_record_offset":9440,"ancestor_record_offset":3304,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":266,"record_offset":8796,"parent_record_offset":8772,"ancestor_record_offset":3304,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":266,"record_offset":8820,"parent_record_offset":8796,"ancestor_record_offset":3304,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":266,"record_offset":9440,"parent_record_offset":9300,"ancestor_record_offset":3304,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":266,"record_offset":8848,"parent_record_offset":8820,"ancestor_record_offset":3304,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":3304,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":9328,"parent_record_offset":9300,"ancestor_record_offset":3304,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":3304,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":9660,"parent_record_offset":8928,"ancestor_record_offset":3304,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":9696,"parent_record_offset":8900,"ancestor_record_offset":3304,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":3304,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":8876,"parent_record_offset":8820,"ancestor_record_offset":3304,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":8928,"parent_record_offset":8900,"ancestor_record_offset":3304,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":266,"record_offset":9180,"parent_record_offset":9148,"ancestor_record_offset":3304,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":266,"record_offset":9208,"parent_record_offset":9180,"ancestor_record_offset":3304,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":266,"record_offset":9268,"parent_record_offset":9148,"ancestor_record_offset":3304,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":266,"record_offset":9148,"parent_record_offset":8928,"ancestor_record_offset":3304,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":8980,"parent_record_offset":8928,"ancestor_record_offset":3304,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":266,"record_offset":9300,"parent_record_offset":8928,"ancestor_record_offset":3304,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":266,"record_offset":9008,"parent_record_offset":8928,"ancestor_record_offset":3304,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":266,"record_offset":9092,"parent_record_offset":8928,"ancestor_record_offset":3304,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":266,"record_offset":9568,"parent_record_offset":8928,"ancestor_record_offset":3304,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":266,"record_offset":8224,"parent_record_offset":8124,"ancestor_record_offset":3304,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":266,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":3304,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":266,"record_offset":5140,"parent_record_offset":5104,"ancestor_record_offset":3304,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":266,"record_offset":5252,"parent_record_offset":5212,"ancestor_record_offset":3304,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":266,"record_offset":5416,"parent_record_offset":5212,"ancestor_record_offset":3304,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":266,"record_offset":5684,"parent_record_offset":5416,"ancestor_record_offset":3304,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":266,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":3304,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":266,"record_offset":5312,"parent_record_offset":5280,"ancestor_record_offset":3304,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":266,"record_offset":5212,"parent_record_offset":5176,"ancestor_record_offset":3304,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":266,"record_offset":5176,"parent_record_offset":5140,"ancestor_record_offset":3304,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/assemble_roof_tiles.rs
//     evidence={"module":266,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":3304,"inlinee":14719,"source_line":46,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// 记录形式: compiled_procedure; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":753,"record_offset":32852,"rva":9306096,"va":5378015216}
// 编译实例 RVAs: 0x8dfff0, 0x8e1820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=2
//     target_aliases=25 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8342143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e881b580ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5378015882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c6b80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6395063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8496b9eff","size":5,"soft_fail":false,"target_va":5371620224,"va":5378015282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=2
//     target_aliases=79 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5766667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8f501a8ff","size":5,"soft_fail":false,"target_va":5372249392,"va":5378016054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5265639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e819a7afff","size":5,"soft_fail":false,"target_va":5372749696,"va":5378015330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5266234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8c6a4afff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378015941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408db0b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8a2b0ffff","size":5,"soft_fail":false,"target_va":5377994928,"va":5378015241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24962297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8f9e47c01","size":5,"soft_fail":false,"target_va":5402978144,"va":5378015842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10372379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e81b459e00","size":5,"soft_fail":false,"target_va":5388387936,"va":5378015552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t24903661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8edff7b01","size":5,"soft_fail":false,"target_va":5402919072,"va":5378015406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24964298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8caec7c01","size":5,"soft_fail":false,"target_va":5402980128,"va":5378015825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-661871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e891e6f5ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5378015642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24959566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e84eda7c01","size":5,"soft_fail":false,"target_va":5402975328,"va":5378015757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24959944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8c8db7c01","size":5,"soft_fail":false,"target_va":5402975696,"va":5378015747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::once::queue::Once::call
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24959970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378015216,"raw_hex":"e8e2db7c01","size":5,"soft_fail":false,"target_va":5402975712,"va":5378015737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":753,"record_offset":34920,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":8697,"source_line":902,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$24::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":753,"record_offset":33752,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":24636,"source_line":2430,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":753,"record_offset":33212,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":753,"record_offset":35032,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":753,"record_offset":35060,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":753,"record_offset":35116,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":753,"record_offset":33168,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":753,"record_offset":35088,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":753,"record_offset":34948,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":753,"record_offset":34864,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_direct] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\once.rs
//     evidence={"module":753,"record_offset":33048,"parent_record_offset":32852,"ancestor_record_offset":32852,"inlinee":17118,"source_line":139,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":753,"record_offset":33540,"parent_record_offset":33516,"ancestor_record_offset":32852,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":753,"record_offset":33564,"parent_record_offset":33540,"ancestor_record_offset":32852,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":753,"record_offset":34224,"parent_record_offset":34200,"ancestor_record_offset":32852,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":753,"record_offset":33516,"parent_record_offset":33488,"ancestor_record_offset":32852,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":753,"record_offset":34200,"parent_record_offset":34144,"ancestor_record_offset":32852,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":34496,"parent_record_offset":34472,"ancestor_record_offset":32852,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":34172,"parent_record_offset":34144,"ancestor_record_offset":32852,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":34472,"parent_record_offset":34444,"ancestor_record_offset":32852,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":33988,"parent_record_offset":33952,"ancestor_record_offset":32852,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":33920,"parent_record_offset":33896,"ancestor_record_offset":32852,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":33488,"parent_record_offset":33460,"ancestor_record_offset":32852,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":33460,"parent_record_offset":33432,"ancestor_record_offset":32852,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":33632,"parent_record_offset":33460,"ancestor_record_offset":32852,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":753,"record_offset":34144,"parent_record_offset":34116,"ancestor_record_offset":32852,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":753,"record_offset":33396,"parent_record_offset":33368,"ancestor_record_offset":32852,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":753,"record_offset":33368,"parent_record_offset":33340,"ancestor_record_offset":32852,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":753,"record_offset":33312,"parent_record_offset":33268,"ancestor_record_offset":32852,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":753,"record_offset":33340,"parent_record_offset":33312,"ancestor_record_offset":32852,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":753,"record_offset":33268,"parent_record_offset":33240,"ancestor_record_offset":32852,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":753,"record_offset":33240,"parent_record_offset":33212,"ancestor_record_offset":32852,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":753,"record_offset":33788,"parent_record_offset":33752,"ancestor_record_offset":32852,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":753,"record_offset":33860,"parent_record_offset":33836,"ancestor_record_offset":32852,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":753,"record_offset":33952,"parent_record_offset":33860,"ancestor_record_offset":32852,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":753,"record_offset":33812,"parent_record_offset":33788,"ancestor_record_offset":32852,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":753,"record_offset":34020,"parent_record_offset":33860,"ancestor_record_offset":32852,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":753,"record_offset":33896,"parent_record_offset":33860,"ancestor_record_offset":32852,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":753,"record_offset":33432,"parent_record_offset":33396,"ancestor_record_offset":32852,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":753,"record_offset":33836,"parent_record_offset":33812,"ancestor_record_offset":32852,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":753,"record_offset":34888,"parent_record_offset":34864,"ancestor_record_offset":32852,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":753,"record_offset":33692,"parent_record_offset":33668,"ancestor_record_offset":32852,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":753,"record_offset":33668,"parent_record_offset":33396,"ancestor_record_offset":32852,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":753,"record_offset":34064,"parent_record_offset":33752,"ancestor_record_offset":32852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":753,"record_offset":33588,"parent_record_offset":33564,"ancestor_record_offset":32852,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":753,"record_offset":35368,"parent_record_offset":35180,"ancestor_record_offset":32852,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":753,"record_offset":33104,"parent_record_offset":33080,"ancestor_record_offset":32852,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":753,"record_offset":35460,"parent_record_offset":35428,"ancestor_record_offset":32852,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":753,"record_offset":34972,"parent_record_offset":34948,"ancestor_record_offset":32852,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":753,"record_offset":33128,"parent_record_offset":33104,"ancestor_record_offset":32852,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":753,"record_offset":35392,"parent_record_offset":35368,"ancestor_record_offset":32852,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":753,"record_offset":35484,"parent_record_offset":35460,"ancestor_record_offset":32852,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":753,"record_offset":35268,"parent_record_offset":35240,"ancestor_record_offset":32852,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":753,"record_offset":35240,"parent_record_offset":35212,"ancestor_record_offset":32852,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":753,"record_offset":35144,"parent_record_offset":35116,"ancestor_record_offset":32852,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":753,"record_offset":35180,"parent_record_offset":35144,"ancestor_record_offset":32852,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":753,"record_offset":35428,"parent_record_offset":35144,"ancestor_record_offset":32852,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::queue::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\once\queue.rs
//     evidence={"module":753,"record_offset":33080,"parent_record_offset":33048,"ancestor_record_offset":32852,"inlinee":6263,"source_line":117,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":753,"record_offset":35212,"parent_record_offset":35180,"ancestor_record_offset":32852,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00753.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x397F
// 首条位置: {"module":266,"inlinee":14719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":266,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":9852,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00266.jsonl.gz"}
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
// 完整结构及原始类型记录: assemble_roof_tiles.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: assemble_roof_tiles.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: assemble_roof_tiles.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// 信息 ID: b5f8832bd75ba52c2a047b2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fce0cd045cae57e7be2d3064: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8e7058 -> 0x3eff70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x12d1401 -> 0x3eff70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// 信息 ID: 4e7e549d2adb2e0f7bf9ea10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::visual::assemble_roof_tiles::assemble_roof_tiles", "captures": "unknown"}
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$19::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$19::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 5 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dcbea -> 0x8dfff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8deb00 -> 0x8dfff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8ded83 -> 0x8dfff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dd387 -> 0x8dfff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dd53e -> 0x8dfff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e0e8f -> 0x8e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e1105 -> 0x8e1820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// 信息 ID: c2380095b674e671d8d79d6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_roof::visual::assemble_roof_tiles::assemble_roof_tiles", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=266, inline_record=9852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
