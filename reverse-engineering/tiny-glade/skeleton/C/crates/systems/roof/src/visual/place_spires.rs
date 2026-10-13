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
// Inline site 2413:14844 -> host 2413:14504 / rva_1f074a0_len_15b; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1f074c2, 0x1f075d6), 62 instructions
//
// FUNCTION system_roof::visual::place_spires::place_roof_spires
// Candidate interfaces: cd898a0c41c18ea683f31317
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3750:268 -> rva_281a950_len_6d7
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1f074a0_len_15b: RVA 0x1f074a0, 347 PDB bytes; 81 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_1f074a0_len_15b.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2413:14504: core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>
// Frames: F1
// 0x141f074a0  56                             pushq	%rsi
// 0x141f074a1  57                             pushq	%rdi
// 0x141f074a2  4883ec78                       subq	$0x78, %rsp
// 0x141f074a6  440f29442460                   movaps	%xmm8, 0x60(%rsp)
// 0x141f074ac  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141f074b1  0f29742440                     movaps	%xmm6, 0x40(%rsp)
// 0x141f074b6  4c89c7                         movq	%r8, %rdi
// 0x141f074b9  4889ce                         movq	%rcx, %rsi
// Frames: F2
// 0x141f074bc  f3410f107104                   movss	0x4(%r9), %xmm6
// Frames: F3 -> F2
// 0x141f074c2  4c89c1                         movq	%r8, %rcx
// 0x141f074c5  e80618cefe                     callq	0x140be8cd0 <.text+0xbe7cd0>
// 0x141f074ca  84c0                           testb	%al, %al
// 0x141f074cc  0f8402010000                   je	0x141f075d4 <.text+0x1f065d4>
// Frames: F4 -> F2
// 0x141f074d2  4889f9                         movq	%rdi, %rcx
// 0x141f074d5  e8d679bbfe                     callq	0x140abeeb0 <.text+0xabdeb0>
// 0x141f074da  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f074df  488d4c2420                     leaq	0x20(%rsp), %rcx
// 0x141f074e4  e887401cfe                     callq	0x1400cb570 <.text+0xca570>
// 0x141f074e9  660f6ec0                       movd	%eax, %xmm0
// 0x141f074ed  48c1e820                       shrq	$0x20, %rax
// 0x141f074f1  660f6ec8                       movd	%eax, %xmm1
// Frames: F5 -> F2
// 0x141f074f5  660f6fd0                       movdqa	%xmm0, %xmm2
// 0x141f074f9  f30f5ed1                       divss	%xmm1, %xmm2
// 0x141f074fd  f30f5ec8                       divss	%xmm0, %xmm1
// Frames: F6 -> F5 -> F2
// 0x141f07501  0f28c2                         movaps	%xmm2, %xmm0
// 0x141f07504  f30fc2c203                     cmpunordss	%xmm2, %xmm0
// 0x141f07509  0f28d8                         movaps	%xmm0, %xmm3
// 0x141f0750c  0f54d9                         andps	%xmm1, %xmm3
// 0x141f0750f  f30f5fca                       maxss	%xmm2, %xmm1
// 0x141f07513  0f55c1                         andnps	%xmm1, %xmm0
// 0x141f07516  0f56c3                         orps	%xmm3, %xmm0
// 0x141f07519  f30f100db7945001               movss	0x15094b7(%rip), %xmm1  # 0x1434109d8
// Frames: F7 -> F2
// 0x141f07521  0f2ec8                         ucomiss	%xmm0, %xmm1
// 0x141f07524  0f86aa000000                   jbe	0x141f075d4 <.text+0x1f065d4>
// Frames: F8 -> F2
// 0x141f0752a  488d4c2434                     leaq	0x34(%rsp), %rcx
// 0x141f0752f  4889fa                         movq	%rdi, %rdx
// 0x141f07532  e89919cefe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// Frames: F9 -> F2
// 0x141f07537  4889f9                         movq	%rdi, %rcx
// 0x141f0753a  e8f10fcefe                     callq	0x140be8530 <.text+0xbe7530>
// 0x141f0753f  f30f1144242c                   movss	%xmm0, 0x2c(%rsp)
// 0x141f07545  f30f114c2430                   movss	%xmm1, 0x30(%rsp)
// 0x141f0754b  488d4c2420                     leaq	0x20(%rsp), %rcx
// 0x141f07550  488d54242c                     leaq	0x2c(%rsp), %rdx
// 0x141f07555  e80690eefe                     callq	0x140df0560 <.text+0xdef560>
// Frames: F10 -> F8 -> F2
// 0x141f0755a  f30f107c2434                   movss	0x34(%rsp), %xmm7
// Frames: F11 -> F10 -> F8 -> F2
// 0x141f07560  f30f587c2420                   addss	0x20(%rsp), %xmm7
// Frames: F12 -> F8 -> F2
// 0x141f07566  f20f10442438                   movsd	0x38(%rsp), %xmm0
// 0x141f0756c  f2440f10442424                 movsd	0x24(%rsp), %xmm8
// Frames: F11 -> F12 -> F8 -> F2
// 0x141f07573  440f58c0                       addps	%xmm0, %xmm8
// Frames: F13 -> F2
// 0x141f07577  4889f9                         movq	%rdi, %rcx
// 0x141f0757a  e8c117cefe                     callq	0x140be8d40 <.text+0xbe7d40>
// Frames: F14 -> F15 -> F13 -> F2
// 0x141f0757f  f30f12c0                       movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
// 0x141f07583  f30f100d99d4ff00               movss	0xffd499(%rip), %xmm1   # 0x142f04a24
// 0x141f0758b  0f59c8                         mulps	%xmm0, %xmm1
// Frames: F11 -> F10 -> F8 -> F2
// 0x141f0758e  f30f16c1                       movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
// 0x141f07592  f30f58c7                       addss	%xmm7, %xmm0
// Frames: F11 -> F12 -> F8 -> F2
// 0x141f07596  410f58c8                       addps	%xmm8, %xmm1
// Frames: F16 -> F17 -> F18 -> F2
// 0x141f0759a  0f57d2                         xorps	%xmm2, %xmm2
// 0x141f0759d  f30f10d0                       movss	%xmm0, %xmm2            # xmm2 = xmm0[0],xmm2[1,2,3]
// 0x141f075a1  0fc6d14c                       shufps	$0x4c, %xmm1, %xmm2     # xmm2 = xmm2[0,3],xmm1[0,1]
// 0x141f075a5  0fc6d278                       shufps	$0x78, %xmm2, %xmm2     # xmm2 = xmm2[0,2,3,1]
// Frames: F19 -> F2
// 0x141f075a9  f30f10472c                     movss	0x2c(%rdi), %xmm0
// 0x141f075ae  0f295610                       movaps	%xmm2, 0x10(%rsi)
// 0x141f075b2  f30f117620                     movss	%xmm6, 0x20(%rsi)
// 0x141f075b7  0f280d92480f01                 movaps	0x10f4892(%rip), %xmm1  # 0x142ffbe50
// 0x141f075be  f30f10c8                       movss	%xmm0, %xmm1            # xmm1 = xmm0[0],xmm1[1,2,3]
// 0x141f075c2  0f134e24                       movlps	%xmm1, 0x24(%rsi)
// 0x141f075c6  c7462c00000000                 movl	$0x0, 0x2c(%rsi)
// 0x141f075cd  b801000000                     movl	$0x1, %eax
// 0x141f075d2  eb02                           jmp	0x141f075d6 <.text+0x1f065d6>
// 0x141f075d4  31c0                           xorl	%eax, %eax
// Frames: F20
// 0x141f075d6  488906                         movq	%rax, (%rsi)
// 0x141f075d9  48c7460800000000               movq	$0x0, 0x8(%rsi)
// 0x141f075e1  4889f0                         movq	%rsi, %rax
// 0x141f075e4  0f28742440                     movaps	0x40(%rsp), %xmm6
// 0x141f075e9  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f075ee  440f28442460                   movaps	0x60(%rsp), %xmm8
// 0x141f075f4  4883c478                       addq	$0x78, %rsp
// 0x141f075f8  5f                             popq	%rdi
// 0x141f075f9  5e                             popq	%rsi
// 0x141f075fa  c3                             retq
//
// RANGE rva_281a950_len_6d7: RVA 0x281a950, 1751 PDB bytes; 443 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_281a950_len_6d7.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3750:268: system_roof::visual::place_spires::place_roof_spires
// Frames: F21
// 0x14281a950  55                             pushq	%rbp
// 0x14281a951  4157                           pushq	%r15
// 0x14281a953  4156                           pushq	%r14
// 0x14281a955  4155                           pushq	%r13
// 0x14281a957  4154                           pushq	%r12
// 0x14281a959  56                             pushq	%rsi
// 0x14281a95a  57                             pushq	%rdi
// 0x14281a95b  53                             pushq	%rbx
// 0x14281a95c  4881ec68010000                 subq	$0x168, %rsp            # imm = 0x168
// 0x14281a963  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14281a96b  48c785e0000000feffffff         movq	$-0x2, 0xe0(%rbp)
// 0x14281a976  4c89c3                         movq	%r8, %rbx
// 0x14281a979  4889d7                         movq	%rdx, %rdi
// 0x14281a97c  4889ce                         movq	%rcx, %rsi
// Frames: F22
// 0x14281a97f  e83c1767fe                     callq	0x140e8c0c0 <.text+0xe8b0c0>
// 0x14281a984  84c0                           testb	%al, %al
// 0x14281a986  8885d0000000                   movb	%al, 0xd0(%rbp)
// 0x14281a98c  0f84f4000000                   je	0x14281aa86 <.text+0x2819a86>
// Frames: F23 -> F22
// 0x14281a992  41be34000000                   movl	$0x34, %r14d
// 0x14281a998  b934000000                     movl	$0x34, %ecx
// 0x14281a99d  e88e060000                     callq	0x14281b030 <.text+0x281a030>
// 0x14281a9a2  4c8d3d82c9d600                 leaq	0xd6c982(%rip), %r15    # 0x14358732b
// 0x14281a9a9  4883f801                       cmpq	$0x1, %rax
// 0x14281a9ad  753a                           jne	0x14281a9e9 <.text+0x28199e9>
// Frames: F24 -> F25 -> F26 -> F27 -> F28 -> F22
// 0x14281a9af  4989d1                         movq	%rdx, %r9
// 0x14281a9b2  4883fa34                       cmpq	$0x34, %rdx
// 0x14281a9b6  0f8721040000                   ja	0x14281addd <.text+0x2819ddd>
// Frames: F28 -> F22
// 0x14281a9bc  4c89c9                         movq	%r9, %rcx
// 0x14281a9bf  e86c060000                     callq	0x14281b030 <.text+0x281a030>
// 0x14281a9c4  a801                           testb	$0x1, %al
// 0x14281a9c6  7421                           je	0x14281a9e9 <.text+0x28199e9>
// Frames: F29 -> F22
// 0x14281a9c8  4c8d4202                       leaq	0x2(%rdx), %r8
// Frames: F24 -> F30 -> F31 -> F27 -> F29 -> F22
// 0x14281a9cc  4983f834                       cmpq	$0x34, %r8
// 0x14281a9d0  0f87bf040000                   ja	0x14281ae95 <.text+0x2819e95>
// Frames: F32 -> F33 -> F34 -> F31 -> F27 -> F29 -> F22
// 0x14281a9d6  41be32000000                   movl	$0x32, %r14d
// 0x14281a9dc  4929d6                         subq	%rdx, %r14
// Frames: F35 -> F36 -> F33 -> F34 -> F31 -> F27 -> F29 -> F22
// 0x14281a9df  4c8d3d45c9d600                 leaq	0xd6c945(%rip), %r15    # 0x14358732b
// 0x14281a9e6  4d01c7                         addq	%r8, %r15
// 0x14281a9e9  4c8d0525c8d600                 leaq	0xd6c825(%rip), %r8     # 0x143587215
// 0x14281a9f0  4c8d25f0c7d600                 leaq	0xd6c7f0(%rip), %r12    # 0x1435871e7
// 0x14281a9f7  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F37 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa00  410fbe40ff                     movsbl	-0x1(%r8), %eax
// 0x14281aa05  85c0                           testl	%eax, %eax
// 0x14281aa07  7817                           js	0x14281aa20 <.text+0x2819a20>
// Frames: F45 -> F46 -> F47 -> F48 -> F49 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa09  49ffc8                         decq	%r8
// 0x14281aa0c  83f85c                         cmpl	$0x5c, %eax
// 0x14281aa0f  7563                           jne	0x14281aa74 <.text+0x2819a74>
// 0x14281aa11  eb7a                           jmp	0x14281aa8d <.text+0x2819a8d>
// 0x14281aa13  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F50 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa20  410fb648fe                     movzbl	-0x2(%r8), %ecx
// Frames: F51 -> F52 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa25  80f9c0                         cmpb	$-0x40, %cl
// Frames: F52 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa28  7d23                           jge	0x14281aa4d <.text+0x2819a4d>
// Frames: F53 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa2a  410fb650fd                     movzbl	-0x3(%r8), %edx
// Frames: F51 -> F54 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa2f  80fac0                         cmpb	$-0x40, %dl
// Frames: F54 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa32  7d22                           jge	0x14281aa56 <.text+0x2819a56>
// Frames: F55 -> F56 -> F57 -> F58 -> F59 -> F60 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa34  450fb648fc                     movzbl	-0x4(%r8), %r9d
// 0x14281aa39  4983c0fc                       addq	$-0x4, %r8
// Frames: F61 -> F62 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa3d  4183e107                       andl	$0x7, %r9d
// Frames: F63 -> F64 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa41  41c1e106                       shll	$0x6, %r9d
// 0x14281aa45  83e23f                         andl	$0x3f, %edx
// 0x14281aa48  4409ca                         orl	%r9d, %edx
// 0x14281aa4b  eb10                           jmp	0x14281aa5d <.text+0x2819a5d>
// Frames: F52 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa4d  4983c0fe                       addq	$-0x2, %r8
// 0x14281aa51  83e11f                         andl	$0x1f, %ecx
// 0x14281aa54  eb0f                           jmp	0x14281aa65 <.text+0x2819a65>
// 0x14281aa56  4983c0fd                       addq	$-0x3, %r8
// 0x14281aa5a  83e20f                         andl	$0xf, %edx
// Frames: F63 -> F65 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa5d  c1e206                         shll	$0x6, %edx
// 0x14281aa60  83e13f                         andl	$0x3f, %ecx
// 0x14281aa63  09d1                           orl	%edx, %ecx
// Frames: F63 -> F66 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa65  c1e106                         shll	$0x6, %ecx
// 0x14281aa68  243f                           andb	$0x3f, %al
// 0x14281aa6a  0fb6c0                         movzbl	%al, %eax
// 0x14281aa6d  09c8                           orl	%ecx, %eax
// Frames: F45 -> F46 -> F47 -> F48 -> F49 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa6f  83f85c                         cmpl	$0x5c, %eax
// 0x14281aa72  7419                           je	0x14281aa8d <.text+0x2819a8d>
// 0x14281aa74  83f82f                         cmpl	$0x2f, %eax
// 0x14281aa77  7414                           je	0x14281aa8d <.text+0x2819a8d>
// Frames: F67 -> F68 -> F69 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa79  4d39e0                         cmpq	%r12, %r8
// Frames: F70 -> F69 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F44 -> F22
// 0x14281aa7c  7582                           jne	0x14281aa00 <.text+0x2819a00>
// 0x14281aa7e  41bd2e000000                   movl	$0x2e, %r13d
// 0x14281aa84  eb2a                           jmp	0x14281aab0 <.text+0x2819ab0>
// Frames: F22
// 0x14281aa86  31c0                           xorl	%eax, %eax
// 0x14281aa88  e98a000000                     jmp	0x14281ab17 <.text+0x2819b17>
// Frames: F71 -> F22
// 0x14281aa8d  4c8d2553c7d600                 leaq	0xd6c753(%rip), %r12    # 0x1435871e7
// 0x14281aa94  4d29e0                         subq	%r12, %r8
// 0x14281aa97  49ffc0                         incq	%r8
// Frames: F24 -> F30 -> F31 -> F27 -> F71 -> F22
// 0x14281aa9a  4983f82e                       cmpq	$0x2e, %r8
// 0x14281aa9e  0f8716030000                   ja	0x14281adba <.text+0x2819dba>
// Frames: F32 -> F33 -> F34 -> F31 -> F27 -> F71 -> F22
// 0x14281aaa4  41bd2e000000                   movl	$0x2e, %r13d
// 0x14281aaaa  4d29c5                         subq	%r8, %r13
// Frames: F35 -> F36 -> F33 -> F34 -> F31 -> F27 -> F71 -> F22
// 0x14281aaad  4d01c4                         addq	%r8, %r12
// Frames: F72 -> F73 -> F74 -> F75 -> F76 -> F77 -> F22
// 0x14281aab0  e8cb3a67fe                     callq	0x140e8e580 <.text+0xe8d580>
// Frames: F78 -> F72 -> F73 -> F74 -> F75 -> F76 -> F77 -> F22
// 0x14281aab5  80786801                       cmpb	$0x1, 0x68(%rax)
// 0x14281aab9  0f85cb020000                   jne	0x14281ad8a <.text+0x2819d8a>
// Frames: F79 -> F80 -> F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F22
// 0x14281aabf  48833800                       cmpq	$0x0, (%rax)
// 0x14281aac3  0f85b5020000                   jne	0x14281ad7e <.text+0x2819d7e>
// Frames: F84 -> F85 -> F86 -> F80 -> F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F22
// 0x14281aac9  48c700ffffffff                 movq	$-0x1, (%rax)
// Frames: F87 -> F88 -> F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F22
// 0x14281aad0  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F89 -> F82 -> F83 -> F75 -> F76 -> F77 -> F22
// 0x14281aad4  4c896c2420                     movq	%r13, 0x20(%rsp)
// 0x14281aad9  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x14281aae2  48c744242801000000             movq	$0x1, 0x28(%rsp)
// 0x14281aaeb  4c89fa                         movq	%r15, %rdx
// 0x14281aaee  4d89f0                         movq	%r14, %r8
// 0x14281aaf1  4d89e1                         movq	%r12, %r9
// 0x14281aaf4  488985c8000000                 movq	%rax, 0xc8(%rbp)
// 0x14281aafb  e8901967fe                     callq	0x140e8c490 <.text+0xe8b490>
// 0x14281ab00  90                             nop
// Frames: F84 -> F85 -> F90 -> F91 -> F91 -> F82 -> F83 -> F75 -> F76 -> F77 -> F22
// 0x14281ab01  4989c6                         movq	%rax, %r14
// 0x14281ab04  488b85c8000000                 movq	0xc8(%rbp), %rax
// 0x14281ab0b  48ff00                         incq	(%rax)
// Frames: F22
// 0x14281ab0e  4c897528                       movq	%r14, 0x28(%rbp)
// 0x14281ab12  b801000000                     movl	$0x1, %eax
// 0x14281ab17  48894520                       movq	%rax, 0x20(%rbp)
// Frames: F92
// 0x14281ab1b  488b0b                         movq	(%rbx), %rcx
// Frames: F93 -> F94 -> F92
// 0x14281ab1e  488d1583edbf00                 leaq	0xbfed83(%rip), %rdx    # 0x1434198a8
// 0x14281ab25  e8567772ff                     callq	0x141f42280 <.text+0x1f41280>
// 0x14281ab2a  90                             nop
// 0x14281ab2b  4885c0                         testq	%rax, %rax
// 0x14281ab2e  0f8452010000                   je	0x14281ac86 <.text+0x2819c86>
// Frames: F95 -> F94 -> F92
// 0x14281ab34  488b08                         movq	(%rax), %rcx
// 0x14281ab37  488b4008                       movq	0x8(%rax), %rax
// Frames: F94 -> F92
// 0x14281ab3b  ff5018                         callq	*0x18(%rax)
// 0x14281ab3e  90                             nop
// Frames: F92
// 0x14281ab3f  4889c3                         movq	%rax, %rbx
// 0x14281ab42  4885c0                         testq	%rax, %rax
// 0x14281ab45  0f843b010000                   je	0x14281ac86 <.text+0x2819c86>
// Frames: F96 -> F97 -> F98 -> F99 -> F100 -> F101
// 0x14281ab4b  488b07                         movq	(%rdi), %rax
// 0x14281ab4e  488b4f08                       movq	0x8(%rdi), %rcx
// Frames: F102 -> F103 -> F104 -> F105 -> F106 -> F107 -> F108 -> F109 -> F110 -> F100 -> F101
// 0x14281ab52  488b90f8000000                 movq	0xf8(%rax), %rdx
// Frames: F106 -> F107 -> F108 -> F109 -> F110 -> F100 -> F101
// 0x14281ab59  4c8b8000010000                 movq	0x100(%rax), %r8
// Frames: F111 -> F112 -> F113 -> F108 -> F109 -> F110 -> F100 -> F101
// 0x14281ab60  4e8d0482                       leaq	(%rdx,%r8,4), %r8
// Frames: F114 -> F109 -> F110 -> F100 -> F101
// 0x14281ab64  440fb68808010000               movzbl	0x108(%rax), %r9d
// Frames: F115 -> F109 -> F110 -> F100 -> F101
// 0x14281ab6c  0f57c0                         xorps	%xmm0, %xmm0
// 0x14281ab6f  0f1185b0000000                 movups	%xmm0, 0xb0(%rbp)
// 0x14281ab76  0f1185a0000000                 movups	%xmm0, 0xa0(%rbp)
// Frames: F116 -> F117 -> F110 -> F100 -> F101
// 0x14281ab7d  4c8d91f8000000                 leaq	0xf8(%rcx), %r10
// Frames: F118 -> F119 -> F120
// 0x14281ab84  48894d58                       movq	%rcx, 0x58(%rbp)
// Frames: F121 -> F110 -> F100 -> F101
// 0x14281ab88  4881c198010000                 addq	$0x198, %rcx            # imm = 0x198
// Frames: F118 -> F119 -> F120
// 0x14281ab8f  48894d60                       movq	%rcx, 0x60(%rbp)
// 0x14281ab93  4c895568                       movq	%r10, 0x68(%rbp)
// 0x14281ab97  48894570                       movq	%rax, 0x70(%rbp)
// 0x14281ab9b  48895578                       movq	%rdx, 0x78(%rbp)
// 0x14281ab9f  4c898580000000                 movq	%r8, 0x80(%rbp)
// 0x14281aba6  48c7858800000008000000         movq	$0x8, 0x88(%rbp)
// 0x14281abb1  48c7859000000000000000         movq	$0x0, 0x90(%rbp)
// 0x14281abbc  48c7859800000008000000         movq	$0x8, 0x98(%rbp)
// 0x14281abc7  44888dc0000000                 movb	%r9b, 0xc0(%rbp)
// Frames: F122 -> F123 -> F124
// 0x14281abce  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14281abd2  488d5558                       leaq	0x58(%rbp), %rdx
// 0x14281abd6  e8f5417afe                     callq	0x140fbedd0 <.text+0xfbddd0>
// 0x14281abdb  90                             nop
// Frames: F125
// 0x14281abdc  488b0e                         movq	(%rsi), %rcx
// 0x14281abdf  e8ac4826fe                     callq	0x140a7f490 <.text+0xa7e490>
// 0x14281abe4  90                             nop
// 0x14281abe5  488b7548                       movq	0x48(%rbp), %rsi
// 0x14281abe9  4c8b7d50                       movq	0x50(%rbp), %r15
// Frames: F126 -> F127 -> F125
// 0x14281abed  4d85ff                         testq	%r15, %r15
// Frames: F127 -> F125
// 0x14281abf0  746c                           je	0x14281ac5e <.text+0x2819c5e>
// Frames: F125
// 0x14281abf2  4889c7                         movq	%rax, %rdi
// Frames: F128 -> F129 -> F130 -> F131 -> F125
// 0x14281abf5  4889d9                         movq	%rbx, %rcx
// 0x14281abf8  e8f3795ffe                     callq	0x140e125f0 <.text+0xe115f0>
// 0x14281abfd  90                             nop
// Frames: F129 -> F130 -> F131 -> F125
// 0x14281abfe  4c89f9                         movq	%r15, %rcx
// 0x14281ac01  48c1e105                       shlq	$0x5, %rcx
// 0x14281ac05  8b4010                         movl	0x10(%rax), %eax
// 0x14281ac08  4839c1                         cmpq	%rax, %rcx
// 0x14281ac0b  0f87ec010000                   ja	0x14281adfd <.text+0x2819dfd>
// Frames: F132 -> F130 -> F131 -> F125
// 0x14281ac11  48c7451000000000               movq	$0x0, 0x10(%rbp)
// 0x14281ac19  48894d18                       movq	%rcx, 0x18(%rbp)
// 0x14281ac1d  488d4510                       leaq	0x10(%rbp), %rax
// 0x14281ac21  4889442428                     movq	%rax, 0x28(%rsp)
// 0x14281ac26  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x14281ac2b  48c744243800000000             movq	$0x0, 0x38(%rsp)
// 0x14281ac34  48c744243001000000             movq	$0x1, 0x30(%rsp)
// 0x14281ac3d  488d4d58                       leaq	0x58(%rbp), %rcx
// 0x14281ac41  4889fa                         movq	%rdi, %rdx
// 0x14281ac44  4989d8                         movq	%rbx, %r8
// 0x14281ac47  4989f1                         movq	%rsi, %r9
// 0x14281ac4a  e8815f63fe                     callq	0x140e50bd0 <.text+0xe4fbd0>
// 0x14281ac4f  90                             nop
// Frames: F133 -> F134 -> F130 -> F131 -> F125
// 0x14281ac50  488b4558                       movq	0x58(%rbp), %rax
// 0x14281ac54  4883f80b                       cmpq	$0xb, %rax
// 0x14281ac58  0f85a6010000                   jne	0x14281ae04 <.text+0x2819e04>
// Frames: F135 -> F136 -> F137 -> F125
// 0x14281ac5e  4c897b08                       movq	%r15, 0x8(%rbx)
// Frames: F138 -> F139 -> F91 -> F91 -> F140
// 0x14281ac62  488b5540                       movq	0x40(%rbp), %rdx
// Frames: F141 -> F138 -> F139 -> F91 -> F91 -> F140
// 0x14281ac66  4885d2                         testq	%rdx, %rdx
// 0x14281ac69  0f84bc000000                   je	0x14281ad2b <.text+0x2819d2b>
// Frames: F142 -> F143 -> F138 -> F139 -> F91 -> F91 -> F140
// 0x14281ac6f  48c1e205                       shlq	$0x5, %rdx
// Frames: F144 -> F145 -> F146 -> F147 -> F148 -> F139 -> F91 -> F91 -> F140
// 0x14281ac73  41b810000000                   movl	$0x10, %r8d
// 0x14281ac79  4889f1                         movq	%rsi, %rcx
// 0x14281ac7c  e86face8fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x14281ac81  e9a5000000                     jmp	0x14281ad2b <.text+0x2819d2b>
// Frames: F149 -> F150 -> F151 -> F152
// 0x14281ac86  488b0503424701                 movq	0x1474203(%rip), %rax   # 0x143c8ee90
// 0x14281ac8d  488b00                         movq	(%rax), %rax
// Frames: F153 -> F154 -> F152
// 0x14281ac90  4883f802                       cmpq	$0x2, %rax
// Frames: F152
// 0x14281ac94  0f8291000000                   jb	0x14281ad2b <.text+0x2819d2b>
// Frames: F155 -> F156 -> F157 -> F158 -> F152
// 0x14281ac9a  48c7455800000000               movq	$0x0, 0x58(%rbp)
// 0x14281aca2  488d05b4c5d600                 leaq	0xd6c5b4(%rip), %rax    # 0x14358725d
// 0x14281aca9  48894560                       movq	%rax, 0x60(%rbp)
// 0x14281acad  48c7456821000000               movq	$0x21, 0x68(%rbp)
// 0x14281acb5  48c7457000000000               movq	$0x0, 0x70(%rbp)
// 0x14281acbd  488d0df4c4d600                 leaq	0xd6c4f4(%rip), %rcx    # 0x1435871b8
// 0x14281acc4  48894d78                       movq	%rcx, 0x78(%rbp)
// 0x14281acc8  48c785800000002e000000         movq	$0x2e, 0x80(%rbp)
// 0x14281acd3  48c7858800000002000000         movq	$0x2, 0x88(%rbp)
// 0x14281acde  48898590000000                 movq	%rax, 0x90(%rbp)
// 0x14281ace5  48c7859800000021000000         movq	$0x21, 0x98(%rbp)
// 0x14281acf0  48b8010000003a000000           movabsq	$0x3a00000001, %rax # imm = 0x3A00000001
// 0x14281acfa  488985a0000000                 movq	%rax, 0xa0(%rbp)
// 0x14281ad01  488d0528c5d600                 leaq	0xd6c528(%rip), %rax    # 0x143587230
// 0x14281ad08  488985a8000000                 movq	%rax, 0xa8(%rbp)
// 0x14281ad0f  48c785b00000005b000000         movq	$0x5b, 0xb0(%rbp)
// Frames: F157 -> F158 -> F152
// 0x14281ad1a  488d8ddf000000                 leaq	0xdf(%rbp), %rcx
// 0x14281ad21  488d5558                       leaq	0x58(%rbp), %rdx
// 0x14281ad25  e856f170fe                     callq	0x140f29e80 <.text+0xf28e80>
// 0x14281ad2a  90                             nop
// Frames: F91 -> F159
// 0x14281ad2b  80bdd000000000                 cmpb	$0x0, 0xd0(%rbp)
// 0x14281ad32  7436                           je	0x14281ad6a <.text+0x2819d6a>
// Frames: F72 -> F73 -> F74 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ad34  e8473867fe                     callq	0x140e8e580 <.text+0xe8d580>
// Frames: F78 -> F72 -> F73 -> F74 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ad39  80786801                       cmpb	$0x1, 0x68(%rax)
// 0x14281ad3d  7560                           jne	0x14281ad9f <.text+0x2819d9f>
// Frames: F79 -> F80 -> F81 -> F82 -> F83 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ad3f  48833800                       cmpq	$0x0, (%rax)
// 0x14281ad43  7539                           jne	0x14281ad7e <.text+0x2819d7e>
// Frames: F84 -> F85 -> F86 -> F80 -> F81 -> F82 -> F83 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ad45  48c700ffffffff                 movq	$-0x1, (%rax)
// Frames: F87 -> F88 -> F81 -> F82 -> F83 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ad4c  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F161 -> F82 -> F83 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ad50  4c89f2                         movq	%r14, %rdx
// 0x14281ad53  488985d0000000                 movq	%rax, 0xd0(%rbp)
// 0x14281ad5a  e8911b67fe                     callq	0x140e8c8f0 <.text+0xe8b8f0>
// 0x14281ad5f  90                             nop
// Frames: F84 -> F85 -> F90 -> F91 -> F91 -> F82 -> F83 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ad60  488b85d0000000                 movq	0xd0(%rbp), %rax
// 0x14281ad67  48ff00                         incq	(%rax)
// Frames: F159
// 0x14281ad6a  4881c468010000                 addq	$0x168, %rsp            # imm = 0x168
// 0x14281ad71  5b                             popq	%rbx
// 0x14281ad72  5f                             popq	%rdi
// 0x14281ad73  5e                             popq	%rsi
// 0x14281ad74  415c                           popq	%r12
// 0x14281ad76  415d                           popq	%r13
// 0x14281ad78  415e                           popq	%r14
// 0x14281ad7a  415f                           popq	%r15
// 0x14281ad7c  5d                             popq	%rbp
// 0x14281ad7d  c3                             retq
// 0x14281ad7e  488d0dcbd4d600                 leaq	0xd6d4cb(%rip), %rcx    # 0x143588250
// 0x14281ad85  e8d6675900                     callq	0x142db1560 <.text+0x2db0560>
// Frames: F162 -> F72 -> F73 -> F74 -> F75 -> F76 -> F77 -> F22
// 0x14281ad8a  4889c1                         movq	%rax, %rcx
// 0x14281ad8d  31d2                           xorl	%edx, %edx
// 0x14281ad8f  e89c7c5f00                     callq	0x142e12a30 <.text+0x2e11a30>
// Frames: F163 -> F74 -> F75 -> F76 -> F77 -> F22
// 0x14281ad94  4885c0                         testq	%rax, %rax
// 0x14281ad97  0f8522fdffff                   jne	0x14281aabf <.text+0x2819abf>
// 0x14281ad9d  eb0f                           jmp	0x14281adae <.text+0x2819dae>
// Frames: F162 -> F72 -> F73 -> F74 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ad9f  4889c1                         movq	%rax, %rcx
// 0x14281ada2  31d2                           xorl	%edx, %edx
// 0x14281ada4  e8877c5f00                     callq	0x142e12a30 <.text+0x2e11a30>
// Frames: F163 -> F74 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281ada9  4885c0                         testq	%rax, %rax
// 0x14281adac  7591                           jne	0x14281ad3f <.text+0x2819d3f>
// 0x14281adae  488d0d2bd4d600                 leaq	0xd6d42b(%rip), %rcx    # 0x1435881e0
// 0x14281adb5  e8a65f5900                     callq	0x142db0d60 <.text+0x2dafd60>
// Frames: F164 -> F27 -> F71 -> F22
// 0x14281adba  488d0507c6d600                 leaq	0xd6c607(%rip), %rax    # 0x1435873c8
// 0x14281adc1  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14281adc6  488d0d1ac4d600                 leaq	0xd6c41a(%rip), %rcx    # 0x1435871e7
// 0x14281adcd  ba2e000000                     movl	$0x2e, %edx
// 0x14281add2  41b92e000000                   movl	$0x2e, %r9d
// 0x14281add8  e863675900                     callq	0x142db1540 <.text+0x2db0540>
// Frames: F165 -> F27 -> F28 -> F22
// 0x14281addd  488d0504c6d600                 leaq	0xd6c604(%rip), %rax    # 0x1435873e8
// 0x14281ade4  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14281ade9  488d0d3bc5d600                 leaq	0xd6c53b(%rip), %rcx    # 0x14358732b
// 0x14281adf0  ba34000000                     movl	$0x34, %edx
// 0x14281adf5  4531c0                         xorl	%r8d, %r8d
// 0x14281adf8  e843675900                     callq	0x142db1540 <.text+0x2db0540>
// Frames: F129 -> F130 -> F131 -> F125
// 0x14281adfd  b80b000000                     movl	$0xb, %eax
// 0x14281ae02  eb31                           jmp	0x14281ae35 <.text+0x2819e35>
// Frames: F166 -> F134 -> F130 -> F131 -> F125
// 0x14281ae04  488b8da0000000                 movq	0xa0(%rbp), %rcx
// 0x14281ae0b  48894d00                       movq	%rcx, (%rbp)
// 0x14281ae0f  0f104560                       movups	0x60(%rbp), %xmm0
// 0x14281ae13  0f104d70                       movups	0x70(%rbp), %xmm1
// 0x14281ae17  0f109580000000                 movups	0x80(%rbp), %xmm2
// 0x14281ae1e  0f109d90000000                 movups	0x90(%rbp), %xmm3
// 0x14281ae25  0f295df0                       movaps	%xmm3, -0x10(%rbp)
// 0x14281ae29  0f2955e0                       movaps	%xmm2, -0x20(%rbp)
// 0x14281ae2d  0f294dd0                       movaps	%xmm1, -0x30(%rbp)
// 0x14281ae31  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// Frames: F167 -> F125
// 0x14281ae35  48894558                       movq	%rax, 0x58(%rbp)
// 0x14281ae39  0f2845c0                       movaps	-0x40(%rbp), %xmm0
// 0x14281ae3d  0f284dd0                       movaps	-0x30(%rbp), %xmm1
// 0x14281ae41  0f2855e0                       movaps	-0x20(%rbp), %xmm2
// 0x14281ae45  0f285df0                       movaps	-0x10(%rbp), %xmm3
// 0x14281ae49  0f114560                       movups	%xmm0, 0x60(%rbp)
// 0x14281ae4d  0f114d70                       movups	%xmm1, 0x70(%rbp)
// 0x14281ae51  0f119580000000                 movups	%xmm2, 0x80(%rbp)
// 0x14281ae58  0f119d90000000                 movups	%xmm3, 0x90(%rbp)
// 0x14281ae5f  488b4500                       movq	(%rbp), %rax
// 0x14281ae63  488985a0000000                 movq	%rax, 0xa0(%rbp)
// 0x14281ae6a  488d05a7c3d600                 leaq	0xd6c3a7(%rip), %rax    # 0x143587218
// 0x14281ae71  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14281ae76  488d0d83c4d600                 leaq	0xd6c483(%rip), %rcx    # 0x143587300
// 0x14281ae7d  4c8d0d5cc4d600                 leaq	0xd6c45c(%rip), %r9     # 0x1435872e0
// 0x14281ae84  4c8d4558                       leaq	0x58(%rbp), %r8
// 0x14281ae88  ba2b000000                     movl	$0x2b, %edx
// 0x14281ae8d  e8ae675900                     callq	0x142db1640 <.text+0x2db0640>
// 0x14281ae92  90                             nop
// 0x14281ae93  0f0b                           ud2
// Frames: F164 -> F27 -> F29 -> F22
// 0x14281ae95  488d0564c5d600                 leaq	0xd6c564(%rip), %rax    # 0x143587400
// 0x14281ae9c  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14281aea1  488d0d83c4d600                 leaq	0xd6c483(%rip), %rcx    # 0x14358732b
// 0x14281aea8  ba34000000                     movl	$0x34, %edx
// 0x14281aead  41b934000000                   movl	$0x34, %r9d
// 0x14281aeb3  e888665900                     callq	0x142db1540 <.text+0x2db0540>
// 0x14281aeb8  cc                             int3
// 0x14281aeb9  0f1f8000000000                 nopl	(%rax)
// 0x14281aec0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14281aec5  55                             pushq	%rbp
// 0x14281aec6  4157                           pushq	%r15
// 0x14281aec8  4156                           pushq	%r14
// 0x14281aeca  4155                           pushq	%r13
// 0x14281aecc  4154                           pushq	%r12
// 0x14281aece  56                             pushq	%rsi
// 0x14281aecf  57                             pushq	%rdi
// 0x14281aed0  53                             pushq	%rbx
// 0x14281aed1  4883ec48                       subq	$0x48, %rsp
// 0x14281aed5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14281aedc  488b85c8000000                 movq	0xc8(%rbp), %rax
// Frames: F84 -> F85 -> F90 -> F91 -> F91 -> F82 -> F83 -> F75 -> F76 -> F77 -> F22
// 0x14281aee3  48ff00                         incq	(%rax)
// Frames: F82 -> F83 -> F75 -> F76 -> F77 -> F22
// 0x14281aee6  4883c448                       addq	$0x48, %rsp
// 0x14281aeea  5b                             popq	%rbx
// 0x14281aeeb  5f                             popq	%rdi
// 0x14281aeec  5e                             popq	%rsi
// 0x14281aeed  415c                           popq	%r12
// 0x14281aeef  415d                           popq	%r13
// 0x14281aef1  415e                           popq	%r14
// 0x14281aef3  415f                           popq	%r15
// 0x14281aef5  5d                             popq	%rbp
// 0x14281aef6  c3                             retq
// 0x14281aef7  660f1f840000000000             nopw	(%rax,%rax)
// 0x14281af00  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14281af05  55                             pushq	%rbp
// 0x14281af06  4157                           pushq	%r15
// 0x14281af08  4156                           pushq	%r14
// 0x14281af0a  4155                           pushq	%r13
// 0x14281af0c  4154                           pushq	%r12
// 0x14281af0e  56                             pushq	%rsi
// 0x14281af0f  57                             pushq	%rdi
// 0x14281af10  53                             pushq	%rbx
// 0x14281af11  4883ec48                       subq	$0x48, %rsp
// 0x14281af15  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F91 -> F159
// 0x14281af1c  80bdd000000000                 cmpb	$0x0, 0xd0(%rbp)
// 0x14281af23  7421                           je	0x14281af46 <.text+0x2819f46>
// 0x14281af25  488d4528                       leaq	0x28(%rbp), %rax
// 0x14281af29  48894538                       movq	%rax, 0x38(%rbp)
// 0x14281af2d  488d4538                       leaq	0x38(%rbp), %rax
// 0x14281af31  48894530                       movq	%rax, 0x30(%rbp)
// Frames: F76 -> F160 -> F91 -> F91 -> F159
// 0x14281af35  488d0d2cc4d600                 leaq	0xd6c42c(%rip), %rcx    # 0x143587368
// 0x14281af3c  488d5530                       leaq	0x30(%rbp), %rdx
// 0x14281af40  e8eb5d0000                     callq	0x142820d30 <.text+0x281fd30>
// 0x14281af45  90                             nop
// Frames: F21
// 0x14281af46  4883c448                       addq	$0x48, %rsp
// 0x14281af4a  5b                             popq	%rbx
// 0x14281af4b  5f                             popq	%rdi
// 0x14281af4c  5e                             popq	%rsi
// 0x14281af4d  415c                           popq	%r12
// 0x14281af4f  415d                           popq	%r13
// 0x14281af51  415e                           popq	%r14
// 0x14281af53  415f                           popq	%r15
// 0x14281af55  5d                             popq	%rbp
// 0x14281af56  c3                             retq
// 0x14281af57  660f1f840000000000             nopw	(%rax,%rax)
// 0x14281af60  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14281af65  55                             pushq	%rbp
// 0x14281af66  4157                           pushq	%r15
// 0x14281af68  4156                           pushq	%r14
// 0x14281af6a  4155                           pushq	%r13
// 0x14281af6c  4154                           pushq	%r12
// 0x14281af6e  56                             pushq	%rsi
// 0x14281af6f  57                             pushq	%rdi
// 0x14281af70  53                             pushq	%rbx
// 0x14281af71  4883ec48                       subq	$0x48, %rsp
// 0x14281af75  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F138 -> F139 -> F91 -> F91 -> F140
// 0x14281af7c  488b5540                       movq	0x40(%rbp), %rdx
// Frames: F141 -> F138 -> F139 -> F91 -> F91 -> F140
// 0x14281af80  4885d2                         testq	%rdx, %rdx
// 0x14281af83  7414                           je	0x14281af99 <.text+0x2819f99>
// Frames: F138 -> F139 -> F91 -> F91 -> F140
// 0x14281af85  488b4d48                       movq	0x48(%rbp), %rcx
// Frames: F142 -> F143 -> F138 -> F139 -> F91 -> F91 -> F140
// 0x14281af89  48c1e205                       shlq	$0x5, %rdx
// Frames: F144 -> F145 -> F146 -> F147 -> F148 -> F139 -> F91 -> F91 -> F140
// 0x14281af8d  41b810000000                   movl	$0x10, %r8d
// 0x14281af93  e858a9e8fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x14281af98  90                             nop
// Frames: F140
// 0x14281af99  4883c448                       addq	$0x48, %rsp
// 0x14281af9d  5b                             popq	%rbx
// 0x14281af9e  5f                             popq	%rdi
// 0x14281af9f  5e                             popq	%rsi
// 0x14281afa0  415c                           popq	%r12
// 0x14281afa2  415d                           popq	%r13
// 0x14281afa4  415e                           popq	%r14
// 0x14281afa6  415f                           popq	%r15
// 0x14281afa8  5d                             popq	%rbp
// 0x14281afa9  c3                             retq
// 0x14281afaa  660f1f440000                   nopw	(%rax,%rax)
// 0x14281afb0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14281afb5  55                             pushq	%rbp
// 0x14281afb6  4157                           pushq	%r15
// 0x14281afb8  4156                           pushq	%r14
// 0x14281afba  4155                           pushq	%r13
// 0x14281afbc  4154                           pushq	%r12
// 0x14281afbe  56                             pushq	%rsi
// 0x14281afbf  57                             pushq	%rdi
// 0x14281afc0  53                             pushq	%rbx
// 0x14281afc1  4883ec48                       subq	$0x48, %rsp
// 0x14281afc5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14281afcc  488b85d0000000                 movq	0xd0(%rbp), %rax
// Frames: F84 -> F85 -> F90 -> F91 -> F91 -> F82 -> F83 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281afd3  48ff00                         incq	(%rax)
// Frames: F82 -> F83 -> F75 -> F76 -> F160 -> F91 -> F91 -> F159
// 0x14281afd6  4883c448                       addq	$0x48, %rsp
// 0x14281afda  5b                             popq	%rbx
// 0x14281afdb  5f                             popq	%rdi
// 0x14281afdc  5e                             popq	%rsi
// 0x14281afdd  415c                           popq	%r12
// 0x14281afdf  415d                           popq	%r13
// 0x14281afe1  415e                           popq	%r14
// 0x14281afe3  415f                           popq	%r15
// 0x14281afe5  5d                             popq	%rbp
// 0x14281afe6  c3                             retq
// 0x14281afe7  660f1f840000000000             nopw	(%rax,%rax)
// 0x14281aff0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14281aff5  55                             pushq	%rbp
// 0x14281aff6  4157                           pushq	%r15
// 0x14281aff8  4156                           pushq	%r14
// 0x14281affa  4155                           pushq	%r13
// 0x14281affc  4154                           pushq	%r12
// 0x14281affe  56                             pushq	%rsi
// 0x14281afff  57                             pushq	%rdi
// 0x14281b000  53                             pushq	%rbx
// 0x14281b001  4883ec48                       subq	$0x48, %rsp
// 0x14281b005  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14281b00c  488d4d58                       leaq	0x58(%rbp), %rcx
// Frames: F167 -> F125
// 0x14281b010  e8ab5f05fe                     callq	0x140870fc0 <.text+0x86ffc0>
// 0x14281b015  90                             nop
// 0x14281b016  4883c448                       addq	$0x48, %rsp
// 0x14281b01a  5b                             popq	%rbx
// 0x14281b01b  5f                             popq	%rdi
// 0x14281b01c  5e                             popq	%rsi
// 0x14281b01d  415c                           popq	%r12
// 0x14281b01f  415d                           popq	%r13
// 0x14281b021  415e                           popq	%r14
// 0x14281b023  415f                           popq	%r15
// 0x14281b025  5d                             popq	%rbp
// 0x14281b026  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:297:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::h6ca21e48821c27db
// F2: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:298:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::h6ca21e48821c27db
// F3: crates/systems/roof/src/visual/place_spires.rs:29:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F4: crates/systems/roof/src/visual/place_spires.rs:30:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F5: crates/systems/roof/src/visual/place_spires.rs:32:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F6: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:992:0 core::f32::impl$0::max
// F7: crates/systems/roof/src/visual/place_spires.rs:35:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F8: crates/systems/roof/src/visual/place_spires.rs:36:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F9: crates/systems/roof/src/visual/place_spires.rs:37:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1591:0 glam::f32::vec3::impl$34::add
// F11: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F12: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1592:0 glam::f32::vec3::impl$34::add
// F13: crates/systems/roof/src/visual/place_spires.rs:38:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:352:0 core::ops::arith::impl$141::mul
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1503:0 glam::f32::vec3::impl$24::mul
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs:111:0 glam::f32::sse2::vec4::Vec4::new
// F17: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:182:0 glam::f32::vec3::Vec3::extend
// F18: crates/systems/roof/src/visual/place_spires.rs:40:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F19: crates/systems/roof/src/visual/place_spires.rs:39:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F20: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:299:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::h6ca21e48821c27db
// F21: crates/systems/roof/src/visual/place_spires.rs:18:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F22: crates/systems/roof/src/visual/place_spires.rs:23:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F23: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F24: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:382:0 core::str::impl$0::is_char_boundary
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:439:0 core::str::traits::impl$10::get
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:470:0 core::str::traits::impl$10::index
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:0 core::str::traits::impl$4::index
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:508:0 core::str::traits::impl$11::get
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:541:0 core::str::traits::impl$11::index
// F32: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:211:0 core::str::traits::impl$7::get_unchecked
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:530:0 core::str::traits::impl$11::get_unchecked
// F34: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:511:0 core::str::traits::impl$11::get
// F35: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:863:0 core::ptr::const_ptr::impl$0::add
// F36: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:212:0 core::str::traits::impl$7::get_unchecked
// F37: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F38: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/iter.rs:128:0 core::str::iter::impl$2::next_back
// F39: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/iter.rs:216:0 core::str::iter::impl$6::next_back
// F40: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:714:0 core::str::pattern::impl$11::next_back
// F41: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:317:0 core::str::pattern::ReverseSearcher::next_match_back
// F42: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:799:0 core::str::pattern::impl$22::next_match_back
// F43: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:1531:0 core::str::impl$0::rfind
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:332:0 core::slice::iter::impl$171::any
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/cmp.rs:417:0 core::slice::cmp::impl$49::slice_contains
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:2593:0 core::slice::impl$0::contains
// F48: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:660:0 core::str::pattern::impl$8::matches
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:717:0 core::str::pattern::impl$11::next_back
// F50: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:93:0 core::str::validations::next_code_point_reverse
// F51: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:24:0 core::str::validations::utf8_is_cont_byte
// F52: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:95:0 core::str::validations::next_code_point_reverse
// F53: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:98:0 core::str::validations::next_code_point_reverse
// F54: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:100:0 core::str::validations::next_code_point_reverse
// F55: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:583:0 core::ptr::non_null::NonNull::offset
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:739:0 core::ptr::non_null::NonNull::sub
// F57: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:132:0 core::slice::iter::Iter::pre_dec_end
// F58: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:84:0 core::slice::iter::Iter::next_back_unchecked
// F59: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:447:0 core::slice::iter::impl$172::next_back
// F60: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:103:0 core::str::validations::next_code_point_reverse
// F61: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:11:0 core::str::validations::utf8_first_byte
// F62: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F63: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:17:0 core::str::validations::utf8_acc_cont_byte
// F64: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F65: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:107:0 core::str::validations::next_code_point_reverse
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:109:0 core::str::validations::next_code_point_reverse
// F67: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F68: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$172::next_back
// F69: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:83:0 core::str::validations::next_code_point_reverse
// F70: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$172::next_back
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F72: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/mod.rs:97:0 puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
// F73: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F74: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:512:0 std::thread::local::LocalKey::try_with
// F75: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:477:0 std::thread::local::LocalKey::with
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F78: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/lazy.rs:59:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F79: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2066:0 core::cell::BorrowRefMut::new
// F80: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1256:0 core::cell::RefCell::try_borrow_mut
// F81: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1222:0 core::cell::RefCell::borrow_mut
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F83: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:513:0 std::thread::local::LocalKey::try_with
// F84: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:902:0 core::mem::replace
// F85: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:513:0 core::cell::Cell::replace
// F86: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2068:0 core::cell::BorrowRefMut::new
// F87: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2447:0 core::cell::UnsafeCell::get
// F88: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1264:0 core::cell::RefCell::try_borrow_mut
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F90: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2055:0 core::cell::impl$52::drop
// F91: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:805:0 core::ptr::drop_in_place
// F92: crates/systems/roof/src/visual/place_spires.rs:25:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F93: crates/country-core/src/asset_libraries/ssbo_library.rs:248:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F94: crates/country-core/src/asset_libraries/ssbo_library.rs:284:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
// F95: crates/country-core/src/asset_libraries/ssbo_library.rs:250:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:528:0 bevy_ecs::system::query::Query::new
// F97: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:643:0 bevy_ecs::system::query::Query::copy_unsafe
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:625:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F99: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:548:0 bevy_ecs::system::query::Query::as_readonly
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:674:0 bevy_ecs::system::query::Query::iter
// F101: crates/systems/roof/src/visual/place_spires.rs:27:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F102: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F103: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F104: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F105: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F106: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F107: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2421:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:53:0 bevy_ecs::query::iter::QueryIter::new
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:2510:0 bevy_ecs::system::query::Query::as_readonly
// F111: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F112: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F113: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 alloc::vec::impl$9::deref
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2422:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2416:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:267:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:51:0 bevy_ecs::query::iter::QueryIter::new
// F118: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter_map.rs:24:0 core::iter::adapters::filter_map::FilterMap::new
// F119: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:962:0 core::iter::traits::iterator::Iterator::filter_map
// F120: crates/systems/roof/src/visual/place_spires.rs:28:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:50:0 bevy_ecs::query::iter::QueryIter::new
// F122: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3865:0 alloc::vec::impl$16::from_iter
// F123: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2064:0 core::iter::traits::iterator::Iterator::collect
// F124: crates/systems/roof/src/visual/place_spires.rs:53:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F125: crates/systems/roof/src/visual/place_spires.rs:55:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F126: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:137:0 core::slice::impl$0::is_empty
// F127: crates/country-core/src/render/ssbo.rs:367:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F128: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:110:0 rhapsody::raw::buffer::BufferHandleExt::desc
// F129: crates/country-core/src/render/ssbo.rs:230:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F130: crates/country-core/src/render/ssbo.rs:174:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write
// F131: crates/country-core/src/render/ssbo.rs:368:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F132: crates/country-core/src/render/ssbo.rs:232:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F133: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:966:0 core::result::Result::map_err
// F134: crates/country-core/src/render/ssbo.rs:233:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F135: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3875:0 core::sync::atomic::atomic_store
// F136: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2872:0 core::sync::atomic::AtomicUsize::store
// F137: crates/country-core/src/render/ssbo.rs:372:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F138: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:872:0 alloc::raw_vec::RawVecInner::deallocate
// F139: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:423:0 alloc::raw_vec::impl$3::drop
// F140: crates/systems/roof/src/visual/place_spires.rs:56:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F141: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:632:0 alloc::raw_vec::RawVecInner::current_memory
// F142: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:1232:0 core::num::impl$11::unchecked_mul
// F143: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:640:0 alloc::raw_vec::RawVecInner::current_memory
// F144: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:115:0 alloc::alloc::dealloc
// F145: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:209:0 alloc::alloc::Global::deallocate_impl_runtime
// F146: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:324:0 alloc::alloc::Global::deallocate_impl
// F147: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:442:0 alloc::alloc::impl$1::deallocate
// F148: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:874:0 alloc::raw_vec::RawVecInner::deallocate
// F149: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3890:0 core::sync::atomic::atomic_load
// F150: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2844:0 core::sync::atomic::AtomicUsize::load
// F151: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:1328:0 log::max_level
// F152: crates/systems/roof/src/visual/place_spires.rs:58:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F153: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:662:0 core::option::Option::is_some_and
// F154: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:1429:0 core::cmp::PartialOrd::le
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:766:0 log::impl$55::clone
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:1034:0 log::RecordBuilder::build
// F157: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:81:0 log::__private_api::log_impl
// F158: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:94:0 log::__private_api::log
// F159: crates/systems/roof/src/visual/place_spires.rs:60:0 system_roof::visual::place_spires::place_roof_spires::h24cbcca971f8d1bb
// F160: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop::closure$0
// F162: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/lazy.rs:62:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F163: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:269:0 core::ptr::const_ptr::impl$0::as_ref
// F164: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:543:0 core::str::traits::impl$11::index
// F165: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:472:0 core::str::traits::impl$10::index
// F166: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:968:0 core::result::Result::map_err
// F167: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1233:0 core::result::Result::unwrap
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/visual/place_spires.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: fc4b22dfc9b1321a61b518260f885d9435fa1253b0612961653d3261ee8a6d08
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: place_spires.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2413, 3750
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
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
// 首条位置: {"module":3750,"record_offset":268,"rva":42051920,"va":5410761040}
// 编译实例 RVAs: 0x281a950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26152618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e856f170fe","size":5,"soft_fail":false,"target_va":5384609408,"va":5410762021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27297293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8f3795ffe","size":5,"soft_fail":false,"target_va":5383464432,"va":5410761720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-35083153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e86face8fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5410761852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33202261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8ab5f05fe","size":5,"soft_fail":false,"target_va":5377560512,"va":5410762768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31045460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8ac4826fe","size":5,"soft_fail":false,"target_va":5379716240,"va":5410761695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbedd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25542155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8f5417afe","size":5,"soft_fail":false,"target_va":5385219536,"va":5410761686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f42280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9275562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8567772ff","size":5,"soft_fail":false,"target_va":5401485952,"va":5410761509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5859286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8d6675900","size":5,"soft_fail":false,"target_va":5416621408,"va":5410762117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5859246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8ae675900","size":5,"soft_fail":false,"target_va":5416621632,"va":5410762381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e88e060000","size":5,"soft_fail":false,"target_va":5410762800,"va":5410761117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5859171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e863675900","size":5,"soft_fail":false,"target_va":5416621376,"va":5410762200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-26797680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8901967fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5410761467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-26789173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8cb3a67fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5410761392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-26797167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8911b67fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5410762074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-26798276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e83c1767fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5410761087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-27041919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8815f63fe","size":5,"soft_fail":false,"target_va":5383719888,"va":5410761802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6257820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e89c7c5f00","size":5,"soft_fail":false,"target_va":5417019952,"va":5410762127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8eb5d0000","size":5,"soft_fail":false,"target_va":5410786608,"va":5410762560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5857190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410761040,"raw_hex":"e8a65f5900","size":5,"soft_fail":false,"target_va":5416619360,"va":5410762165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5410761040,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5410761531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3750,"record_offset":2716,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3750,"record_offset":3972,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3750,"record_offset":3264,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3750,"record_offset":3200,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3750,"record_offset":3600,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3750,"record_offset":5008,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3750,"record_offset":2656,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3750,"record_offset":3320,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":27014,"source_line":366,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3750,"record_offset":4028,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3750,"record_offset":3888,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3750,"record_offset":1848,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3750,"record_offset":452,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3750,"record_offset":836,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3750,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3750,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3750,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3750,"record_offset":3760,"parent_record_offset":3672,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3750,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3750,"record_offset":3700,"parent_record_offset":3672,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3750,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3750,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3750,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3750,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3750,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3750,"record_offset":2956,"parent_record_offset":2924,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3750,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":268,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3750,"record_offset":2924,"parent_record_offset":2892,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3750,"record_offset":2860,"parent_record_offset":2744,"ancestor_record_offset":268,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3750,"record_offset":2892,"parent_record_offset":2860,"ancestor_record_offset":268,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3750,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":268,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3750,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":268,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3750,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":268,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3750,"record_offset":2776,"parent_record_offset":2744,"ancestor_record_offset":268,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":3750,"record_offset":3160,"parent_record_offset":2860,"ancestor_record_offset":268,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3750,"record_offset":2176,"parent_record_offset":2148,"ancestor_record_offset":268,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3750,"record_offset":2204,"parent_record_offset":2176,"ancestor_record_offset":268,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3750,"record_offset":2124,"parent_record_offset":2096,"ancestor_record_offset":268,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3750,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":268,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3750,"record_offset":2264,"parent_record_offset":2148,"ancestor_record_offset":268,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3750,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":268,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3750,"record_offset":3228,"parent_record_offset":3200,"ancestor_record_offset":268,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3750,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":268,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3750,"record_offset":3728,"parent_record_offset":3700,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3750,"record_offset":1996,"parent_record_offset":1956,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3750,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":268,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3750,"record_offset":784,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3750,"record_offset":2612,"parent_record_offset":1956,"ancestor_record_offset":268,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3750,"record_offset":2328,"parent_record_offset":2096,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3750,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3750,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":268,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3750,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":268,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3750,"record_offset":1440,"parent_record_offset":1412,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3750,"record_offset":3496,"parent_record_offset":3420,"ancestor_record_offset":268,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3750,"record_offset":1540,"parent_record_offset":1512,"ancestor_record_offset":268,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3750,"record_offset":1512,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3750,"record_offset":3360,"parent_record_offset":3320,"ancestor_record_offset":268,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3750,"record_offset":3096,"parent_record_offset":2924,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3750,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":268,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3750,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":268,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3750,"record_offset":1568,"parent_record_offset":1540,"ancestor_record_offset":268,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3750,"record_offset":1160,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3750,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":268,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3750,"record_offset":872,"parent_record_offset":836,"ancestor_record_offset":268,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":3750,"record_offset":1012,"parent_record_offset":980,"ancestor_record_offset":268,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":3750,"record_offset":980,"parent_record_offset":944,"ancestor_record_offset":268,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3750,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":268,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3750,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":268,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3750,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":268,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3750,"record_offset":1484,"parent_record_offset":944,"ancestor_record_offset":268,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3750,"record_offset":556,"parent_record_offset":524,"ancestor_record_offset":268,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3750,"record_offset":524,"parent_record_offset":496,"ancestor_record_offset":268,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3750,"record_offset":680,"parent_record_offset":648,"ancestor_record_offset":268,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3750,"record_offset":732,"parent_record_offset":680,"ancestor_record_offset":268,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3750,"record_offset":648,"parent_record_offset":620,"ancestor_record_offset":268,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3750,"record_offset":496,"parent_record_offset":452,"ancestor_record_offset":268,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3750,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":268,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3750,"record_offset":1044,"parent_record_offset":1012,"ancestor_record_offset":268,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3750,"record_offset":1328,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3750,"record_offset":1300,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3750,"record_offset":1104,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3750,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":268,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3750,"record_offset":3540,"parent_record_offset":3320,"ancestor_record_offset":268,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3750,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":268,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3750,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":268,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3750,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":268,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3750,"record_offset":3388,"parent_record_offset":3320,"ancestor_record_offset":268,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3750,"record_offset":3420,"parent_record_offset":3388,"ancestor_record_offset":268,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3750,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":268,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3750,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":268,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3750,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":268,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3750,"record_offset":1884,"parent_record_offset":1848,"ancestor_record_offset":268,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3750,"record_offset":4204,"parent_record_offset":4172,"ancestor_record_offset":268,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3750,"record_offset":4644,"parent_record_offset":4440,"ancestor_record_offset":268,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3750,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":268,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3750,"record_offset":2096,"parent_record_offset":1956,"ancestor_record_offset":268,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3750,"record_offset":2300,"parent_record_offset":2096,"ancestor_record_offset":268,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":3750,"record_offset":3464,"parent_record_offset":3420,"ancestor_record_offset":268,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3750,"record_offset":2052,"parent_record_offset":2024,"ancestor_record_offset":268,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3750,"record_offset":1956,"parent_record_offset":1920,"ancestor_record_offset":268,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3750,"record_offset":1920,"parent_record_offset":1884,"ancestor_record_offset":268,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/03750.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::place_spires::place_roof_spires::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19A45
// 首条位置: {"module":2413,"inlinee":105029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2413,"record_offset":14900,"parent_record_offset":14844,"ancestor_record_offset":14844,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/02413.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2413,"record_offset":15160,"parent_record_offset":14844,"ancestor_record_offset":14844,"inlinee":27398,"source_line":181,"archive":"_pdb/C/modules/02413.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2413,"record_offset":15016,"parent_record_offset":14844,"ancestor_record_offset":14844,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/02413.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2413,"record_offset":14928,"parent_record_offset":14844,"ancestor_record_offset":14844,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2413,"record_offset":14956,"parent_record_offset":14928,"ancestor_record_offset":14844,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2413,"record_offset":15040,"parent_record_offset":15016,"ancestor_record_offset":14844,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":2413,"record_offset":15184,"parent_record_offset":15160,"ancestor_record_offset":14844,"inlinee":11831,"source_line":110,"archive":"_pdb/C/modules/02413.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>

// ------------------------------------------------------------------
// 函数: system_roof::visual::place_spires::place_roof_spires
// 信息 ID: 3a6560146507183379024618
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cd898a0c41c18ea683f31317: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> @ 0x1f07261 -> 0x281a950 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x1f28f73 -> 0x281a950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::place_spires::place_roof_spires::closure$0
// 信息 ID: 98a8eb82990102bb8e4156fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::visual::place_spires::place_roof_spires", "captures": "unknown"}
// [闭包环境类型引用] system_roof::visual::place_spires::place_roof_spires::closure_env$0 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::place_spires::place_roof_spires::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::place_spires::place_roof_spires::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>; module=2413, inline_record=14844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
