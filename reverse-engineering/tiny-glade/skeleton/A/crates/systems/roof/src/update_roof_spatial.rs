// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: update_roof_spatial.rs.assembly.json.gz

// FUNCTION system_roof::update_roof_spatial::update_roof_spatial::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3194:1300 -> host 3194:248 / rva_1987c10_len_4ed; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1987d81, 0x1987e0a), 22 instructions
//
// FUNCTION system_roof::update_roof_spatial::get_shape_params
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3194:1820 -> host 3194:248 / rva_1987c10_len_4ed; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1987eb9, 0x1987ed9), 5 instructions
//
// FUNCTION system_roof::update_roof_spatial::update_roof_spatial
// Candidate interfaces: 8aa669a5df0750f1da372704
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3194:248 -> rva_1987c10_len_4ed
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1987c10_len_4ed: RVA 0x1987c10, 1261 PDB bytes; 287 instructions
// Evidence: _assembly/roof-v1/A/pack-002/rva_1987c10_len_4ed.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3194:248: system_roof::update_roof_spatial::update_roof_spatial
// Frames: F1
// 0x141987c10  4157                           pushq	%r15
// 0x141987c12  4156                           pushq	%r14
// 0x141987c14  4155                           pushq	%r13
// 0x141987c16  4154                           pushq	%r12
// 0x141987c18  56                             pushq	%rsi
// 0x141987c19  57                             pushq	%rdi
// 0x141987c1a  55                             pushq	%rbp
// 0x141987c1b  53                             pushq	%rbx
// 0x141987c1c  4881ec18010000                 subq	$0x118, %rsp            # imm = 0x118
// 0x141987c23  0f29b42400010000               movaps	%xmm6, 0x100(%rsp)
// Frames: F2 -> F3 -> F4
// 0x141987c2b  498b18                         movq	(%r8), %rbx
// Frames: F5 -> F3 -> F4
// 0x141987c2e  498b4008                       movq	0x8(%r8), %rax
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x141987c32  4c8b1b                         movq	(%rbx), %r11
// 0x141987c35  4531d2                         xorl	%r10d, %r10d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x141987c38  4c89de                         movq	%r11, %rsi
// 0x141987c3b  482b7018                       subq	0x18(%rax), %rsi
// 0x141987c3f  490f42f2                       cmovbq	%r10, %rsi
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x141987c43  4c2b5838                       subq	0x38(%rax), %r11
// 0x141987c47  4d0f42da                       cmovbq	%r10, %r11
// Frames: F12 -> F13 -> F14 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x141987c4b  4989f0                         movq	%rsi, %r8
// 0x141987c4e  49c1e004                       shlq	$0x4, %r8
// 0x141987c52  4c034008                       addq	0x8(%rax), %r8
// Frames: F15 -> F7 -> F8 -> F3 -> F4
// 0x141987c56  4c8b7810                       movq	0x10(%rax), %r15
// Frames: F16 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x141987c5a  4929f7                         subq	%rsi, %r15
// 0x141987c5d  4d0f42fa                       cmovbq	%r10, %r15
// Frames: F17 -> F7 -> F8 -> F3 -> F4
// 0x141987c61  4c8b7030                       movq	0x30(%rax), %r14
// Frames: F16 -> F15 -> F7 -> F8 -> F3 -> F4
// 0x141987c65  488d3d44069500                 leaq	0x950644(%rip), %rdi    # 0x1422d82b0
// 0x141987c6c  4c0f42c7                       cmovbq	%rdi, %r8
// Frames: F12 -> F13 -> F14 -> F17 -> F7 -> F8 -> F3 -> F4
// 0x141987c70  4c89de                         movq	%r11, %rsi
// 0x141987c73  48c1e604                       shlq	$0x4, %rsi
// 0x141987c77  48037028                       addq	0x28(%rax), %rsi
// 0x141987c7b  4d39f3                         cmpq	%r14, %r11
// Frames: F16 -> F17 -> F7 -> F8 -> F3 -> F4
// 0x141987c7e  480f47f7                       cmovaq	%rdi, %rsi
// 0x141987c82  4d29de                         subq	%r11, %r14
// 0x141987c85  4d0f42f2                       cmovbq	%r10, %r14
// Frames: F18 -> F7 -> F8 -> F3 -> F4
// 0x141987c89  488b4040                       movq	0x40(%rax), %rax
// 0x141987c8d  4c29f0                         subq	%r14, %rax
// 0x141987c90  4c29f8                         subq	%r15, %rax
// 0x141987c93  488903                         movq	%rax, (%rbx)
// Frames: F19 -> F20 -> F21 -> F22 -> F7 -> F8 -> F3 -> F4
// 0x141987c96  49c1e704                       shlq	$0x4, %r15
// 0x141987c9a  4d01c7                         addq	%r8, %r15
// 0x141987c9d  49c1e604                       shlq	$0x4, %r14
// 0x141987ca1  4901f6                         addq	%rsi, %r14
// 0x141987ca4  4c89b424a0000000               movq	%r14, 0xa0(%rsp)
// 0x141987cac  4c8b22                         movq	(%rdx), %r12
// 0x141987caf  488b01                         movq	(%rcx), %rax
// 0x141987cb2  4889842498000000               movq	%rax, 0x98(%rsp)
// 0x141987cba  488b4108                       movq	0x8(%rcx), %rax
// 0x141987cbe  4889842490000000               movq	%rax, 0x90(%rsp)
// 0x141987cc6  8b4110                         movl	0x10(%rcx), %eax
// 0x141987cc9  89442454                       movl	%eax, 0x54(%rsp)
// 0x141987ccd  8b4114                         movl	0x14(%rcx), %eax
// 0x141987cd0  89442450                       movl	%eax, 0x50(%rsp)
// 0x141987cd4  418b411c                       movl	0x1c(%r9), %eax
// 0x141987cd8  89442438                       movl	%eax, 0x38(%rsp)
// 0x141987cdc  498b01                         movq	(%r9), %rax
// 0x141987cdf  498b4910                       movq	0x10(%r9), %rcx
// 0x141987ce3  48894c2460                     movq	%rcx, 0x60(%rsp)
// 0x141987ce8  4889442440                     movq	%rax, 0x40(%rsp)
// 0x141987ced  4883c020                       addq	$0x20, %rax
// 0x141987cf1  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141987cf6  f30f103592bf8a00               movss	0x8abf92(%rip), %xmm6   # 0x142233c90
// 0x141987cfe  48895c2478                     movq	%rbx, 0x78(%rsp)
// 0x141987d03  4c897c2470                     movq	%r15, 0x70(%rsp)
// 0x141987d08  4c89642468                     movq	%r12, 0x68(%rsp)
// 0x141987d0d  eb11                           jmp	0x141987d20 <.text+0x1986d20>
// 0x141987d0f  488b742448                     movq	0x48(%rsp), %rsi
// 0x141987d14  4989f8                         movq	%rdi, %r8
// 0x141987d17  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F23 -> F24 -> F25 -> F26 -> F27 -> F4
// 0x141987d20  4d85c0                         testq	%r8, %r8
// 0x141987d23  741b                           je	0x141987d40 <.text+0x1986d40>
// 0x141987d25  4d39f8                         cmpq	%r15, %r8
// 0x141987d28  7416                           je	0x141987d40 <.text+0x1986d40>
// 0x141987d2a  498d7810                       leaq	0x10(%r8), %rdi
// 0x141987d2e  4889f5                         movq	%rsi, %rbp
// 0x141987d31  4d89c6                         movq	%r8, %r14
// 0x141987d34  eb2d                           jmp	0x141987d63 <.text+0x1986d63>
// 0x141987d36  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F23 -> F28 -> F29 -> F25 -> F26 -> F27 -> F4
// 0x141987d40  4885f6                         testq	%rsi, %rsi
// 0x141987d43  0f8425030000                   je	0x14198806e <.text+0x198706e>
// 0x141987d49  483bb424a0000000               cmpq	0xa0(%rsp), %rsi
// 0x141987d51  0f8417030000                   je	0x14198806e <.text+0x198706e>
// Frames: F30 -> F31 -> F32 -> F28 -> F29 -> F25 -> F26 -> F27 -> F4
// 0x141987d57  488d6e10                       leaq	0x10(%rsi), %rbp
// 0x141987d5b  31ff                           xorl	%edi, %edi
// 0x141987d5d  4989f6                         movq	%rsi, %r14
// 0x141987d60  4889ee                         movq	%rbp, %rsi
// Frames: F33 -> F27 -> F4
// 0x141987d63  48ff03                         incq	(%rbx)
// Frames: F34
// 0x141987d66  4d8b6e08                       movq	0x8(%r14), %r13
// Frames: F35
// 0x141987d6a  4c89e1                         movq	%r12, %rcx
// 0x141987d6d  4c89ea                         movq	%r13, %rdx
// 0x141987d70  e88b27cefe                     callq	0x14066a500 <.text+0x669500>
// Frames: F36 -> F35
// 0x141987d75  4889c1                         movq	%rax, %rcx
// 0x141987d78  48c1e920                       shrq	$0x20, %rcx
// 0x141987d7c  4989f8                         movq	%rdi, %r8
// 0x141987d7f  749f                           je	0x141987d20 <.text+0x1986d20>
// Frames: F37 -> F38 -> F39 -> F35
// 0x141987d81  48896c2448                     movq	%rbp, 0x48(%rsp)
// 0x141987d86  8b4c2450                       movl	0x50(%rsp), %ecx
// 0x141987d8a  894c2428                       movl	%ecx, 0x28(%rsp)
// 0x141987d8e  8b4c2454                       movl	0x54(%rsp), %ecx
// 0x141987d92  894c2420                       movl	%ecx, 0x20(%rsp)
// 0x141987d96  488d8c24a8000000               leaq	0xa8(%rsp), %rcx
// 0x141987d9e  488b942498000000               movq	0x98(%rsp), %rdx
// 0x141987da6  4c8b842490000000               movq	0x90(%rsp), %r8
// 0x141987dae  4989c1                         movq	%rax, %r9
// 0x141987db1  e8da11f5fe                     callq	0x1408d8f90 <.text+0x8d7f90>
// Frames: F40 -> F38 -> F39 -> F35
// 0x141987db6  488bac24a8000000               movq	0xa8(%rsp), %rbp
// 0x141987dbe  4885ed                         testq	%rbp, %rbp
// 0x141987dc1  0f8448ffffff                   je	0x141987d0f <.text+0x1986d0f>
// Frames: F41 -> F38 -> F39 -> F35
// 0x141987dc7  488b9c24b8000000               movq	0xb8(%rsp), %rbx
// 0x141987dcf  448bbc24c4000000               movl	0xc4(%rsp), %r15d
// 0x141987dd7  488b8424c8000000               movq	0xc8(%rsp), %rax
// 0x141987ddf  4889842480000000               movq	%rax, 0x80(%rsp)
// 0x141987de7  488bb424d0000000               movq	0xd0(%rsp), %rsi
// 0x141987def  488b8424e0000000               movq	0xe0(%rsp), %rax
// 0x141987df7  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x141987dff  8b8424ec000000                 movl	0xec(%rsp), %eax
// 0x141987e06  8944243c                       movl	%eax, 0x3c(%rsp)
// Frames: F42
// 0x141987e0a  4c89e1                         movq	%r12, %rcx
// 0x141987e0d  4c89ea                         movq	%r13, %rdx
// 0x141987e10  4c8d0541b0d600                 leaq	0xd6b041(%rip), %r8     # 0x1426f2e58
// 0x141987e17  e8c422cefe                     callq	0x14066a0e0 <.text+0x6690e0>
// 0x141987e1c  4885c0                         testq	%rax, %rax
// 0x141987e1f  0f8465020000                   je	0x14198808a <.text+0x198708a>
// Frames: F43
// 0x141987e25  4989c4                         movq	%rax, %r12
// 0x141987e28  4889c1                         movq	%rax, %rcx
// 0x141987e2b  e8104931ff                     callq	0x140c9c740 <.text+0xc9b740>
// Frames: F44 -> F45 -> F43
// 0x141987e30  44893b                         movl	%r15d, (%rbx)
// Frames: F43
// 0x141987e33  f30f114540                     movss	%xmm0, 0x40(%rbp)
// Frames: F46
// 0x141987e38  4d89e6                         movq	%r12, %r14
// 0x141987e3b  4981c67f020000                 addq	$0x27f, %r14            # imm = 0x27F
// 0x141987e42  4c89f1                         movq	%r14, %rcx
// 0x141987e45  e8b62bb9fe                     callq	0x14051aa00 <.text+0x519a00>
// Frames: F47 -> F48 -> F46
// 0x141987e4a  384548                         cmpb	%al, 0x48(%rbp)
// Frames: F46
// 0x141987e4d  746a                           je	0x141987eb9 <.text+0x1986eb9>
// Frames: F49
// 0x141987e4f  4c89f1                         movq	%r14, %rcx
// 0x141987e52  e8a92bb9fe                     callq	0x14051aa00 <.text+0x519a00>
// Frames: F44 -> F45 -> F49
// 0x141987e57  44893b                         movl	%r15d, (%rbx)
// Frames: F49
// 0x141987e5a  884548                         movb	%al, 0x48(%rbp)
// 0x141987e5d  8b442438                       movl	0x38(%rsp), %eax
// 0x141987e61  488b4c2460                     movq	0x60(%rsp), %rcx
// Frames: F50 -> F51 -> F52 -> F53
// 0x141987e66  8901                           movl	%eax, (%rcx)
// 0x141987e68  488b4c2440                     movq	0x40(%rsp), %rcx
// Frames: F54 -> F55 -> F52 -> F53
// 0x141987e6d  488b5130                       movq	0x30(%rcx), %rdx
// Frames: F56 -> F52 -> F53
// 0x141987e71  4c8b7140                       movq	0x40(%rcx), %r14
// Frames: F54 -> F55 -> F52 -> F53
// 0x141987e75  483b5120                       cmpq	0x20(%rcx), %rdx
// 0x141987e79  7513                           jne	0x141987e8e <.text+0x1986e8e>
// Frames: F57 -> F55 -> F52 -> F53
// 0x141987e7b  488b4c2458                     movq	0x58(%rsp), %rcx
// 0x141987e80  e88b1e8ffe                     callq	0x140279d10 <.text+0x278d10>
// 0x141987e85  488b4c2440                     movq	0x40(%rsp), %rcx
// Frames: F58 -> F55 -> F52 -> F53
// 0x141987e8a  488b5130                       movq	0x30(%rcx), %rdx
// Frames: F59 -> F60 -> F58 -> F55 -> F52 -> F53
// 0x141987e8e  488b4128                       movq	0x28(%rcx), %rax
// Frames: F61 -> F58 -> F55 -> F52 -> F53
// 0x141987e92  48c1e205                       shlq	$0x5, %rdx
// Frames: F62 -> F63 -> F55 -> F52 -> F53
// 0x141987e96  4c893410                       movq	%r14, (%rax,%rdx)
// 0x141987e9a  4c8b842480000000               movq	0x80(%rsp), %r8
// 0x141987ea2  4c89441008                     movq	%r8, 0x8(%rax,%rdx)
// 0x141987ea7  4c896c1010                     movq	%r13, 0x10(%rax,%rdx)
// 0x141987eac  c644101802                     movb	$0x2, 0x18(%rax,%rdx)
// Frames: F64 -> F55 -> F52 -> F53
// 0x141987eb1  48ff4130                       incq	0x30(%rcx)
// Frames: F65 -> F52 -> F53
// 0x141987eb5  48ff4140                       incq	0x40(%rcx)
// Frames: F66 -> F67
// 0x141987eb9  418b842448020000               movl	0x248(%r12), %eax
// 0x141987ec1  83f801                         cmpl	$0x1, %eax
// 0x141987ec4  777a                           ja	0x141987f40 <.text+0x1986f40>
// Frames: F68 -> F67
// 0x141987ec6  410f1084244c020000             movups	0x24c(%r12), %xmm0
// 0x141987ecf  f2410f108c245c020000           movsd	0x25c(%r12), %xmm1
// Frames: F69 -> F48 -> F70
// 0x141987ed9  3b4508                         cmpl	0x8(%rbp), %eax
// 0x141987edc  0f85aa000000                   jne	0x141987f8c <.text+0x1986f8c>
// 0x141987ee2  f30f10550c                     movss	0xc(%rbp), %xmm2
// 0x141987ee7  f30f105d10                     movss	0x10(%rbp), %xmm3
// 0x141987eec  f30f16e0                       movshdup	%xmm0, %xmm4            # xmm4 = xmm0[1,1,3,3]
// 0x141987ef0  0fc2e300                       cmpeqps	%xmm3, %xmm4
// 0x141987ef4  0fc2d000                       cmpeqps	%xmm0, %xmm2
// 0x141987ef8  0f54d4                         andps	%xmm4, %xmm2
// 0x141987efb  660f7ed1                       movd	%xmm2, %ecx
// 0x141987eff  85c0                           testl	%eax, %eax
// 0x141987f01  746f                           je	0x141987f72 <.text+0x1986f72>
// Frames: F71 -> F69 -> F48 -> F70
// 0x141987f03  f6c101                         testb	$0x1, %cl
// 0x141987f06  0f8480000000                   je	0x141987f8c <.text+0x1986f8c>
// Frames: F72 -> F69 -> F48 -> F70
// 0x141987f0c  0f2e4d1c                       ucomiss	0x1c(%rbp), %xmm1
// 0x141987f10  757a                           jne	0x141987f8c <.text+0x1986f8c>
// 0x141987f12  7a78                           jp	0x141987f8c <.text+0x1986f8c>
// Frames: F73 -> F69 -> F48 -> F70
// 0x141987f14  f30f16d1                       movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
// 0x141987f18  0f2e5520                       ucomiss	0x20(%rbp), %xmm2
// 0x141987f1c  756e                           jne	0x141987f8c <.text+0x1986f8c>
// 0x141987f1e  7a6c                           jp	0x141987f8c <.text+0x1986f8c>
// Frames: F74 -> F75 -> F69 -> F48 -> F70
// 0x141987f20  0f28d0                         movaps	%xmm0, %xmm2
// 0x141987f23  660f15d0                       unpckhpd	%xmm0, %xmm2            # xmm2 = xmm2[1],xmm0[1]
// 0x141987f27  0f2e5514                       ucomiss	0x14(%rbp), %xmm2
// Frames: F70
// 0x141987f2b  755f                           jne	0x141987f8c <.text+0x1986f8c>
// 0x141987f2d  7a5d                           jp	0x141987f8c <.text+0x1986f8c>
// 0x141987f2f  0f28d0                         movaps	%xmm0, %xmm2
// 0x141987f32  0fc6d0ff                       shufps	$0xff, %xmm0, %xmm2     # xmm2 = xmm2[3,3],xmm0[3,3]
// 0x141987f36  0f2e5518                       ucomiss	0x18(%rbp), %xmm2
// 0x141987f3a  7550                           jne	0x141987f8c <.text+0x1986f8c>
// 0x141987f3c  7a4e                           jp	0x141987f8c <.text+0x1986f8c>
// 0x141987f3e  6690                           nop
// Frames: F76
// 0x141987f40  4889e9                         movq	%rbp, %rcx
// 0x141987f43  e8d824d3fe                     callq	0x1406ba420 <.text+0x6b9420>
// 0x141987f48  84c0                           testb	%al, %al
// 0x141987f4a  0f84b6000000                   je	0x141988006 <.text+0x1987006>
// Frames: F44 -> F45 -> F77
// 0x141987f50  488b842488000000               movq	0x88(%rsp), %rax
// 0x141987f58  8b4c243c                       movl	0x3c(%rsp), %ecx
// 0x141987f5c  8908                           movl	%ecx, (%rax)
// Frames: F77
// 0x141987f5e  c7461800000000                 movl	$0x0, 0x18(%rsi)
// 0x141987f65  48c7461000000000               movq	$0x0, 0x10(%rsi)
// 0x141987f6d  e9e0000000                     jmp	0x141988052 <.text+0x1987052>
// Frames: F78 -> F79 -> F48 -> F70
// 0x141987f72  0f28d0                         movaps	%xmm0, %xmm2
// 0x141987f75  660f15d0                       unpckhpd	%xmm0, %xmm2            # xmm2 = xmm2[1],xmm0[1]
// 0x141987f79  0f2e5514                       ucomiss	0x14(%rbp), %xmm2
// 0x141987f7d  0f9bc2                         setnp	%dl
// 0x141987f80  410f94c0                       sete	%r8b
// 0x141987f84  4120d0                         andb	%dl, %r8b
// Frames: F70
// 0x141987f87  4484c1                         testb	%r8b, %cl
// 0x141987f8a  75b4                           jne	0x141987f40 <.text+0x1986f40>
// Frames: F44 -> F45 -> F80
// 0x141987f8c  44893b                         movl	%r15d, (%rbx)
// Frames: F80
// 0x141987f8f  894508                         movl	%eax, 0x8(%rbp)
// 0x141987f92  0f11450c                       movups	%xmm0, 0xc(%rbp)
// 0x141987f96  0f134d1c                       movlps	%xmm1, 0x1c(%rbp)
// 0x141987f9a  8b442438                       movl	0x38(%rsp), %eax
// 0x141987f9e  488b4c2460                     movq	0x60(%rsp), %rcx
// Frames: F50 -> F51 -> F52 -> F81
// 0x141987fa3  8901                           movl	%eax, (%rcx)
// 0x141987fa5  488b4c2440                     movq	0x40(%rsp), %rcx
// Frames: F54 -> F55 -> F52 -> F81
// 0x141987faa  488b5130                       movq	0x30(%rcx), %rdx
// Frames: F56 -> F52 -> F81
// 0x141987fae  488b5940                       movq	0x40(%rcx), %rbx
// Frames: F54 -> F55 -> F52 -> F81
// 0x141987fb2  483b5120                       cmpq	0x20(%rcx), %rdx
// 0x141987fb6  7513                           jne	0x141987fcb <.text+0x1986fcb>
// Frames: F57 -> F55 -> F52 -> F81
// 0x141987fb8  488b4c2458                     movq	0x58(%rsp), %rcx
// 0x141987fbd  e84e1d8ffe                     callq	0x140279d10 <.text+0x278d10>
// 0x141987fc2  488b4c2440                     movq	0x40(%rsp), %rcx
// Frames: F58 -> F55 -> F52 -> F81
// 0x141987fc7  488b5130                       movq	0x30(%rcx), %rdx
// Frames: F59 -> F60 -> F58 -> F55 -> F52 -> F81
// 0x141987fcb  488b4128                       movq	0x28(%rcx), %rax
// Frames: F61 -> F58 -> F55 -> F52 -> F81
// 0x141987fcf  48c1e205                       shlq	$0x5, %rdx
// Frames: F62 -> F63 -> F55 -> F52 -> F81
// 0x141987fd3  48891c10                       movq	%rbx, (%rax,%rdx)
// 0x141987fd7  4c8b842480000000               movq	0x80(%rsp), %r8
// 0x141987fdf  4c89441008                     movq	%r8, 0x8(%rax,%rdx)
// 0x141987fe4  4c896c1010                     movq	%r13, 0x10(%rax,%rdx)
// 0x141987fe9  c644101802                     movb	$0x2, 0x18(%rax,%rdx)
// Frames: F64 -> F55 -> F52 -> F81
// 0x141987fee  48ff4130                       incq	0x30(%rcx)
// Frames: F65 -> F52 -> F81
// 0x141987ff2  48ff4140                       incq	0x40(%rcx)
// Frames: F76
// 0x141987ff6  4889e9                         movq	%rbp, %rcx
// 0x141987ff9  e82224d3fe                     callq	0x1406ba420 <.text+0x6b9420>
// 0x141987ffe  84c0                           testb	%al, %al
// 0x141988000  0f854affffff                   jne	0x141987f50 <.text+0x1986f50>
// Frames: F82
// 0x141988006  488d8c24a8000000               leaq	0xa8(%rsp), %rcx
// 0x14198800e  4889ea                         movq	%rbp, %rdx
// 0x141988011  e8da23d3fe                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F83 -> F82
// 0x141988016  f30f108424a8000000             movss	0xa8(%rsp), %xmm0
// 0x14198801f  f30f108c24ac000000             movss	0xac(%rsp), %xmm1
// Frames: F84 -> F85 -> F82
// 0x141988028  f30f58ce                       addss	%xmm6, %xmm1
// Frames: F86 -> F82
// 0x14198802c  f30f109424b0000000             movss	0xb0(%rsp), %xmm2
// 0x141988035  488b842488000000               movq	0x88(%rsp), %rax
// 0x14198803d  8b4c243c                       movl	0x3c(%rsp), %ecx
// Frames: F44 -> F45 -> F82
// 0x141988041  8908                           movl	%ecx, (%rax)
// Frames: F82
// 0x141988043  f30f114610                     movss	%xmm0, 0x10(%rsi)
// 0x141988048  f30f114e14                     movss	%xmm1, 0x14(%rsi)
// 0x14198804d  f30f115618                     movss	%xmm2, 0x18(%rsi)
// 0x141988052  488b742448                     movq	0x48(%rsp), %rsi
// 0x141988057  4989f8                         movq	%rdi, %r8
// 0x14198805a  488b5c2478                     movq	0x78(%rsp), %rbx
// 0x14198805f  4c8b7c2470                     movq	0x70(%rsp), %r15
// 0x141988064  4c8b642468                     movq	0x68(%rsp), %r12
// 0x141988069  e9b2fcffff                     jmp	0x141987d20 <.text+0x1986d20>
// Frames: F87
// 0x14198806e  0f28b42400010000               movaps	0x100(%rsp), %xmm6
// 0x141988076  4881c418010000                 addq	$0x118, %rsp            # imm = 0x118
// 0x14198807d  5b                             popq	%rbx
// 0x14198807e  5d                             popq	%rbp
// 0x14198807f  5f                             popq	%rdi
// 0x141988080  5e                             popq	%rsi
// 0x141988081  415c                           popq	%r12
// 0x141988083  415d                           popq	%r13
// 0x141988085  415e                           popq	%r14
// 0x141988087  415f                           popq	%r15
// 0x141988089  c3                             retq
// Frames: F88
// 0x14198808a  4983c608                       addq	$0x8, %r14
// 0x14198808e  4c89b424f0000000               movq	%r14, 0xf0(%rsp)
// 0x141988096  488d05c3c2c4fe                 leaq	-0x13b3d3d(%rip), %rax  # 0x1405d4360 <.text+0x5d3360>
// 0x14198809d  48898424f8000000               movq	%rax, 0xf8(%rsp)
// Frames: F89 -> F88
// 0x1419880a5  488d0514aed600                 leaq	0xd6ae14(%rip), %rax    # 0x1426f2ec0
// 0x1419880ac  48898424a8000000               movq	%rax, 0xa8(%rsp)
// 0x1419880b4  48c78424b000000002000000       movq	$0x2, 0xb0(%rsp)
// 0x1419880c0  48c78424c800000000000000       movq	$0x0, 0xc8(%rsp)
// 0x1419880cc  488d8424f0000000               leaq	0xf0(%rsp), %rax
// 0x1419880d4  48898424b8000000               movq	%rax, 0xb8(%rsp)
// 0x1419880dc  48c78424c000000001000000       movq	$0x1, 0xc0(%rsp)
// Frames: F88
// 0x1419880e8  488d15f1add600                 leaq	0xd6adf1(%rip), %rdx    # 0x1426f2ee0
// 0x1419880ef  488d8c24a8000000               leaq	0xa8(%rsp), %rcx
// 0x1419880f7  e884697200                     callq	0x1420aea80 <.text+0x20ada80>
// 0x1419880fc  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/update_roof_spatial.rs:12:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F2: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:806:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:469:0 bevy_ecs::event::EventReader::read
// F4: crates/systems/roof/src/update_roof_spatial.rs:19:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
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
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:705:0 core::option::Option::as_mut
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:300:0 core::iter::adapters::chain::and_then_or_clear
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next
// F26: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:845:0 bevy_ecs::event::impl$23::next
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:771:0 bevy_ecs::event::impl$20::next
// F28: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next::closure$0
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1513:0 core::option::Option::or_else
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F31: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F33: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:850:0 bevy_ecs::event::impl$23::next
// F34: crates/systems/roof/src/update_roof_spatial.rs:21:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F35: crates/systems/roof/src/update_roof_spatial.rs:20:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1412:0 core::option::Option::and_then
// F37: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:967:0 bevy_ecs::system::query::Query::get_mut
// F38: crates/systems/roof/src/update_roof_spatial.rs:22:0 system_roof::update_roof_spatial::update_roof_spatial::closure$0
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1413:0 core::option::Option::and_then
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:632:0 core::result::Result::ok
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:633:0 core::result::Result::ok
// F42: crates/systems/roof/src/update_roof_spatial.rs:24:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F43: crates/systems/roof/src/update_roof_spatial.rs:25:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F44: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$49::set_changed
// F45: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$50::deref_mut
// F46: crates/systems/roof/src/update_roof_spatial.rs:28:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F47: crates/country-core/src/resources/roofs/roof_shape.rs:46:0 country_core::resources::roofs::roof_shape::impl$14::eq
// F48: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:264:0 core::cmp::PartialEq::ne
// F49: crates/systems/roof/src/update_roof_spatial.rs:29:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F50: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$29::set_changed
// F51: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F52: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:625:0 bevy_ecs::event::EventWriter::send
// F53: crates/systems/roof/src/update_roof_spatial.rs:30:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1970:0 alloc::vec::Vec::push
// F55: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::event::Events::send
// F56: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::event::Events::send
// F57: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1971:0 alloc::vec::Vec::push
// F58: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1974:0 alloc::vec::Vec::push
// F59: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F60: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F61: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 alloc::vec::Vec::as_mut_ptr
// F62: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F63: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1975:0 alloc::vec::Vec::push
// F64: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1976:0 alloc::vec::Vec::push
// F65: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::event::Events::send
// F66: crates/systems/roof/src/update_roof_spatial.rs:76:0 system_roof::update_roof_spatial::get_shape_params
// F67: crates/systems/roof/src/update_roof_spatial.rs:38:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F68: crates/systems/roof/src/update_roof_spatial.rs:78:0 system_roof::update_roof_spatial::get_shape_params
// F69: crates/country-core/src/systems/wall/valid_enclosure/mod.rs:64:0 country_core::systems::wall::valid_enclosure::impl$9::eq
// F70: crates/systems/roof/src/update_roof_spatial.rs:40:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F71: crates/utils/src/geometry/rectangle.rs:93:0 utils::geometry::rectangle::impl$10::eq
// F72: crates/utils/src/geometry/rectangle.rs:94:0 utils::geometry::rectangle::impl$10::eq
// F73: crates/utils/src/geometry/rectangle.rs:95:0 utils::geometry::rectangle::impl$10::eq
// F74: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:23:0 glam::f32::vec2::impl$46::eq
// F75: crates/utils/src/geometry/rectangle.rs:97:0 utils::geometry::rectangle::impl$10::eq
// F76: crates/systems/roof/src/update_roof_spatial.rs:55:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F77: crates/systems/roof/src/update_roof_spatial.rs:62:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F78: crates/utils/src/geometry/circle.rs:14:0 utils::geometry::circle::impl$6::eq
// F79: crates/country-core/src/systems/wall/valid_enclosure/mod.rs:63:0 country_core::systems::wall::valid_enclosure::impl$9::eq
// F80: crates/systems/roof/src/update_roof_spatial.rs:42:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F81: crates/systems/roof/src/update_roof_spatial.rs:43:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F82: crates/systems/roof/src/update_roof_spatial.rs:58:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F83: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1137:0 glam::f32::vec3::impl$17::sub
// F84: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:221:0 core::ops::arith::impl$68::sub
// F85: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1138:0 glam::f32::vec3::impl$17::sub
// F86: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1139:0 glam::f32::vec3::impl$17::sub
// F87: crates/systems/roof/src/update_roof_spatial.rs:73:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F88: crates/systems/roof/src/update_roof_spatial.rs:66:0 system_roof::update_roof_spatial::update_roof_spatial::h3ffe76b69c571805
// F89: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:343:0 core::fmt::Arguments::new_v1
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/update_roof_spatial.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 39e9f0afb4d4e99fdc8ae97df4716cc47fbcfb4533b929a8f94a69b280f261b4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: update_roof_spatial.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3194
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use system_roof; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::update_roof_spatial::update_roof_spatial
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3194,"record_offset":248,"rva":26770448,"va":5395479568}
// 编译实例 RVAs: 0x1987c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_roof_spatial.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21419082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e8b62bb9fe","size":5,"soft_fail":false,"target_va":5374061056,"va":5395480133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-24174965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e88b1e8ffe","size":5,"soft_fail":false,"target_va":5371305232,"va":5395480192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17493542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e8da11f5fe","size":5,"soft_fail":false,"target_va":5377986448,"va":5395479985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7498116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e884697200","size":5,"soft_fail":false,"target_va":5402978944,"va":5395480823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19717158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e8da23d3fe","size":5,"soft_fail":false,"target_va":5375763440,"va":5395480593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19716904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e8d824d3fe","size":5,"soft_fail":false,"target_va":5375763488,"va":5395480387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13547248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e8104931ff","size":5,"soft_fail":false,"target_va":5381932864,"va":5395480107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20045116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e8c422cefe","size":5,"soft_fail":false,"target_va":5375434976,"va":5395480087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20043893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395479568,"raw_hex":"e88b27cefe","size":5,"soft_fail":false,"target_va":5375436032,"va":5395479920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3194,"record_offset":1392,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3194,"record_offset":420,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3194,"record_offset":1560,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3194,"record_offset":960,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3194,"record_offset":1448,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3194,"record_offset":2504,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3194,"record_offset":1272,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3194,"record_offset":2384,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] system_roof::update_roof_spatial::get_shape_params
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/update_roof_spatial.rs
//     evidence={"module":3194,"record_offset":1820,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":101520,"source_line":75,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3194,"record_offset":1748,"parent_record_offset":1716,"ancestor_record_offset":248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3194,"record_offset":1716,"parent_record_offset":1676,"ancestor_record_offset":248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3194,"record_offset":1676,"parent_record_offset":1640,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3194,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3194,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3194,"record_offset":1416,"parent_record_offset":1392,"ancestor_record_offset":248,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3194,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":248,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3194,"record_offset":1640,"parent_record_offset":1560,"ancestor_record_offset":248,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3194,"record_offset":484,"parent_record_offset":444,"ancestor_record_offset":248,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3194,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":248,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3194,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":248,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3194,"record_offset":444,"parent_record_offset":420,"ancestor_record_offset":248,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3194,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":248,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3194,"record_offset":1012,"parent_record_offset":984,"ancestor_record_offset":248,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3194,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":248,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3194,"record_offset":552,"parent_record_offset":508,"ancestor_record_offset":248,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3194,"record_offset":2416,"parent_record_offset":2384,"ancestor_record_offset":248,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3194,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":248,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3194,"record_offset":1092,"parent_record_offset":1012,"ancestor_record_offset":248,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3194,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3194,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3194,"record_offset":1780,"parent_record_offset":1676,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3194,"record_offset":1352,"parent_record_offset":1300,"ancestor_record_offset":248,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3194,"record_offset":616,"parent_record_offset":508,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3194,"record_offset":804,"parent_record_offset":508,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3194,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":248,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3194,"record_offset":652,"parent_record_offset":616,"ancestor_record_offset":248,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3194,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3194,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":248,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3194,"record_offset":1168,"parent_record_offset":1116,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3194,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":248,"inlinee":101518,"source_line":46,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":3194,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":248,"inlinee":81186,"source_line":60,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3194,"record_offset":1944,"parent_record_offset":1912,"ancestor_record_offset":248,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::update_roof_spatial::update_roof_spatial::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/update_roof_spatial.rs
//     evidence={"module":3194,"record_offset":1300,"parent_record_offset":1272,"ancestor_record_offset":248,"inlinee":101516,"source_line":22,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3194,"record_offset":1976,"parent_record_offset":1880,"ancestor_record_offset":248,"inlinee":81190,"source_line":12,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3194,"record_offset":1912,"parent_record_offset":1880,"ancestor_record_offset":248,"inlinee":81188,"source_line":91,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::update_roof_spatial::update_roof_spatial::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18C8C
// 首条位置: {"module":3194,"inlinee":101516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3194,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3194,"record_offset":1352,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/03194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::update_roof_spatial::get_shape_params
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18C90
// 首条位置: {"module":3194,"inlinee":101520}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: update_roof_spatial.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: update_roof_spatial.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/update_roof_spatial.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: update_roof_spatial.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::get_shape_params
// 信息 ID: c1cddf7f68a09959a3905552
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3194, inline_record=1820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::update_roof_spatial
// 信息 ID: e5deeb1eb8790b6229615d79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8aa669a5df0750f1da372704: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)> > @ 0x12d53f7 -> 0x1987c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)> @ 0x12ddbea -> 0x1987c10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::update_roof_spatial::closure$0
// 信息 ID: b32dc70c205ee75c01f32c09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::update_roof_spatial::update_roof_spatial", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3194, inline_record=1300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
