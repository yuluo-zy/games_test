// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: create_roof.rs.assembly.json.gz

// FUNCTION system_roof::create_roof::create_roof::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3467:2016 -> host 3467:396 / rva_21aaea0_len_81b; exact_symbolizer_function_frame
//   Function-frame RVA span [0x21ab0f6, 0x21ab129), 9 instructions
//   Function-frame RVA span [0x21ab551, 0x21ab5aa), 14 instructions
//
// FUNCTION system_roof::create_roof::create_roof
// Candidate interfaces: 1dc34b2b460ad5ed063463c3
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3467:396 -> rva_21aaea0_len_81b
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
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/create_roof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1c160ab96ebf135af143ddf1184c2fecace55a3bb7b22ac26460912fc8a83168
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: create_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3467
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
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::create_roof::create_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3467,"record_offset":396,"rva":35303072,"va":5404012192}
// 编译实例 RVAs: 0x21aaea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002370
//     owner=unresolved_namespace; count=1
//     target_aliases=1937 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35295014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8da70e5fd","size":5,"soft_fail":false,"target_va":5368718192,"va":5404013201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35059539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8ad08e9fd","size":5,"soft_fail":false,"target_va":5368953728,"va":5404013262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b450
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33948225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8bffdf9fd","size":5,"soft_fail":false,"target_va":5370066000,"va":5404014220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14082bd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26735967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8a10a68fe","size":5,"soft_fail":false,"target_va":5377277280,"va":5404013242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26706021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e89b7f68fe","size":5,"soft_fail":false,"target_va":5377307216,"va":5404013232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26106624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e800a571fe","size":5,"soft_fail":false,"target_va":5377906288,"va":5404012907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e4520
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25980261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e89b9273fe","size":5,"soft_fail":false,"target_va":5378032928,"va":5404013184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22079120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e87019affe","size":5,"soft_fail":false,"target_va":5381934560,"va":5404013675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcfcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20822379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e89546c2fe","size":5,"soft_fail":false,"target_va":5383191728,"va":5404014102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-29658113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8ff733bfe","size":5,"soft_fail":false,"target_va":5374354992,"va":5404013100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20823540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e80c42c2fe","size":5,"soft_fail":false,"target_va":5383190048,"va":5404013583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7527770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e85add7200","size":5,"soft_fail":false,"target_va":5411541776,"va":5404014001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7527910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8e6dd7200","size":5,"soft_fail":false,"target_va":5411541904,"va":5404013989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_animation::RoofAnimation::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26015306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8b60973fe","size":5,"soft_fail":false,"target_va":5377997888,"va":5404013189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25984567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8c98173fe","size":5,"soft_fail":false,"target_va":5378028464,"va":5404013026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25985045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8eb7f73fe","size":5,"soft_fail":false,"target_va":5378027968,"va":5404013008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25984909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8738073fe","size":5,"soft_fail":false,"target_va":5378028112,"va":5404013016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_tile_sim::RoofTileSimulation::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25632489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e817e178fe","size":5,"soft_fail":false,"target_va":5378380688,"va":5404013172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23595805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8e3f497fe","size":5,"soft_fail":false,"target_va":5380417056,"va":5404012856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24220125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8236e8efe","size":5,"soft_fail":false,"target_va":5379792624,"va":5404012744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16433581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404012192,"raw_hex":"e8533e05ff","size":5,"soft_fail":false,"target_va":5387580064,"va":5404013640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3467,"record_offset":1608,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3467,"record_offset":544,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3467,"record_offset":1256,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":1984,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":1948,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3467,"record_offset":2860,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] system_roof::visual::switch_roof_type::stone_floor_bundle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/switch_roof_type.rs
//     evidence={"module":3467,"record_offset":2412,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":116704,"source_line":128,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":1448,"parent_record_offset":1416,"ancestor_record_offset":396,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":396,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":396,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3467,"record_offset":1416,"parent_record_offset":1372,"ancestor_record_offset":396,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3467,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":396,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3467,"record_offset":1372,"parent_record_offset":1328,"ancestor_record_offset":396,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3467,"record_offset":860,"parent_record_offset":632,"ancestor_record_offset":396,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3467,"record_offset":1572,"parent_record_offset":1284,"ancestor_record_offset":396,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3467,"record_offset":2112,"parent_record_offset":2088,"ancestor_record_offset":396,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3467,"record_offset":1284,"parent_record_offset":1256,"ancestor_record_offset":396,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3467,"record_offset":1328,"parent_record_offset":1284,"ancestor_record_offset":396,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3467,"record_offset":608,"parent_record_offset":568,"ancestor_record_offset":396,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3467,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":396,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3467,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":396,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3467,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":396,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3467,"record_offset":2448,"parent_record_offset":2412,"ancestor_record_offset":396,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3467,"record_offset":2052,"parent_record_offset":2016,"ancestor_record_offset":396,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3467,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":396,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3467,"record_offset":1660,"parent_record_offset":1632,"ancestor_record_offset":396,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3467,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":396,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3467,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":396,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3467,"record_offset":676,"parent_record_offset":632,"ancestor_record_offset":396,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":396,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":1740,"parent_record_offset":1660,"ancestor_record_offset":396,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3467,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3467,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":396,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3467,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":396,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3467,"record_offset":1536,"parent_record_offset":1372,"ancestor_record_offset":396,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":732,"parent_record_offset":632,"ancestor_record_offset":396,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":1100,"parent_record_offset":632,"ancestor_record_offset":396,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3467,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":396,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3467,"record_offset":760,"parent_record_offset":732,"ancestor_record_offset":396,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3467,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":396,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3467,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":396,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3467,"record_offset":1816,"parent_record_offset":1764,"ancestor_record_offset":396,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3467,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":396,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::create_roof::create_roof::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/create_roof.rs
//     evidence={"module":3467,"record_offset":2016,"parent_record_offset":1984,"ancestor_record_offset":396,"inlinee":116702,"source_line":34,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3467,"record_offset":2476,"parent_record_offset":2412,"ancestor_record_offset":396,"inlinee":45127,"source_line":15,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::create_roof::create_roof::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C7DE
// 首条位置: {"module":3467,"inlinee":116702}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3467,"record_offset":2052,"parent_record_offset":2016,"ancestor_record_offset":2016,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: create_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [已有逆向证据 B / native_observed_partial_layout; not full original struct declaration] B Roof creation Bundle component graph
// name: "B Roof creation Bundle component graph"
// baseline: "B"
// status: "native_observed_partial_layout; not full original struct declaration"
// alignment: "unknown"
// complete_size: "unknown"
// unobserved_fields: "unknown"
// original_field_names: "unknown unless explicitly established"
// evidence: [{"path": "evidence/building-nani/findings.md", "sha256": "fb45340911620628e66e4edf7a50c4902bf8ccb4f50c994e08a7d3c256565571"}]
// explicit_components: ["Roof", "RoofTileSimulation", "RoofTriggerVolume", "RoofAnimation", "RoofTiles", "RoofSpire", "RoofSupportBeams", "NaniInstanceDesc", "utils::components::transform::Transform", "InstanceDataMarker<StoneFloorInstanceData>"]
// required_component_edges: []
// zst_markers: ["RoofSpire", "RoofSupportBeams", "InstanceDataMarker<StoneFloorInstanceData>"]
// known_component_storage: {"NaniInstanceDesc_stride": 24, "Transform_stride": 48, "NaniBundle_copy_span": 80}
// limitations: ["ZST markers do not prove generated support/spire geometry", "self registration does not form require-self edges", "Bundle copy span includes alignment/padding; original Rust repr unknown"]
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: create_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/create_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: create_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::create_roof::create_roof
// 信息 ID: 2dbd12bd298e3260a6d77f7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1dc34b2b460ad5ed063463c3: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)> > > @ 0x1a29e4c -> 0x21aaea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)> > @ 0x1a33707 -> 0x21aaea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::create_roof::create_roof::closure$0
// 信息 ID: 80308d5b4769caa36bf0c22e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::create_roof::create_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::create_roof::create_roof; module=3467, inline_record=2016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

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
