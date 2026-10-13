// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: replay_roof.rs.assembly.json.gz

// FUNCTION system_roof::replay_roof::replay_roof
// Candidate interfaces: 5deebb0d68fbc98334793912
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 2446:268 -> rva_1f3efa0_len_54d
//
// FUNCTION system_roof::replay_roof::impl$0::process
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3787:268 -> rva_28310f0_len_503
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1f3efa0_len_54d: RVA 0x1f3efa0, 1357 PDB bytes; 371 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f3efa0_len_54d.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2446:268: system_roof::replay_roof::replay_roof
// Frames: F1
// 0x141f3efa0  55                             pushq	%rbp
// 0x141f3efa1  4157                           pushq	%r15
// 0x141f3efa3  4156                           pushq	%r14
// 0x141f3efa5  4155                           pushq	%r13
// 0x141f3efa7  4154                           pushq	%r12
// 0x141f3efa9  56                             pushq	%rsi
// 0x141f3efaa  57                             pushq	%rdi
// 0x141f3efab  53                             pushq	%rbx
// 0x141f3efac  4881ecf8000000                 subq	$0xf8, %rsp
// 0x141f3efb3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x141f3efbb  0f297560                       movaps	%xmm6, 0x60(%rbp)
// 0x141f3efbf  48c74558feffffff               movq	$-0x2, 0x58(%rbp)
// 0x141f3efc7  4c894d50                       movq	%r9, 0x50(%rbp)
// 0x141f3efcb  4c89c3                         movq	%r8, %rbx
// 0x141f3efce  4889d7                         movq	%rdx, %rdi
// 0x141f3efd1  4889ce                         movq	%rcx, %rsi
// Frames: F2
// 0x141f3efd4  4c8b39                         movq	(%rcx), %r15
// 0x141f3efd7  4c89f9                         movq	%r15, %rcx
// 0x141f3efda  e851a3b1fe                     callq	0x140a59330 <.text+0xa58330>
// Frames: F3
// 0x141f3efdf  f3410f107728                   movss	0x28(%r15), %xmm6
// Frames: F4 -> F5 -> F6
// 0x141f3efe5  8b461c                         movl	0x1c(%rsi), %eax
// 0x141f3efe8  488b4e10                       movq	0x10(%rsi), %rcx
// 0x141f3efec  8901                           movl	%eax, (%rcx)
// Frames: F6
// 0x141f3efee  4c89f9                         movq	%r15, %rcx
// 0x141f3eff1  e8fa54c3fe                     callq	0x140b744f0 <.text+0xb734f0>
// 0x141f3eff6  4885c0                         testq	%rax, %rax
// 0x141f3eff9  0f8402040000                   je	0x141f3f401 <.text+0x1f3e401>
// Frames: F7 -> F8
// 0x141f3efff  4889c6                         movq	%rax, %rsi
// 0x141f3f002  48c7453800000000               movq	$0x0, 0x38(%rbp)
// 0x141f3f00a  48c7454008000000               movq	$0x8, 0x40(%rbp)
// 0x141f3f012  48c7454800000000               movq	$0x0, 0x48(%rbp)
// Frames: F9
// 0x141f3f01a  4c8b7008                       movq	0x8(%rax), %r14
// 0x141f3f01e  488b4010                       movq	0x10(%rax), %rax
// Frames: F10 -> F11 -> F12 -> F9
// 0x141f3f022  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141f3f026  48c1e004                       shlq	$0x4, %rax
// 0x141f3f02a  4c01f0                         addq	%r14, %rax
// 0x141f3f02d  488945f0                       movq	%rax, -0x10(%rbp)
// 0x141f3f031  4c8b3f                         movq	(%rdi), %r15
// 0x141f3f034  4531ed                         xorl	%r13d, %r13d
// 0x141f3f037  eb16                           jmp	0x141f3f04f <.text+0x1f3e04f>
// 0x141f3f039  0f1f8000000000                 nopl	(%rax)
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F19
// 0x141f3f040  488b4540                       movq	0x40(%rbp), %rax
// Frames: F20 -> F21 -> F18 -> F19
// 0x141f3f044  4c8924f8                       movq	%r12, (%rax,%rdi,8)
// Frames: F22 -> F18 -> F19
// 0x141f3f048  48ffc7                         incq	%rdi
// 0x141f3f04b  48897d48                       movq	%rdi, 0x48(%rbp)
// Frames: F9
// 0x141f3f04f  4c89f0                         movq	%r14, %rax
// Frames: F23 -> F24 -> F25 -> F9
// 0x141f3f052  4c3b75f0                       cmpq	-0x10(%rbp), %r14
// Frames: F24 -> F25 -> F9
// 0x141f3f056  7475                           je	0x141f3f0cd <.text+0x1f3e0cd>
// Frames: F9
// 0x141f3f058  4d89ec                         movq	%r13, %r12
// Frames: F26 -> F27 -> F25 -> F9
// 0x141f3f05b  4c8d7030                       leaq	0x30(%rax), %r14
// Frames: F28 -> F9
// 0x141f3f05f  49ffc5                         incq	%r13
// Frames: F29
// 0x141f3f062  803805                         cmpb	$0x5, (%rax)
// 0x141f3f065  75e8                           jne	0x141f3f04f <.text+0x1f3e04f>
// 0x141f3f067  48894528                       movq	%rax, 0x28(%rbp)
// 0x141f3f06b  488d4808                       leaq	0x8(%rax), %rcx
// 0x141f3f06f  f3410f104f28                   movss	0x28(%r15), %xmm1
// Frames: F30
// 0x141f3f075  f30f59ce                       mulss	%xmm6, %xmm1
// Frames: F31
// 0x141f3f079  488b85f0000000                 movq	0xf0(%rbp), %rax
// 0x141f3f080  4889442430                     movq	%rax, 0x30(%rsp)
// 0x141f3f085  488b85e8000000                 movq	0xe8(%rbp), %rax
// 0x141f3f08c  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141f3f091  488b85e0000000                 movq	0xe0(%rbp), %rax
// 0x141f3f098  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3f09d  4989d8                         movq	%rbx, %r8
// 0x141f3f0a0  4c8b4d50                       movq	0x50(%rbp), %r9
// 0x141f3f0a4  e847208f00                     callq	0x1428310f0 <.text+0x28300f0>
// 0x141f3f0a9  90                             nop
// Frames: F32
// 0x141f3f0aa  488b4528                       movq	0x28(%rbp), %rax
// 0x141f3f0ae  80781d01                       cmpb	$0x1, 0x1d(%rax)
// 0x141f3f0b2  759b                           jne	0x141f3f04f <.text+0x1f3e04f>
// Frames: F33 -> F18 -> F19
// 0x141f3f0b4  488b7d48                       movq	0x48(%rbp), %rdi
// Frames: F34 -> F18 -> F19
// 0x141f3f0b8  483b7d38                       cmpq	0x38(%rbp), %rdi
// 0x141f3f0bc  7582                           jne	0x141f3f040 <.text+0x1f3e040>
// Frames: F35 -> F18 -> F19
// 0x141f3f0be  488d4d38                       leaq	0x38(%rbp), %rcx
// 0x141f3f0c2  e8c93ee300                     callq	0x142d72f90 <.text+0x2d71f90>
// 0x141f3f0c7  90                             nop
// 0x141f3f0c8  e973ffffff                     jmp	0x141f3f040 <.text+0x1f3e040>
// Frames: F36
// 0x141f3f0cd  48894528                       movq	%rax, 0x28(%rbp)
// 0x141f3f0d1  488b7d40                       movq	0x40(%rbp), %rdi
// 0x141f3f0d5  488b4d48                       movq	0x48(%rbp), %rcx
// Frames: F37 -> F36
// 0x141f3f0d9  4889c8                         movq	%rcx, %rax
// 0x141f3f0dc  48d1e8                         shrq	%rax
// Frames: F38 -> F39 -> F36
// 0x141f3f0df  740d                           je	0x141f3f0ee <.text+0x1f3e0ee>
// Frames: F40 -> F39 -> F36
// 0x141f3f0e1  4883f908                       cmpq	$0x8, %rcx
// 0x141f3f0e5  7321                           jae	0x141f3f108 <.text+0x1f3e108>
// 0x141f3f0e7  31d2                           xorl	%edx, %edx
// 0x141f3f0e9  e998000000                     jmp	0x141f3f186 <.text+0x1f3e186>
// Frames: F41
// 0x141f3f0ee  48897d50                       movq	%rdi, 0x50(%rbp)
// 0x141f3f0f2  488b4538                       movq	0x38(%rbp), %rax
// 0x141f3f0f6  48894530                       movq	%rax, 0x30(%rbp)
// Frames: F23 -> F42 -> F41
// 0x141f3f0fa  4885c9                         testq	%rcx, %rcx
// Frames: F42 -> F41
// 0x141f3f0fd  0f85d4000000                   jne	0x141f3f1d7 <.text+0x1f3e1d7>
// 0x141f3f103  e9dd020000                     jmp	0x141f3f3e5 <.text+0x1f3e3e5>
// Frames: F40 -> F39 -> F36
// 0x141f3f108  4889c2                         movq	%rax, %rdx
// 0x141f3f10b  4883e2fc                       andq	$-0x4, %rdx
// 0x141f3f10f  4989c0                         movq	%rax, %r8
// 0x141f3f112  4983e0fc                       andq	$-0x4, %r8
// 0x141f3f116  49f7d8                         negq	%r8
// 0x141f3f119  4c8d4f10                       leaq	0x10(%rdi), %r9
// 0x141f3f11d  4c8d14cf                       leaq	(%rdi,%rcx,8), %r10
// 0x141f3f121  4983c2f0                       addq	$-0x10, %r10
// 0x141f3f125  4531db                         xorl	%r11d, %r11d
// 0x141f3f128  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F43 -> F40 -> F39 -> F36
// 0x141f3f130  f3410f6f41f0                   movdqu	-0x10(%r9), %xmm0
// 0x141f3f136  f3410f6f09                     movdqu	(%r9), %xmm1
// 0x141f3f13b  f3430f6f54daf0                 movdqu	-0x10(%r10,%r11,8), %xmm2
// 0x141f3f142  f3430f6f1cda                   movdqu	(%r10,%r11,8), %xmm3
// 0x141f3f148  660f70db4e                     pshufd	$0x4e, %xmm3, %xmm3     # xmm3 = xmm3[2,3,0,1]
// 0x141f3f14d  660f70d24e                     pshufd	$0x4e, %xmm2, %xmm2     # xmm2 = xmm2[2,3,0,1]
// 0x141f3f152  f3410f7f59f0                   movdqu	%xmm3, -0x10(%r9)
// 0x141f3f158  f3410f7f11                     movdqu	%xmm2, (%r9)
// 0x141f3f15d  660f70c04e                     pshufd	$0x4e, %xmm0, %xmm0     # xmm0 = xmm0[2,3,0,1]
// 0x141f3f162  660f70c94e                     pshufd	$0x4e, %xmm1, %xmm1     # xmm1 = xmm1[2,3,0,1]
// 0x141f3f167  f3430f7f04da                   movdqu	%xmm0, (%r10,%r11,8)
// 0x141f3f16d  f3430f7f4cdaf0                 movdqu	%xmm1, -0x10(%r10,%r11,8)
// Frames: F38 -> F39 -> F36
// 0x141f3f174  4983c3fc                       addq	$-0x4, %r11
// 0x141f3f178  4983c120                       addq	$0x20, %r9
// 0x141f3f17c  4d39d8                         cmpq	%r11, %r8
// 0x141f3f17f  75af                           jne	0x141f3f130 <.text+0x1f3e130>
// 0x141f3f181  4839d0                         cmpq	%rdx, %rax
// 0x141f3f184  7434                           je	0x141f3f1ba <.text+0x1f3e1ba>
// 0x141f3f186  48f7d8                         negq	%rax
// 0x141f3f189  4c8d04d7                       leaq	(%rdi,%rdx,8), %r8
// 0x141f3f18d  48f7da                         negq	%rdx
// 0x141f3f190  488d0ccf                       leaq	(%rdi,%rcx,8), %rcx
// 0x141f3f194  4883c1f8                       addq	$-0x8, %rcx
// 0x141f3f198  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F43 -> F40 -> F39 -> F36
// 0x141f3f1a0  4d8b08                         movq	(%r8), %r9
// 0x141f3f1a3  4c8b14d1                       movq	(%rcx,%rdx,8), %r10
// 0x141f3f1a7  4d8910                         movq	%r10, (%r8)
// 0x141f3f1aa  4c890cd1                       movq	%r9, (%rcx,%rdx,8)
// Frames: F38 -> F39 -> F36
// 0x141f3f1ae  48ffca                         decq	%rdx
// 0x141f3f1b1  4983c008                       addq	$0x8, %r8
// 0x141f3f1b5  4839d0                         cmpq	%rdx, %rax
// 0x141f3f1b8  75e6                           jne	0x141f3f1a0 <.text+0x1f3e1a0>
// Frames: F41
// 0x141f3f1ba  488b4540                       movq	0x40(%rbp), %rax
// 0x141f3f1be  48894550                       movq	%rax, 0x50(%rbp)
// 0x141f3f1c2  488b4d48                       movq	0x48(%rbp), %rcx
// 0x141f3f1c6  488b4538                       movq	0x38(%rbp), %rax
// 0x141f3f1ca  48894530                       movq	%rax, 0x30(%rbp)
// Frames: F23 -> F42 -> F41
// 0x141f3f1ce  4885c9                         testq	%rcx, %rcx
// Frames: F42 -> F41
// 0x141f3f1d1  0f840e020000                   je	0x141f3f3e5 <.text+0x1f3e3e5>
// 0x141f3f1d7  488b5d50                       movq	0x50(%rbp), %rbx
// 0x141f3f1db  4c8d24cb                       leaq	(%rbx,%rcx,8), %r12
// 0x141f3f1df  488d7d00                       leaq	(%rbp), %rdi
// 0x141f3f1e3  eb18                           jmp	0x141f3f1fd <.text+0x1f3e1fd>
// Frames: F44 -> F45
// 0x141f3f1e5  4889f9                         movq	%rdi, %rcx
// 0x141f3f1e8  e8c305a6fe                     callq	0x14099f7b0 <.text+0x99e7b0>
// 0x141f3f1ed  90                             nop
// 0x141f3f1ee  6690                           nop
// Frames: F23 -> F42 -> F41
// 0x141f3f1f0  4883c308                       addq	$0x8, %rbx
// 0x141f3f1f4  4c39e3                         cmpq	%r12, %rbx
// Frames: F42 -> F41
// 0x141f3f1f7  0f84e8010000                   je	0x141f3f3e5 <.text+0x1f3e3e5>
// Frames: F46 -> F47 -> F48 -> F41
// 0x141f3f1fd  4c8b33                         movq	(%rbx), %r14
// Frames: F49 -> F50 -> F51 -> F45
// 0x141f3f200  4c8b7e10                       movq	0x10(%rsi), %r15
// Frames: F52 -> F51 -> F45
// 0x141f3f204  4d39fe                         cmpq	%r15, %r14
// 0x141f3f207  0f830c020000                   jae	0x141f3f419 <.text+0x1f3e419>
// Frames: F13 -> F14 -> F15 -> F16 -> F53 -> F51 -> F45
// 0x141f3f20d  488b4608                       movq	0x8(%rsi), %rax
// Frames: F54 -> F53 -> F51 -> F45
// 0x141f3f211  4b8d1476                       leaq	(%r14,%r14,2), %rdx
// 0x141f3f215  48c1e204                       shlq	$0x4, %rdx
// 0x141f3f219  488d0c10                       leaq	(%rax,%rdx), %rcx
// Frames: F46 -> F55 -> F51 -> F45
// 0x141f3f21d  440fb62c10                     movzbl	(%rax,%rdx), %r13d
// 0x141f3f222  0f10441001                     movups	0x1(%rax,%rdx), %xmm0
// 0x141f3f227  0f104c1011                     movups	0x11(%rax,%rdx), %xmm1
// 0x141f3f22c  f30f6f541020                   movdqu	0x20(%rax,%rdx), %xmm2
// 0x141f3f232  f30f7f55df                     movdqu	%xmm2, -0x21(%rbp)
// 0x141f3f237  0f294dd0                       movaps	%xmm1, -0x30(%rbp)
// 0x141f3f23b  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// Frames: F10 -> F56 -> F51 -> F45
// 0x141f3f23f  4801c2                         addq	%rax, %rdx
// 0x141f3f242  4883c230                       addq	$0x30, %rdx
// Frames: F56 -> F51 -> F45
// 0x141f3f246  4c89f0                         movq	%r14, %rax
// 0x141f3f249  48f7d0                         notq	%rax
// 0x141f3f24c  4c01f8                         addq	%r15, %rax
// Frames: F57 -> F56 -> F51 -> F45
// 0x141f3f24f  48c1e004                       shlq	$0x4, %rax
// 0x141f3f253  4c8d0440                       leaq	(%rax,%rax,2), %r8
// 0x141f3f257  e8d4c8ed00                     callq	0x142e1bb30 <.text+0x2e1ab30>
// Frames: F58 -> F51 -> F45
// 0x141f3f25c  49ffcf                         decq	%r15
// Frames: F59 -> F58 -> F51 -> F45
// 0x141f3f25f  4c897e10                       movq	%r15, 0x10(%rsi)
// Frames: F46 -> F55 -> F51 -> F45
// 0x141f3f263  4183fd0c                       cmpl	$0xc, %r13d
// Frames: F51 -> F45
// 0x141f3f267  0f84ac010000                   je	0x141f3f419 <.text+0x1f3e419>
// Frames: F60 -> F45
// 0x141f3f26d  44886df8                       movb	%r13b, -0x8(%rbp)
// 0x141f3f271  0f1045df                       movups	-0x21(%rbp), %xmm0
// 0x141f3f275  0f114718                       movups	%xmm0, 0x18(%rdi)
// 0x141f3f279  660f6f45c0                     movdqa	-0x40(%rbp), %xmm0
// 0x141f3f27e  0f284dd0                       movaps	-0x30(%rbp), %xmm1
// 0x141f3f282  0f114f09                       movups	%xmm1, 0x9(%rdi)
// 0x141f3f286  f30f7f47f9                     movdqu	%xmm0, -0x7(%rdi)
// Frames: F44 -> F45
// 0x141f3f28b  4183fd0a                       cmpl	$0xa, %r13d
// 0x141f3f28f  0f8734010000                   ja	0x141f3f3c9 <.text+0x1f3e3c9>
// 0x141f3f295  4489e8                         movl	%r13d, %eax
// 0x141f3f298  488d0d81a54d01                 leaq	0x14da581(%rip), %rcx   # 0x143419820
// 0x141f3f29f  48630481                       movslq	(%rcx,%rax,4), %rax
// 0x141f3f2a3  4801c8                         addq	%rcx, %rax
// 0x141f3f2a6  ffe0                           jmpq	*%rax
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2a8  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2ac  f0                             lock
// 0x141f3f2ad  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2b0  0f853affffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2b6  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f2ba  e8912fd5fe                     callq	0x140c92250 <.text+0xc91250>
// 0x141f3f2bf  90                             nop
// 0x141f3f2c0  e92bffffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2c5  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2c9  f0                             lock
// 0x141f3f2ca  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2cd  0f851dffffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2d3  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f2d7  e804864ffe                     callq	0x1404378e0 <.text+0x4368e0>
// 0x141f3f2dc  90                             nop
// 0x141f3f2dd  e90effffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2e2  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2e6  f0                             lock
// 0x141f3f2e7  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2ea  0f8500ffffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2f0  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f2f4  e8572fd5fe                     callq	0x140c92250 <.text+0xc91250>
// 0x141f3f2f9  90                             nop
// 0x141f3f2fa  e9f1feffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f2ff  488b4500                       movq	(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f303  f0                             lock
// 0x141f3f304  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f307  0f85e3feffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f30d  4889f9                         movq	%rdi, %rcx
// 0x141f3f310  e8ab23d5fe                     callq	0x140c916c0 <.text+0xc906c0>
// 0x141f3f315  90                             nop
// 0x141f3f316  e9d5feffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f31b  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f31f  f0                             lock
// 0x141f3f320  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f323  0f85c7feffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f329  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f32d  e8be2cd5fe                     callq	0x140c91ff0 <.text+0xc90ff0>
// 0x141f3f332  90                             nop
// 0x141f3f333  e9b8feffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f338  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f33c  f0                             lock
// 0x141f3f33d  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f340  0f85aafeffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f346  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f34a  e8012fd5fe                     callq	0x140c92250 <.text+0xc91250>
// 0x141f3f34f  90                             nop
// 0x141f3f350  e99bfeffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f355  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f359  f0                             lock
// 0x141f3f35a  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f35d  0f858dfeffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f363  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f367  e81415d5fe                     callq	0x140c90880 <.text+0xc8f880>
// 0x141f3f36c  90                             nop
// 0x141f3f36d  e97efeffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f372  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f376  f0                             lock
// 0x141f3f377  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f37a  0f8570feffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f380  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f384  e8672cd5fe                     callq	0x140c91ff0 <.text+0xc90ff0>
// 0x141f3f389  90                             nop
// 0x141f3f38a  e961feffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f38f  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f393  f0                             lock
// 0x141f3f394  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f397  0f8553feffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f39d  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f3a1  e8fab87ffe                     callq	0x14073aca0 <.text+0x739ca0>
// 0x141f3f3a6  90                             nop
// 0x141f3f3a7  e944feffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f3ac  488b4508                       movq	0x8(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f3b0  f0                             lock
// 0x141f3f3b1  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f3b4  0f8536feffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f3ba  488d4d08                       leaq	0x8(%rbp), %rcx
// 0x141f3f3be  e89d22d5fe                     callq	0x140c91660 <.text+0xc90660>
// 0x141f3f3c3  90                             nop
// 0x141f3f3c4  e927feffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F61 -> F62 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f3c9  488b4500                       movq	(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f3cd  f0                             lock
// 0x141f3f3ce  48ff08                         decq	(%rax)
// Frames: F63 -> F44 -> F44 -> F44 -> F45
// 0x141f3f3d1  0f8519feffff                   jne	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F66 -> F44 -> F44 -> F44 -> F45
// 0x141f3f3d7  4889f9                         movq	%rdi, %rcx
// 0x141f3f3da  e8e124d5fe                     callq	0x140c918c0 <.text+0xc908c0>
// 0x141f3f3df  90                             nop
// 0x141f3f3e0  e90bfeffff                     jmp	0x141f3f1f0 <.text+0x1f3e1f0>
// Frames: F67 -> F68 -> F69 -> F44 -> F70 -> F44 -> F71 -> F44 -> F72
// 0x141f3f3e5  488b5530                       movq	0x30(%rbp), %rdx
// 0x141f3f3e9  4885d2                         testq	%rdx, %rdx
// 0x141f3f3ec  7413                           je	0x141f3f401 <.text+0x1f3e401>
// Frames: F73 -> F74 -> F68 -> F69 -> F44 -> F70 -> F44 -> F71 -> F44 -> F72
// 0x141f3f3ee  48c1e203                       shlq	$0x3, %rdx
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F69 -> F44 -> F70 -> F44 -> F71 -> F44 -> F72
// 0x141f3f3f2  41b808000000                   movl	$0x8, %r8d
// 0x141f3f3f8  488b4d50                       movq	0x50(%rbp), %rcx
// 0x141f3f3fc  e8ef6476fe                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F80
// 0x141f3f401  0f287560                       movaps	0x60(%rbp), %xmm6
// 0x141f3f405  4881c4f8000000                 addq	$0xf8, %rsp
// 0x141f3f40c  5b                             popq	%rbx
// 0x141f3f40d  5f                             popq	%rdi
// 0x141f3f40e  5e                             popq	%rsi
// 0x141f3f40f  415c                           popq	%r12
// 0x141f3f411  415d                           popq	%r13
// 0x141f3f413  415e                           popq	%r14
// 0x141f3f415  415f                           popq	%r15
// 0x141f3f417  5d                             popq	%rbp
// 0x141f3f418  c3                             retq
// Frames: F81 -> F45
// 0x141f3f419  4c8d05e8a34d01                 leaq	0x14da3e8(%rip), %r8    # 0x143419808
// 0x141f3f420  4c89f1                         movq	%r14, %rcx
// 0x141f3f423  4c89fa                         movq	%r15, %rdx
// 0x141f3f426  e89d1fe700                     callq	0x142db13c8 <.text+0x2db03c8>
// 0x141f3f42b  90                             nop
// 0x141f3f42c  0f0b                           ud2
// 0x141f3f42e  6690                           nop
// 0x141f3f430  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f3f435  55                             pushq	%rbp
// 0x141f3f436  4157                           pushq	%r15
// 0x141f3f438  4156                           pushq	%r14
// 0x141f3f43a  4155                           pushq	%r13
// 0x141f3f43c  4154                           pushq	%r12
// 0x141f3f43e  56                             pushq	%rsi
// 0x141f3f43f  57                             pushq	%rdi
// 0x141f3f440  53                             pushq	%rbx
// 0x141f3f441  4883ec48                       subq	$0x48, %rsp
// 0x141f3f445  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3f44c  0f29742430                     movaps	%xmm6, 0x30(%rsp)
// Frames: F67 -> F68 -> F69 -> F44 -> F70 -> F44 -> F71 -> F44 -> F72
// 0x141f3f451  48837d3000                     cmpq	$0x0, 0x30(%rbp)
// 0x141f3f456  7417                           je	0x141f3f46f <.text+0x1f3e46f>
// Frames: F73 -> F74 -> F68 -> F69 -> F44 -> F70 -> F44 -> F71 -> F44 -> F72
// 0x141f3f458  488b5530                       movq	0x30(%rbp), %rdx
// 0x141f3f45c  48c1e203                       shlq	$0x3, %rdx
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F69 -> F44 -> F70 -> F44 -> F71 -> F44 -> F72
// 0x141f3f460  41b808000000                   movl	$0x8, %r8d
// 0x141f3f466  488b4d50                       movq	0x50(%rbp), %rcx
// 0x141f3f46a  e8816476fe                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F72
// 0x141f3f46f  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141f3f474  4883c448                       addq	$0x48, %rsp
// 0x141f3f478  5b                             popq	%rbx
// 0x141f3f479  5f                             popq	%rdi
// 0x141f3f47a  5e                             popq	%rsi
// 0x141f3f47b  415c                           popq	%r12
// 0x141f3f47d  415d                           popq	%r13
// 0x141f3f47f  415e                           popq	%r14
// 0x141f3f481  415f                           popq	%r15
// 0x141f3f483  5d                             popq	%rbp
// 0x141f3f484  c3                             retq
// 0x141f3f485  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x141f3f490  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f3f495  55                             pushq	%rbp
// 0x141f3f496  4157                           pushq	%r15
// 0x141f3f498  4156                           pushq	%r14
// 0x141f3f49a  4155                           pushq	%r13
// 0x141f3f49c  4154                           pushq	%r12
// 0x141f3f49e  56                             pushq	%rsi
// 0x141f3f49f  57                             pushq	%rdi
// 0x141f3f4a0  53                             pushq	%rbx
// 0x141f3f4a1  4883ec48                       subq	$0x48, %rsp
// 0x141f3f4a5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3f4ac  0f29742430                     movaps	%xmm6, 0x30(%rsp)
// 0x141f3f4b1  488b4528                       movq	0x28(%rbp), %rax
// Frames: F23 -> F24 -> F25 -> F9
// 0x141f3f4b5  483b45f0                       cmpq	-0x10(%rbp), %rax
// Frames: F82
// 0x141f3f4b9  741c                           je	0x141f3f4d7 <.text+0x1f3e4d7>
// Frames: F68 -> F69 -> F44 -> F44 -> F82
// 0x141f3f4bb  488b5538                       movq	0x38(%rbp), %rdx
// Frames: F67 -> F68 -> F69 -> F44 -> F44 -> F82
// 0x141f3f4bf  4885d2                         testq	%rdx, %rdx
// 0x141f3f4c2  7413                           je	0x141f3f4d7 <.text+0x1f3e4d7>
// Frames: F68 -> F69 -> F44 -> F44 -> F82
// 0x141f3f4c4  488b4d40                       movq	0x40(%rbp), %rcx
// Frames: F73 -> F74 -> F68 -> F69 -> F44 -> F44 -> F82
// 0x141f3f4c8  48c1e203                       shlq	$0x3, %rdx
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F69 -> F44 -> F44 -> F82
// 0x141f3f4cc  41b808000000                   movl	$0x8, %r8d
// 0x141f3f4d2  e8196476fe                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F1
// 0x141f3f4d7  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141f3f4dc  4883c448                       addq	$0x48, %rsp
// 0x141f3f4e0  5b                             popq	%rbx
// 0x141f3f4e1  5f                             popq	%rdi
// 0x141f3f4e2  5e                             popq	%rsi
// 0x141f3f4e3  415c                           popq	%r12
// 0x141f3f4e5  415d                           popq	%r13
// 0x141f3f4e7  415e                           popq	%r14
// 0x141f3f4e9  415f                           popq	%r15
// 0x141f3f4eb  5d                             popq	%rbp
// 0x141f3f4ec  c3                             retq
//
// RANGE rva_28310f0_len_503: RVA 0x28310f0, 1283 PDB bytes; 303 instructions
// Evidence: _assembly/roof-v1/C/pack-003/rva_28310f0_len_503.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3787:268: system_roof::replay_roof::impl$0::process
// Frames: F83
// 0x1428310f0  4157                           pushq	%r15
// 0x1428310f2  4156                           pushq	%r14
// 0x1428310f4  4154                           pushq	%r12
// 0x1428310f6  56                             pushq	%rsi
// 0x1428310f7  57                             pushq	%rdi
// 0x1428310f8  55                             pushq	%rbp
// 0x1428310f9  53                             pushq	%rbx
// 0x1428310fa  4881ecb0000000                 subq	$0xb0, %rsp
// 0x142831101  440f299c24a0000000             movaps	%xmm11, 0xa0(%rsp)
// 0x14283110a  440f29942490000000             movaps	%xmm10, 0x90(%rsp)
// 0x142831113  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x14283111c  440f29442470                   movaps	%xmm8, 0x70(%rsp)
// 0x142831122  0f297c2460                     movaps	%xmm7, 0x60(%rsp)
// 0x142831127  0f29742450                     movaps	%xmm6, 0x50(%rsp)
// Frames: F84
// 0x14283112c  488b4108                       movq	0x8(%rcx), %rax
// 0x142831130  0fb65010                       movzbl	0x10(%rax), %edx
// 0x142831134  4c8d155584d500                 leaq	0xd58455(%rip), %r10    # 0x143589590
// 0x14283113b  49631492                       movslq	(%r10,%rdx,4), %rdx
// 0x14283113f  4c01d2                         addq	%r10, %rdx
// 0x142831142  ffe2                           jmpq	*%rdx
// Frames: F85
// 0x142831144  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x142831148  0f8512040000                   jne	0x142831560 <.text+0x2830560>
// Frames: F86
// 0x14283114e  488b7840                       movq	0x40(%rax), %rdi
// Frames: F87
// 0x142831152  0f104024                       movups	0x24(%rax), %xmm0
// 0x142831156  0f104830                       movups	0x30(%rax), %xmm1
// 0x14283115a  0f114c243c                     movups	%xmm1, 0x3c(%rsp)
// 0x14283115f  0f29442430                     movaps	%xmm0, 0x30(%rsp)
// Frames: F88
// 0x142831164  8b6814                         movl	0x14(%rax), %ebp
// Frames: F89
// 0x142831167  8b5020                         movl	0x20(%rax), %edx
// 0x14283116a  89542428                       movl	%edx, 0x28(%rsp)
// 0x14283116e  488b4018                       movq	0x18(%rax), %rax
// 0x142831172  4889442420                     movq	%rax, 0x20(%rsp)
// Frames: F4 -> F5 -> F90 -> F91
// 0x142831177  418b401c                       movl	0x1c(%r8), %eax
// 0x14283117b  498b5010                       movq	0x10(%r8), %rdx
// 0x14283117f  8902                           movl	%eax, (%rdx)
// Frames: F92 -> F90 -> F91
// 0x142831181  498b30                         movq	(%r8), %rsi
// Frames: F33 -> F18 -> F93 -> F94 -> F90 -> F91
// 0x142831184  488b5e30                       movq	0x30(%rsi), %rbx
// Frames: F95 -> F94 -> F90 -> F91
// 0x142831188  4c8b7640                       movq	0x40(%rsi), %r14
// Frames: F34 -> F18 -> F93 -> F94 -> F90 -> F91
// 0x14283118c  483b5e20                       cmpq	0x20(%rsi), %rbx
// 0x142831190  7512                           jne	0x1428311a4 <.text+0x28301a4>
// Frames: F93 -> F94 -> F90 -> F91
// 0x142831192  488d4620                       leaq	0x20(%rsi), %rax
// 0x142831196  4989cf                         movq	%rcx, %r15
// Frames: F35 -> F18 -> F93 -> F94 -> F90 -> F91
// 0x142831199  4889c1                         movq	%rax, %rcx
// 0x14283119c  e86fe55300                     callq	0x142d6f710 <.text+0x2d6e710>
// 0x1428311a1  4c89f9                         movq	%r15, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F93 -> F94 -> F90 -> F91
// 0x1428311a4  488b4628                       movq	0x28(%rsi), %rax
// Frames: F54 -> F17 -> F18 -> F93 -> F94 -> F90 -> F91
// 0x1428311a8  4889da                         movq	%rbx, %rdx
// 0x1428311ab  48c1e206                       shlq	$0x6, %rdx
// Frames: F20 -> F21 -> F18 -> F93 -> F94 -> F90 -> F91
// 0x1428311af  4c893410                       movq	%r14, (%rax,%rdx)
// 0x1428311b3  48897c1008                     movq	%rdi, 0x8(%rax,%rdx)
// 0x1428311b8  896c1010                       movl	%ebp, 0x10(%rax,%rdx)
// 0x1428311bc  4c8b442420                     movq	0x20(%rsp), %r8
// 0x1428311c1  4c89441014                     movq	%r8, 0x14(%rax,%rdx)
// 0x1428311c6  448b442428                     movl	0x28(%rsp), %r8d
// 0x1428311cb  448944101c                     movl	%r8d, 0x1c(%rax,%rdx)
// 0x1428311d0  0f28442430                     movaps	0x30(%rsp), %xmm0
// 0x1428311d5  0f11441020                     movups	%xmm0, 0x20(%rax,%rdx)
// 0x1428311da  0f1044243c                     movups	0x3c(%rsp), %xmm0
// 0x1428311df  0f1144102c                     movups	%xmm0, 0x2c(%rax,%rdx)
// 0x1428311e4  c644103c00                     movb	$0x0, 0x3c(%rax,%rdx)
// 0x1428311e9  e9b9030000                     jmp	0x1428315a7 <.text+0x28305a7>
// Frames: F96
// 0x1428311ee  f30f107110                     movss	0x10(%rcx), %xmm6
// 0x1428311f3  0f57c0                         xorps	%xmm0, %xmm0
// 0x1428311f6  0f2ef0                         ucomiss	%xmm0, %xmm6
// 0x1428311f9  0f8579020000                   jne	0x142831478 <.text+0x2830478>
// 0x1428311ff  0f8a73020000                   jp	0x142831478 <.text+0x2830478>
// Frames: F97
// 0x142831205  488b942420010000               movq	0x120(%rsp), %rdx
// 0x14283120d  440fb64114                     movzbl	0x14(%rcx), %r8d
// 0x142831212  4989c1                         movq	%rax, %r9
// 0x142831215  4d29c1                         subq	%r8, %r9
// 0x142831218  410fb65912                     movzbl	0x12(%r9), %ebx
// Frames: F98
// 0x14283121d  4c8b7018                       movq	0x18(%rax), %r14
// Frames: F4 -> F5 -> F90 -> F99
// 0x142831221  8b421c                         movl	0x1c(%rdx), %eax
// 0x142831224  4c8b4210                       movq	0x10(%rdx), %r8
// 0x142831228  418900                         movl	%eax, (%r8)
// Frames: F92 -> F90 -> F99
// 0x14283122b  488b32                         movq	(%rdx), %rsi
// Frames: F33 -> F18 -> F93 -> F94 -> F90 -> F99
// 0x14283122e  488b7e30                       movq	0x30(%rsi), %rdi
// Frames: F95 -> F94 -> F90 -> F99
// 0x142831232  4c8b7e40                       movq	0x40(%rsi), %r15
// Frames: F34 -> F18 -> F93 -> F94 -> F90 -> F99
// 0x142831236  483b7e20                       cmpq	0x20(%rsi), %rdi
// 0x14283123a  7518                           jne	0x142831254 <.text+0x2830254>
// Frames: F93 -> F94 -> F90 -> F99
// 0x14283123c  488d4620                       leaq	0x20(%rsi), %rax
// 0x142831240  4989cc                         movq	%rcx, %r12
// Frames: F35 -> F18 -> F93 -> F94 -> F90 -> F99
// 0x142831243  4889c1                         movq	%rax, %rcx
// 0x142831246  0f28f9                         movaps	%xmm1, %xmm7
// 0x142831249  e892e25300                     callq	0x142d6f4e0 <.text+0x2d6e4e0>
// 0x14283124e  0f28cf                         movaps	%xmm7, %xmm1
// 0x142831251  4c89e1                         movq	%r12, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F93 -> F94 -> F90 -> F99
// 0x142831254  488b4628                       movq	0x28(%rsi), %rax
// Frames: F54 -> F17 -> F18 -> F93 -> F94 -> F90 -> F99
// 0x142831258  488d147f                       leaq	(%rdi,%rdi,2), %rdx
// Frames: F20 -> F21 -> F18 -> F93 -> F94 -> F90 -> F99
// 0x14283125c  4c893cd0                       movq	%r15, (%rax,%rdx,8)
// 0x142831260  4c8974d008                     movq	%r14, 0x8(%rax,%rdx,8)
// 0x142831265  885cd010                       movb	%bl, 0x10(%rax,%rdx,8)
// 0x142831269  885cd011                       movb	%bl, 0x11(%rax,%rdx,8)
// 0x14283126d  c644d01200                     movb	$0x0, 0x12(%rax,%rdx,8)
// Frames: F22 -> F18 -> F93 -> F94 -> F90 -> F99
// 0x142831272  48ffc7                         incq	%rdi
// 0x142831275  48897e30                       movq	%rdi, 0x30(%rsi)
// Frames: F100 -> F94 -> F90 -> F99
// 0x142831279  e9f6010000                     jmp	0x142831474 <.text+0x2830474>
// Frames: F101
// 0x14283127e  803900                         cmpb	$0x0, (%rcx)
// 0x142831281  741f                           je	0x1428312a2 <.text+0x28302a2>
// Frames: F102
// 0x142831283  f30f584910                     addss	0x10(%rcx), %xmm1
// 0x142831288  f30f114910                     movss	%xmm1, 0x10(%rcx)
// Frames: F103
// 0x14283128d  f30f5e4904                     divss	0x4(%rcx), %xmm1
// 0x142831292  0f57f6                         xorps	%xmm6, %xmm6
// Frames: F104 -> F103
// 0x142831295  f30f5ff1                       maxss	%xmm1, %xmm6
// 0x142831299  0f2e3584376d00                 ucomiss	0x6d3784(%rip), %xmm6   # 0x142f04a24
// 0x1428312a0  720c                           jb	0x1428312ae <.text+0x28302ae>
// Frames: F105
// 0x1428312a2  c6411501                       movb	$0x1, 0x15(%rcx)
// 0x1428312a6  f30f103576376d00               movss	0x6d3776(%rip), %xmm6   # 0x142f04a24
// Frames: F106
// 0x1428312ae  488b942410010000               movq	0x110(%rsp), %rdx
// 0x1428312b6  f30f100566376d00               movss	0x6d3766(%rip), %xmm0   # 0x142f04a24
// 0x1428312be  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x1428312c2  740a                           je	0x1428312ce <.text+0x28302ce>
// 0x1428312c4  0f28c8                         movaps	%xmm0, %xmm1
// 0x1428312c7  f30f5cce                       subss	%xmm6, %xmm1
// 0x1428312cb  0f28f1                         movaps	%xmm1, %xmm6
// Frames: F107
// 0x1428312ce  488b7038                       movq	0x38(%rax), %rsi
// Frames: F108 -> F109 -> F110
// 0x1428312d2  f30f5cc6                       subss	%xmm6, %xmm0
// 0x1428312d6  f30f104818                     movss	0x18(%rax), %xmm1
// Frames: F111 -> F109 -> F110
// 0x1428312db  f30f59c8                       mulss	%xmm0, %xmm1
// 0x1428312df  f30f107828                     movss	0x28(%rax), %xmm7
// 0x1428312e4  f30f59fe                       mulss	%xmm6, %xmm7
// Frames: F112 -> F109 -> F110
// 0x1428312e8  f30f58f9                       addss	%xmm1, %xmm7
// 0x1428312ec  f30f104814                     movss	0x14(%rax), %xmm1
// Frames: F111 -> F109 -> F113
// 0x1428312f1  f30f59c8                       mulss	%xmm0, %xmm1
// 0x1428312f5  f3440f104024                   movss	0x24(%rax), %xmm8
// 0x1428312fb  f3440f59c6                     mulss	%xmm6, %xmm8
// Frames: F112 -> F109 -> F113
// 0x142831300  f3440f58c1                     addss	%xmm1, %xmm8
// 0x142831305  f30f104840                     movss	0x40(%rax), %xmm1
// Frames: F111 -> F114 -> F115 -> F116
// 0x14283130a  f30f59c8                       mulss	%xmm0, %xmm1
// 0x14283130e  f30f105044                     movss	0x44(%rax), %xmm2
// Frames: F111 -> F117 -> F115 -> F116
// 0x142831313  f30f59d0                       mulss	%xmm0, %xmm2
// 0x142831317  f3440f104848                   movss	0x48(%rax), %xmm9
// Frames: F111 -> F114 -> F115 -> F116
// 0x14283131d  f3440f59ce                     mulss	%xmm6, %xmm9
// Frames: F112 -> F118 -> F115 -> F116
// 0x142831322  f3440f58c9                     addss	%xmm1, %xmm9
// 0x142831327  f3440f10504c                   movss	0x4c(%rax), %xmm10
// Frames: F111 -> F117 -> F115 -> F116
// 0x14283132d  f3440f59d6                     mulss	%xmm6, %xmm10
// Frames: F112 -> F119 -> F115 -> F116
// 0x142831332  f3440f58d2                     addss	%xmm2, %xmm10
// 0x142831337  f30f10481c                     movss	0x1c(%rax), %xmm1
// Frames: F111 -> F109 -> F120
// 0x14283133c  f30f59c8                       mulss	%xmm0, %xmm1
// 0x142831340  f3440f10582c                   movss	0x2c(%rax), %xmm11
// 0x142831346  f3440f59de                     mulss	%xmm6, %xmm11
// Frames: F111 -> F109 -> F121
// 0x14283134b  f30f594020                     mulss	0x20(%rax), %xmm0
// Frames: F112 -> F109 -> F120
// 0x142831350  f3440f58d9                     addss	%xmm1, %xmm11
// Frames: F111 -> F109 -> F121
// 0x142831355  f30f597030                     mulss	0x30(%rax), %xmm6
// Frames: F112 -> F109 -> F121
// 0x14283135a  f30f58f0                       addss	%xmm0, %xmm6
// Frames: F4 -> F5 -> F90 -> F122
// 0x14283135e  8b421c                         movl	0x1c(%rdx), %eax
// 0x142831361  488b4a10                       movq	0x10(%rdx), %rcx
// 0x142831365  8901                           movl	%eax, (%rcx)
// Frames: F92 -> F90 -> F122
// 0x142831367  488b3a                         movq	(%rdx), %rdi
// Frames: F33 -> F18 -> F93 -> F94 -> F90 -> F122
// 0x14283136a  488b5f30                       movq	0x30(%rdi), %rbx
// Frames: F95 -> F94 -> F90 -> F122
// 0x14283136e  4c8b7740                       movq	0x40(%rdi), %r14
// Frames: F34 -> F18 -> F93 -> F94 -> F90 -> F122
// 0x142831372  483b5f20                       cmpq	0x20(%rdi), %rbx
// 0x142831376  7509                           jne	0x142831381 <.text+0x2830381>
// Frames: F93 -> F94 -> F90 -> F122
// 0x142831378  488d4f20                       leaq	0x20(%rdi), %rcx
// Frames: F35 -> F18 -> F93 -> F94 -> F90 -> F122
// 0x14283137c  e88fe35300                     callq	0x142d6f710 <.text+0x2d6e710>
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F93 -> F94 -> F90 -> F122
// 0x142831381  488b4728                       movq	0x28(%rdi), %rax
// Frames: F54 -> F17 -> F18 -> F93 -> F94 -> F90 -> F122
// 0x142831385  4889d9                         movq	%rbx, %rcx
// 0x142831388  48c1e106                       shlq	$0x6, %rcx
// Frames: F20 -> F21 -> F18 -> F93 -> F94 -> F90 -> F122
// 0x14283138c  c7040801000000                 movl	$0x1, (%rax,%rcx)
// 0x142831393  f30f117c0804                   movss	%xmm7, 0x4(%rax,%rcx)
// 0x142831399  c744080801000000               movl	$0x1, 0x8(%rax,%rcx)
// 0x1428313a1  f3440f1144080c                 movss	%xmm8, 0xc(%rax,%rcx)
// 0x1428313a8  c744081001000000               movl	$0x1, 0x10(%rax,%rcx)
// 0x1428313b0  f3440f115c0814                 movss	%xmm11, 0x14(%rax,%rcx)
// 0x1428313b7  c744081801000000               movl	$0x1, 0x18(%rax,%rcx)
// 0x1428313bf  f30f1174081c                   movss	%xmm6, 0x1c(%rax,%rcx)
// 0x1428313c5  4889740820                     movq	%rsi, 0x20(%rax,%rcx)
// 0x1428313ca  c744082801000000               movl	$0x1, 0x28(%rax,%rcx)
// 0x1428313d2  f3440f114c082c                 movss	%xmm9, 0x2c(%rax,%rcx)
// 0x1428313d9  f3440f11540830                 movss	%xmm10, 0x30(%rax,%rcx)
// 0x1428313e0  c644083401                     movb	$0x1, 0x34(%rax,%rcx)
// 0x1428313e5  4c89740838                     movq	%r14, 0x38(%rax,%rcx)
// Frames: F22 -> F18 -> F93 -> F94 -> F90 -> F122
// 0x1428313ea  48ffc3                         incq	%rbx
// 0x1428313ed  48895f30                       movq	%rbx, 0x30(%rdi)
// Frames: F100 -> F94 -> F90 -> F122
// 0x1428313f1  48ff4740                       incq	0x40(%rdi)
// Frames: F123
// 0x1428313f5  e9bc010000                     jmp	0x1428315b6 <.text+0x28305b6>
// Frames: F124
// 0x1428313fa  f30f107110                     movss	0x10(%rcx), %xmm6
// 0x1428313ff  0f57c0                         xorps	%xmm0, %xmm0
// 0x142831402  0f2ef0                         ucomiss	%xmm0, %xmm6
// 0x142831405  7571                           jne	0x142831478 <.text+0x2830478>
// 0x142831407  7a6f                           jp	0x142831478 <.text+0x2830478>
// Frames: F125
// 0x142831409  488b942418010000               movq	0x118(%rsp), %rdx
// 0x142831411  440fb64114                     movzbl	0x14(%rcx), %r8d
// 0x142831416  4989c1                         movq	%rax, %r9
// 0x142831419  4d29c1                         subq	%r8, %r9
// 0x14283141c  410fb65912                     movzbl	0x12(%r9), %ebx
// Frames: F126
// 0x142831421  488b7818                       movq	0x18(%rax), %rdi
// Frames: F4 -> F5 -> F90 -> F127
// 0x142831425  8b421c                         movl	0x1c(%rdx), %eax
// 0x142831428  4c8b4210                       movq	0x10(%rdx), %r8
// 0x14283142c  418900                         movl	%eax, (%r8)
// Frames: F92 -> F90 -> F127
// 0x14283142f  488b32                         movq	(%rdx), %rsi
// Frames: F33 -> F18 -> F93 -> F94 -> F90 -> F127
// 0x142831432  4c8b7630                       movq	0x30(%rsi), %r14
// Frames: F95 -> F94 -> F90 -> F127
// 0x142831436  4c8b7e40                       movq	0x40(%rsi), %r15
// Frames: F34 -> F18 -> F93 -> F94 -> F90 -> F127
// 0x14283143a  4c3b7620                       cmpq	0x20(%rsi), %r14
// 0x14283143e  7518                           jne	0x142831458 <.text+0x2830458>
// Frames: F93 -> F94 -> F90 -> F127
// 0x142831440  488d4620                       leaq	0x20(%rsi), %rax
// 0x142831444  4989cc                         movq	%rcx, %r12
// Frames: F35 -> F18 -> F93 -> F94 -> F90 -> F127
// 0x142831447  4889c1                         movq	%rax, %rcx
// 0x14283144a  0f28f9                         movaps	%xmm1, %xmm7
// 0x14283144d  e88ee05300                     callq	0x142d6f4e0 <.text+0x2d6e4e0>
// 0x142831452  0f28cf                         movaps	%xmm7, %xmm1
// 0x142831455  4c89e1                         movq	%r12, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F93 -> F94 -> F90 -> F127
// 0x142831458  488b4628                       movq	0x28(%rsi), %rax
// Frames: F54 -> F17 -> F18 -> F93 -> F94 -> F90 -> F127
// 0x14283145c  4b8d1476                       leaq	(%r14,%r14,2), %rdx
// Frames: F20 -> F21 -> F18 -> F93 -> F94 -> F90 -> F127
// 0x142831460  4c893cd0                       movq	%r15, (%rax,%rdx,8)
// 0x142831464  48897cd008                     movq	%rdi, 0x8(%rax,%rdx,8)
// 0x142831469  885cd010                       movb	%bl, 0x10(%rax,%rdx,8)
// Frames: F22 -> F18 -> F93 -> F94 -> F90 -> F127
// 0x14283146d  49ffc6                         incq	%r14
// 0x142831470  4c897630                       movq	%r14, 0x30(%rsi)
// 0x142831474  48ff4640                       incq	0x40(%rsi)
// 0x142831478  833901                         cmpl	$0x1, (%rcx)
// 0x14283147b  0f8531010000                   jne	0x1428315b2 <.text+0x28305b2>
// Frames: F104 -> F128
// 0x142831481  f30f58ce                       addss	%xmm6, %xmm1
// 0x142831485  f30f114910                     movss	%xmm1, 0x10(%rcx)
// 0x14283148a  f30f5e4904                     divss	0x4(%rcx), %xmm1
// 0x14283148f  0f57c0                         xorps	%xmm0, %xmm0
// 0x142831492  f30f5fc1                       maxss	%xmm1, %xmm0
// 0x142831496  f30f100d86356d00               movss	0x6d3586(%rip), %xmm1   # 0x142f04a24
// Frames: F129 -> F128
// 0x14283149e  0f28d1                         movaps	%xmm1, %xmm2
// 0x1428314a1  f30f5dd0                       minss	%xmm0, %xmm2
// 0x1428314a5  0f2ed1                         ucomiss	%xmm1, %xmm2
// 0x1428314a8  0f8508010000                   jne	0x1428315b6 <.text+0x28305b6>
// 0x1428314ae  0f8bfe000000                   jnp	0x1428315b2 <.text+0x28305b2>
// 0x1428314b4  e9fd000000                     jmp	0x1428315b6 <.text+0x28305b6>
// Frames: F130
// 0x1428314b9  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x1428314bd  0f849d000000                   je	0x142831560 <.text+0x2830560>
// Frames: F131
// 0x1428314c3  488b7840                       movq	0x40(%rax), %rdi
// Frames: F132
// 0x1428314c7  0f104024                       movups	0x24(%rax), %xmm0
// 0x1428314cb  0f104830                       movups	0x30(%rax), %xmm1
// 0x1428314cf  0f114c243c                     movups	%xmm1, 0x3c(%rsp)
// 0x1428314d4  0f29442430                     movaps	%xmm0, 0x30(%rsp)
// Frames: F133
// 0x1428314d9  8b6814                         movl	0x14(%rax), %ebp
// Frames: F134
// 0x1428314dc  8b5020                         movl	0x20(%rax), %edx
// 0x1428314df  89542428                       movl	%edx, 0x28(%rsp)
// 0x1428314e3  488b4018                       movq	0x18(%rax), %rax
// 0x1428314e7  4889442420                     movq	%rax, 0x20(%rsp)
// Frames: F4 -> F5 -> F90 -> F135
// 0x1428314ec  418b401c                       movl	0x1c(%r8), %eax
// 0x1428314f0  498b5010                       movq	0x10(%r8), %rdx
// 0x1428314f4  8902                           movl	%eax, (%rdx)
// Frames: F92 -> F90 -> F135
// 0x1428314f6  498b30                         movq	(%r8), %rsi
// Frames: F33 -> F18 -> F93 -> F94 -> F90 -> F135
// 0x1428314f9  488b5e30                       movq	0x30(%rsi), %rbx
// Frames: F95 -> F94 -> F90 -> F135
// 0x1428314fd  4c8b7640                       movq	0x40(%rsi), %r14
// Frames: F34 -> F18 -> F93 -> F94 -> F90 -> F135
// 0x142831501  483b5e20                       cmpq	0x20(%rsi), %rbx
// 0x142831505  7512                           jne	0x142831519 <.text+0x2830519>
// Frames: F93 -> F94 -> F90 -> F135
// 0x142831507  488d4620                       leaq	0x20(%rsi), %rax
// 0x14283150b  4989cf                         movq	%rcx, %r15
// Frames: F35 -> F18 -> F93 -> F94 -> F90 -> F135
// 0x14283150e  4889c1                         movq	%rax, %rcx
// 0x142831511  e8fae15300                     callq	0x142d6f710 <.text+0x2d6e710>
// 0x142831516  4c89f9                         movq	%r15, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F93 -> F94 -> F90 -> F135
// 0x142831519  488b4628                       movq	0x28(%rsi), %rax
// Frames: F54 -> F17 -> F18 -> F93 -> F94 -> F90 -> F135
// 0x14283151d  4889da                         movq	%rbx, %rdx
// 0x142831520  48c1e206                       shlq	$0x6, %rdx
// Frames: F20 -> F21 -> F18 -> F93 -> F94 -> F90 -> F135
// 0x142831524  4c893410                       movq	%r14, (%rax,%rdx)
// 0x142831528  48897c1008                     movq	%rdi, 0x8(%rax,%rdx)
// 0x14283152d  896c1010                       movl	%ebp, 0x10(%rax,%rdx)
// 0x142831531  4c8b442420                     movq	0x20(%rsp), %r8
// 0x142831536  4c89441014                     movq	%r8, 0x14(%rax,%rdx)
// 0x14283153b  448b442428                     movl	0x28(%rsp), %r8d
// 0x142831540  448944101c                     movl	%r8d, 0x1c(%rax,%rdx)
// 0x142831545  0f28442430                     movaps	0x30(%rsp), %xmm0
// 0x14283154a  0f11441020                     movups	%xmm0, 0x20(%rax,%rdx)
// 0x14283154f  0f1044243c                     movups	0x3c(%rsp), %xmm0
// 0x142831554  0f1144102c                     movups	%xmm0, 0x2c(%rax,%rdx)
// 0x142831559  c644103c01                     movb	$0x1, 0x3c(%rax,%rdx)
// Frames: F22 -> F18 -> F93 -> F94 -> F90 -> F135
// 0x14283155e  eb47                           jmp	0x1428315a7 <.text+0x28305a7>
// Frames: F4 -> F5 -> F136 -> F128
// 0x142831560  488b7840                       movq	0x40(%rax), %rdi
// 0x142831564  418b411c                       movl	0x1c(%r9), %eax
// 0x142831568  498b5110                       movq	0x10(%r9), %rdx
// 0x14283156c  8902                           movl	%eax, (%rdx)
// Frames: F92 -> F136 -> F128
// 0x14283156e  498b31                         movq	(%r9), %rsi
// Frames: F33 -> F18 -> F93 -> F94 -> F136 -> F128
// 0x142831571  488b5e30                       movq	0x30(%rsi), %rbx
// Frames: F95 -> F94 -> F136 -> F128
// 0x142831575  4c8b7640                       movq	0x40(%rsi), %r14
// Frames: F34 -> F18 -> F93 -> F94 -> F136 -> F128
// 0x142831579  483b5e20                       cmpq	0x20(%rsi), %rbx
// 0x14283157d  7512                           jne	0x142831591 <.text+0x2830591>
// Frames: F93 -> F94 -> F136 -> F128
// 0x14283157f  488d4620                       leaq	0x20(%rsi), %rax
// 0x142831583  4989cf                         movq	%rcx, %r15
// Frames: F35 -> F18 -> F93 -> F94 -> F136 -> F128
// 0x142831586  4889c1                         movq	%rax, %rcx
// 0x142831589  e852df5300                     callq	0x142d6f4e0 <.text+0x2d6e4e0>
// 0x14283158e  4c89f9                         movq	%r15, %rcx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F93 -> F94 -> F136 -> F128
// 0x142831591  488b4628                       movq	0x28(%rsi), %rax
// Frames: F54 -> F17 -> F18 -> F93 -> F94 -> F136 -> F128
// 0x142831595  488d145b                       leaq	(%rbx,%rbx,2), %rdx
// Frames: F20 -> F21 -> F18 -> F93 -> F94 -> F136 -> F128
// 0x142831599  4c8934d0                       movq	%r14, (%rax,%rdx,8)
// 0x14283159d  48897cd008                     movq	%rdi, 0x8(%rax,%rdx,8)
// 0x1428315a2  c644d01001                     movb	$0x1, 0x10(%rax,%rdx,8)
// 0x1428315a7  48ffc3                         incq	%rbx
// 0x1428315aa  48895e30                       movq	%rbx, 0x30(%rsi)
// 0x1428315ae  48ff4640                       incq	0x40(%rsi)
// 0x1428315b2  c6411501                       movb	$0x1, 0x15(%rcx)
// Frames: F137
// 0x1428315b6  0f28742450                     movaps	0x50(%rsp), %xmm6
// 0x1428315bb  0f287c2460                     movaps	0x60(%rsp), %xmm7
// 0x1428315c0  440f28442470                   movaps	0x70(%rsp), %xmm8
// 0x1428315c6  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1428315cf  440f28942490000000             movaps	0x90(%rsp), %xmm10
// 0x1428315d8  440f289c24a0000000             movaps	0xa0(%rsp), %xmm11
// 0x1428315e1  4881c4b0000000                 addq	$0xb0, %rsp
// 0x1428315e8  5b                             popq	%rbx
// 0x1428315e9  5d                             popq	%rbp
// 0x1428315ea  5f                             popq	%rdi
// 0x1428315eb  5e                             popq	%rsi
// 0x1428315ec  415c                           popq	%r12
// 0x1428315ee  415e                           popq	%r14
// 0x1428315f0  415f                           popq	%r15
// 0x1428315f2  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/replay_roof.rs:23:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F2: crates/systems/roof/src/replay_roof.rs:32:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F3: crates/systems/roof/src/replay_roof.rs:33:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F6: crates/systems/roof/src/replay_roof.rs:35:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F7: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:464:0 alloc::vec::Vec::new
// F8: crates/systems/roof/src/replay_roof.rs:36:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F9: crates/systems/roof/src/replay_roof.rs:38:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F10: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F11: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:242:0 core::slice::iter::IterMut::new
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1061:0 core::slice::impl$0::iter_mut
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F16: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2000:0 alloc::vec::Vec::as_mut_ptr
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1032:0 alloc::vec::Vec::push_mut
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:992:0 alloc::vec::Vec::push
// F19: crates/systems/roof/src/replay_roof.rs:64:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F20: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1917:0 core::ptr::write
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:0 alloc::vec::Vec::push_mut
// F22: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1034:0 alloc::vec::Vec::push_mut
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F24: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:180:0 core::slice::iter::impl$179::next
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/enumerate.rs:80:0 core::iter::adapters::enumerate::impl$1::next
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:659:0 core::ptr::non_null::NonNull::add
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:185:0 core::slice::iter::impl$179::next
// F28: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/enumerate.rs:82:0 core::iter::adapters::enumerate::impl$1::next
// F29: crates/systems/roof/src/replay_roof.rs:39:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F30: crates/systems/roof/src/replay_roof.rs:55:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F31: crates/systems/roof/src/replay_roof.rs:54:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F32: crates/systems/roof/src/replay_roof.rs:63:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1025:0 alloc::vec::Vec::push_mut
// F34: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1028:0 alloc::vec::Vec::push_mut
// F35: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1029:0 alloc::vec::Vec::push_mut
// F36: crates/systems/roof/src/replay_roof.rs:70:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F37: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:979:0 core::slice::impl$0::reverse
// F38: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1014:0 core::slice::impl$0::reverse::revswap
// F39: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:998:0 core::slice::impl$0::reverse
// F40: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1015:0 core::slice::impl$0::reverse::revswap
// F41: crates/systems/roof/src/replay_roof.rs:72:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F42: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/into_iter.rs:223:0 alloc::vec::into_iter::impl$6::next
// F43: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:759:0 core::mem::swap
// F44: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:805:0 core::ptr::drop_in_place
// F45: crates/systems/roof/src/replay_roof.rs:73:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1717:0 core::ptr::read
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:990:0 core::ptr::non_null::NonNull::read
// F48: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/into_iter.rs:230:0 alloc::vec::into_iter::impl$6::next
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2997:0 alloc::vec::Vec::len
// F50: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2378:0 alloc::vec::Vec::try_remove
// F51: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2350:0 alloc::vec::Vec::remove
// F52: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2379:0 alloc::vec::Vec::try_remove
// F53: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2387:0 alloc::vec::Vec::try_remove
// F54: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 alloc::vec::Vec::as_mut_ptr
// F55: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2390:0 alloc::vec::Vec::try_remove
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2393:0 alloc::vec::Vec::try_remove
// F57: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:638:0 core::ptr::mut_ptr::impl$0::add
// F58: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2395:0 alloc::vec::Vec::try_remove
// F59: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2168:0 alloc::vec::Vec::set_len
// F60: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2351:0 alloc::vec::Vec::remove
// F61: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:444:0 core::ptr::non_null::NonNull::as_ref
// F62: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2104:0 alloc::sync::Arc::inner
// F63: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2811:0 alloc::sync::impl$42::drop
// F64: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3942:0 core::sync::atomic::atomic_sub
// F65: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3167:0 alloc::sync::Arc::inner
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2854:0 alloc::sync::impl$42::drop
// F67: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:632:0 alloc::raw_vec::RawVecInner::current_memory
// F68: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:872:0 alloc::raw_vec::RawVecInner::deallocate
// F69: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:423:0 alloc::raw_vec::impl$3::drop
// F70: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/into_iter.rs:506:0 alloc::vec::into_iter::impl$16::drop::impl$0::drop
// F71: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/into_iter.rs:517:0 alloc::vec::into_iter::impl$16::drop
// F72: crates/systems/roof/src/replay_roof.rs:74:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F73: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:1232:0 core::num::impl$11::unchecked_mul
// F74: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:640:0 alloc::raw_vec::RawVecInner::current_memory
// F75: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:115:0 alloc::alloc::dealloc
// F76: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:209:0 alloc::alloc::Global::deallocate_impl_runtime
// F77: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:324:0 alloc::alloc::Global::deallocate_impl
// F78: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:442:0 alloc::alloc::impl$1::deallocate
// F79: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:874:0 alloc::raw_vec::RawVecInner::deallocate
// F80: crates/systems/roof/src/replay_roof.rs:76:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F81: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2352:0 alloc::vec::Vec::remove
// F82: crates/systems/roof/src/replay_roof.rs:75:0 system_roof::replay_roof::replay_roof::hd069cad863ff5a71
// F83: crates/systems/roof/src/replay_roof.rs:80:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F84: crates/systems/roof/src/replay_roof.rs:92:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F85: crates/systems/roof/src/replay_roof.rs:198:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F86: crates/systems/roof/src/replay_roof.rs:202:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F87: crates/systems/roof/src/replay_roof.rs:203:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F88: crates/systems/roof/src/replay_roof.rs:204:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F89: crates/systems/roof/src/replay_roof.rs:205:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:70:0 bevy_ecs::message::message_writer::MessageWriter::write
// F91: crates/systems/roof/src/replay_roof.rs:200:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:437:0 bevy_ecs::change_detection::impl$38::deref_mut
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:143:0 bevy_ecs::message::messages::Messages::write_with_caller
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::change_detection::impl$38::deref_mut
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:131:0 bevy_ecs::message::messages::Messages::write_with_caller
// F96: crates/systems/roof/src/replay_roof.rs:223:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F97: crates/systems/roof/src/replay_roof.rs:224:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F98: crates/systems/roof/src/replay_roof.rs:229:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F99: crates/systems/roof/src/replay_roof.rs:228:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:144:0 bevy_ecs::message::messages::Messages::write_with_caller
// F101: crates/systems/roof/src/replay_roof.rs:160:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F102: crates/systems/roof/src/replay_roof.rs:163:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F103: crates/systems/roof/src/replay_roof.rs:164:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F104: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1512:0 core::f32::impl$0::clamp
// F105: crates/systems/roof/src/replay_roof.rs:169:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F106: crates/systems/roof/src/replay_roof.rs:173:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F107: crates/systems/roof/src/replay_roof.rs:180:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F108: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:218:0 core::ops::arith::impl$77::sub
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F110: crates/systems/roof/src/replay_roof.rs:182:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F111: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:352:0 core::ops::arith::impl$141::mul
// F112: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F113: crates/systems/roof/src/replay_roof.rs:183:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1311:0 glam::f32::vec2::impl$24::mul
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:797:0 glam::f32::vec2::Vec2::lerp
// F116: crates/systems/roof/src/replay_roof.rs:186:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1312:0 glam::f32::vec2::impl$24::mul
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1396:0 glam::f32::vec2::impl$34::add
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1397:0 glam::f32::vec2::impl$34::add
// F120: crates/systems/roof/src/replay_roof.rs:188:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F121: crates/systems/roof/src/replay_roof.rs:189:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F122: crates/systems/roof/src/replay_roof.rs:179:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F123: crates/systems/roof/src/replay_roof.rs:191:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F124: crates/systems/roof/src/replay_roof.rs:124:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F125: crates/systems/roof/src/replay_roof.rs:125:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F126: crates/systems/roof/src/replay_roof.rs:130:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F127: crates/systems/roof/src/replay_roof.rs:129:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F128: crates/systems/roof/src/replay_roof.rs:0:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F129: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1515:0 core::f32::impl$0::clamp
// F130: crates/systems/roof/src/replay_roof.rs:99:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F131: crates/systems/roof/src/replay_roof.rs:109:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F132: crates/systems/roof/src/replay_roof.rs:110:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F133: crates/systems/roof/src/replay_roof.rs:111:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F134: crates/systems/roof/src/replay_roof.rs:112:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F135: crates/systems/roof/src/replay_roof.rs:107:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:70:0 core::f32::impl$0::clamp
// F137: crates/systems/roof/src/replay_roof.rs:249:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::h1582ea5c947c4b64
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/replay_roof.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 83f3d5d1d78a7f745370c4157b73b1d4d1a522765ee9b024985e76484872105f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2446, 3787
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":3787,"record_offset":268,"rva":42143984,"va":5410853104}
// 编译实例 RVAs: 0x28310f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5497490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410853104,"raw_hex":"e892e25300","size":5,"soft_fail":false,"target_va":5416350944,"va":5410853449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f710
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5498223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410853104,"raw_hex":"e86fe55300","size":5,"soft_fail":false,"target_va":5416351504,"va":5410853276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3787,"record_offset":408,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3787,"record_offset":1132,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3787,"record_offset":1412,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3787,"record_offset":1160,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3787,"record_offset":628,"parent_record_offset":596,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3787,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3787,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3787,"record_offset":596,"parent_record_offset":560,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3787,"record_offset":528,"parent_record_offset":496,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3787,"record_offset":560,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3787,"record_offset":468,"parent_record_offset":432,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3787,"record_offset":432,"parent_record_offset":408,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3787,"record_offset":496,"parent_record_offset":432,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3787,"record_offset":1268,"parent_record_offset":1160,"ancestor_record_offset":268,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3787,"record_offset":1212,"parent_record_offset":1160,"ancestor_record_offset":268,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3787,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3787,"record_offset":716,"parent_record_offset":560,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3787,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":268,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03787.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3787,"record_offset":1584,"parent_record_offset":1412,"ancestor_record_offset":268,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03787.jsonl.gz"}
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
// 首条位置: {"module":2446,"record_offset":268,"rva":32763808,"va":5401472928}
// 编译实例 RVAs: 0x1f3efa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15146909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e89d1fe700","size":5,"soft_fail":false,"target_va":5416621000,"va":5401474086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-25795345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8ef6476fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5401474044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28342780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e804864ffe","size":5,"soft_fail":false,"target_va":5373130976,"va":5401473751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073aca0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25184006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8fab87ffe","size":5,"soft_fail":false,"target_va":5376289952,"va":5401473953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099f7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22673981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8c305a6fe","size":5,"soft_fail":false,"target_va":5378799536,"va":5401473512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21912751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e851a3b1fe","size":5,"soft_fail":false,"target_va":5379560240,"va":5401472986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19589868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e81415d5fe","size":5,"soft_fail":false,"target_va":5381884032,"va":5401473895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19586403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e89d22d5fe","size":5,"soft_fail":false,"target_va":5381887584,"va":5401473982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c916c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19586133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8ab23d5fe","size":5,"soft_fail":false,"target_va":5381887680,"va":5401473808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c918c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19585823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8e124d5fe","size":5,"soft_fail":false,"target_va":5381888192,"va":5401474010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91ff0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19583810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8be2cd5fe","size":5,"soft_fail":false,"target_va":5381890032,"va":5401473837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c92250
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19583087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8912fd5fe","size":5,"soft_fail":false,"target_va":5381890640,"va":5401473722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428310f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9379911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e847208f00","size":5,"soft_fail":false,"target_va":5410853104,"va":5401473188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15583444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8d4c8ed00","size":5,"soft_fail":false,"target_va":5417057072,"va":5401473623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14892745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8c93ee300","size":5,"soft_fail":false,"target_va":5416365968,"va":5401473218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20753158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401472928,"raw_hex":"e8fa54c3fe","size":5,"soft_fail":false,"target_va":5380719856,"va":5401473009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2446,"record_offset":480,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2446,"record_offset":592,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2446,"record_offset":3600,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2446,"record_offset":1060,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2446,"record_offset":424,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2446,"record_offset":808,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2446,"record_offset":1192,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2446,"record_offset":508,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2446,"record_offset":948,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2446,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2446,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2446,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2446,"record_offset":4172,"parent_record_offset":4084,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2446,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2446,"record_offset":4112,"parent_record_offset":4084,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2446,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2446,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2446,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2446,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2446,"record_offset":1304,"parent_record_offset":1268,"ancestor_record_offset":268,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2446,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":268,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2446,"record_offset":660,"parent_record_offset":620,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2446,"record_offset":3684,"parent_record_offset":3636,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2446,"record_offset":620,"parent_record_offset":592,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2446,"record_offset":3904,"parent_record_offset":3636,"ancestor_record_offset":268,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2446,"record_offset":3636,"parent_record_offset":3600,"ancestor_record_offset":268,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2446,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":268,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2446,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":268,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2446,"record_offset":448,"parent_record_offset":424,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2446,"record_offset":1020,"parent_record_offset":980,"ancestor_record_offset":268,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2446,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2446,"record_offset":1220,"parent_record_offset":1192,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2446,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2446,"record_offset":912,"parent_record_offset":844,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2446,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":268,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2446,"record_offset":1132,"parent_record_offset":1060,"ancestor_record_offset":268,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2446,"record_offset":880,"parent_record_offset":844,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2446,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":268,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2446,"record_offset":772,"parent_record_offset":620,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2446,"record_offset":980,"parent_record_offset":948,"ancestor_record_offset":268,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2446,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":268,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2446,"record_offset":844,"parent_record_offset":808,"ancestor_record_offset":268,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2446,"record_offset":1364,"parent_record_offset":1304,"ancestor_record_offset":268,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/02446.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2446,"record_offset":1396,"parent_record_offset":1268,"ancestor_record_offset":268,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/02446.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>

// ------------------------------------------------------------------
// 函数: system_roof::replay_roof::impl$0::process
// 信息 ID: 76882be78145bba1789f8674
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::replay_roof::replay_roof @ 0x1f3f0a4 -> 0x28310f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::replay_roof::replay_roof
// 信息 ID: 05cfde99280013f3c1c0ca4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5deebb0d68fbc98334793912: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)> @ 0xfd6420 -> 0x1f3efa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)> > @ 0x1f2b040 -> 0x1f3efa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
