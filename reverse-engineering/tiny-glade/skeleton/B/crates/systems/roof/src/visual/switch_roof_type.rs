// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: switch_roof_type.rs.assembly.json.gz

// FUNCTION system_roof::visual::switch_roof_type::stone_floor_bundle
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3467:2412 -> host 3467:396 / rva_21aaea0_len_81b; exact_symbolizer_function_frame
//   Function-frame RVA span [0x21ab2ae, 0x21ab2f6), 14 instructions
// Inline site 3469:4536 -> host 3469:372 / rva_21ac4f0_len_9f9; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x21ac95f, 0x21ac9b2), 15 instructions
//   Function-frame RVA span [0x21acab0, 0x21acafc), 14 instructions
// Inline site 3469:5252 -> host 3469:372 / rva_21ac4f0_len_9f9; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x21ac95f, 0x21ac9b2), 15 instructions
//   Function-frame RVA span [0x21acab0, 0x21acafc), 14 instructions
//
// FUNCTION system_roof::visual::switch_roof_type::switch_roof_type
// Candidate interfaces: 97ca56c246a3e7a05fa2db17
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3469:372 -> rva_21ac4f0_len_9f9
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21aaea0_len_81b: RVA 0x21aaea0, 2075 PDB bytes; 432 instructions
// Evidence: _assembly/roof-v1/B/pack-002/rva_21aaea0_len_81b.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3467:396: system_roof::create_roof::create_roof
// Frames: F1
// 0x1421aaea0  55                             pushq	%rbp
// 0x1421aaea1  4157                           pushq	%r15
// 0x1421aaea3  4156                           pushq	%r14
// 0x1421aaea5  4155                           pushq	%r13
// 0x1421aaea7  4154                           pushq	%r12
// 0x1421aaea9  56                             pushq	%rsi
// 0x1421aaeaa  57                             pushq	%rdi
// 0x1421aaeab  53                             pushq	%rbx
// 0x1421aaeac  4881ec78030000                 subq	$0x378, %rsp            # imm = 0x378
// 0x1421aaeb3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1421aaebb  440f298de0020000               movaps	%xmm9, 0x2e0(%rbp)
// 0x1421aaec3  440f2985d0020000               movaps	%xmm8, 0x2d0(%rbp)
// 0x1421aaecb  0f29bdc0020000                 movaps	%xmm7, 0x2c0(%rbp)
// 0x1421aaed2  0f29b5b0020000                 movaps	%xmm6, 0x2b0(%rbp)
// Frames: F2 -> F3 -> F4
// 0x1421aaed9  48c785a8020000feffffff         movq	$-0x2, 0x2a8(%rbp)
// 0x1421aaee4  488b19                         movq	(%rcx), %rbx
// Frames: F5 -> F3 -> F4
// 0x1421aaee7  4c8b5108                       movq	0x8(%rcx), %r10
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x1421aaeeb  488b33                         movq	(%rbx), %rsi
// 0x1421aaeee  4531db                         xorl	%r11d, %r11d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x1421aaef1  4889f0                         movq	%rsi, %rax
// 0x1421aaef4  492b4218                       subq	0x18(%r10), %rax
// 0x1421aaef8  490f42c3                       cmovbq	%r11, %rax
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x1421aaefc  492b7238                       subq	0x38(%r10), %rsi
// 0x1421aaf00  490f42f3                       cmovbq	%r11, %rsi
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421aaf04  4989c4                         movq	%rax, %r12
// 0x1421aaf07  49c1e406                       shlq	$0x6, %r12
// 0x1421aaf0b  4d036208                       addq	0x8(%r10), %r12
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421aaf0f  4d8b7210                       movq	0x10(%r10), %r14
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421aaf13  4929c6                         subq	%rax, %r14
// 0x1421aaf16  4d0f42f3                       cmovbq	%r11, %r14
// 0x1421aaf1a  488b8d60030000                 movq	0x360(%rbp), %rcx
// 0x1421aaf21  bf08000000                     movl	$0x8, %edi
// 0x1421aaf26  4c0f42e7                       cmovbq	%rdi, %r12
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421aaf2a  4889f0                         movq	%rsi, %rax
// 0x1421aaf2d  48c1e006                       shlq	$0x6, %rax
// 0x1421aaf31  49034228                       addq	0x28(%r10), %rax
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421aaf35  4d8b7a30                       movq	0x30(%r10), %r15
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421aaf39  4929f7                         subq	%rsi, %r15
// 0x1421aaf3c  4d0f42fb                       cmovbq	%r11, %r15
// 0x1421aaf40  480f42c7                       cmovbq	%rdi, %rax
// Frames: F20 -> F7 -> F8 -> F3 -> F4
// 0x1421aaf44  4d8b5240                       movq	0x40(%r10), %r10
// 0x1421aaf48  4d29fa                         subq	%r15, %r10
// 0x1421aaf4b  4d29f2                         subq	%r14, %r10
// 0x1421aaf4e  48899d20020000                 movq	%rbx, 0x220(%rbp)
// 0x1421aaf55  4c8913                         movq	%r10, (%rbx)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F4
// 0x1421aaf58  49c1e606                       shlq	$0x6, %r14
// 0x1421aaf5c  4d01e6                         addq	%r12, %r14
// 0x1421aaf5f  4c89b540020000                 movq	%r14, 0x240(%rbp)
// 0x1421aaf66  49c1e706                       shlq	$0x6, %r15
// 0x1421aaf6a  4901c7                         addq	%rax, %r15
// 0x1421aaf6d  4c89bd48020000                 movq	%r15, 0x248(%rbp)
// 0x1421aaf74  4c8b32                         movq	(%rdx), %r14
// 0x1421aaf77  418b511c                       movl	0x1c(%r9), %edx
// 0x1421aaf7b  8995a0020000                   movl	%edx, 0x2a0(%rbp)
// 0x1421aaf81  498b11                         movq	(%r9), %rdx
// 0x1421aaf84  4d8b4910                       movq	0x10(%r9), %r9
// 0x1421aaf88  4c898d50020000                 movq	%r9, 0x250(%rbp)
// 0x1421aaf8f  448b491c                       movl	0x1c(%rcx), %r9d
// 0x1421aaf93  44898d98020000                 movl	%r9d, 0x298(%rbp)
// 0x1421aaf9a  4c8b09                         movq	(%rcx), %r9
// 0x1421aaf9d  488b4910                       movq	0x10(%rcx), %rcx
// 0x1421aafa1  48898d30020000                 movq	%rcx, 0x230(%rbp)
// 0x1421aafa8  418b481c                       movl	0x1c(%r8), %ecx
// 0x1421aafac  898d9c020000                   movl	%ecx, 0x29c(%rbp)
// 0x1421aafb2  498b30                         movq	(%r8), %rsi
// 0x1421aafb5  498b4810                       movq	0x10(%r8), %rcx
// 0x1421aafb9  48898d38020000                 movq	%rcx, 0x238(%rbp)
// 0x1421aafc0  440f280df8307900               movaps	0x7930f8(%rip), %xmm9   # 0x14293e0c0
// 0x1421aafc8  4c8d6d08                       leaq	0x8(%rbp), %r13
// 0x1421aafcc  48899590020000                 movq	%rdx, 0x290(%rbp)
// 0x1421aafd3  488d4a20                       leaq	0x20(%rdx), %rcx
// 0x1421aafd7  48898d68020000                 movq	%rcx, 0x268(%rbp)
// 0x1421aafde  488dbd00020000                 leaq	0x200(%rbp), %rdi
// 0x1421aafe5  4c898d70020000                 movq	%r9, 0x270(%rbp)
// 0x1421aafec  498d4920                       leaq	0x20(%r9), %rcx
// 0x1421aaff0  48898d58020000                 movq	%rcx, 0x258(%rbp)
// 0x1421aaff7  488d4e20                       leaq	0x20(%rsi), %rcx
// 0x1421aaffb  48898d60020000                 movq	%rcx, 0x260(%rbp)
// 0x1421ab002  4c89b528020000                 movq	%r14, 0x228(%rbp)
// 0x1421ab009  eb4d                           jmp	0x1421ab058 <.text+0x21aa058>
// 0x1421ab00b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32 -> F33
// 0x1421ab010  488b4628                       movq	0x28(%rsi), %rax
// Frames: F34 -> F29 -> F30 -> F31 -> F32 -> F33
// 0x1421ab014  4889d9                         movq	%rbx, %rcx
// 0x1421ab017  48c1e105                       shlq	$0x5, %rcx
// Frames: F35 -> F36 -> F30 -> F31 -> F32 -> F33
// 0x1421ab01b  4c893c08                       movq	%r15, (%rax,%rcx)
// 0x1421ab01f  4c89740808                     movq	%r14, 0x8(%rax,%rcx)
// 0x1421ab024  4c89640810                     movq	%r12, 0x10(%rax,%rcx)
// 0x1421ab029  c644081800                     movb	$0x0, 0x18(%rax,%rcx)
// Frames: F37 -> F30 -> F31 -> F32 -> F33
// 0x1421ab02e  48ffc3                         incq	%rbx
// 0x1421ab031  48895e30                       movq	%rbx, 0x30(%rsi)
// Frames: F38 -> F31 -> F32 -> F33
// 0x1421ab035  48ff4640                       incq	0x40(%rsi)
// 0x1421ab039  488b8578020000                 movq	0x278(%rbp), %rax
// 0x1421ab040  4c8ba580020000                 movq	0x280(%rbp), %r12
// 0x1421ab047  4c8bb528020000                 movq	0x228(%rbp), %r14
// 0x1421ab04e  4189ff                         movl	%edi, %r15d
// 0x1421ab051  488dbd00020000                 leaq	0x200(%rbp), %rdi
// Frames: F39 -> F40 -> F41 -> F42 -> F43 -> F4
// 0x1421ab058  4d85e4                         testq	%r12, %r12
// 0x1421ab05b  7423                           je	0x1421ab080 <.text+0x21aa080>
// 0x1421ab05d  4c3ba540020000                 cmpq	0x240(%rbp), %r12
// 0x1421ab064  741a                           je	0x1421ab080 <.text+0x21aa080>
// 0x1421ab066  498d4c2440                     leaq	0x40(%r12), %rcx
// 0x1421ab06b  48898d80020000                 movq	%rcx, 0x280(%rbp)
// 0x1421ab072  48898578020000                 movq	%rax, 0x278(%rbp)
// 0x1421ab079  eb34                           jmp	0x1421ab0af <.text+0x21aa0af>
// 0x1421ab07b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F39 -> F44 -> F45 -> F41 -> F42 -> F43 -> F4
// 0x1421ab080  4885c0                         testq	%rax, %rax
// 0x1421ab083  0f8496040000                   je	0x1421ab51f <.text+0x21aa51f>
// 0x1421ab089  483b8548020000                 cmpq	0x248(%rbp), %rax
// 0x1421ab090  0f8489040000                   je	0x1421ab51f <.text+0x21aa51f>
// Frames: F46 -> F47 -> F48 -> F49 -> F44 -> F45 -> F41 -> F42 -> F43 -> F4
// 0x1421ab096  488d4840                       leaq	0x40(%rax), %rcx
// 0x1421ab09a  48898d78020000                 movq	%rcx, 0x278(%rbp)
// 0x1421ab0a1  48c7858002000000000000         movq	$0x0, 0x280(%rbp)
// 0x1421ab0ac  4989c4                         movq	%rax, %r12
// Frames: F50 -> F43 -> F4
// 0x1421ab0af  488b8520020000                 movq	0x220(%rbp), %rax
// 0x1421ab0b6  48ff00                         incq	(%rax)
// Frames: F51
// 0x1421ab0b9  498b542408                     movq	0x8(%r12), %rdx
// 0x1421ab0be  4c89f1                         movq	%r14, %rcx
// 0x1421ab0c1  4c8d0518c3e400                 leaq	0xe4c318(%rip), %r8     # 0x142ff73e0
// 0x1421ab0c8  e8236e8efe                     callq	0x140a91ef0 <.text+0xa90ef0>
// Frames: F52 -> F51
// 0x1421ab0cd  4885c0                         testq	%rax, %rax
// 0x1421ab0d0  0f84d4040000                   je	0x1421ab5aa <.text+0x21aa5aa>
// Frames: F53
// 0x1421ab0d6  4889c3                         movq	%rax, %rbx
// 0x1421ab0d9  498b542408                     movq	0x8(%r12), %rdx
// Frames: F54
// 0x1421ab0de  4c89f1                         movq	%r14, %rcx
// 0x1421ab0e1  4c8d0528c3e400                 leaq	0xe4c328(%rip), %r8     # 0x142ff7410
// 0x1421ab0e8  e8036e8efe                     callq	0x140a91ef0 <.text+0xa90ef0>
// Frames: F55 -> F54
// 0x1421ab0ed  4885c0                         testq	%rax, %rax
// 0x1421ab0f0  0f84c0040000                   je	0x1421ab5b6 <.text+0x21aa5b6>
// Frames: F56 -> F57 -> F54
// 0x1421ab0f6  8b8858020000                   movl	0x258(%rax), %ecx
// 0x1421ab0fc  83f902                         cmpl	$0x2, %ecx
// 0x1421ab0ff  0f834c040000                   jae	0x1421ab551 <.text+0x21aa551>
// 0x1421ab105  488b906c020000                 movq	0x26c(%rax), %rdx
// 0x1421ab10c  4c8d8560010000                 leaq	0x160(%rbp), %r8
// 0x1421ab113  49895010                       movq	%rdx, 0x10(%r8)
// 0x1421ab117  0f10805c020000                 movups	0x25c(%rax), %xmm0
// 0x1421ab11e  410f1100                       movups	%xmm0, (%r8)
// Frames: F58 -> F57 -> F54
// 0x1421ab122  0fb68099020000                 movzbl	0x299(%rax), %eax
// Frames: F54
// 0x1421ab129  898d5c010000                   movl	%ecx, 0x15c(%rbp)
// 0x1421ab12f  8885a7020000                   movb	%al, 0x2a7(%rbp)
// Frames: F59
// 0x1421ab135  4889d9                         movq	%rbx, %rcx
// 0x1421ab138  e8e3f497fe                     callq	0x140b2a620 <.text+0xb29620>
// 0x1421ab13d  0f28f0                         movaps	%xmm0, %xmm6
// Frames: F60
// 0x1421ab140  498b5c2408                     movq	0x8(%r12), %rbx
// Frames: F61
// 0x1421ab145  410f10442420                   movups	0x20(%r12), %xmm0
// 0x1421ab14b  410f104c242c                   movups	0x2c(%r12), %xmm1
// 0x1421ab151  0f118ddc010000                 movups	%xmm1, 0x1dc(%rbp)
// 0x1421ab158  0f2985d0010000                 movaps	%xmm0, 0x1d0(%rbp)
// Frames: F62
// 0x1421ab15f  458b742410                     movl	0x10(%r12), %r14d
// Frames: F63
// 0x1421ab164  488d8da7020000                 leaq	0x2a7(%rbp), %rcx
// 0x1421ab16b  e800a571fe                     callq	0x1408c5670 <.text+0x8c4670>
// Frames: F64
// 0x1421ab170  418b4c241c                     movl	0x1c(%r12), %ecx
// 0x1421ab175  898d80010000                   movl	%ecx, 0x180(%rbp)
// 0x1421ab17b  498b4c2414                     movq	0x14(%r12), %rcx
// 0x1421ab180  48898d78010000                 movq	%rcx, 0x178(%rbp)
// Frames: F65
// 0x1421ab187  0f10855c010000                 movups	0x15c(%rbp), %xmm0
// 0x1421ab18e  0f108d68010000                 movups	0x168(%rbp), %xmm1
// 0x1421ab195  0f114dcc                       movups	%xmm1, -0x34(%rbp)
// 0x1421ab199  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// 0x1421ab19d  488d8d78010000                 leaq	0x178(%rbp), %rcx
// 0x1421ab1a4  48894c2438                     movq	%rcx, 0x38(%rsp)
// 0x1421ab1a9  88442430                       movb	%al, 0x30(%rsp)
// 0x1421ab1ad  4489742428                     movl	%r14d, 0x28(%rsp)
// 0x1421ab1b2  f30f11742420                   movss	%xmm6, 0x20(%rsp)
// 0x1421ab1b8  4c8db5e8000000                 leaq	0xe8(%rbp), %r14
// 0x1421ab1bf  4c89f1                         movq	%r14, %rcx
// 0x1421ab1c2  4889da                         movq	%rbx, %rdx
// 0x1421ab1c5  4c8d85d0010000                 leaq	0x1d0(%rbp), %r8
// 0x1421ab1cc  4c8d4dc0                       leaq	-0x40(%rbp), %r9
// 0x1421ab1d0  e8eb7f73fe                     callq	0x1408e31c0 <.text+0x8e21c0>
// Frames: F66
// 0x1421ab1d5  4c89f1                         movq	%r14, %rcx
// 0x1421ab1d8  e8738073fe                     callq	0x1408e3250 <.text+0x8e2250>
// 0x1421ab1dd  89c3                           movl	%eax, %ebx
// Frames: F67
// 0x1421ab1df  4c89f1                         movq	%r14, %rcx
// 0x1421ab1e2  e8c98173fe                     callq	0x1408e33b0 <.text+0x8e23b0>
// 0x1421ab1e7  84c0                           testb	%al, %al
// 0x1421ab1e9  747f                           je	0x1421ab26a <.text+0x21aa26a>
// Frames: F68
// 0x1421ab1eb  4989f5                         movq	%rsi, %r13
// 0x1421ab1ee  89de                           movl	%ebx, %esi
// 0x1421ab1f0  4489ff                         movl	%r15d, %edi
// 0x1421ab1f3  4c8bbde8000000                 movq	0xe8(%rbp), %r15
// 0x1421ab1fa  8b85a0020000                   movl	0x2a0(%rbp), %eax
// 0x1421ab200  488b8d50020000                 movq	0x250(%rbp), %rcx
// Frames: F69 -> F70 -> F32 -> F71
// 0x1421ab207  8901                           movl	%eax, (%rcx)
// 0x1421ab209  488b8590020000                 movq	0x290(%rbp), %rax
// Frames: F72 -> F30 -> F73 -> F32 -> F71
// 0x1421ab210  4c8b7030                       movq	0x30(%rax), %r14
// Frames: F74 -> F73 -> F32 -> F71
// 0x1421ab214  488b5840                       movq	0x40(%rax), %rbx
// Frames: F75 -> F30 -> F73 -> F32 -> F71
// 0x1421ab218  4c3b7020                       cmpq	0x20(%rax), %r14
// 0x1421ab21c  7513                           jne	0x1421ab231 <.text+0x21aa231>
// Frames: F76 -> F30 -> F73 -> F32 -> F71
// 0x1421ab21e  488b8d68020000                 movq	0x268(%rbp), %rcx
// 0x1421ab225  488d15447b9c00                 leaq	0x9c7b44(%rip), %rdx    # 0x142b72d70
// 0x1421ab22c  e8ff733bfe                     callq	0x140562630 <.text+0x561630>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F73 -> F32 -> F71
// 0x1421ab231  488b9590020000                 movq	0x290(%rbp), %rdx
// 0x1421ab238  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F73 -> F32 -> F71
// 0x1421ab23c  4c89f1                         movq	%r14, %rcx
// 0x1421ab23f  48c1e104                       shlq	$0x4, %rcx
// Frames: F35 -> F36 -> F30 -> F73 -> F32 -> F71
// 0x1421ab243  48891c08                       movq	%rbx, (%rax,%rcx)
// 0x1421ab247  4c897c0808                     movq	%r15, 0x8(%rax,%rcx)
// Frames: F37 -> F30 -> F73 -> F32 -> F71
// 0x1421ab24c  49ffc6                         incq	%r14
// 0x1421ab24f  4c897230                       movq	%r14, 0x30(%rdx)
// Frames: F38 -> F73 -> F32 -> F71
// 0x1421ab253  48ff4240                       incq	0x40(%rdx)
// 0x1421ab257  4189ff                         movl	%edi, %r15d
// 0x1421ab25a  488dbd00020000                 leaq	0x200(%rbp), %rdi
// 0x1421ab261  89f3                           movl	%esi, %ebx
// 0x1421ab263  4c89ee                         movq	%r13, %rsi
// 0x1421ab266  4c8d6d08                       leaq	0x8(%rbp), %r13
// Frames: F77
// 0x1421ab26a  488d8d44010000                 leaq	0x144(%rbp), %rcx
// 0x1421ab271  0f28ce                         movaps	%xmm6, %xmm1
// 0x1421ab274  e817e178fe                     callq	0x140939390 <.text+0x938390>
// Frames: F78
// 0x1421ab279  488d8d78010000                 leaq	0x178(%rbp), %rcx
// 0x1421ab280  e89b9273fe                     callq	0x1408e4520 <.text+0x8e3520>
// Frames: F79
// 0x1421ab285  e8b60973fe                     callq	0x1408dbc40 <.text+0x8dac40>
// Frames: F80
// 0x1421ab28a  0f28f8                         movaps	%xmm0, %xmm7
// 0x1421ab28d  440f28c1                       movaps	%xmm1, %xmm8
// 0x1421ab291  e8da70e5fd                     callq	0x140002370 <.text+0x1370>
// 0x1421ab296  4989d6                         movq	%rdx, %r14
// 0x1421ab299  48898588020000                 movq	%rax, 0x288(%rbp)
// 0x1421ab2a0  488985c0010000                 movq	%rax, 0x1c0(%rbp)
// 0x1421ab2a7  488995c8010000                 movq	%rdx, 0x1c8(%rbp)
// Frames: F81 -> F82
// 0x1421ab2ae  31c9                           xorl	%ecx, %ecx
// 0x1421ab2b0  e89b7f68fe                     callq	0x140833250 <.text+0x832250>
// Frames: F83 -> F81 -> F82
// 0x1421ab2b5  4889f9                         movq	%rdi, %rcx
// 0x1421ab2b8  89c2                           movl	%eax, %edx
// 0x1421ab2ba  e8a10a68fe                     callq	0x14082bd60 <.text+0x82ad60>
// Frames: F84 -> F85 -> F82
// 0x1421ab2bf  c747e800000000                 movl	$0x0, -0x18(%rdi)
// 0x1421ab2c6  48c747e000000000               movq	$0x0, -0x20(%rdi)
// Frames: F86 -> F82
// 0x1421ab2ce  e8ad08e9fd                     callq	0x14003bb80 <.text+0x3ab80>
// Frames: F87 -> F82
// 0x1421ab2d3  c6851002000009                 movb	$0x9, 0x210(%rbp)
// 0x1421ab2da  440f298dd0010000               movaps	%xmm9, 0x1d0(%rbp)
// 0x1421ab2e2  8b05fcc3e400                   movl	0xe4c3fc(%rip), %eax    # 0x142ff76e4
// 0x1421ab2e8  8947f4                         movl	%eax, -0xc(%rdi)
// 0x1421ab2eb  488b05eac3e400                 movq	0xe4c3ea(%rip), %rax    # 0x142ff76dc
// 0x1421ab2f2  488947ec                       movq	%rax, -0x14(%rdi)
// Frames: F88
// 0x1421ab2f6  488b8538010000                 movq	0x138(%rbp), %rax
// 0x1421ab2fd  49894550                       movq	%rax, 0x50(%r13)
// 0x1421ab301  0f108528010000                 movups	0x128(%rbp), %xmm0
// 0x1421ab308  410f114540                     movups	%xmm0, 0x40(%r13)
// 0x1421ab30d  0f1085e8000000                 movups	0xe8(%rbp), %xmm0
// 0x1421ab314  0f108df8000000                 movups	0xf8(%rbp), %xmm1
// 0x1421ab31b  0f109508010000                 movups	0x108(%rbp), %xmm2
// 0x1421ab322  0f109d18010000                 movups	0x118(%rbp), %xmm3
// 0x1421ab329  410f115d30                     movups	%xmm3, 0x30(%r13)
// 0x1421ab32e  410f115520                     movups	%xmm2, 0x20(%r13)
// 0x1421ab333  410f114d10                     movups	%xmm1, 0x10(%r13)
// 0x1421ab338  410f114500                     movups	%xmm0, (%r13)
// 0x1421ab33d  0f108544010000                 movups	0x144(%rbp), %xmm0
// 0x1421ab344  410f114558                     movups	%xmm0, 0x58(%r13)
// 0x1421ab349  488b8554010000                 movq	0x154(%rbp), %rax
// 0x1421ab350  49894568                       movq	%rax, 0x68(%r13)
// 0x1421ab354  0f108578010000                 movups	0x178(%rbp), %xmm0
// 0x1421ab35b  0f108d88010000                 movups	0x188(%rbp), %xmm1
// 0x1421ab362  0f109598010000                 movups	0x198(%rbp), %xmm2
// 0x1421ab369  0f109da8010000                 movups	0x1a8(%rbp), %xmm3
// 0x1421ab370  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// 0x1421ab374  0f294dd0                       movaps	%xmm1, -0x30(%rbp)
// 0x1421ab378  0f2955e0                       movaps	%xmm2, -0x20(%rbp)
// 0x1421ab37c  0f295df0                       movaps	%xmm3, -0x10(%rbp)
// 0x1421ab380  488b85b8010000                 movq	0x1b8(%rbp), %rax
// 0x1421ab387  48894500                       movq	%rax, (%rbp)
// 0x1421ab38b  f30f117d78                     movss	%xmm7, 0x78(%rbp)
// 0x1421ab390  f3440f11457c                   movss	%xmm8, 0x7c(%rbp)
// 0x1421ab396  488b8588020000                 movq	0x288(%rbp), %rax
// 0x1421ab39d  48898580000000                 movq	%rax, 0x80(%rbp)
// 0x1421ab3a4  4c89b588000000                 movq	%r14, 0x88(%rbp)
// 0x1421ab3ab  0f288510020000                 movaps	0x210(%rbp), %xmm0
// 0x1421ab3b2  410f2985c8000000               movaps	%xmm0, 0xc8(%r13)
// 0x1421ab3ba  0f2885d0010000                 movaps	0x1d0(%rbp), %xmm0
// 0x1421ab3c1  0f288de0010000                 movaps	0x1e0(%rbp), %xmm1
// 0x1421ab3c8  0f2895f0010000                 movaps	0x1f0(%rbp), %xmm2
// 0x1421ab3cf  0f289d00020000                 movaps	0x200(%rbp), %xmm3
// 0x1421ab3d6  410f299db8000000               movaps	%xmm3, 0xb8(%r13)
// 0x1421ab3de  410f2995a8000000               movaps	%xmm2, 0xa8(%r13)
// 0x1421ab3e6  410f298d98000000               movaps	%xmm1, 0x98(%r13)
// 0x1421ab3ee  410f298588000000               movaps	%xmm0, 0x88(%r13)
// 0x1421ab3f6  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1421ab3fd  488b9568030000                 movq	0x368(%rbp), %rdx
// 0x1421ab404  4c8d45c0                       leaq	-0x40(%rbp), %r8
// 0x1421ab408  4c8d0d31c0e400                 leaq	0xe4c031(%rip), %r9     # 0x142ff7440
// 0x1421ab40f  e80c42c2fe                     callq	0x140dcf620 <.text+0xdce620>
// Frames: F89
// 0x1421ab414  4c8bb5f0010000                 movq	0x1f0(%rbp), %r14
// Frames: F90
// 0x1421ab41b  84db                           testb	%bl, %bl
// 0x1421ab41d  7411                           je	0x1421ab430 <.text+0x21aa430>
// 0x1421ab41f  31db                           xorl	%ebx, %ebx
// 0x1421ab421  eb1e                           jmp	0x1421ab441 <.text+0x21aa441>
// 0x1421ab423  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F91
// 0x1421ab430  488d8da7020000                 leaq	0x2a7(%rbp), %rcx
// 0x1421ab437  e834a271fe                     callq	0x1408c5670 <.text+0x8c4670>
// 0x1421ab43c  4189c7                         movl	%eax, %r15d
// 0x1421ab43f  b306                           movb	$0x6, %bl
// Frames: F92
// 0x1421ab441  488d8d5c010000                 leaq	0x15c(%rbp), %rcx
// 0x1421ab448  e8533e05ff                     callq	0x1411ff2a0 <.text+0x11fe2a0>
// 0x1421ab44d  f30f118578010000               movss	%xmm0, 0x178(%rbp)
// 0x1421ab455  f30f118d7c010000               movss	%xmm1, 0x17c(%rbp)
// 0x1421ab45d  488d4dd0                       leaq	-0x30(%rbp), %rcx
// 0x1421ab461  488d9578010000                 leaq	0x178(%rbp), %rdx
// 0x1421ab468  0f28d6                         movaps	%xmm6, %xmm2
// 0x1421ab46b  e87019affe                     callq	0x140c9cde0 <.text+0xc9bde0>
// Frames: F93
// 0x1421ab470  885dc0                         movb	%bl, -0x40(%rbp)
// 0x1421ab473  4489ff                         movl	%r15d, %edi
// 0x1421ab476  44887dc1                       movb	%r15b, -0x3f(%rbp)
// 0x1421ab47a  8b8598020000                   movl	0x298(%rbp), %eax
// 0x1421ab480  488b8d30020000                 movq	0x230(%rbp), %rcx
// Frames: F69 -> F70 -> F32 -> F93
// 0x1421ab487  8901                           movl	%eax, (%rcx)
// 0x1421ab489  488b8570020000                 movq	0x270(%rbp), %rax
// Frames: F72 -> F30 -> F73 -> F32 -> F93
// 0x1421ab490  488b5830                       movq	0x30(%rax), %rbx
// Frames: F74 -> F73 -> F32 -> F93
// 0x1421ab494  4c8b7840                       movq	0x40(%rax), %r15
// Frames: F75 -> F30 -> F73 -> F32 -> F93
// 0x1421ab498  483b5820                       cmpq	0x20(%rax), %rbx
// 0x1421ab49c  7513                           jne	0x1421ab4b1 <.text+0x21aa4b1>
// Frames: F76 -> F30 -> F73 -> F32 -> F93
// 0x1421ab49e  488b8d58020000                 movq	0x258(%rbp), %rcx
// 0x1421ab4a5  488d15c4789c00                 leaq	0x9c78c4(%rip), %rdx    # 0x142b72d70
// 0x1421ab4ac  e8ff753bfe                     callq	0x140562ab0 <.text+0x561ab0>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F73 -> F32 -> F93
// 0x1421ab4b1  488b9570020000                 movq	0x270(%rbp), %rdx
// 0x1421ab4b8  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F73 -> F32 -> F93
// 0x1421ab4bc  488d0c9b                       leaq	(%rbx,%rbx,4), %rcx
// Frames: F35 -> F36 -> F30 -> F73 -> F32 -> F93
// 0x1421ab4c0  4c893cc8                       movq	%r15, (%rax,%rcx,8)
// 0x1421ab4c4  0f1045c0                       movups	-0x40(%rbp), %xmm0
// 0x1421ab4c8  0f104dcc                       movups	-0x34(%rbp), %xmm1
// 0x1421ab4cc  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1421ab4d1  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// Frames: F37 -> F30 -> F73 -> F32 -> F93
// 0x1421ab4d6  48ffc3                         incq	%rbx
// 0x1421ab4d9  48895a30                       movq	%rbx, 0x30(%rdx)
// Frames: F38 -> F73 -> F32 -> F93
// 0x1421ab4dd  48ff4240                       incq	0x40(%rdx)
// Frames: F94
// 0x1421ab4e1  4d8b642408                     movq	0x8(%r12), %r12
// 0x1421ab4e6  8b859c020000                   movl	0x29c(%rbp), %eax
// 0x1421ab4ec  488b8d38020000                 movq	0x238(%rbp), %rcx
// Frames: F69 -> F95 -> F32 -> F33
// 0x1421ab4f3  8901                           movl	%eax, (%rcx)
// Frames: F72 -> F30 -> F31 -> F32 -> F33
// 0x1421ab4f5  488b5e30                       movq	0x30(%rsi), %rbx
// Frames: F74 -> F31 -> F32 -> F33
// 0x1421ab4f9  4c8b7e40                       movq	0x40(%rsi), %r15
// Frames: F75 -> F30 -> F31 -> F32 -> F33
// 0x1421ab4fd  483b5e20                       cmpq	0x20(%rsi), %rbx
// 0x1421ab501  0f8509fbffff                   jne	0x1421ab010 <.text+0x21aa010>
// Frames: F76 -> F30 -> F31 -> F32 -> F33
// 0x1421ab507  488b8d60020000                 movq	0x260(%rbp), %rcx
// 0x1421ab50e  488d155b789c00                 leaq	0x9c785b(%rip), %rdx    # 0x142b72d70
// 0x1421ab515  e816773bfe                     callq	0x140562c30 <.text+0x561c30>
// 0x1421ab51a  e9f1faffff                     jmp	0x1421ab010 <.text+0x21aa010>
// Frames: F96
// 0x1421ab51f  0f28b5b0020000                 movaps	0x2b0(%rbp), %xmm6
// 0x1421ab526  0f28bdc0020000                 movaps	0x2c0(%rbp), %xmm7
// 0x1421ab52d  440f2885d0020000               movaps	0x2d0(%rbp), %xmm8
// 0x1421ab535  440f288de0020000               movaps	0x2e0(%rbp), %xmm9
// 0x1421ab53d  4881c478030000                 addq	$0x378, %rsp            # imm = 0x378
// 0x1421ab544  5b                             popq	%rbx
// 0x1421ab545  5f                             popq	%rdi
// 0x1421ab546  5e                             popq	%rsi
// 0x1421ab547  415c                           popq	%r12
// 0x1421ab549  415d                           popq	%r13
// 0x1421ab54b  415e                           popq	%r14
// 0x1421ab54d  415f                           popq	%r15
// 0x1421ab54f  5d                             popq	%rbp
// 0x1421ab550  c3                             retq
// Frames: F97 -> F57 -> F54
// 0x1421ab551  480558020000                   addq	$0x258, %rax            # imm = 0x258
// 0x1421ab557  488985d0010000                 movq	%rax, 0x1d0(%rbp)
// 0x1421ab55e  488d059bf8ffff                 leaq	-0x765(%rip), %rax      # 0x1421aae00 <.text+0x21a9e00>
// 0x1421ab565  488985d8010000                 movq	%rax, 0x1d8(%rbp)
// Frames: F98 -> F97 -> F57 -> F54
// 0x1421ab56c  488d0535bfe400                 leaq	0xe4bf35(%rip), %rax    # 0x142ff74a8
// 0x1421ab573  488945c0                       movq	%rax, -0x40(%rbp)
// 0x1421ab577  48c745c801000000               movq	$0x1, -0x38(%rbp)
// 0x1421ab57f  48c745e000000000               movq	$0x0, -0x20(%rbp)
// 0x1421ab587  488d85d0010000                 leaq	0x1d0(%rbp), %rax
// 0x1421ab58e  488945d0                       movq	%rax, -0x30(%rbp)
// 0x1421ab592  48c745d801000000               movq	$0x1, -0x28(%rbp)
// Frames: F97 -> F57 -> F54
// 0x1421ab59a  488d1517bfe400                 leaq	0xe4bf17(%rip), %rdx    # 0x142ff74b8
// 0x1421ab5a1  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1421ab5a5  e8e6dd7200                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F99 -> F51
// 0x1421ab5aa  488d0d47bee400                 leaq	0xe4be47(%rip), %rcx    # 0x142ff73f8
// 0x1421ab5b1  e85add7200                     callq	0x1428d9310 <.text+0x28d8310>
// Frames: F99 -> F54
// 0x1421ab5b6  488d0d6bbee400                 leaq	0xe4be6b(%rip), %rcx    # 0x142ff7428
// 0x1421ab5bd  e84edd7200                     callq	0x1428d9310 <.text+0x28d8310>
// 0x1421ab5c2  cc                             int3
// 0x1421ab5c3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1421ab5d0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ab5d5  55                             pushq	%rbp
// 0x1421ab5d6  4157                           pushq	%r15
// 0x1421ab5d8  4156                           pushq	%r14
// 0x1421ab5da  4155                           pushq	%r13
// 0x1421ab5dc  4154                           pushq	%r12
// 0x1421ab5de  56                             pushq	%rsi
// 0x1421ab5df  57                             pushq	%rdi
// 0x1421ab5e0  53                             pushq	%rbx
// 0x1421ab5e1  4881ec88000000                 subq	$0x88, %rsp
// 0x1421ab5e8  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ab5ef  440f294c2440                   movaps	%xmm9, 0x40(%rsp)
// 0x1421ab5f5  440f29442450                   movaps	%xmm8, 0x50(%rsp)
// 0x1421ab5fb  0f297c2460                     movaps	%xmm7, 0x60(%rsp)
// 0x1421ab600  0f29742470                     movaps	%xmm6, 0x70(%rsp)
// Frames: F100 -> F100 -> F101
// 0x1421ab605  4883bd8802000000               cmpq	$0x0, 0x288(%rbp)
// 0x1421ab60d  740c                           je	0x1421ab61b <.text+0x21aa61b>
// 0x1421ab60f  488d8dc0010000                 leaq	0x1c0(%rbp), %rcx
// 0x1421ab616  e89546c2fe                     callq	0x140dcfcb0 <.text+0xdcecb0>
// Frames: F101
// 0x1421ab61b  0f28742470                     movaps	0x70(%rsp), %xmm6
// 0x1421ab620  0f287c2460                     movaps	0x60(%rsp), %xmm7
// 0x1421ab625  440f28442450                   movaps	0x50(%rsp), %xmm8
// 0x1421ab62b  440f284c2440                   movaps	0x40(%rsp), %xmm9
// 0x1421ab631  4881c488000000                 addq	$0x88, %rsp
// 0x1421ab638  5b                             popq	%rbx
// 0x1421ab639  5f                             popq	%rdi
// 0x1421ab63a  5e                             popq	%rsi
// 0x1421ab63b  415c                           popq	%r12
// 0x1421ab63d  415d                           popq	%r13
// 0x1421ab63f  415e                           popq	%r14
// 0x1421ab641  415f                           popq	%r15
// 0x1421ab643  5d                             popq	%rbp
// 0x1421ab644  c3                             retq
// 0x1421ab645  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x1421ab650  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ab655  55                             pushq	%rbp
// 0x1421ab656  4157                           pushq	%r15
// 0x1421ab658  4156                           pushq	%r14
// 0x1421ab65a  4155                           pushq	%r13
// 0x1421ab65c  4154                           pushq	%r12
// 0x1421ab65e  56                             pushq	%rsi
// 0x1421ab65f  57                             pushq	%rdi
// 0x1421ab660  53                             pushq	%rbx
// 0x1421ab661  4881ec88000000                 subq	$0x88, %rsp
// 0x1421ab668  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ab66f  440f294c2440                   movaps	%xmm9, 0x40(%rsp)
// 0x1421ab675  440f29442450                   movaps	%xmm8, 0x50(%rsp)
// 0x1421ab67b  0f297c2460                     movaps	%xmm7, 0x60(%rsp)
// 0x1421ab680  0f29742470                     movaps	%xmm6, 0x70(%rsp)
// 0x1421ab685  488d8d78010000                 leaq	0x178(%rbp), %rcx
// 0x1421ab68c  e8bffdf9fd                     callq	0x14014b450 <.text+0x14a450>
// 0x1421ab691  0f28742470                     movaps	0x70(%rsp), %xmm6
// 0x1421ab696  0f287c2460                     movaps	0x60(%rsp), %xmm7
// 0x1421ab69b  440f28442450                   movaps	0x50(%rsp), %xmm8
// 0x1421ab6a1  440f284c2440                   movaps	0x40(%rsp), %xmm9
// Frames: F1
// 0x1421ab6a7  4881c488000000                 addq	$0x88, %rsp
// 0x1421ab6ae  5b                             popq	%rbx
// 0x1421ab6af  5f                             popq	%rdi
// 0x1421ab6b0  5e                             popq	%rsi
// 0x1421ab6b1  415c                           popq	%r12
// 0x1421ab6b3  415d                           popq	%r13
// 0x1421ab6b5  415e                           popq	%r14
// 0x1421ab6b7  415f                           popq	%r15
// 0x1421ab6b9  5d                             popq	%rbp
// 0x1421ab6ba  c3                             retq
//
// RANGE rva_21ac4f0_len_9f9: RVA 0x21ac4f0, 2553 PDB bytes; 567 instructions
// Evidence: _assembly/roof-v1/B/pack-002/rva_21ac4f0_len_9f9.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3469:372: system_roof::visual::switch_roof_type::switch_roof_type
// Frames: F102
// 0x1421ac4f0  55                             pushq	%rbp
// 0x1421ac4f1  4157                           pushq	%r15
// 0x1421ac4f3  4156                           pushq	%r14
// 0x1421ac4f5  4155                           pushq	%r13
// 0x1421ac4f7  4154                           pushq	%r12
// 0x1421ac4f9  56                             pushq	%rsi
// 0x1421ac4fa  57                             pushq	%rdi
// 0x1421ac4fb  53                             pushq	%rbx
// 0x1421ac4fc  4881ecb8010000                 subq	$0x1b8, %rsp            # imm = 0x1B8
// 0x1421ac503  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1421ac50b  440f298d20010000               movaps	%xmm9, 0x120(%rbp)
// 0x1421ac513  440f298510010000               movaps	%xmm8, 0x110(%rbp)
// 0x1421ac51b  0f29bd00010000                 movaps	%xmm7, 0x100(%rbp)
// 0x1421ac522  0f29b5f0000000                 movaps	%xmm6, 0xf0(%rbp)
// 0x1421ac529  48c785e8000000feffffff         movq	$-0x2, 0xe8(%rbp)
// 0x1421ac534  4c8b95a0010000                 movq	0x1a0(%rbp), %r10
// Frames: F2 -> F3 -> F103
// 0x1421ac53b  4c8b31                         movq	(%rcx), %r14
// Frames: F5 -> F3 -> F103
// 0x1421ac53e  488b7108                       movq	0x8(%rcx), %rsi
// Frames: F6 -> F7 -> F8 -> F3 -> F103
// 0x1421ac542  4d8b1e                         movq	(%r14), %r11
// 0x1421ac545  31ff                           xorl	%edi, %edi
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F103
// 0x1421ac547  4c89d9                         movq	%r11, %rcx
// 0x1421ac54a  482b4e18                       subq	0x18(%rsi), %rcx
// 0x1421ac54e  480f42cf                       cmovbq	%rdi, %rcx
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F103
// 0x1421ac552  4c2b5e38                       subq	0x38(%rsi), %r11
// 0x1421ac556  4c0f42df                       cmovbq	%rdi, %r11
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F103
// 0x1421ac55a  4889c8                         movq	%rcx, %rax
// 0x1421ac55d  48c1e005                       shlq	$0x5, %rax
// 0x1421ac561  48034608                       addq	0x8(%rsi), %rax
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F103
// 0x1421ac565  4c8b7e10                       movq	0x10(%rsi), %r15
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F103
// 0x1421ac569  4929cf                         subq	%rcx, %r15
// 0x1421ac56c  4c0f42ff                       cmovbq	%rdi, %r15
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F103
// 0x1421ac570  4c8b6630                       movq	0x30(%rsi), %r12
// 0x1421ac574  bb08000000                     movl	$0x8, %ebx
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F103
// 0x1421ac579  480f42c3                       cmovbq	%rbx, %rax
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F103
// 0x1421ac57d  4c89d9                         movq	%r11, %rcx
// 0x1421ac580  48c1e105                       shlq	$0x5, %rcx
// 0x1421ac584  48034e28                       addq	0x28(%rsi), %rcx
// 0x1421ac588  4d29dc                         subq	%r11, %r12
// 0x1421ac58b  4c8b9db0010000                 movq	0x1b0(%rbp), %r11
// 0x1421ac592  4c0f42e7                       cmovbq	%rdi, %r12
// 0x1421ac596  488bbdb8010000                 movq	0x1b8(%rbp), %rdi
// 0x1421ac59d  480f42cb                       cmovbq	%rbx, %rcx
// Frames: F20 -> F7 -> F8 -> F3 -> F103
// 0x1421ac5a1  488b7640                       movq	0x40(%rsi), %rsi
// 0x1421ac5a5  4c29e6                         subq	%r12, %rsi
// 0x1421ac5a8  4c29fe                         subq	%r15, %rsi
// 0x1421ac5ab  488b9da8010000                 movq	0x1a8(%rbp), %rbx
// 0x1421ac5b2  4c8975d0                       movq	%r14, -0x30(%rbp)
// 0x1421ac5b6  498936                         movq	%rsi, (%r14)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F103
// 0x1421ac5b9  49c1e705                       shlq	$0x5, %r15
// 0x1421ac5bd  4c897d28                       movq	%r15, 0x28(%rbp)
// 0x1421ac5c1  49c1e405                       shlq	$0x5, %r12
// 0x1421ac5c5  4c896540                       movq	%r12, 0x40(%rbp)
// 0x1421ac5c9  488b7208                       movq	0x8(%rdx), %rsi
// 0x1421ac5cd  4885f6                         testq	%rsi, %rsi
// 0x1421ac5d0  7504                           jne	0x1421ac5d6 <.text+0x21ab5d6>
// 0x1421ac5d2  488b7210                       movq	0x10(%rdx), %rsi
// 0x1421ac5d6  48014528                       addq	%rax, 0x28(%rbp)
// 0x1421ac5da  48014d40                       addq	%rcx, 0x40(%rbp)
// 0x1421ac5de  4c8b33                         movq	(%rbx), %r14
// 0x1421ac5e1  488b5b08                       movq	0x8(%rbx), %rbx
// 0x1421ac5e5  488b17                         movq	(%rdi), %rdx
// 0x1421ac5e8  48895550                       movq	%rdx, 0x50(%rbp)
// 0x1421ac5ec  488b5710                       movq	0x10(%rdi), %rdx
// 0x1421ac5f0  48895500                       movq	%rdx, (%rbp)
// 0x1421ac5f4  8b571c                         movl	0x1c(%rdi), %edx
// 0x1421ac5f7  8995b4000000                   movl	%edx, 0xb4(%rbp)
// 0x1421ac5fd  498b10                         movq	(%r8), %rdx
// 0x1421ac600  48895530                       movq	%rdx, 0x30(%rbp)
// 0x1421ac604  418b531c                       movl	0x1c(%r11), %edx
// 0x1421ac608  8995dc000000                   movl	%edx, 0xdc(%rbp)
// 0x1421ac60e  498b13                         movq	(%r11), %rdx
// 0x1421ac611  4d8b4310                       movq	0x10(%r11), %r8
// 0x1421ac615  4c894518                       movq	%r8, 0x18(%rbp)
// 0x1421ac619  458b421c                       movl	0x1c(%r10), %r8d
// 0x1421ac61d  448985e0000000                 movl	%r8d, 0xe0(%rbp)
// 0x1421ac624  4d8b02                         movq	(%r10), %r8
// 0x1421ac627  4d8b5210                       movq	0x10(%r10), %r10
// 0x1421ac62b  4c895538                       movq	%r10, 0x38(%rbp)
// 0x1421ac62f  458b511c                       movl	0x1c(%r9), %r10d
// 0x1421ac633  448995b0000000                 movl	%r10d, 0xb0(%rbp)
// 0x1421ac63a  4d8b39                         movq	(%r9), %r15
// 0x1421ac63d  4d8b4910                       movq	0x10(%r9), %r9
// 0x1421ac641  4c894df0                       movq	%r9, -0x10(%rbp)
// 0x1421ac645  0f2835741a7900                 movaps	0x791a74(%rip), %xmm6   # 0x14293e0c0
// 0x1421ac64c  f30f103db0927700               movss	0x7792b0(%rip), %xmm7   # 0x142925904
// 0x1421ac654  f3440f10054f368d00             movss	0x8d364f(%rip), %xmm8   # 0x142a7fcac
// 0x1421ac65d  f3440f100d9a178d00             movss	0x8d179a(%rip), %xmm9   # 0x142a7de00
// 0x1421ac666  488995c0000000                 movq	%rdx, 0xc0(%rbp)
// 0x1421ac66d  488d5220                       leaq	0x20(%rdx), %rdx
// 0x1421ac671  48895548                       movq	%rdx, 0x48(%rbp)
// 0x1421ac675  4c8985c8000000                 movq	%r8, 0xc8(%rbp)
// 0x1421ac67c  498d5020                       leaq	0x20(%r8), %rdx
// 0x1421ac680  48895558                       movq	%rdx, 0x58(%rbp)
// 0x1421ac684  498d5720                       leaq	0x20(%r15), %rdx
// 0x1421ac688  488955f8                       movq	%rdx, -0x8(%rbp)
// 0x1421ac68c  48895de0                       movq	%rbx, -0x20(%rbp)
// 0x1421ac690  4c8975d8                       movq	%r14, -0x28(%rbp)
// 0x1421ac694  4c897de8                       movq	%r15, -0x18(%rbp)
// 0x1421ac698  eb33                           jmp	0x1421ac6cd <.text+0x21ab6cd>
// 0x1421ac69a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32 -> F104
// 0x1421ac6a0  498b4728                       movq	0x28(%r15), %rax
// Frames: F34 -> F29 -> F30 -> F31 -> F32 -> F104
// 0x1421ac6a4  4889f9                         movq	%rdi, %rcx
// 0x1421ac6a7  48c1e104                       shlq	$0x4, %rcx
// Frames: F35 -> F36 -> F30 -> F31 -> F32 -> F104
// 0x1421ac6ab  48891c08                       movq	%rbx, (%rax,%rcx)
// 0x1421ac6af  4c89740808                     movq	%r14, 0x8(%rax,%rcx)
// Frames: F37 -> F30 -> F31 -> F32 -> F104
// 0x1421ac6b4  48ffc7                         incq	%rdi
// 0x1421ac6b7  49897f30                       movq	%rdi, 0x30(%r15)
// Frames: F38 -> F31 -> F32 -> F104
// 0x1421ac6bb  49ff4740                       incq	0x40(%r15)
// 0x1421ac6bf  4c89e1                         movq	%r12, %rcx
// 0x1421ac6c2  4c89e8                         movq	%r13, %rax
// 0x1421ac6c5  4c8b75d8                       movq	-0x28(%rbp), %r14
// 0x1421ac6c9  488b5de0                       movq	-0x20(%rbp), %rbx
// Frames: F39 -> F40 -> F41 -> F42 -> F43 -> F103
// 0x1421ac6cd  4885c0                         testq	%rax, %rax
// 0x1421ac6d0  741e                           je	0x1421ac6f0 <.text+0x21ab6f0>
// 0x1421ac6d2  483b4528                       cmpq	0x28(%rbp), %rax
// 0x1421ac6d6  7418                           je	0x1421ac6f0 <.text+0x21ab6f0>
// 0x1421ac6d8  4c8d6820                       leaq	0x20(%rax), %r13
// 0x1421ac6dc  4989cc                         movq	%rcx, %r12
// 0x1421ac6df  eb2c                           jmp	0x1421ac70d <.text+0x21ab70d>
// 0x1421ac6e1  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F39 -> F44 -> F45 -> F41 -> F42 -> F43 -> F103
// 0x1421ac6f0  4885c9                         testq	%rcx, %rcx
// 0x1421ac6f3  0f84cc060000                   je	0x1421acdc5 <.text+0x21abdc5>
// 0x1421ac6f9  483b4d40                       cmpq	0x40(%rbp), %rcx
// 0x1421ac6fd  0f84c2060000                   je	0x1421acdc5 <.text+0x21abdc5>
// Frames: F46 -> F47 -> F48 -> F49 -> F44 -> F45 -> F41 -> F42 -> F43 -> F103
// 0x1421ac703  4c8d6120                       leaq	0x20(%rcx), %r12
// 0x1421ac707  4531ed                         xorl	%r13d, %r13d
// 0x1421ac70a  4889c8                         movq	%rcx, %rax
// Frames: F50 -> F43 -> F103
// 0x1421ac70d  488b4dd0                       movq	-0x30(%rbp), %rcx
// 0x1421ac711  48ff01                         incq	(%rcx)
// Frames: F105
// 0x1421ac714  488b4810                       movq	0x10(%rax), %rcx
// 0x1421ac718  48898db8000000                 movq	%rcx, 0xb8(%rbp)
// Frames: F106
// 0x1421ac71f  0fb67819                       movzbl	0x19(%rax), %edi
// 0x1421ac723  4088bde7000000                 movb	%dil, 0xe7(%rbp)
// Frames: F107 -> F108 -> F109 -> F110
// 0x1421ac72a  4189c8                         movl	%ecx, %r8d
// Frames: F111 -> F15 -> F107 -> F108 -> F109 -> F110
// 0x1421ac72d  4c3b4310                       cmpq	0x10(%rbx), %r8
// 0x1421ac731  0f83ea060000                   jae	0x1421ace21 <.text+0x21abe21>
// Frames: F25 -> F26 -> F27 -> F112 -> F17 -> F113 -> F107 -> F108 -> F109 -> F110
// 0x1421ac737  488b5308                       movq	0x8(%rbx), %rdx
// 0x1421ac73b  4989ca                         movq	%rcx, %r10
// 0x1421ac73e  49c1ea20                       shrq	$0x20, %r10
// Frames: F114 -> F111 -> F15 -> F107 -> F108 -> F109 -> F110
// 0x1421ac742  4f8d0c80                       leaq	(%r8,%r8,4), %r9
// Frames: F115 -> F116 -> F117 -> F108 -> F109 -> F110
// 0x1421ac746  4639148a                       cmpl	%r10d, (%rdx,%r9,4)
// Frames: F117 -> F108 -> F109 -> F110
// 0x1421ac74a  0f85d8060000                   jne	0x1421ace28 <.text+0x21abe28>
// Frames: F118 -> F119 -> F108 -> F109 -> F110
// 0x1421ac750  4e8d0c8a                       leaq	(%rdx,%r9,4), %r9
// 0x1421ac754  418b5104                       movl	0x4(%r9), %edx
// 0x1421ac758  41bbffffffff                   movl	$0xffffffff, %r11d      # imm = 0xFFFFFFFF
// 0x1421ac75e  4c39da                         cmpq	%r11, %rdx
// Frames: F119 -> F108 -> F109 -> F110
// 0x1421ac761  0f84c1060000                   je	0x1421ace28 <.text+0x21abe28>
// Frames: F120 -> F121 -> F109 -> F110
// 0x1421ac767  4531c0                         xorl	%r8d, %r8d
// 0x1421ac76a  49395638                       cmpq	%rdx, 0x38(%r14)
// Frames: F122 -> F123 -> F121 -> F109 -> F110
// 0x1421ac76e  0f86c4060000                   jbe	0x1421ace38 <.text+0x21abe38>
// Frames: F124 -> F125 -> F126 -> F122 -> F123 -> F121 -> F109 -> F110
// 0x1421ac774  4c896dc8                       movq	%r13, -0x38(%rbp)
// 0x1421ac778  4c8965c0                       movq	%r12, -0x40(%rbp)
// 0x1421ac77c  4d8b5628                       movq	0x28(%r14), %r10
// Frames: F21 -> F124 -> F125 -> F126 -> F122 -> F123 -> F121 -> F109 -> F110
// 0x1421ac780  4189d3                         movl	%edx, %r11d
// 0x1421ac783  41c1eb06                       shrl	$0x6, %r11d
// Frames: F127 -> F125 -> F126 -> F122 -> F123 -> F121 -> F109 -> F110
// 0x1421ac787  4f8b14da                       movq	(%r10,%r11,8), %r10
// Frames: F125 -> F126 -> F122 -> F123 -> F121 -> F109 -> F110
// 0x1421ac78b  490fa3d2                       btq	%rdx, %r10
// Frames: F128 -> F109 -> F110
// 0x1421ac78f  0f83a3060000                   jae	0x1421ace38 <.text+0x21abe38>
// Frames: F25 -> F26 -> F27 -> F112 -> F17 -> F18 -> F129 -> F130 -> F109 -> F110
// 0x1421ac795  4c8b6808                       movq	0x8(%rax), %r13
// 0x1421ac799  440fb66018                     movzbl	0x18(%rax), %r12d
// 0x1421ac79e  418b410c                       movl	0xc(%r9), %eax
// 0x1421ac7a2  418b4910                       movl	0x10(%r9), %ecx
// 0x1421ac7a6  488b93a8010000                 movq	0x1a8(%rbx), %rdx
// Frames: F114 -> F111 -> F131 -> F129 -> F130 -> F109 -> F110
// 0x1421ac7ad  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F132 -> F109 -> F110
// 0x1421ac7b1  4d8b8610010000                 movq	0x110(%r14), %r8
// Frames: F133 -> F134 -> F135 -> F136 -> F137 -> F132 -> F109 -> F110
// 0x1421ac7b8  4c8b4cc218                     movq	0x18(%rdx,%rax,8), %r9
// 0x1421ac7bd  488b44c238                     movq	0x38(%rdx,%rax,8), %rax
// Frames: F138 -> F139 -> F140 -> F141 -> F133 -> F134 -> F135 -> F136 -> F137 -> F132 -> F109 -> F110
// 0x1421ac7c2  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F142 -> F143 -> F57 -> F133 -> F134 -> F135 -> F136 -> F137 -> F132 -> F109 -> F110
// 0x1421ac7c6  48f7d0                         notq	%rax
// Frames: F144 -> F145 -> F146 -> F57 -> F147 -> F136 -> F137 -> F132 -> F109 -> F110
// 0x1421ac7c9  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1421ac7cd  48c1e004                       shlq	$0x4, %rax
// Frames: F21 -> F148 -> F149 -> F150 -> F151 -> F152 -> F109 -> F110
// 0x1421ac7d1  4c6bf958                       imulq	$0x58, %rcx, %r15
// 0x1421ac7d5  4d037c0110                     addq	0x10(%r9,%rax), %r15
// Frames: F153
// 0x1421ac7da  4c89f9                         movq	%r15, %rcx
// 0x1421ac7dd  e86e6a73fe                     callq	0x1408e3250 <.text+0x8e2250>
// 0x1421ac7e2  884508                         movb	%al, 0x8(%rbp)
// Frames: F154 -> F153
// 0x1421ac7e5  4038c7                         cmpb	%al, %dil
// Frames: F153
// 0x1421ac7e8  0f8509060000                   jne	0x1421acdf7 <.text+0x21abdf7>
// Frames: F155
// 0x1421ac7ee  4c8bb5b8000000                 movq	0xb8(%rbp), %r14
// 0x1421ac7f5  4084ff                         testb	%dil, %dil
// 0x1421ac7f8  0f84b2020000                   je	0x1421acab0 <.text+0x21abab0>
// Frames: F156 -> F157 -> F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165
// 0x1421ac7fe  488b06                         movq	(%rsi), %rax
// 0x1421ac801  488b7e10                       movq	0x10(%rsi), %rdi
// Frames: F166 -> F157 -> F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165
// 0x1421ac805  4829f8                         subq	%rdi, %rax
// Frames: F157 -> F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165
// 0x1421ac808  4883f817                       cmpq	$0x17, %rax
// Frames: F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165
// 0x1421ac80c  0f8644050000                   jbe	0x1421acd56 <.text+0x21abd56>
// Frames: F167
// 0x1421ac812  488b4608                       movq	0x8(%rsi), %rax
// 0x1421ac816  488d0dc324c2fe                 leaq	-0x13ddb3d(%rip), %rcx  # 0x140dcece0 <.text+0xdcdce0>
// 0x1421ac81d  48890c38                       movq	%rcx, (%rax,%rdi)
// 0x1421ac821  488d0de82bc2fe                 leaq	-0x13dd418(%rip), %rcx  # 0x140dcf410 <.text+0xdce410>
// 0x1421ac828  48894c3808                     movq	%rcx, 0x8(%rax,%rdi)
// 0x1421ac82d  4c89743810                     movq	%r14, 0x10(%rax,%rdi)
// 0x1421ac832  4883c718                       addq	$0x18, %rdi
// 0x1421ac836  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x1421ac83a  488d85b8000000                 leaq	0xb8(%rbp), %rax
// 0x1421ac841  48894508                       movq	%rax, 0x8(%rbp)
// 0x1421ac845  488d05b4e3b3fe                 leaq	-0x14c1c4c(%rip), %rax  # 0x140ceac00 <.text+0xce9c00>
// 0x1421ac84c  48894510                       movq	%rax, 0x10(%rbp)
// Frames: F168 -> F169 -> F170 -> F167
// 0x1421ac850  488d05d9afe400                 leaq	0xe4afd9(%rip), %rax    # 0x142ff7830
// 0x1421ac857  48894560                       movq	%rax, 0x60(%rbp)
// 0x1421ac85b  48c7456801000000               movq	$0x1, 0x68(%rbp)
// 0x1421ac863  488d4508                       leaq	0x8(%rbp), %rax
// 0x1421ac867  48894570                       movq	%rax, 0x70(%rbp)
// 0x1421ac86b  48c7457801000000               movq	$0x1, 0x78(%rbp)
// 0x1421ac873  48c7858000000000000000         movq	$0x0, 0x80(%rbp)
// 0x1421ac87e  488d4da8                       leaq	-0x58(%rbp), %rcx
// 0x1421ac882  488d5560                       leaq	0x60(%rbp), %rdx
// 0x1421ac886  e805d7bcfe                     callq	0x140d79f90 <.text+0xd78f90>
// Frames: F171 -> F167
// 0x1421ac88b  488b45a8                       movq	-0x58(%rbp), %rax
// 0x1421ac88f  488985d0000000                 movq	%rax, 0xd0(%rbp)
// 0x1421ac896  488b55b0                       movq	-0x50(%rbp), %rdx
// 0x1421ac89a  4c8b45b8                       movq	-0x48(%rbp), %r8
// Frames: F167
// 0x1421ac89e  488b4d30                       movq	0x30(%rbp), %rcx
// 0x1421ac8a2  48895520                       movq	%rdx, 0x20(%rbp)
// 0x1421ac8a6  e8c50c78fe                     callq	0x14092d570 <.text+0x92c570>
// Frames: F172
// 0x1421ac8ab  4883f801                       cmpq	$0x1, %rax
// 0x1421ac8af  751b                           jne	0x1421ac8cc <.text+0x21ab8cc>
// Frames: F173
// 0x1421ac8b1  4889d7                         movq	%rdx, %rdi
// 0x1421ac8b4  488b4d30                       movq	0x30(%rbp), %rcx
// 0x1421ac8b8  e8935be5fd                     callq	0x140002450 <.text+0x1450>
// 0x1421ac8bd  488b08                         movq	(%rax), %rcx
// Frames: F174 -> F173
// 0x1421ac8c0  4883c110                       addq	$0x10, %rcx
// Frames: F173
// 0x1421ac8c4  4889fa                         movq	%rdi, %rdx
// 0x1421ac8c7  e8a4b78ffe                     callq	0x140aa8070 <.text+0xaa7070>
// Frames: F175 -> F176 -> F177 -> F100 -> F100 -> F100 -> F178
// 0x1421ac8cc  488b95d0000000                 movq	0xd0(%rbp), %rdx
// 0x1421ac8d3  4885d2                         testq	%rdx, %rdx
// 0x1421ac8d6  740f                           je	0x1421ac8e7 <.text+0x21ab8e7>
// Frames: F179 -> F180 -> F181 -> F177 -> F100 -> F100 -> F100 -> F178
// 0x1421ac8d8  41b801000000                   movl	$0x1, %r8d
// 0x1421ac8de  488b4d20                       movq	0x20(%rbp), %rcx
// 0x1421ac8e2  e8397346fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F182
// 0x1421ac8e7  488d4d70                       leaq	0x70(%rbp), %rcx
// 0x1421ac8eb  4c89fa                         movq	%r15, %rdx
// 0x1421ac8ee  e82d6973fe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F183
// 0x1421ac8f3  c6456000                       movb	$0x0, 0x60(%rbp)
// 0x1421ac8f7  8b85dc000000                   movl	0xdc(%rbp), %eax
// 0x1421ac8fd  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F69 -> F70 -> F32 -> F183
// 0x1421ac901  8901                           movl	%eax, (%rcx)
// 0x1421ac903  488b85c0000000                 movq	0xc0(%rbp), %rax
// Frames: F72 -> F30 -> F73 -> F32 -> F183
// 0x1421ac90a  488b7830                       movq	0x30(%rax), %rdi
// Frames: F74 -> F73 -> F32 -> F183
// 0x1421ac90e  488b5840                       movq	0x40(%rax), %rbx
// Frames: F75 -> F30 -> F73 -> F32 -> F183
// 0x1421ac912  483b7820                       cmpq	0x20(%rax), %rdi
// 0x1421ac916  7510                           jne	0x1421ac928 <.text+0x21ab928>
// Frames: F76 -> F30 -> F73 -> F32 -> F183
// 0x1421ac918  488b4d48                       movq	0x48(%rbp), %rcx
// 0x1421ac91c  488d154d649c00                 leaq	0x9c644d(%rip), %rdx    # 0x142b72d70
// 0x1421ac923  e888613bfe                     callq	0x140562ab0 <.text+0x561ab0>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F73 -> F32 -> F183
// 0x1421ac928  488b95c0000000                 movq	0xc0(%rbp), %rdx
// 0x1421ac92f  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F73 -> F32 -> F183
// 0x1421ac933  488d0cbf                       leaq	(%rdi,%rdi,4), %rcx
// Frames: F35 -> F36 -> F30 -> F73 -> F32 -> F183
// 0x1421ac937  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x1421ac93b  0f104560                       movups	0x60(%rbp), %xmm0
// 0x1421ac93f  0f104d6c                       movups	0x6c(%rbp), %xmm1
// 0x1421ac943  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1421ac948  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// Frames: F37 -> F30 -> F73 -> F32 -> F183
// 0x1421ac94d  48ffc7                         incq	%rdi
// 0x1421ac950  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F38 -> F73 -> F32 -> F183
// 0x1421ac954  48ff4240                       incq	0x40(%rdx)
// Frames: F184
// 0x1421ac958  4c8bb5b8000000                 movq	0xb8(%rbp), %r14
// Frames: F81 -> F184
// 0x1421ac95f  31c9                           xorl	%ecx, %ecx
// 0x1421ac961  e8ea6868fe                     callq	0x140833250 <.text+0x832250>
// Frames: F83 -> F81 -> F184
// 0x1421ac966  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x1421ac96d  89c2                           movl	%eax, %edx
// 0x1421ac96f  e8ecf367fe                     callq	0x14082bd60 <.text+0x82ad60>
// 0x1421ac974  488d4570                       leaq	0x70(%rbp), %rax
// Frames: F84 -> F85 -> F184
// 0x1421ac978  c7400800000000                 movl	$0x0, 0x8(%rax)
// 0x1421ac97f  48c70000000000                 movq	$0x0, (%rax)
// Frames: F86 -> F184
// 0x1421ac986  e8f5f1e8fd                     callq	0x14003bb80 <.text+0x3ab80>
// Frames: F87 -> F184
// 0x1421ac98b  c685a000000009                 movb	$0x9, 0xa0(%rbp)
// 0x1421ac992  0f297560                       movaps	%xmm6, 0x60(%rbp)
// 0x1421ac996  488d8590000000                 leaq	0x90(%rbp), %rax
// 0x1421ac99d  48b90000803f0000803f           movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
// 0x1421ac9a7  488948ec                       movq	%rcx, -0x14(%rax)
// 0x1421ac9ab  c740f40000803f                 movl	$0x3f800000, -0xc(%rax) # imm = 0x3F800000
// Frames: F156 -> F157 -> F158 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F184
// 0x1421ac9b2  488b06                         movq	(%rsi), %rax
// 0x1421ac9b5  488b7e10                       movq	0x10(%rsi), %rdi
// Frames: F166 -> F157 -> F158 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F184
// 0x1421ac9b9  4829f8                         subq	%rdi, %rax
// Frames: F157 -> F158 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F184
// 0x1421ac9bc  483d87000000                   cmpq	$0x87, %rax
// Frames: F158 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F184
// 0x1421ac9c2  0f86d8030000                   jbe	0x1421acda0 <.text+0x21abda0>
// Frames: F188
// 0x1421ac9c8  488b4608                       movq	0x8(%rsi), %rax
// 0x1421ac9cc  488d0dcd26c2fe                 leaq	-0x13dd933(%rip), %rcx  # 0x140dcf0a0 <.text+0xdce0a0>
// 0x1421ac9d3  48890c38                       movq	%rcx, (%rax,%rdi)
// 0x1421ac9d7  0f284560                       movaps	0x60(%rbp), %xmm0
// 0x1421ac9db  0f284d70                       movaps	0x70(%rbp), %xmm1
// 0x1421ac9df  0f289580000000                 movaps	0x80(%rbp), %xmm2
// 0x1421ac9e6  0f289d90000000                 movaps	0x90(%rbp), %xmm3
// 0x1421ac9ed  0f11443808                     movups	%xmm0, 0x8(%rax,%rdi)
// 0x1421ac9f2  0f114c3818                     movups	%xmm1, 0x18(%rax,%rdi)
// 0x1421ac9f7  0f11543828                     movups	%xmm2, 0x28(%rax,%rdi)
// 0x1421ac9fc  0f115c3838                     movups	%xmm3, 0x38(%rax,%rdi)
// 0x1421aca01  0f2885a0000000                 movaps	0xa0(%rbp), %xmm0
// 0x1421aca08  0f11443848                     movups	%xmm0, 0x48(%rax,%rdi)
// 0x1421aca0d  c644385800                     movb	$0x0, 0x58(%rax,%rdi)
// 0x1421aca12  4c89743868                     movq	%r14, 0x68(%rax,%rdi)
// 0x1421aca17  488d0d6230c2fe                 leaq	-0x13dcf9e(%rip), %rcx  # 0x140dcfa80 <.text+0xdcea80>
// 0x1421aca1e  48894c3878                     movq	%rcx, 0x78(%rax,%rdi)
// 0x1421aca23  4881c788000000                 addq	$0x88, %rdi
// 0x1421aca2a  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x1421aca2e  f3410f104734                   movss	0x34(%r15), %xmm0
// 0x1421aca34  0f2ec7                         ucomiss	%xmm7, %xmm0
// 0x1421aca37  0f8573020000                   jne	0x1421accb0 <.text+0x21abcb0>
// 0x1421aca3d  0f8a6d020000                   jp	0x1421accb0 <.text+0x21abcb0>
// Frames: F189
// 0x1421aca43  4d8b37                         movq	(%r15), %r14
// 0x1421aca46  8b85e0000000                   movl	0xe0(%rbp), %eax
// 0x1421aca4c  488b4d38                       movq	0x38(%rbp), %rcx
// Frames: F69 -> F70 -> F32 -> F190
// 0x1421aca50  8901                           movl	%eax, (%rcx)
// 0x1421aca52  488b85c8000000                 movq	0xc8(%rbp), %rax
// Frames: F72 -> F30 -> F73 -> F32 -> F190
// 0x1421aca59  488b7830                       movq	0x30(%rax), %rdi
// Frames: F74 -> F73 -> F32 -> F190
// 0x1421aca5d  488b5840                       movq	0x40(%rax), %rbx
// Frames: F75 -> F30 -> F73 -> F32 -> F190
// 0x1421aca61  483b7820                       cmpq	0x20(%rax), %rdi
// 0x1421aca65  7510                           jne	0x1421aca77 <.text+0x21aba77>
// Frames: F76 -> F30 -> F73 -> F32 -> F190
// 0x1421aca67  488b4d58                       movq	0x58(%rbp), %rcx
// 0x1421aca6b  488d15fe629c00                 leaq	0x9c62fe(%rip), %rdx    # 0x142b72d70
// 0x1421aca72  e8b95b3bfe                     callq	0x140562630 <.text+0x561630>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F73 -> F32 -> F190
// 0x1421aca77  488b95c8000000                 movq	0xc8(%rbp), %rdx
// 0x1421aca7e  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F73 -> F32 -> F190
// 0x1421aca82  4889f9                         movq	%rdi, %rcx
// 0x1421aca85  48c1e104                       shlq	$0x4, %rcx
// Frames: F35 -> F36 -> F30 -> F73 -> F32 -> F190
// 0x1421aca89  48891c08                       movq	%rbx, (%rax,%rcx)
// 0x1421aca8d  4c89740808                     movq	%r14, 0x8(%rax,%rcx)
// Frames: F37 -> F30 -> F73 -> F32 -> F190
// 0x1421aca92  48ffc7                         incq	%rdi
// 0x1421aca95  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F38 -> F73 -> F32 -> F190
// 0x1421aca99  48ff4240                       incq	0x40(%rdx)
// Frames: F188
// 0x1421aca9d  e90e020000                     jmp	0x1421accb0 <.text+0x21abcb0>
// 0x1421acaa2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F81 -> F191
// 0x1421acab0  31c9                           xorl	%ecx, %ecx
// 0x1421acab2  e8996768fe                     callq	0x140833250 <.text+0x832250>
// 0x1421acab7  488dbd90000000                 leaq	0x90(%rbp), %rdi
// Frames: F83 -> F81 -> F191
// 0x1421acabe  4889f9                         movq	%rdi, %rcx
// 0x1421acac1  89c2                           movl	%eax, %edx
// 0x1421acac3  e898f267fe                     callq	0x14082bd60 <.text+0x82ad60>
// Frames: F84 -> F85 -> F191
// 0x1421acac8  c747e800000000                 movl	$0x0, -0x18(%rdi)
// 0x1421acacf  48c747e000000000               movq	$0x0, -0x20(%rdi)
// Frames: F86 -> F191
// 0x1421acad7  e8a4f0e8fd                     callq	0x14003bb80 <.text+0x3ab80>
// Frames: F87 -> F191
// 0x1421acadc  c685a000000009                 movb	$0x9, 0xa0(%rbp)
// 0x1421acae3  0f297560                       movaps	%xmm6, 0x60(%rbp)
// 0x1421acae7  48b80000803f0000803f           movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
// 0x1421acaf1  488947ec                       movq	%rax, -0x14(%rdi)
// 0x1421acaf5  c747f40000803f                 movl	$0x3f800000, -0xc(%rdi) # imm = 0x3F800000
// Frames: F156 -> F157 -> F158 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F191
// 0x1421acafc  488b06                         movq	(%rsi), %rax
// 0x1421acaff  488b7e10                       movq	0x10(%rsi), %rdi
// Frames: F166 -> F157 -> F158 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F191
// 0x1421acb03  4829f8                         subq	%rdi, %rax
// Frames: F157 -> F158 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F191
// 0x1421acb06  483d87000000                   cmpq	$0x87, %rax
// Frames: F158 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F191
// 0x1421acb0c  0f8669020000                   jbe	0x1421acd7b <.text+0x21abd7b>
// Frames: F192
// 0x1421acb12  488b4608                       movq	0x8(%rsi), %rax
// 0x1421acb16  488d0d8325c2fe                 leaq	-0x13dda7d(%rip), %rcx  # 0x140dcf0a0 <.text+0xdce0a0>
// 0x1421acb1d  48890c38                       movq	%rcx, (%rax,%rdi)
// 0x1421acb21  0f284560                       movaps	0x60(%rbp), %xmm0
// 0x1421acb25  0f284d70                       movaps	0x70(%rbp), %xmm1
// 0x1421acb29  0f289580000000                 movaps	0x80(%rbp), %xmm2
// 0x1421acb30  0f289d90000000                 movaps	0x90(%rbp), %xmm3
// 0x1421acb37  0f11443808                     movups	%xmm0, 0x8(%rax,%rdi)
// 0x1421acb3c  0f114c3818                     movups	%xmm1, 0x18(%rax,%rdi)
// 0x1421acb41  0f11543828                     movups	%xmm2, 0x28(%rax,%rdi)
// 0x1421acb46  0f115c3838                     movups	%xmm3, 0x38(%rax,%rdi)
// 0x1421acb4b  0f2885a0000000                 movaps	0xa0(%rbp), %xmm0
// 0x1421acb52  0f11443848                     movups	%xmm0, 0x48(%rax,%rdi)
// 0x1421acb57  c644385800                     movb	$0x0, 0x58(%rax,%rdi)
// 0x1421acb5c  4c89743868                     movq	%r14, 0x68(%rax,%rdi)
// 0x1421acb61  488d0d182fc2fe                 leaq	-0x13dd0e8(%rip), %rcx  # 0x140dcfa80 <.text+0xdcea80>
// 0x1421acb68  48894c3878                     movq	%rcx, 0x78(%rax,%rdi)
// 0x1421acb6d  4881c788000000                 addq	$0x88, %rdi
// 0x1421acb74  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x1421acb78  410fb65f54                     movzbl	0x54(%r15), %ebx
// Frames: F193
// 0x1421acb7d  488d4d70                       leaq	0x70(%rbp), %rcx
// 0x1421acb81  4c89fa                         movq	%r15, %rdx
// 0x1421acb84  e8976673fe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F194
// 0x1421acb89  c6456006                       movb	$0x6, 0x60(%rbp)
// 0x1421acb8d  885d61                         movb	%bl, 0x61(%rbp)
// 0x1421acb90  8b85dc000000                   movl	0xdc(%rbp), %eax
// 0x1421acb96  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F69 -> F70 -> F32 -> F194
// 0x1421acb9a  8901                           movl	%eax, (%rcx)
// 0x1421acb9c  488b85c0000000                 movq	0xc0(%rbp), %rax
// Frames: F72 -> F30 -> F73 -> F32 -> F194
// 0x1421acba3  488b7830                       movq	0x30(%rax), %rdi
// Frames: F74 -> F73 -> F32 -> F194
// 0x1421acba7  488b5840                       movq	0x40(%rax), %rbx
// Frames: F75 -> F30 -> F73 -> F32 -> F194
// 0x1421acbab  483b7820                       cmpq	0x20(%rax), %rdi
// 0x1421acbaf  7510                           jne	0x1421acbc1 <.text+0x21abbc1>
// Frames: F76 -> F30 -> F73 -> F32 -> F194
// 0x1421acbb1  488b4d48                       movq	0x48(%rbp), %rcx
// 0x1421acbb5  488d15b4619c00                 leaq	0x9c61b4(%rip), %rdx    # 0x142b72d70
// 0x1421acbbc  e8ef5e3bfe                     callq	0x140562ab0 <.text+0x561ab0>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F73 -> F32 -> F194
// 0x1421acbc1  488b95c0000000                 movq	0xc0(%rbp), %rdx
// 0x1421acbc8  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F73 -> F32 -> F194
// 0x1421acbcc  488d0cbf                       leaq	(%rdi,%rdi,4), %rcx
// Frames: F35 -> F36 -> F30 -> F73 -> F32 -> F194
// 0x1421acbd0  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x1421acbd4  0f104560                       movups	0x60(%rbp), %xmm0
// 0x1421acbd8  0f104d6c                       movups	0x6c(%rbp), %xmm1
// 0x1421acbdc  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1421acbe1  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// Frames: F37 -> F30 -> F73 -> F32 -> F194
// 0x1421acbe6  48ffc7                         incq	%rdi
// 0x1421acbe9  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F38 -> F73 -> F32 -> F194
// 0x1421acbed  48ff4240                       incq	0x40(%rdx)
// Frames: F195
// 0x1421acbf1  f3410f104734                   movss	0x34(%r15), %xmm0
// 0x1421acbf7  0f2ec7                         ucomiss	%xmm7, %xmm0
// 0x1421acbfa  755c                           jne	0x1421acc58 <.text+0x21abc58>
// 0x1421acbfc  7a5a                           jp	0x1421acc58 <.text+0x21abc58>
// Frames: F196
// 0x1421acbfe  4d8b37                         movq	(%r15), %r14
// 0x1421acc01  8b85e0000000                   movl	0xe0(%rbp), %eax
// 0x1421acc07  488b4d38                       movq	0x38(%rbp), %rcx
// Frames: F69 -> F70 -> F32 -> F197
// 0x1421acc0b  8901                           movl	%eax, (%rcx)
// 0x1421acc0d  488b85c8000000                 movq	0xc8(%rbp), %rax
// Frames: F72 -> F30 -> F73 -> F32 -> F197
// 0x1421acc14  488b7830                       movq	0x30(%rax), %rdi
// Frames: F74 -> F73 -> F32 -> F197
// 0x1421acc18  488b5840                       movq	0x40(%rax), %rbx
// Frames: F75 -> F30 -> F73 -> F32 -> F197
// 0x1421acc1c  483b7820                       cmpq	0x20(%rax), %rdi
// 0x1421acc20  7510                           jne	0x1421acc32 <.text+0x21abc32>
// Frames: F76 -> F30 -> F73 -> F32 -> F197
// 0x1421acc22  488b4d58                       movq	0x58(%rbp), %rcx
// 0x1421acc26  488d1543619c00                 leaq	0x9c6143(%rip), %rdx    # 0x142b72d70
// 0x1421acc2d  e8fe593bfe                     callq	0x140562630 <.text+0x561630>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F73 -> F32 -> F197
// 0x1421acc32  488b95c8000000                 movq	0xc8(%rbp), %rdx
// 0x1421acc39  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F73 -> F32 -> F197
// 0x1421acc3d  4889f9                         movq	%rdi, %rcx
// 0x1421acc40  48c1e104                       shlq	$0x4, %rcx
// Frames: F35 -> F36 -> F30 -> F73 -> F32 -> F197
// 0x1421acc44  48891c08                       movq	%rbx, (%rax,%rcx)
// 0x1421acc48  4c89740808                     movq	%r14, 0x8(%rax,%rcx)
// Frames: F37 -> F30 -> F73 -> F32 -> F197
// 0x1421acc4d  48ffc7                         incq	%rdi
// 0x1421acc50  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F38 -> F73 -> F32 -> F197
// 0x1421acc54  48ff4240                       incq	0x40(%rdx)
// Frames: F198
// 0x1421acc58  4180fc01                       cmpb	$0x1, %r12b
// 0x1421acc5c  7552                           jne	0x1421accb0 <.text+0x21abcb0>
// Frames: F199
// 0x1421acc5e  488b4d50                       movq	0x50(%rbp), %rcx
// 0x1421acc62  4c89ea                         movq	%r13, %rdx
// 0x1421acc65  4c8d05a4abe400                 leaq	0xe4aba4(%rip), %r8     # 0x142ff7810
// 0x1421acc6c  e8dfd96afe                     callq	0x14085a650 <.text+0x859650>
// 0x1421acc71  f30f104054                     movss	0x54(%rax), %xmm0
// Frames: F200
// 0x1421acc76  410f2ec0                       ucomiss	%xmm8, %xmm0
// 0x1421acc7a  7502                           jne	0x1421acc7e <.text+0x21abc7e>
// 0x1421acc7c  7b32                           jnp	0x1421accb0 <.text+0x21abcb0>
// Frames: F201 -> F202
// 0x1421acc7e  410f28c8                       movaps	%xmm8, %xmm1
// 0x1421acc82  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F203 -> F202
// 0x1421acc86  410f28d1                       movaps	%xmm9, %xmm2
// 0x1421acc8a  f30f5dd1                       minss	%xmm1, %xmm2
// 0x1421acc8e  488b4500                       movq	(%rbp), %rax
// 0x1421acc92  8b8db4000000                   movl	0xb4(%rbp), %ecx
// Frames: F69 -> F70 -> F204
// 0x1421acc98  8908                           movl	%ecx, (%rax)
// 0x1421acc9a  488b4d50                       movq	0x50(%rbp), %rcx
// Frames: F204
// 0x1421acc9e  4c89ea                         movq	%r13, %rdx
// 0x1421acca1  e8bae06afe                     callq	0x14085ad60 <.text+0x859d60>
// 0x1421acca6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F205
// 0x1421accb0  4c8bb5b8000000                 movq	0xb8(%rbp), %r14
// Frames: F156 -> F157 -> F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F205
// 0x1421accb7  488b06                         movq	(%rsi), %rax
// 0x1421accba  488b7e10                       movq	0x10(%rsi), %rdi
// Frames: F166 -> F157 -> F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F205
// 0x1421accbe  4829f8                         subq	%rdi, %rax
// Frames: F157 -> F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F205
// 0x1421accc1  4883f817                       cmpq	$0x17, %rax
// Frames: F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F205
// 0x1421accc5  766a                           jbe	0x1421acd31 <.text+0x21abd31>
// Frames: F206
// 0x1421accc7  488b4608                       movq	0x8(%rsi), %rax
// 0x1421acccb  488d0dee20c2fe                 leaq	-0x13ddf12(%rip), %rcx  # 0x140dcedc0 <.text+0xdcddc0>
// 0x1421accd2  48890c38                       movq	%rcx, (%rax,%rdi)
// 0x1421accd6  488d0d3327c2fe                 leaq	-0x13dd8cd(%rip), %rcx  # 0x140dcf410 <.text+0xdce410>
// 0x1421accdd  48894c3808                     movq	%rcx, 0x8(%rax,%rdi)
// 0x1421acce2  4c89743810                     movq	%r14, 0x10(%rax,%rdi)
// 0x1421acce7  4883c718                       addq	$0x18, %rdi
// 0x1421acceb  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x1421accef  4d8b37                         movq	(%r15), %r14
// 0x1421accf2  8b85b0000000                   movl	0xb0(%rbp), %eax
// 0x1421accf8  488b4df0                       movq	-0x10(%rbp), %rcx
// Frames: F69 -> F95 -> F32 -> F104
// 0x1421accfc  8901                           movl	%eax, (%rcx)
// 0x1421accfe  4c8b7de8                       movq	-0x18(%rbp), %r15
// Frames: F72 -> F30 -> F31 -> F32 -> F104
// 0x1421acd02  498b7f30                       movq	0x30(%r15), %rdi
// Frames: F74 -> F31 -> F32 -> F104
// 0x1421acd06  498b5f40                       movq	0x40(%r15), %rbx
// Frames: F75 -> F30 -> F31 -> F32 -> F104
// 0x1421acd0a  493b7f20                       cmpq	0x20(%r15), %rdi
// 0x1421acd0e  4c8b65c0                       movq	-0x40(%rbp), %r12
// 0x1421acd12  4c8b6dc8                       movq	-0x38(%rbp), %r13
// 0x1421acd16  0f8584f9ffff                   jne	0x1421ac6a0 <.text+0x21ab6a0>
// Frames: F76 -> F30 -> F31 -> F32 -> F104
// 0x1421acd1c  488b4df8                       movq	-0x8(%rbp), %rcx
// 0x1421acd20  488d1549609c00                 leaq	0x9c6049(%rip), %rdx    # 0x142b72d70
// 0x1421acd27  e804593bfe                     callq	0x140562630 <.text+0x561630>
// 0x1421acd2c  e96ff9ffff                     jmp	0x1421ac6a0 <.text+0x21ab6a0>
// Frames: F207 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F205
// 0x1421acd31  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421acd3a  41b818000000                   movl	$0x18, %r8d
// 0x1421acd40  41b901000000                   movl	$0x1, %r9d
// 0x1421acd46  4889f1                         movq	%rsi, %rcx
// 0x1421acd49  4889fa                         movq	%rdi, %rdx
// 0x1421acd4c  e87fdf7200                     callq	0x1428dacd0 <.text+0x28d9cd0>
// 0x1421acd51  e971ffffff                     jmp	0x1421accc7 <.text+0x21abcc7>
// Frames: F207 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165
// 0x1421acd56  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421acd5f  41b818000000                   movl	$0x18, %r8d
// 0x1421acd65  41b901000000                   movl	$0x1, %r9d
// 0x1421acd6b  4889f1                         movq	%rsi, %rcx
// 0x1421acd6e  4889fa                         movq	%rdi, %rdx
// 0x1421acd71  e85adf7200                     callq	0x1428dacd0 <.text+0x28d9cd0>
// 0x1421acd76  e997faffff                     jmp	0x1421ac812 <.text+0x21ab812>
// Frames: F207 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F191
// 0x1421acd7b  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421acd84  41b888000000                   movl	$0x88, %r8d
// 0x1421acd8a  41b901000000                   movl	$0x1, %r9d
// 0x1421acd90  4889f1                         movq	%rsi, %rcx
// 0x1421acd93  4889fa                         movq	%rdi, %rdx
// 0x1421acd96  e835df7200                     callq	0x1428dacd0 <.text+0x28d9cd0>
// 0x1421acd9b  e972fdffff                     jmp	0x1421acb12 <.text+0x21abb12>
// Frames: F207 -> F159 -> F160 -> F161 -> F185 -> F186 -> F187 -> F184
// 0x1421acda0  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421acda9  41b888000000                   movl	$0x88, %r8d
// 0x1421acdaf  41b901000000                   movl	$0x1, %r9d
// 0x1421acdb5  4889f1                         movq	%rsi, %rcx
// 0x1421acdb8  4889fa                         movq	%rdi, %rdx
// 0x1421acdbb  e810df7200                     callq	0x1428dacd0 <.text+0x28d9cd0>
// 0x1421acdc0  e903fcffff                     jmp	0x1421ac9c8 <.text+0x21ab9c8>
// Frames: F208
// 0x1421acdc5  0f28b5f0000000                 movaps	0xf0(%rbp), %xmm6
// 0x1421acdcc  0f28bd00010000                 movaps	0x100(%rbp), %xmm7
// 0x1421acdd3  440f288510010000               movaps	0x110(%rbp), %xmm8
// 0x1421acddb  440f288d20010000               movaps	0x120(%rbp), %xmm9
// 0x1421acde3  4881c4b8010000                 addq	$0x1b8, %rsp            # imm = 0x1B8
// 0x1421acdea  5b                             popq	%rbx
// 0x1421acdeb  5f                             popq	%rdi
// 0x1421acdec  5e                             popq	%rsi
// 0x1421acded  415c                           popq	%r12
// 0x1421acdef  415d                           popq	%r13
// 0x1421acdf1  415e                           popq	%r14
// 0x1421acdf3  415f                           popq	%r15
// 0x1421acdf5  5d                             popq	%rbp
// 0x1421acdf6  c3                             retq
// Frames: F153
// 0x1421acdf7  48c7456000000000               movq	$0x0, 0x60(%rbp)
// 0x1421acdff  488d05f2a9e400                 leaq	0xe4a9f2(%rip), %rax    # 0x142ff77f8
// 0x1421ace06  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421ace0b  488d5508                       leaq	0x8(%rbp), %rdx
// 0x1421ace0f  4c8d85e7000000                 leaq	0xe7(%rbp), %r8
// 0x1421ace16  4c8d4d60                       leaq	0x60(%rbp), %r9
// 0x1421ace1a  31c9                           xorl	%ecx, %ecx
// 0x1421ace1c  e8263a7700                     callq	0x142920847 <.text+0x291f847>
// Frames: F209 -> F108 -> F109 -> F110
// 0x1421ace21  48c1e920                       shrq	$0x20, %rcx
// 0x1421ace25  4989ca                         movq	%rcx, %r10
// Frames: F210 -> F108 -> F109 -> F110
// 0x1421ace28  49c1e220                       shlq	$0x20, %r10
// 0x1421ace2c  4d09c2                         orq	%r8, %r10
// 0x1421ace2f  41b801000000                   movl	$0x1, %r8d
// 0x1421ace35  4c89d1                         movq	%r10, %rcx
// Frames: F211 -> F110
// 0x1421ace38  44894560                       movl	%r8d, 0x60(%rbp)
// 0x1421ace3c  895564                         movl	%edx, 0x64(%rbp)
// 0x1421ace3f  48894d68                       movq	%rcx, 0x68(%rbp)
// 0x1421ace43  488d0596a9e400                 leaq	0xe4a996(%rip), %rax    # 0x142ff77e0
// 0x1421ace4a  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421ace4f  488d0d5aa8e400                 leaq	0xe4a85a(%rip), %rcx    # 0x142ff76b0
// 0x1421ace56  4c8d0d33a8e400                 leaq	0xe4a833(%rip), %r9     # 0x142ff7690
// 0x1421ace5d  4c8d4560                       leaq	0x60(%rbp), %r8
// 0x1421ace61  ba2b000000                     movl	$0x2b, %edx
// 0x1421ace66  e8f5c87200                     callq	0x1428d9760 <.text+0x28d8760>
// 0x1421ace6b  cc                             int3
// 0x1421ace6c  0f1f4000                       nopl	(%rax)
// 0x1421ace70  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421ace75  55                             pushq	%rbp
// 0x1421ace76  4157                           pushq	%r15
// 0x1421ace78  4156                           pushq	%r14
// 0x1421ace7a  4155                           pushq	%r13
// 0x1421ace7c  4154                           pushq	%r12
// 0x1421ace7e  56                             pushq	%rsi
// 0x1421ace7f  57                             pushq	%rdi
// 0x1421ace80  53                             pushq	%rbx
// 0x1421ace81  4883ec68                       subq	$0x68, %rsp
// 0x1421ace85  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421ace8c  440f294c2420                   movaps	%xmm9, 0x20(%rsp)
// 0x1421ace92  440f29442430                   movaps	%xmm8, 0x30(%rsp)
// 0x1421ace98  0f297c2440                     movaps	%xmm7, 0x40(%rsp)
// 0x1421ace9d  0f29742450                     movaps	%xmm6, 0x50(%rsp)
// Frames: F175 -> F176 -> F177 -> F100 -> F100 -> F100 -> F178
// 0x1421acea2  4883bdd000000000               cmpq	$0x0, 0xd0(%rbp)
// 0x1421aceaa  7416                           je	0x1421acec2 <.text+0x21abec2>
// Frames: F179 -> F180 -> F181 -> F177 -> F100 -> F100 -> F100 -> F178
// 0x1421aceac  41b801000000                   movl	$0x1, %r8d
// 0x1421aceb2  488b4d20                       movq	0x20(%rbp), %rcx
// 0x1421aceb6  488b95d0000000                 movq	0xd0(%rbp), %rdx
// 0x1421acebd  e85e6d46fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F102
// 0x1421acec2  0f28742450                     movaps	0x50(%rsp), %xmm6
// 0x1421acec7  0f287c2440                     movaps	0x40(%rsp), %xmm7
// 0x1421acecc  440f28442430                   movaps	0x30(%rsp), %xmm8
// 0x1421aced2  440f284c2420                   movaps	0x20(%rsp), %xmm9
// 0x1421aced8  4883c468                       addq	$0x68, %rsp
// 0x1421acedc  5b                             popq	%rbx
// 0x1421acedd  5f                             popq	%rdi
// 0x1421acede  5e                             popq	%rsi
// 0x1421acedf  415c                           popq	%r12
// 0x1421acee1  415d                           popq	%r13
// 0x1421acee3  415e                           popq	%r14
// 0x1421acee5  415f                           popq	%r15
// 0x1421acee7  5d                             popq	%rbp
// 0x1421acee8  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/create_roof.rs:17:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1235:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs:28:0 bevy_ecs::event::reader::EventReader::read
// F4: crates/systems/roof/src/create_roof.rs:29:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:55:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:89:0 bevy_ecs::event::event_cursor::EventCursor::read_with_id
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:79:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:1860:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:57:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:60:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:659:0 core::num::impl$11::checked_sub
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:366:0 core::slice::index::impl$4::get
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:538:0 core::slice::index::impl$7::get
// F15: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:61:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:62:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:67:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F21: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F22: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F23: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 core::slice::impl$0::iter
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:69:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F28: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1734:0 alloc::vec::Vec::as_mut_ptr
// F29: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2426:0 alloc::vec::Vec::push
// F30: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:139:0 bevy_ecs::event::collections::Events::send_with_caller
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::event::collections::Events::send
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs:75:0 bevy_ecs::event::writer::EventWriter::write
// F33: crates/systems/roof/src/create_roof.rs:86:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 alloc::vec::Vec::as_mut_ptr
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F36: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2427:0 alloc::vec::Vec::push
// F37: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2428:0 alloc::vec::Vec::push
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:140:0 bevy_ecs::event::collections::Events::send_with_caller
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:729:0 core::option::Option::as_mut
// F40: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:333:0 core::iter::adapters::chain::and_then_or_clear
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:89:0 bevy_ecs::event::iterators::impl$3::next
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:15:0 bevy_ecs::event::iterators::impl$0::next
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next::closure$0
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1552:0 core::option::Option::or_else
// F46: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:617:0 core::ptr::non_null::NonNull::add
// F47: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:108:0 core::slice::iter::Iter::post_inc_start
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:443:0 core::slice::iter::impl$186::next_unchecked
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:165:0 core::slice::iter::impl$182::next
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:95:0 bevy_ecs::event::iterators::impl$3::next
// F51: crates/systems/roof/src/create_roof.rs:30:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F52: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:976:0 core::option::Option::unwrap
// F53: crates/systems/roof/src/create_roof.rs:33:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F54: crates/systems/roof/src/create_roof.rs:32:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F55: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1118:0 core::option::Option::map
// F56: crates/systems/roof/src/create_roof.rs:35:0 system_roof::create_roof::create_roof::closure$0
// F57: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F58: crates/systems/roof/src/create_roof.rs:42:0 system_roof::create_roof::create_roof::closure$0
// F59: crates/systems/roof/src/create_roof.rs:45:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F60: crates/systems/roof/src/create_roof.rs:47:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F61: crates/systems/roof/src/create_roof.rs:48:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F62: crates/systems/roof/src/create_roof.rs:51:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F63: crates/systems/roof/src/create_roof.rs:52:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F64: crates/systems/roof/src/create_roof.rs:53:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F65: crates/systems/roof/src/create_roof.rs:46:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F66: crates/systems/roof/src/create_roof.rs:55:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F67: crates/systems/roof/src/create_roof.rs:57:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F68: crates/systems/roof/src/create_roof.rs:59:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F71: crates/systems/roof/src/create_roof.rs:58:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F72: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2419:0 alloc::vec::Vec::push
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::change_detection::impl$38::deref_mut
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:130:0 bevy_ecs::event::collections::Events::send_with_caller
// F75: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2422:0 alloc::vec::Vec::push
// F76: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2423:0 alloc::vec::Vec::push
// F77: crates/systems/roof/src/create_roof.rs:65:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F78: crates/systems/roof/src/create_roof.rs:66:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F79: crates/systems/roof/src/create_roof.rs:67:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F80: crates/systems/roof/src/create_roof.rs:69:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F81: crates/systems/roof/src/visual/switch_roof_type.rs:132:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F82: crates/systems/roof/src/create_roof.rs:72:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F83: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/convert/mod.rs:761:0 core::convert::impl$3::into
// F84: crates/utils/src/components/transform.rs:16:0 utils::components::transform::Transform::identity
// F85: crates/systems/roof/src/visual/switch_roof_type.rs:135:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F86: crates/systems/roof/src/visual/switch_roof_type.rs:136:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F87: crates/systems/roof/src/visual/switch_roof_type.rs:129:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F88: crates/systems/roof/src/create_roof.rs:63:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F89: crates/systems/roof/src/create_roof.rs:75:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F90: crates/systems/roof/src/create_roof.rs:78:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F91: crates/systems/roof/src/create_roof.rs:80:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F92: crates/systems/roof/src/create_roof.rs:84:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F93: crates/systems/roof/src/create_roof.rs:77:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F94: crates/systems/roof/src/create_roof.rs:89:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::event::collections::Events::send
// F96: crates/systems/roof/src/create_roof.rs:92:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F97: crates/systems/roof/src/create_roof.rs:37:0 system_roof::create_roof::create_roof::closure$0
// F98: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/mod.rs:613:0 core::fmt::Arguments::new_v1
// F99: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:978:0 core::option::Option::unwrap
// F100: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:523:0 core::ptr::drop_in_place
// F101: crates/systems/roof/src/create_roof.rs:73:0 system_roof::create_roof::create_roof::hfe2ca6048f3d2061
// F102: crates/systems/roof/src/visual/switch_roof_type.rs:24:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F103: crates/systems/roof/src/visual/switch_roof_type.rs:41:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F104: crates/systems/roof/src/visual/switch_roof_type.rs:121:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F105: crates/systems/roof/src/visual/switch_roof_type.rs:37:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F106: crates/systems/roof/src/visual/switch_roof_type.rs:38:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:843:0 bevy_ecs::entity::Entities::get
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1595:0 bevy_ecs::system::query::Query::get_inner
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1386:0 bevy_ecs::system::query::Query::get
// F110: crates/systems/roof/src/visual/switch_roof_type.rs:43:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F111: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F112: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F113: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 core::slice::impl$0::get
// F114: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F115: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1742:0 core::cmp::impls::impl$24::ne
// F116: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:201:0 core::num::nonzero::impl$2::ne
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:844:0 bevy_ecs::entity::Entities::get
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:76:0 bevy_ecs::archetype::impl$25::eq
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:845:0 bevy_ecs::entity::Entities::get
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1603:0 bevy_ecs::system::query::Query::get_inner
// F122: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/bool.rs:62:0 core::bool::impl$0::then
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F127: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/internal_macros.rs:36:0 fixedbitset::FixedBitSet::get_unchecked
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1600:0 bevy_ecs::system::query::Query::get_inner
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F130: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1632:0 bevy_ecs::system::query::Query::get_inner
// F131: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 alloc::vec::impl$8::deref
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1634:0 bevy_ecs::system::query::Query::get_inner
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:367:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1181:0 bevy_ecs::query::fetch::impl$26::set_table
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1169:0 bevy_ecs::query::fetch::impl$26::set_archetype
// F138: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F139: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F140: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::option::Option::and_then
// F141: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F142: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F143: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:304:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F146: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F149: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1239:0 bevy_ecs::query::fetch::impl$27::fetch::closure$0
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F151: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1234:0 bevy_ecs::query::fetch::impl$27::fetch
// F152: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1638:0 bevy_ecs::system::query::Query::get_inner
// F153: crates/systems/roof/src/visual/switch_roof_type.rs:46:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F154: crates/country-core/src/resources/roofs/roof_shape.rs:31:0 country_core::resources::roofs::roof_shape::impl$11::eq
// F155: crates/systems/roof/src/visual/switch_roof_type.rs:48:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F156: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:513:0 alloc::raw_vec::RawVecInner::capacity
// F157: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:623:0 alloc::raw_vec::RawVecInner::needs_to_grow
// F158: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:558:0 alloc::raw_vec::RawVecInner::reserve
// F159: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:327:0 alloc::raw_vec::RawVec::reserve
// F160: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1272:0 alloc::vec::Vec::reserve
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:0:0 bevy_ecs::system::commands::Commands::queue_internal
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:609:0 bevy_ecs::system::commands::Commands::queue_handled
// F163: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:1849:0 bevy_ecs::system::commands::EntityCommands::queue_handled
// F164: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:1618:0 bevy_ecs::system::commands::EntityCommands::remove
// F165: crates/systems/roof/src/visual/switch_roof_type.rs:50:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F166: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:2022:0 alloc::raw_vec::RawVecInner::capacity
// F167: crates/systems/roof/src/visual/switch_roof_type.rs:54:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F168: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/fmt.rs:654:0 alloc::fmt::format::closure$0
// F169: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1225:0 core::option::Option::map_or_else
// F170: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/fmt.rs:654:0 alloc::fmt::format
// F171: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/hint.rs:600:0 core::hint::must_use
// F172: crates/systems/roof/src/visual/switch_roof_type.rs:53:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F173: crates/systems/roof/src/visual/switch_roof_type.rs:56:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F174: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:2206:0 alloc::sync::impl$29::deref
// F175: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:523:0 alloc::raw_vec::RawVecInner::current_memory
// F176: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:751:0 alloc::raw_vec::RawVecInner::deallocate
// F177: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:402:0 alloc::raw_vec::impl$3::drop
// F178: crates/systems/roof/src/visual/switch_roof_type.rs:57:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F179: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:116:0 alloc::alloc::dealloc
// F180: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:269:0 alloc::alloc::impl$1::deallocate
// F181: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:753:0 alloc::raw_vec::RawVecInner::deallocate
// F182: crates/systems/roof/src/visual/switch_roof_type.rs:61:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F183: crates/systems/roof/src/visual/switch_roof_type.rs:59:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F184: crates/systems/roof/src/visual/switch_roof_type.rs:64:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F185: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:554:0 bevy_ecs::system::commands::Commands::queue
// F186: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:1802:0 bevy_ecs::system::commands::EntityCommands::queue
// F187: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:1348:0 bevy_ecs::system::commands::EntityCommands::insert
// F188: crates/systems/roof/src/visual/switch_roof_type.rs:67:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F189: crates/systems/roof/src/visual/switch_roof_type.rs:69:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F190: crates/systems/roof/src/visual/switch_roof_type.rs:68:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F191: crates/systems/roof/src/visual/switch_roof_type.rs:78:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F192: crates/systems/roof/src/visual/switch_roof_type.rs:83:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F193: crates/systems/roof/src/visual/switch_roof_type.rs:87:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F194: crates/systems/roof/src/visual/switch_roof_type.rs:80:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F195: crates/systems/roof/src/visual/switch_roof_type.rs:91:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F196: crates/systems/roof/src/visual/switch_roof_type.rs:93:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F197: crates/systems/roof/src/visual/switch_roof_type.rs:92:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F198: crates/systems/roof/src/visual/switch_roof_type.rs:104:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F199: crates/systems/roof/src/visual/switch_roof_type.rs:105:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F200: crates/systems/roof/src/visual/switch_roof_type.rs:110:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F201: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1418:0 core::f32::impl$0::clamp
// F202: crates/systems/roof/src/visual/switch_roof_type.rs:108:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F203: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1421:0 core::f32::impl$0::clamp
// F204: crates/systems/roof/src/visual/switch_roof_type.rs:111:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F205: crates/systems/roof/src/visual/switch_roof_type.rs:118:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F206: crates/systems/roof/src/visual/switch_roof_type.rs:122:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F207: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:559:0 alloc::raw_vec::RawVecInner::reserve
// F208: crates/systems/roof/src/visual/switch_roof_type.rs:125:0 system_roof::visual::switch_roof_type::switch_roof_type::ha5cd13de1c7a5004
// F209: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:2003:0 core::result::impl$26::branch
// F210: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:2014:0 core::result::impl$27::from_residual
// F211: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1109:0 core::result::Result::unwrap
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/visual/switch_roof_type.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 221316b9f58bac4e6f46b30321d940421e3b35b85a6fecc5daa0c9be09f231ac
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: switch_roof_type.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3467, 3469
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::switch_roof_type::switch_roof_type
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3469,"record_offset":372,"rva":35308784,"va":5404017904}
// 编译实例 RVAs: 0x21ac4f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002450
//     owner=unresolved_namespace; count=1
//     target_aliases=418 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35300461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8935be5fd","size":5,"soft_fail":false,"target_va":5368718416,"va":5404018872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35065355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8f5f1e8fd","size":5,"soft_fail":false,"target_va":5368953728,"va":5404019078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28937415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8397346fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5404018914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14082bd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26741780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8ecf367fe","size":5,"soft_fail":false,"target_va":5377277280,"va":5404019055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26711830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8ea6868fe","size":5,"soft_fail":false,"target_va":5377307216,"va":5404019041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21178619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e805d7bcfe","size":5,"soft_fail":false,"target_va":5382840208,"va":5404018822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-29662840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e888613bfe","size":5,"soft_fail":false,"target_va":5374356144,"va":5404018979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7528319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e87fdf7200","size":5,"soft_fail":false,"target_va":5411548368,"va":5404020044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7813670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8263a7700","size":5,"soft_fail":false,"target_va":5411833927,"va":5404020252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7522549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8f5c87200","size":5,"soft_fail":false,"target_va":5411542880,"va":5404020326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::try_get_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25686843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8c50c78fe","size":5,"soft_fail":false,"target_va":5378332016,"va":5404018854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24135772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8a4b78ffe","size":5,"soft_fail":false,"target_va":5379883120,"va":5404018887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25990867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e82d6973fe","size":5,"soft_fail":false,"target_va":5378028064,"va":5404018926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25990546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e86e6a73fe","size":5,"soft_fail":false,"target_va":5378028112,"va":5404018653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26551841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8dfd96afe","size":5,"soft_fail":false,"target_va":5377467984,"va":5404019820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26550086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404017904,"raw_hex":"e8bae06afe","size":5,"soft_fail":false,"target_va":5377469792,"va":5404019873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3469,"record_offset":3844,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3469,"record_offset":3956,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3469,"record_offset":6324,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3469,"record_offset":1636,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3469,"record_offset":568,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3469,"record_offset":1284,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3469,"record_offset":4632,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::remove
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3469,"record_offset":3548,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":9430,"source_line":1617,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3469,"record_offset":1976,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3469,"record_offset":6292,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":3469,"record_offset":3928,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3469,"record_offset":3984,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3469,"record_offset":6672,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3469,"record_offset":3520,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":116716,"source_line":31,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_direct] system_roof::visual::switch_roof_type::stone_floor_bundle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/switch_roof_type.rs
//     evidence={"module":3469,"record_offset":4536,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":116704,"source_line":128,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3469,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":372,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3469,"record_offset":4136,"parent_record_offset":4080,"ancestor_record_offset":372,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3469,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":372,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":1476,"parent_record_offset":1444,"ancestor_record_offset":372,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":3692,"parent_record_offset":3664,"ancestor_record_offset":372,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":372,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":372,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":372,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":3752,"parent_record_offset":3720,"ancestor_record_offset":372,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":372,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":372,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":372,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3469,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":372,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3469,"record_offset":1444,"parent_record_offset":1400,"ancestor_record_offset":372,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3469,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":372,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3469,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":372,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3469,"record_offset":1400,"parent_record_offset":1356,"ancestor_record_offset":372,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3469,"record_offset":3664,"parent_record_offset":3632,"ancestor_record_offset":372,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3469,"record_offset":904,"parent_record_offset":656,"ancestor_record_offset":372,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3469,"record_offset":2392,"parent_record_offset":2060,"ancestor_record_offset":372,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3469,"record_offset":1600,"parent_record_offset":1312,"ancestor_record_offset":372,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3469,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":372,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3469,"record_offset":2060,"parent_record_offset":2008,"ancestor_record_offset":372,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3469,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":372,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3469,"record_offset":1356,"parent_record_offset":1312,"ancestor_record_offset":372,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3469,"record_offset":632,"parent_record_offset":592,"ancestor_record_offset":372,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3469,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":372,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3469,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":372,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3469,"record_offset":3340,"parent_record_offset":3316,"ancestor_record_offset":372,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3469,"record_offset":2864,"parent_record_offset":2008,"ancestor_record_offset":372,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3469,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":372,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3469,"record_offset":3316,"parent_record_offset":2008,"ancestor_record_offset":372,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3469,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":372,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3469,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":372,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3469,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":372,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3469,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":372,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3469,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":372,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3469,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":372,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3469,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":372,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3469,"record_offset":2604,"parent_record_offset":2008,"ancestor_record_offset":372,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3469,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":372,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3469,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":372,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3469,"record_offset":4688,"parent_record_offset":4660,"ancestor_record_offset":372,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_handled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3469,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":372,"inlinee":9422,"source_line":604,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3469,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":372,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3469,"record_offset":4660,"parent_record_offset":4632,"ancestor_record_offset":372,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue_handled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3469,"record_offset":3576,"parent_record_offset":3548,"ancestor_record_offset":372,"inlinee":9421,"source_line":1843,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3469,"record_offset":2008,"parent_record_offset":1976,"ancestor_record_offset":372,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3469,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":372,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3469,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":372,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3469,"record_offset":2452,"parent_record_offset":2424,"ancestor_record_offset":372,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3469,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":372,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3469,"record_offset":4572,"parent_record_offset":4536,"ancestor_record_offset":372,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3469,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":372,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3469,"record_offset":1688,"parent_record_offset":1660,"ancestor_record_offset":372,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3469,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":372,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3469,"record_offset":856,"parent_record_offset":824,"ancestor_record_offset":372,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3469,"record_offset":704,"parent_record_offset":656,"ancestor_record_offset":372,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3469,"record_offset":2336,"parent_record_offset":2060,"ancestor_record_offset":372,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3469,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":372,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3469,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":372,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3469,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":372,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3469,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":372,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3469,"record_offset":3100,"parent_record_offset":2964,"ancestor_record_offset":372,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3469,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":372,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3469,"record_offset":1768,"parent_record_offset":1688,"ancestor_record_offset":372,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3469,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3469,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":372,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3469,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":372,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3469,"record_offset":1564,"parent_record_offset":1400,"ancestor_record_offset":372,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3469,"record_offset":3456,"parent_record_offset":2008,"ancestor_record_offset":372,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3469,"record_offset":3484,"parent_record_offset":2008,"ancestor_record_offset":372,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3469,"record_offset":760,"parent_record_offset":656,"ancestor_record_offset":372,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3469,"record_offset":1128,"parent_record_offset":656,"ancestor_record_offset":372,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3469,"record_offset":2160,"parent_record_offset":2132,"ancestor_record_offset":372,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3469,"record_offset":2132,"parent_record_offset":2092,"ancestor_record_offset":372,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3469,"record_offset":824,"parent_record_offset":792,"ancestor_record_offset":372,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3469,"record_offset":792,"parent_record_offset":760,"ancestor_record_offset":372,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3469,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":372,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3469,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":372,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3469,"record_offset":1844,"parent_record_offset":1792,"ancestor_record_offset":372,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3469,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":372,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3469,"record_offset":2424,"parent_record_offset":2008,"ancestor_record_offset":372,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3469,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":372,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3469,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":372,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3469,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":372,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3469,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":372,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   [pdb_inline_transitive] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3469,"record_offset":4600,"parent_record_offset":4536,"ancestor_record_offset":372,"inlinee":45127,"source_line":15,"archive":"_pdb/B/modules/03469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::switch_roof_type::stone_floor_bundle
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C7E0
// 首条位置: {"module":3467,"inlinee":116704}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3467,"record_offset":2448,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3467,"record_offset":2476,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":45127,"source_line":15,"archive":"_pdb/B/modules/03467.jsonl.gz"}
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
// 完整结构及原始类型记录: switch_roof_type.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: switch_roof_type.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: switch_roof_type.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::visual::switch_roof_type::stone_floor_bundle
// 信息 ID: 1457285cb2ad17aa04254012
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::create_roof::create_roof; module=3467, inline_record=2412; 不是独立调用边。
// [PDB 内联宿主] system_roof::visual::switch_roof_type::switch_roof_type; module=3469, inline_record=4536; 不是独立调用边。
// [PDB 内联宿主] system_roof::visual::switch_roof_type::switch_roof_type; module=3469, inline_record=5252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::switch_roof_type::switch_roof_type
// 信息 ID: 7da8e4da88531d10b06f7b4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 97ca56c246a3e7a05fa2db17: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>)> > > @ 0x1a26f70 -> 0x21ac4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>)> > @ 0x1a32378 -> 0x21ac4f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
