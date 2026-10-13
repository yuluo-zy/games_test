// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: simulate_roof_tile_for_audio.rs.assembly.json.gz

// FUNCTION system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3776:3628 -> host 3776:268 / rva_28219b0_len_6ec; exact_symbolizer_function_frame
//   Function-frame RVA span [0x2821d7b, 0x2821e65), 54 instructions
//   Function-frame RVA span [0x2822034, 0x282209c), 14 instructions
//
// FUNCTION system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// Candidate interfaces: ea9a227b3d2821787617065a
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3776:268 -> rva_28219b0_len_6ec
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_28219b0_len_6ec: RVA 0x28219b0, 1772 PDB bytes; 368 instructions
// Evidence: _assembly/roof-v1/C/pack-002/rva_28219b0_len_6ec.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3776:268: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// Frames: F1
// 0x1428219b0  4157                           pushq	%r15
// 0x1428219b2  4156                           pushq	%r14
// 0x1428219b4  4155                           pushq	%r13
// 0x1428219b6  4154                           pushq	%r12
// 0x1428219b8  56                             pushq	%rsi
// 0x1428219b9  57                             pushq	%rdi
// 0x1428219ba  55                             pushq	%rbp
// 0x1428219bb  53                             pushq	%rbx
// 0x1428219bc  4881ec68010000                 subq	$0x168, %rsp            # imm = 0x168
// 0x1428219c3  440f29bc2450010000             movaps	%xmm15, 0x150(%rsp)
// 0x1428219cc  440f29b42440010000             movaps	%xmm14, 0x140(%rsp)
// 0x1428219d5  440f29ac2430010000             movaps	%xmm13, 0x130(%rsp)
// 0x1428219de  440f29a42420010000             movaps	%xmm12, 0x120(%rsp)
// 0x1428219e7  440f299c2410010000             movaps	%xmm11, 0x110(%rsp)
// 0x1428219f0  440f29942400010000             movaps	%xmm10, 0x100(%rsp)
// 0x1428219f9  440f298c24f0000000             movaps	%xmm9, 0xf0(%rsp)
// 0x142821a02  440f298424e0000000             movaps	%xmm8, 0xe0(%rsp)
// 0x142821a0b  0f29bc24d0000000               movaps	%xmm7, 0xd0(%rsp)
// 0x142821a13  0f29b424c0000000               movaps	%xmm6, 0xc0(%rsp)
// Frames: F2
// 0x142821a1b  4d8b30                         movq	(%r8), %r14
// 0x142821a1e  f3410f104628                   movss	0x28(%r14), %xmm0
// 0x142821a24  0f57f6                         xorps	%xmm6, %xmm6
// 0x142821a27  0f2ec6                         ucomiss	%xmm6, %xmm0
// 0x142821a2a  756e                           jne	0x142821a9a <.text+0x2820a9a>
// 0x142821a2c  7a6c                           jp	0x142821a9a <.text+0x2820a9a>
// Frames: F3
// 0x142821a2e  0f28b424c0000000               movaps	0xc0(%rsp), %xmm6
// 0x142821a36  0f28bc24d0000000               movaps	0xd0(%rsp), %xmm7
// 0x142821a3e  440f288424e0000000             movaps	0xe0(%rsp), %xmm8
// 0x142821a47  440f288c24f0000000             movaps	0xf0(%rsp), %xmm9
// 0x142821a50  440f28942400010000             movaps	0x100(%rsp), %xmm10
// 0x142821a59  440f289c2410010000             movaps	0x110(%rsp), %xmm11
// 0x142821a62  440f28a42420010000             movaps	0x120(%rsp), %xmm12
// 0x142821a6b  440f28ac2430010000             movaps	0x130(%rsp), %xmm13
// 0x142821a74  440f28b42440010000             movaps	0x140(%rsp), %xmm14
// 0x142821a7d  440f28bc2450010000             movaps	0x150(%rsp), %xmm15
// 0x142821a86  4881c468010000                 addq	$0x168, %rsp            # imm = 0x168
// 0x142821a8d  5b                             popq	%rbx
// 0x142821a8e  5d                             popq	%rbp
// 0x142821a8f  5f                             popq	%rdi
// 0x142821a90  5e                             popq	%rsi
// 0x142821a91  415c                           popq	%r12
// 0x142821a93  415d                           popq	%r13
// 0x142821a95  415e                           popq	%r14
// 0x142821a97  415f                           popq	%r15
// 0x142821a99  c3                             retq
// Frames: F4 -> F5 -> F6 -> F7 -> F8 -> F9
// 0x142821a9a  4c8b01                         movq	(%rcx), %r8
// 0x142821a9d  488b4108                       movq	0x8(%rcx), %rax
// 0x142821aa1  4889442460                     movq	%rax, 0x60(%rsp)
// 0x142821aa6  8b4114                         movl	0x14(%rcx), %eax
// 0x142821aa9  89442438                       movl	%eax, 0x38(%rsp)
// Frames: F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F16 -> F17 -> F18 -> F8 -> F9
// 0x142821aad  4d8ba8f8000000                 movq	0xf8(%r8), %r13
// Frames: F14 -> F15 -> F16 -> F17 -> F18 -> F8 -> F9
// 0x142821ab4  498b8000010000                 movq	0x100(%r8), %rax
// Frames: F19 -> F20 -> F21 -> F16 -> F17 -> F18 -> F8 -> F9
// 0x142821abb  498d748500                     leaq	(%r13,%rax,4), %rsi
// 0x142821ac0  4c89442448                     movq	%r8, 0x48(%rsp)
// Frames: F22 -> F17 -> F18 -> F8 -> F9
// 0x142821ac5  410fb68008010000               movzbl	0x108(%r8), %eax
// 0x142821acd  8844242f                       movb	%al, 0x2f(%rsp)
// 0x142821ad1  8b421c                         movl	0x1c(%rdx), %eax
// 0x142821ad4  8944243c                       movl	%eax, 0x3c(%rsp)
// 0x142821ad8  488b02                         movq	(%rdx), %rax
// 0x142821adb  488b4a10                       movq	0x10(%rdx), %rcx
// 0x142821adf  48898c2490000000               movq	%rcx, 0x90(%rsp)
// 0x142821ae7  4889442440                     movq	%rax, 0x40(%rsp)
// 0x142821aec  4883c020                       addq	$0x20, %rax
// 0x142821af0  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x142821af8  31ed                           xorl	%ebp, %ebp
// 0x142821afa  b808000000                     movl	$0x8, %eax
// 0x142821aff  4889842498000000               movq	%rax, 0x98(%rsp)
// 0x142821b07  f30f103d0569d600               movss	0xd66905(%rip), %xmm7   # 0x143588414
// 0x142821b0f  f3440f100d0869d600             movss	0xd66908(%rip), %xmm9   # 0x143588420
// 0x142821b18  f3440f1015032f6e00             movss	0x6e2f03(%rip), %xmm10  # 0x142f04a24
// 0x142821b21  440f281d17c76d00               movaps	0x6dc717(%rip), %xmm11  # 0x142efe240
// 0x142821b29  f3440f1025020f7000             movss	0x700f02(%rip), %xmm12  # 0x142f22a34
// 0x142821b32  f3440f10351d6e7000             movss	0x706e1d(%rip), %xmm14  # 0x142f28958
// 0x142821b3b  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x142821b44  48c744245000000000             movq	$0x0, 0x50(%rsp)
// 0x142821b4d  4531ff                         xorl	%r15d, %r15d
// Frames: F23 -> F24 -> F9
// 0x142821b50  807c242f00                     cmpb	$0x0, 0x2f(%rsp)
// 0x142821b55  7449                           je	0x142821ba0 <.text+0x2820ba0>
// Frames: F25 -> F24 -> F9
// 0x142821b57  4139ef                         cmpl	%ebp, %r15d
// 0x142821b5a  0f856b010000                   jne	0x142821ccb <.text+0x2820ccb>
// Frames: F26 -> F27 -> F28 -> F24 -> F9
// 0x142821b60  4939f5                         cmpq	%rsi, %r13
// Frames: F27 -> F28 -> F24 -> F9
// 0x142821b63  0f84c5feffff                   je	0x142821a2e <.text+0x2820a2e>
// 0x142821b69  488b442460                     movq	0x60(%rsp), %rax
// 0x142821b6e  488b80a0010000                 movq	0x1a0(%rax), %rax
// 0x142821b75  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F28 -> F24 -> F9
// 0x142821b80  418b4d00                       movl	(%r13), %ecx
// Frames: F29 -> F30 -> F31 -> F32 -> F24 -> F9
// 0x142821b84  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F33 -> F34 -> F35 -> F36 -> F24 -> F9
// 0x142821b88  488b6cc810                     movq	0x10(%rax,%rcx,8), %rbp
// Frames: F36 -> F24 -> F9
// 0x142821b8d  4885ed                         testq	%rbp, %rbp
// 0x142821b90  7566                           jne	0x142821bf8 <.text+0x2820bf8>
// Frames: F26 -> F27 -> F28 -> F24 -> F9
// 0x142821b92  4983c504                       addq	$0x4, %r13
// 0x142821b96  4939f5                         cmpq	%rsi, %r13
// Frames: F27 -> F28 -> F24 -> F9
// 0x142821b99  75e5                           jne	0x142821b80 <.text+0x2820b80>
// 0x142821b9b  e98efeffff                     jmp	0x142821a2e <.text+0x2820a2e>
// Frames: F37 -> F24 -> F9
// 0x142821ba0  4139ef                         cmpl	%ebp, %r15d
// 0x142821ba3  754b                           jne	0x142821bf0 <.text+0x2820bf0>
// Frames: F26 -> F27 -> F38 -> F24 -> F9
// 0x142821ba5  4939f5                         cmpq	%rsi, %r13
// Frames: F27 -> F38 -> F24 -> F9
// 0x142821ba8  0f8480feffff                   je	0x142821a2e <.text+0x2820a2e>
// 0x142821bae  488b442460                     movq	0x60(%rsp), %rax
// 0x142821bb3  488b8000010000                 movq	0x100(%rax), %rax
// 0x142821bba  660f1f440000                   nopw	(%rax,%rax)
// Frames: F38 -> F24 -> F9
// 0x142821bc0  418b4d00                       movl	(%r13), %ecx
// Frames: F29 -> F30 -> F39 -> F40 -> F24 -> F9
// 0x142821bc4  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x142821bc8  48c1e105                       shlq	$0x5, %rcx
// Frames: F33 -> F34 -> F41 -> F42 -> F24 -> F9
// 0x142821bcc  488b6c0858                     movq	0x58(%rax,%rcx), %rbp
// Frames: F42 -> F24 -> F9
// 0x142821bd1  4885ed                         testq	%rbp, %rbp
// 0x142821bd4  7560                           jne	0x142821c36 <.text+0x2820c36>
// Frames: F26 -> F27 -> F38 -> F24 -> F9
// 0x142821bd6  4983c504                       addq	$0x4, %r13
// 0x142821bda  4939f5                         cmpq	%rsi, %r13
// Frames: F27 -> F38 -> F24 -> F9
// 0x142821bdd  75e1                           jne	0x142821bc0 <.text+0x2820bc0>
// 0x142821bdf  e94afeffff                     jmp	0x142821a2e <.text+0x2820a2e>
// 0x142821be4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F43 -> F24 -> F9
// 0x142821bf0  4489f8                         movl	%r15d, %eax
// 0x142821bf3  e925010000                     jmp	0x142821d1d <.text+0x2820d1d>
// Frames: F44 -> F24 -> F9
// 0x142821bf8  4c8d04c8                       leaq	(%rax,%rcx,8), %r8
// 0x142821bfc  488b442448                     movq	0x48(%rsp), %rax
// 0x142821c01  488b9018010000                 movq	0x118(%rax), %rdx
// Frames: F45 -> F46 -> F47 -> F48 -> F44 -> F24 -> F9
// 0x142821c08  498b4018                       movq	0x18(%r8), %rax
// 0x142821c0c  498b4838                       movq	0x38(%r8), %rcx
// Frames: F29 -> F30 -> F49 -> F45 -> F46 -> F50 -> F51 -> F48 -> F44 -> F24 -> F9
// 0x142821c10  493b5040                       cmpq	0x40(%r8), %rdx
// 0x142821c14  7376                           jae	0x142821c8c <.text+0x2820c8c>
// Frames: F52 -> F53 -> F54 -> F49 -> F45 -> F46 -> F50 -> F51 -> F48 -> F44 -> F24 -> F9
// 0x142821c16  488b14d1                       movq	(%rcx,%rdx,8), %rdx
// 0x142821c1a  4885d2                         testq	%rdx, %rdx
// 0x142821c1d  746d                           je	0x142821c8c <.text+0x2820c8c>
// Frames: F55 -> F56 -> F57 -> F45 -> F46 -> F50 -> F51 -> F48 -> F44 -> F24 -> F9
// 0x142821c1f  48f7d2                         notq	%rdx
// Frames: F58 -> F59 -> F56 -> F57 -> F45 -> F46 -> F50 -> F51 -> F48 -> F44 -> F24 -> F9
// 0x142821c22  488d1452                       leaq	(%rdx,%rdx,2), %rdx
// 0x142821c26  48c1e204                       shlq	$0x4, %rdx
// Frames: F60 -> F61 -> F62 -> F57 -> F63 -> F51 -> F48 -> F44 -> F24 -> F9
// 0x142821c2a  488b541010                     movq	0x10(%rax,%rdx), %rdx
// 0x142821c2f  4889542430                     movq	%rdx, 0x30(%rsp)
// 0x142821c34  eb5f                           jmp	0x142821c95 <.text+0x2820c95>
// Frames: F64 -> F65 -> F24 -> F9
// 0x142821c36  4801c8                         addq	%rcx, %rax
// 0x142821c39  8b8894000000                   movl	0x94(%rax), %ecx
// 0x142821c3f  488b542460                     movq	0x60(%rsp), %rdx
// Frames: F29 -> F30 -> F66 -> F65 -> F24 -> F9
// 0x142821c44  4c8b8aa0010000                 movq	0x1a0(%rdx), %r9
// 0x142821c4b  4c8d14c9                       leaq	(%rcx,%rcx,8), %r10
// 0x142821c4f  488b4c2448                     movq	0x48(%rsp), %rcx
// Frames: F67 -> F24 -> F9
// 0x142821c54  4c8b8118010000                 movq	0x118(%rcx), %r8
// Frames: F45 -> F46 -> F47 -> F68 -> F69 -> F67 -> F24 -> F9
// 0x142821c5b  4b8b4cd118                     movq	0x18(%r9,%r10,8), %rcx
// 0x142821c60  4b8b54d138                     movq	0x38(%r9,%r10,8), %rdx
// Frames: F29 -> F30 -> F49 -> F45 -> F46 -> F50 -> F51 -> F70 -> F69 -> F67 -> F24 -> F9
// 0x142821c65  4f3b44d140                     cmpq	0x40(%r9,%r10,8), %r8
// 0x142821c6a  7364                           jae	0x142821cd0 <.text+0x2820cd0>
// Frames: F52 -> F53 -> F54 -> F49 -> F45 -> F46 -> F50 -> F51 -> F70 -> F69 -> F67 -> F24 -> F9
// 0x142821c6c  4e8b04c2                       movq	(%rdx,%r8,8), %r8
// 0x142821c70  4d85c0                         testq	%r8, %r8
// 0x142821c73  745b                           je	0x142821cd0 <.text+0x2820cd0>
// Frames: F55 -> F56 -> F57 -> F45 -> F46 -> F50 -> F51 -> F70 -> F69 -> F67 -> F24 -> F9
// 0x142821c75  49f7d0                         notq	%r8
// Frames: F58 -> F59 -> F56 -> F57 -> F45 -> F46 -> F50 -> F51 -> F70 -> F69 -> F67 -> F24 -> F9
// 0x142821c78  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x142821c7c  49c1e004                       shlq	$0x4, %r8
// Frames: F60 -> F61 -> F62 -> F57 -> F63 -> F51 -> F70 -> F69 -> F67 -> F24 -> F9
// 0x142821c80  4e8b440110                     movq	0x10(%rcx,%r8), %r8
// 0x142821c85  4c89442430                     movq	%r8, 0x30(%rsp)
// 0x142821c8a  eb4d                           jmp	0x142821cd9 <.text+0x2820cd9>
// 0x142821c8c  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x142821c95  488b542448                     movq	0x48(%rsp), %rdx
// 0x142821c9a  488b9210010000                 movq	0x110(%rdx), %rdx
// 0x142821ca1  488b0cd1                       movq	(%rcx,%rdx,8), %rcx
// 0x142821ca5  48f7d1                         notq	%rcx
// 0x142821ca8  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x142821cac  48c1e104                       shlq	$0x4, %rcx
// 0x142821cb0  488b540810                     movq	0x10(%rax,%rcx), %rdx
// 0x142821cb5  4889542450                     movq	%rdx, 0x50(%rsp)
// 0x142821cba  488b440828                     movq	0x28(%rax,%rcx), %rax
// 0x142821cbf  4889442468                     movq	%rax, 0x68(%rsp)
// 0x142821cc4  4983c504                       addq	$0x4, %r13
// 0x142821cc8  4531ff                         xorl	%r15d, %r15d
// Frames: F71 -> F24 -> F9
// 0x142821ccb  4489f8                         movl	%r15d, %eax
// 0x142821cce  eb5f                           jmp	0x142821d2f <.text+0x2820d2f>
// 0x142821cd0  48c744243000000000             movq	$0x0, 0x30(%rsp)
// Frames: F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F72 -> F73 -> F24 -> F9
// 0x142821cd9  4c8b442448                     movq	0x48(%rsp), %r8
// 0x142821cde  4d8b8010010000                 movq	0x110(%r8), %r8
// 0x142821ce5  4a8b14c2                       movq	(%rdx,%r8,8), %rdx
// 0x142821ce9  48f7d2                         notq	%rdx
// 0x142821cec  488d1452                       leaq	(%rdx,%rdx,2), %rdx
// 0x142821cf0  48c1e204                       shlq	$0x4, %rdx
// 0x142821cf4  4c8b441110                     movq	0x10(%rcx,%rdx), %r8
// 0x142821cf9  4c89442450                     movq	%r8, 0x50(%rsp)
// 0x142821cfe  488b4c1128                     movq	0x28(%rcx,%rdx), %rcx
// 0x142821d03  48894c2468                     movq	%rcx, 0x68(%rsp)
// 0x142821d08  4983c504                       addq	$0x4, %r13
// 0x142821d0c  488b4050                       movq	0x50(%rax), %rax
// 0x142821d10  4889842498000000               movq	%rax, 0x98(%rsp)
// 0x142821d18  4531ff                         xorl	%r15d, %r15d
// 0x142821d1b  31c0                           xorl	%eax, %eax
// Frames: F58 -> F74 -> F43 -> F24 -> F9
// 0x142821d1d  48c1e004                       shlq	$0x4, %rax
// 0x142821d21  488b8c2498000000               movq	0x98(%rsp), %rcx
// Frames: F75 -> F76 -> F24 -> F9
// 0x142821d29  8b440108                       movl	0x8(%rcx,%rax), %eax
// Frames: F77 -> F78 -> F79 -> F80 -> F81 -> F82 -> F83 -> F24 -> F9
// 0x142821d2d  f7d0                           notl	%eax
// Frames: F19 -> F84 -> F79 -> F80 -> F81 -> F82 -> F85 -> F24 -> F9
// 0x142821d2f  488d0c40                       leaq	(%rax,%rax,2), %rcx
// 0x142821d33  488b542450                     movq	0x50(%rsp), %rdx
// 0x142821d38  488d3cca                       leaq	(%rdx,%rcx,8), %rdi
// 0x142821d3c  488b4c2468                     movq	0x68(%rsp), %rcx
// Frames: F19 -> F84 -> F86 -> F80 -> F81 -> F82 -> F85 -> F24 -> F9
// 0x142821d41  4c8d2481                       leaq	(%rcx,%rax,4), %r12
// Frames: F19 -> F84 -> F87 -> F80 -> F88 -> F82 -> F85 -> F24 -> F9
// 0x142821d45  486bd858                       imulq	$0x58, %rax, %rbx
// 0x142821d49  48035c2430                     addq	0x30(%rsp), %rbx
// 0x142821d4e  41ffc7                         incl	%r15d
// Frames: F89
// 0x142821d51  4889d9                         movq	%rbx, %rcx
// 0x142821d54  e837713cfe                     callq	0x140be8e90 <.text+0xbe7e90>
// 0x142821d59  84c0                           testb	%al, %al
// 0x142821d5b  0f85effdffff                   jne	0x142821b50 <.text+0x2820b50>
// Frames: F90
// 0x142821d61  488d4c2474                     leaq	0x74(%rsp), %rcx
// 0x142821d66  4889da                         movq	%rbx, %rdx
// 0x142821d69  e862713cfe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// Frames: F91
// 0x142821d6e  f3440f106c2478                 movss	0x78(%rsp), %xmm13
// 0x142821d75  f3410f104628                   movss	0x28(%r14), %xmm0
// Frames: F92 -> F91
// 0x142821d7b  f30f1017                       movss	(%rdi), %xmm2
// Frames: F93 -> F91
// 0x142821d7f  f30f105f08                     movss	0x8(%rdi), %xmm3
// 0x142821d84  0f28ca                         movaps	%xmm2, %xmm1
// 0x142821d87  f30f5ec8                       divss	%xmm0, %xmm1
// 0x142821d8b  410f28e5                       movaps	%xmm13, %xmm4
// 0x142821d8f  f30f5ce2                       subss	%xmm2, %xmm4
// Frames: F94 -> F91
// 0x142821d93  0f28ef                         movaps	%xmm7, %xmm5
// 0x142821d96  f30fc2e905                     cmpnltss	%xmm1, %xmm5
// 0x142821d9b  440f28c5                       movaps	%xmm5, %xmm8
// 0x142821d9f  440f54c3                       andps	%xmm3, %xmm8
// 0x142821da3  410f55ed                       andnps	%xmm13, %xmm5
// 0x142821da7  410f56e8                       orps	%xmm8, %xmm5
// 0x142821dab  f30fc2d605                     cmpnltss	%xmm6, %xmm2
// 0x142821db0  440f28c2                       movaps	%xmm2, %xmm8
// 0x142821db4  440f55c5                       andnps	%xmm5, %xmm8
// 0x142821db8  0f54d3                         andps	%xmm3, %xmm2
// 0x142821dbb  410f56d0                       orps	%xmm8, %xmm2
// 0x142821dbf  0f28eb                         movaps	%xmm3, %xmm5
// 0x142821dc2  f30fc2ec06                     cmpnless	%xmm4, %xmm5
// 0x142821dc7  0f28e5                         movaps	%xmm5, %xmm4
// 0x142821dca  0f55e2                         andnps	%xmm2, %xmm4
// 0x142821dcd  0f54eb                         andps	%xmm3, %xmm5
// 0x142821dd0  0f56ec                         orps	%xmm4, %xmm5
// Frames: F95 -> F91
// 0x142821dd3  440f28f8                       movaps	%xmm0, %xmm15
// 0x142821dd7  f3440f593d3866d600             mulss	0xd66638(%rip), %xmm15  # 0x143588418
// 0x142821de0  f3440f587f0c                   addss	0xc(%rdi), %xmm15
// Frames: F96 -> F91
// 0x142821de6  440f28c0                       movaps	%xmm0, %xmm8
// 0x142821dea  f3450f59c7                     mulss	%xmm15, %xmm8
// 0x142821def  f3440f58c5                     addss	%xmm5, %xmm8
// Frames: F97 -> F91
// 0x142821df4  450f2ee8                       ucomiss	%xmm8, %xmm13
// 0x142821df8  7243                           jb	0x142821e3d <.text+0x2820e3d>
// Frames: F98 -> F99 -> F91
// 0x142821dfa  450f2ef7                       ucomiss	%xmm15, %xmm14
// 0x142821dfe  0f8230020000                   jb	0x142822034 <.text+0x2821034>
// Frames: F99 -> F91
// 0x142821e04  f30f590d1066d600               mulss	0xd66610(%rip), %xmm1   # 0x14358841c
// Frames: F100 -> F101 -> F91
// 0x142821e0c  410f28d5                       movaps	%xmm13, %xmm2
// 0x142821e10  f3410f5fd0                     maxss	%xmm8, %xmm2
// 0x142821e15  f3450fc2c003                   cmpunordss	%xmm8, %xmm8
// 0x142821e1b  410f28d8                       movaps	%xmm8, %xmm3
// 0x142821e1f  410f54dd                       andps	%xmm13, %xmm3
// 0x142821e23  440f55c2                       andnps	%xmm2, %xmm8
// 0x142821e27  440f56c3                       orps	%xmm3, %xmm8
// Frames: F102 -> F99 -> F91
// 0x142821e2b  f3440f5ff9                     maxss	%xmm1, %xmm15
// Frames: F103 -> F99 -> F91
// 0x142821e30  410f28ce                       movaps	%xmm14, %xmm1
// 0x142821e34  f3410f5dcf                     minss	%xmm15, %xmm1
// 0x142821e39  440f28f9                       movaps	%xmm1, %xmm15
// Frames: F104 -> F91
// 0x142821e3d  f3410f59c1                     mulss	%xmm9, %xmm0
// Frames: F105 -> F104 -> F91
// 0x142821e42  e8999f5f00                     callq	0x142e1bde0 <.text+0x2e1ade0>
// Frames: F104 -> F91
// 0x142821e47  410f28ca                       movaps	%xmm10, %xmm1
// 0x142821e4b  f30f5cc8                       subss	%xmm0, %xmm1
// Frames: F106 -> F107 -> F108 -> F91
// 0x142821e4f  410f28c2                       movaps	%xmm10, %xmm0
// 0x142821e53  f30f5cc1                       subss	%xmm1, %xmm0
// Frames: F109 -> F107 -> F108 -> F91
// 0x142821e57  f3410f59c0                     mulss	%xmm8, %xmm0
// 0x142821e5c  f3410f59cd                     mulss	%xmm13, %xmm1
// Frames: F110 -> F107 -> F108 -> F91
// 0x142821e61  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F111
// 0x142821e65  f30f114f08                     movss	%xmm1, 0x8(%rdi)
// 0x142821e6a  8b442438                       movl	0x38(%rsp), %eax
// Frames: F112 -> F113 -> F114
// 0x142821e6e  41890424                       movl	%eax, (%r12)
// Frames: F114
// 0x142821e72  f3440f117f0c                   movss	%xmm15, 0xc(%rdi)
// 0x142821e78  f3410f5ccd                     subss	%xmm13, %xmm1
// Frames: F115 -> F116
// 0x142821e7d  410f54cb                       andps	%xmm11, %xmm1
// Frames: F117
// 0x142821e81  807f1401                       cmpb	$0x1, 0x14(%rdi)
// 0x142821e85  752f                           jne	0x142821eb6 <.text+0x2820eb6>
// Frames: F118
// 0x142821e87  410f2ecc                       ucomiss	%xmm12, %xmm1
// 0x142821e8b  763f                           jbe	0x142821ecc <.text+0x2820ecc>
// Frames: F119
// 0x142821e8d  b001                           movb	$0x1, %al
// 0x142821e8f  410f2ef7                       ucomiss	%xmm15, %xmm6
// 0x142821e93  0f868b010000                   jbe	0x142822024 <.text+0x2821024>
// Frames: F120
// 0x142821e99  f3410f104628                   movss	0x28(%r14), %xmm0
// 0x142821e9f  8b4c2438                       movl	0x38(%rsp), %ecx
// Frames: F112 -> F113 -> F120
// 0x142821ea3  41890c24                       movl	%ecx, (%r12)
// Frames: F120
// 0x142821ea7  f30f584710                     addss	0x10(%rdi), %xmm0
// 0x142821eac  f30f114710                     movss	%xmm0, 0x10(%rdi)
// Frames: F119
// 0x142821eb1  e96e010000                     jmp	0x142822024 <.text+0x2821024>
// Frames: F121
// 0x142821eb6  440f2ee1                       ucomiss	%xmm1, %xmm12
// 0x142821eba  0f86d8000000                   jbe	0x142821f98 <.text+0x2820f98>
// Frames: F122
// 0x142821ec0  c7471000000000                 movl	$0x0, 0x10(%rdi)
// 0x142821ec7  e9c5000000                     jmp	0x142821f91 <.text+0x2820f91>
// Frames: F123
// 0x142821ecc  f3440f104710                   movss	0x10(%rdi), %xmm8
// 0x142821ed2  440f2e0506137000               ucomiss	0x701306(%rip), %xmm8   # 0x142f231e0
// 0x142821eda  0f86b1000000                   jbe	0x142821f91 <.text+0x2820f91>
// Frames: F124
// 0x142821ee0  f3440f5805073f7d00             addss	0x7d3f07(%rip), %xmm8   # 0x142ff5df0
// 0x142821ee9  f3440f5e056ec3cd00             divss	0xcdc36e(%rip), %xmm8   # 0x1434fe260
// Frames: F125
// 0x142821ef2  8b44247c                       movl	0x7c(%rsp), %eax
// 0x142821ef6  488d8c24af000000               leaq	0xaf(%rsp), %rcx
// 0x142821efe  894101                         movl	%eax, 0x1(%rcx)
// 0x142821f01  488b442474                     movq	0x74(%rsp), %rax
// 0x142821f06  488941f9                       movq	%rax, -0x7(%rcx)
// 0x142821f0a  8b44243c                       movl	0x3c(%rsp), %eax
// 0x142821f0e  488b8c2490000000               movq	0x90(%rsp), %rcx
// Frames: F126 -> F127 -> F128 -> F125
// 0x142821f16  8901                           movl	%eax, (%rcx)
// 0x142821f18  488b442440                     movq	0x40(%rsp), %rax
// Frames: F129 -> F130 -> F131 -> F132 -> F128 -> F125
// 0x142821f1d  488b5830                       movq	0x30(%rax), %rbx
// Frames: F133 -> F132 -> F128 -> F125
// 0x142821f21  488b5040                       movq	0x40(%rax), %rdx
// Frames: F134 -> F130 -> F131 -> F132 -> F128 -> F125
// 0x142821f25  483b5820                       cmpq	0x20(%rax), %rbx
// 0x142821f29  7517                           jne	0x142821f42 <.text+0x2820f42>
// Frames: F135 -> F130 -> F131 -> F132 -> F128 -> F125
// 0x142821f2b  488b8c2488000000               movq	0x88(%rsp), %rcx
// 0x142821f33  4889542458                     movq	%rdx, 0x58(%rsp)
// 0x142821f38  e8e3d35400                     callq	0x142d6f320 <.text+0x2d6e320>
// 0x142821f3d  488b542458                     movq	0x58(%rsp), %rdx
// Frames: F10 -> F11 -> F12 -> F136 -> F137 -> F130 -> F131 -> F132 -> F128 -> F125
// 0x142821f42  0f57c0                         xorps	%xmm0, %xmm0
// 0x142821f45  f3410f5fc0                     maxss	%xmm8, %xmm0
// 0x142821f4a  410f28ca                       movaps	%xmm10, %xmm1
// 0x142821f4e  f30f5dc8                       minss	%xmm0, %xmm1
// 0x142821f52  4c8b442440                     movq	0x40(%rsp), %r8
// 0x142821f57  498b4028                       movq	0x28(%r8), %rax
// Frames: F138 -> F137 -> F130 -> F131 -> F132 -> F128 -> F125
// 0x142821f5b  488d0c9b                       leaq	(%rbx,%rbx,4), %rcx
// Frames: F139 -> F140 -> F130 -> F131 -> F132 -> F128 -> F125
// 0x142821f5f  488914c8                       movq	%rdx, (%rax,%rcx,8)
// 0x142821f63  c644c80804                     movb	$0x4, 0x8(%rax,%rcx,8)
// 0x142821f68  f30f114cc80c                   movss	%xmm1, 0xc(%rax,%rcx,8)
// 0x142821f6e  8b9424b0000000                 movl	0xb0(%rsp), %edx
// 0x142821f75  8954c820                       movl	%edx, 0x20(%rax,%rcx,8)
// 0x142821f79  0f108424a0000000               movups	0xa0(%rsp), %xmm0
// 0x142821f81  0f1144c810                     movups	%xmm0, 0x10(%rax,%rcx,8)
// Frames: F141 -> F130 -> F131 -> F132 -> F128 -> F125
// 0x142821f86  48ffc3                         incq	%rbx
// 0x142821f89  49895830                       movq	%rbx, 0x30(%r8)
// Frames: F142 -> F132 -> F128 -> F125
// 0x142821f8d  49ff4040                       incq	0x40(%r8)
// 0x142821f91  31c0                           xorl	%eax, %eax
// 0x142821f93  e98c000000                     jmp	0x142822024 <.text+0x2821024>
// Frames: F143
// 0x142821f98  4c89742458                     movq	%r14, 0x58(%rsp)
// 0x142821f9d  8b44247c                       movl	0x7c(%rsp), %eax
// 0x142821fa1  488d8c24af000000               leaq	0xaf(%rsp), %rcx
// 0x142821fa9  894108                         movl	%eax, 0x8(%rcx)
// 0x142821fac  488b442474                     movq	0x74(%rsp), %rax
// 0x142821fb1  488901                         movq	%rax, (%rcx)
// 0x142821fb4  8b44243c                       movl	0x3c(%rsp), %eax
// 0x142821fb8  488b8c2490000000               movq	0x90(%rsp), %rcx
// Frames: F126 -> F127 -> F128 -> F143
// 0x142821fc0  8901                           movl	%eax, (%rcx)
// 0x142821fc2  488b442440                     movq	0x40(%rsp), %rax
// Frames: F129 -> F130 -> F131 -> F132 -> F128 -> F143
// 0x142821fc7  488b5830                       movq	0x30(%rax), %rbx
// Frames: F133 -> F132 -> F128 -> F143
// 0x142821fcb  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F134 -> F130 -> F131 -> F132 -> F128 -> F143
// 0x142821fcf  483b5820                       cmpq	0x20(%rax), %rbx
// 0x142821fd3  750d                           jne	0x142821fe2 <.text+0x2820fe2>
// Frames: F135 -> F130 -> F131 -> F132 -> F128 -> F143
// 0x142821fd5  488b8c2488000000               movq	0x88(%rsp), %rcx
// 0x142821fdd  e83ed35400                     callq	0x142d6f320 <.text+0x2d6e320>
// Frames: F10 -> F11 -> F12 -> F136 -> F137 -> F130 -> F131 -> F132 -> F128 -> F143
// 0x142821fe2  488b542440                     movq	0x40(%rsp), %rdx
// 0x142821fe7  488b4228                       movq	0x28(%rdx), %rax
// Frames: F138 -> F137 -> F130 -> F131 -> F132 -> F128 -> F143
// 0x142821feb  488d0c9b                       leaq	(%rbx,%rbx,4), %rcx
// Frames: F139 -> F140 -> F130 -> F131 -> F132 -> F128 -> F143
// 0x142821fef  4c8934c8                       movq	%r14, (%rax,%rcx,8)
// 0x142821ff3  c644c80805                     movb	$0x5, 0x8(%rax,%rcx,8)
// 0x142821ff8  0f108424a0000000               movups	0xa0(%rsp), %xmm0
// 0x142822000  0f108c24ab000000               movups	0xab(%rsp), %xmm1
// 0x142822008  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x14282200d  0f1144c809                     movups	%xmm0, 0x9(%rax,%rcx,8)
// Frames: F141 -> F130 -> F131 -> F132 -> F128 -> F143
// 0x142822012  48ffc3                         incq	%rbx
// 0x142822015  48895a30                       movq	%rbx, 0x30(%rdx)
// Frames: F142 -> F132 -> F128 -> F143
// 0x142822019  48ff4240                       incq	0x40(%rdx)
// 0x14282201d  b001                           movb	$0x1, %al
// 0x14282201f  4c8b742458                     movq	0x58(%rsp), %r14
// Frames: F112 -> F113 -> F117
// 0x142822024  8b4c2438                       movl	0x38(%rsp), %ecx
// 0x142822028  41890c24                       movl	%ecx, (%r12)
// Frames: F117
// 0x14282202c  884714                         movb	%al, 0x14(%rdi)
// Frames: F9
// 0x14282202f  e91cfbffff                     jmp	0x142821b50 <.text+0x2820b50>
// Frames: F144 -> F99 -> F91
// 0x142822034  f3440f11bc2480000000           movss	%xmm15, 0x80(%rsp)
// 0x14282203e  c784248400000000007041         movl	$0x41700000, 0x84(%rsp) # imm = 0x41700000
// 0x142822049  488d842480000000               leaq	0x80(%rsp), %rax
// Frames: F145 -> F146 -> F147 -> F99 -> F91
// 0x142822051  48898424a0000000               movq	%rax, 0xa0(%rsp)
// 0x142822059  488d05e05074fe                 leaq	-0x18baf20(%rip), %rax  # 0x140f67140 <.text+0xf66140>
// 0x142822060  48898424a8000000               movq	%rax, 0xa8(%rsp)
// 0x142822068  488d8c2484000000               leaq	0x84(%rsp), %rcx
// 0x142822070  48898c24b0000000               movq	%rcx, 0xb0(%rsp)
// 0x142822078  48898424b8000000               movq	%rax, 0xb8(%rsp)
// 0x142822080  488d0d9d63d600                 leaq	0xd6639d(%rip), %rcx    # 0x143588424
// 0x142822087  4c8d053264d600                 leaq	0xd66432(%rip), %r8     # 0x1435884c0
// 0x14282208e  488d9424a0000000               leaq	0xa0(%rsp), %rdx
// 0x142822096  e805f85800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x14282209b  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:14:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F2: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:21:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F3: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:85:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:528:0 bevy_ecs::system::query::Query::new
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:643:0 bevy_ecs::system::query::Query::copy_unsafe
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:625:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:607:0 bevy_ecs::system::query::Query::reborrow
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:705:0 bevy_ecs::system::query::Query::iter_mut
// F9: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:25:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F10: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F11: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2421:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F17: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:53:0 bevy_ecs::query::iter::QueryIter::new
// F18: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:2510:0 bevy_ecs::system::query::Query::reborrow
// F19: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F20: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 alloc::vec::impl$9::deref
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2422:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F23: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2517:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:915:0 bevy_ecs::query::iter::impl$1::next
// F25: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2520:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:180:0 core::slice::iter::impl$171::next
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2521:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F29: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2522:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2997:0 alloc::vec::Vec::len
// F34: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3022:0 alloc::vec::Vec::is_empty
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:647:0 bevy_ecs::storage::table::Table::is_empty
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2523:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2561:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2562:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:865:0 bevy_ecs::archetype::Archetypes::get
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2563:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:656:0 bevy_ecs::archetype::Archetype::is_empty
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2564:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2593:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2529:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1932:0 bevy_ecs::query::fetch::impl$48::set_table
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$2::set_table
// F49: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:381:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1552:0 bevy_ecs::query::fetch::impl$36::set_table
// F52: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F53: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F54: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::slice::impl$0::get
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F57: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F58: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F59: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:298:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:479:0 bevy_ecs::archetype::Archetype::table_id
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2567:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::archetype::Archetype::table_id
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2571:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1921:0 bevy_ecs::query::fetch::impl$48::set_archetype
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1539:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2540:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:485:0 bevy_ecs::archetype::Archetype::entities
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2584:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F74: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 core::slice::impl$0::get_unchecked
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:347:0 bevy_ecs::archetype::ArchetypeEntity::table_row
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2615:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2000:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1993:0 bevy_ecs::query::fetch::impl$49::fetch
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2529:0 bevy_ecs::query::fetch::impl$89::fetch
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2611:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ptr::ThinSlicePtr::get
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:0:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2004:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1609:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1604:0 bevy_ecs::query::fetch::impl$49::fetch
// F89: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:26:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F90: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:30:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F91: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:33:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F92: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:89:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F93: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:90:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F94: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:96:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F95: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:109:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F96: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:110:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F97: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:113:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F98: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1505:0 core::f32::impl$0::clamp
// F99: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:117:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F100: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:992:0 core::f32::impl$0::max
// F101: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:115:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F102: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1512:0 core::f32::impl$0::clamp
// F103: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1515:0 core::f32::impl$0::clamp
// F104: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:130:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F105: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:412:0 std::f32::impl$0::exp
// F106: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:218:0 core::ops::arith::impl$77::sub
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F108: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:132:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F109: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:352:0 core::ops::arith::impl$141::mul
// F110: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F111: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:35:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F114: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:36:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F115: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1570:0 core::f32::impl$0::abs
// F116: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:0:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F117: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:38:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F118: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:52:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F119: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:53:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F120: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:54:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F121: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:40:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F122: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:41:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F123: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:62:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F124: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:64:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F125: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:68:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F127: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:70:0 bevy_ecs::message::message_writer::MessageWriter::write
// F129: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1025:0 alloc::vec::Vec::push_mut
// F130: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:992:0 alloc::vec::Vec::push
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:143:0 bevy_ecs::message::messages::Messages::write_with_caller
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::change_detection::impl$38::deref_mut
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:131:0 bevy_ecs::message::messages::Messages::write_with_caller
// F134: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1028:0 alloc::vec::Vec::push_mut
// F135: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1029:0 alloc::vec::Vec::push_mut
// F136: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2000:0 alloc::vec::Vec::as_mut_ptr
// F137: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1032:0 alloc::vec::Vec::push_mut
// F138: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 alloc::vec::Vec::as_mut_ptr
// F139: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1917:0 core::ptr::write
// F140: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:0 alloc::vec::Vec::push_mut
// F141: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1034:0 alloc::vec::Vec::push_mut
// F142: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:144:0 bevy_ecs::message::messages::Messages::write_with_caller
// F143: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:44:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h98f36bce0778487f
// F144: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:185:0 core::f32::impl$0::clamp
// F145: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:172:0 core::f32::impl$0::clamp::do_panic::runtime
// F146: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs:2447:0 core::f32::impl$0::clamp::do_panic
// F147: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:177:0 core::f32::impl$0::clamp
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/simulate_roof_tile_for_audio.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 8fca6a7a6f5682e98d99967242dfb5857396fa85bcfa9faca340e43bdd30a4ca
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: simulate_roof_tile_for_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3776
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3776,"record_offset":268,"rva":42080688,"va":5410789808}
// 编译实例 RVAs: 0x28219b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: simulate_roof_tile_for_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5559267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410789808,"raw_hex":"e8e3d35400","size":5,"soft_fail":false,"target_va":5416350496,"va":5410791224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: simulate_roof_tile_for_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6266777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410789808,"raw_hex":"e8999f5f00","size":5,"soft_fail":false,"target_va":5417057760,"va":5410790978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5830661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410789808,"raw_hex":"e805f85800","size":5,"soft_fail":false,"target_va":5416622240,"va":5410791574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29593289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410789808,"raw_hex":"e837713cfe","size":5,"soft_fail":false,"target_va":5381197456,"va":5410790740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29593246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410789808,"raw_hex":"e862713cfe","size":5,"soft_fail":false,"target_va":5381197520,"va":5410790761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3776,"record_offset":4012,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3776,"record_offset":4152,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3776,"record_offset":884,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3776,"record_offset":444,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3776,"record_offset":4068,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_direct] system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/simulate_roof_tile_for_audio.rs
//     evidence={"module":3776,"record_offset":3628,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":133019,"source_line":88,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3776,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3776,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3776,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3776,"record_offset":4344,"parent_record_offset":4300,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3776,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3776,"record_offset":668,"parent_record_offset":636,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3776,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":268,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3776,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3776,"record_offset":4272,"parent_record_offset":4236,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3776,"record_offset":4300,"parent_record_offset":4272,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3776,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3776,"record_offset":2828,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3776,"record_offset":1380,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3776,"record_offset":2072,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3776,"record_offset":3080,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3776,"record_offset":1296,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3776,"record_offset":4208,"parent_record_offset":4176,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3776,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3776,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":268,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3776,"record_offset":4236,"parent_record_offset":4176,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":2300,"parent_record_offset":2164,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":1572,"parent_record_offset":1464,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":3524,"parent_record_offset":3500,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":268,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":268,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":268,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":268,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3776,"record_offset":3108,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3776,"record_offset":584,"parent_record_offset":468,"ancestor_record_offset":268,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3776,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3776,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":268,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3776,"record_offset":2164,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3776,"record_offset":1464,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":3776,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3776,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3776,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3776,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3776,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3776,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3776,"record_offset":1148,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3776,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3776,"record_offset":1064,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3776,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3776,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3776,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":268,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3776,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":268,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3776,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":268,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3776,"record_offset":500,"parent_record_offset":468,"ancestor_record_offset":268,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3776,"record_offset":3380,"parent_record_offset":3352,"ancestor_record_offset":268,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3776,"record_offset":3696,"parent_record_offset":3628,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":3776,"record_offset":3744,"parent_record_offset":3696,"ancestor_record_offset":268,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3776,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":268,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3776,"record_offset":3812,"parent_record_offset":3628,"ancestor_record_offset":268,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3776,"record_offset":3976,"parent_record_offset":3868,"ancestor_record_offset":268,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3776,"record_offset":3920,"parent_record_offset":3868,"ancestor_record_offset":268,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3776,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3776,"record_offset":1756,"parent_record_offset":1696,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3776,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3776,"record_offset":1820,"parent_record_offset":1648,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3776,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3776,"record_offset":1028,"parent_record_offset":1000,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3776,"record_offset":4464,"parent_record_offset":4300,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3776,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3776,"record_offset":3024,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3776,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3776,"record_offset":1900,"parent_record_offset":1868,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3776,"record_offset":808,"parent_record_offset":636,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3776,"record_offset":1000,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3776,"record_offset":3868,"parent_record_offset":3628,"ancestor_record_offset":268,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3776,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3776,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3776,"record_offset":3840,"parent_record_offset":3628,"ancestor_record_offset":268,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2079B
// 首条位置: {"module":3776,"inlinee":133019}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3776,"record_offset":3696,"parent_record_offset":3628,"ancestor_record_offset":3628,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3776,"record_offset":3812,"parent_record_offset":3628,"ancestor_record_offset":3628,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3776,"record_offset":3868,"parent_record_offset":3628,"ancestor_record_offset":3628,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3776,"record_offset":3840,"parent_record_offset":3628,"ancestor_record_offset":3628,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":3776,"record_offset":3744,"parent_record_offset":3696,"ancestor_record_offset":3628,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3776,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":3628,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3776,"record_offset":3976,"parent_record_offset":3868,"ancestor_record_offset":3628,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3776,"record_offset":3920,"parent_record_offset":3868,"ancestor_record_offset":3628,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3776,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":3628,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03776.jsonl.gz"}
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
// 完整结构及原始类型记录: simulate_roof_tile_for_audio.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: simulate_roof_tile_for_audio.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: simulate_roof_tile_for_audio.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// 信息 ID: 053fe138191f5f8a7c9fe707
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio; module=3776, inline_record=3628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// 信息 ID: c8b2309029f1c9de0acd4a9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ea9a227b3d2821787617065a: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)> > @ 0x1f297e4 -> 0x28219b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)> @ 0x1f30761 -> 0x28219b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
