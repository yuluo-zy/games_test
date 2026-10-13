// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: roof_animation.rs.assembly.json.gz

// FUNCTION system_roof::visual::roof_animation::roof_animation
// Candidate interfaces: 1e44a27472b886e31da12704
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3460:260 -> rva_21a2da0_len_473
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21a2da0_len_473: RVA 0x21a2da0, 1139 PDB bytes; 251 instructions
// Evidence: _assembly/roof-v1/B/pack-001/rva_21a2da0_len_473.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3460:260: system_roof::visual::roof_animation::roof_animation
// Frames: F1
// 0x1421a2da0  4157                           pushq	%r15
// 0x1421a2da2  4156                           pushq	%r14
// 0x1421a2da4  4155                           pushq	%r13
// 0x1421a2da6  4154                           pushq	%r12
// 0x1421a2da8  56                             pushq	%rsi
// 0x1421a2da9  57                             pushq	%rdi
// 0x1421a2daa  55                             pushq	%rbp
// 0x1421a2dab  53                             pushq	%rbx
// 0x1421a2dac  4881ece8000000                 subq	$0xe8, %rsp
// 0x1421a2db3  440f29ac24d0000000             movaps	%xmm13, 0xd0(%rsp)
// 0x1421a2dbc  440f29a424c0000000             movaps	%xmm12, 0xc0(%rsp)
// 0x1421a2dc5  440f299c24b0000000             movaps	%xmm11, 0xb0(%rsp)
// 0x1421a2dce  440f299424a0000000             movaps	%xmm10, 0xa0(%rsp)
// 0x1421a2dd7  440f298c2490000000             movaps	%xmm9, 0x90(%rsp)
// 0x1421a2de0  440f29842480000000             movaps	%xmm8, 0x80(%rsp)
// 0x1421a2de9  0f297c2470                     movaps	%xmm7, 0x70(%rsp)
// 0x1421a2dee  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F2 -> F3 -> F4 -> F5 -> F6
// 0x1421a2df3  4c8b01                         movq	(%rcx), %r8
// 0x1421a2df6  488b4108                       movq	0x8(%rcx), %rax
// 0x1421a2dfa  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1421a2dff  8b4114                         movl	0x14(%rcx), %eax
// 0x1421a2e02  8944243c                       movl	%eax, 0x3c(%rsp)
// Frames: F7 -> F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F5 -> F6
// 0x1421a2e06  4d8bb0f8000000                 movq	0xf8(%r8), %r14
// Frames: F11 -> F12 -> F13 -> F14 -> F15 -> F5 -> F6
// 0x1421a2e0d  498b8000010000                 movq	0x100(%r8), %rax
// Frames: F16 -> F17 -> F18 -> F13 -> F14 -> F15 -> F5 -> F6
// 0x1421a2e14  4d8d3c86                       leaq	(%r14,%rax,4), %r15
// 0x1421a2e18  4c89442430                     movq	%r8, 0x30(%rsp)
// Frames: F19 -> F14 -> F15 -> F5 -> F6
// 0x1421a2e1d  410fb68008010000               movzbl	0x108(%r8), %eax
// 0x1421a2e25  88442427                       movb	%al, 0x27(%rsp)
// 0x1421a2e29  488b02                         movq	(%rdx), %rax
// 0x1421a2e2c  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1421a2e31  b808000000                     movl	$0x8, %eax
// 0x1421a2e36  4889442450                     movq	%rax, 0x50(%rsp)
// 0x1421a2e3b  4531e4                         xorl	%r12d, %r12d
// 0x1421a2e3e  f30f1035229e8e00               movss	0x8e9e22(%rip), %xmm6   # 0x142a8cc68
// 0x1421a2e46  f30f103dc23e8700               movss	0x873ec2(%rip), %xmm7   # 0x142a16d10
// 0x1421a2e4e  450f57c0                       xorps	%xmm8, %xmm8
// 0x1421a2e52  f3440f100da92a7800             movss	0x782aa9(%rip), %xmm9   # 0x142925904
// 0x1421a2e5b  f3440f10158cb87800             movss	0x78b88c(%rip), %xmm10  # 0x14292e6f0
// 0x1421a2e64  f3440f101da738e500             movss	0xe538a7(%rip), %xmm11  # 0x142ff6714
// 0x1421a2e6d  f3440f1025a238e500             movss	0xe538a2(%rip), %xmm12  # 0x142ff6718
// 0x1421a2e76  f3440f102d591a8f00             movss	0x8f1a59(%rip), %xmm13  # 0x142a948d8
// 0x1421a2e7f  31ed                           xorl	%ebp, %ebp
// 0x1421a2e81  48c744242800000000             movq	$0x0, 0x28(%rsp)
// 0x1421a2e8a  4531ed                         xorl	%r13d, %r13d
// 0x1421a2e8d  31f6                           xorl	%esi, %esi
// Frames: F20 -> F21 -> F6
// 0x1421a2e8f  807c242700                     cmpb	$0x0, 0x27(%rsp)
// 0x1421a2e94  744a                           je	0x1421a2ee0 <.text+0x21a1ee0>
// Frames: F22 -> F21 -> F6
// 0x1421a2e96  4c39ee                         cmpq	%r13, %rsi
// 0x1421a2e99  0f85c3010000                   jne	0x1421a3062 <.text+0x21a2062>
// Frames: F23 -> F24 -> F25 -> F21 -> F6
// 0x1421a2e9f  4d39fe                         cmpq	%r15, %r14
// Frames: F26 -> F25 -> F21 -> F6
// 0x1421a2ea2  0f8417030000                   je	0x1421a31bf <.text+0x21a21bf>
// 0x1421a2ea8  488b442440                     movq	0x40(%rsp), %rax
// 0x1421a2ead  488b80a8010000                 movq	0x1a8(%rax), %rax
// 0x1421a2eb4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F25 -> F21 -> F6
// 0x1421a2ec0  418b0e                         movl	(%r14), %ecx
// Frames: F27 -> F28 -> F29 -> F30 -> F21 -> F6
// 0x1421a2ec3  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F31 -> F32 -> F33 -> F34 -> F21 -> F6
// 0x1421a2ec7  4c8b6cc810                     movq	0x10(%rax,%rcx,8), %r13
// Frames: F34 -> F21 -> F6
// 0x1421a2ecc  4d85ed                         testq	%r13, %r13
// 0x1421a2ecf  7552                           jne	0x1421a2f23 <.text+0x21a1f23>
// Frames: F23 -> F24 -> F25 -> F21 -> F6
// 0x1421a2ed1  4983c604                       addq	$0x4, %r14
// 0x1421a2ed5  4d39fe                         cmpq	%r15, %r14
// 0x1421a2ed8  75e6                           jne	0x1421a2ec0 <.text+0x21a1ec0>
// 0x1421a2eda  e9e0020000                     jmp	0x1421a31bf <.text+0x21a21bf>
// 0x1421a2edf  90                             nop
// Frames: F35 -> F21 -> F6
// 0x1421a2ee0  4c39ee                         cmpq	%r13, %rsi
// 0x1421a2ee3  0f85ec010000                   jne	0x1421a30d5 <.text+0x21a20d5>
// Frames: F23 -> F24 -> F36 -> F21 -> F6
// 0x1421a2ee9  4d39fe                         cmpq	%r15, %r14
// Frames: F26 -> F36 -> F21 -> F6
// 0x1421a2eec  0f84cd020000                   je	0x1421a31bf <.text+0x21a21bf>
// 0x1421a2ef2  488b442440                     movq	0x40(%rsp), %rax
// 0x1421a2ef7  488b8000010000                 movq	0x100(%rax), %rax
// 0x1421a2efe  6690                           nop
// Frames: F36 -> F21 -> F6
// 0x1421a2f00  418b0e                         movl	(%r14), %ecx
// Frames: F27 -> F28 -> F37 -> F38 -> F21 -> F6
// 0x1421a2f03  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x1421a2f07  48c1e105                       shlq	$0x5, %rcx
// Frames: F31 -> F32 -> F39 -> F40 -> F21 -> F6
// 0x1421a2f0b  4c8b6c0858                     movq	0x58(%rax,%rcx), %r13
// Frames: F40 -> F21 -> F6
// 0x1421a2f10  4d85ed                         testq	%r13, %r13
// 0x1421a2f13  7572                           jne	0x1421a2f87 <.text+0x21a1f87>
// Frames: F23 -> F24 -> F36 -> F21 -> F6
// 0x1421a2f15  4983c604                       addq	$0x4, %r14
// 0x1421a2f19  4d39fe                         cmpq	%r15, %r14
// Frames: F26 -> F36 -> F21 -> F6
// 0x1421a2f1c  75e2                           jne	0x1421a2f00 <.text+0x21a1f00>
// 0x1421a2f1e  e99c020000                     jmp	0x1421a31bf <.text+0x21a21bf>
// Frames: F41 -> F42 -> F21 -> F6
// 0x1421a2f23  488d0cc8                       leaq	(%rax,%rcx,8), %rcx
// 0x1421a2f27  488b442430                     movq	0x30(%rsp), %rax
// 0x1421a2f2c  4c8b8010010000                 movq	0x110(%rax), %r8
// 0x1421a2f33  488b4138                       movq	0x38(%rcx), %rax
// Frames: F43 -> F44 -> F45 -> F46 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f37  488b5140                       movq	0x40(%rcx), %rdx
// Frames: F27 -> F28 -> F47 -> F43 -> F44 -> F45 -> F46 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f3b  4c39c2                         cmpq	%r8, %rdx
// 0x1421a2f3e  0f86c3000000                   jbe	0x1421a3007 <.text+0x21a2007>
// Frames: F48 -> F49 -> F50 -> F47 -> F43 -> F44 -> F45 -> F46 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f44  4e8b04c0                       movq	(%rax,%r8,8), %r8
// Frames: F43 -> F44 -> F51 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f48  488b4918                       movq	0x18(%rcx), %rcx
// Frames: F48 -> F49 -> F50 -> F47 -> F43 -> F44 -> F45 -> F46 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f4c  4d85c0                         testq	%r8, %r8
// Frames: F50 -> F47 -> F43 -> F44 -> F45 -> F46 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f4f  0f84b6000000                   je	0x1421a300b <.text+0x21a200b>
// Frames: F52 -> F53 -> F54 -> F43 -> F44 -> F45 -> F46 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f55  49f7d0                         notq	%r8
// Frames: F55 -> F56 -> F57 -> F54 -> F58 -> F46 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f58  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x1421a2f5c  49c1e004                       shlq	$0x4, %r8
// 0x1421a2f60  4e8b440110                     movq	0x10(%rcx,%r8), %r8
// 0x1421a2f65  4c89442428                     movq	%r8, 0x28(%rsp)
// 0x1421a2f6a  4c8b442430                     movq	0x30(%rsp), %r8
// Frames: F59 -> F60 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f6f  4d8b8020010000                 movq	0x120(%r8), %r8
// Frames: F27 -> F28 -> F61 -> F62 -> F63 -> F64 -> F65 -> F60 -> F41 -> F42 -> F21 -> F6
// 0x1421a2f76  4c39c2                         cmpq	%r8, %rdx
// 0x1421a2f79  0f87aa000000                   ja	0x1421a3029 <.text+0x21a2029>
// 0x1421a2f7f  4531e4                         xorl	%r12d, %r12d
// 0x1421a2f82  e9ab000000                     jmp	0x1421a3032 <.text+0x21a2032>
// Frames: F66 -> F67 -> F21 -> F6
// 0x1421a2f87  4801c8                         addq	%rcx, %rax
// 0x1421a2f8a  8b8894000000                   movl	0x94(%rax), %ecx
// 0x1421a2f90  488b542440                     movq	0x40(%rsp), %rdx
// Frames: F27 -> F28 -> F68 -> F67 -> F21 -> F6
// 0x1421a2f95  4c8b8aa8010000                 movq	0x1a8(%rdx), %r9
// 0x1421a2f9c  4c8d14c9                       leaq	(%rcx,%rcx,8), %r10
// 0x1421a2fa0  4b8d14d1                       leaq	(%r9,%r10,8), %rdx
// 0x1421a2fa4  488b4c2430                     movq	0x30(%rsp), %rcx
// Frames: F69 -> F70 -> F21 -> F6
// 0x1421a2fa9  4c8b8110010000                 movq	0x110(%rcx), %r8
// 0x1421a2fb0  4b8b4cd138                     movq	0x38(%r9,%r10,8), %rcx
// Frames: F27 -> F28 -> F47 -> F43 -> F44 -> F45 -> F46 -> F71 -> F69 -> F70 -> F21 -> F6
// 0x1421a2fb5  4f3944d140                     cmpq	%r8, 0x40(%r9,%r10,8)
// 0x1421a2fba  0f86a6000000                   jbe	0x1421a3066 <.text+0x21a2066>
// Frames: F48 -> F49 -> F50 -> F47 -> F43 -> F44 -> F45 -> F46 -> F71 -> F69 -> F70 -> F21 -> F6
// 0x1421a2fc0  4e8b04c1                       movq	(%rcx,%r8,8), %r8
// Frames: F43 -> F44 -> F51 -> F72 -> F69 -> F70 -> F21 -> F6
// 0x1421a2fc4  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F48 -> F49 -> F50 -> F47 -> F43 -> F44 -> F45 -> F46 -> F71 -> F69 -> F70 -> F21 -> F6
// 0x1421a2fc8  4d85c0                         testq	%r8, %r8
// Frames: F50 -> F47 -> F43 -> F44 -> F45 -> F46 -> F71 -> F69 -> F70 -> F21 -> F6
// 0x1421a2fcb  0f8499000000                   je	0x1421a306a <.text+0x21a206a>
// Frames: F52 -> F53 -> F54 -> F43 -> F44 -> F45 -> F46 -> F71 -> F69 -> F70 -> F21 -> F6
// 0x1421a2fd1  49f7d0                         notq	%r8
// Frames: F55 -> F56 -> F57 -> F54 -> F58 -> F46 -> F71 -> F69 -> F70 -> F21 -> F6
// 0x1421a2fd4  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x1421a2fd8  49c1e004                       shlq	$0x4, %r8
// 0x1421a2fdc  4e8b440210                     movq	0x10(%rdx,%r8), %r8
// 0x1421a2fe1  4c89442428                     movq	%r8, 0x28(%rsp)
// 0x1421a2fe6  4c8b442430                     movq	0x30(%rsp), %r8
// Frames: F59 -> F73 -> F69 -> F70 -> F21 -> F6
// 0x1421a2feb  4d8b8020010000                 movq	0x120(%r8), %r8
// Frames: F27 -> F28 -> F61 -> F62 -> F74 -> F75 -> F65 -> F73 -> F69 -> F70 -> F21 -> F6
// 0x1421a2ff2  4c398088000000                 cmpq	%r8, 0x88(%rax)
// 0x1421a2ff9  0f878d000000                   ja	0x1421a308c <.text+0x21a208c>
// 0x1421a2fff  4531e4                         xorl	%r12d, %r12d
// 0x1421a3002  e995000000                     jmp	0x1421a309c <.text+0x21a209c>
// Frames: F43 -> F44 -> F51 -> F41 -> F42 -> F21 -> F6
// 0x1421a3007  488b4918                       movq	0x18(%rcx), %rcx
// Frames: F59 -> F60 -> F41 -> F42 -> F21 -> F6
// 0x1421a300b  48c744242800000000             movq	$0x0, 0x28(%rsp)
// 0x1421a3014  4c8b442430                     movq	0x30(%rsp), %r8
// 0x1421a3019  4d8b8020010000                 movq	0x120(%r8), %r8
// Frames: F27 -> F28 -> F61 -> F62 -> F63 -> F64 -> F65 -> F60 -> F41 -> F42 -> F21 -> F6
// 0x1421a3020  4c39c2                         cmpq	%r8, %rdx
// 0x1421a3023  0f8656ffffff                   jbe	0x1421a2f7f <.text+0x21a1f7f>
// Frames: F76 -> F49 -> F77 -> F61 -> F62 -> F63 -> F64 -> F65 -> F60 -> F41 -> F42 -> F21 -> F6
// 0x1421a3029  4a833cc000                     cmpq	$0x0, (%rax,%r8,8)
// 0x1421a302e  410f95c4                       setne	%r12b
// 0x1421a3032  4983c604                       addq	$0x4, %r14
// 0x1421a3036  488b542430                     movq	0x30(%rsp), %rdx
// 0x1421a303b  488b9218010000                 movq	0x118(%rdx), %rdx
// 0x1421a3042  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x1421a3046  48f7d0                         notq	%rax
// 0x1421a3049  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1421a304d  48c1e004                       shlq	$0x4, %rax
// 0x1421a3051  488b6c0110                     movq	0x10(%rcx,%rax), %rbp
// 0x1421a3056  488b440128                     movq	0x28(%rcx,%rax), %rax
// 0x1421a305b  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1421a3060  31f6                           xorl	%esi, %esi
// Frames: F78 -> F79 -> F80 -> F81 -> F82 -> F83 -> F21 -> F6
// 0x1421a3062  89f0                           movl	%esi, %eax
// 0x1421a3064  eb7f                           jmp	0x1421a30e5 <.text+0x21a20e5>
// Frames: F43 -> F44 -> F51 -> F72 -> F69 -> F70 -> F21 -> F6
// 0x1421a3066  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F59 -> F73 -> F69 -> F70 -> F21 -> F6
// 0x1421a306a  48c744242800000000             movq	$0x0, 0x28(%rsp)
// 0x1421a3073  4c8b442430                     movq	0x30(%rsp), %r8
// 0x1421a3078  4d8b8020010000                 movq	0x120(%r8), %r8
// Frames: F27 -> F28 -> F61 -> F62 -> F74 -> F75 -> F65 -> F73 -> F69 -> F70 -> F21 -> F6
// 0x1421a307f  4c398088000000                 cmpq	%r8, 0x88(%rax)
// 0x1421a3086  0f8673ffffff                   jbe	0x1421a2fff <.text+0x21a1fff>
// Frames: F62 -> F74 -> F75 -> F65 -> F73 -> F69 -> F70 -> F21 -> F6
// 0x1421a308c  4c8b8880000000                 movq	0x80(%rax), %r9
// Frames: F76 -> F49 -> F77 -> F61 -> F62 -> F74 -> F75 -> F65 -> F73 -> F69 -> F70 -> F21 -> F6
// 0x1421a3093  4b833cc100                     cmpq	$0x0, (%r9,%r8,8)
// 0x1421a3098  410f95c4                       setne	%r12b
// Frames: F7 -> F8 -> F9 -> F10 -> F11 -> F12 -> F84 -> F85 -> F21 -> F6
// 0x1421a309c  4983c604                       addq	$0x4, %r14
// 0x1421a30a0  4c8b442430                     movq	0x30(%rsp), %r8
// 0x1421a30a5  4d8b8018010000                 movq	0x118(%r8), %r8
// 0x1421a30ac  4a8b0cc1                       movq	(%rcx,%r8,8), %rcx
// 0x1421a30b0  48f7d1                         notq	%rcx
// 0x1421a30b3  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1421a30b7  48c1e104                       shlq	$0x4, %rcx
// 0x1421a30bb  488b6c0a10                     movq	0x10(%rdx,%rcx), %rbp
// 0x1421a30c0  488b4c0a28                     movq	0x28(%rdx,%rcx), %rcx
// 0x1421a30c5  48894c2448                     movq	%rcx, 0x48(%rsp)
// 0x1421a30ca  488b4050                       movq	0x50(%rax), %rax
// 0x1421a30ce  4889442450                     movq	%rax, 0x50(%rsp)
// 0x1421a30d3  31f6                           xorl	%esi, %esi
// Frames: F86 -> F87 -> F21 -> F6
// 0x1421a30d5  4889f0                         movq	%rsi, %rax
// 0x1421a30d8  48c1e004                       shlq	$0x4, %rax
// 0x1421a30dc  488b4c2450                     movq	0x50(%rsp), %rcx
// Frames: F86 -> F88 -> F21 -> F6
// 0x1421a30e1  8b440108                       movl	0x8(%rcx,%rax), %eax
// Frames: F16 -> F89 -> F79 -> F80 -> F81 -> F82 -> F90 -> F21 -> F6
// 0x1421a30e5  486bc858                       imulq	$0x58, %rax, %rcx
// 0x1421a30e9  48034c2428                     addq	0x28(%rsp), %rcx
// Frames: F16 -> F89 -> F91 -> F80 -> F92 -> F82 -> F90 -> F21 -> F6
// 0x1421a30ee  488d3cc500000000               leaq	(,%rax,8), %rdi
// 0x1421a30f6  4801ef                         addq	%rbp, %rdi
// 0x1421a30f9  488b542448                     movq	0x48(%rsp), %rdx
// Frames: F16 -> F89 -> F93 -> F80 -> F92 -> F82 -> F90 -> F21 -> F6
// 0x1421a30fe  488d1c82                       leaq	(%rdx,%rax,4), %rbx
// Frames: F94
// 0x1421a3102  e8790274fe                     callq	0x1408e3380 <.text+0x8e2380>
// Frames: F95
// 0x1421a3107  41f6c401                       testb	$0x1, %r12b
// 0x1421a310b  7423                           je	0x1421a3130 <.text+0x21a2130>
// Frames: F96
// 0x1421a310d  488b442458                     movq	0x58(%rsp), %rax
// 0x1421a3112  f30f104028                     movss	0x28(%rax), %xmm0
// 0x1421a3117  0f28c8                         movaps	%xmm0, %xmm1
// 0x1421a311a  f30f59ce                       mulss	%xmm6, %xmm1
// Frames: F97
// 0x1421a311e  f30f59c7                       mulss	%xmm7, %xmm0
// Frames: F98
// 0x1421a3122  410f2ec8                       ucomiss	%xmm8, %xmm1
// 0x1421a3126  7547                           jne	0x1421a316f <.text+0x21a216f>
// 0x1421a3128  7a45                           jp	0x1421a316f <.text+0x21a216f>
// 0x1421a312a  eb60                           jmp	0x1421a318c <.text+0x21a218c>
// 0x1421a312c  0f1f4000                       nopl	(%rax)
// Frames: F94
// 0x1421a3130  410f28d2                       movaps	%xmm10, %xmm2
// 0x1421a3134  84c0                           testb	%al, %al
// 0x1421a3136  7504                           jne	0x1421a313c <.text+0x21a213c>
// 0x1421a3138  410f28d1                       movaps	%xmm9, %xmm2
// Frames: F99
// 0x1421a313c  488b442458                     movq	0x58(%rsp), %rax
// 0x1421a3141  f30f105828                     movss	0x28(%rax), %xmm3
// 0x1421a3146  0f28cb                         movaps	%xmm3, %xmm1
// 0x1421a3149  f3410f59cb                     mulss	%xmm11, %xmm1
// Frames: F100
// 0x1421a314e  410f28c5                       movaps	%xmm13, %xmm0
// 0x1421a3152  f30fc20701                     cmpltss	(%rdi), %xmm0
// 0x1421a3157  f3410f59dc                     mulss	%xmm12, %xmm3
// 0x1421a315c  0f54c3                         andps	%xmm3, %xmm0
// Frames: F101
// 0x1421a315f  f30f59ca                       mulss	%xmm2, %xmm1
// 0x1421a3163  f30f59c2                       mulss	%xmm2, %xmm0
// Frames: F98
// 0x1421a3167  410f2ec8                       ucomiss	%xmm8, %xmm1
// 0x1421a316b  7502                           jne	0x1421a316f <.text+0x21a216f>
// 0x1421a316d  7b1d                           jnp	0x1421a318c <.text+0x21a218c>
// Frames: F102 -> F103 -> F104
// 0x1421a316f  8b44243c                       movl	0x3c(%rsp), %eax
// 0x1421a3173  8903                           movl	%eax, (%rbx)
// Frames: F104
// 0x1421a3175  f30f580f                       addss	(%rdi), %xmm1
// Frames: F105 -> F106
// 0x1421a3179  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421a317c  f30f5fd1                       maxss	%xmm1, %xmm2
// Frames: F107 -> F106
// 0x1421a3180  410f28c9                       movaps	%xmm9, %xmm1
// 0x1421a3184  f30f5dca                       minss	%xmm2, %xmm1
// Frames: F106
// 0x1421a3188  f30f110f                       movss	%xmm1, (%rdi)
// Frames: F108
// 0x1421a318c  48ffc6                         incq	%rsi
// 0x1421a318f  410f2ec0                       ucomiss	%xmm8, %xmm0
// 0x1421a3193  7506                           jne	0x1421a319b <.text+0x21a219b>
// 0x1421a3195  0f8bf4fcffff                   jnp	0x1421a2e8f <.text+0x21a1e8f>
// Frames: F102 -> F103 -> F109
// 0x1421a319b  8b44243c                       movl	0x3c(%rsp), %eax
// 0x1421a319f  8903                           movl	%eax, (%rbx)
// Frames: F109
// 0x1421a31a1  f30f584704                     addss	0x4(%rdi), %xmm0
// Frames: F105 -> F110
// 0x1421a31a6  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421a31a9  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F107 -> F110
// 0x1421a31ad  410f28c1                       movaps	%xmm9, %xmm0
// 0x1421a31b1  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F110
// 0x1421a31b5  f30f114704                     movss	%xmm0, 0x4(%rdi)
// Frames: F108
// 0x1421a31ba  e9d0fcffff                     jmp	0x1421a2e8f <.text+0x21a1e8f>
// Frames: F111
// 0x1421a31bf  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x1421a31c4  0f287c2470                     movaps	0x70(%rsp), %xmm7
// 0x1421a31c9  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x1421a31d2  440f288c2490000000             movaps	0x90(%rsp), %xmm9
// 0x1421a31db  440f289424a0000000             movaps	0xa0(%rsp), %xmm10
// 0x1421a31e4  440f289c24b0000000             movaps	0xb0(%rsp), %xmm11
// 0x1421a31ed  440f28a424c0000000             movaps	0xc0(%rsp), %xmm12
// 0x1421a31f6  440f28ac24d0000000             movaps	0xd0(%rsp), %xmm13
// 0x1421a31ff  4881c4e8000000                 addq	$0xe8, %rsp
// 0x1421a3206  5b                             popq	%rbx
// 0x1421a3207  5d                             popq	%rbp
// 0x1421a3208  5f                             popq	%rdi
// 0x1421a3209  5e                             popq	%rsi
// 0x1421a320a  415c                           popq	%r12
// 0x1421a320c  415d                           popq	%r13
// 0x1421a320e  415e                           popq	%r14
// 0x1421a3210  415f                           popq	%r15
// 0x1421a3212  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/roof_animation.rs:11:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:640:0 bevy_ecs::system::query::Query::copy_unsafe
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:622:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:604:0 bevy_ecs::system::query::Query::reborrow
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:702:0 bevy_ecs::system::query::Query::iter_mut
// F6: crates/systems/roof/src/visual/roof_animation.rs:20:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F7: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F8: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F10: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F11: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F13: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2372:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F14: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:52:0 bevy_ecs::query::iter::QueryIter::new
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:2579:0 bevy_ecs::system::query::Query::reborrow
// F16: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 alloc::vec::impl$8::deref
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2373:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2464:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:889:0 bevy_ecs::query::iter::impl$1::next
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2467:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F23: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:1623:0 core::ptr::non_null::impl$14::eq
// F24: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$182::next
// F25: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2468:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$182::next
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F28: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F30: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2469:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F31: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2681:0 alloc::vec::Vec::len
// F32: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2706:0 alloc::vec::Vec::is_empty
// F33: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:616:0 bevy_ecs::storage::table::Table::is_empty
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2470:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2505:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2506:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:896:0 bevy_ecs::archetype::Archetypes::get
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2507:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:616:0 bevy_ecs::archetype::Archetype::is_empty
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2508:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$4::set_table
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2476:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:367:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1181:0 bevy_ecs::query::fetch::impl$26::set_table
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::slice::impl$0::get
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1544:0 bevy_ecs::query::fetch::impl$36::set_table
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F54: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:304:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1211:0 bevy_ecs::query::fetch::impl$26::matches_component_set
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1816:0 bevy_ecs::query::fetch::impl$41::set_table
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:51:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:389:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:497:0 bevy_ecs::storage::table::Table::has_column
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1816:0 bevy_ecs::query::fetch::impl$41::set_table::closure$0
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1214:0 bevy_ecs::query::fetch::impl$26::matches_component_set
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:461:0 bevy_ecs::archetype::Archetype::table_id
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2511:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::archetype::Archetype::table_id
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$4::set_archetype
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2515:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1169:0 bevy_ecs::query::fetch::impl$26::set_archetype
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1533:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1805:0 bevy_ecs::query::fetch::impl$41::set_archetype
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:622:0 bevy_ecs::archetype::Archetype::contains
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1805:0 bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
// F76: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:608:0 core::option::Option::is_some
// F77: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:631:0 core::slice::impl$0::get
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:129:0 bevy_ecs::storage::table::TableRow::as_usize
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1239:0 bevy_ecs::query::fetch::impl$27::fetch::closure$0
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1234:0 bevy_ecs::query::fetch::impl$27::fetch
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2090:0 bevy_ecs::query::fetch::impl$65::fetch
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2498:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:467:0 bevy_ecs::archetype::Archetype::entities
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2528:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:326:0 bevy_ecs::archetype::ArchetypeEntity::table_row
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2540:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2555:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:0:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1608:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1601:0 bevy_ecs::query::fetch::impl$27::fetch
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1612:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F94: crates/systems/roof/src/visual/roof_animation.rs:21:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F95: crates/systems/roof/src/visual/roof_animation.rs:23:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F96: crates/systems/roof/src/visual/roof_animation.rs:25:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F97: crates/systems/roof/src/visual/roof_animation.rs:26:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F98: crates/systems/roof/src/visual/roof_animation.rs:40:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F99: crates/systems/roof/src/visual/roof_animation.rs:30:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F100: crates/systems/roof/src/visual/roof_animation.rs:31:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F101: crates/systems/roof/src/visual/roof_animation.rs:37:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F102: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F104: crates/systems/roof/src/visual/roof_animation.rs:41:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F105: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1418:0 core::f32::impl$0::clamp
// F106: crates/systems/roof/src/visual/roof_animation.rs:42:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F107: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1421:0 core::f32::impl$0::clamp
// F108: crates/systems/roof/src/visual/roof_animation.rs:45:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F109: crates/systems/roof/src/visual/roof_animation.rs:46:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F110: crates/systems/roof/src/visual/roof_animation.rs:47:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// F111: crates/systems/roof/src/visual/roof_animation.rs:50:0 system_roof::visual::roof_animation::roof_animation::h32e1e9580e8dfcd5
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/visual/roof_animation.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 29b5ee6b87c6eb106b3a0c5c13831027a5be153d6ad7343ac17efb742b3dc5e5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3460
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::roof_animation::roof_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3460,"record_offset":260,"rva":35270048,"va":5403979168}
// 编译实例 RVAs: 0x21a2da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25951623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403979168,"raw_hex":"e8790274fe","size":5,"soft_fail":false,"target_va":5378028416,"va":5403980034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3460,"record_offset":4216,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3460,"record_offset":816,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3460,"record_offset":408,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3460,"record_offset":4272,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3460,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3460,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3460,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3460,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3460,"record_offset":600,"parent_record_offset":568,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3460,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":260,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3460,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3460,"record_offset":568,"parent_record_offset":540,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3460,"record_offset":3228,"parent_record_offset":3200,"ancestor_record_offset":260,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3460,"record_offset":3612,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3460,"record_offset":1332,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3460,"record_offset":2364,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3460,"record_offset":3808,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3460,"record_offset":1248,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3460,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":260,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":260,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":2060,"parent_record_offset":2032,"ancestor_record_offset":260,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":2484,"parent_record_offset":2456,"ancestor_record_offset":260,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":1444,"parent_record_offset":1416,"ancestor_record_offset":260,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":3496,"parent_record_offset":3472,"ancestor_record_offset":260,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":260,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":3008,"parent_record_offset":2456,"ancestor_record_offset":260,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":1936,"parent_record_offset":1416,"ancestor_record_offset":260,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":260,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":3136,"parent_record_offset":2456,"ancestor_record_offset":260,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":3200,"parent_record_offset":3164,"ancestor_record_offset":260,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":2032,"parent_record_offset":1416,"ancestor_record_offset":260,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":2096,"parent_record_offset":2060,"ancestor_record_offset":260,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$65::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3460,"record_offset":3472,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":17024,"source_line":2083,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3460,"record_offset":516,"parent_record_offset":432,"ancestor_record_offset":260,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3460,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":260,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3460,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":260,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3460,"record_offset":2456,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":17025,"source_line":182,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3460,"record_offset":1416,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":17022,"source_line":195,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3460,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":260,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3460,"record_offset":2180,"parent_record_offset":2152,"ancestor_record_offset":260,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3460,"record_offset":1560,"parent_record_offset":1532,"ancestor_record_offset":260,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3460,"record_offset":2152,"parent_record_offset":2124,"ancestor_record_offset":260,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3460,"record_offset":1532,"parent_record_offset":1504,"ancestor_record_offset":260,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3460,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":260,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3460,"record_offset":1504,"parent_record_offset":1472,"ancestor_record_offset":260,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3460,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":260,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3460,"record_offset":2124,"parent_record_offset":2096,"ancestor_record_offset":260,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3460,"record_offset":1088,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3460,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":260,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3460,"record_offset":1004,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3460,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":260,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3460,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":260,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3460,"record_offset":484,"parent_record_offset":460,"ancestor_record_offset":260,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3460,"record_offset":432,"parent_record_offset":408,"ancestor_record_offset":260,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3460,"record_offset":460,"parent_record_offset":432,"ancestor_record_offset":260,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3460,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":260,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3460,"record_offset":1652,"parent_record_offset":1588,"ancestor_record_offset":260,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3460,"record_offset":1680,"parent_record_offset":1652,"ancestor_record_offset":260,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3460,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":260,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3460,"record_offset":1724,"parent_record_offset":1532,"ancestor_record_offset":260,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3460,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3460,"record_offset":968,"parent_record_offset":932,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3460,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3460,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":260,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3460,"record_offset":740,"parent_record_offset":568,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3460,"record_offset":932,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03460.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3460,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":260,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03460.jsonl.gz"}
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
// 完整结构及原始类型记录: roof_animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [已有逆向证据 B / native_observed_partial_layout; not full original struct declaration] RoofAnimation
// name: "RoofAnimation"
// baseline: "B"
// status: "native_observed_partial_layout; not full original struct declaration"
// alignment: "unknown"
// complete_size: "unknown"
// unobserved_fields: "unknown"
// original_field_names: "unknown unless explicitly established"
// evidence: [{"path": "reconstruction/building/roof_components.rs", "sha256": "18f23980d35689b61cf219e26e0f812f813564edf791d09b7e6ae73d4ddcf6df"}, {"path": "evidence/building-lifecycle/B-verification.json", "sha256": "50c460dfeb7f1bab9fc2237fb5bad1a3e1405e66c3e00e5dd6a1c8d3209a59ca"}]
// observed_storage_span: 8
// source_mapping: "PDB namespace candidate; original definition file unknown"
// [观察字段；未命名者 original_name=unknown] {"offset": 0, "width": 4, "type": "f32", "original_name": "unknown", "observed_semantics": "primary animation channel"}
// [观察字段；未命名者 original_name=unknown] {"offset": 4, "width": 4, "type": "f32", "original_name": "unknown", "observed_semantics": "secondary animation channel"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_roof::visual::roof_animation::roof_animation
// 信息 ID: c791c74cf661c060548eba89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e44a27472b886e31da12704: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x1a27205 -> 0x21a2da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x1a35e2c -> 0x21a2da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
