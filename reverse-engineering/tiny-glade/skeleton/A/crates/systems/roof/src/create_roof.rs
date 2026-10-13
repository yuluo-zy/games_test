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
// Inline site 3196:1904 -> host 3196:564 / rva_1988750_len_7a5; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1988984, 0x198899a), 4 instructions
//   Function-frame RVA span [0x1988da0, 0x1988df9), 14 instructions
//
// FUNCTION system_roof::create_roof::create_roof
// Candidate interfaces: be7f5cd5abb1d07943bf22ad
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3196:564 -> rva_1988750_len_7a5
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
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/create_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: cb1556f254b148c0359dba47e9f76bc27c9d1b898d41f4292bfd83758e485da7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: create_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3196
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::create_roof::create_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3196,"record_offset":564,"rva":26773328,"va":5395482448}
// 编译实例 RVAs: 0x1988750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26762782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8e2a167fe","size":5,"soft_fail":false,"target_va":5368720672,"va":5395483449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003390
//     owner=unresolved_namespace; count=1
//     target_aliases=1000 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26761084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e884a867fe","size":5,"soft_fail":false,"target_va":5368722320,"va":5395483399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402daad0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23782398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8021c95fe","size":5,"soft_fail":false,"target_va":5371701968,"va":5395484361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21422092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8f41fb9fe","size":5,"soft_fail":false,"target_va":5374061056,"va":5395483143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20928731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e825a7c0fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5395483430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406605b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20088201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8777acdfe","size":5,"soft_fail":false,"target_va":5375395248,"va":5395483444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18539856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8b01ae5fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5395483851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cd060
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17546760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8f841f4fe","size":5,"soft_fail":false,"target_va":5377937504,"va":5395484259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-24178386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e82e118ffe","size":5,"soft_fail":false,"target_va":5371304880,"va":5395483261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17549033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e81739f4fe","size":5,"soft_fail":false,"target_va":5377934720,"va":5395483748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7494715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e83b5c7200","size":5,"soft_fail":false,"target_va":5402978880,"va":5395484160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7494791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8875c7200","size":5,"soft_fail":false,"target_va":5402978944,"va":5395484148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_animation::RoofAnimation::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11293328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e870ad53ff","size":5,"soft_fail":false,"target_va":5384190064,"va":5395483387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19719639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8291ad3fe","size":5,"soft_fail":false,"target_va":5375763568,"va":5395483202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19719828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e86c19d3fe","size":5,"soft_fail":false,"target_va":5375763360,"va":5395483183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19719708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8e419d3fe","size":5,"soft_fail":false,"target_va":5375763488,"va":5395483191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_tile_sim::RoofTileSimulation::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20147279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8b193ccfe","size":5,"soft_fail":false,"target_va":5375336096,"va":5395483370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21422891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8d51cb9fe","size":5,"soft_fail":false,"target_va":5374060496,"va":5395483382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13550219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8753d31ff","size":5,"soft_fail":false,"target_va":5381932864,"va":5395483078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20047997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e88317cefe","size":5,"soft_fail":false,"target_va":5375434976,"va":5395482968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19760013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395482448,"raw_hex":"e8737cd2fe","size":5,"soft_fail":false,"target_va":5375723808,"va":5395483816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3196,"record_offset":712,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3196,"record_offset":1248,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3196,"record_offset":1524,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3196,"record_offset":1872,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3196,"record_offset":1836,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3196,"record_offset":2584,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_direct] system_roof::visual::switch_roof_type::stone_floor_bundle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/switch_roof_type.rs
//     evidence={"module":3196,"record_offset":2232,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":101532,"source_line":101,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3196,"record_offset":1396,"parent_record_offset":1364,"ancestor_record_offset":564,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3196,"record_offset":1364,"parent_record_offset":1320,"ancestor_record_offset":564,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3196,"record_offset":1320,"parent_record_offset":1276,"ancestor_record_offset":564,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3196,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":564,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3196,"record_offset":1464,"parent_record_offset":1248,"ancestor_record_offset":564,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3196,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":564,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3196,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":564,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3196,"record_offset":776,"parent_record_offset":736,"ancestor_record_offset":564,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3196,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":564,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3196,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":564,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3196,"record_offset":2264,"parent_record_offset":2232,"ancestor_record_offset":564,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3196,"record_offset":1936,"parent_record_offset":1904,"ancestor_record_offset":564,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3196,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":564,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3196,"record_offset":1576,"parent_record_offset":1548,"ancestor_record_offset":564,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3196,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":564,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3196,"record_offset":844,"parent_record_offset":800,"ancestor_record_offset":564,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3196,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":564,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3196,"record_offset":1656,"parent_record_offset":1576,"ancestor_record_offset":564,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3196,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":564,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3196,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":564,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3196,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":564,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3196,"record_offset":1428,"parent_record_offset":1320,"ancestor_record_offset":564,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3196,"record_offset":908,"parent_record_offset":800,"ancestor_record_offset":564,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3196,"record_offset":1092,"parent_record_offset":800,"ancestor_record_offset":564,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3196,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":564,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3196,"record_offset":940,"parent_record_offset":908,"ancestor_record_offset":564,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3196,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":564,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3196,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":564,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3196,"record_offset":1732,"parent_record_offset":1680,"ancestor_record_offset":564,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::create_roof::create_roof::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/create_roof.rs
//     evidence={"module":3196,"record_offset":1904,"parent_record_offset":1872,"ancestor_record_offset":564,"inlinee":101530,"source_line":34,"archive":"_pdb/A/modules/03196.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::create_roof::create_roof::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18C9A
// 首条位置: {"module":3196,"inlinee":101530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3196,"record_offset":1936,"parent_record_offset":1904,"ancestor_record_offset":1904,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03196.jsonl.gz"}
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
// 完整结构及原始类型记录: create_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::create_roof::create_roof
// 信息 ID: ecad1ceabda5f8b737c1cb3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] be7f5cd5abb1d07943bf22ad: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)> > @ 0x12d4f87 -> 0x1988750 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)> @ 0x12decae -> 0x1988750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::create_roof::create_roof::closure$0
// 信息 ID: 912f5a54375596695325253d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::create_roof::create_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::create_roof::create_roof; module=3196, inline_record=1904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
