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
// Inline site 3196:2232 -> host 3196:564 / rva_1988750_len_7a5; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1988b24, 0x1988b64), 11 instructions
// Inline site 3198:4396 -> host 3198:852 / rva_1989be0_len_aab; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x1989ee8, 0x1989f3d), 15 instructions
//   Function-frame RVA span [0x198a13b, 0x198a190), 15 instructions
// Inline site 3198:6132 -> host 3198:852 / rva_1989be0_len_aab; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x1989ee8, 0x1989f3d), 15 instructions
//   Function-frame RVA span [0x198a13b, 0x198a190), 15 instructions
//
// FUNCTION system_roof::visual::switch_roof_type::switch_roof_type
// Candidate interfaces: 282da8d2586e414caff12814
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3198:852 -> rva_1989be0_len_aab
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1988750_len_7a5: RVA 0x1988750, 1957 PDB bytes; 412 instructions
// Evidence: _assembly/roof-v1/A/pack-002/rva_1988750_len_7a5.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3196:564: system_roof::create_roof::create_roof
// Frames: F1
// 0x141988750  55                             pushq	%rbp
// 0x141988751  4157                           pushq	%r15
// 0x141988753  4156                           pushq	%r14
// 0x141988755  4155                           pushq	%r13
// 0x141988757  4154                           pushq	%r12
// 0x141988759  56                             pushq	%rsi
// 0x14198875a  57                             pushq	%rdi
// 0x14198875b  53                             pushq	%rbx
// 0x14198875c  4881ec98030000                 subq	$0x398, %rsp            # imm = 0x398
// 0x141988763  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14198876b  440f298d00030000               movaps	%xmm9, 0x300(%rbp)
// 0x141988773  440f2985f0020000               movaps	%xmm8, 0x2f0(%rbp)
// 0x14198877b  0f29bde0020000                 movaps	%xmm7, 0x2e0(%rbp)
// 0x141988782  0f29b5d0020000                 movaps	%xmm6, 0x2d0(%rbp)
// Frames: F2 -> F3 -> F4
// 0x141988789  48c785c8020000feffffff         movq	$-0x2, 0x2c8(%rbp)
// 0x141988794  4c8b21                         movq	(%rcx), %r12
// Frames: F5 -> F3 -> F4
// 0x141988797  4c8b5108                       movq	0x8(%rcx), %r10
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x14198879b  498b3424                       movq	(%r12), %rsi
// 0x14198879f  4531db                         xorl	%r11d, %r11d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x1419887a2  4889f0                         movq	%rsi, %rax
// 0x1419887a5  492b4218                       subq	0x18(%r10), %rax
// 0x1419887a9  490f42c3                       cmovbq	%r11, %rax
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x1419887ad  492b7238                       subq	0x38(%r10), %rsi
// 0x1419887b1  490f42f3                       cmovbq	%r11, %rsi
// Frames: F12 -> F13 -> F14 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x1419887b5  4c6be838                       imulq	$0x38, %rax, %r13
// 0x1419887b9  4d036a08                       addq	0x8(%r10), %r13
// Frames: F15 -> F7 -> F8 -> F3 -> F4
// 0x1419887bd  498b7a10                       movq	0x10(%r10), %rdi
// Frames: F16 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x1419887c1  4829c7                         subq	%rax, %rdi
// 0x1419887c4  490f42fb                       cmovbq	%r11, %rdi
// 0x1419887c8  4c8bbd80030000                 movq	0x380(%rbp), %r15
// 0x1419887cf  488d1ddafa9400                 leaq	0x94fada(%rip), %rbx    # 0x1422d82b0
// 0x1419887d6  4c0f42eb                       cmovbq	%rbx, %r13
// Frames: F17 -> F7 -> F8 -> F3 -> F4
// 0x1419887da  4d8b7230                       movq	0x30(%r10), %r14
// Frames: F12 -> F13 -> F14 -> F17 -> F7 -> F8 -> F3 -> F4
// 0x1419887de  486bc638                       imulq	$0x38, %rsi, %rax
// 0x1419887e2  49034228                       addq	0x28(%r10), %rax
// 0x1419887e6  4c39f6                         cmpq	%r14, %rsi
// Frames: F16 -> F17 -> F7 -> F8 -> F3 -> F4
// 0x1419887e9  480f47c3                       cmovaq	%rbx, %rax
// 0x1419887ed  4929f6                         subq	%rsi, %r14
// 0x1419887f0  4d0f42f3                       cmovbq	%r11, %r14
// Frames: F18 -> F7 -> F8 -> F3 -> F4
// 0x1419887f4  4d8b5240                       movq	0x40(%r10), %r10
// 0x1419887f8  4d29f2                         subq	%r14, %r10
// 0x1419887fb  4929fa                         subq	%rdi, %r10
// 0x1419887fe  4c89a548020000                 movq	%r12, 0x248(%rbp)
// 0x141988805  4d891424                       movq	%r10, (%r12)
// Frames: F19 -> F20 -> F21 -> F22 -> F7 -> F8 -> F3 -> F4
// 0x141988809  4c6bd738                       imulq	$0x38, %rdi, %r10
// 0x14198880d  4d01ea                         addq	%r13, %r10
// 0x141988810  4c899568020000                 movq	%r10, 0x268(%rbp)
// 0x141988817  4d6bd638                       imulq	$0x38, %r14, %r10
// 0x14198881b  4901c2                         addq	%rax, %r10
// 0x14198881e  4c899570020000                 movq	%r10, 0x270(%rbp)
// 0x141988825  488b0a                         movq	(%rdx), %rcx
// 0x141988828  418b511c                       movl	0x1c(%r9), %edx
// 0x14198882c  8995c0020000                   movl	%edx, 0x2c0(%rbp)
// 0x141988832  498b11                         movq	(%r9), %rdx
// 0x141988835  4d8b4910                       movq	0x10(%r9), %r9
// 0x141988839  4c898d78020000                 movq	%r9, 0x278(%rbp)
// 0x141988840  458b4f1c                       movl	0x1c(%r15), %r9d
// 0x141988844  44898db8020000                 movl	%r9d, 0x2b8(%rbp)
// 0x14198884b  4d8b27                         movq	(%r15), %r12
// 0x14198884e  4d8b4f10                       movq	0x10(%r15), %r9
// 0x141988852  4c898d58020000                 movq	%r9, 0x258(%rbp)
// 0x141988859  458b481c                       movl	0x1c(%r8), %r9d
// 0x14198885d  44898dbc020000                 movl	%r9d, 0x2bc(%rbp)
// 0x141988864  4d8b30                         movq	(%r8), %r14
// 0x141988867  4d8b4010                       movq	0x10(%r8), %r8
// 0x14198886b  4c898560020000                 movq	%r8, 0x260(%rbp)
// 0x141988872  440f280d76258300               movaps	0x832576(%rip), %xmm9   # 0x1421badf0
// 0x14198887a  488d7558                       leaq	0x58(%rbp), %rsi
// 0x14198887e  488995b0020000                 movq	%rdx, 0x2b0(%rbp)
// 0x141988885  4883c220                       addq	$0x20, %rdx
// 0x141988889  48899590020000                 movq	%rdx, 0x290(%rbp)
// 0x141988890  498d542420                     leaq	0x20(%r12), %rdx
// 0x141988895  48899580020000                 movq	%rdx, 0x280(%rbp)
// 0x14198889c  498d5620                       leaq	0x20(%r14), %rdx
// 0x1419888a0  48899588020000                 movq	%rdx, 0x288(%rbp)
// 0x1419888a7  48898d50020000                 movq	%rcx, 0x250(%rbp)
// 0x1419888ae  eb38                           jmp	0x1419888e8 <.text+0x19878e8>
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F28
// 0x1419888b0  498b4628                       movq	0x28(%r14), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F28
// 0x1419888b4  48c1e205                       shlq	$0x5, %rdx
// Frames: F30 -> F31 -> F26 -> F27 -> F28
// 0x1419888b8  48891c10                       movq	%rbx, (%rax,%rdx)
// 0x1419888bc  4c896c1008                     movq	%r13, 0x8(%rax,%rdx)
// 0x1419888c1  48897c1010                     movq	%rdi, 0x10(%rax,%rdx)
// 0x1419888c6  c644101800                     movb	$0x0, 0x18(%rax,%rdx)
// Frames: F32 -> F26 -> F27 -> F28
// 0x1419888cb  49ff4630                       incq	0x30(%r14)
// Frames: F33 -> F27 -> F28
// 0x1419888cf  49ff4640                       incq	0x40(%r14)
// 0x1419888d3  488b8598020000                 movq	0x298(%rbp), %rax
// 0x1419888da  4c8bada0020000                 movq	0x2a0(%rbp), %r13
// 0x1419888e1  488b8d50020000                 movq	0x250(%rbp), %rcx
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F4
// 0x1419888e8  4d85ed                         testq	%r13, %r13
// 0x1419888eb  7423                           je	0x141988910 <.text+0x1987910>
// 0x1419888ed  4c3bad68020000                 cmpq	0x268(%rbp), %r13
// 0x1419888f4  741a                           je	0x141988910 <.text+0x1987910>
// 0x1419888f6  498d5538                       leaq	0x38(%r13), %rdx
// 0x1419888fa  488995a0020000                 movq	%rdx, 0x2a0(%rbp)
// 0x141988901  48898598020000                 movq	%rax, 0x298(%rbp)
// 0x141988908  eb33                           jmp	0x14198893d <.text+0x198793d>
// 0x14198890a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F34 -> F39 -> F40 -> F36 -> F37 -> F38 -> F4
// 0x141988910  4885c0                         testq	%rax, %rax
// 0x141988913  0f8455040000                   je	0x141988d6e <.text+0x1987d6e>
// 0x141988919  483b8570020000                 cmpq	0x270(%rbp), %rax
// 0x141988920  0f8448040000                   je	0x141988d6e <.text+0x1987d6e>
// Frames: F41 -> F42 -> F43 -> F39 -> F40 -> F36 -> F37 -> F38 -> F4
// 0x141988926  488d5038                       leaq	0x38(%rax), %rdx
// 0x14198892a  48899598020000                 movq	%rdx, 0x298(%rbp)
// 0x141988931  31d2                           xorl	%edx, %edx
// 0x141988933  488995a0020000                 movq	%rdx, 0x2a0(%rbp)
// 0x14198893a  4989c5                         movq	%rax, %r13
// Frames: F44 -> F38 -> F4
// 0x14198893d  488b8548020000                 movq	0x248(%rbp), %rax
// 0x141988944  48ff00                         incq	(%rax)
// Frames: F45
// 0x141988947  498b7d08                       movq	0x8(%r13), %rdi
// 0x14198894b  4989cf                         movq	%rcx, %r15
// 0x14198894e  4889fa                         movq	%rdi, %rdx
// 0x141988951  4c8d05c8a6d600                 leaq	0xd6a6c8(%rip), %r8     # 0x1426f3020
// 0x141988958  e88317cefe                     callq	0x14066a0e0 <.text+0x6690e0>
// Frames: F46 -> F45
// 0x14198895d  4885c0                         testq	%rax, %rax
// 0x141988960  0f8493040000                   je	0x141988df9 <.text+0x1987df9>
// Frames: F47
// 0x141988966  4889c3                         movq	%rax, %rbx
// 0x141988969  4c89f9                         movq	%r15, %rcx
// 0x14198896c  4889fa                         movq	%rdi, %rdx
// 0x14198896f  4c8d05daa6d600                 leaq	0xd6a6da(%rip), %r8     # 0x1426f3050
// 0x141988976  e86517cefe                     callq	0x14066a0e0 <.text+0x6690e0>
// Frames: F48 -> F47
// 0x14198897b  4885c0                         testq	%rax, %rax
// 0x14198897e  0f8481040000                   je	0x141988e05 <.text+0x1987e05>
// Frames: F49 -> F50 -> F47
// 0x141988984  8b8848020000                   movl	0x248(%rax), %ecx
// 0x14198898a  83f902                         cmpl	$0x2, %ecx
// 0x14198898d  0f830d040000                   jae	0x141988da0 <.text+0x1987da0>
// 0x141988993  0fb6907f020000                 movzbl	0x27f(%rax), %edx
// Frames: F50 -> F47
// 0x14198899a  4c8b805c020000                 movq	0x25c(%rax), %r8
// 0x1419889a1  4c8d8dc8010000                 leaq	0x1c8(%rbp), %r9
// 0x1419889a8  4d894110                       movq	%r8, 0x10(%r9)
// 0x1419889ac  0f10804c020000                 movups	0x24c(%rax), %xmm0
// 0x1419889b3  410f1101                       movups	%xmm0, (%r9)
// Frames: F47
// 0x1419889b7  898dc4010000                   movl	%ecx, 0x1c4(%rbp)
// 0x1419889bd  8895c7020000                   movb	%dl, 0x2c7(%rbp)
// Frames: F51
// 0x1419889c3  4889d9                         movq	%rbx, %rcx
// 0x1419889c6  e8753d31ff                     callq	0x140c9c740 <.text+0xc9b740>
// 0x1419889cb  0f28f0                         movaps	%xmm0, %xmm6
// Frames: F52
// 0x1419889ce  410f104514                     movups	0x14(%r13), %xmm0
// 0x1419889d3  410f104d20                     movups	0x20(%r13), %xmm1
// 0x1419889d8  0f118dfc010000                 movups	%xmm1, 0x1fc(%rbp)
// 0x1419889df  0f2985f0010000                 movaps	%xmm0, 0x1f0(%rbp)
// Frames: F53
// 0x1419889e6  0f1085c4010000                 movups	0x1c4(%rbp), %xmm0
// 0x1419889ed  0f108dd0010000                 movups	0x1d0(%rbp), %xmm1
// 0x1419889f4  0f114d1c                       movups	%xmm1, 0x1c(%rbp)
// 0x1419889f8  0f294510                       movaps	%xmm0, 0x10(%rbp)
// Frames: F54
// 0x1419889fc  418b5d10                       movl	0x10(%r13), %ebx
// Frames: F55
// 0x141988a00  488d8dc7020000                 leaq	0x2c7(%rbp), %rcx
// 0x141988a07  e8f41fb9fe                     callq	0x14051aa00 <.text+0x519a00>
// Frames: F56
// 0x141988a0c  88442430                       movb	%al, 0x30(%rsp)
// 0x141988a10  895c2428                       movl	%ebx, 0x28(%rsp)
// 0x141988a14  f30f11742420                   movss	%xmm6, 0x20(%rsp)
// 0x141988a1a  488d5dc0                       leaq	-0x40(%rbp), %rbx
// 0x141988a1e  4889d9                         movq	%rbx, %rcx
// 0x141988a21  4889fa                         movq	%rdi, %rdx
// 0x141988a24  4c8d85f0010000                 leaq	0x1f0(%rbp), %r8
// 0x141988a2b  4c8d4d10                       leaq	0x10(%rbp), %r9
// 0x141988a2f  e86c19d3fe                     callq	0x1406ba3a0 <.text+0x6b93a0>
// Frames: F57
// 0x141988a34  4889d9                         movq	%rbx, %rcx
// 0x141988a37  e8e419d3fe                     callq	0x1406ba420 <.text+0x6b9420>
// 0x141988a3c  4189c7                         movl	%eax, %r15d
// Frames: F58
// 0x141988a3f  4889d9                         movq	%rbx, %rcx
// 0x141988a42  e8291ad3fe                     callq	0x1406ba470 <.text+0x6b9470>
// 0x141988a47  84c0                           testb	%al, %al
// 0x141988a49  745e                           je	0x141988aa9 <.text+0x1987aa9>
// Frames: F59
// 0x141988a4b  4489fb                         movl	%r15d, %ebx
// 0x141988a4e  4c8b6dc0                       movq	-0x40(%rbp), %r13
// 0x141988a52  8b85c0020000                   movl	0x2c0(%rbp), %eax
// 0x141988a58  488b8d78020000                 movq	0x278(%rbp), %rcx
// Frames: F60 -> F61 -> F27 -> F62
// 0x141988a5f  8901                           movl	%eax, (%rcx)
// 0x141988a61  488b8db0020000                 movq	0x2b0(%rbp), %rcx
// Frames: F63 -> F26 -> F27 -> F62
// 0x141988a68  488b5130                       movq	0x30(%rcx), %rdx
// Frames: F64 -> F27 -> F62
// 0x141988a6c  4c8b7940                       movq	0x40(%rcx), %r15
// Frames: F63 -> F26 -> F27 -> F62
// 0x141988a70  483b5120                       cmpq	0x20(%rcx), %rdx
// 0x141988a74  7517                           jne	0x141988a8d <.text+0x1987a8d>
// Frames: F65 -> F26 -> F27 -> F62
// 0x141988a76  488b8d90020000                 movq	0x290(%rbp), %rcx
// 0x141988a7d  e82e118ffe                     callq	0x140279bb0 <.text+0x278bb0>
// 0x141988a82  488b8db0020000                 movq	0x2b0(%rbp), %rcx
// Frames: F25 -> F26 -> F27 -> F62
// 0x141988a89  488b5130                       movq	0x30(%rcx), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F62
// 0x141988a8d  488b4128                       movq	0x28(%rcx), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F62
// 0x141988a91  48c1e204                       shlq	$0x4, %rdx
// Frames: F30 -> F31 -> F26 -> F27 -> F62
// 0x141988a95  4c893c10                       movq	%r15, (%rax,%rdx)
// 0x141988a99  4c896c1008                     movq	%r13, 0x8(%rax,%rdx)
// Frames: F32 -> F26 -> F27 -> F62
// 0x141988a9e  48ff4130                       incq	0x30(%rcx)
// Frames: F33 -> F27 -> F62
// 0x141988aa2  48ff4140                       incq	0x40(%rcx)
// 0x141988aa6  4189df                         movl	%ebx, %r15d
// Frames: F66
// 0x141988aa9  0f104500                       movups	(%rbp), %xmm0
// 0x141988aad  0f298530020000                 movaps	%xmm0, 0x230(%rbp)
// 0x141988ab4  0f1045c0                       movups	-0x40(%rbp), %xmm0
// 0x141988ab8  0f104dd0                       movups	-0x30(%rbp), %xmm1
// 0x141988abc  0f1055e0                       movups	-0x20(%rbp), %xmm2
// 0x141988ac0  0f105df0                       movups	-0x10(%rbp), %xmm3
// 0x141988ac4  0f299d20020000                 movaps	%xmm3, 0x220(%rbp)
// 0x141988acb  0f299510020000                 movaps	%xmm2, 0x210(%rbp)
// 0x141988ad2  0f298d00020000                 movaps	%xmm1, 0x200(%rbp)
// 0x141988ad9  0f2985f0010000                 movaps	%xmm0, 0x1f0(%rbp)
// Frames: F67
// 0x141988ae0  488d8d38010000                 leaq	0x138(%rbp), %rcx
// 0x141988ae7  0f28ce                         movaps	%xmm6, %xmm1
// 0x141988aea  e8b193ccfe                     callq	0x140651ea0 <.text+0x650ea0>
// Frames: F68
// 0x141988aef  488d8d78010000                 leaq	0x178(%rbp), %rcx
// 0x141988af6  e8d51cb9fe                     callq	0x14051a7d0 <.text+0x5197d0>
// Frames: F69
// 0x141988afb  e870ad53ff                     callq	0x140ec3870 <.text+0xec2870>
// Frames: F70
// 0x141988b00  0f28f8                         movaps	%xmm0, %xmm7
// 0x141988b03  440f28c1                       movaps	%xmm1, %xmm8
// 0x141988b07  e884a867fe                     callq	0x140003390 <.text+0x2390>
// 0x141988b0c  4989d5                         movq	%rdx, %r13
// 0x141988b0f  488985a8020000                 movq	%rax, 0x2a8(%rbp)
// 0x141988b16  488985e0010000                 movq	%rax, 0x1e0(%rbp)
// 0x141988b1d  488995e8010000                 movq	%rdx, 0x1e8(%rbp)
// Frames: F71 -> F72
// 0x141988b24  31c9                           xorl	%ecx, %ecx
// 0x141988b26  e825a7c0fe                     callq	0x140593250 <.text+0x592250>
// Frames: F73 -> F71 -> F72
// 0x141988b2b  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x141988b32  89c2                           movl	%eax, %edx
// 0x141988b34  e8777acdfe                     callq	0x1406605b0 <.text+0x65f5b0>
// Frames: F74 -> F72
// 0x141988b39  e8e2a167fe                     callq	0x140002d20 <.text+0x1d20>
// Frames: F75 -> F72
// 0x141988b3e  0f108550010000                 movups	0x150(%rbp), %xmm0
// 0x141988b45  488d8568010000                 leaq	0x168(%rbp), %rax
// 0x141988b4c  0f1100                         movups	%xmm0, (%rax)
// 0x141988b4f  c786a000000000000000           movl	$0x0, 0xa0(%rsi)
// 0x141988b59  48c7869800000000000000         movq	$0x0, 0x98(%rsi)
// Frames: F76
// 0x141988b64  0f288530020000                 movaps	0x230(%rbp), %xmm0
// 0x141988b6b  0f114640                       movups	%xmm0, 0x40(%rsi)
// 0x141988b6f  0f2885f0010000                 movaps	0x1f0(%rbp), %xmm0
// 0x141988b76  0f288d00020000                 movaps	0x200(%rbp), %xmm1
// 0x141988b7d  0f289510020000                 movaps	0x210(%rbp), %xmm2
// 0x141988b84  0f289d20020000                 movaps	0x220(%rbp), %xmm3
// 0x141988b8b  0f115e30                       movups	%xmm3, 0x30(%rsi)
// 0x141988b8f  0f115620                       movups	%xmm2, 0x20(%rsi)
// 0x141988b93  0f114e10                       movups	%xmm1, 0x10(%rsi)
// 0x141988b97  0f1106                         movups	%xmm0, (%rsi)
// 0x141988b9a  488b8548010000                 movq	0x148(%rbp), %rax
// 0x141988ba1  48894660                       movq	%rax, 0x60(%rsi)
// 0x141988ba5  0f108538010000                 movups	0x138(%rbp), %xmm0
// 0x141988bac  0f114650                       movups	%xmm0, 0x50(%rsi)
// 0x141988bb0  0f108578010000                 movups	0x178(%rbp), %xmm0
// 0x141988bb7  0f108d88010000                 movups	0x188(%rbp), %xmm1
// 0x141988bbe  0f109598010000                 movups	0x198(%rbp), %xmm2
// 0x141988bc5  0f109da8010000                 movups	0x1a8(%rbp), %xmm3
// 0x141988bcc  0f294510                       movaps	%xmm0, 0x10(%rbp)
// 0x141988bd0  0f294d20                       movaps	%xmm1, 0x20(%rbp)
// 0x141988bd4  0f295530                       movaps	%xmm2, 0x30(%rbp)
// 0x141988bd8  0f295d40                       movaps	%xmm3, 0x40(%rbp)
// 0x141988bdc  488b85b8010000                 movq	0x1b8(%rbp), %rax
// 0x141988be3  48894550                       movq	%rax, 0x50(%rbp)
// 0x141988be7  f30f11bdc0000000               movss	%xmm7, 0xc0(%rbp)
// 0x141988bef  f3440f1185c4000000             movss	%xmm8, 0xc4(%rbp)
// 0x141988bf8  488b85a8020000                 movq	0x2a8(%rbp), %rax
// 0x141988bff  488985c8000000                 movq	%rax, 0xc8(%rbp)
// 0x141988c06  4c89add0000000                 movq	%r13, 0xd0(%rbp)
// 0x141988c0d  440f298de0000000               movaps	%xmm9, 0xe0(%rbp)
// 0x141988c15  8b0535a6d600                   movl	0xd6a635(%rip), %eax    # 0x1426f3250
// 0x141988c1b  8986ac000000                   movl	%eax, 0xac(%rsi)
// 0x141988c21  488b0520a6d600                 movq	0xd6a620(%rip), %rax    # 0x1426f3248
// 0x141988c28  488986a4000000                 movq	%rax, 0xa4(%rsi)
// 0x141988c2f  0f108560010000                 movups	0x160(%rbp), %xmm0
// 0x141988c36  0f1186b0000000                 movups	%xmm0, 0xb0(%rsi)
// 0x141988c3d  488b8570010000                 movq	0x170(%rbp), %rax
// 0x141988c44  488986c0000000                 movq	%rax, 0xc0(%rsi)
// 0x141988c4b  c6852001000009                 movb	$0x9, 0x120(%rbp)
// 0x141988c52  488d8df0010000                 leaq	0x1f0(%rbp), %rcx
// 0x141988c59  488b9588030000                 movq	0x388(%rbp), %rdx
// 0x141988c60  4c8d4510                       leaq	0x10(%rbp), %r8
// 0x141988c64  e81739f4fe                     callq	0x1408cc580 <.text+0x8cb580>
// Frames: F77
// 0x141988c69  4c8bad10020000                 movq	0x210(%rbp), %r13
// Frames: F78
// 0x141988c70  4584ff                         testb	%r15b, %r15b
// 0x141988c73  741b                           je	0x141988c90 <.text+0x1987c90>
// 0x141988c75  31db                           xorl	%ebx, %ebx
// 0x141988c77  440fb6bdc6020000               movzbl	0x2c6(%rbp), %r15d
// 0x141988c7f  eb20                           jmp	0x141988ca1 <.text+0x1987ca1>
// 0x141988c81  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F79
// 0x141988c90  488d8dc7020000                 leaq	0x2c7(%rbp), %rcx
// 0x141988c97  e8641db9fe                     callq	0x14051aa00 <.text+0x519a00>
// 0x141988c9c  4189c7                         movl	%eax, %r15d
// 0x141988c9f  b306                           movb	$0x6, %bl
// Frames: F80
// 0x141988ca1  488d8dc4010000                 leaq	0x1c4(%rbp), %rcx
// 0x141988ca8  e8737cd2fe                     callq	0x1406b0920 <.text+0x6af920>
// 0x141988cad  f30f118578010000               movss	%xmm0, 0x178(%rbp)
// 0x141988cb5  f30f118d7c010000               movss	%xmm1, 0x17c(%rbp)
// 0x141988cbd  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x141988cc1  488d9578010000                 leaq	0x178(%rbp), %rdx
// 0x141988cc8  0f28d6                         movaps	%xmm6, %xmm2
// 0x141988ccb  e8b01ae5fe                     callq	0x1407da780 <.text+0x7d9780>
// Frames: F81
// 0x141988cd0  885d10                         movb	%bl, 0x10(%rbp)
// 0x141988cd3  44887d11                       movb	%r15b, 0x11(%rbp)
// 0x141988cd7  8b85b8020000                   movl	0x2b8(%rbp), %eax
// 0x141988cdd  488b8d58020000                 movq	0x258(%rbp), %rcx
// Frames: F60 -> F61 -> F27 -> F81
// 0x141988ce4  8901                           movl	%eax, (%rcx)
// Frames: F63 -> F26 -> F27 -> F81
// 0x141988ce6  498b542430                     movq	0x30(%r12), %rdx
// Frames: F64 -> F27 -> F81
// 0x141988ceb  498b5c2440                     movq	0x40(%r12), %rbx
// Frames: F63 -> F26 -> F27 -> F81
// 0x141988cf0  493b542420                     cmpq	0x20(%r12), %rdx
// 0x141988cf5  4488bdc6020000                 movb	%r15b, 0x2c6(%rbp)
// 0x141988cfc  7511                           jne	0x141988d0f <.text+0x1987d0f>
// Frames: F65 -> F26 -> F27 -> F81
// 0x141988cfe  488b8d80020000                 movq	0x280(%rbp), %rcx
// 0x141988d05  e806188ffe                     callq	0x14027a510 <.text+0x279510>
// Frames: F25 -> F26 -> F27 -> F81
// 0x141988d0a  498b542430                     movq	0x30(%r12), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F81
// 0x141988d0f  498b442428                     movq	0x28(%r12), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F81
// 0x141988d14  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F30 -> F31 -> F26 -> F27 -> F81
// 0x141988d18  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x141988d1c  0f104510                       movups	0x10(%rbp), %xmm0
// 0x141988d20  0f104d1c                       movups	0x1c(%rbp), %xmm1
// 0x141988d24  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// 0x141988d29  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// Frames: F32 -> F26 -> F27 -> F81
// 0x141988d2e  49ff442430                     incq	0x30(%r12)
// Frames: F33 -> F27 -> F81
// 0x141988d33  49ff442440                     incq	0x40(%r12)
// 0x141988d38  8b85bc020000                   movl	0x2bc(%rbp), %eax
// 0x141988d3e  488b8d60020000                 movq	0x260(%rbp), %rcx
// Frames: F60 -> F61 -> F27 -> F28
// 0x141988d45  8901                           movl	%eax, (%rcx)
// Frames: F63 -> F26 -> F27 -> F28
// 0x141988d47  498b5630                       movq	0x30(%r14), %rdx
// Frames: F64 -> F27 -> F28
// 0x141988d4b  498b5e40                       movq	0x40(%r14), %rbx
// Frames: F63 -> F26 -> F27 -> F28
// 0x141988d4f  493b5620                       cmpq	0x20(%r14), %rdx
// 0x141988d53  0f8557fbffff                   jne	0x1419888b0 <.text+0x19878b0>
// Frames: F65 -> F26 -> F27 -> F28
// 0x141988d59  488b8d88020000                 movq	0x288(%rbp), %rcx
// 0x141988d60  e8ab0f8ffe                     callq	0x140279d10 <.text+0x278d10>
// Frames: F25 -> F26 -> F27 -> F28
// 0x141988d65  498b5630                       movq	0x30(%r14), %rdx
// 0x141988d69  e942fbffff                     jmp	0x1419888b0 <.text+0x19878b0>
// Frames: F82
// 0x141988d6e  0f28b5d0020000                 movaps	0x2d0(%rbp), %xmm6
// 0x141988d75  0f28bde0020000                 movaps	0x2e0(%rbp), %xmm7
// 0x141988d7c  440f2885f0020000               movaps	0x2f0(%rbp), %xmm8
// 0x141988d84  440f288d00030000               movaps	0x300(%rbp), %xmm9
// 0x141988d8c  4881c498030000                 addq	$0x398, %rsp            # imm = 0x398
// 0x141988d93  5b                             popq	%rbx
// 0x141988d94  5f                             popq	%rdi
// 0x141988d95  5e                             popq	%rsi
// 0x141988d96  415c                           popq	%r12
// 0x141988d98  415d                           popq	%r13
// 0x141988d9a  415e                           popq	%r14
// 0x141988d9c  415f                           popq	%r15
// 0x141988d9e  5d                             popq	%rbp
// 0x141988d9f  c3                             retq
// Frames: F83 -> F50 -> F47
// 0x141988da0  480548020000                   addq	$0x248, %rax            # imm = 0x248
// 0x141988da6  488985f0010000                 movq	%rax, 0x1f0(%rbp)
// 0x141988dad  488d05fcf8ffff                 leaq	-0x704(%rip), %rax      # 0x1419886b0 <.text+0x19876b0>
// 0x141988db4  488985f8010000                 movq	%rax, 0x1f8(%rbp)
// Frames: F84 -> F83 -> F50 -> F47
// 0x141988dbb  488d050ea3d600                 leaq	0xd6a30e(%rip), %rax    # 0x1426f30d0
// 0x141988dc2  48894510                       movq	%rax, 0x10(%rbp)
// 0x141988dc6  48c7451801000000               movq	$0x1, 0x18(%rbp)
// 0x141988dce  48c7453000000000               movq	$0x0, 0x30(%rbp)
// 0x141988dd6  488d85f0010000                 leaq	0x1f0(%rbp), %rax
// 0x141988ddd  48894520                       movq	%rax, 0x20(%rbp)
// 0x141988de1  48c7452801000000               movq	$0x1, 0x28(%rbp)
// Frames: F83 -> F50 -> F47
// 0x141988de9  488d15f0a2d600                 leaq	0xd6a2f0(%rip), %rdx    # 0x1426f30e0
// 0x141988df0  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x141988df4  e8875c7200                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F85 -> F45
// 0x141988df9  488d0d38a2d600                 leaq	0xd6a238(%rip), %rcx    # 0x1426f3038
// 0x141988e00  e83b5c7200                     callq	0x1420aea40 <.text+0x20ada40>
// Frames: F85 -> F47
// 0x141988e05  488d0d5ca2d600                 leaq	0xd6a25c(%rip), %rcx    # 0x1426f3068
// 0x141988e0c  e82f5c7200                     callq	0x1420aea40 <.text+0x20ada40>
// 0x141988e11  cc                             int3
// 0x141988e12  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x141988e20  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141988e25  55                             pushq	%rbp
// 0x141988e26  4157                           pushq	%r15
// 0x141988e28  4156                           pushq	%r14
// 0x141988e2a  4155                           pushq	%r13
// 0x141988e2c  4154                           pushq	%r12
// 0x141988e2e  56                             pushq	%rsi
// 0x141988e2f  57                             pushq	%rdi
// 0x141988e30  53                             pushq	%rbx
// 0x141988e31  4883ec78                       subq	$0x78, %rsp
// 0x141988e35  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141988e3c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141988e42  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141988e48  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141988e4d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F86 -> F86 -> F87
// 0x141988e52  4883bda802000000               cmpq	$0x0, 0x2a8(%rbp)
// 0x141988e5a  740c                           je	0x141988e68 <.text+0x1987e68>
// 0x141988e5c  488d8de0010000                 leaq	0x1e0(%rbp), %rcx
// 0x141988e63  e8f841f4fe                     callq	0x1408cd060 <.text+0x8cc060>
// Frames: F87
// 0x141988e68  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141988e6d  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141988e72  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141988e78  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x141988e7e  4883c478                       addq	$0x78, %rsp
// 0x141988e82  5b                             popq	%rbx
// 0x141988e83  5f                             popq	%rdi
// 0x141988e84  5e                             popq	%rsi
// 0x141988e85  415c                           popq	%r12
// 0x141988e87  415d                           popq	%r13
// 0x141988e89  415e                           popq	%r14
// 0x141988e8b  415f                           popq	%r15
// 0x141988e8d  5d                             popq	%rbp
// 0x141988e8e  c3                             retq
// 0x141988e8f  90                             nop
// 0x141988e90  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141988e95  55                             pushq	%rbp
// 0x141988e96  4157                           pushq	%r15
// 0x141988e98  4156                           pushq	%r14
// 0x141988e9a  4155                           pushq	%r13
// 0x141988e9c  4154                           pushq	%r12
// 0x141988e9e  56                             pushq	%rsi
// 0x141988e9f  57                             pushq	%rdi
// 0x141988ea0  53                             pushq	%rbx
// 0x141988ea1  4883ec78                       subq	$0x78, %rsp
// 0x141988ea5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141988eac  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141988eb2  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141988eb8  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141988ebd  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// 0x141988ec2  488d8d78010000                 leaq	0x178(%rbp), %rcx
// 0x141988ec9  e8021c95fe                     callq	0x1402daad0 <.text+0x2d9ad0>
// 0x141988ece  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141988ed3  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141988ed8  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141988ede  440f284c2430                   movaps	0x30(%rsp), %xmm9
// Frames: F1
// 0x141988ee4  4883c478                       addq	$0x78, %rsp
// 0x141988ee8  5b                             popq	%rbx
// 0x141988ee9  5f                             popq	%rdi
// 0x141988eea  5e                             popq	%rsi
// 0x141988eeb  415c                           popq	%r12
// 0x141988eed  415d                           popq	%r13
// 0x141988eef  415e                           popq	%r14
// 0x141988ef1  415f                           popq	%r15
// 0x141988ef3  5d                             popq	%rbp
// 0x141988ef4  c3                             retq
//
// RANGE rva_1989be0_len_aab: RVA 0x1989be0, 2731 PDB bytes; 611 instructions
// Evidence: _assembly/roof-v1/A/pack-002/rva_1989be0_len_aab.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3198:852: system_roof::visual::switch_roof_type::switch_roof_type
// Frames: F88
// 0x141989be0  55                             pushq	%rbp
// 0x141989be1  4157                           pushq	%r15
// 0x141989be3  4156                           pushq	%r14
// 0x141989be5  4155                           pushq	%r13
// 0x141989be7  4154                           pushq	%r12
// 0x141989be9  56                             pushq	%rsi
// 0x141989bea  57                             pushq	%rdi
// 0x141989beb  53                             pushq	%rbx
// 0x141989bec  4881ec88010000                 subq	$0x188, %rsp            # imm = 0x188
// 0x141989bf3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x141989bfb  0f29bdf0000000                 movaps	%xmm7, 0xf0(%rbp)
// 0x141989c02  0f29b5e0000000                 movaps	%xmm6, 0xe0(%rbp)
// 0x141989c09  48c785d8000000feffffff         movq	$-0x2, 0xd8(%rbp)
// 0x141989c14  4c8b9580010000                 movq	0x180(%rbp), %r10
// Frames: F2 -> F3 -> F89
// 0x141989c1b  4c8b31                         movq	(%rcx), %r14
// Frames: F5 -> F3 -> F89
// 0x141989c1e  4c8b5908                       movq	0x8(%rcx), %r11
// Frames: F6 -> F7 -> F8 -> F3 -> F89
// 0x141989c22  498b3e                         movq	(%r14), %rdi
// 0x141989c25  31f6                           xorl	%esi, %esi
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F89
// 0x141989c27  4889f9                         movq	%rdi, %rcx
// 0x141989c2a  492b4b18                       subq	0x18(%r11), %rcx
// 0x141989c2e  480f42ce                       cmovbq	%rsi, %rcx
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F89
// 0x141989c32  492b7b38                       subq	0x38(%r11), %rdi
// 0x141989c36  480f42fe                       cmovbq	%rsi, %rdi
// Frames: F12 -> F13 -> F14 -> F15 -> F7 -> F8 -> F3 -> F89
// 0x141989c3a  4889c8                         movq	%rcx, %rax
// 0x141989c3d  48c1e005                       shlq	$0x5, %rax
// 0x141989c41  49034308                       addq	0x8(%r11), %rax
// Frames: F15 -> F7 -> F8 -> F3 -> F89
// 0x141989c45  4d8b7b10                       movq	0x10(%r11), %r15
// Frames: F16 -> F15 -> F7 -> F8 -> F3 -> F89
// 0x141989c49  4929cf                         subq	%rcx, %r15
// 0x141989c4c  4c0f42fe                       cmovbq	%rsi, %r15
// Frames: F17 -> F7 -> F8 -> F3 -> F89
// 0x141989c50  4d8b6330                       movq	0x30(%r11), %r12
// Frames: F16 -> F15 -> F7 -> F8 -> F3 -> F89
// 0x141989c54  488d1d55e69400                 leaq	0x94e655(%rip), %rbx    # 0x1422d82b0
// 0x141989c5b  480f42c3                       cmovbq	%rbx, %rax
// Frames: F12 -> F13 -> F14 -> F17 -> F7 -> F8 -> F3 -> F89
// 0x141989c5f  4889f9                         movq	%rdi, %rcx
// 0x141989c62  48c1e105                       shlq	$0x5, %rcx
// 0x141989c66  49034b28                       addq	0x28(%r11), %rcx
// 0x141989c6a  4c39e7                         cmpq	%r12, %rdi
// Frames: F16 -> F17 -> F7 -> F8 -> F3 -> F89
// 0x141989c6d  480f47cb                       cmovaq	%rbx, %rcx
// 0x141989c71  488b9d78010000                 movq	0x178(%rbp), %rbx
// 0x141989c78  4929fc                         subq	%rdi, %r12
// 0x141989c7b  4c0f42e6                       cmovbq	%rsi, %r12
// Frames: F18 -> F7 -> F8 -> F3 -> F89
// 0x141989c7f  4d8b5b40                       movq	0x40(%r11), %r11
// 0x141989c83  4d29e3                         subq	%r12, %r11
// 0x141989c86  4d29fb                         subq	%r15, %r11
// 0x141989c89  4c8975d0                       movq	%r14, -0x30(%rbp)
// 0x141989c8d  4d891e                         movq	%r11, (%r14)
// Frames: F19 -> F20 -> F21 -> F22 -> F7 -> F8 -> F3 -> F89
// 0x141989c90  49c1e705                       shlq	$0x5, %r15
// 0x141989c94  4901c7                         addq	%rax, %r15
// 0x141989c97  4c897de0                       movq	%r15, -0x20(%rbp)
// 0x141989c9b  49c1e405                       shlq	$0x5, %r12
// 0x141989c9f  4901cc                         addq	%rcx, %r12
// 0x141989ca2  4c8965f0                       movq	%r12, -0x10(%rbp)
// 0x141989ca6  4c8b1b                         movq	(%rbx), %r11
// 0x141989ca9  4c8b6308                       movq	0x8(%rbx), %r12
// 0x141989cad  488b32                         movq	(%rdx), %rsi
// 0x141989cb0  48897560                       movq	%rsi, 0x60(%rbp)
// 0x141989cb4  488b7a08                       movq	0x8(%rdx), %rdi
// 0x141989cb8  488b5210                       movq	0x10(%rdx), %rdx
// 0x141989cbc  48895578                       movq	%rdx, 0x78(%rbp)
// 0x141989cc0  498b10                         movq	(%r8), %rdx
// 0x141989cc3  48895528                       movq	%rdx, 0x28(%rbp)
// 0x141989cc7  418b521c                       movl	0x1c(%r10), %edx
// 0x141989ccb  8995d0000000                   movl	%edx, 0xd0(%rbp)
// 0x141989cd1  4d8b3a                         movq	(%r10), %r15
// 0x141989cd4  498b5210                       movq	0x10(%r10), %rdx
// 0x141989cd8  48895508                       movq	%rdx, 0x8(%rbp)
// 0x141989cdc  488b9570010000                 movq	0x170(%rbp), %rdx
// 0x141989ce3  448b421c                       movl	0x1c(%rdx), %r8d
// 0x141989ce7  4489455c                       movl	%r8d, 0x5c(%rbp)
// 0x141989ceb  4c8b02                         movq	(%rdx), %r8
// 0x141989cee  488b5210                       movq	0x10(%rdx), %rdx
// 0x141989cf2  488955e8                       movq	%rdx, -0x18(%rbp)
// 0x141989cf6  418b511c                       movl	0x1c(%r9), %edx
// 0x141989cfa  895558                         movl	%edx, 0x58(%rbp)
// 0x141989cfd  498b31                         movq	(%r9), %rsi
// 0x141989d00  498b5110                       movq	0x10(%r9), %rdx
// 0x141989d04  488955d8                       movq	%rdx, -0x28(%rbp)
// 0x141989d08  0f2835e1108300                 movaps	0x8310e1(%rip), %xmm6   # 0x1421badf0
// 0x141989d0f  f30f103da9ea7b00               movss	0x7beaa9(%rip), %xmm7   # 0x1421487c0
// 0x141989d17  498d5720                       leaq	0x20(%r15), %rdx
// 0x141989d1b  48895538                       movq	%rdx, 0x38(%rbp)
// 0x141989d1f  4c894530                       movq	%r8, 0x30(%rbp)
// 0x141989d23  498d5020                       leaq	0x20(%r8), %rdx
// 0x141989d27  48895500                       movq	%rdx, (%rbp)
// 0x141989d2b  488d5620                       leaq	0x20(%rsi), %rdx
// 0x141989d2f  488955f8                       movq	%rdx, -0x8(%rbp)
// 0x141989d33  4c895d10                       movq	%r11, 0x10(%rbp)
// 0x141989d37  4c896518                       movq	%r12, 0x18(%rbp)
// 0x141989d3b  eb2b                           jmp	0x141989d68 <.text+0x1988d68>
// 0x141989d3d  0f1f00                         nopl	(%rax)
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F90
// 0x141989d40  4c8b5d10                       movq	0x10(%rbp), %r11
// 0x141989d44  4c8b6518                       movq	0x18(%rbp), %r12
// 0x141989d48  488b4628                       movq	0x28(%rsi), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F90
// 0x141989d4c  48c1e204                       shlq	$0x4, %rdx
// Frames: F30 -> F31 -> F26 -> F27 -> F90
// 0x141989d50  4c893410                       movq	%r14, (%rax,%rdx)
// 0x141989d54  48895c1008                     movq	%rbx, 0x8(%rax,%rdx)
// Frames: F32 -> F26 -> F27 -> F90
// 0x141989d59  48ff4630                       incq	0x30(%rsi)
// Frames: F33 -> F27 -> F90
// 0x141989d5d  48ff4640                       incq	0x40(%rsi)
// 0x141989d61  4c89e9                         movq	%r13, %rcx
// 0x141989d64  488b45c8                       movq	-0x38(%rbp), %rax
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F89
// 0x141989d68  4885c0                         testq	%rax, %rax
// 0x141989d6b  7413                           je	0x141989d80 <.text+0x1988d80>
// 0x141989d6d  483b45e0                       cmpq	-0x20(%rbp), %rax
// 0x141989d71  740d                           je	0x141989d80 <.text+0x1988d80>
// 0x141989d73  4c8d7020                       leaq	0x20(%rax), %r14
// 0x141989d77  4989ca                         movq	%rcx, %r10
// 0x141989d7a  eb21                           jmp	0x141989d9d <.text+0x1988d9d>
// 0x141989d7c  0f1f4000                       nopl	(%rax)
// Frames: F34 -> F39 -> F40 -> F36 -> F37 -> F38 -> F89
// 0x141989d80  4885c9                         testq	%rcx, %rcx
// 0x141989d83  0f84d3070000                   je	0x14198a55c <.text+0x198955c>
// 0x141989d89  483b4df0                       cmpq	-0x10(%rbp), %rcx
// 0x141989d8d  0f84c9070000                   je	0x14198a55c <.text+0x198955c>
// Frames: F41 -> F42 -> F43 -> F39 -> F40 -> F36 -> F37 -> F38 -> F89
// 0x141989d93  4c8d5120                       leaq	0x20(%rcx), %r10
// 0x141989d97  4531f6                         xorl	%r14d, %r14d
// 0x141989d9a  4889c8                         movq	%rcx, %rax
// Frames: F44 -> F38 -> F89
// 0x141989d9d  488b4dd0                       movq	-0x30(%rbp), %rcx
// 0x141989da1  48ff01                         incq	(%rcx)
// Frames: F91
// 0x141989da4  488b4808                       movq	0x8(%rax), %rcx
// 0x141989da8  48894d70                       movq	%rcx, 0x70(%rbp)
// Frames: F92
// 0x141989dac  0fb65818                       movzbl	0x18(%rax), %ebx
// 0x141989db0  889dd7000000                   movb	%bl, 0xd7(%rbp)
// Frames: F93 -> F94 -> F95 -> F96
// 0x141989db6  89ca                           movl	%ecx, %edx
// 0x141989db8  b801000000                     movl	$0x1, %eax
// Frames: F97 -> F14 -> F93 -> F94 -> F95 -> F96
// 0x141989dbd  493b542410                     cmpq	0x10(%r12), %rdx
// Frames: F93 -> F94 -> F95 -> F96
// 0x141989dc2  0f83b8070000                   jae	0x14198a580 <.text+0x1989580>
// Frames: F98 -> F99 -> F100 -> F94 -> F95 -> F96
// 0x141989dc8  488d1492                       leaq	(%rdx,%rdx,4), %rdx
// 0x141989dcc  48c1e202                       shlq	$0x2, %rdx
// 0x141989dd0  4903542408                     addq	0x8(%r12), %rdx
// 0x141989dd5  4989c8                         movq	%rcx, %r8
// 0x141989dd8  49c1e820                       shrq	$0x20, %r8
// 0x141989ddc  443902                         cmpl	%r8d, (%rdx)
// Frames: F100 -> F94 -> F95 -> F96
// 0x141989ddf  0f859b070000                   jne	0x14198a580 <.text+0x1989580>
// Frames: F101 -> F102 -> F94 -> F95 -> F96
// 0x141989de5  448b4204                       movl	0x4(%rdx), %r8d
// 0x141989de9  41b9ffffffff                   movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
// 0x141989def  4d39c8                         cmpq	%r9, %r8
// Frames: F102 -> F94 -> F95 -> F96
// 0x141989df2  0f8488070000                   je	0x14198a580 <.text+0x1989580>
// Frames: F103 -> F104 -> F95 -> F96
// 0x141989df8  4d394338                       cmpq	%r8, 0x38(%r11)
// Frames: F105 -> F106 -> F104 -> F95 -> F96
// 0x141989dfc  0f867c070000                   jbe	0x14198a57e <.text+0x198957e>
// Frames: F104 -> F95 -> F96
// 0x141989e02  498b4328                       movq	0x28(%r11), %rax
// Frames: F19 -> F107 -> F108 -> F109 -> F105 -> F106 -> F104 -> F95 -> F96
// 0x141989e06  4589c1                         movl	%r8d, %r9d
// 0x141989e09  41c1e906                       shrl	$0x6, %r9d
// Frames: F110 -> F108 -> F109 -> F105 -> F106 -> F104 -> F95 -> F96
// 0x141989e0d  4a8b04c8                       movq	(%rax,%r9,8), %rax
// Frames: F108 -> F109 -> F105 -> F106 -> F104 -> F95 -> F96
// 0x141989e11  4c0fa3c0                       btq	%r8, %rax
// Frames: F104 -> F95 -> F96
// 0x141989e15  0f8363070000                   jae	0x14198a57e <.text+0x198957e>
// Frames: F111 -> F94 -> F95 -> F96
// 0x141989e1b  4c8975c8                       movq	%r14, -0x38(%rbp)
// 0x141989e1f  4c8955c0                       movq	%r10, -0x40(%rbp)
// 0x141989e23  488b4208                       movq	0x8(%rdx), %rax
// 0x141989e27  8b4a10                         movl	0x10(%rdx), %ecx
// 0x141989e2a  48c1e820                       shrq	$0x20, %rax
// Frames: F112 -> F113 -> F95 -> F96
// 0x141989e2e  498b942428010000               movq	0x128(%r12), %rdx
// Frames: F114 -> F95 -> F96
// 0x141989e36  4d8b83d8000000                 movq	0xd8(%r11), %r8
// Frames: F115 -> F116 -> F117 -> F114 -> F95 -> F96
// 0x141989e3d  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F118 -> F115 -> F116 -> F117 -> F114 -> F95 -> F96
// 0x141989e41  4c8b4cc218                     movq	0x18(%rdx,%rax,8), %r9
// 0x141989e46  488b44c238                     movq	0x38(%rdx,%rax,8), %rax
// Frames: F119 -> F120 -> F50 -> F121 -> F118 -> F115 -> F116 -> F117 -> F114 -> F95 -> F96
// 0x141989e4b  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F122 -> F123 -> F50 -> F118 -> F115 -> F116 -> F117 -> F114 -> F95 -> F96
// 0x141989e4f  48f7d0                         notq	%rax
// Frames: F124 -> F125 -> F126 -> F117 -> F114 -> F95 -> F96
// 0x141989e52  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141989e56  48c1e005                       shlq	$0x5, %rax
// Frames: F19 -> F127 -> F128 -> F129 -> F95 -> F96
// 0x141989e5a  4c8d2489                       leaq	(%rcx,%rcx,4), %r12
// 0x141989e5e  49c1e404                       shlq	$0x4, %r12
// 0x141989e62  4d03640110                     addq	0x10(%r9,%rax), %r12
// Frames: F130
// 0x141989e67  4c89e1                         movq	%r12, %rcx
// 0x141989e6a  e8b105d3fe                     callq	0x1406ba420 <.text+0x6b9420>
// 0x141989e6f  884540                         movb	%al, 0x40(%rbp)
// Frames: F131 -> F130
// 0x141989e72  38c3                           cmpb	%al, %bl
// Frames: F130
// 0x141989e74  0f853f070000                   jne	0x14198a5b9 <.text+0x19895b9>
// Frames: F132 -> F133 -> F134
// 0x141989e7a  4c8b6d70                       movq	0x70(%rbp), %r13
// 0x141989e7e  488b4d60                       movq	0x60(%rbp), %rcx
// 0x141989e82  4c89ea                         movq	%r13, %rdx
// 0x141989e85  e8e6a1e9fe                     callq	0x140824070 <.text+0x823070>
// Frames: F135
// 0x141989e8a  84db                           testb	%bl, %bl
// 0x141989e8c  7452                           je	0x141989ee0 <.text+0x1988ee0>
// Frames: F136 -> F132 -> F133 -> F137
// 0x141989e8e  84c0                           testb	%al, %al
// 0x141989e90  0f8462070000                   je	0x14198a5f8 <.text+0x19895f8>
// Frames: F138 -> F139 -> F140 -> F141 -> F137
// 0x141989e96  4885ff                         testq	%rdi, %rdi
// 0x141989e99  0f8421010000                   je	0x141989fc0 <.text+0x1988fc0>
// Frames: F142 -> F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989e9f  488b07                         movq	(%rdi), %rax
// Frames: F148 -> F149 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989ea2  488b5f10                       movq	0x10(%rdi), %rbx
// Frames: F150 -> F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989ea6  4829d8                         subq	%rbx, %rax
// Frames: F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989ea9  4883f80f                       cmpq	$0xf, %rax
// Frames: F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989ead  0f860f060000                   jbe	0x14198a4c2 <.text+0x19894c2>
// Frames: F23 -> F24 -> F151 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989eb3  488b4708                       movq	0x8(%rdi), %rax
// Frames: F152 -> F153 -> F154 -> F155 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989eb7  488d0da21ff4fe                 leaq	-0x10be05e(%rip), %rcx  # 0x1408cbe60 <.text+0x8cae60>
// 0x141989ebe  48890c18                       movq	%rcx, (%rax,%rbx)
// 0x141989ec2  4c896c1808                     movq	%r13, 0x8(%rax,%rbx)
// Frames: F156 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989ec7  4883c310                       addq	$0x10, %rbx
// Frames: F157 -> F156 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x141989ecb  48895f10                       movq	%rbx, 0x10(%rdi)
// 0x141989ecf  e933010000                     jmp	0x14198a007 <.text+0x1989007>
// 0x141989ed4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F136 -> F132 -> F133 -> F158
// 0x141989ee0  84c0                           testb	%al, %al
// 0x141989ee2  0f842e070000                   je	0x14198a616 <.text+0x1989616>
// Frames: F71 -> F158
// 0x141989ee8  31c9                           xorl	%ecx, %ecx
// 0x141989eea  e86193c0fe                     callq	0x140593250 <.text+0x592250>
// Frames: F73 -> F71 -> F158
// 0x141989eef  488d8db0000000                 leaq	0xb0(%rbp), %rcx
// 0x141989ef6  89c2                           movl	%eax, %edx
// 0x141989ef8  e8b366cdfe                     callq	0x1406605b0 <.text+0x65f5b0>
// Frames: F74 -> F158
// 0x141989efd  e81e8e67fe                     callq	0x140002d20 <.text+0x1d20>
// Frames: F75 -> F158
// 0x141989f02  c685c000000009                 movb	$0x9, 0xc0(%rbp)
// 0x141989f09  0f29b580000000                 movaps	%xmm6, 0x80(%rbp)
// 0x141989f10  488d8590000000                 leaq	0x90(%rbp), %rax
// 0x141989f17  c7400800000000                 movl	$0x0, 0x8(%rax)
// 0x141989f1e  48c70000000000                 movq	$0x0, (%rax)
// 0x141989f25  48b90000803f0000803f           movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
// 0x141989f2f  4889480c                       movq	%rcx, 0xc(%rax)
// 0x141989f33  c740140000803f                 movl	$0x3f800000, 0x14(%rax) # imm = 0x3F800000
// 0x141989f3a  4885ff                         testq	%rdi, %rdi
// Frames: F138 -> F139 -> F140 -> F159 -> F158
// 0x141989f3d  0f844f030000                   je	0x14198a292 <.text+0x1989292>
// Frames: F142 -> F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989f43  488b07                         movq	(%rdi), %rax
// Frames: F148 -> F149 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989f46  488b5f10                       movq	0x10(%rdi), %rbx
// Frames: F150 -> F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989f4a  4829d8                         subq	%rbx, %rax
// Frames: F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989f4d  4883f867                       cmpq	$0x67, %rax
// Frames: F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989f51  0f86ad050000                   jbe	0x14198a504 <.text+0x1989504>
// Frames: F23 -> F24 -> F151 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989f57  488b4708                       movq	0x8(%rdi), %rax
// Frames: F152 -> F153 -> F154 -> F155 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989f5b  488d0dae1df4fe                 leaq	-0x10be252(%rip), %rcx  # 0x1408cbd10 <.text+0x8cad10>
// 0x141989f62  48890c18                       movq	%rcx, (%rax,%rbx)
// 0x141989f66  0f288580000000                 movaps	0x80(%rbp), %xmm0
// 0x141989f6d  0f288d90000000                 movaps	0x90(%rbp), %xmm1
// 0x141989f74  0f2895a0000000                 movaps	0xa0(%rbp), %xmm2
// 0x141989f7b  0f289db0000000                 movaps	0xb0(%rbp), %xmm3
// 0x141989f82  0f11441808                     movups	%xmm0, 0x8(%rax,%rbx)
// 0x141989f87  0f114c1818                     movups	%xmm1, 0x18(%rax,%rbx)
// 0x141989f8c  0f11541828                     movups	%xmm2, 0x28(%rax,%rbx)
// 0x141989f91  0f115c1838                     movups	%xmm3, 0x38(%rax,%rbx)
// 0x141989f96  0f2885c0000000                 movaps	0xc0(%rbp), %xmm0
// 0x141989f9d  0f11441848                     movups	%xmm0, 0x48(%rax,%rbx)
// 0x141989fa2  4c896c1858                     movq	%r13, 0x58(%rax,%rbx)
// Frames: F156 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989fa7  4883c368                       addq	$0x68, %rbx
// Frames: F157 -> F156 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x141989fab  48895f10                       movq	%rbx, 0x10(%rdi)
// 0x141989faf  e95b030000                     jmp	0x14198a30f <.text+0x198930f>
// 0x141989fb4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989fc0  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x141989fc7  488b5578                       movq	0x78(%rbp), %rdx
// 0x141989fcb  e870a5e9fe                     callq	0x140824540 <.text+0x823540>
// 0x141989fd0  488b9d80000000                 movq	0x80(%rbp), %rbx
// Frames: F142 -> F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989fd7  488b03                         movq	(%rbx), %rax
// Frames: F148 -> F149 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989fda  4c8b7310                       movq	0x10(%rbx), %r14
// Frames: F150 -> F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989fde  4c29f0                         subq	%r14, %rax
// Frames: F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989fe1  4883f80f                       cmpq	$0xf, %rax
// Frames: F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989fe5  0f862f050000                   jbe	0x14198a51a <.text+0x198951a>
// Frames: F23 -> F24 -> F151 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989feb  488b4308                       movq	0x8(%rbx), %rax
// Frames: F152 -> F153 -> F154 -> F155 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989fef  488d0d6a1ef4fe                 leaq	-0x10be196(%rip), %rcx  # 0x1408cbe60 <.text+0x8cae60>
// 0x141989ff6  4a890c30                       movq	%rcx, (%rax,%r14)
// 0x141989ffa  4e896c3008                     movq	%r13, 0x8(%rax,%r14)
// Frames: F156 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x141989fff  4983c610                       addq	$0x10, %r14
// Frames: F157 -> F156 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x14198a003  4c897310                       movq	%r14, 0x10(%rbx)
// Frames: F162
// 0x14198a007  488d4570                       leaq	0x70(%rbp), %rax
// 0x14198a00b  488945b0                       movq	%rax, -0x50(%rbp)
// 0x14198a00f  488d05bafaffff                 leaq	-0x546(%rip), %rax      # 0x141989ad0 <.text+0x1988ad0>
// 0x14198a016  488945b8                       movq	%rax, -0x48(%rbp)
// Frames: F163 -> F164 -> F165 -> F162
// 0x14198a01a  488d057f93d600                 leaq	0xd6937f(%rip), %rax    # 0x1426f33a0
// 0x14198a021  48898580000000                 movq	%rax, 0x80(%rbp)
// 0x14198a028  48c7858800000001000000         movq	$0x1, 0x88(%rbp)
// 0x14198a033  488d45b0                       leaq	-0x50(%rbp), %rax
// 0x14198a037  48898590000000                 movq	%rax, 0x90(%rbp)
// 0x14198a03e  48c7859800000001000000         movq	$0x1, 0x98(%rbp)
// 0x14198a049  48c785a000000000000000         movq	$0x0, 0xa0(%rbp)
// 0x14198a054  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198a058  488d9580000000                 leaq	0x80(%rbp), %rdx
// 0x14198a05f  e8bcf7edfe                     callq	0x140869820 <.text+0x868820>
// Frames: F162
// 0x14198a064  488b4540                       movq	0x40(%rbp), %rax
// 0x14198a068  48894568                       movq	%rax, 0x68(%rbp)
// 0x14198a06c  488b5548                       movq	0x48(%rbp), %rdx
// 0x14198a070  4c8b4550                       movq	0x50(%rbp), %r8
// 0x14198a074  488b4d28                       movq	0x28(%rbp), %rcx
// 0x14198a078  48895520                       movq	%rdx, 0x20(%rbp)
// 0x14198a07c  e89fc6c0fe                     callq	0x140596720 <.text+0x595720>
// Frames: F166
// 0x14198a081  4883f801                       cmpq	$0x1, %rax
// 0x14198a085  751b                           jne	0x14198a0a2 <.text+0x19890a2>
// Frames: F167
// 0x14198a087  4889d3                         movq	%rdx, %rbx
// 0x14198a08a  488b4d28                       movq	0x28(%rbp), %rcx
// 0x14198a08e  e8cdc9c0fe                     callq	0x140596a60 <.text+0x595a60>
// 0x14198a093  488b08                         movq	(%rax), %rcx
// Frames: F168 -> F167
// 0x14198a096  4883c110                       addq	$0x10, %rcx
// Frames: F167
// 0x14198a09a  4889da                         movq	%rbx, %rdx
// 0x14198a09d  e8fee835ff                     callq	0x140ce89a0 <.text+0xce79a0>
// Frames: F169 -> F170 -> F86 -> F86 -> F86 -> F171
// 0x14198a0a2  488b5568                       movq	0x68(%rbp), %rdx
// 0x14198a0a6  4885d2                         testq	%rdx, %rdx
// 0x14198a0a9  740f                           je	0x14198a0ba <.text+0x19890ba>
// Frames: F172 -> F173 -> F174 -> F86 -> F86 -> F86 -> F171
// 0x14198a0ab  41b801000000                   movl	$0x1, %r8d
// 0x14198a0b1  488b4d20                       movq	0x20(%rbp), %rcx
// 0x14198a0b5  e8d606a5fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F175
// 0x14198a0ba  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x14198a0c1  4c89e2                         movq	%r12, %rdx
// 0x14198a0c4  e82703d3fe                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F176
// 0x14198a0c9  c6858000000000                 movb	$0x0, 0x80(%rbp)
// 0x14198a0d0  8b85d0000000                   movl	0xd0(%rbp), %eax
// 0x14198a0d6  488b4d08                       movq	0x8(%rbp), %rcx
// Frames: F60 -> F61 -> F27 -> F176
// 0x14198a0da  8901                           movl	%eax, (%rcx)
// Frames: F63 -> F26 -> F27 -> F176
// 0x14198a0dc  498b5730                       movq	0x30(%r15), %rdx
// Frames: F64 -> F27 -> F176
// 0x14198a0e0  498b5f40                       movq	0x40(%r15), %rbx
// Frames: F63 -> F26 -> F27 -> F176
// 0x14198a0e4  493b5720                       cmpq	0x20(%r15), %rdx
// 0x14198a0e8  750d                           jne	0x14198a0f7 <.text+0x19890f7>
// Frames: F65 -> F26 -> F27 -> F176
// 0x14198a0ea  488b4d38                       movq	0x38(%rbp), %rcx
// 0x14198a0ee  e81d048ffe                     callq	0x14027a510 <.text+0x279510>
// Frames: F25 -> F26 -> F27 -> F176
// 0x14198a0f3  498b5730                       movq	0x30(%r15), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F176
// 0x14198a0f7  498b4728                       movq	0x28(%r15), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F176
// 0x14198a0fb  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F30 -> F31 -> F26 -> F27 -> F176
// 0x14198a0ff  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x14198a103  0f108580000000                 movups	0x80(%rbp), %xmm0
// 0x14198a10a  0f108d8c000000                 movups	0x8c(%rbp), %xmm1
// 0x14198a111  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// 0x14198a116  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// Frames: F32 -> F26 -> F27 -> F176
// 0x14198a11b  49ff4730                       incq	0x30(%r15)
// Frames: F33 -> F27 -> F176
// 0x14198a11f  49ff4740                       incq	0x40(%r15)
// Frames: F177
// 0x14198a123  4c8b6d70                       movq	0x70(%rbp), %r13
// 0x14198a127  488b4d60                       movq	0x60(%rbp), %rcx
// Frames: F132 -> F133 -> F177
// 0x14198a12b  4c89ea                         movq	%r13, %rdx
// 0x14198a12e  e83d9fe9fe                     callq	0x140824070 <.text+0x823070>
// Frames: F136 -> F132 -> F133 -> F177
// 0x14198a133  84c0                           testb	%al, %al
// 0x14198a135  0f84cc040000                   je	0x14198a607 <.text+0x1989607>
// Frames: F71 -> F177
// 0x14198a13b  31c9                           xorl	%ecx, %ecx
// 0x14198a13d  e80e91c0fe                     callq	0x140593250 <.text+0x592250>
// Frames: F73 -> F71 -> F177
// 0x14198a142  488d8db0000000                 leaq	0xb0(%rbp), %rcx
// 0x14198a149  89c2                           movl	%eax, %edx
// 0x14198a14b  e86064cdfe                     callq	0x1406605b0 <.text+0x65f5b0>
// Frames: F74 -> F177
// 0x14198a150  e8cb8b67fe                     callq	0x140002d20 <.text+0x1d20>
// Frames: F75 -> F177
// 0x14198a155  c685c000000009                 movb	$0x9, 0xc0(%rbp)
// 0x14198a15c  0f29b580000000                 movaps	%xmm6, 0x80(%rbp)
// 0x14198a163  488d8590000000                 leaq	0x90(%rbp), %rax
// 0x14198a16a  c7400800000000                 movl	$0x0, 0x8(%rax)
// 0x14198a171  48c70000000000                 movq	$0x0, (%rax)
// 0x14198a178  48b90000803f0000803f           movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
// 0x14198a182  4889480c                       movq	%rcx, 0xc(%rax)
// 0x14198a186  c740140000803f                 movl	$0x3f800000, 0x14(%rax) # imm = 0x3F800000
// 0x14198a18d  4885ff                         testq	%rdi, %rdi
// Frames: F138 -> F139 -> F140 -> F159 -> F177
// 0x14198a190  747e                           je	0x14198a210 <.text+0x1989210>
// Frames: F142 -> F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a192  488b07                         movq	(%rdi), %rax
// Frames: F148 -> F149 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a195  4c8b7710                       movq	0x10(%rdi), %r14
// Frames: F150 -> F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a199  4c29f0                         subq	%r14, %rax
// Frames: F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a19c  4883f867                       cmpq	$0x67, %rax
// Frames: F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a1a0  0f8632030000                   jbe	0x14198a4d8 <.text+0x19894d8>
// Frames: F23 -> F24 -> F151 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a1a6  488b4708                       movq	0x8(%rdi), %rax
// Frames: F152 -> F153 -> F154 -> F155 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a1aa  488d0d5f1bf4fe                 leaq	-0x10be4a1(%rip), %rcx  # 0x1408cbd10 <.text+0x8cad10>
// 0x14198a1b1  4a890c30                       movq	%rcx, (%rax,%r14)
// 0x14198a1b5  0f288580000000                 movaps	0x80(%rbp), %xmm0
// 0x14198a1bc  0f288d90000000                 movaps	0x90(%rbp), %xmm1
// 0x14198a1c3  0f2895a0000000                 movaps	0xa0(%rbp), %xmm2
// 0x14198a1ca  0f289db0000000                 movaps	0xb0(%rbp), %xmm3
// 0x14198a1d1  420f11443008                   movups	%xmm0, 0x8(%rax,%r14)
// 0x14198a1d7  420f114c3018                   movups	%xmm1, 0x18(%rax,%r14)
// 0x14198a1dd  420f11543028                   movups	%xmm2, 0x28(%rax,%r14)
// 0x14198a1e3  420f115c3038                   movups	%xmm3, 0x38(%rax,%r14)
// 0x14198a1e9  0f2885c0000000                 movaps	0xc0(%rbp), %xmm0
// 0x14198a1f0  420f11443048                   movups	%xmm0, 0x48(%rax,%r14)
// 0x14198a1f6  4e896c3058                     movq	%r13, 0x58(%rax,%r14)
// Frames: F156 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a1fb  4983c668                       addq	$0x68, %r14
// Frames: F157 -> F156 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a1ff  4c897710                       movq	%r14, 0x10(%rdi)
// 0x14198a203  e97c010000                     jmp	0x14198a384 <.text+0x1989384>
// 0x14198a208  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a210  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198a214  488b5578                       movq	0x78(%rbp), %rdx
// 0x14198a218  e823a3e9fe                     callq	0x140824540 <.text+0x823540>
// 0x14198a21d  4c8b7540                       movq	0x40(%rbp), %r14
// Frames: F142 -> F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a221  498b06                         movq	(%r14), %rax
// Frames: F148 -> F149 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a224  498b5e10                       movq	0x10(%r14), %rbx
// Frames: F150 -> F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a228  4829d8                         subq	%rbx, %rax
// Frames: F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a22b  4883f867                       cmpq	$0x67, %rax
// Frames: F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a22f  0f86fb020000                   jbe	0x14198a530 <.text+0x1989530>
// Frames: F23 -> F24 -> F151 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a235  498b4608                       movq	0x8(%r14), %rax
// Frames: F152 -> F153 -> F154 -> F155 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a239  488d0dd01af4fe                 leaq	-0x10be530(%rip), %rcx  # 0x1408cbd10 <.text+0x8cad10>
// 0x14198a240  48890c18                       movq	%rcx, (%rax,%rbx)
// 0x14198a244  0f288580000000                 movaps	0x80(%rbp), %xmm0
// 0x14198a24b  0f288d90000000                 movaps	0x90(%rbp), %xmm1
// 0x14198a252  0f2895a0000000                 movaps	0xa0(%rbp), %xmm2
// 0x14198a259  0f289db0000000                 movaps	0xb0(%rbp), %xmm3
// 0x14198a260  0f11441808                     movups	%xmm0, 0x8(%rax,%rbx)
// 0x14198a265  0f114c1818                     movups	%xmm1, 0x18(%rax,%rbx)
// 0x14198a26a  0f11541828                     movups	%xmm2, 0x28(%rax,%rbx)
// 0x14198a26f  0f115c1838                     movups	%xmm3, 0x38(%rax,%rbx)
// 0x14198a274  0f2885c0000000                 movaps	0xc0(%rbp), %xmm0
// 0x14198a27b  0f11441848                     movups	%xmm0, 0x48(%rax,%rbx)
// 0x14198a280  4c896c1858                     movq	%r13, 0x58(%rax,%rbx)
// Frames: F156 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a285  4883c368                       addq	$0x68, %rbx
// Frames: F157 -> F156 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a289  49895e10                       movq	%rbx, 0x10(%r14)
// 0x14198a28d  e9f2000000                     jmp	0x14198a384 <.text+0x1989384>
// Frames: F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a292  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198a296  488b5578                       movq	0x78(%rbp), %rdx
// 0x14198a29a  e8a1a2e9fe                     callq	0x140824540 <.text+0x823540>
// 0x14198a29f  4c8b7540                       movq	0x40(%rbp), %r14
// Frames: F142 -> F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a2a3  498b06                         movq	(%r14), %rax
// Frames: F148 -> F149 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a2a6  498b5e10                       movq	0x10(%r14), %rbx
// Frames: F150 -> F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a2aa  4829d8                         subq	%rbx, %rax
// Frames: F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a2ad  4883f867                       cmpq	$0x67, %rax
// Frames: F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a2b1  0f868f020000                   jbe	0x14198a546 <.text+0x1989546>
// Frames: F23 -> F24 -> F151 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a2b7  498b4608                       movq	0x8(%r14), %rax
// Frames: F152 -> F153 -> F154 -> F155 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a2bb  488d0d4e1af4fe                 leaq	-0x10be5b2(%rip), %rcx  # 0x1408cbd10 <.text+0x8cad10>
// 0x14198a2c2  48890c18                       movq	%rcx, (%rax,%rbx)
// 0x14198a2c6  0f288580000000                 movaps	0x80(%rbp), %xmm0
// 0x14198a2cd  0f288d90000000                 movaps	0x90(%rbp), %xmm1
// 0x14198a2d4  0f2895a0000000                 movaps	0xa0(%rbp), %xmm2
// 0x14198a2db  0f289db0000000                 movaps	0xb0(%rbp), %xmm3
// 0x14198a2e2  0f11441808                     movups	%xmm0, 0x8(%rax,%rbx)
// 0x14198a2e7  0f114c1818                     movups	%xmm1, 0x18(%rax,%rbx)
// 0x14198a2ec  0f11541828                     movups	%xmm2, 0x28(%rax,%rbx)
// 0x14198a2f1  0f115c1838                     movups	%xmm3, 0x38(%rax,%rbx)
// 0x14198a2f6  0f2885c0000000                 movaps	0xc0(%rbp), %xmm0
// 0x14198a2fd  0f11441848                     movups	%xmm0, 0x48(%rax,%rbx)
// 0x14198a302  4c896c1858                     movq	%r13, 0x58(%rax,%rbx)
// Frames: F156 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a307  4883c368                       addq	$0x68, %rbx
// Frames: F157 -> F156 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a30b  49895e10                       movq	%rbx, 0x10(%r14)
// Frames: F178
// 0x14198a30f  410fb65c2448                   movzbl	0x48(%r12), %ebx
// Frames: F179
// 0x14198a315  488d8d90000000                 leaq	0x90(%rbp), %rcx
// 0x14198a31c  4c89e2                         movq	%r12, %rdx
// 0x14198a31f  e8cc00d3fe                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F180
// 0x14198a324  c6858000000006                 movb	$0x6, 0x80(%rbp)
// 0x14198a32b  889d81000000                   movb	%bl, 0x81(%rbp)
// 0x14198a331  8b85d0000000                   movl	0xd0(%rbp), %eax
// 0x14198a337  488b4d08                       movq	0x8(%rbp), %rcx
// Frames: F60 -> F61 -> F27 -> F180
// 0x14198a33b  8901                           movl	%eax, (%rcx)
// Frames: F63 -> F26 -> F27 -> F180
// 0x14198a33d  498b5730                       movq	0x30(%r15), %rdx
// Frames: F64 -> F27 -> F180
// 0x14198a341  498b5f40                       movq	0x40(%r15), %rbx
// Frames: F63 -> F26 -> F27 -> F180
// 0x14198a345  493b5720                       cmpq	0x20(%r15), %rdx
// 0x14198a349  750d                           jne	0x14198a358 <.text+0x1989358>
// Frames: F65 -> F26 -> F27 -> F180
// 0x14198a34b  488b4d38                       movq	0x38(%rbp), %rcx
// 0x14198a34f  e8bc018ffe                     callq	0x14027a510 <.text+0x279510>
// Frames: F25 -> F26 -> F27 -> F180
// 0x14198a354  498b5730                       movq	0x30(%r15), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F180
// 0x14198a358  498b4728                       movq	0x28(%r15), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F180
// 0x14198a35c  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F30 -> F31 -> F26 -> F27 -> F180
// 0x14198a360  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x14198a364  0f108580000000                 movups	0x80(%rbp), %xmm0
// 0x14198a36b  0f108d8c000000                 movups	0x8c(%rbp), %xmm1
// 0x14198a372  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// 0x14198a377  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// Frames: F32 -> F26 -> F27 -> F180
// 0x14198a37c  49ff4730                       incq	0x30(%r15)
// Frames: F33 -> F27 -> F180
// 0x14198a380  49ff4740                       incq	0x40(%r15)
// 0x14198a384  f3410f10442434                 movss	0x34(%r12), %xmm0
// 0x14198a38b  0f2ec7                         ucomiss	%xmm7, %xmm0
// 0x14198a38e  754b                           jne	0x14198a3db <.text+0x19893db>
// 0x14198a390  7a49                           jp	0x14198a3db <.text+0x19893db>
// Frames: F60 -> F61 -> F27 -> F134
// 0x14198a392  498b1c24                       movq	(%r12), %rbx
// 0x14198a396  8b455c                         movl	0x5c(%rbp), %eax
// 0x14198a399  488b4de8                       movq	-0x18(%rbp), %rcx
// 0x14198a39d  8901                           movl	%eax, (%rcx)
// 0x14198a39f  488b4d30                       movq	0x30(%rbp), %rcx
// Frames: F63 -> F26 -> F27 -> F134
// 0x14198a3a3  488b5130                       movq	0x30(%rcx), %rdx
// Frames: F64 -> F27 -> F134
// 0x14198a3a7  4c8b7140                       movq	0x40(%rcx), %r14
// Frames: F63 -> F26 -> F27 -> F134
// 0x14198a3ab  483b5120                       cmpq	0x20(%rcx), %rdx
// 0x14198a3af  7511                           jne	0x14198a3c2 <.text+0x19893c2>
// Frames: F65 -> F26 -> F27 -> F134
// 0x14198a3b1  488b4d00                       movq	(%rbp), %rcx
// 0x14198a3b5  e8f6f78efe                     callq	0x140279bb0 <.text+0x278bb0>
// 0x14198a3ba  488b4d30                       movq	0x30(%rbp), %rcx
// Frames: F25 -> F26 -> F27 -> F134
// 0x14198a3be  488b5130                       movq	0x30(%rcx), %rdx
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F134
// 0x14198a3c2  488b4128                       movq	0x28(%rcx), %rax
// Frames: F29 -> F25 -> F26 -> F27 -> F134
// 0x14198a3c6  48c1e204                       shlq	$0x4, %rdx
// Frames: F30 -> F31 -> F26 -> F27 -> F134
// 0x14198a3ca  4c893410                       movq	%r14, (%rax,%rdx)
// 0x14198a3ce  48895c1008                     movq	%rbx, 0x8(%rax,%rdx)
// Frames: F32 -> F26 -> F27 -> F134
// 0x14198a3d3  48ff4130                       incq	0x30(%rcx)
// Frames: F33 -> F27 -> F134
// 0x14198a3d7  48ff4140                       incq	0x40(%rcx)
// Frames: F181
// 0x14198a3db  4c8b6d70                       movq	0x70(%rbp), %r13
// 0x14198a3df  488b4d60                       movq	0x60(%rbp), %rcx
// Frames: F132 -> F133 -> F181
// 0x14198a3e3  4c89ea                         movq	%r13, %rdx
// 0x14198a3e6  e8859ce9fe                     callq	0x140824070 <.text+0x823070>
// Frames: F136 -> F132 -> F133 -> F181
// 0x14198a3eb  84c0                           testb	%al, %al
// 0x14198a3ed  0f84f6010000                   je	0x14198a5e9 <.text+0x19895e9>
// Frames: F138 -> F139 -> F140 -> F141 -> F181
// 0x14198a3f3  4885ff                         testq	%rdi, %rdi
// 0x14198a3f6  7438                           je	0x14198a430 <.text+0x1989430>
// Frames: F142 -> F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a3f8  488b07                         movq	(%rdi), %rax
// Frames: F148 -> F149 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a3fb  488b5f10                       movq	0x10(%rdi), %rbx
// Frames: F150 -> F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a3ff  4829d8                         subq	%rbx, %rax
// Frames: F143 -> F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a402  4883f80f                       cmpq	$0xf, %rax
// Frames: F144 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a406  0f86a0000000                   jbe	0x14198a4ac <.text+0x19894ac>
// Frames: F23 -> F24 -> F151 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a40c  488b4708                       movq	0x8(%rdi), %rax
// Frames: F152 -> F153 -> F154 -> F155 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a410  488d0d191df4fe                 leaq	-0x10be2e7(%rip), %rcx  # 0x1408cc130 <.text+0x8cb130>
// 0x14198a417  48890c18                       movq	%rcx, (%rax,%rbx)
// 0x14198a41b  4c896c1808                     movq	%r13, 0x8(%rax,%rbx)
// Frames: F156 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a420  4883c310                       addq	$0x10, %rbx
// Frames: F157 -> F156 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a424  48895f10                       movq	%rbx, 0x10(%rdi)
// 0x14198a428  eb4d                           jmp	0x14198a477 <.text+0x1989477>
// 0x14198a42a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a430  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x14198a437  488b5578                       movq	0x78(%rbp), %rdx
// 0x14198a43b  e800a1e9fe                     callq	0x140824540 <.text+0x823540>
// 0x14198a440  488b9d80000000                 movq	0x80(%rbp), %rbx
// Frames: F142 -> F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a447  488b03                         movq	(%rbx), %rax
// Frames: F148 -> F149 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a44a  4c8b7310                       movq	0x10(%rbx), %r14
// Frames: F150 -> F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a44e  4c29f0                         subq	%r14, %rax
// Frames: F143 -> F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a451  4883f80f                       cmpq	$0xf, %rax
// Frames: F144 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a455  0f8693000000                   jbe	0x14198a4ee <.text+0x19894ee>
// Frames: F23 -> F24 -> F151 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a45b  488b4308                       movq	0x8(%rbx), %rax
// Frames: F152 -> F153 -> F154 -> F155 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a45f  488d0dca1cf4fe                 leaq	-0x10be336(%rip), %rcx  # 0x1408cc130 <.text+0x8cb130>
// 0x14198a466  4a890c30                       movq	%rcx, (%rax,%r14)
// 0x14198a46a  4e896c3008                     movq	%r13, 0x8(%rax,%r14)
// Frames: F156 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a46f  4983c610                       addq	$0x10, %r14
// Frames: F157 -> F156 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a473  4c897310                       movq	%r14, 0x10(%rbx)
// Frames: F182
// 0x14198a477  498b1c24                       movq	(%r12), %rbx
// 0x14198a47b  8b4558                         movl	0x58(%rbp), %eax
// 0x14198a47e  488b4dd8                       movq	-0x28(%rbp), %rcx
// Frames: F60 -> F61 -> F27 -> F90
// 0x14198a482  8901                           movl	%eax, (%rcx)
// Frames: F63 -> F26 -> F27 -> F90
// 0x14198a484  488b5630                       movq	0x30(%rsi), %rdx
// Frames: F64 -> F27 -> F90
// 0x14198a488  4c8b7640                       movq	0x40(%rsi), %r14
// Frames: F63 -> F26 -> F27 -> F90
// 0x14198a48c  483b5620                       cmpq	0x20(%rsi), %rdx
// 0x14198a490  4c8b6dc0                       movq	-0x40(%rbp), %r13
// 0x14198a494  0f85a6f8ffff                   jne	0x141989d40 <.text+0x1988d40>
// Frames: F65 -> F26 -> F27 -> F90
// 0x14198a49a  488b4df8                       movq	-0x8(%rbp), %rcx
// 0x14198a49e  e80df78efe                     callq	0x140279bb0 <.text+0x278bb0>
// Frames: F25 -> F26 -> F27 -> F90
// 0x14198a4a3  488b5630                       movq	0x30(%rsi), %rdx
// 0x14198a4a7  e994f8ffff                     jmp	0x141989d40 <.text+0x1988d40>
// Frames: F183 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F181
// 0x14198a4ac  41b810000000                   movl	$0x10, %r8d
// 0x14198a4b2  4889f9                         movq	%rdi, %rcx
// 0x14198a4b5  4889da                         movq	%rbx, %rdx
// 0x14198a4b8  e8e35b7100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x14198a4bd  e94affffff                     jmp	0x14198a40c <.text+0x198940c>
// Frames: F183 -> F145 -> F146 -> F147 -> F139 -> F140 -> F141 -> F137
// 0x14198a4c2  41b810000000                   movl	$0x10, %r8d
// 0x14198a4c8  4889f9                         movq	%rdi, %rcx
// 0x14198a4cb  4889da                         movq	%rbx, %rdx
// 0x14198a4ce  e8cd5b7100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x14198a4d3  e9dbf9ffff                     jmp	0x141989eb3 <.text+0x1988eb3>
// Frames: F183 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F177
// 0x14198a4d8  41b868000000                   movl	$0x68, %r8d
// 0x14198a4de  4889f9                         movq	%rdi, %rcx
// 0x14198a4e1  4c89f2                         movq	%r14, %rdx
// 0x14198a4e4  e8b75b7100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x14198a4e9  e9b8fcffff                     jmp	0x14198a1a6 <.text+0x19891a6>
// Frames: F183 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F181
// 0x14198a4ee  41b810000000                   movl	$0x10, %r8d
// 0x14198a4f4  4889d9                         movq	%rbx, %rcx
// 0x14198a4f7  4c89f2                         movq	%r14, %rdx
// 0x14198a4fa  e8a15b7100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x14198a4ff  e957ffffff                     jmp	0x14198a45b <.text+0x198945b>
// Frames: F183 -> F145 -> F146 -> F147 -> F139 -> F140 -> F159 -> F158
// 0x14198a504  41b868000000                   movl	$0x68, %r8d
// 0x14198a50a  4889f9                         movq	%rdi, %rcx
// 0x14198a50d  4889da                         movq	%rbx, %rdx
// 0x14198a510  e88b5b7100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x14198a515  e93dfaffff                     jmp	0x141989f57 <.text+0x1988f57>
// Frames: F183 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F141 -> F137
// 0x14198a51a  41b810000000                   movl	$0x10, %r8d
// 0x14198a520  4889d9                         movq	%rbx, %rcx
// 0x14198a523  4c89f2                         movq	%r14, %rdx
// 0x14198a526  e8755b7100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x14198a52b  e9bbfaffff                     jmp	0x141989feb <.text+0x1988feb>
// Frames: F183 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F177
// 0x14198a530  41b868000000                   movl	$0x68, %r8d
// 0x14198a536  4c89f1                         movq	%r14, %rcx
// 0x14198a539  4889da                         movq	%rbx, %rdx
// 0x14198a53c  e85f5b7100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x14198a541  e9effcffff                     jmp	0x14198a235 <.text+0x1989235>
// Frames: F183 -> F145 -> F146 -> F160 -> F161 -> F139 -> F140 -> F159 -> F158
// 0x14198a546  41b868000000                   movl	$0x68, %r8d
// 0x14198a54c  4c89f1                         movq	%r14, %rcx
// 0x14198a54f  4889da                         movq	%rbx, %rdx
// 0x14198a552  e8495b7100                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x14198a557  e95bfdffff                     jmp	0x14198a2b7 <.text+0x19892b7>
// Frames: F184
// 0x14198a55c  0f28b5e0000000                 movaps	0xe0(%rbp), %xmm6
// 0x14198a563  0f28bdf0000000                 movaps	0xf0(%rbp), %xmm7
// 0x14198a56a  4881c488010000                 addq	$0x188, %rsp            # imm = 0x188
// 0x14198a571  5b                             popq	%rbx
// 0x14198a572  5f                             popq	%rdi
// 0x14198a573  5e                             popq	%rsi
// 0x14198a574  415c                           popq	%r12
// 0x14198a576  415d                           popq	%r13
// 0x14198a578  415e                           popq	%r14
// 0x14198a57a  415f                           popq	%r15
// 0x14198a57c  5d                             popq	%rbp
// 0x14198a57d  c3                             retq
// 0x14198a57e  31c0                           xorl	%eax, %eax
// Frames: F185 -> F96
// 0x14198a580  48898580000000                 movq	%rax, 0x80(%rbp)
// 0x14198a587  48898d88000000                 movq	%rcx, 0x88(%rbp)
// 0x14198a58e  488d05a38dd600                 leaq	0xd68da3(%rip), %rax    # 0x1426f3338
// 0x14198a595  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198a59a  488d0d578cd600                 leaq	0xd68c57(%rip), %rcx    # 0x1426f31f8
// 0x14198a5a1  4c8d0d808cd600                 leaq	0xd68c80(%rip), %r9     # 0x1426f3228
// 0x14198a5a8  4c8d8580000000                 leaq	0x80(%rbp), %r8
// 0x14198a5af  ba2b000000                     movl	$0x2b, %edx
// 0x14198a5b4  e867497200                     callq	0x1420aef20 <.text+0x20adf20>
// Frames: F130
// 0x14198a5b9  48c7858000000000000000         movq	$0x0, 0x80(%rbp)
// 0x14198a5c4  488d05858dd600                 leaq	0xd68d85(%rip), %rax    # 0x1426f3350
// 0x14198a5cb  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198a5d0  488d5540                       leaq	0x40(%rbp), %rdx
// 0x14198a5d4  4c8d85d7000000                 leaq	0xd7(%rbp), %r8
// 0x14198a5db  4c8d8d80000000                 leaq	0x80(%rbp), %r9
// 0x14198a5e2  31c9                           xorl	%ecx, %ecx
// 0x14198a5e4  e8f7717200                     callq	0x1420b17e0 <.text+0x20b07e0>
// Frames: F186 -> F181
// 0x14198a5e9  488d15d88dd600                 leaq	0xd68dd8(%rip), %rdx    # 0x1426f33c8
// 0x14198a5f0  4c89e9                         movq	%r13, %rcx
// 0x14198a5f3  e8b8297200                     callq	0x1420acfb0 <.text+0x20abfb0>
// Frames: F186 -> F137
// 0x14198a5f8  488d15818dd600                 leaq	0xd68d81(%rip), %rdx    # 0x1426f3380
// 0x14198a5ff  4c89e9                         movq	%r13, %rcx
// 0x14198a602  e8a9297200                     callq	0x1420acfb0 <.text+0x20abfb0>
// Frames: F186 -> F177
// 0x14198a607  488d15a28dd600                 leaq	0xd68da2(%rip), %rdx    # 0x1426f33b0
// 0x14198a60e  4c89e9                         movq	%r13, %rcx
// 0x14198a611  e89a297200                     callq	0x1420acfb0 <.text+0x20abfb0>
// Frames: F186 -> F158
// 0x14198a616  488d154b8dd600                 leaq	0xd68d4b(%rip), %rdx    # 0x1426f3368
// 0x14198a61d  4c89e9                         movq	%r13, %rcx
// 0x14198a620  e88b297200                     callq	0x1420acfb0 <.text+0x20abfb0>
// 0x14198a625  cc                             int3
// 0x14198a626  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x14198a630  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198a635  55                             pushq	%rbp
// 0x14198a636  4157                           pushq	%r15
// 0x14198a638  4156                           pushq	%r14
// 0x14198a63a  4155                           pushq	%r13
// 0x14198a63c  4154                           pushq	%r12
// 0x14198a63e  56                             pushq	%rsi
// 0x14198a63f  57                             pushq	%rdi
// 0x14198a640  53                             pushq	%rbx
// 0x14198a641  4883ec48                       subq	$0x48, %rsp
// 0x14198a645  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198a64c  0f297c2420                     movaps	%xmm7, 0x20(%rsp)
// 0x14198a651  0f29742430                     movaps	%xmm6, 0x30(%rsp)
// Frames: F169 -> F170 -> F86 -> F86 -> F86 -> F171
// 0x14198a656  48837d6800                     cmpq	$0x0, 0x68(%rbp)
// 0x14198a65b  7413                           je	0x14198a670 <.text+0x1989670>
// Frames: F172 -> F173 -> F174 -> F86 -> F86 -> F86 -> F171
// 0x14198a65d  41b801000000                   movl	$0x1, %r8d
// 0x14198a663  488b4d20                       movq	0x20(%rbp), %rcx
// 0x14198a667  488b5568                       movq	0x68(%rbp), %rdx
// 0x14198a66b  e82001a5fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F88
// 0x14198a670  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x14198a675  0f287c2420                     movaps	0x20(%rsp), %xmm7
// 0x14198a67a  4883c448                       addq	$0x48, %rsp
// 0x14198a67e  5b                             popq	%rbx
// 0x14198a67f  5f                             popq	%rdi
// 0x14198a680  5e                             popq	%rsi
// 0x14198a681  415c                           popq	%r12
// 0x14198a683  415d                           popq	%r13
// 0x14198a685  415e                           popq	%r14
// 0x14198a687  415f                           popq	%r15
// 0x14198a689  5d                             popq	%rbp
// 0x14198a68a  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/create_roof.rs:17:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F2: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:806:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:469:0 bevy_ecs::event::EventReader::read
// F4: crates/systems/roof/src/create_roof.rs:29:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:547:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F6: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:811:0 bevy_ecs::event::EventIteratorWithId::new
// F7: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:723:0 bevy_ecs::event::ManualEventReader::read_with_id
// F8: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:718:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:813:0 bevy_ecs::event::EventIteratorWithId::new
// F11: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:816:0 bevy_ecs::event::EventIteratorWithId::new
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:354:0 core::slice::index::impl$4::get
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:480:0 core::slice::index::impl$6::get
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F15: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:817:0 bevy_ecs::event::EventIteratorWithId::new
// F16: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1007:0 core::slice::impl$0::get
// F17: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:818:0 bevy_ecs::event::EventIteratorWithId::new
// F18: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:823:0 bevy_ecs::event::EventIteratorWithId::new
// F19: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F20: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F22: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:825:0 bevy_ecs::event::EventIteratorWithId::new
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1974:0 alloc::vec::Vec::push
// F26: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::event::Events::send
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:625:0 bevy_ecs::event::EventWriter::send
// F28: crates/systems/roof/src/create_roof.rs:85:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 alloc::vec::Vec::as_mut_ptr
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F31: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1975:0 alloc::vec::Vec::push
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1976:0 alloc::vec::Vec::push
// F33: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::event::Events::send
// F34: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:705:0 core::option::Option::as_mut
// F35: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:300:0 core::iter::adapters::chain::and_then_or_clear
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next
// F37: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:845:0 bevy_ecs::event::impl$23::next
// F38: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:771:0 bevy_ecs::event::impl$20::next
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next::closure$0
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1513:0 core::option::Option::or_else
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F42: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F43: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F44: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:850:0 bevy_ecs::event::impl$23::next
// F45: crates/systems/roof/src/create_roof.rs:30:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:931:0 core::option::Option::unwrap
// F47: crates/systems/roof/src/create_roof.rs:32:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F48: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::option::Option::map
// F49: crates/systems/roof/src/create_roof.rs:42:0 system_roof::create_roof::create_roof::closure$0
// F50: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F51: crates/systems/roof/src/create_roof.rs:45:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F52: crates/systems/roof/src/create_roof.rs:48:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F53: crates/systems/roof/src/create_roof.rs:49:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F54: crates/systems/roof/src/create_roof.rs:51:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F55: crates/systems/roof/src/create_roof.rs:52:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F56: crates/systems/roof/src/create_roof.rs:46:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F57: crates/systems/roof/src/create_roof.rs:54:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F58: crates/systems/roof/src/create_roof.rs:56:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F59: crates/systems/roof/src/create_roof.rs:58:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F60: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$29::set_changed
// F61: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F62: crates/systems/roof/src/create_roof.rs:57:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F63: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1970:0 alloc::vec::Vec::push
// F64: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::event::Events::send
// F65: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1971:0 alloc::vec::Vec::push
// F66: crates/systems/roof/src/create_roof.rs:63:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F67: crates/systems/roof/src/create_roof.rs:64:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F68: crates/systems/roof/src/create_roof.rs:65:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F69: crates/systems/roof/src/create_roof.rs:66:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F70: crates/systems/roof/src/create_roof.rs:68:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F71: crates/systems/roof/src/visual/switch_roof_type.rs:105:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F72: crates/systems/roof/src/create_roof.rs:71:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F73: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/convert/mod.rs:759:0 core::convert::impl$3::into
// F74: crates/systems/roof/src/visual/switch_roof_type.rs:109:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F75: crates/systems/roof/src/visual/switch_roof_type.rs:102:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F76: crates/systems/roof/src/create_roof.rs:62:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F77: crates/systems/roof/src/create_roof.rs:74:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F78: crates/systems/roof/src/create_roof.rs:77:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F79: crates/systems/roof/src/create_roof.rs:79:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F80: crates/systems/roof/src/create_roof.rs:83:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F81: crates/systems/roof/src/create_roof.rs:76:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F82: crates/systems/roof/src/create_roof.rs:91:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F83: crates/systems/roof/src/create_roof.rs:37:0 system_roof::create_roof::create_roof::closure$0
// F84: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:343:0 core::fmt::Arguments::new_v1
// F85: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:933:0 core::option::Option::unwrap
// F86: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:515:0 core::ptr::drop_in_place
// F87: crates/systems/roof/src/create_roof.rs:72:0 system_roof::create_roof::create_roof::hcd5aafd543c9856b
// F88: crates/systems/roof/src/visual/switch_roof_type.rs:18:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F89: crates/systems/roof/src/visual/switch_roof_type.rs:32:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F90: crates/systems/roof/src/visual/switch_roof_type.rs:94:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F91: crates/systems/roof/src/visual/switch_roof_type.rs:29:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F92: crates/systems/roof/src/visual/switch_roof_type.rs:30:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F93: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:740:0 bevy_ecs::entity::Entities::get
// F94: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:866:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F95: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:850:0 bevy_ecs::system::query::Query::get
// F96: crates/systems/roof/src/visual/switch_roof_type.rs:34:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F97: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F98: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1488:0 core::cmp::impls::impl$24::ne
// F99: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/nonzero.rs:137:0 core::num::nonzero::impl$2::ne
// F100: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:741:0 bevy_ecs::entity::Entities::get
// F101: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:74:0 bevy_ecs::archetype::impl$24::eq
// F102: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:742:0 bevy_ecs::entity::Entities::get
// F103: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F104: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:870:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F105: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 core::bool::impl$0::then
// F106: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F107: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F108: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F109: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F110: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/bit.rs:178:0 fixedbitset::FixedBitSet::get_unchecked
// F111: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1952:0 core::result::impl$26::branch
// F112: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F113: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:886:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F114: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:888:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F115: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:693:0 bevy_ecs::storage::table::Table::get_column
// F116: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:955:0 bevy_ecs::query::fetch::impl$21::set_table
// F117: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:942:0 bevy_ecs::query::fetch::impl$21::set_archetype
// F118: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F119: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F120: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::impl$11::get::closure$0
// F121: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F122: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs:398:0 nonmax::NonMaxUsize::get
// F123: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:372:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F124: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:412:0 bevy_ecs::storage::blob_vec::BlobVec::get_slice
// F125: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:322:0 bevy_ecs::storage::table::Column::get_data_slice
// F126: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:957:0 bevy_ecs::query::fetch::impl$21::set_table
// F127: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs:506:0 bevy_ptr::ThinSlicePtr::get
// F128: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:973:0 bevy_ecs::query::fetch::impl$21::fetch
// F129: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:892:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F130: crates/systems/roof/src/visual/switch_roof_type.rs:37:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F131: crates/country-core/src/resources/roofs/roof_shape.rs:30:0 country_core::resources::roofs::roof_shape::impl$9::eq
// F132: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:448:0 bevy_ecs::system::commands::Commands::get_entity
// F133: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:408:0 bevy_ecs::system::commands::Commands::entity
// F134: crates/systems/roof/src/visual/switch_roof_type.rs:0:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F135: crates/systems/roof/src/visual/switch_roof_type.rs:39:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F136: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:34:0 core::bool::impl$0::then_some
// F137: crates/systems/roof/src/visual/switch_roof_type.rs:41:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F138: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:502:0 bevy_ecs::system::commands::Commands::push
// F139: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:749:0 bevy_ecs::system::commands::Commands::add
// F140: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:1101:0 bevy_ecs::system::commands::EntityCommands::add
// F141: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:1040:0 bevy_ecs::system::commands::EntityCommands::remove
// F142: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:271:0 alloc::raw_vec::RawVec::capacity
// F143: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:415:0 alloc::raw_vec::RawVec::needs_to_grow
// F144: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:332:0 alloc::raw_vec::RawVec::reserve
// F145: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:945:0 alloc::vec::Vec::reserve
// F146: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:200:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F147: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:510:0 bevy_ecs::system::commands::Commands::push
// F148: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2203:0 alloc::vec::Vec::len
// F149: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:197:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F150: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 alloc::raw_vec::RawVec::capacity
// F151: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:204:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F152: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2987:0 core::intrinsics::copy_nonoverlapping
// F153: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1584:0 core::ptr::write_unaligned
// F154: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1551:0 core::ptr::mut_ptr::impl$0::write_unaligned
// F155: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:213:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F156: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:220:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F157: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1458:0 alloc::vec::Vec::set_len
// F158: crates/systems/roof/src/visual/switch_roof_type.rs:69:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F159: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:946:0 bevy_ecs::system::commands::EntityCommands::insert
// F160: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:84:0 bevy_ecs::world::command_queue::CommandQueue::push
// F161: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:504:0 bevy_ecs::system::commands::Commands::push
// F162: crates/systems/roof/src/visual/switch_roof_type.rs:45:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F163: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/fmt.rs:637:0 alloc::fmt::format::closure$0
// F164: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1179:0 core::option::Option::map_or_else
// F165: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/fmt.rs:637:0 alloc::fmt::format
// F166: crates/systems/roof/src/visual/switch_roof_type.rs:44:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F167: crates/systems/roof/src/visual/switch_roof_type.rs:47:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F168: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:2106:0 alloc::sync::impl$29::deref
// F169: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:280:0 alloc::raw_vec::RawVec::current_memory
// F170: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:554:0 alloc::raw_vec::impl$4::drop
// F171: crates/systems/roof/src/visual/switch_roof_type.rs:48:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F172: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:119:0 alloc::alloc::dealloc
// F173: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:256:0 alloc::alloc::impl$1::deallocate
// F174: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:555:0 alloc::raw_vec::impl$4::drop
// F175: crates/systems/roof/src/visual/switch_roof_type.rs:52:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F176: crates/systems/roof/src/visual/switch_roof_type.rs:50:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F177: crates/systems/roof/src/visual/switch_roof_type.rs:55:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F178: crates/systems/roof/src/visual/switch_roof_type.rs:74:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F179: crates/systems/roof/src/visual/switch_roof_type.rs:78:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F180: crates/systems/roof/src/visual/switch_roof_type.rs:71:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F181: crates/systems/roof/src/visual/switch_roof_type.rs:91:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F182: crates/systems/roof/src/visual/switch_roof_type.rs:95:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F183: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:333:0 alloc::raw_vec::RawVec::reserve
// F184: crates/systems/roof/src/visual/switch_roof_type.rs:98:0 system_roof::visual::switch_roof_type::switch_roof_type::h1e8eb02bb4ba23b3
// F185: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1077:0 core::result::Result::unwrap
// F186: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:410:0 bevy_ecs::system::commands::Commands::entity
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/visual/switch_roof_type.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 701d6f23460d0c97373e4ee893c089478258f3a24d1da6746089ad0cfd2c05b7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: switch_roof_type.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3196, 3198
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use system_roof; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::switch_roof_type::switch_roof_type
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3198,"record_offset":852,"rva":26778592,"va":5395487712}
// 编译实例 RVAs: 0x1989be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=2
//     target_aliases=1889 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26767842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e81e8e67fe","size":5,"soft_fail":false,"target_va":5368720672,"va":5395488509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22739242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8d606a5fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395488949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20933791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e86193c0fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5395488490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140596a60
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20919859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8cdc9c0fe","size":5,"soft_fail":false,"target_va":5374569056,"va":5395488910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406605b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20093261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8b366cdfe","size":5,"soft_fail":false,"target_va":5375395248,"va":5395488504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17958980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8bcf7edfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5395488863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t7429091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8e35b7100","size":5,"soft_fail":false,"target_va":5402919072,"va":5395489976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-24181731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e81d048ffe","size":5,"soft_fail":false,"target_va":5371307280,"va":5395489006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18243098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8e6a1e9fe","size":5,"soft_fail":false,"target_va":5377245296,"va":5395488389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t7481784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8b8297200","size":5,"soft_fail":false,"target_va":5402972080,"va":5395490291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-18242192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e870a5e9fe","size":5,"soft_fail":false,"target_va":5377246528,"va":5395488715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7500279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8f7717200","size":5,"soft_fail":false,"target_va":5402990560,"va":5395490276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7489895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e867497200","size":5,"soft_fail":false,"target_va":5402980128,"va":5395490228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::try_get_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20920673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e89fc6c0fe","size":5,"soft_fail":false,"target_va":5374568224,"va":5395488892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13244162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8fee835ff","size":5,"soft_fail":false,"target_va":5382244768,"va":5395488925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19725529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e82703d3fe","size":5,"soft_fail":false,"target_va":5375763440,"va":5395488964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19724879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395487712,"raw_hex":"e8b105d3fe","size":5,"soft_fail":false,"target_va":5375763488,"va":5395488362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3198,"record_offset":5468,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3198,"record_offset":5552,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3198,"record_offset":1040,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3198,"record_offset":1584,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3198,"record_offset":1860,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3198,"record_offset":3148,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3198,"record_offset":4460,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::remove
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3198,"record_offset":3296,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":25442,"source_line":1036,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3198,"record_offset":2172,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3198,"record_offset":5580,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3198,"record_offset":8784,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3198,"record_offset":3120,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":101542,"source_line":30,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_direct] system_roof::visual::switch_roof_type::stone_floor_bundle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/switch_roof_type.rs
//     evidence={"module":3198,"record_offset":4396,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":101532,"source_line":101,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3198,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":852,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3198,"record_offset":5708,"parent_record_offset":5652,"ancestor_record_offset":852,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3198,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":852,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3198,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":852,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3198,"record_offset":5680,"parent_record_offset":5652,"ancestor_record_offset":852,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3198,"record_offset":3580,"parent_record_offset":3544,"ancestor_record_offset":852,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3198,"record_offset":1732,"parent_record_offset":1700,"ancestor_record_offset":852,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3198,"record_offset":3544,"parent_record_offset":3512,"ancestor_record_offset":852,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3198,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":852,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3198,"record_offset":1700,"parent_record_offset":1656,"ancestor_record_offset":852,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3198,"record_offset":3660,"parent_record_offset":3464,"ancestor_record_offset":852,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3198,"record_offset":1656,"parent_record_offset":1612,"ancestor_record_offset":852,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3198,"record_offset":3512,"parent_record_offset":3464,"ancestor_record_offset":852,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3198,"record_offset":3828,"parent_record_offset":3464,"ancestor_record_offset":852,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3198,"record_offset":2384,"parent_record_offset":2240,"ancestor_record_offset":852,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3198,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":852,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3198,"record_offset":1800,"parent_record_offset":1584,"ancestor_record_offset":852,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3198,"record_offset":2240,"parent_record_offset":2196,"ancestor_record_offset":852,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3198,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":852,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3198,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":852,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3198,"record_offset":1104,"parent_record_offset":1064,"ancestor_record_offset":852,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3198,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":852,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3198,"record_offset":3028,"parent_record_offset":2196,"ancestor_record_offset":852,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3198,"record_offset":2660,"parent_record_offset":2196,"ancestor_record_offset":852,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3198,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":852,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3198,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":852,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3198,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":852,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3198,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":852,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3198,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":852,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3198,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":852,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3198,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":852,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3198,"record_offset":2964,"parent_record_offset":2684,"ancestor_record_offset":852,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3198,"record_offset":2712,"parent_record_offset":2684,"ancestor_record_offset":852,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3198,"record_offset":2632,"parent_record_offset":2196,"ancestor_record_offset":852,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3198,"record_offset":3376,"parent_record_offset":3336,"ancestor_record_offset":852,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3198,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":852,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3198,"record_offset":3416,"parent_record_offset":3376,"ancestor_record_offset":852,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3198,"record_offset":3336,"parent_record_offset":3296,"ancestor_record_offset":852,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3198,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":852,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3198,"record_offset":3860,"parent_record_offset":3416,"ancestor_record_offset":852,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3198,"record_offset":3464,"parent_record_offset":3416,"ancestor_record_offset":852,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3198,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":852,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3198,"record_offset":2448,"parent_record_offset":2416,"ancestor_record_offset":852,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3198,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":852,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3198,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":852,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3198,"record_offset":4428,"parent_record_offset":4396,"ancestor_record_offset":852,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3198,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":852,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3198,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":852,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3198,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":852,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3198,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":852,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3198,"record_offset":1176,"parent_record_offset":1128,"ancestor_record_offset":852,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3198,"record_offset":2328,"parent_record_offset":2240,"ancestor_record_offset":852,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3198,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":852,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3198,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":852,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3198,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":852,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3198,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":852,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3198,"record_offset":1992,"parent_record_offset":1912,"ancestor_record_offset":852,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3198,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3198,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":852,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3198,"record_offset":3744,"parent_record_offset":3464,"ancestor_record_offset":852,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3198,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":852,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3198,"record_offset":1764,"parent_record_offset":1656,"ancestor_record_offset":852,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3198,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":852,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3198,"record_offset":2604,"parent_record_offset":2196,"ancestor_record_offset":852,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3198,"record_offset":1240,"parent_record_offset":1128,"ancestor_record_offset":852,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3198,"record_offset":1428,"parent_record_offset":1128,"ancestor_record_offset":852,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3198,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":852,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3198,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":852,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3198,"record_offset":1276,"parent_record_offset":1240,"ancestor_record_offset":852,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3198,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":852,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3198,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":852,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3198,"record_offset":2068,"parent_record_offset":2016,"ancestor_record_offset":852,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3198,"record_offset":2416,"parent_record_offset":2196,"ancestor_record_offset":852,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3198,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":852,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3198,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":852,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3198,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":852,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3198,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":852,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03198.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::switch_roof_type::stone_floor_bundle
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18C9C
// 首条位置: {"module":3196,"inlinee":101532}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3196,"record_offset":2264,"parent_record_offset":2232,"ancestor_record_offset":2232,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03196.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::visual::switch_roof_type::stone_floor_bundle
// 信息 ID: 0363aae818f7593f53c046b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::create_roof::create_roof; module=3196, inline_record=2232; 不是独立调用边。
// [PDB 内联宿主] system_roof::visual::switch_roof_type::switch_roof_type; module=3198, inline_record=4396; 不是独立调用边。
// [PDB 内联宿主] system_roof::visual::switch_roof_type::switch_roof_type; module=3198, inline_record=6132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::switch_roof_type::switch_roof_type
// 信息 ID: 960d8d325bb76c3afa06d431
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 282da8d2586e414caff12814: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > @ 0x12d59d9 -> 0x1989be0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> @ 0x12e1124 -> 0x1989be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
