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
// Inline site 2241:21952 -> host 2241:21612 / rva_19ee9e0_len_151; exact_symbolizer_function_frame
//   Function-frame RVA span [0x19eea02, 0x19eeb10), 62 instructions
//
// FUNCTION system_roof::visual::place_spires::place_roof_spires
// Candidate interfaces: cd898a0c41c18ea683f31317
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3430:3164 -> rva_2196e70_len_797
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_19ee9e0_len_151: RVA 0x19ee9e0, 337 PDB bytes; 81 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_19ee9e0_len_151.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2241:21612: core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>
// Frames: F1
// 0x1419ee9e0  56                             pushq	%rsi
// 0x1419ee9e1  57                             pushq	%rdi
// 0x1419ee9e2  4883ec78                       subq	$0x78, %rsp
// 0x1419ee9e6  440f29442460                   movaps	%xmm8, 0x60(%rsp)
// 0x1419ee9ec  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x1419ee9f1  0f29742440                     movaps	%xmm6, 0x40(%rsp)
// 0x1419ee9f6  4c89c7                         movq	%r8, %rdi
// 0x1419ee9f9  4889ce                         movq	%rcx, %rsi
// Frames: F2
// 0x1419ee9fc  f3410f107104                   movss	0x4(%r9), %xmm6
// Frames: F3 -> F2
// 0x1419eea02  4c89c1                         movq	%r8, %rcx
// 0x1419eea05  e84648effe                     callq	0x1408e3250 <.text+0x8e2250>
// 0x1419eea0a  31c9                           xorl	%ecx, %ecx
// 0x1419eea0c  ba00000000                     movl	$0x0, %edx
// 0x1419eea11  84c0                           testb	%al, %al
// 0x1419eea13  0f84f7000000                   je	0x1419eeb10 <.text+0x19edb10>
// Frames: F4 -> F2
// 0x1419eea19  4889f9                         movq	%rdi, %rcx
// 0x1419eea1c  e86f3ceffe                     callq	0x1408e2690 <.text+0x8e1690>
// 0x1419eea21  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1419eea26  488d4c2420                     leaq	0x20(%rsp), %rcx
// 0x1419eea2b  e8a09b69fe                     callq	0x1400885d0 <.text+0x875d0>
// 0x1419eea30  660f6ec0                       movd	%eax, %xmm0
// 0x1419eea34  48c1e820                       shrq	$0x20, %rax
// 0x1419eea38  660f6ec8                       movd	%eax, %xmm1
// Frames: F5 -> F2
// 0x1419eea3c  660f6fd0                       movdqa	%xmm0, %xmm2
// 0x1419eea40  f30f5ed1                       divss	%xmm1, %xmm2
// 0x1419eea44  f30f5ec8                       divss	%xmm0, %xmm1
// Frames: F6 -> F5 -> F2
// 0x1419eea48  0f28c2                         movaps	%xmm2, %xmm0
// 0x1419eea4b  f30fc2c203                     cmpunordss	%xmm2, %xmm0
// 0x1419eea50  0f28d8                         movaps	%xmm0, %xmm3
// 0x1419eea53  0f54d9                         andps	%xmm1, %xmm3
// 0x1419eea56  f30f5fca                       maxss	%xmm2, %xmm1
// 0x1419eea5a  0f55c1                         andnps	%xmm1, %xmm0
// 0x1419eea5d  0f56c3                         orps	%xmm3, %xmm0
// 0x1419eea60  31c9                           xorl	%ecx, %ecx
// 0x1419eea62  f30f100d3ab14b01               movss	0x14bb13a(%rip), %xmm1  # 0x142ea9ba4
// Frames: F7 -> F2
// 0x1419eea6a  0f2ec8                         ucomiss	%xmm0, %xmm1
// 0x1419eea6d  ba00000000                     movl	$0x0, %edx
// 0x1419eea72  0f8698000000                   jbe	0x1419eeb10 <.text+0x19edb10>
// Frames: F8 -> F2
// 0x1419eea78  488d4c2434                     leaq	0x34(%rsp), %rcx
// 0x1419eea7d  4889fa                         movq	%rdi, %rdx
// 0x1419eea80  e89b47effe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F9 -> F2
// 0x1419eea85  4889f9                         movq	%rdi, %rcx
// 0x1419eea88  e8f34ceffe                     callq	0x1408e3780 <.text+0x8e2780>
// 0x1419eea8d  f30f1144242c                   movss	%xmm0, 0x2c(%rsp)
// 0x1419eea93  f30f114c2430                   movss	%xmm1, 0x30(%rsp)
// 0x1419eea99  488d4c2420                     leaq	0x20(%rsp), %rcx
// 0x1419eea9e  488d54242c                     leaq	0x2c(%rsp), %rdx
// 0x1419eeaa3  e818e32aff                     callq	0x140c9cdc0 <.text+0xc9bdc0>
// Frames: F10 -> F8 -> F2
// 0x1419eeaa8  f30f107c2434                   movss	0x34(%rsp), %xmm7
// Frames: F11 -> F10 -> F8 -> F2
// 0x1419eeaae  f30f587c2420                   addss	0x20(%rsp), %xmm7
// Frames: F12 -> F8 -> F2
// 0x1419eeab4  f20f10442438                   movsd	0x38(%rsp), %xmm0
// 0x1419eeaba  f2440f10442424                 movsd	0x24(%rsp), %xmm8
// Frames: F11 -> F12 -> F8 -> F2
// 0x1419eeac1  440f58c0                       addps	%xmm0, %xmm8
// Frames: F13 -> F2
// 0x1419eeac5  4889f9                         movq	%rdi, %rcx
// 0x1419eeac8  e89347effe                     callq	0x1408e3260 <.text+0x8e2260>
// 0x1419eeacd  0f57c9                         xorps	%xmm1, %xmm1
// Frames: F14 -> F15 -> F13 -> F2
// 0x1419eead0  f30f59c8                       mulss	%xmm0, %xmm1
// Frames: F11 -> F10 -> F8 -> F2
// 0x1419eead4  f30f58f9                       addss	%xmm1, %xmm7
// Frames: F11 -> F12 -> F8 -> F2
// 0x1419eead8  0f14c1                         unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
// 0x1419eeadb  410f58c0                       addps	%xmm8, %xmm0
// Frames: F16 -> F17 -> F18 -> F2
// 0x1419eeadf  0f57c9                         xorps	%xmm1, %xmm1
// 0x1419eeae2  f30f10cf                       movss	%xmm7, %xmm1            # xmm1 = xmm7[0],xmm1[1,2,3]
// 0x1419eeae6  0fc6c84c                       shufps	$0x4c, %xmm0, %xmm1     # xmm1 = xmm1[0,3],xmm0[0,1]
// 0x1419eeaea  0fc6c978                       shufps	$0x78, %xmm1, %xmm1     # xmm1 = xmm1[0,2,3,1]
// Frames: F19 -> F2
// 0x1419eeaee  f30f10472c                     movss	0x2c(%rdi), %xmm0
// Frames: F20 -> F2
// 0x1419eeaf3  0f294e10                       movaps	%xmm1, 0x10(%rsi)
// 0x1419eeaf7  f30f117620                     movss	%xmm6, 0x20(%rsi)
// 0x1419eeafc  f30f114624                     movss	%xmm0, 0x24(%rsi)
// 0x1419eeb01  48c746280000803e               movq	$0x3e800000, 0x28(%rsi) # imm = 0x3E800000
// 0x1419eeb09  b901000000                     movl	$0x1, %ecx
// 0x1419eeb0e  31d2                           xorl	%edx, %edx
// Frames: F21
// 0x1419eeb10  48890e                         movq	%rcx, (%rsi)
// 0x1419eeb13  48895608                       movq	%rdx, 0x8(%rsi)
// 0x1419eeb17  4889f0                         movq	%rsi, %rax
// 0x1419eeb1a  0f28742440                     movaps	0x40(%rsp), %xmm6
// 0x1419eeb1f  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x1419eeb24  440f28442460                   movaps	0x60(%rsp), %xmm8
// 0x1419eeb2a  4883c478                       addq	$0x78, %rsp
// 0x1419eeb2e  5f                             popq	%rdi
// 0x1419eeb2f  5e                             popq	%rsi
// 0x1419eeb30  c3                             retq
//
// RANGE rva_2196e70_len_797: RVA 0x2196e70, 1943 PDB bytes; 483 instructions
// Evidence: _assembly/roof-v1/B/pack-001/rva_2196e70_len_797.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3430:3164: system_roof::visual::place_spires::place_roof_spires
// Frames: F22
// 0x142196e70  55                             pushq	%rbp
// 0x142196e71  4157                           pushq	%r15
// 0x142196e73  4156                           pushq	%r14
// 0x142196e75  4155                           pushq	%r13
// 0x142196e77  4154                           pushq	%r12
// 0x142196e79  56                             pushq	%rsi
// 0x142196e7a  57                             pushq	%rdi
// 0x142196e7b  53                             pushq	%rbx
// 0x142196e7c  4881ec68010000                 subq	$0x168, %rsp            # imm = 0x168
// 0x142196e83  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x142196e8b  48c785e0000000feffffff         movq	$-0x2, 0xe0(%rbp)
// 0x142196e96  4c89c3                         movq	%r8, %rbx
// 0x142196e99  4889d7                         movq	%rdx, %rdi
// 0x142196e9c  4889ce                         movq	%rcx, %rsi
// Frames: F23
// 0x142196e9f  e85cfba7fe                     callq	0x140c16a00 <.text+0xc15a00>
// 0x142196ea4  84c0                           testb	%al, %al
// 0x142196ea6  8885d8000000                   movb	%al, 0xd8(%rbp)
// 0x142196eac  745d                           je	0x142196f0b <.text+0x2195f0b>
// Frames: F24 -> F23
// 0x142196eae  41be34000000                   movl	$0x34, %r14d
// 0x142196eb4  b934000000                     movl	$0x34, %ecx
// 0x142196eb9  e822fbffff                     callq	0x1421969e0 <.text+0x21959e0>
// 0x142196ebe  4c8d3db6d1e500                 leaq	0xe5d1b6(%rip), %r15    # 0x142ff407b
// 0x142196ec5  4883f801                       cmpq	$0x1, %rax
// 0x142196ec9  0f85a9000000                   jne	0x142196f78 <.text+0x2195f78>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x142196ecf  4989d1                         movq	%rdx, %r9
// 0x142196ed2  4885d2                         testq	%rdx, %rdx
// 0x142196ed5  743e                           je	0x142196f15 <.text+0x2195f15>
// Frames: F30 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x142196ed7  4983f934                       cmpq	$0x34, %r9
// 0x142196edb  7336                           jae	0x142196f13 <.text+0x2195f13>
// Frames: F31 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x142196edd  488d0597d1e500                 leaq	0xe5d197(%rip), %rax    # 0x142ff407b
// Frames: F32 -> F31 -> F26 -> F27 -> F28 -> F29 -> F23
// 0x142196ee4  41803c01bf                     cmpb	$-0x41, (%r9,%rax)
// Frames: F26 -> F27 -> F28 -> F29 -> F23
// 0x142196ee9  7f2a                           jg	0x142196f15 <.text+0x2195f15>
// Frames: F33 -> F28 -> F29 -> F23
// 0x142196eeb  488d0546d2e500                 leaq	0xe5d246(%rip), %rax    # 0x142ff4138
// 0x142196ef2  4889442420                     movq	%rax, 0x20(%rsp)
// 0x142196ef7  488d0d7dd1e500                 leaq	0xe5d17d(%rip), %rcx    # 0x142ff407b
// 0x142196efe  ba34000000                     movl	$0x34, %edx
// 0x142196f03  4531c0                         xorl	%r8d, %r8d
// 0x142196f06  e8352a7400                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F23
// 0x142196f0b  4531ff                         xorl	%r15d, %r15d
// 0x142196f0e  e9be010000                     jmp	0x1421970d1 <.text+0x21960d1>
// Frames: F26 -> F27 -> F28 -> F29 -> F23
// 0x142196f13  75d6                           jne	0x142196eeb <.text+0x2195eeb>
// Frames: F29 -> F23
// 0x142196f15  4c89c9                         movq	%r9, %rcx
// 0x142196f18  e8c3faffff                     callq	0x1421969e0 <.text+0x21959e0>
// 0x142196f1d  4883f801                       cmpq	$0x1, %rax
// 0x142196f21  7555                           jne	0x142196f78 <.text+0x2195f78>
// Frames: F25 -> F34 -> F35 -> F28 -> F36 -> F23
// 0x142196f23  4989d0                         movq	%rdx, %r8
// 0x142196f26  4983c002                       addq	$0x2, %r8
// 0x142196f2a  7439                           je	0x142196f65 <.text+0x2195f65>
// Frames: F30 -> F34 -> F35 -> F28 -> F36 -> F23
// 0x142196f2c  4983f834                       cmpq	$0x34, %r8
// 0x142196f30  7331                           jae	0x142196f63 <.text+0x2195f63>
// Frames: F31 -> F34 -> F35 -> F28 -> F36 -> F23
// 0x142196f32  488d0542d1e500                 leaq	0xe5d142(%rip), %rax    # 0x142ff407b
// Frames: F32 -> F31 -> F34 -> F35 -> F28 -> F36 -> F23
// 0x142196f39  41803c00bf                     cmpb	$-0x41, (%r8,%rax)
// Frames: F34 -> F35 -> F28 -> F36 -> F23
// 0x142196f3e  7f25                           jg	0x142196f65 <.text+0x2195f65>
// Frames: F37 -> F28 -> F36 -> F23
// 0x142196f40  488d0509d2e500                 leaq	0xe5d209(%rip), %rax    # 0x142ff4150
// 0x142196f47  4889442420                     movq	%rax, 0x20(%rsp)
// 0x142196f4c  488d0d28d1e500                 leaq	0xe5d128(%rip), %rcx    # 0x142ff407b
// 0x142196f53  ba34000000                     movl	$0x34, %edx
// 0x142196f58  41b934000000                   movl	$0x34, %r9d
// 0x142196f5e  e8dd297400                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F34 -> F35 -> F28 -> F36 -> F23
// 0x142196f63  75db                           jne	0x142196f40 <.text+0x2195f40>
// Frames: F38 -> F39 -> F40 -> F35 -> F28 -> F36 -> F23
// 0x142196f65  41be32000000                   movl	$0x32, %r14d
// 0x142196f6b  4929d6                         subq	%rdx, %r14
// Frames: F41 -> F42 -> F39 -> F40 -> F35 -> F28 -> F36 -> F23
// 0x142196f6e  4c8d3d06d1e500                 leaq	0xe5d106(%rip), %r15    # 0x142ff407b
// 0x142196f75  4d01c7                         addq	%r8, %r15
// 0x142196f78  4c8d056fd2e500                 leaq	0xe5d26f(%rip), %r8     # 0x142ff41ee
// 0x142196f7f  4c8d253ad2e500                 leaq	0xe5d23a(%rip), %r12    # 0x142ff41c0
// 0x142196f86  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F43 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196f90  410fbe40ff                     movsbl	-0x1(%r8), %eax
// 0x142196f95  85c0                           testl	%eax, %eax
// 0x142196f97  7817                           js	0x142196fb0 <.text+0x2195fb0>
// Frames: F51 -> F52 -> F53 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196f99  49ffc8                         decq	%r8
// 0x142196f9c  83f85c                         cmpl	$0x5c, %eax
// 0x142196f9f  7563                           jne	0x142197004 <.text+0x2196004>
// 0x142196fa1  eb73                           jmp	0x142197016 <.text+0x2196016>
// 0x142196fa3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F54 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fb0  410fb648fe                     movzbl	-0x2(%r8), %ecx
// Frames: F55 -> F56 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fb5  80f9c0                         cmpb	$-0x40, %cl
// Frames: F56 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fb8  7d23                           jge	0x142196fdd <.text+0x2195fdd>
// Frames: F57 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fba  410fb650fd                     movzbl	-0x3(%r8), %edx
// Frames: F55 -> F58 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fbf  80fac0                         cmpb	$-0x40, %dl
// Frames: F58 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fc2  7d22                           jge	0x142196fe6 <.text+0x2195fe6>
// Frames: F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fc4  450fb648fc                     movzbl	-0x4(%r8), %r9d
// 0x142196fc9  4983c0fc                       addq	$-0x4, %r8
// Frames: F65 -> F66 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fcd  4183e107                       andl	$0x7, %r9d
// Frames: F67 -> F68 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fd1  41c1e106                       shll	$0x6, %r9d
// 0x142196fd5  83e23f                         andl	$0x3f, %edx
// 0x142196fd8  4409ca                         orl	%r9d, %edx
// 0x142196fdb  eb10                           jmp	0x142196fed <.text+0x2195fed>
// Frames: F56 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fdd  4983c0fe                       addq	$-0x2, %r8
// 0x142196fe1  83e11f                         andl	$0x1f, %ecx
// 0x142196fe4  eb0f                           jmp	0x142196ff5 <.text+0x2195ff5>
// 0x142196fe6  4983c0fd                       addq	$-0x3, %r8
// 0x142196fea  83e20f                         andl	$0xf, %edx
// Frames: F67 -> F69 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fed  c1e206                         shll	$0x6, %edx
// 0x142196ff0  83e13f                         andl	$0x3f, %ecx
// 0x142196ff3  09d1                           orl	%edx, %ecx
// Frames: F67 -> F70 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196ff5  c1e106                         shll	$0x6, %ecx
// 0x142196ff8  243f                           andb	$0x3f, %al
// 0x142196ffa  0fb6c0                         movzbl	%al, %eax
// 0x142196ffd  09c8                           orl	%ecx, %eax
// Frames: F51 -> F52 -> F53 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142196fff  83f85c                         cmpl	$0x5c, %eax
// 0x142197002  7412                           je	0x142197016 <.text+0x2196016>
// 0x142197004  83f82f                         cmpl	$0x2f, %eax
// 0x142197007  740d                           je	0x142197016 <.text+0x2196016>
// Frames: F71 -> F72 -> F73 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x142197009  4d39e0                         cmpq	%r12, %r8
// Frames: F74 -> F73 -> F44 -> F45 -> F46 -> F47 -> F48 -> F49 -> F50 -> F23
// 0x14219700c  7582                           jne	0x142196f90 <.text+0x2195f90>
// 0x14219700e  41bd2e000000                   movl	$0x2e, %r13d
// 0x142197014  eb4d                           jmp	0x142197063 <.text+0x2196063>
// Frames: F75 -> F23
// 0x142197016  4c8d25a3d1e500                 leaq	0xe5d1a3(%rip), %r12    # 0x142ff41c0
// 0x14219701d  4d29e0                         subq	%r12, %r8
// Frames: F25 -> F34 -> F35 -> F28 -> F75 -> F23
// 0x142197020  49ffc0                         incq	%r8
// 0x142197023  7432                           je	0x142197057 <.text+0x2196057>
// Frames: F30 -> F34 -> F35 -> F28 -> F75 -> F23
// 0x142197025  4983f82e                       cmpq	$0x2e, %r8
// 0x142197029  732a                           jae	0x142197055 <.text+0x2196055>
// Frames: F32 -> F31 -> F34 -> F35 -> F28 -> F75 -> F23
// 0x14219702b  43803c20bf                     cmpb	$-0x41, (%r8,%r12)
// Frames: F34 -> F35 -> F28 -> F75 -> F23
// 0x142197030  7f25                           jg	0x142197057 <.text+0x2196057>
// Frames: F37 -> F28 -> F75 -> F23
// 0x142197032  488d05dfd0e500                 leaq	0xe5d0df(%rip), %rax    # 0x142ff4118
// 0x142197039  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14219703e  488d0d7bd1e500                 leaq	0xe5d17b(%rip), %rcx    # 0x142ff41c0
// 0x142197045  ba2e000000                     movl	$0x2e, %edx
// 0x14219704a  41b92e000000                   movl	$0x2e, %r9d
// 0x142197050  e8eb287400                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F34 -> F35 -> F28 -> F75 -> F23
// 0x142197055  75db                           jne	0x142197032 <.text+0x2196032>
// Frames: F38 -> F39 -> F40 -> F35 -> F28 -> F75 -> F23
// 0x142197057  41bd2e000000                   movl	$0x2e, %r13d
// 0x14219705d  4d29c5                         subq	%r8, %r13
// Frames: F41 -> F42 -> F39 -> F40 -> F35 -> F28 -> F75 -> F23
// 0x142197060  4d01c4                         addq	%r8, %r12
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x142197063  e8c810a8fe                     callq	0x140c18130 <.text+0xc17130>
// 0x142197068  4889c1                         movq	%rax, %rcx
// Frames: F82 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x14219706b  488b00                         movq	(%rax), %rax
// 0x14219706e  4883f801                       cmpq	$0x1, %rax
// 0x142197072  0f8534030000                   jne	0x1421973ac <.text+0x21963ac>
// Frames: F83 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x142197078  4883c108                       addq	$0x8, %rcx
// Frames: F84 -> F85 -> F86 -> F87 -> F88 -> F79 -> F80 -> F81 -> F23
// 0x14219707c  48833900                       cmpq	$0x0, (%rcx)
// 0x142197080  0f854a030000                   jne	0x1421973d0 <.text+0x21963d0>
// Frames: F89 -> F90 -> F91 -> F92 -> F93 -> F85 -> F86 -> F87 -> F88 -> F79 -> F80 -> F81 -> F23
// 0x142197086  48c701ffffffff                 movq	$-0x1, (%rcx)
// 0x14219708d  48898db8000000                 movq	%rcx, 0xb8(%rbp)
// Frames: F94 -> F95 -> F86 -> F87 -> F88 -> F79 -> F80 -> F81 -> F23
// 0x142197094  4883c108                       addq	$0x8, %rcx
// Frames: F96 -> F87 -> F88 -> F79 -> F80 -> F81 -> F23
// 0x142197098  4c896c2420                     movq	%r13, 0x20(%rsp)
// 0x14219709d  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x1421970a6  48c744242801000000             movq	$0x1, 0x28(%rsp)
// 0x1421970af  4c89fa                         movq	%r15, %rdx
// 0x1421970b2  4d89f0                         movq	%r14, %r8
// 0x1421970b5  4d89e1                         movq	%r12, %r9
// 0x1421970b8  e883fca7fe                     callq	0x140c16d40 <.text+0xc15d40>
// Frames: F89 -> F90 -> F91 -> F92 -> F97 -> F98 -> F98 -> F87 -> F88 -> F79 -> F80 -> F81 -> F23
// 0x1421970bd  488b8db8000000                 movq	0xb8(%rbp), %rcx
// 0x1421970c4  48ff01                         incq	(%rcx)
// Frames: F23
// 0x1421970c7  48894530                       movq	%rax, 0x30(%rbp)
// 0x1421970cb  41bf01000000                   movl	$0x1, %r15d
// 0x1421970d1  4c897d28                       movq	%r15, 0x28(%rbp)
// Frames: F99
// 0x1421970d5  488b0b                         movq	(%rbx), %rcx
// Frames: F100 -> F101 -> F99
// 0x1421970d8  488d158142d200                 leaq	0xd24281(%rip), %rdx    # 0x142ebb360
// 0x1421970df  e85c7a0100                     callq	0x1421aeb40 <.text+0x21adb40>
// 0x1421970e4  4889c3                         movq	%rax, %rbx
// 0x1421970e7  4885c0                         testq	%rax, %rax
// 0x1421970ea  0f849e010000                   je	0x14219728e <.text+0x219628e>
// Frames: F102 -> F101 -> F99
// 0x1421970f0  488d05660de600                 leaq	0xe60d66(%rip), %rax    # 0x142ff7e5d
// 0x1421970f7  488985c0000000                 movq	%rax, 0xc0(%rbp)
// 0x1421970fe  48c785c800000036000000         movq	$0x36, 0xc8(%rbp)
// 0x142197109  4989de                         movq	%rbx, %r14
// 0x14219710c  4983c610                       addq	$0x10, %r14
// Frames: F103 -> F104 -> F105 -> F106 -> F105 -> F102 -> F101 -> F99
// 0x142197110  48837b1836                     cmpq	$0x36, 0x18(%rbx)
// 0x142197115  0f85c1020000                   jne	0x1421973dc <.text+0x21963dc>
// Frames: F102 -> F101 -> F99
// 0x14219711b  498b16                         movq	(%r14), %rdx
// Frames: F107 -> F104 -> F105 -> F106 -> F105 -> F102 -> F101 -> F99
// 0x14219711e  488d0d380de600                 leaq	0xe60d38(%rip), %rcx    # 0x142ff7e5d
// 0x142197125  41b836000000                   movl	$0x36, %r8d
// 0x14219712b  e8c0ca7800                     callq	0x142923bf0 <.text+0x2922bf0>
// 0x142197130  85c0                           testl	%eax, %eax
// Frames: F102 -> F101 -> F99
// 0x142197132  0f85a4020000                   jne	0x1421973dc <.text+0x21963dc>
// Frames: F108 -> F101 -> F99
// 0x142197138  488b0b                         movq	(%rbx), %rcx
// 0x14219713b  488b4308                       movq	0x8(%rbx), %rax
// Frames: F101 -> F99
// 0x14219713f  ff5018                         callq	*0x18(%rax)
// Frames: F99
// 0x142197142  4889c3                         movq	%rax, %rbx
// 0x142197145  4885c0                         testq	%rax, %rax
// 0x142197148  0f8440010000                   je	0x14219728e <.text+0x219628e>
// Frames: F109 -> F110 -> F111 -> F112 -> F113
// 0x14219714e  488b07                         movq	(%rdi), %rax
// 0x142197151  488b4f08                       movq	0x8(%rdi), %rcx
// Frames: F114 -> F115 -> F116 -> F117 -> F118 -> F119 -> F120 -> F121 -> F122 -> F112 -> F113
// 0x142197155  488b90f8000000                 movq	0xf8(%rax), %rdx
// Frames: F118 -> F119 -> F120 -> F121 -> F122 -> F112 -> F113
// 0x14219715c  4c8b8000010000                 movq	0x100(%rax), %r8
// Frames: F123 -> F124 -> F125 -> F120 -> F121 -> F122 -> F112 -> F113
// 0x142197163  4e8d0482                       leaq	(%rdx,%r8,4), %r8
// Frames: F126 -> F121 -> F122 -> F112 -> F113
// 0x142197167  440fb68808010000               movzbl	0x108(%rax), %r9d
// Frames: F127 -> F121 -> F122 -> F112 -> F113
// 0x14219716f  0f57c0                         xorps	%xmm0, %xmm0
// 0x142197172  0f118590000000                 movups	%xmm0, 0x90(%rbp)
// 0x142197179  0f118580000000                 movups	%xmm0, 0x80(%rbp)
// 0x142197180  48c785a000000000000000         movq	$0x0, 0xa0(%rbp)
// Frames: F128 -> F129 -> F122 -> F112 -> F113
// 0x14219718b  4c8d91f8000000                 leaq	0xf8(%rcx), %r10
// Frames: F130 -> F131 -> F113
// 0x142197192  48894d38                       movq	%rcx, 0x38(%rbp)
// Frames: F132 -> F122 -> F112 -> F113
// 0x142197196  4881c1a0010000                 addq	$0x1a0, %rcx            # imm = 0x1A0
// Frames: F130 -> F131 -> F113
// 0x14219719d  48894d40                       movq	%rcx, 0x40(%rbp)
// 0x1421971a1  4c895548                       movq	%r10, 0x48(%rbp)
// 0x1421971a5  48894550                       movq	%rax, 0x50(%rbp)
// 0x1421971a9  48895558                       movq	%rdx, 0x58(%rbp)
// 0x1421971ad  4c894560                       movq	%r8, 0x60(%rbp)
// 0x1421971b1  48c7456808000000               movq	$0x8, 0x68(%rbp)
// 0x1421971b9  48c7457000000000               movq	$0x0, 0x70(%rbp)
// 0x1421971c1  48c7457808000000               movq	$0x8, 0x78(%rbp)
// 0x1421971c9  44888da8000000                 movb	%r9b, 0xa8(%rbp)
// Frames: F133 -> F134 -> F113
// 0x1421971d0  4c8d0541cee500                 leaq	0xe5ce41(%rip), %r8     # 0x142ff4018
// 0x1421971d7  488d8dc0000000                 leaq	0xc0(%rbp), %rcx
// 0x1421971de  488d5538                       leaq	0x38(%rbp), %rdx
// 0x1421971e2  e89971c3fe                     callq	0x140dce380 <.text+0xdcd380>
// Frames: F135
// 0x1421971e7  488b0e                         movq	(%rsi), %rcx
// 0x1421971ea  e8b10a7ffe                     callq	0x140987ca0 <.text+0x986ca0>
// 0x1421971ef  488bb5d0000000                 movq	0xd0(%rbp), %rsi
// Frames: F136 -> F137 -> F135
// 0x1421971f6  4885f6                         testq	%rsi, %rsi
// Frames: F137 -> F135
// 0x1421971f9  7465                           je	0x142197260 <.text+0x2196260>
// Frames: F138 -> F139 -> F140 -> F141 -> F142 -> F135
// 0x1421971fb  4889f1                         movq	%rsi, %rcx
// 0x1421971fe  48c1e105                       shlq	$0x5, %rcx
// Frames: F143 -> F141 -> F142 -> F135
// 0x142197202  483b4b20                       cmpq	0x20(%rbx), %rcx
// 0x142197206  0f87fe010000                   ja	0x14219740a <.text+0x219640a>
// Frames: F135
// 0x14219720c  4c8b8dc8000000                 movq	0xc8(%rbp), %r9
// Frames: F142 -> F135
// 0x142197213  4989d8                         movq	%rbx, %r8
// 0x142197216  4983c008                       addq	$0x8, %r8
// Frames: F144 -> F141 -> F142 -> F135
// 0x14219721a  48c7450800000000               movq	$0x0, 0x8(%rbp)
// 0x142197222  48894d10                       movq	%rcx, 0x10(%rbp)
// Frames: F145 -> F141 -> F142 -> F135
// 0x142197226  488d5508                       leaq	0x8(%rbp), %rdx
// 0x14219722a  4889542428                     movq	%rdx, 0x28(%rsp)
// 0x14219722f  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x142197234  48c744243800000000             movq	$0x0, 0x38(%rsp)
// 0x14219723d  48c744243001000000             movq	$0x1, 0x30(%rsp)
// 0x142197246  488d4d38                       leaq	0x38(%rbp), %rcx
// 0x14219724a  4889c2                         movq	%rax, %rdx
// 0x14219724d  e84e7da5fe                     callq	0x140beefa0 <.text+0xbedfa0>
// Frames: F146 -> F145 -> F141 -> F142 -> F135
// 0x142197252  488b4538                       movq	0x38(%rbp), %rax
// 0x142197256  4883f80b                       cmpq	$0xb, %rax
// 0x14219725a  0f85b1010000                   jne	0x142197411 <.text+0x2196411>
// Frames: F147 -> F148 -> F149 -> F135
// 0x142197260  488933                         movq	%rsi, (%rbx)
// Frames: F150 -> F151 -> F98 -> F98 -> F152
// 0x142197263  488b95c0000000                 movq	0xc0(%rbp), %rdx
// Frames: F153 -> F150 -> F151 -> F98 -> F98 -> F152
// 0x14219726a  4885d2                         testq	%rdx, %rdx
// 0x14219726d  7416                           je	0x142197285 <.text+0x2196285>
// Frames: F150 -> F151 -> F98 -> F98 -> F152
// 0x14219726f  488b8dc8000000                 movq	0xc8(%rbp), %rcx
// Frames: F154 -> F155 -> F150 -> F151 -> F98 -> F98 -> F152
// 0x142197276  48c1e205                       shlq	$0x5, %rdx
// Frames: F156 -> F157 -> F158 -> F151 -> F98 -> F98 -> F152
// 0x14219727a  41b810000000                   movl	$0x10, %r8d
// 0x142197280  e89bc947fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F98 -> F159
// 0x142197285  4c8b7d28                       movq	0x28(%rbp), %r15
// 0x142197289  e9af000000                     jmp	0x14219733d <.text+0x219633d>
// Frames: F160 -> F161 -> F162 -> F163
// 0x14219728e  488b0513514901                 movq	0x1495113(%rip), %rax   # 0x14362c3a8
// 0x142197295  488b00                         movq	(%rax), %rax
// Frames: F164 -> F163
// 0x142197298  4883f802                       cmpq	$0x2, %rax
// Frames: F163
// 0x14219729c  0f829b000000                   jb	0x14219733d <.text+0x219633d>
// 0x1421972a2  488d0d9fcfe500                 leaq	0xe5cf9f(%rip), %rcx    # 0x142ff4248
// 0x1421972a9  e8d26a4efe                     callq	0x14067dd80 <.text+0x67cd80>
// Frames: F165 -> F166 -> F167 -> F163
// 0x1421972ae  0f1000                         movups	(%rax), %xmm0
// Frames: F168 -> F169 -> F167 -> F163
// 0x1421972b1  8b4010                         movl	0x10(%rax), %eax
// Frames: F170 -> F171 -> F172 -> F167 -> F163
// 0x1421972b4  48c7456802000000               movq	$0x2, 0x68(%rbp)
// 0x1421972bc  488d0d9dcfe500                 leaq	0xe5cf9d(%rip), %rcx    # 0x142ff4260
// 0x1421972c3  48894d70                       movq	%rcx, 0x70(%rbp)
// 0x1421972c7  48c7457821000000               movq	$0x21, 0x78(%rbp)
// 0x1421972cf  488d1562cfe500                 leaq	0xe5cf62(%rip), %rdx    # 0x142ff4238
// 0x1421972d6  48899588000000                 movq	%rdx, 0x88(%rbp)
// 0x1421972dd  48c7859000000001000000         movq	$0x1, 0x90(%rbp)
// 0x1421972e8  48c7859800000008000000         movq	$0x8, 0x98(%rbp)
// 0x1421972f3  48c7453800000000               movq	$0x0, 0x38(%rbp)
// 0x1421972fb  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421972fe  0f118da0000000                 movups	%xmm1, 0xa0(%rbp)
// 0x142197305  48894d40                       movq	%rcx, 0x40(%rbp)
// 0x142197309  48c7454821000000               movq	$0x21, 0x48(%rbp)
// 0x142197311  48c7455000000000               movq	$0x0, 0x50(%rbp)
// 0x142197319  0f114558                       movups	%xmm0, 0x58(%rbp)
// 0x14219731d  c7858000000001000000           movl	$0x1, 0x80(%rbp)
// 0x142197327  898584000000                   movl	%eax, 0x84(%rbp)
// Frames: F172 -> F167 -> F163
// 0x14219732d  488d8dc0000000                 leaq	0xc0(%rbp), %rcx
// 0x142197334  488d5538                       leaq	0x38(%rbp), %rdx
// 0x142197338  e8f32fbbfe                     callq	0x140d4a330 <.text+0xd49330>
// Frames: F98 -> F159
// 0x14219733d  4d85ff                         testq	%r15, %r15
// 0x142197340  7440                           je	0x142197382 <.text+0x2196382>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x142197342  e8e90da8fe                     callq	0x140c18130 <.text+0xc17130>
// 0x142197347  4889c1                         movq	%rax, %rcx
// Frames: F82 -> F76 -> F77 -> F78 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x14219734a  488b00                         movq	(%rax), %rax
// 0x14219734d  4883f801                       cmpq	$0x1, %rax
// 0x142197351  7543                           jne	0x142197396 <.text+0x2196396>
// Frames: F83 -> F76 -> F77 -> F78 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x142197353  4883c108                       addq	$0x8, %rcx
// Frames: F84 -> F85 -> F86 -> F87 -> F88 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x142197357  48833900                       cmpq	$0x0, (%rcx)
// 0x14219735b  7573                           jne	0x1421973d0 <.text+0x21963d0>
// Frames: F89 -> F90 -> F91 -> F92 -> F93 -> F85 -> F86 -> F87 -> F88 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x14219735d  48c701ffffffff                 movq	$-0x1, (%rcx)
// 0x142197364  48898dd8000000                 movq	%rcx, 0xd8(%rbp)
// Frames: F94 -> F95 -> F86 -> F87 -> F88 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x14219736b  4883c108                       addq	$0x8, %rcx
// Frames: F174 -> F87 -> F88 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x14219736f  488b5530                       movq	0x30(%rbp), %rdx
// 0x142197373  e878fda7fe                     callq	0x140c170f0 <.text+0xc160f0>
// Frames: F89 -> F90 -> F91 -> F92 -> F97 -> F98 -> F98 -> F87 -> F88 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x142197378  488b85d8000000                 movq	0xd8(%rbp), %rax
// 0x14219737f  48ff00                         incq	(%rax)
// Frames: F159
// 0x142197382  4881c468010000                 addq	$0x168, %rsp            # imm = 0x168
// 0x142197389  5b                             popq	%rbx
// 0x14219738a  5f                             popq	%rdi
// 0x14219738b  5e                             popq	%rsi
// 0x14219738c  415c                           popq	%r12
// 0x14219738e  415d                           popq	%r13
// 0x142197390  415e                           popq	%r14
// 0x142197392  415f                           popq	%r15
// 0x142197394  5d                             popq	%rbp
// 0x142197395  c3                             retq
// Frames: F82 -> F76 -> F77 -> F78 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x142197396  4885c0                         testq	%rax, %rax
// 0x142197399  7529                           jne	0x1421973c4 <.text+0x21963c4>
// Frames: F175 -> F76 -> F77 -> F78 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x14219739b  31d2                           xorl	%edx, %edx
// 0x14219739d  e89e0f7800                     callq	0x142918340 <.text+0x2917340>
// 0x1421973a2  4889c1                         movq	%rax, %rcx
// Frames: F176 -> F78 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x1421973a5  4885c0                         testq	%rax, %rax
// 0x1421973a8  75ad                           jne	0x142197357 <.text+0x2196357>
// 0x1421973aa  eb18                           jmp	0x1421973c4 <.text+0x21963c4>
// Frames: F82 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x1421973ac  4885c0                         testq	%rax, %rax
// 0x1421973af  7513                           jne	0x1421973c4 <.text+0x21963c4>
// Frames: F175 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x1421973b1  31d2                           xorl	%edx, %edx
// 0x1421973b3  e8880f7800                     callq	0x142918340 <.text+0x2917340>
// 0x1421973b8  4889c1                         movq	%rax, %rcx
// Frames: F176 -> F78 -> F79 -> F80 -> F81 -> F23
// 0x1421973bb  4885c0                         testq	%rax, %rax
// 0x1421973be  0f85b8fcffff                   jne	0x14219707c <.text+0x219607c>
// 0x1421973c4  488d0d1df1e500                 leaq	0xe5f11d(%rip), %rcx    # 0x142ff64e8
// 0x1421973cb  e8000d7400                     callq	0x1428d80d0 <.text+0x28d70d0>
// 0x1421973d0  488d0d81f1e500                 leaq	0xe5f181(%rip), %rcx    # 0x142ff6558
// 0x1421973d7  e8741e7400                     callq	0x1428d9250 <.text+0x28d8250>
// Frames: F102 -> F101 -> F99
// 0x1421973dc  48c7453800000000               movq	$0x0, 0x38(%rbp)
// 0x1421973e4  488d055d3fd200                 leaq	0xd23f5d(%rip), %rax    # 0x142ebb348
// 0x1421973eb  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421973f0  488d95c0000000                 leaq	0xc0(%rbp), %rdx
// 0x1421973f7  4c8d4d38                       leaq	0x38(%rbp), %r9
// 0x1421973fb  31c9                           xorl	%ecx, %ecx
// 0x1421973fd  4d89f0                         movq	%r14, %r8
// 0x142197400  e83c377300                     callq	0x1428cab41 <.text+0x28c9b41>
// 0x142197405  e989000000                     jmp	0x142197493 <.text+0x2196493>
// Frames: F143 -> F141 -> F142 -> F135
// 0x14219740a  b80b000000                     movl	$0xb, %eax
// 0x14219740f  eb2b                           jmp	0x14219743c <.text+0x219643c>
// Frames: F177 -> F145 -> F141 -> F142 -> F135
// 0x142197411  488b8d80000000                 movq	0x80(%rbp), %rcx
// 0x142197418  48894d00                       movq	%rcx, (%rbp)
// 0x14219741c  0f104540                       movups	0x40(%rbp), %xmm0
// 0x142197420  0f104d50                       movups	0x50(%rbp), %xmm1
// 0x142197424  0f105560                       movups	0x60(%rbp), %xmm2
// 0x142197428  0f105d70                       movups	0x70(%rbp), %xmm3
// 0x14219742c  0f295df0                       movaps	%xmm3, -0x10(%rbp)
// 0x142197430  0f2955e0                       movaps	%xmm2, -0x20(%rbp)
// 0x142197434  0f294dd0                       movaps	%xmm1, -0x30(%rbp)
// 0x142197438  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// Frames: F178 -> F135
// 0x14219743c  48894538                       movq	%rax, 0x38(%rbp)
// 0x142197440  0f2845c0                       movaps	-0x40(%rbp), %xmm0
// 0x142197444  0f284dd0                       movaps	-0x30(%rbp), %xmm1
// 0x142197448  0f2855e0                       movaps	-0x20(%rbp), %xmm2
// 0x14219744c  0f285df0                       movaps	-0x10(%rbp), %xmm3
// 0x142197450  0f114540                       movups	%xmm0, 0x40(%rbp)
// 0x142197454  0f114d50                       movups	%xmm1, 0x50(%rbp)
// 0x142197458  0f115560                       movups	%xmm2, 0x60(%rbp)
// 0x14219745c  0f115d70                       movups	%xmm3, 0x70(%rbp)
// 0x142197460  488b4500                       movq	(%rbp), %rax
// 0x142197464  48898580000000                 movq	%rax, 0x80(%rbp)
// 0x14219746b  488d057ecde500                 leaq	0xe5cd7e(%rip), %rax    # 0x142ff41f0
// 0x142197472  4889442420                     movq	%rax, 0x20(%rsp)
// 0x142197477  488d0dd2cbe500                 leaq	0xe5cbd2(%rip), %rcx    # 0x142ff4050
// 0x14219747e  4c8d0dabcbe500                 leaq	0xe5cbab(%rip), %r9     # 0x142ff4030
// 0x142197485  4c8d4538                       leaq	0x38(%rbp), %r8
// 0x142197489  ba2b000000                     movl	$0x2b, %edx
// 0x14219748e  e8cd227400                     callq	0x1428d9760 <.text+0x28d8760>
// 0x142197493  0f0b                           ud2
// 0x142197495  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x1421974a0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421974a5  55                             pushq	%rbp
// 0x1421974a6  4157                           pushq	%r15
// 0x1421974a8  4156                           pushq	%r14
// 0x1421974aa  4155                           pushq	%r13
// 0x1421974ac  4154                           pushq	%r12
// 0x1421974ae  56                             pushq	%rsi
// 0x1421974af  57                             pushq	%rdi
// 0x1421974b0  53                             pushq	%rbx
// 0x1421974b1  4883ec48                       subq	$0x48, %rsp
// 0x1421974b5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421974bc  488b85b8000000                 movq	0xb8(%rbp), %rax
// Frames: F89 -> F90 -> F91 -> F92 -> F97 -> F98 -> F98 -> F87 -> F88 -> F79 -> F80 -> F81 -> F23
// 0x1421974c3  48ff00                         incq	(%rax)
// Frames: F87 -> F88 -> F79 -> F80 -> F81 -> F23
// 0x1421974c6  4883c448                       addq	$0x48, %rsp
// 0x1421974ca  5b                             popq	%rbx
// 0x1421974cb  5f                             popq	%rdi
// 0x1421974cc  5e                             popq	%rsi
// 0x1421974cd  415c                           popq	%r12
// 0x1421974cf  415d                           popq	%r13
// 0x1421974d1  415e                           popq	%r14
// 0x1421974d3  415f                           popq	%r15
// 0x1421974d5  5d                             popq	%rbp
// 0x1421974d6  c3                             retq
// 0x1421974d7  660f1f840000000000             nopw	(%rax,%rax)
// 0x1421974e0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421974e5  55                             pushq	%rbp
// 0x1421974e6  4157                           pushq	%r15
// 0x1421974e8  4156                           pushq	%r14
// 0x1421974ea  4155                           pushq	%r13
// 0x1421974ec  4154                           pushq	%r12
// 0x1421974ee  56                             pushq	%rsi
// 0x1421974ef  57                             pushq	%rdi
// 0x1421974f0  53                             pushq	%rbx
// 0x1421974f1  4883ec48                       subq	$0x48, %rsp
// 0x1421974f5  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F98 -> F159
// 0x1421974fc  80bdd800000000                 cmpb	$0x0, 0xd8(%rbp)
// 0x142197503  7420                           je	0x142197525 <.text+0x2196525>
// 0x142197505  488d4530                       leaq	0x30(%rbp), %rax
// 0x142197509  48894520                       movq	%rax, 0x20(%rbp)
// 0x14219750d  488d4520                       leaq	0x20(%rbp), %rax
// 0x142197511  48894518                       movq	%rax, 0x18(%rbp)
// Frames: F80 -> F173 -> F98 -> F98 -> F159
// 0x142197515  488d0d9ccbe500                 leaq	0xe5cb9c(%rip), %rcx    # 0x142ff40b8
// 0x14219751c  488d5518                       leaq	0x18(%rbp), %rdx
// 0x142197520  e8bb8e0000                     callq	0x1421a03e0 <.text+0x219f3e0>
// Frames: F22
// 0x142197525  90                             nop
// 0x142197526  4883c448                       addq	$0x48, %rsp
// 0x14219752a  5b                             popq	%rbx
// 0x14219752b  5f                             popq	%rdi
// 0x14219752c  5e                             popq	%rsi
// 0x14219752d  415c                           popq	%r12
// 0x14219752f  415d                           popq	%r13
// 0x142197531  415e                           popq	%r14
// 0x142197533  415f                           popq	%r15
// 0x142197535  5d                             popq	%rbp
// 0x142197536  c3                             retq
// 0x142197537  660f1f840000000000             nopw	(%rax,%rax)
// 0x142197540  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142197545  55                             pushq	%rbp
// 0x142197546  4157                           pushq	%r15
// 0x142197548  4156                           pushq	%r14
// 0x14219754a  4155                           pushq	%r13
// 0x14219754c  4154                           pushq	%r12
// 0x14219754e  56                             pushq	%rsi
// 0x14219754f  57                             pushq	%rdi
// 0x142197550  53                             pushq	%rbx
// 0x142197551  4883ec48                       subq	$0x48, %rsp
// 0x142197555  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F150 -> F151 -> F98 -> F98 -> F152
// 0x14219755c  488b95c0000000                 movq	0xc0(%rbp), %rdx
// Frames: F153 -> F150 -> F151 -> F98 -> F98 -> F152
// 0x142197563  4885d2                         testq	%rdx, %rdx
// 0x142197566  7416                           je	0x14219757e <.text+0x219657e>
// Frames: F150 -> F151 -> F98 -> F98 -> F152
// 0x142197568  488b8dc8000000                 movq	0xc8(%rbp), %rcx
// Frames: F154 -> F155 -> F150 -> F151 -> F98 -> F98 -> F152
// 0x14219756f  48c1e205                       shlq	$0x5, %rdx
// Frames: F156 -> F157 -> F158 -> F151 -> F98 -> F98 -> F152
// 0x142197573  41b810000000                   movl	$0x10, %r8d
// 0x142197579  e8a2c647fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F152
// 0x14219757e  90                             nop
// 0x14219757f  4883c448                       addq	$0x48, %rsp
// 0x142197583  5b                             popq	%rbx
// 0x142197584  5f                             popq	%rdi
// 0x142197585  5e                             popq	%rsi
// 0x142197586  415c                           popq	%r12
// 0x142197588  415d                           popq	%r13
// 0x14219758a  415e                           popq	%r14
// 0x14219758c  415f                           popq	%r15
// 0x14219758e  5d                             popq	%rbp
// 0x14219758f  c3                             retq
// 0x142197590  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142197595  55                             pushq	%rbp
// 0x142197596  4157                           pushq	%r15
// 0x142197598  4156                           pushq	%r14
// 0x14219759a  4155                           pushq	%r13
// 0x14219759c  4154                           pushq	%r12
// 0x14219759e  56                             pushq	%rsi
// 0x14219759f  57                             pushq	%rdi
// 0x1421975a0  53                             pushq	%rbx
// 0x1421975a1  4883ec48                       subq	$0x48, %rsp
// 0x1421975a5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421975ac  488b85d8000000                 movq	0xd8(%rbp), %rax
// Frames: F89 -> F90 -> F91 -> F92 -> F97 -> F98 -> F98 -> F87 -> F88 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x1421975b3  48ff00                         incq	(%rax)
// Frames: F87 -> F88 -> F79 -> F80 -> F173 -> F98 -> F98 -> F159
// 0x1421975b6  4883c448                       addq	$0x48, %rsp
// 0x1421975ba  5b                             popq	%rbx
// 0x1421975bb  5f                             popq	%rdi
// 0x1421975bc  5e                             popq	%rsi
// 0x1421975bd  415c                           popq	%r12
// 0x1421975bf  415d                           popq	%r13
// 0x1421975c1  415e                           popq	%r14
// 0x1421975c3  415f                           popq	%r15
// 0x1421975c5  5d                             popq	%rbp
// 0x1421975c6  c3                             retq
// 0x1421975c7  660f1f840000000000             nopw	(%rax,%rax)
// 0x1421975d0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421975d5  55                             pushq	%rbp
// 0x1421975d6  4157                           pushq	%r15
// 0x1421975d8  4156                           pushq	%r14
// 0x1421975da  4155                           pushq	%r13
// 0x1421975dc  4154                           pushq	%r12
// 0x1421975de  56                             pushq	%rsi
// 0x1421975df  57                             pushq	%rdi
// 0x1421975e0  53                             pushq	%rbx
// 0x1421975e1  4883ec48                       subq	$0x48, %rsp
// 0x1421975e5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421975ec  488d4d38                       leaq	0x38(%rbp), %rcx
// Frames: F178 -> F135
// 0x1421975f0  e81ba261fe                     callq	0x1407b1810 <.text+0x7b0810>
// 0x1421975f5  90                             nop
// 0x1421975f6  4883c448                       addq	$0x48, %rsp
// 0x1421975fa  5b                             popq	%rbx
// 0x1421975fb  5f                             popq	%rdi
// 0x1421975fc  5e                             popq	%rsi
// 0x1421975fd  415c                           popq	%r12
// 0x1421975ff  415d                           popq	%r13
// 0x142197601  415e                           popq	%r14
// 0x142197603  415f                           popq	%r15
// 0x142197605  5d                             popq	%rbp
// 0x142197606  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:293:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::h1a78e64a7631eae4
// F2: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:294:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::h1a78e64a7631eae4
// F3: crates/systems/roof/src/visual/place_spires.rs:29:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F4: crates/systems/roof/src/visual/place_spires.rs:30:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F5: crates/systems/roof/src/visual/place_spires.rs:32:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F6: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:891:0 core::f32::impl$0::max
// F7: crates/systems/roof/src/visual/place_spires.rs:35:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F8: crates/systems/roof/src/visual/place_spires.rs:36:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F9: crates/systems/roof/src/visual/place_spires.rs:37:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1335:0 glam::f32::vec3::impl$34::add
// F11: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F12: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1336:0 glam::f32::vec3::impl$34::add
// F13: crates/systems/roof/src/visual/place_spires.rs:38:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:344:0 core::ops::arith::impl$141::mul
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1247:0 glam::f32::vec3::impl$24::mul
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs:92:0 glam::f32::sse2::vec4::Vec4::new
// F17: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:163:0 glam::f32::vec3::Vec3::extend
// F18: crates/systems/roof/src/visual/place_spires.rs:40:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F19: crates/systems/roof/src/visual/place_spires.rs:42:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F20: crates/systems/roof/src/visual/place_spires.rs:39:0 system_roof::visual::place_spires::place_roof_spires::closure$0
// F21: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:295:0 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$::call_mut::h1a78e64a7631eae4
// F22: crates/systems/roof/src/visual/place_spires.rs:18:0 system_roof::visual::place_spires::place_roof_spires::hbe609233dae85440
// F23: crates/systems/roof/src/visual/place_spires.rs:23:0 system_roof::visual::place_spires::place_roof_spires::hbe609233dae85440
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:371:0 core::str::impl$0::is_char_boundary
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:434:0 core::str::traits::impl$10::get
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:465:0 core::str::traits::impl$10::index
// F28: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:60:0 core::str::traits::impl$4::index
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:375:0 core::str::impl$0::is_char_boundary
// F31: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:387:0 core::str::impl$0::is_char_boundary
// F32: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/mod.rs:1046:0 core::num::impl$6::is_utf8_char_boundary
// F33: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:467:0 core::str::traits::impl$10::index
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:502:0 core::str::traits::impl$11::get
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:535:0 core::str::traits::impl$11::index
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F37: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:537:0 core::str::traits::impl$11::index
// F38: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:211:0 core::str::traits::impl$7::get_unchecked
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:524:0 core::str::traits::impl$11::get_unchecked
// F40: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:505:0 core::str::traits::impl$11::get
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:967:0 core::ptr::const_ptr::impl$0::add
// F42: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:212:0 core::str::traits::impl$7::get_unchecked
// F43: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:85:0 core::str::validations::next_code_point_reverse
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/iter.rs:131:0 core::str::iter::impl$2::next_back
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/iter.rs:219:0 core::str::iter::impl$6::next_back
// F46: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:714:0 core::str::pattern::impl$11::next_back
// F47: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:317:0 core::str::pattern::ReverseSearcher::next_match_back
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:799:0 core::str::pattern::impl$22::next_match_back
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:1492:0 core::str::impl$0::rfind
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F51: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:285:0 core::slice::iter::impl$182::any
// F52: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:660:0 core::str::pattern::impl$8::matches
// F53: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:717:0 core::str::pattern::impl$11::next_back
// F54: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:94:0 core::str::validations::next_code_point_reverse
// F55: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:25:0 core::str::validations::utf8_is_cont_byte
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:96:0 core::str::validations::next_code_point_reverse
// F57: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:99:0 core::str::validations::next_code_point_reverse
// F58: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:101:0 core::str::validations::next_code_point_reverse
// F59: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:541:0 core::ptr::non_null::NonNull::offset
// F60: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:697:0 core::ptr::non_null::NonNull::sub
// F61: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:131:0 core::slice::iter::Iter::pre_dec_end
// F62: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:83:0 core::slice::iter::Iter::next_back_unchecked
// F63: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:401:0 core::slice::iter::impl$183::next_back
// F64: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F65: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:12:0 core::str::validations::utf8_first_byte
// F66: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F67: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:18:0 core::str::validations::utf8_acc_cont_byte
// F68: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:106:0 core::str::validations::next_code_point_reverse
// F69: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:108:0 core::str::validations::next_code_point_reverse
// F70: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:110:0 core::str::validations::next_code_point_reverse
// F71: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:1623:0 core::ptr::non_null::impl$14::eq
// F72: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$183::next_back
// F73: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F74: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$183::next_back
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F76: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/mod.rs:94:0 puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
// F77: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F78: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:309:0 std::thread::local::LocalKey::try_with
// F79: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:274:0 std::thread::local::LocalKey::with
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F82: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:53:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F83: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:54:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F84: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1822:0 core::cell::BorrowRefMut::new
// F85: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1122:0 core::cell::RefCell::try_borrow_mut
// F86: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1090:0 core::cell::RefCell::borrow_mut
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F88: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:310:0 std::thread::local::LocalKey::try_with
// F89: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F90: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:870:0 core::mem::replace
// F91: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:503:0 core::cell::Cell::replace
// F92: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:429:0 core::cell::Cell::set
// F93: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1824:0 core::cell::BorrowRefMut::new
// F94: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:2198:0 core::cell::UnsafeCell::get
// F95: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1130:0 core::cell::RefCell::try_borrow_mut
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F97: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1811:0 core::cell::impl$44::drop
// F98: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:523:0 core::ptr::drop_in_place
// F99: crates/systems/roof/src/visual/place_spires.rs:25:0 system_roof::visual::place_spires::place_roof_spires::hbe609233dae85440
// F100: crates/country-core/src/asset_libraries/ssbo_library.rs:209:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F101: crates/country-core/src/asset_libraries/ssbo_library.rs:245:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
// F102: crates/country-core/src/asset_libraries/ssbo_library.rs:210:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F103: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/cmp.rs:83:0 core::slice::cmp::impl$5::equal
// F104: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/cmp.rs:15:0 core::slice::cmp::impl$0::eq
// F105: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1922:0 core::cmp::impls::impl$9::eq
// F106: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:29:0 core::str::traits::impl$1::eq
// F107: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/cmp.rs:91:0 core::slice::cmp::impl$5::equal
// F108: crates/country-core/src/asset_libraries/ssbo_library.rs:211:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:640:0 bevy_ecs::system::query::Query::copy_unsafe
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:622:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:545:0 bevy_ecs::system::query::Query::as_readonly
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:671:0 bevy_ecs::system::query::Query::iter
// F113: crates/systems/roof/src/visual/place_spires.rs:26:0 system_roof::visual::place_spires::place_roof_spires::hbe609233dae85440
// F114: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F115: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F116: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F117: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F118: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F119: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2372:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:52:0 bevy_ecs::query::iter::QueryIter::new
// F122: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:2579:0 bevy_ecs::system::query::Query::as_readonly
// F123: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F124: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F125: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 alloc::vec::impl$8::deref
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2373:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F127: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2367:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:266:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:50:0 bevy_ecs::query::iter::QueryIter::new
// F130: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/filter_map.rs:24:0 core::iter::adapters::filter_map::FilterMap::new
// F131: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/traits/iterator.rs:916:0 core::iter::traits::iterator::Iterator::filter_map
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:49:0 bevy_ecs::query::iter::QueryIter::new
// F133: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3424:0 alloc::vec::impl$15::from_iter
// F134: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/traits/iterator.rs:1971:0 core::iter::traits::iterator::Iterator::collect
// F135: crates/systems/roof/src/visual/place_spires.rs:55:0 system_roof::visual::place_spires::place_roof_spires::hbe609233dae85440
// F136: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:135:0 core::slice::impl$0::is_empty
// F137: crates/country-core/src/render/ssbo.rs:358:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F138: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:340:0 core::mem::size_of_val
// F139: crates/rhapsody/src/raw/bytes.rs:12:0 rhapsody::raw::bytes::slice_as_bytes
// F140: crates/country-core/src/render/ssbo.rs:219:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F141: crates/country-core/src/render/ssbo.rs:162:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write
// F142: crates/country-core/src/render/ssbo.rs:359:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F143: crates/country-core/src/render/ssbo.rs:223:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F144: crates/country-core/src/render/ssbo.rs:225:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F145: crates/country-core/src/render/ssbo.rs:224:0 country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
// F146: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:854:0 core::result::Result::map_err
// F147: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3716:0 core::sync::atomic::atomic_store
// F148: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2727:0 core::sync::atomic::AtomicUsize::store
// F149: crates/country-core/src/render/ssbo.rs:362:0 country_core::render::ssbo::InstanceBuffer::gpu_write
// F150: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:751:0 alloc::raw_vec::RawVecInner::deallocate
// F151: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:402:0 alloc::raw_vec::impl$3::drop
// F152: crates/systems/roof/src/visual/place_spires.rs:56:0 system_roof::visual::place_spires::place_roof_spires::hbe609233dae85440
// F153: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:523:0 alloc::raw_vec::RawVecInner::current_memory
// F154: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:945:0 core::num::impl$11::unchecked_mul
// F155: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:531:0 alloc::raw_vec::RawVecInner::current_memory
// F156: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:116:0 alloc::alloc::dealloc
// F157: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:269:0 alloc::alloc::impl$1::deallocate
// F158: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:753:0 alloc::raw_vec::RawVecInner::deallocate
// F159: crates/systems/roof/src/visual/place_spires.rs:60:0 system_roof::visual::place_spires::place_roof_spires::hbe609233dae85440
// F160: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3731:0 core::sync::atomic::atomic_load
// F161: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2700:0 core::sync::atomic::AtomicUsize::load
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:1328:0 log::max_level
// F163: crates/systems/roof/src/visual/place_spires.rs:58:0 system_roof::visual::place_spires::place_roof_spires::hbe609233dae85440
// F164: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/macros/mod.rs:538:0 core::cmp::PartialOrd::le
// F165: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panic/location.rs:130:0 core::panic::location::Location::file
// F166: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:75:0 log::__private_api::log_impl
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:94:0 log::__private_api::log
// F168: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panic/location.rs:155:0 core::panic::location::Location::line
// F169: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:76:0 log::__private_api::log_impl
// F170: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:766:0 log::impl$52::clone
// F171: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:1034:0 log::RecordBuilder::build
// F172: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:81:0 log::__private_api::log_impl
// F173: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop
// F174: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop::closure$0
// F175: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:56:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F176: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:299:0 core::ptr::const_ptr::impl$0::as_ref
// F177: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:856:0 core::result::Result::map_err
// F178: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1109:0 core::result::Result::unwrap
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/visual/place_spires.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fc4b22dfc9b1321a61b518260f885d9435fa1253b0612961653d3261ee8a6d08
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: place_spires.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2241, 3430
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
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
// 首条位置: {"module":3430,"record_offset":3164,"rva":35221104,"va":5403930224}
// 编译实例 RVAs: 0x2196e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21286925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8f32fbbfe","size":5,"soft_fail":false,"target_va":5382644528,"va":5403931448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28849765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e89bc947fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5403931264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28415278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8d26a4efe","size":5,"soft_fail":false,"target_va":5375516032,"va":5403931305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27155941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e81ba261fe","size":5,"soft_fail":false,"target_va":5376776208,"va":5403932144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25228623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8b10a7ffe","size":5,"soft_fail":false,"target_va":5378702496,"va":5403931114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22708914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e84e7da5fe","size":5,"soft_fail":false,"target_va":5381222304,"va":5403931213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20745831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e89971c3fe","size":5,"soft_fail":false,"target_va":5383185280,"va":5403931106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421aeb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t96860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e85c7a0100","size":5,"soft_fail":false,"target_va":5404027712,"va":5403930847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: place_spires.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7916224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8c0ca7800","size":5,"soft_fail":false,"target_va":5411847152,"va":5403930923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7609972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8741e7400","size":5,"soft_fail":false,"target_va":5411541584,"va":5403931607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7550780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e83c377300","size":5,"soft_fail":false,"target_va":5411482433,"va":5403931648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7611085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8cd227400","size":5,"soft_fail":false,"target_va":5411542880,"va":5403931790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e822fbffff","size":5,"soft_fail":false,"target_va":5403929056,"va":5403930297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7612981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8352a7400","size":5,"soft_fail":false,"target_va":5411543360,"va":5403930374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22545277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e883fca7fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5403930808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22540088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8c810a8fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5403930723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22545032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e878fda7fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5403931507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22545572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e85cfba7fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5403930271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7868318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e89e0f7800","size":5,"soft_fail":false,"target_va":5411799872,"va":5403931549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8bb8e0000","size":5,"soft_fail":false,"target_va":5403968480,"va":5403931936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7605504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403930224,"raw_hex":"e8000d7400","size":5,"soft_fail":false,"target_va":5411537104,"va":5403931595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5403930224,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5403930943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3430,"record_offset":6004,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3430,"record_offset":8248,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3430,"record_offset":6524,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3430,"record_offset":6460,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":15461,"source_line":911,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3430,"record_offset":6884,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3430,"record_offset":8456,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3430,"record_offset":5764,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3430,"record_offset":6580,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":25075,"source_line":357,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3430,"record_offset":8280,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3430,"record_offset":8164,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3430,"record_offset":4780,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3430,"record_offset":3348,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3430,"record_offset":3804,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3430,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":3164,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3430,"record_offset":7048,"parent_record_offset":6956,"ancestor_record_offset":3164,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3430,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":3164,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3430,"record_offset":6984,"parent_record_offset":6956,"ancestor_record_offset":3164,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3430,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":3164,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3430,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":3164,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3430,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":3164,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3430,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":3164,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3430,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":3164,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3430,"record_offset":6216,"parent_record_offset":6184,"ancestor_record_offset":3164,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3430,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":3164,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3430,"record_offset":6184,"parent_record_offset":6152,"ancestor_record_offset":3164,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3430,"record_offset":6120,"parent_record_offset":6032,"ancestor_record_offset":3164,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3430,"record_offset":6152,"parent_record_offset":6120,"ancestor_record_offset":3164,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3430,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":3164,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3430,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":3164,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3430,"record_offset":6064,"parent_record_offset":6032,"ancestor_record_offset":3164,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":3430,"record_offset":6420,"parent_record_offset":6120,"ancestor_record_offset":3164,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3430,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":3164,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3430,"record_offset":5168,"parent_record_offset":5144,"ancestor_record_offset":3164,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3430,"record_offset":5144,"parent_record_offset":5116,"ancestor_record_offset":3164,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3430,"record_offset":5064,"parent_record_offset":5036,"ancestor_record_offset":3164,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3430,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":3164,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3430,"record_offset":5260,"parent_record_offset":5088,"ancestor_record_offset":3164,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3430,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":3164,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3430,"record_offset":5832,"parent_record_offset":5792,"ancestor_record_offset":3164,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3430,"record_offset":6488,"parent_record_offset":6460,"ancestor_record_offset":3164,"inlinee":15463,"source_line":23,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3430,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":3164,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3430,"record_offset":6748,"parent_record_offset":6724,"ancestor_record_offset":3164,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3430,"record_offset":7016,"parent_record_offset":6984,"ancestor_record_offset":3164,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3430,"record_offset":3504,"parent_record_offset":3472,"ancestor_record_offset":3164,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3430,"record_offset":4932,"parent_record_offset":4888,"ancestor_record_offset":3164,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3430,"record_offset":8336,"parent_record_offset":8304,"ancestor_record_offset":3164,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3430,"record_offset":8364,"parent_record_offset":8304,"ancestor_record_offset":3164,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3430,"record_offset":3752,"parent_record_offset":3724,"ancestor_record_offset":3164,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3430,"record_offset":5720,"parent_record_offset":4888,"ancestor_record_offset":3164,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3430,"record_offset":5324,"parent_record_offset":5036,"ancestor_record_offset":3164,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3430,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":3164,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3430,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":3164,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3430,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":3164,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3430,"record_offset":4400,"parent_record_offset":4372,"ancestor_record_offset":3164,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3430,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":3164,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3430,"record_offset":6780,"parent_record_offset":6680,"ancestor_record_offset":3164,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3430,"record_offset":5916,"parent_record_offset":5888,"ancestor_record_offset":3164,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3430,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":3164,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3430,"record_offset":6620,"parent_record_offset":6580,"ancestor_record_offset":3164,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3430,"record_offset":6356,"parent_record_offset":6184,"ancestor_record_offset":3164,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3430,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":3164,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3430,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":3164,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3430,"record_offset":4472,"parent_record_offset":4444,"ancestor_record_offset":3164,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3430,"record_offset":4120,"parent_record_offset":4004,"ancestor_record_offset":3164,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3430,"record_offset":3472,"parent_record_offset":3444,"ancestor_record_offset":3164,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3430,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":3164,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3430,"record_offset":3972,"parent_record_offset":3940,"ancestor_record_offset":3164,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3430,"record_offset":3940,"parent_record_offset":3904,"ancestor_record_offset":3164,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3430,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":3164,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3430,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3164,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3430,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":3164,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3430,"record_offset":4444,"parent_record_offset":3904,"ancestor_record_offset":3164,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3430,"record_offset":3444,"parent_record_offset":3412,"ancestor_record_offset":3164,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3430,"record_offset":3412,"parent_record_offset":3384,"ancestor_record_offset":3164,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3430,"record_offset":3604,"parent_record_offset":3572,"ancestor_record_offset":3164,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3430,"record_offset":3700,"parent_record_offset":3604,"ancestor_record_offset":3164,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3430,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":3164,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3430,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":3164,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3430,"record_offset":3384,"parent_record_offset":3348,"ancestor_record_offset":3164,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3430,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":3164,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3430,"record_offset":4004,"parent_record_offset":3972,"ancestor_record_offset":3164,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3430,"record_offset":4288,"parent_record_offset":4004,"ancestor_record_offset":3164,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3430,"record_offset":4260,"parent_record_offset":4004,"ancestor_record_offset":3164,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3430,"record_offset":4064,"parent_record_offset":4004,"ancestor_record_offset":3164,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3430,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":3164,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3430,"record_offset":6824,"parent_record_offset":6580,"ancestor_record_offset":3164,"inlinee":7965,"source_line":2725,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3430,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":3164,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3430,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":3164,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3430,"record_offset":5792,"parent_record_offset":5764,"ancestor_record_offset":3164,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3430,"record_offset":6648,"parent_record_offset":6580,"ancestor_record_offset":3164,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3430,"record_offset":6680,"parent_record_offset":6648,"ancestor_record_offset":3164,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3430,"record_offset":8392,"parent_record_offset":8304,"ancestor_record_offset":3164,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3430,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":3164,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3430,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":3164,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3430,"record_offset":4816,"parent_record_offset":4780,"ancestor_record_offset":3164,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3430,"record_offset":7188,"parent_record_offset":7156,"ancestor_record_offset":3164,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3430,"record_offset":7688,"parent_record_offset":7428,"ancestor_record_offset":3164,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3430,"record_offset":4960,"parent_record_offset":4932,"ancestor_record_offset":3164,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3430,"record_offset":5036,"parent_record_offset":4888,"ancestor_record_offset":3164,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3430,"record_offset":5296,"parent_record_offset":5036,"ancestor_record_offset":3164,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":3430,"record_offset":6724,"parent_record_offset":6680,"ancestor_record_offset":3164,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3430,"record_offset":4988,"parent_record_offset":4960,"ancestor_record_offset":3164,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3430,"record_offset":4888,"parent_record_offset":4852,"ancestor_record_offset":3164,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3430,"record_offset":4852,"parent_record_offset":4816,"ancestor_record_offset":3164,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03430.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::place_spires::place_roof_spires::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16C03
// 首条位置: {"module":2241,"inlinee":93187}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2241,"record_offset":22008,"parent_record_offset":21952,"ancestor_record_offset":21952,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2241,"record_offset":22268,"parent_record_offset":21952,"ancestor_record_offset":21952,"inlinee":30378,"source_line":162,"archive":"_pdb/B/modules/02241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2241,"record_offset":22124,"parent_record_offset":21952,"ancestor_record_offset":21952,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2241,"record_offset":22036,"parent_record_offset":21952,"ancestor_record_offset":21952,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/02241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2241,"record_offset":22064,"parent_record_offset":22036,"ancestor_record_offset":21952,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2241,"record_offset":22148,"parent_record_offset":22124,"ancestor_record_offset":21952,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":2241,"record_offset":22292,"parent_record_offset":22268,"ancestor_record_offset":21952,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/02241.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>

// ------------------------------------------------------------------
// 函数: system_roof::visual::place_spires::place_roof_spires
// 信息 ID: 01abf1a872ed14f87605ca5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cd898a0c41c18ea683f31317: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x19ee58f -> 0x2196e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > > @ 0x1a2d5f4 -> 0x2196e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::place_spires::place_roof_spires::closure$0
// 信息 ID: 72c65500063c994f903e5930
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::visual::place_spires::place_roof_spires", "captures": "unknown"}
// [闭包环境类型引用] system_roof::visual::place_spires::place_roof_spires::closure_env$0 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::place_spires::place_roof_spires::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::place_spires::place_roof_spires::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>; module=2241, inline_record=21952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
