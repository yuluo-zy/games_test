// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: replay_roof.rs.assembly.json.gz

// FUNCTION system_roof::replay_roof::replay_roof
// Candidate interfaces: add80410eca410d20a536794
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 2277:260 -> rva_1a37ad0_len_4fd
//
// FUNCTION system_roof::replay_roof::impl$0::process
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3476:260 -> rva_21b48e0_len_4ff
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1a37ad0_len_4fd: RVA 0x1a37ad0, 1277 PDB bytes; 338 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_1a37ad0_len_4fd.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2277:260: system_roof::replay_roof::replay_roof
// Frames: F1
// 0x141a37ad0  55                             pushq	%rbp
// 0x141a37ad1  4157                           pushq	%r15
// 0x141a37ad3  4156                           pushq	%r14
// 0x141a37ad5  4155                           pushq	%r13
// 0x141a37ad7  4154                           pushq	%r12
// 0x141a37ad9  56                             pushq	%rsi
// 0x141a37ada  57                             pushq	%rdi
// 0x141a37adb  53                             pushq	%rbx
// 0x141a37adc  4881ecc8000000                 subq	$0xc8, %rsp
// 0x141a37ae3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x141a37aeb  0f297530                       movaps	%xmm6, 0x30(%rbp)
// 0x141a37aef  48c74528feffffff               movq	$-0x2, 0x28(%rbp)
// 0x141a37af7  4c894d20                       movq	%r9, 0x20(%rbp)
// 0x141a37afb  4c89c3                         movq	%r8, %rbx
// 0x141a37afe  4889d7                         movq	%rdx, %rdi
// 0x141a37b01  4889ce                         movq	%rcx, %rsi
// Frames: F2
// 0x141a37b04  4c8b39                         movq	(%rcx), %r15
// 0x141a37b07  4c89f9                         movq	%r15, %rcx
// 0x141a37b0a  e86116f1fe                     callq	0x140949170 <.text+0x948170>
// Frames: F3
// 0x141a37b0f  f3410f107728                   movss	0x28(%r15), %xmm6
// Frames: F4 -> F5 -> F6
// 0x141a37b15  8b461c                         movl	0x1c(%rsi), %eax
// 0x141a37b18  488b4e10                       movq	0x10(%rsi), %rcx
// 0x141a37b1c  8901                           movl	%eax, (%rcx)
// Frames: F6
// 0x141a37b1e  4c89f9                         movq	%r15, %rcx
// 0x141a37b21  e85a9ffbfe                     callq	0x1409f1a80 <.text+0x9f0a80>
// 0x141a37b26  4885c0                         testq	%rax, %rax
// 0x141a37b29  0f84b5030000                   je	0x141a37ee4 <.text+0x1a36ee4>
// Frames: F7 -> F8
// 0x141a37b2f  4889c6                         movq	%rax, %rsi
// 0x141a37b32  48c7450800000000               movq	$0x0, 0x8(%rbp)
// 0x141a37b3a  48c7451008000000               movq	$0x8, 0x10(%rbp)
// 0x141a37b42  48c7451800000000               movq	$0x0, 0x18(%rbp)
// Frames: F9
// 0x141a37b4a  4c8b7008                       movq	0x8(%rax), %r14
// 0x141a37b4e  488b4010                       movq	0x10(%rax), %rax
// Frames: F10 -> F11 -> F12 -> F9
// 0x141a37b52  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141a37b56  48c1e004                       shlq	$0x4, %rax
// 0x141a37b5a  4c01f0                         addq	%r14, %rax
// 0x141a37b5d  488945f0                       movq	%rax, -0x10(%rbp)
// 0x141a37b61  4c8b27                         movq	(%rdi), %r12
// 0x141a37b64  4531ff                         xorl	%r15d, %r15d
// 0x141a37b67  eb16                           jmp	0x141a37b7f <.text+0x1a36b7f>
// 0x141a37b69  0f1f8000000000                 nopl	(%rax)
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18
// 0x141a37b70  488b4510                       movq	0x10(%rbp), %rax
// Frames: F19 -> F20 -> F18
// 0x141a37b74  4c892cf8                       movq	%r13, (%rax,%rdi,8)
// Frames: F21 -> F18
// 0x141a37b78  48ffc7                         incq	%rdi
// 0x141a37b7b  48897d18                       movq	%rdi, 0x18(%rbp)
// Frames: F9
// 0x141a37b7f  4c89f0                         movq	%r14, %rax
// Frames: F22 -> F23 -> F24 -> F9
// 0x141a37b82  4c3b75f0                       cmpq	-0x10(%rbp), %r14
// Frames: F25 -> F24 -> F9
// 0x141a37b86  747b                           je	0x141a37c03 <.text+0x1a36c03>
// Frames: F9
// 0x141a37b88  4d89fd                         movq	%r15, %r13
// Frames: F26 -> F27 -> F28 -> F29 -> F24 -> F9
// 0x141a37b8b  4c8d7030                       leaq	0x30(%rax), %r14
// Frames: F30 -> F9
// 0x141a37b8f  49ffc7                         incq	%r15
// Frames: F31
// 0x141a37b92  803805                         cmpb	$0x5, (%rax)
// 0x141a37b95  75e8                           jne	0x141a37b7f <.text+0x1a36b7f>
// 0x141a37b97  48894500                       movq	%rax, (%rbp)
// 0x141a37b9b  488d4808                       leaq	0x8(%rax), %rcx
// 0x141a37b9f  f3410f104c2428                 movss	0x28(%r12), %xmm1
// Frames: F32
// 0x141a37ba6  f30f59ce                       mulss	%xmm6, %xmm1
// Frames: F33
// 0x141a37baa  488b85c0000000                 movq	0xc0(%rbp), %rax
// 0x141a37bb1  4889442430                     movq	%rax, 0x30(%rsp)
// 0x141a37bb6  488b85b8000000                 movq	0xb8(%rbp), %rax
// 0x141a37bbd  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141a37bc2  488b85b0000000                 movq	0xb0(%rbp), %rax
// 0x141a37bc9  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a37bce  4989d8                         movq	%rbx, %r8
// 0x141a37bd1  4c8b4d20                       movq	0x20(%rbp), %r9
// 0x141a37bd5  e806cd7700                     callq	0x1421b48e0 <.text+0x21b38e0>
// Frames: F34
// 0x141a37bda  488b4500                       movq	(%rbp), %rax
// 0x141a37bde  80781d01                       cmpb	$0x1, 0x1d(%rax)
// 0x141a37be2  759b                           jne	0x141a37b7f <.text+0x1a36b7f>
// Frames: F35 -> F18
// 0x141a37be4  488b7d18                       movq	0x18(%rbp), %rdi
// Frames: F36 -> F18
// 0x141a37be8  483b7d08                       cmpq	0x8(%rbp), %rdi
// 0x141a37bec  7582                           jne	0x141a37b70 <.text+0x1a36b70>
// Frames: F37 -> F18
// 0x141a37bee  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141a37bf2  488d15b7304801                 leaq	0x14830b7(%rip), %rdx   # 0x142ebacb0
// 0x141a37bf9  e8f2a114ff                     callq	0x140b81df0 <.text+0xb80df0>
// 0x141a37bfe  e96dffffff                     jmp	0x141a37b70 <.text+0x1a36b70>
// Frames: F38
// 0x141a37c03  48894500                       movq	%rax, (%rbp)
// 0x141a37c07  4c8b5510                       movq	0x10(%rbp), %r10
// 0x141a37c0b  488b4518                       movq	0x18(%rbp), %rax
// Frames: F39 -> F40 -> F38
// 0x141a37c0f  4883f802                       cmpq	$0x2, %rax
// 0x141a37c13  731a                           jae	0x141a37c2f <.text+0x1a36c2f>
// Frames: F41
// 0x141a37c15  4c895520                       movq	%r10, 0x20(%rbp)
// 0x141a37c19  488b4d08                       movq	0x8(%rbp), %rcx
// 0x141a37c1d  48894df8                       movq	%rcx, -0x8(%rbp)
// Frames: F22 -> F42 -> F41
// 0x141a37c21  4885c0                         testq	%rax, %rax
// Frames: F42 -> F41
// 0x141a37c24  0f85dd000000                   jne	0x141a37d07 <.text+0x1a36d07>
// 0x141a37c2a  e999020000                     jmp	0x141a37ec8 <.text+0x1a36ec8>
// Frames: F43 -> F40 -> F38
// 0x141a37c2f  4889c1                         movq	%rax, %rcx
// 0x141a37c32  48d1e9                         shrq	%rcx
// 0x141a37c35  4883f808                       cmpq	$0x8, %rax
// 0x141a37c39  7304                           jae	0x141a37c3f <.text+0x1a36c3f>
// 0x141a37c3b  31d2                           xorl	%edx, %edx
// 0x141a37c3d  eb77                           jmp	0x141a37cb6 <.text+0x1a36cb6>
// 0x141a37c3f  4889ca                         movq	%rcx, %rdx
// 0x141a37c42  4883e2fc                       andq	$-0x4, %rdx
// 0x141a37c46  4d8d04c2                       leaq	(%r10,%rax,8), %r8
// 0x141a37c4a  4983c0f0                       addq	$-0x10, %r8
// 0x141a37c4e  4531c9                         xorl	%r9d, %r9d
// 0x141a37c51  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F44 -> F43 -> F40 -> F38
// 0x141a37c60  f3430f6f04ca                   movdqu	(%r10,%r9,8), %xmm0
// 0x141a37c66  f3430f6f4cca10                 movdqu	0x10(%r10,%r9,8), %xmm1
// 0x141a37c6d  f3410f6f50f0                   movdqu	-0x10(%r8), %xmm2
// 0x141a37c73  f3410f6f18                     movdqu	(%r8), %xmm3
// 0x141a37c78  660f70db4e                     pshufd	$0x4e, %xmm3, %xmm3     # xmm3 = xmm3[2,3,0,1]
// 0x141a37c7d  660f70d24e                     pshufd	$0x4e, %xmm2, %xmm2     # xmm2 = xmm2[2,3,0,1]
// 0x141a37c82  f3430f7f1cca                   movdqu	%xmm3, (%r10,%r9,8)
// 0x141a37c88  f3430f7f54ca10                 movdqu	%xmm2, 0x10(%r10,%r9,8)
// 0x141a37c8f  660f70c04e                     pshufd	$0x4e, %xmm0, %xmm0     # xmm0 = xmm0[2,3,0,1]
// 0x141a37c94  f3410f7f00                     movdqu	%xmm0, (%r8)
// 0x141a37c99  660f70c14e                     pshufd	$0x4e, %xmm1, %xmm0     # xmm0 = xmm1[2,3,0,1]
// 0x141a37c9e  f3410f7f40f0                   movdqu	%xmm0, -0x10(%r8)
// Frames: F45 -> F40 -> F38
// 0x141a37ca4  4983c104                       addq	$0x4, %r9
// 0x141a37ca8  4983c0e0                       addq	$-0x20, %r8
// 0x141a37cac  4c39ca                         cmpq	%r9, %rdx
// 0x141a37caf  75af                           jne	0x141a37c60 <.text+0x1a36c60>
// Frames: F39 -> F40 -> F38
// 0x141a37cb1  4839d1                         cmpq	%rdx, %rcx
// 0x141a37cb4  7434                           je	0x141a37cea <.text+0x1a36cea>
// 0x141a37cb6  48f7d9                         negq	%rcx
// 0x141a37cb9  4d8d04d2                       leaq	(%r10,%rdx,8), %r8
// 0x141a37cbd  48f7da                         negq	%rdx
// 0x141a37cc0  498d04c2                       leaq	(%r10,%rax,8), %rax
// 0x141a37cc4  4883c0f8                       addq	$-0x8, %rax
// 0x141a37cc8  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F44 -> F43 -> F40 -> F38
// 0x141a37cd0  4d8b08                         movq	(%r8), %r9
// 0x141a37cd3  4c8b14d0                       movq	(%rax,%rdx,8), %r10
// 0x141a37cd7  4d8910                         movq	%r10, (%r8)
// 0x141a37cda  4c890cd0                       movq	%r9, (%rax,%rdx,8)
// Frames: F39 -> F40 -> F38
// 0x141a37cde  48ffca                         decq	%rdx
// 0x141a37ce1  4983c008                       addq	$0x8, %r8
// 0x141a37ce5  4839d1                         cmpq	%rdx, %rcx
// 0x141a37ce8  75e6                           jne	0x141a37cd0 <.text+0x1a36cd0>
// Frames: F41
// 0x141a37cea  488b4510                       movq	0x10(%rbp), %rax
// 0x141a37cee  48894520                       movq	%rax, 0x20(%rbp)
// 0x141a37cf2  488b4518                       movq	0x18(%rbp), %rax
// 0x141a37cf6  488b4d08                       movq	0x8(%rbp), %rcx
// 0x141a37cfa  48894df8                       movq	%rcx, -0x8(%rbp)
// Frames: F22 -> F42 -> F41
// 0x141a37cfe  4885c0                         testq	%rax, %rax
// Frames: F42 -> F41
// 0x141a37d01  0f84c1010000                   je	0x141a37ec8 <.text+0x1a36ec8>
// 0x141a37d07  4c8b6d20                       movq	0x20(%rbp), %r13
// 0x141a37d0b  4c8d3cc500000000               leaq	(,%rax,8), %r15
// 0x141a37d13  4d01ef                         addq	%r13, %r15
// 0x141a37d16  488d7dc8                       leaq	-0x38(%rbp), %rdi
// 0x141a37d1a  488d5dd0                       leaq	-0x30(%rbp), %rbx
// 0x141a37d1e  4c8d25a32f4801                 leaq	0x1482fa3(%rip), %r12   # 0x142ebacc8
// 0x141a37d25  eb16                           jmp	0x141a37d3d <.text+0x1a36d3d>
// Frames: F46 -> F47
// 0x141a37d27  4889f9                         movq	%rdi, %rcx
// 0x141a37d2a  e881a1e5fe                     callq	0x140891eb0 <.text+0x890eb0>
// 0x141a37d2f  90                             nop
// Frames: F22 -> F42 -> F41
// 0x141a37d30  4983c508                       addq	$0x8, %r13
// 0x141a37d34  4d39fd                         cmpq	%r15, %r13
// Frames: F42 -> F41
// 0x141a37d37  0f848b010000                   je	0x141a37ec8 <.text+0x1a36ec8>
// Frames: F48 -> F49 -> F50 -> F41
// 0x141a37d3d  498b4d00                       movq	(%r13), %rcx
// Frames: F51 -> F52 -> F47
// 0x141a37d41  4c8b7610                       movq	0x10(%rsi), %r14
// Frames: F53 -> F47
// 0x141a37d45  4939ce                         cmpq	%rcx, %r14
// 0x141a37d48  0f86ae010000                   jbe	0x141a37efc <.text+0x1a36efc>
// Frames: F13 -> F14 -> F15 -> F16 -> F54 -> F47
// 0x141a37d4e  488b5608                       movq	0x8(%rsi), %rdx
// Frames: F55 -> F54 -> F47
// 0x141a37d52  4c8d0449                       leaq	(%rcx,%rcx,2), %r8
// 0x141a37d56  49c1e004                       shlq	$0x4, %r8
// 0x141a37d5a  4a8d0402                       leaq	(%rdx,%r8), %rax
// Frames: F48 -> F56 -> F47
// 0x141a37d5e  f3420f6f0402                   movdqu	(%rdx,%r8), %xmm0
// 0x141a37d64  420f104c0210                   movups	0x10(%rdx,%r8), %xmm1
// 0x141a37d6a  f3420f6f540220                 movdqu	0x20(%rdx,%r8), %xmm2
// 0x141a37d71  660f7f55e0                     movdqa	%xmm2, -0x20(%rbp)
// 0x141a37d76  0f294dd0                       movaps	%xmm1, -0x30(%rbp)
// 0x141a37d7a  660f7f45c0                     movdqa	%xmm0, -0x40(%rbp)
// Frames: F10 -> F57 -> F47
// 0x141a37d7f  4c01c2                         addq	%r8, %rdx
// 0x141a37d82  4883c230                       addq	$0x30, %rdx
// Frames: F57 -> F47
// 0x141a37d86  48f7d1                         notq	%rcx
// 0x141a37d89  4c01f1                         addq	%r14, %rcx
// Frames: F58 -> F57 -> F47
// 0x141a37d8c  48c1e104                       shlq	$0x4, %rcx
// 0x141a37d90  4c8d0449                       leaq	(%rcx,%rcx,2), %r8
// 0x141a37d94  4889c1                         movq	%rax, %rcx
// 0x141a37d97  e874beee00                     callq	0x142923c10 <.text+0x2922c10>
// Frames: F59 -> F47
// 0x141a37d9c  49ffce                         decq	%r14
// Frames: F60 -> F59 -> F47
// 0x141a37d9f  4c897610                       movq	%r14, 0x10(%rsi)
// Frames: F46 -> F47
// 0x141a37da3  0fb645c0                       movzbl	-0x40(%rbp), %eax
// 0x141a37da7  4883f809                       cmpq	$0x9, %rax
// 0x141a37dab  0f87e1000000                   ja	0x141a37e92 <.text+0x1a36e92>
// 0x141a37db1  49630484                       movslq	(%r12,%rax,4), %rax
// 0x141a37db5  4c01e0                         addq	%r12, %rax
// 0x141a37db8  ffe0                           jmpq	*%rax
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37dba  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37dbe  f0                             lock
// 0x141a37dbf  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37dc2  0f8568ffffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37dc8  4889d9                         movq	%rbx, %rcx
// 0x141a37dcb  e8f05112ff                     callq	0x140b5cfc0 <.text+0xb5bfc0>
// 0x141a37dd0  e95bffffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37dd5  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37dd9  f0                             lock
// 0x141a37dda  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37ddd  0f854dffffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37de3  4889d9                         movq	%rbx, %rcx
// 0x141a37de6  e895df8afe                     callq	0x1402e5d80 <.text+0x2e4d80>
// 0x141a37deb  e940ffffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37df0  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37df4  f0                             lock
// 0x141a37df5  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37df8  0f8532ffffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37dfe  4889d9                         movq	%rbx, %rcx
// 0x141a37e01  e8ba5112ff                     callq	0x140b5cfc0 <.text+0xb5bfc0>
// 0x141a37e06  e925ffffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e0b  488b45c8                       movq	-0x38(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e0f  f0                             lock
// 0x141a37e10  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e13  0f8517ffffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37e19  4889f9                         movq	%rdi, %rcx
// 0x141a37e1c  e87f4412ff                     callq	0x140b5c2a0 <.text+0xb5b2a0>
// 0x141a37e21  e90affffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e26  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e2a  f0                             lock
// 0x141a37e2b  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e2e  0f85fcfeffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37e34  4889d9                         movq	%rbx, %rcx
// 0x141a37e37  e8d45112ff                     callq	0x140b5d010 <.text+0xb5c010>
// 0x141a37e3c  e9effeffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e41  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e45  f0                             lock
// 0x141a37e46  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e49  0f85e1feffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37e4f  4889d9                         movq	%rbx, %rcx
// 0x141a37e52  e8695112ff                     callq	0x140b5cfc0 <.text+0xb5bfc0>
// 0x141a37e57  e9d4feffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e5c  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e60  f0                             lock
// 0x141a37e61  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e64  0f85c6feffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37e6a  4889d9                         movq	%rbx, %rcx
// 0x141a37e6d  e8ce4e12ff                     callq	0x140b5cd40 <.text+0xb5bd40>
// 0x141a37e72  e9b9feffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e77  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e7b  f0                             lock
// 0x141a37e7c  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e7f  0f85abfeffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37e85  4889d9                         movq	%rbx, %rcx
// 0x141a37e88  e8835112ff                     callq	0x140b5d010 <.text+0xb5c010>
// 0x141a37e8d  e99efeffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e92  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e96  f0                             lock
// 0x141a37e97  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37e9a  0f8590feffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37ea0  4889d9                         movq	%rbx, %rcx
// 0x141a37ea3  e838fac5fe                     callq	0x1406978e0 <.text+0x6968e0>
// 0x141a37ea8  e983feffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F61 -> F62 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37ead  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37eb1  f0                             lock
// 0x141a37eb2  48ff08                         decq	(%rax)
// Frames: F63 -> F46 -> F46 -> F46 -> F47
// 0x141a37eb5  0f8575feffff                   jne	0x141a37d30 <.text+0x1a36d30>
// Frames: F66 -> F46 -> F46 -> F46 -> F47
// 0x141a37ebb  4889d9                         movq	%rbx, %rcx
// 0x141a37ebe  e87d4112ff                     callq	0x140b5c040 <.text+0xb5b040>
// 0x141a37ec3  e968feffff                     jmp	0x141a37d30 <.text+0x1a36d30>
// Frames: F67 -> F68 -> F69 -> F46 -> F70 -> F46 -> F71 -> F46 -> F72
// 0x141a37ec8  488b55f8                       movq	-0x8(%rbp), %rdx
// 0x141a37ecc  4885d2                         testq	%rdx, %rdx
// 0x141a37ecf  7413                           je	0x141a37ee4 <.text+0x1a36ee4>
// Frames: F73 -> F74 -> F68 -> F69 -> F46 -> F70 -> F46 -> F71 -> F46 -> F72
// 0x141a37ed1  48c1e203                       shlq	$0x3, %rdx
// Frames: F75 -> F76 -> F77 -> F69 -> F46 -> F70 -> F46 -> F71 -> F46 -> F72
// 0x141a37ed5  41b808000000                   movl	$0x8, %r8d
// 0x141a37edb  488b4d20                       movq	0x20(%rbp), %rcx
// 0x141a37edf  e83cbdbdfe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F78
// 0x141a37ee4  0f287530                       movaps	0x30(%rbp), %xmm6
// 0x141a37ee8  4881c4c8000000                 addq	$0xc8, %rsp
// 0x141a37eef  5b                             popq	%rbx
// 0x141a37ef0  5f                             popq	%rdi
// 0x141a37ef1  5e                             popq	%rsi
// 0x141a37ef2  415c                           popq	%r12
// 0x141a37ef4  415d                           popq	%r13
// 0x141a37ef6  415e                           popq	%r14
// 0x141a37ef8  415f                           popq	%r15
// 0x141a37efa  5d                             popq	%rbp
// 0x141a37efb  c3                             retq
// Frames: F79 -> F47
// 0x141a37efc  4c8d05952d4801                 leaq	0x1482d95(%rip), %r8    # 0x142ebac98
// 0x141a37f03  4c89f2                         movq	%r14, %rdx
// 0x141a37f06  e8de11ea00                     callq	0x1428d90e9 <.text+0x28d80e9>
// 0x141a37f0b  0f0b                           ud2
// 0x141a37f0d  0f1f00                         nopl	(%rax)
// 0x141a37f10  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141a37f15  55                             pushq	%rbp
// 0x141a37f16  4157                           pushq	%r15
// 0x141a37f18  4156                           pushq	%r14
// 0x141a37f1a  4155                           pushq	%r13
// 0x141a37f1c  4154                           pushq	%r12
// 0x141a37f1e  56                             pushq	%rsi
// 0x141a37f1f  57                             pushq	%rdi
// 0x141a37f20  53                             pushq	%rbx
// 0x141a37f21  4883ec48                       subq	$0x48, %rsp
// 0x141a37f25  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141a37f2c  0f29742430                     movaps	%xmm6, 0x30(%rsp)
// 0x141a37f31  488b55f8                       movq	-0x8(%rbp), %rdx
// Frames: F67 -> F68 -> F69 -> F46 -> F70 -> F46 -> F71 -> F46 -> F72
// 0x141a37f35  4885d2                         testq	%rdx, %rdx
// 0x141a37f38  7413                           je	0x141a37f4d <.text+0x1a36f4d>
// Frames: F73 -> F74 -> F68 -> F69 -> F46 -> F70 -> F46 -> F71 -> F46 -> F72
// 0x141a37f3a  48c1e203                       shlq	$0x3, %rdx
// Frames: F75 -> F76 -> F77 -> F69 -> F46 -> F70 -> F46 -> F71 -> F46 -> F72
// 0x141a37f3e  41b808000000                   movl	$0x8, %r8d
// 0x141a37f44  488b4d20                       movq	0x20(%rbp), %rcx
// 0x141a37f48  e8d3bcbdfe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F72
// 0x141a37f4d  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141a37f52  4883c448                       addq	$0x48, %rsp
// 0x141a37f56  5b                             popq	%rbx
// 0x141a37f57  5f                             popq	%rdi
// 0x141a37f58  5e                             popq	%rsi
// 0x141a37f59  415c                           popq	%r12
// 0x141a37f5b  415d                           popq	%r13
// 0x141a37f5d  415e                           popq	%r14
// 0x141a37f5f  415f                           popq	%r15
// 0x141a37f61  5d                             popq	%rbp
// 0x141a37f62  c3                             retq
// 0x141a37f63  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x141a37f70  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141a37f75  55                             pushq	%rbp
// 0x141a37f76  4157                           pushq	%r15
// 0x141a37f78  4156                           pushq	%r14
// 0x141a37f7a  4155                           pushq	%r13
// 0x141a37f7c  4154                           pushq	%r12
// 0x141a37f7e  56                             pushq	%rsi
// 0x141a37f7f  57                             pushq	%rdi
// 0x141a37f80  53                             pushq	%rbx
// 0x141a37f81  4883ec48                       subq	$0x48, %rsp
// 0x141a37f85  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141a37f8c  0f29742430                     movaps	%xmm6, 0x30(%rsp)
// 0x141a37f91  488b4500                       movq	(%rbp), %rax
// Frames: F22 -> F23 -> F24 -> F9
// 0x141a37f95  483b45f0                       cmpq	-0x10(%rbp), %rax
// Frames: F80
// 0x141a37f99  741c                           je	0x141a37fb7 <.text+0x1a36fb7>
// Frames: F68 -> F69 -> F46 -> F46 -> F80
// 0x141a37f9b  488b5508                       movq	0x8(%rbp), %rdx
// Frames: F67 -> F68 -> F69 -> F46 -> F46 -> F80
// 0x141a37f9f  4885d2                         testq	%rdx, %rdx
// 0x141a37fa2  7413                           je	0x141a37fb7 <.text+0x1a36fb7>
// Frames: F68 -> F69 -> F46 -> F46 -> F80
// 0x141a37fa4  488b4d10                       movq	0x10(%rbp), %rcx
// Frames: F73 -> F74 -> F68 -> F69 -> F46 -> F46 -> F80
// 0x141a37fa8  48c1e203                       shlq	$0x3, %rdx
// Frames: F75 -> F76 -> F77 -> F69 -> F46 -> F46 -> F80
// 0x141a37fac  41b808000000                   movl	$0x8, %r8d
// 0x141a37fb2  e869bcbdfe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F1
// 0x141a37fb7  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141a37fbc  4883c448                       addq	$0x48, %rsp
// 0x141a37fc0  5b                             popq	%rbx
// 0x141a37fc1  5f                             popq	%rdi
// 0x141a37fc2  5e                             popq	%rsi
// 0x141a37fc3  415c                           popq	%r12
// 0x141a37fc5  415d                           popq	%r13
// 0x141a37fc7  415e                           popq	%r14
// 0x141a37fc9  415f                           popq	%r15
// 0x141a37fcb  5d                             popq	%rbp
// 0x141a37fcc  c3                             retq
//
// RANGE rva_21b48e0_len_4ff: RVA 0x21b48e0, 1279 PDB bytes; 303 instructions
// Evidence: _assembly/roof-v1/B/pack-003/rva_21b48e0_len_4ff.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3476:260: system_roof::replay_roof::impl$0::process
// Frames: F81
// 0x1421b48e0  4157                           pushq	%r15
// 0x1421b48e2  4156                           pushq	%r14
// 0x1421b48e4  4154                           pushq	%r12
// 0x1421b48e6  56                             pushq	%rsi
// 0x1421b48e7  57                             pushq	%rdi
// 0x1421b48e8  55                             pushq	%rbp
// 0x1421b48e9  53                             pushq	%rbx
// 0x1421b48ea  4881eca0000000                 subq	$0xa0, %rsp
// 0x1421b48f1  440f29942490000000             movaps	%xmm10, 0x90(%rsp)
// 0x1421b48fa  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x1421b4903  440f29442470                   movaps	%xmm8, 0x70(%rsp)
// 0x1421b4909  0f297c2460                     movaps	%xmm7, 0x60(%rsp)
// 0x1421b490e  0f29742450                     movaps	%xmm6, 0x50(%rsp)
// Frames: F82
// 0x1421b4913  488b4108                       movq	0x8(%rcx), %rax
// 0x1421b4917  0fb65010                       movzbl	0x10(%rax), %edx
// 0x1421b491b  4c8d15963de400                 leaq	0xe43d96(%rip), %r10    # 0x142ff86b8
// 0x1421b4922  49631492                       movslq	(%r10,%rdx,4), %rdx
// 0x1421b4926  4c01d2                         addq	%r10, %rdx
// 0x1421b4929  ffe2                           jmpq	*%rdx
// Frames: F83
// 0x1421b492b  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x1421b492f  0f8519040000                   jne	0x1421b4d4e <.text+0x21b3d4e>
// Frames: F84
// 0x1421b4935  488b7840                       movq	0x40(%rax), %rdi
// Frames: F85
// 0x1421b4939  0f104024                       movups	0x24(%rax), %xmm0
// 0x1421b493d  0f104830                       movups	0x30(%rax), %xmm1
// 0x1421b4941  0f114c243c                     movups	%xmm1, 0x3c(%rsp)
// 0x1421b4946  0f29442430                     movaps	%xmm0, 0x30(%rsp)
// Frames: F86
// 0x1421b494b  8b6814                         movl	0x14(%rax), %ebp
// Frames: F87
// 0x1421b494e  8b5020                         movl	0x20(%rax), %edx
// 0x1421b4951  89542428                       movl	%edx, 0x28(%rsp)
// 0x1421b4955  488b4018                       movq	0x18(%rax), %rax
// 0x1421b4959  4889442420                     movq	%rax, 0x20(%rsp)
// Frames: F4 -> F5 -> F88 -> F89
// 0x1421b495e  418b401c                       movl	0x1c(%r8), %eax
// 0x1421b4962  498b5010                       movq	0x10(%r8), %rdx
// 0x1421b4966  8902                           movl	%eax, (%rdx)
// Frames: F90 -> F88 -> F89
// 0x1421b4968  498b30                         movq	(%r8), %rsi
// Frames: F35 -> F91 -> F92 -> F88 -> F89
// 0x1421b496b  488b5e30                       movq	0x30(%rsi), %rbx
// Frames: F93 -> F92 -> F88 -> F89
// 0x1421b496f  4c8b7640                       movq	0x40(%rsi), %r14
// Frames: F36 -> F91 -> F92 -> F88 -> F89
// 0x1421b4973  483b5e20                       cmpq	0x20(%rsi), %rbx
// 0x1421b4977  7519                           jne	0x1421b4992 <.text+0x21b3992>
// Frames: F91 -> F92 -> F88 -> F89
// 0x1421b4979  488d4620                       leaq	0x20(%rsi), %rax
// Frames: F37 -> F91 -> F92 -> F88 -> F89
// 0x1421b497d  488d15ece39b00                 leaq	0x9be3ec(%rip), %rdx    # 0x142b72d70
// 0x1421b4984  4989cf                         movq	%rcx, %r15
// 0x1421b4987  4889c1                         movq	%rax, %rcx
// 0x1421b498a  e821de3afe                     callq	0x1405627b0 <.text+0x5617b0>
// 0x1421b498f  4c89f9                         movq	%r15, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F91 -> F92 -> F88 -> F89
// 0x1421b4992  488b4628                       movq	0x28(%rsi), %rax
// Frames: F55 -> F17 -> F91 -> F92 -> F88 -> F89
// 0x1421b4996  4889da                         movq	%rbx, %rdx
// 0x1421b4999  48c1e206                       shlq	$0x6, %rdx
// Frames: F19 -> F20 -> F91 -> F92 -> F88 -> F89
// 0x1421b499d  4c893410                       movq	%r14, (%rax,%rdx)
// 0x1421b49a1  48897c1008                     movq	%rdi, 0x8(%rax,%rdx)
// 0x1421b49a6  896c1010                       movl	%ebp, 0x10(%rax,%rdx)
// 0x1421b49aa  4c8b442420                     movq	0x20(%rsp), %r8
// 0x1421b49af  4c89441014                     movq	%r8, 0x14(%rax,%rdx)
// 0x1421b49b4  448b442428                     movl	0x28(%rsp), %r8d
// 0x1421b49b9  448944101c                     movl	%r8d, 0x1c(%rax,%rdx)
// 0x1421b49be  0f28442430                     movaps	0x30(%rsp), %xmm0
// 0x1421b49c3  0f11441020                     movups	%xmm0, 0x20(%rax,%rdx)
// 0x1421b49c8  0f1044243c                     movups	0x3c(%rsp), %xmm0
// 0x1421b49cd  0f1144102c                     movups	%xmm0, 0x2c(%rax,%rdx)
// 0x1421b49d2  c644103c00                     movb	$0x0, 0x3c(%rax,%rdx)
// Frames: F21 -> F91 -> F92 -> F88 -> F89
// 0x1421b49d7  e9c0030000                     jmp	0x1421b4d9c <.text+0x21b3d9c>
// Frames: F94
// 0x1421b49dc  f30f107110                     movss	0x10(%rcx), %xmm6
// 0x1421b49e1  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421b49e4  0f2ef0                         ucomiss	%xmm0, %xmm6
// 0x1421b49e7  0f8572020000                   jne	0x1421b4c5f <.text+0x21b3c5f>
// 0x1421b49ed  0f8a6c020000                   jp	0x1421b4c5f <.text+0x21b3c5f>
// Frames: F95
// 0x1421b49f3  488b942410010000               movq	0x110(%rsp), %rdx
// 0x1421b49fb  440fb64114                     movzbl	0x14(%rcx), %r8d
// 0x1421b4a00  4989c1                         movq	%rax, %r9
// 0x1421b4a03  4d29c1                         subq	%r8, %r9
// 0x1421b4a06  410fb65912                     movzbl	0x12(%r9), %ebx
// Frames: F96
// 0x1421b4a0b  4c8b7018                       movq	0x18(%rax), %r14
// Frames: F4 -> F5 -> F88 -> F97
// 0x1421b4a0f  8b421c                         movl	0x1c(%rdx), %eax
// 0x1421b4a12  4c8b4210                       movq	0x10(%rdx), %r8
// 0x1421b4a16  418900                         movl	%eax, (%r8)
// Frames: F90 -> F88 -> F97
// 0x1421b4a19  488b32                         movq	(%rdx), %rsi
// Frames: F35 -> F91 -> F92 -> F88 -> F97
// 0x1421b4a1c  488b7e30                       movq	0x30(%rsi), %rdi
// Frames: F93 -> F92 -> F88 -> F97
// 0x1421b4a20  4c8b7e40                       movq	0x40(%rsi), %r15
// Frames: F36 -> F91 -> F92 -> F88 -> F97
// 0x1421b4a24  483b7e20                       cmpq	0x20(%rsi), %rdi
// 0x1421b4a28  751f                           jne	0x1421b4a49 <.text+0x21b3a49>
// Frames: F91 -> F92 -> F88 -> F97
// 0x1421b4a2a  488d4620                       leaq	0x20(%rsi), %rax
// Frames: F37 -> F91 -> F92 -> F88 -> F97
// 0x1421b4a2e  488d153be39b00                 leaq	0x9be33b(%rip), %rdx    # 0x142b72d70
// 0x1421b4a35  4989cc                         movq	%rcx, %r12
// 0x1421b4a38  4889c1                         movq	%rax, %rcx
// 0x1421b4a3b  0f28f9                         movaps	%xmm1, %xmm7
// 0x1421b4a3e  e86de33afe                     callq	0x140562db0 <.text+0x561db0>
// 0x1421b4a43  0f28cf                         movaps	%xmm7, %xmm1
// 0x1421b4a46  4c89e1                         movq	%r12, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F91 -> F92 -> F88 -> F97
// 0x1421b4a49  488b4628                       movq	0x28(%rsi), %rax
// Frames: F55 -> F17 -> F91 -> F92 -> F88 -> F97
// 0x1421b4a4d  488d147f                       leaq	(%rdi,%rdi,2), %rdx
// Frames: F19 -> F20 -> F91 -> F92 -> F88 -> F97
// 0x1421b4a51  4c893cd0                       movq	%r15, (%rax,%rdx,8)
// 0x1421b4a55  4c8974d008                     movq	%r14, 0x8(%rax,%rdx,8)
// 0x1421b4a5a  885cd010                       movb	%bl, 0x10(%rax,%rdx,8)
// 0x1421b4a5e  885cd011                       movb	%bl, 0x11(%rax,%rdx,8)
// 0x1421b4a62  c644d01200                     movb	$0x0, 0x12(%rax,%rdx,8)
// Frames: F21 -> F91 -> F92 -> F88 -> F97
// 0x1421b4a67  48ffc7                         incq	%rdi
// 0x1421b4a6a  48897e30                       movq	%rdi, 0x30(%rsi)
// Frames: F98 -> F92 -> F88 -> F97
// 0x1421b4a6e  e9e8010000                     jmp	0x1421b4c5b <.text+0x21b3c5b>
// Frames: F99
// 0x1421b4a73  f60101                         testb	$0x1, (%rcx)
// 0x1421b4a76  741f                           je	0x1421b4a97 <.text+0x21b3a97>
// Frames: F100
// 0x1421b4a78  f30f584910                     addss	0x10(%rcx), %xmm1
// 0x1421b4a7d  f30f114910                     movss	%xmm1, 0x10(%rcx)
// Frames: F101
// 0x1421b4a82  f30f5e4904                     divss	0x4(%rcx), %xmm1
// 0x1421b4a87  0f57f6                         xorps	%xmm6, %xmm6
// Frames: F102 -> F101
// 0x1421b4a8a  f30f5ff1                       maxss	%xmm1, %xmm6
// 0x1421b4a8e  0f2e356f0e7700                 ucomiss	0x770e6f(%rip), %xmm6   # 0x142925904
// 0x1421b4a95  720c                           jb	0x1421b4aa3 <.text+0x21b3aa3>
// Frames: F103
// 0x1421b4a97  c6411501                       movb	$0x1, 0x15(%rcx)
// 0x1421b4a9b  f30f1035610e7700               movss	0x770e61(%rip), %xmm6   # 0x142925904
// Frames: F104
// 0x1421b4aa3  488b942400010000               movq	0x100(%rsp), %rdx
// 0x1421b4aab  f30f1005510e7700               movss	0x770e51(%rip), %xmm0   # 0x142925904
// 0x1421b4ab3  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x1421b4ab7  740a                           je	0x1421b4ac3 <.text+0x21b3ac3>
// 0x1421b4ab9  0f28c8                         movaps	%xmm0, %xmm1
// 0x1421b4abc  f30f5cce                       subss	%xmm6, %xmm1
// 0x1421b4ac0  0f28f1                         movaps	%xmm1, %xmm6
// Frames: F105
// 0x1421b4ac3  488b7038                       movq	0x38(%rax), %rsi
// Frames: F106 -> F107 -> F108
// 0x1421b4ac7  f30f5cc6                       subss	%xmm6, %xmm0
// 0x1421b4acb  f30f104818                     movss	0x18(%rax), %xmm1
// Frames: F109 -> F107 -> F108
// 0x1421b4ad0  f30f59c8                       mulss	%xmm0, %xmm1
// 0x1421b4ad4  f30f107828                     movss	0x28(%rax), %xmm7
// 0x1421b4ad9  f30f59fe                       mulss	%xmm6, %xmm7
// Frames: F110 -> F107 -> F108
// 0x1421b4add  f30f58f9                       addss	%xmm1, %xmm7
// 0x1421b4ae1  f30f104814                     movss	0x14(%rax), %xmm1
// Frames: F109 -> F107 -> F111
// 0x1421b4ae6  f30f59c8                       mulss	%xmm0, %xmm1
// 0x1421b4aea  f3440f104024                   movss	0x24(%rax), %xmm8
// 0x1421b4af0  f3440f59c6                     mulss	%xmm6, %xmm8
// Frames: F110 -> F107 -> F111
// 0x1421b4af5  f3440f58c1                     addss	%xmm1, %xmm8
// Frames: F112
// 0x1421b4afa  f20f104840                     movsd	0x40(%rax), %xmm1
// Frames: F113
// 0x1421b4aff  f20f105048                     movsd	0x48(%rax), %xmm2
// Frames: F109 -> F114 -> F115 -> F112
// 0x1421b4b04  f30f12d8                       movsldup	%xmm0, %xmm3            # xmm3 = xmm0[0,0,2,2]
// 0x1421b4b08  0f59d9                         mulps	%xmm1, %xmm3
// 0x1421b4b0b  f3440f12ce                     movsldup	%xmm6, %xmm9            # xmm9 = xmm6[0,0,2,2]
// 0x1421b4b10  440f59ca                       mulps	%xmm2, %xmm9
// Frames: F110 -> F116 -> F115 -> F112
// 0x1421b4b14  440f58cb                       addps	%xmm3, %xmm9
// 0x1421b4b18  f30f10481c                     movss	0x1c(%rax), %xmm1
// Frames: F109 -> F107 -> F117
// 0x1421b4b1d  f30f59c8                       mulss	%xmm0, %xmm1
// 0x1421b4b21  f3440f10502c                   movss	0x2c(%rax), %xmm10
// 0x1421b4b27  f3440f59d6                     mulss	%xmm6, %xmm10
// Frames: F109 -> F107 -> F118
// 0x1421b4b2c  f30f594020                     mulss	0x20(%rax), %xmm0
// Frames: F110 -> F107 -> F117
// 0x1421b4b31  f3440f58d1                     addss	%xmm1, %xmm10
// Frames: F109 -> F107 -> F118
// 0x1421b4b36  f30f597030                     mulss	0x30(%rax), %xmm6
// Frames: F110 -> F107 -> F118
// 0x1421b4b3b  f30f58f0                       addss	%xmm0, %xmm6
// Frames: F4 -> F5 -> F88 -> F119
// 0x1421b4b3f  8b421c                         movl	0x1c(%rdx), %eax
// 0x1421b4b42  488b4a10                       movq	0x10(%rdx), %rcx
// 0x1421b4b46  8901                           movl	%eax, (%rcx)
// Frames: F90 -> F88 -> F119
// 0x1421b4b48  488b3a                         movq	(%rdx), %rdi
// Frames: F35 -> F91 -> F92 -> F88 -> F119
// 0x1421b4b4b  488b5f30                       movq	0x30(%rdi), %rbx
// Frames: F93 -> F92 -> F88 -> F119
// 0x1421b4b4f  4c8b7740                       movq	0x40(%rdi), %r14
// Frames: F36 -> F91 -> F92 -> F88 -> F119
// 0x1421b4b53  483b5f20                       cmpq	0x20(%rdi), %rbx
// 0x1421b4b57  7510                           jne	0x1421b4b69 <.text+0x21b3b69>
// Frames: F91 -> F92 -> F88 -> F119
// 0x1421b4b59  488d4f20                       leaq	0x20(%rdi), %rcx
// Frames: F37 -> F91 -> F92 -> F88 -> F119
// 0x1421b4b5d  488d150ce29b00                 leaq	0x9be20c(%rip), %rdx    # 0x142b72d70
// 0x1421b4b64  e847dc3afe                     callq	0x1405627b0 <.text+0x5617b0>
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F91 -> F92 -> F88 -> F119
// 0x1421b4b69  488b4728                       movq	0x28(%rdi), %rax
// Frames: F55 -> F17 -> F91 -> F92 -> F88 -> F119
// 0x1421b4b6d  4889d9                         movq	%rbx, %rcx
// 0x1421b4b70  48c1e106                       shlq	$0x6, %rcx
// Frames: F19 -> F20 -> F91 -> F92 -> F88 -> F119
// 0x1421b4b74  c7040801000000                 movl	$0x1, (%rax,%rcx)
// 0x1421b4b7b  f30f117c0804                   movss	%xmm7, 0x4(%rax,%rcx)
// 0x1421b4b81  c744080801000000               movl	$0x1, 0x8(%rax,%rcx)
// 0x1421b4b89  f3440f1144080c                 movss	%xmm8, 0xc(%rax,%rcx)
// 0x1421b4b90  c744081001000000               movl	$0x1, 0x10(%rax,%rcx)
// 0x1421b4b98  f3440f11540814                 movss	%xmm10, 0x14(%rax,%rcx)
// 0x1421b4b9f  c744081801000000               movl	$0x1, 0x18(%rax,%rcx)
// 0x1421b4ba7  f30f1174081c                   movss	%xmm6, 0x1c(%rax,%rcx)
// 0x1421b4bad  4889740820                     movq	%rsi, 0x20(%rax,%rcx)
// 0x1421b4bb2  c744082801000000               movl	$0x1, 0x28(%rax,%rcx)
// 0x1421b4bba  440f134c082c                   movlps	%xmm9, 0x2c(%rax,%rcx)
// 0x1421b4bc0  c644083401                     movb	$0x1, 0x34(%rax,%rcx)
// 0x1421b4bc5  4c89740838                     movq	%r14, 0x38(%rax,%rcx)
// Frames: F21 -> F91 -> F92 -> F88 -> F119
// 0x1421b4bca  48ffc3                         incq	%rbx
// 0x1421b4bcd  48895f30                       movq	%rbx, 0x30(%rdi)
// Frames: F98 -> F92 -> F88 -> F119
// 0x1421b4bd1  48ff4740                       incq	0x40(%rdi)
// Frames: F120
// 0x1421b4bd5  e9d1010000                     jmp	0x1421b4dab <.text+0x21b3dab>
// Frames: F121
// 0x1421b4bda  f30f107110                     movss	0x10(%rcx), %xmm6
// 0x1421b4bdf  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421b4be2  0f2ef0                         ucomiss	%xmm0, %xmm6
// 0x1421b4be5  7578                           jne	0x1421b4c5f <.text+0x21b3c5f>
// 0x1421b4be7  7a76                           jp	0x1421b4c5f <.text+0x21b3c5f>
// Frames: F122
// 0x1421b4be9  488b942408010000               movq	0x108(%rsp), %rdx
// 0x1421b4bf1  440fb64114                     movzbl	0x14(%rcx), %r8d
// 0x1421b4bf6  4989c1                         movq	%rax, %r9
// 0x1421b4bf9  4d29c1                         subq	%r8, %r9
// 0x1421b4bfc  410fb65912                     movzbl	0x12(%r9), %ebx
// Frames: F123
// 0x1421b4c01  488b7818                       movq	0x18(%rax), %rdi
// Frames: F4 -> F5 -> F88 -> F124
// 0x1421b4c05  8b421c                         movl	0x1c(%rdx), %eax
// 0x1421b4c08  4c8b4210                       movq	0x10(%rdx), %r8
// 0x1421b4c0c  418900                         movl	%eax, (%r8)
// Frames: F90 -> F88 -> F124
// 0x1421b4c0f  488b32                         movq	(%rdx), %rsi
// Frames: F35 -> F91 -> F92 -> F88 -> F124
// 0x1421b4c12  4c8b7630                       movq	0x30(%rsi), %r14
// Frames: F93 -> F92 -> F88 -> F124
// 0x1421b4c16  4c8b7e40                       movq	0x40(%rsi), %r15
// Frames: F36 -> F91 -> F92 -> F88 -> F124
// 0x1421b4c1a  4c3b7620                       cmpq	0x20(%rsi), %r14
// 0x1421b4c1e  751f                           jne	0x1421b4c3f <.text+0x21b3c3f>
// Frames: F91 -> F92 -> F88 -> F124
// 0x1421b4c20  488d4620                       leaq	0x20(%rsi), %rax
// Frames: F37 -> F91 -> F92 -> F88 -> F124
// 0x1421b4c24  488d1545e19b00                 leaq	0x9be145(%rip), %rdx    # 0x142b72d70
// 0x1421b4c2b  4989cc                         movq	%rcx, %r12
// 0x1421b4c2e  4889c1                         movq	%rax, %rcx
// 0x1421b4c31  0f28f9                         movaps	%xmm1, %xmm7
// 0x1421b4c34  e877e13afe                     callq	0x140562db0 <.text+0x561db0>
// 0x1421b4c39  0f28cf                         movaps	%xmm7, %xmm1
// 0x1421b4c3c  4c89e1                         movq	%r12, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F91 -> F92 -> F88 -> F124
// 0x1421b4c3f  488b4628                       movq	0x28(%rsi), %rax
// Frames: F55 -> F17 -> F91 -> F92 -> F88 -> F124
// 0x1421b4c43  4b8d1476                       leaq	(%r14,%r14,2), %rdx
// Frames: F19 -> F20 -> F91 -> F92 -> F88 -> F124
// 0x1421b4c47  4c893cd0                       movq	%r15, (%rax,%rdx,8)
// 0x1421b4c4b  48897cd008                     movq	%rdi, 0x8(%rax,%rdx,8)
// 0x1421b4c50  885cd010                       movb	%bl, 0x10(%rax,%rdx,8)
// Frames: F21 -> F91 -> F92 -> F88 -> F124
// 0x1421b4c54  49ffc6                         incq	%r14
// 0x1421b4c57  4c897630                       movq	%r14, 0x30(%rsi)
// 0x1421b4c5b  48ff4640                       incq	0x40(%rsi)
// 0x1421b4c5f  833901                         cmpl	$0x1, (%rcx)
// 0x1421b4c62  0f853f010000                   jne	0x1421b4da7 <.text+0x21b3da7>
// Frames: F102 -> F125
// 0x1421b4c68  f30f58f1                       addss	%xmm1, %xmm6
// 0x1421b4c6c  f30f117110                     movss	%xmm6, 0x10(%rcx)
// 0x1421b4c71  f30f5e7104                     divss	0x4(%rcx), %xmm6
// 0x1421b4c76  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421b4c79  f30f5fc6                       maxss	%xmm6, %xmm0
// 0x1421b4c7d  f30f100d7f0c7700               movss	0x770c7f(%rip), %xmm1   # 0x142925904
// Frames: F126 -> F125
// 0x1421b4c85  0f28d1                         movaps	%xmm1, %xmm2
// 0x1421b4c88  f30f5dd0                       minss	%xmm0, %xmm2
// 0x1421b4c8c  0f2ed1                         ucomiss	%xmm1, %xmm2
// 0x1421b4c8f  0f8516010000                   jne	0x1421b4dab <.text+0x21b3dab>
// 0x1421b4c95  0f8b0c010000                   jnp	0x1421b4da7 <.text+0x21b3da7>
// 0x1421b4c9b  e90b010000                     jmp	0x1421b4dab <.text+0x21b3dab>
// Frames: F127
// 0x1421b4ca0  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x1421b4ca4  0f84a4000000                   je	0x1421b4d4e <.text+0x21b3d4e>
// Frames: F128
// 0x1421b4caa  488b7840                       movq	0x40(%rax), %rdi
// Frames: F129
// 0x1421b4cae  0f104024                       movups	0x24(%rax), %xmm0
// 0x1421b4cb2  0f104830                       movups	0x30(%rax), %xmm1
// 0x1421b4cb6  0f114c243c                     movups	%xmm1, 0x3c(%rsp)
// 0x1421b4cbb  0f29442430                     movaps	%xmm0, 0x30(%rsp)
// Frames: F130
// 0x1421b4cc0  8b6814                         movl	0x14(%rax), %ebp
// Frames: F131
// 0x1421b4cc3  8b5020                         movl	0x20(%rax), %edx
// 0x1421b4cc6  89542428                       movl	%edx, 0x28(%rsp)
// 0x1421b4cca  488b4018                       movq	0x18(%rax), %rax
// 0x1421b4cce  4889442420                     movq	%rax, 0x20(%rsp)
// Frames: F4 -> F5 -> F88 -> F132
// 0x1421b4cd3  418b401c                       movl	0x1c(%r8), %eax
// 0x1421b4cd7  498b5010                       movq	0x10(%r8), %rdx
// 0x1421b4cdb  8902                           movl	%eax, (%rdx)
// Frames: F90 -> F88 -> F132
// 0x1421b4cdd  498b30                         movq	(%r8), %rsi
// Frames: F35 -> F91 -> F92 -> F88 -> F132
// 0x1421b4ce0  488b5e30                       movq	0x30(%rsi), %rbx
// Frames: F93 -> F92 -> F88 -> F132
// 0x1421b4ce4  4c8b7640                       movq	0x40(%rsi), %r14
// Frames: F36 -> F91 -> F92 -> F88 -> F132
// 0x1421b4ce8  483b5e20                       cmpq	0x20(%rsi), %rbx
// 0x1421b4cec  7519                           jne	0x1421b4d07 <.text+0x21b3d07>
// Frames: F91 -> F92 -> F88 -> F132
// 0x1421b4cee  488d4620                       leaq	0x20(%rsi), %rax
// Frames: F37 -> F91 -> F92 -> F88 -> F132
// 0x1421b4cf2  488d1577e09b00                 leaq	0x9be077(%rip), %rdx    # 0x142b72d70
// 0x1421b4cf9  4989cf                         movq	%rcx, %r15
// 0x1421b4cfc  4889c1                         movq	%rax, %rcx
// 0x1421b4cff  e8acda3afe                     callq	0x1405627b0 <.text+0x5617b0>
// 0x1421b4d04  4c89f9                         movq	%r15, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F91 -> F92 -> F88 -> F132
// 0x1421b4d07  488b4628                       movq	0x28(%rsi), %rax
// Frames: F55 -> F17 -> F91 -> F92 -> F88 -> F132
// 0x1421b4d0b  4889da                         movq	%rbx, %rdx
// 0x1421b4d0e  48c1e206                       shlq	$0x6, %rdx
// Frames: F19 -> F20 -> F91 -> F92 -> F88 -> F132
// 0x1421b4d12  4c893410                       movq	%r14, (%rax,%rdx)
// 0x1421b4d16  48897c1008                     movq	%rdi, 0x8(%rax,%rdx)
// 0x1421b4d1b  896c1010                       movl	%ebp, 0x10(%rax,%rdx)
// 0x1421b4d1f  4c8b442420                     movq	0x20(%rsp), %r8
// 0x1421b4d24  4c89441014                     movq	%r8, 0x14(%rax,%rdx)
// 0x1421b4d29  448b442428                     movl	0x28(%rsp), %r8d
// 0x1421b4d2e  448944101c                     movl	%r8d, 0x1c(%rax,%rdx)
// 0x1421b4d33  0f28442430                     movaps	0x30(%rsp), %xmm0
// 0x1421b4d38  0f11441020                     movups	%xmm0, 0x20(%rax,%rdx)
// 0x1421b4d3d  0f1044243c                     movups	0x3c(%rsp), %xmm0
// 0x1421b4d42  0f1144102c                     movups	%xmm0, 0x2c(%rax,%rdx)
// 0x1421b4d47  c644103c01                     movb	$0x1, 0x3c(%rax,%rdx)
// Frames: F21 -> F91 -> F92 -> F88 -> F132
// 0x1421b4d4c  eb4e                           jmp	0x1421b4d9c <.text+0x21b3d9c>
// Frames: F4 -> F5 -> F133 -> F125
// 0x1421b4d4e  488b7840                       movq	0x40(%rax), %rdi
// 0x1421b4d52  418b411c                       movl	0x1c(%r9), %eax
// 0x1421b4d56  498b5110                       movq	0x10(%r9), %rdx
// 0x1421b4d5a  8902                           movl	%eax, (%rdx)
// Frames: F90 -> F133 -> F125
// 0x1421b4d5c  498b31                         movq	(%r9), %rsi
// Frames: F35 -> F91 -> F92 -> F133 -> F125
// 0x1421b4d5f  488b5e30                       movq	0x30(%rsi), %rbx
// Frames: F93 -> F92 -> F133 -> F125
// 0x1421b4d63  4c8b7640                       movq	0x40(%rsi), %r14
// Frames: F36 -> F91 -> F92 -> F133 -> F125
// 0x1421b4d67  483b5e20                       cmpq	0x20(%rsi), %rbx
// 0x1421b4d6b  7519                           jne	0x1421b4d86 <.text+0x21b3d86>
// Frames: F91 -> F92 -> F133 -> F125
// 0x1421b4d6d  488d4620                       leaq	0x20(%rsi), %rax
// Frames: F37 -> F91 -> F92 -> F133 -> F125
// 0x1421b4d71  488d15f8df9b00                 leaq	0x9bdff8(%rip), %rdx    # 0x142b72d70
// 0x1421b4d78  4989cf                         movq	%rcx, %r15
// 0x1421b4d7b  4889c1                         movq	%rax, %rcx
// 0x1421b4d7e  e82de03afe                     callq	0x140562db0 <.text+0x561db0>
// 0x1421b4d83  4c89f9                         movq	%r15, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F91 -> F92 -> F133 -> F125
// 0x1421b4d86  488b4628                       movq	0x28(%rsi), %rax
// Frames: F55 -> F17 -> F91 -> F92 -> F133 -> F125
// 0x1421b4d8a  488d145b                       leaq	(%rbx,%rbx,2), %rdx
// Frames: F19 -> F20 -> F91 -> F92 -> F133 -> F125
// 0x1421b4d8e  4c8934d0                       movq	%r14, (%rax,%rdx,8)
// 0x1421b4d92  48897cd008                     movq	%rdi, 0x8(%rax,%rdx,8)
// 0x1421b4d97  c644d01001                     movb	$0x1, 0x10(%rax,%rdx,8)
// 0x1421b4d9c  48ffc3                         incq	%rbx
// 0x1421b4d9f  48895e30                       movq	%rbx, 0x30(%rsi)
// 0x1421b4da3  48ff4640                       incq	0x40(%rsi)
// 0x1421b4da7  c6411501                       movb	$0x1, 0x15(%rcx)
// Frames: F134
// 0x1421b4dab  0f28742450                     movaps	0x50(%rsp), %xmm6
// 0x1421b4db0  0f287c2460                     movaps	0x60(%rsp), %xmm7
// 0x1421b4db5  440f28442470                   movaps	0x70(%rsp), %xmm8
// 0x1421b4dbb  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1421b4dc4  440f28942490000000             movaps	0x90(%rsp), %xmm10
// 0x1421b4dcd  4881c4a0000000                 addq	$0xa0, %rsp
// 0x1421b4dd4  5b                             popq	%rbx
// 0x1421b4dd5  5d                             popq	%rbp
// 0x1421b4dd6  5f                             popq	%rdi
// 0x1421b4dd7  5e                             popq	%rsi
// 0x1421b4dd8  415c                           popq	%r12
// 0x1421b4dda  415e                           popq	%r14
// 0x1421b4ddc  415f                           popq	%r15
// 0x1421b4dde  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/replay_roof.rs:23:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F2: crates/systems/roof/src/replay_roof.rs:32:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F3: crates/systems/roof/src/replay_roof.rs:33:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F6: crates/systems/roof/src/replay_roof.rs:35:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F7: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:423:0 alloc::vec::Vec::new
// F8: crates/systems/roof/src/replay_roof.rs:36:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F9: crates/systems/roof/src/replay_roof.rs:38:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F10: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F11: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:242:0 core::slice::iter::IterMut::new
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1070:0 core::slice::impl$0::iter_mut
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F15: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F16: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1734:0 alloc::vec::Vec::as_mut_ptr
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2426:0 alloc::vec::Vec::push
// F18: crates/systems/roof/src/replay_roof.rs:64:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F19: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F20: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2427:0 alloc::vec::Vec::push
// F21: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2428:0 alloc::vec::Vec::push
// F22: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:1623:0 core::ptr::non_null::impl$14::eq
// F23: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$190::next
// F24: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/enumerate.rs:47:0 core::iter::adapters::enumerate::impl$1::next
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$190::next
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:617:0 core::ptr::non_null::NonNull::add
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:108:0 core::slice::iter::IterMut::post_inc_start
// F28: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:443:0 core::slice::iter::impl$194::next_unchecked
// F29: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:165:0 core::slice::iter::impl$190::next
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/enumerate.rs:49:0 core::iter::adapters::enumerate::impl$1::next
// F31: crates/systems/roof/src/replay_roof.rs:39:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F32: crates/systems/roof/src/replay_roof.rs:55:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F33: crates/systems/roof/src/replay_roof.rs:54:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F34: crates/systems/roof/src/replay_roof.rs:63:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2419:0 alloc::vec::Vec::push
// F36: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2422:0 alloc::vec::Vec::push
// F37: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2423:0 alloc::vec::Vec::push
// F38: crates/systems/roof/src/replay_roof.rs:70:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1025:0 core::slice::impl$0::reverse::revswap
// F40: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1010:0 core::slice::impl$0::reverse
// F41: crates/systems/roof/src/replay_roof.rs:72:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F42: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/into_iter.rs:218:0 alloc::vec::into_iter::impl$5::next
// F43: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1026:0 core::slice::impl$0::reverse::revswap
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:733:0 core::mem::swap
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1027:0 core::slice::impl$0::reverse::revswap
// F46: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:523:0 core::ptr::drop_in_place
// F47: crates/systems/roof/src/replay_roof.rs:73:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1380:0 core::ptr::read
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:950:0 core::ptr::non_null::NonNull::read
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/into_iter.rs:225:0 alloc::vec::into_iter::impl$5::next
// F51: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2681:0 alloc::vec::Vec::len
// F52: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2049:0 alloc::vec::Vec::remove
// F53: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2050:0 alloc::vec::Vec::remove
// F54: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2058:0 alloc::vec::Vec::remove
// F55: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 alloc::vec::Vec::as_mut_ptr
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2061:0 alloc::vec::Vec::remove
// F57: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2064:0 alloc::vec::Vec::remove
// F58: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:4543:0 core::ptr::mut_ptr::impl$0::add
// F59: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2066:0 alloc::vec::Vec::remove
// F60: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1898:0 alloc::vec::Vec::set_len
// F61: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:429:0 core::ptr::non_null::NonNull::as_ref
// F62: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:1894:0 alloc::sync::Arc::inner
// F63: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:2554:0 alloc::sync::impl$37::drop
// F64: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3783:0 core::sync::atomic::atomic_sub
// F65: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2988:0 alloc::sync::Arc::inner
// F66: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:2597:0 alloc::sync::impl$37::drop
// F67: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:523:0 alloc::raw_vec::RawVecInner::current_memory
// F68: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:751:0 alloc::raw_vec::RawVecInner::deallocate
// F69: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:402:0 alloc::raw_vec::impl$3::drop
// F70: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/into_iter.rs:496:0 alloc::vec::into_iter::impl$15::drop::impl$0::drop
// F71: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/into_iter.rs:507:0 alloc::vec::into_iter::impl$15::drop
// F72: crates/systems/roof/src/replay_roof.rs:74:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F73: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:945:0 core::num::impl$11::unchecked_mul
// F74: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:531:0 alloc::raw_vec::RawVecInner::current_memory
// F75: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:116:0 alloc::alloc::dealloc
// F76: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:269:0 alloc::alloc::impl$1::deallocate
// F77: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:753:0 alloc::raw_vec::RawVecInner::deallocate
// F78: crates/systems/roof/src/replay_roof.rs:76:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F79: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2051:0 alloc::vec::Vec::remove
// F80: crates/systems/roof/src/replay_roof.rs:75:0 system_roof::replay_roof::replay_roof::ha83abe60a6b01145
// F81: crates/systems/roof/src/replay_roof.rs:80:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F82: crates/systems/roof/src/replay_roof.rs:92:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F83: crates/systems/roof/src/replay_roof.rs:198:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F84: crates/systems/roof/src/replay_roof.rs:202:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F85: crates/systems/roof/src/replay_roof.rs:203:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F86: crates/systems/roof/src/replay_roof.rs:204:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F87: crates/systems/roof/src/replay_roof.rs:205:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs:75:0 bevy_ecs::event::writer::EventWriter::write
// F89: crates/systems/roof/src/replay_roof.rs:200:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:437:0 bevy_ecs::change_detection::impl$38::deref_mut
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:139:0 bevy_ecs::event::collections::Events::send_with_caller
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::change_detection::impl$38::deref_mut
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:130:0 bevy_ecs::event::collections::Events::send_with_caller
// F94: crates/systems/roof/src/replay_roof.rs:223:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F95: crates/systems/roof/src/replay_roof.rs:224:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F96: crates/systems/roof/src/replay_roof.rs:229:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F97: crates/systems/roof/src/replay_roof.rs:228:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:140:0 bevy_ecs::event::collections::Events::send_with_caller
// F99: crates/systems/roof/src/replay_roof.rs:160:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F100: crates/systems/roof/src/replay_roof.rs:163:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F101: crates/systems/roof/src/replay_roof.rs:164:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F102: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1418:0 core::f32::impl$0::clamp
// F103: crates/systems/roof/src/replay_roof.rs:169:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F104: crates/systems/roof/src/replay_roof.rs:173:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F105: crates/systems/roof/src/replay_roof.rs:180:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F106: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$77::sub
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F108: crates/systems/roof/src/replay_roof.rs:182:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F109: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:344:0 core::ops::arith::impl$141::mul
// F110: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F111: crates/systems/roof/src/replay_roof.rs:183:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F112: crates/systems/roof/src/replay_roof.rs:185:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F113: crates/systems/roof/src/replay_roof.rs:186:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1200:0 glam::f32::vec2::impl$24::mul
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:694:0 glam::f32::vec2::Vec2::lerp
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1285:0 glam::f32::vec2::impl$34::add
// F117: crates/systems/roof/src/replay_roof.rs:188:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F118: crates/systems/roof/src/replay_roof.rs:189:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F119: crates/systems/roof/src/replay_roof.rs:179:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F120: crates/systems/roof/src/replay_roof.rs:191:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F121: crates/systems/roof/src/replay_roof.rs:124:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F122: crates/systems/roof/src/replay_roof.rs:125:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F123: crates/systems/roof/src/replay_roof.rs:130:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F124: crates/systems/roof/src/replay_roof.rs:129:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F125: crates/systems/roof/src/replay_roof.rs:0:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F126: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1421:0 core::f32::impl$0::clamp
// F127: crates/systems/roof/src/replay_roof.rs:99:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F128: crates/systems/roof/src/replay_roof.rs:109:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F129: crates/systems/roof/src/replay_roof.rs:110:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F130: crates/systems/roof/src/replay_roof.rs:111:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F131: crates/systems/roof/src/replay_roof.rs:112:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F132: crates/systems/roof/src/replay_roof.rs:107:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs:75:0 core::f32::impl$0::clamp
// F134: crates/systems/roof/src/replay_roof.rs:249:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::hc22b5d0af2fc7105
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/replay_roof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a172b4a4a243b785fb4267400f172089fc07d8aa18fc7846c5707fdb63e225ef
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2277, 3476
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::replay_roof::impl$0::process
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3476,"record_offset":260,"rva":35342560,"va":5404051680}
// 编译实例 RVAs: 0x21b48e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-29696479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404051680,"raw_hex":"e821de3afe","size":5,"soft_fail":false,"target_va":5374355376,"va":5404051850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3476,"record_offset":400,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3476,"record_offset":1056,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3476,"record_offset":1336,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3476,"record_offset":1084,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3476,"record_offset":592,"parent_record_offset":560,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3476,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3476,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3476,"record_offset":560,"parent_record_offset":516,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3476,"record_offset":516,"parent_record_offset":484,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3476,"record_offset":456,"parent_record_offset":424,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3476,"record_offset":424,"parent_record_offset":400,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3476,"record_offset":484,"parent_record_offset":424,"ancestor_record_offset":260,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3476,"record_offset":1192,"parent_record_offset":1084,"ancestor_record_offset":260,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3476,"record_offset":1136,"parent_record_offset":1084,"ancestor_record_offset":260,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3476,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":260,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3476,"record_offset":680,"parent_record_offset":516,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3476,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":260,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3476,"record_offset":1444,"parent_record_offset":1336,"ancestor_record_offset":260,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::replay_roof::replay_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2277,"record_offset":260,"rva":27491024,"va":5396200144}
// 编译实例 RVAs: 0x1a37ad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24453227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e895df8afe","size":5,"soft_fail":false,"target_va":5371747712,"va":5396200934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21119684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e83cbdbdfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5396201183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406978e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20579784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e838fac5fe","size":5,"soft_fail":false,"target_va":5375621344,"va":5396201123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140891eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18505343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e881a1e5fe","size":5,"soft_fail":false,"target_va":5377695408,"va":5396200746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140949170
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17754527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e86116f1fe","size":5,"soft_fail":false,"target_va":5378445680,"va":5396200202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15580803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e87d4112ff","size":5,"soft_fail":false,"target_va":5380620352,"va":5396201150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15580033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e87f4412ff","size":5,"soft_fail":false,"target_va":5380620960,"va":5396200988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15577394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e8ce4e12ff","size":5,"soft_fail":false,"target_va":5380623680,"va":5396201069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cfc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15576592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e8f05112ff","size":5,"soft_fail":false,"target_va":5380624320,"va":5396200907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15576620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e8d45112ff","size":5,"soft_fail":false,"target_va":5380624400,"va":5396201015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421b48e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7851270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e806cd7700","size":5,"soft_fail":false,"target_va":5404051680,"va":5396200405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15646324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e874beee00","size":5,"soft_fail":false,"target_va":5411847184,"va":5396200855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15425038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e8f2a114ff","size":5,"soft_fail":false,"target_va":5380775408,"va":5396200441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15339998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e8de11ea00","size":5,"soft_fail":false,"target_va":5411541225,"va":5396201222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17064102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396200144,"raw_hex":"e85a9ffbfe","size":5,"soft_fail":false,"target_va":5379136128,"va":5396200225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2277,"record_offset":472,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2277,"record_offset":584,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2277,"record_offset":3408,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2277,"record_offset":1084,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2277,"record_offset":416,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2277,"record_offset":768,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2277,"record_offset":1216,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2277,"record_offset":500,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2277,"record_offset":972,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2277,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2277,"record_offset":3936,"parent_record_offset":3848,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2277,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2277,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2277,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2277,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2277,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2277,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2277,"record_offset":1328,"parent_record_offset":1292,"ancestor_record_offset":260,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2277,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":260,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2277,"record_offset":624,"parent_record_offset":584,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2277,"record_offset":3456,"parent_record_offset":3408,"ancestor_record_offset":260,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2277,"record_offset":3672,"parent_record_offset":3408,"ancestor_record_offset":260,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2277,"record_offset":3728,"parent_record_offset":3704,"ancestor_record_offset":260,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2277,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":260,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2277,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2277,"record_offset":1044,"parent_record_offset":1000,"ancestor_record_offset":260,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2277,"record_offset":3904,"parent_record_offset":3876,"ancestor_record_offset":260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2277,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2277,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2277,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2277,"record_offset":1360,"parent_record_offset":1328,"ancestor_record_offset":260,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2277,"record_offset":1156,"parent_record_offset":1084,"ancestor_record_offset":260,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2277,"record_offset":848,"parent_record_offset":804,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2277,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":260,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2277,"record_offset":736,"parent_record_offset":584,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2277,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":260,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2277,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":260,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2277,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":260,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2277,"record_offset":804,"parent_record_offset":768,"ancestor_record_offset":260,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2277,"record_offset":880,"parent_record_offset":804,"ancestor_record_offset":260,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2277,"record_offset":1388,"parent_record_offset":1328,"ancestor_record_offset":260,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/02277.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2277,"record_offset":1420,"parent_record_offset":1292,"ancestor_record_offset":260,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/02277.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-history-owners/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/replay_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/replay_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/replay_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>

// ------------------------------------------------------------------
// 函数: system_roof::replay_roof::impl$0::process
// 信息 ID: 6258971009696ce845fa0049
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::replay_roof::replay_roof @ 0x1a37bd5 -> 0x21b48e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::replay_roof::replay_roof
// 信息 ID: a479fc19178c1dc42e6951b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] add80410eca410d20a536794: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)> > @ 0xde0966 -> 0x1a37ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)> > > @ 0x1a2a998 -> 0x1a37ad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
