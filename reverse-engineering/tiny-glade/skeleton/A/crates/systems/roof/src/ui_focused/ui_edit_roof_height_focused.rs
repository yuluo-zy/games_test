// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: ui_edit_roof_height_focused.rs.assembly.json.gz

// FUNCTION system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3200:3696 -> host 3200:1260 / rva_198cff0_len_db8; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x198d6da, 0x198d785), 28 instructions
//   Function-frame RVA span [0x198d8ad, 0x198d967), 31 instructions
// Inline site 3200:4200 -> host 3200:1260 / rva_198cff0_len_db8; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x198d6da, 0x198d785), 28 instructions
//   Function-frame RVA span [0x198d8ad, 0x198d967), 31 instructions
//
// FUNCTION system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// Candidate interfaces: 47c6405e8e70abf74f4bb199
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3200:1260 -> rva_198cff0_len_db8
//
// FUNCTION system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3200:5896 -> rva_198ddb0_len_1d9
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_198cff0_len_db8: RVA 0x198cff0, 3512 PDB bytes; 721 instructions
// Evidence: _assembly/roof-v1/A/pack-003/rva_198cff0_len_db8.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3200:1260: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// Frames: F1
// 0x14198cff0  55                             pushq	%rbp
// 0x14198cff1  4157                           pushq	%r15
// 0x14198cff3  4156                           pushq	%r14
// 0x14198cff5  4155                           pushq	%r13
// 0x14198cff7  4154                           pushq	%r12
// 0x14198cff9  56                             pushq	%rsi
// 0x14198cffa  57                             pushq	%rdi
// 0x14198cffb  53                             pushq	%rbx
// 0x14198cffc  4881ece8020000                 subq	$0x2e8, %rsp            # imm = 0x2E8
// 0x14198d003  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14198d00b  440f298d50020000               movaps	%xmm9, 0x250(%rbp)
// 0x14198d013  440f298540020000               movaps	%xmm8, 0x240(%rbp)
// 0x14198d01b  0f29bd30020000                 movaps	%xmm7, 0x230(%rbp)
// 0x14198d022  0f29b520020000                 movaps	%xmm6, 0x220(%rbp)
// Frames: F2
// 0x14198d029  48c78518020000feffffff         movq	$-0x2, 0x218(%rbp)
// 0x14198d034  488b02                         movq	(%rdx), %rax
// 0x14198d037  488b10                         movq	(%rax), %rdx
// 0x14198d03a  4883fa02                       cmpq	$0x2, %rdx
// 0x14198d03e  772e                           ja	0x14198d06e <.text+0x198c06e>
// 0x14198d040  4885d2                         testq	%rdx, %rdx
// 0x14198d043  7429                           je	0x14198d06e <.text+0x198c06e>
// 0x14198d045  83fa01                         cmpl	$0x1, %edx
// 0x14198d048  48898d30010000                 movq	%rcx, 0x130(%rbp)
// 0x14198d04f  4c89ce                         movq	%r9, %rsi
// 0x14198d052  488b5008                       movq	0x8(%rax), %rdx
// 0x14198d056  488995d8000000                 movq	%rdx, 0xd8(%rbp)
// Frames: F3
// 0x14198d05d  498b08                         movq	(%r8), %rcx
// 0x14198d060  e89bd4cdfe                     callq	0x14066a500 <.text+0x669500>
// 0x14198d065  4989c2                         movq	%rax, %r10
// 0x14198d068  49c1ea20                       shrq	$0x20, %r10
// 0x14198d06c  7532                           jne	0x14198d0a0 <.text+0x198c0a0>
// Frames: F4
// 0x14198d06e  0f28b520020000                 movaps	0x220(%rbp), %xmm6
// 0x14198d075  0f28bd30020000                 movaps	0x230(%rbp), %xmm7
// 0x14198d07c  440f288540020000               movaps	0x240(%rbp), %xmm8
// 0x14198d084  440f288d50020000               movaps	0x250(%rbp), %xmm9
// 0x14198d08c  4881c4e8020000                 addq	$0x2e8, %rsp            # imm = 0x2E8
// 0x14198d093  5b                             popq	%rbx
// 0x14198d094  5f                             popq	%rdi
// 0x14198d095  5e                             popq	%rsi
// 0x14198d096  415c                           popq	%r12
// 0x14198d098  415d                           popq	%r13
// 0x14198d09a  415e                           popq	%r14
// 0x14198d09c  415f                           popq	%r15
// 0x14198d09e  5d                             popq	%rbp
// 0x14198d09f  c3                             retq
// Frames: F5 -> F6
// 0x14198d0a0  488b4e08                       movq	0x8(%rsi), %rcx
// Frames: F7 -> F8 -> F9 -> F6
// 0x14198d0a4  4189c0                         movl	%eax, %r8d
// 0x14198d0a7  ba01000000                     movl	$0x1, %edx
// Frames: F10 -> F11 -> F7 -> F8 -> F9 -> F6
// 0x14198d0ac  4c3b4110                       cmpq	0x10(%rcx), %r8
// Frames: F7 -> F8 -> F9 -> F6
// 0x14198d0b0  0f83b30a0000                   jae	0x14198db69 <.text+0x198cb69>
// Frames: F12 -> F13 -> F14 -> F8 -> F9 -> F6
// 0x14198d0b6  4f8d0480                       leaq	(%r8,%r8,4), %r8
// 0x14198d0ba  49c1e002                       shlq	$0x2, %r8
// 0x14198d0be  4c034108                       addq	0x8(%rcx), %r8
// 0x14198d0c2  453910                         cmpl	%r10d, (%r8)
// Frames: F14 -> F8 -> F9 -> F6
// 0x14198d0c5  0f859e0a0000                   jne	0x14198db69 <.text+0x198cb69>
// Frames: F15 -> F16 -> F8 -> F9 -> F6
// 0x14198d0cb  458b5004                       movl	0x4(%r8), %r10d
// 0x14198d0cf  41bbffffffff                   movl	$0xffffffff, %r11d      # imm = 0xFFFFFFFF
// 0x14198d0d5  4d39da                         cmpq	%r11, %r10
// Frames: F16 -> F8 -> F9 -> F6
// 0x14198d0d8  0f848b0a0000                   je	0x14198db69 <.text+0x198cb69>
// Frames: F17 -> F18 -> F9 -> F6
// 0x14198d0de  4989f1                         movq	%rsi, %r9
// 0x14198d0e1  488b16                         movq	(%rsi), %rdx
// 0x14198d0e4  4c395238                       cmpq	%r10, 0x38(%rdx)
// Frames: F19 -> F20 -> F18 -> F9 -> F6
// 0x14198d0e8  0f86790a0000                   jbe	0x14198db67 <.text+0x198cb67>
// Frames: F18 -> F9 -> F6
// 0x14198d0ee  4c8b4a28                       movq	0x28(%rdx), %r9
// Frames: F21 -> F22 -> F23 -> F24 -> F19 -> F20 -> F18 -> F9 -> F6
// 0x14198d0f2  4589d3                         movl	%r10d, %r11d
// 0x14198d0f5  41c1eb06                       shrl	$0x6, %r11d
// Frames: F25 -> F23 -> F24 -> F19 -> F20 -> F18 -> F9 -> F6
// 0x14198d0f9  4f8b0cd9                       movq	(%r9,%r11,8), %r9
// Frames: F23 -> F24 -> F19 -> F20 -> F18 -> F9 -> F6
// 0x14198d0fd  4d0fa3d1                       btq	%r10, %r9
// Frames: F18 -> F9 -> F6
// 0x14198d101  0f83600a0000                   jae	0x14198db67 <.text+0x198cb67>
// Frames: F26 -> F8 -> F9 -> F6
// 0x14198d107  4c8bb508030000                 movq	0x308(%rbp), %r14
// 0x14198d10e  488bbdf0020000                 movq	0x2f0(%rbp), %rdi
// 0x14198d115  498b4008                       movq	0x8(%r8), %rax
// 0x14198d119  458b4010                       movl	0x10(%r8), %r8d
// 0x14198d11d  48c1e820                       shrq	$0x20, %rax
// Frames: F27 -> F28 -> F9 -> F6
// 0x14198d121  488b8928010000                 movq	0x128(%rcx), %rcx
// Frames: F29 -> F9 -> F6
// 0x14198d128  488b92d8000000                 movq	0xd8(%rdx), %rdx
// Frames: F30 -> F31 -> F32 -> F29 -> F9 -> F6
// 0x14198d12f  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F33 -> F30 -> F31 -> F32 -> F29 -> F9 -> F6
// 0x14198d133  4c8b4cc118                     movq	0x18(%rcx,%rax,8), %r9
// 0x14198d138  488b44c138                     movq	0x38(%rcx,%rax,8), %rax
// Frames: F34 -> F35 -> F36 -> F37 -> F33 -> F30 -> F31 -> F32 -> F29 -> F9 -> F6
// 0x14198d13d  488b04d0                       movq	(%rax,%rdx,8), %rax
// Frames: F38 -> F39 -> F36 -> F33 -> F30 -> F31 -> F32 -> F29 -> F9 -> F6
// 0x14198d141  48f7d0                         notq	%rax
// Frames: F40 -> F41 -> F42 -> F32 -> F29 -> F9 -> F6
// 0x14198d144  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x14198d148  48c1e005                       shlq	$0x5, %rax
// Frames: F21 -> F43 -> F44 -> F45 -> F9 -> F6
// 0x14198d14c  4b8d3480                       leaq	(%r8,%r8,4), %rsi
// 0x14198d150  48c1e604                       shlq	$0x4, %rsi
// 0x14198d154  4903740110                     addq	0x10(%r9,%rax), %rsi
// Frames: F46
// 0x14198d159  4889f1                         movq	%rsi, %rcx
// 0x14198d15c  e86fe5c0fe                     callq	0x14059b6d0 <.text+0x59a6d0>
// 0x14198d161  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x14198d168  488d8dbc000000                 leaq	0xbc(%rbp), %rcx
// 0x14198d16f  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x14198d176  4989f0                         movq	%rsi, %r8
// 0x14198d179  e812e6c0fe                     callq	0x14059b790 <.text+0x59a790>
// Frames: F47
// 0x14198d17e  83bdbc00000000                 cmpl	$0x0, 0xbc(%rbp)
// 0x14198d185  f20f10bdc0000000               movsd	0xc0(%rbp), %xmm7
// 0x14198d18d  f3440f1085c8000000             movss	0xc8(%rbp), %xmm8
// 0x14198d196  7424                           je	0x14198d1bc <.text+0x198c1bc>
// Frames: F48
// 0x14198d198  f20f1085cc000000               movsd	0xcc(%rbp), %xmm0
// Frames: F49 -> F50 -> F48
// 0x14198d1a0  0f58f8                         addps	%xmm0, %xmm7
// Frames: F49 -> F51 -> F48
// 0x14198d1a3  f3440f5885d4000000             addss	0xd4(%rbp), %xmm8
// Frames: F52 -> F53 -> F48
// 0x14198d1ac  0f593d3daa8300                 mulps	0x83aa3d(%rip), %xmm7   # 0x1421c7bf0
// Frames: F52 -> F54 -> F48
// 0x14198d1b3  f3440f590548d27b00             mulss	0x7bd248(%rip), %xmm8   # 0x14214a404
// Frames: F49 -> F50 -> F55
// 0x14198d1bc  0f28053d67d600                 movaps	0xd6673d(%rip), %xmm0   # 0x1426f3900
// 0x14198d1c3  0f58c7                         addps	%xmm7, %xmm0
// 0x14198d1c6  450f57c9                       xorps	%xmm9, %xmm9
// Frames: F49 -> F51 -> F55
// 0x14198d1ca  410f28c8                       movaps	%xmm8, %xmm1
// 0x14198d1ce  f3410f58c9                     addss	%xmm9, %xmm1
// Frames: F56 -> F55
// 0x14198d1d3  0f138500020000                 movlps	%xmm0, 0x200(%rbp)
// 0x14198d1da  f30f118d08020000               movss	%xmm1, 0x208(%rbp)
// Frames: F57
// 0x14198d1e2  4c8b2f                         movq	(%rdi), %r13
// 0x14198d1e5  498d95c0030000                 leaq	0x3c0(%r13), %rdx
// 0x14198d1ec  488dbd60010000                 leaq	0x160(%rbp), %rdi
// 0x14198d1f3  4889f9                         movq	%rdi, %rcx
// 0x14198d1f6  e815b543ff                     callq	0x140dc8710 <.text+0xdc7710>
// 0x14198d1fb  f30f108560010000               movss	0x160(%rbp), %xmm0
// 0x14198d203  f30f109568010000               movss	0x168(%rbp), %xmm2
// 0x14198d20b  0f280dbe6f7f00                 movaps	0x7f6fbe(%rip), %xmm1   # 0x1421841d0
// Frames: F58 -> F57
// 0x14198d212  0f57ca                         xorps	%xmm2, %xmm1
// Frames: F59 -> F60 -> F61 -> F62 -> F57
// 0x14198d215  f30f59d2                       mulss	%xmm2, %xmm2
// 0x14198d219  0f28d8                         movaps	%xmm0, %xmm3
// 0x14198d21c  f30f59d8                       mulss	%xmm0, %xmm3
// 0x14198d220  f30f58da                       addss	%xmm2, %xmm3
// Frames: F63 -> F64 -> F60 -> F61 -> F62 -> F57
// 0x14198d224  f30f51db                       sqrtss	%xmm3, %xmm3
// 0x14198d228  f30f103590b57b00               movss	0x7bb590(%rip), %xmm6   # 0x1421487c0
// Frames: F65 -> F61 -> F62 -> F57
// 0x14198d230  0f28d6                         movaps	%xmm6, %xmm2
// 0x14198d233  f30f5ed3                       divss	%xmm3, %xmm2
// Frames: F66 -> F67 -> F62 -> F57
// 0x14198d237  f30f59ca                       mulss	%xmm2, %xmm1
// Frames: F66 -> F68 -> F62 -> F57
// 0x14198d23b  f30f59d0                       mulss	%xmm0, %xmm2
// 0x14198d23f  488d9d90000000                 leaq	0x90(%rbp), %rbx
// Frames: F57
// 0x14198d246  4889d9                         movq	%rbx, %rcx
// 0x14198d249  e812d1e4fe                     callq	0x1407da360 <.text+0x7d9360>
// Frames: F49 -> F51 -> F69
// 0x14198d24e  f3440f588d08020000             addss	0x208(%rbp), %xmm9
// Frames: F70
// 0x14198d257  0f288590000000                 movaps	0x90(%rbp), %xmm0
// Frames: F69
// 0x14198d25e  f20f108d00020000               movsd	0x200(%rbp), %xmm1
// Frames: F49 -> F50 -> F69
// 0x14198d266  0f580d03a78a00                 addps	0x8aa703(%rip), %xmm1   # 0x142237970
// Frames: F71
// 0x14198d26d  0f138d70010000                 movlps	%xmm1, 0x170(%rbp)
// 0x14198d274  f3440f118d78010000             movss	%xmm9, 0x178(%rbp)
// 0x14198d27d  0f298560010000                 movaps	%xmm0, 0x160(%rbp)
// 0x14198d284  48b80000803f0000803f           movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
// 0x14198d28e  4889857c010000                 movq	%rax, 0x17c(%rbp)
// 0x14198d295  c785840100000000803f           movl	$0x3f800000, 0x184(%rbp) # imm = 0x3F800000
// Frames: F72
// 0x14198d29f  48897c2420                     movq	%rdi, 0x20(%rsp)
// 0x14198d2a4  488d8da0000000                 leaq	0xa0(%rbp), %rcx
// 0x14198d2ab  f30f100dcdda8200               movss	0x82dacd(%rip), %xmm1   # 0x1421bad80
// 0x14198d2b3  f30f101d49d17b00               movss	0x7bd149(%rip), %xmm3   # 0x14214a404
// 0x14198d2bb  0f28d6                         movaps	%xmm6, %xmm2
// 0x14198d2be  e83d46bcfe                     callq	0x140551900 <.text+0x550900>
// Frames: F73
// 0x14198d2c3  498b06                         movq	(%r14), %rax
// 0x14198d2c6  0fb64832                       movzbl	0x32(%rax), %ecx
// 0x14198d2ca  c6851702000001                 movb	$0x1, 0x217(%rbp)
// 0x14198d2d1  e87a9ec0fe                     callq	0x140597150 <.text+0x596150>
// Frames: F74
// 0x14198d2d6  84c0                           testb	%al, %al
// 0x14198d2d8  7412                           je	0x14198d2ec <.text+0x198c2ec>
// Frames: F75
// 0x14198d2da  c6851702000001                 movb	$0x1, 0x217(%rbp)
// 0x14198d2e1  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x14198d2e5  e8b6f0d2fe                     callq	0x1406bc3a0 <.text+0x6bb3a0>
// 0x14198d2ea  eb36                           jmp	0x14198d322 <.text+0x198c322>
// Frames: F76
// 0x14198d2ec  c6851702000001                 movb	$0x1, 0x217(%rbp)
// 0x14198d2f3  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x14198d2fa  e871f0d2fe                     callq	0x1406bc370 <.text+0x6bb370>
// 0x14198d2ff  c6851702000001                 movb	$0x1, 0x217(%rbp)
// 0x14198d306  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x14198d30a  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x14198d311  41b801000000                   movl	$0x1, %r8d
// 0x14198d317  41b902000000                   movl	$0x2, %r9d
// 0x14198d31d  e8aef0d2fe                     callq	0x1406bc3d0 <.text+0x6bb3d0>
// Frames: F77 -> F78 -> F79
// 0x14198d322  488b85d8020000                 movq	0x2d8(%rbp), %rax
// 0x14198d329  8b481c                         movl	0x1c(%rax), %ecx
// 0x14198d32c  488b5010                       movq	0x10(%rax), %rdx
// 0x14198d330  890a                           movl	%ecx, (%rdx)
// Frames: F80 -> F79
// 0x14198d332  4c8b30                         movq	(%rax), %r14
// Frames: F81
// 0x14198d335  4889b5f0000000                 movq	%rsi, 0xf0(%rbp)
// 0x14198d33c  4c8d25ddfbffff                 leaq	-0x423(%rip), %r12      # 0x14198cf20 <.text+0x198bf20>
// 0x14198d343  4c89a5f8000000                 movq	%r12, 0xf8(%rbp)
// Frames: F82 -> F83 -> F84 -> F81
// 0x14198d34a  488d05f766d600                 leaq	0xd666f7(%rip), %rax    # 0x1426f3a48
// 0x14198d351  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x14198d358  48c7856801000001000000         movq	$0x1, 0x168(%rbp)
// 0x14198d363  488d85f0000000                 leaq	0xf0(%rbp), %rax
// 0x14198d36a  48898570010000                 movq	%rax, 0x170(%rbp)
// 0x14198d371  48c7857801000001000000         movq	$0x1, 0x178(%rbp)
// 0x14198d37c  48c7858001000000000000         movq	$0x0, 0x180(%rbp)
// 0x14198d387  c6851702000001                 movb	$0x1, 0x217(%rbp)
// 0x14198d38e  4889e9                         movq	%rbp, %rcx
// 0x14198d391  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x14198d398  e883c4edfe                     callq	0x140869820 <.text+0x868820>
// Frames: F81
// 0x14198d39d  0f104500                       movups	(%rbp), %xmm0
// 0x14198d3a1  0f298540010000                 movaps	%xmm0, 0x140(%rbp)
// 0x14198d3a8  488b4510                       movq	0x10(%rbp), %rax
// 0x14198d3ac  48898550010000                 movq	%rax, 0x150(%rbp)
// Frames: F85
// 0x14198d3b3  488b85b0000000                 movq	0xb0(%rbp), %rax
// 0x14198d3ba  48894518                       movq	%rax, 0x18(%rbp)
// 0x14198d3be  0f1085a0000000                 movups	0xa0(%rbp), %xmm0
// 0x14198d3c5  0f114508                       movups	%xmm0, 0x8(%rbp)
// Frames: F86
// 0x14198d3c9  c6450002                       movb	$0x2, (%rbp)
// Frames: F87
// 0x14198d3cd  c685f000000000                 movb	$0x0, 0xf0(%rbp)
// 0x14198d3d4  c6851602000001                 movb	$0x1, 0x216(%rbp)
// Frames: F88
// 0x14198d3db  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x14198d3e2  4889ea                         movq	%rbp, %rdx
// 0x14198d3e5  4c8d85f0000000                 leaq	0xf0(%rbp), %r8
// 0x14198d3ec  e82feed2fe                     callq	0x1406bc220 <.text+0x6bb220>
// Frames: F89
// 0x14198d3f1  0f1045c0                       movups	-0x40(%rbp), %xmm0
// 0x14198d3f5  0f104dd0                       movups	-0x30(%rbp), %xmm1
// 0x14198d3f9  0f1055e0                       movups	-0x20(%rbp), %xmm2
// 0x14198d3fd  0f105df0                       movups	-0x10(%rbp), %xmm3
// 0x14198d401  0f299d20010000                 movaps	%xmm3, 0x120(%rbp)
// 0x14198d408  0f299510010000                 movaps	%xmm2, 0x110(%rbp)
// 0x14198d40f  0f298d00010000                 movaps	%xmm1, 0x100(%rbp)
// 0x14198d416  0f2985f0000000                 movaps	%xmm0, 0xf0(%rbp)
// 0x14198d41d  c6851602000001                 movb	$0x1, 0x216(%rbp)
// Frames: F88
// 0x14198d424  4889e9                         movq	%rbp, %rcx
// 0x14198d427  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x14198d42e  4c8d85f0000000                 leaq	0xf0(%rbp), %r8
// 0x14198d435  e856eed2fe                     callq	0x1406bc290 <.text+0x6bb290>
// 0x14198d43a  c6851602000001                 movb	$0x1, 0x216(%rbp)
// 0x14198d441  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x14198d448  4889ea                         movq	%rbp, %rdx
// 0x14198d44b  4531c0                         xorl	%r8d, %r8d
// 0x14198d44e  e87deed2fe                     callq	0x1406bc2d0 <.text+0x6bb2d0>
// Frames: F79
// 0x14198d453  c6851602000000                 movb	$0x0, 0x216(%rbp)
// 0x14198d45a  488dbd40010000                 leaq	0x140(%rbp), %rdi
// 0x14198d461  4c8d8560010000                 leaq	0x160(%rbp), %r8
// 0x14198d468  4c89f1                         movq	%r14, %rcx
// 0x14198d46b  4889fa                         movq	%rdi, %rdx
// 0x14198d46e  e87d1136ff                     callq	0x140cee5f0 <.text+0xced5f0>
// 0x14198d473  488985e0000000                 movq	%rax, 0xe0(%rbp)
// 0x14198d47a  488995e8000000                 movq	%rdx, 0xe8(%rbp)
// 0x14198d481  c6851702000000                 movb	$0x0, 0x217(%rbp)
// Frames: F90
// 0x14198d488  488d8de0000000                 leaq	0xe0(%rbp), %rcx
// 0x14198d48f  e8dc8ecffe                     callq	0x140686370 <.text+0x685370>
// 0x14198d494  888560010000                   movb	%al, 0x160(%rbp)
// 0x14198d49a  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198d4a1  4c8dbd60010000                 leaq	0x160(%rbp), %r15
// 0x14198d4a8  4c89f9                         movq	%r15, %rcx
// 0x14198d4ab  e8b008befe                     callq	0x14056dd60 <.text+0x56cd60>
// 0x14198d4b0  84c0                           testb	%al, %al
// 0x14198d4b2  7517                           jne	0x14198d4cb <.text+0x198c4cb>
// 0x14198d4b4  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198d4bb  488d8de0000000                 leaq	0xe0(%rbp), %rcx
// 0x14198d4c2  e8c98ecffe                     callq	0x140686390 <.text+0x685390>
// 0x14198d4c7  3c02                           cmpb	$0x2, %al
// 0x14198d4c9  7408                           je	0x14198d4d3 <.text+0x198c4d3>
// 0x14198d4cb  f30f1035c1ef8b00               movss	0x8befc1(%rip), %xmm6   # 0x14224c494
// Frames: F91
// 0x14198d4d3  488b8510030000                 movq	0x310(%rbp), %rax
// 0x14198d4da  f30f11b510020000               movss	%xmm6, 0x210(%rbp)
// 0x14198d4e2  4c8b30                         movq	(%rax), %r14
// 0x14198d4e5  488d85d8000000                 leaq	0xd8(%rbp), %rax
// Frames: F92
// 0x14198d4ec  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x14198d4f3  4c89a568010000                 movq	%r12, 0x168(%rbp)
// 0x14198d4fa  48899d70010000                 movq	%rbx, 0x170(%rbp)
// 0x14198d501  488d0558ffeafe                 leaq	-0x11500a8(%rip), %rax  # 0x14083d460 <.text+0x83c460>
// 0x14198d508  48898578010000                 movq	%rax, 0x178(%rbp)
// 0x14198d50f  488d8500020000                 leaq	0x200(%rbp), %rax
// 0x14198d516  48898580010000                 movq	%rax, 0x180(%rbp)
// 0x14198d51d  488d053c02ebfe                 leaq	-0x114fdc4(%rip), %rax  # 0x14083d760 <.text+0x83c760>
// 0x14198d524  48898588010000                 movq	%rax, 0x188(%rbp)
// 0x14198d52b  488d8510020000                 leaq	0x210(%rbp), %rax
// 0x14198d532  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x14198d539  488d0520eaeefe                 leaq	-0x11115e0(%rip), %rax  # 0x14087bf60 <.text+0x87af60>
// 0x14198d540  48898598010000                 movq	%rax, 0x198(%rbp)
// Frames: F82 -> F83 -> F84 -> F92
// 0x14198d547  488d051a65d600                 leaq	0xd6651a(%rip), %rax    # 0x1426f3a68
// 0x14198d54e  48894500                       movq	%rax, (%rbp)
// 0x14198d552  48c7450804000000               movq	$0x4, 0x8(%rbp)
// 0x14198d55a  4c897d10                       movq	%r15, 0x10(%rbp)
// 0x14198d55e  48c7451804000000               movq	$0x4, 0x18(%rbp)
// 0x14198d566  48c7452000000000               movq	$0x0, 0x20(%rbp)
// 0x14198d56e  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198d575  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x14198d57c  4889ea                         movq	%rbp, %rdx
// 0x14198d57f  e89cc2edfe                     callq	0x140869820 <.text+0x868820>
// Frames: F92
// 0x14198d584  488b85f0000000                 movq	0xf0(%rbp), %rax
// 0x14198d58b  488985f8010000                 movq	%rax, 0x1f8(%rbp)
// 0x14198d592  4c8ba5f8000000                 movq	0xf8(%rbp), %r12
// 0x14198d599  4c8bbd00010000                 movq	0x100(%rbp), %r15
// Frames: F93
// 0x14198d5a0  0f28b590000000                 movaps	0x90(%rbp), %xmm6
// 0x14198d5a7  f3440f108d10020000             movss	0x210(%rbp), %xmm9
// 0x14198d5b0  488b8500020000                 movq	0x200(%rbp), %rax
// 0x14198d5b7  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x14198d5be  8b8508020000                   movl	0x208(%rbp), %eax
// 0x14198d5c4  898568010000                   movl	%eax, 0x168(%rbp)
// Frames: F94 -> F95 -> F96 -> F97 -> F98 -> F99 -> F100 -> F93
// 0x14198d5ca  0fb605f0d42e01                 movzbl	0x12ed4f0(%rip), %eax   # 0x142c7aac1
// Frames: F101 -> F96 -> F97 -> F98 -> F99 -> F100 -> F93
// 0x14198d5d1  b930000000                     movl	$0x30, %ecx
// 0x14198d5d6  ba10000000                     movl	$0x10, %edx
// 0x14198d5db  e8a0d1a4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F98 -> F99 -> F100 -> F93
// 0x14198d5e0  4885c0                         testq	%rax, %rax
// 0x14198d5e3  4c89a538010000                 movq	%r12, 0x138(%rbp)
// 0x14198d5ea  0f84b2050000                   je	0x14198dba2 <.text+0x198cba2>
// Frames: F99 -> F100 -> F93
// 0x14198d5f0  48c70001000000                 movq	$0x1, (%rax)
// 0x14198d5f7  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x14198d5ff  0f297010                       movaps	%xmm6, 0x10(%rax)
// 0x14198d603  f3440f114820                   movss	%xmm9, 0x20(%rax)
// 0x14198d609  488b8d60010000                 movq	0x160(%rbp), %rcx
// 0x14198d610  48894824                       movq	%rcx, 0x24(%rax)
// 0x14198d614  8b8d68010000                   movl	0x168(%rbp), %ecx
// 0x14198d61a  89482c                         movl	%ecx, 0x2c(%rax)
// Frames: F102
// 0x14198d61d  488d0d8464d600                 leaq	0xd66484(%rip), %rcx    # 0x1426f3aa8
// 0x14198d624  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x14198d629  488d9d60010000                 leaq	0x160(%rbp), %rbx
// 0x14198d630  4889d9                         movq	%rbx, %rcx
// 0x14198d633  4c89e2                         movq	%r12, %rdx
// 0x14198d636  4d89f8                         movq	%r15, %r8
// 0x14198d639  4989c1                         movq	%rax, %r9
// 0x14198d63c  e81f95ccfe                     callq	0x140656b60 <.text+0x655b60>
// Frames: F91
// 0x14198d641  488b8580010000                 movq	0x180(%rbp), %rax
// 0x14198d648  48894520                       movq	%rax, 0x20(%rbp)
// 0x14198d64c  0f108560010000                 movups	0x160(%rbp), %xmm0
// 0x14198d653  0f108d70010000                 movups	0x170(%rbp), %xmm1
// 0x14198d65a  0f294d10                       movaps	%xmm1, 0x10(%rbp)
// 0x14198d65e  0f294500                       movaps	%xmm0, (%rbp)
// 0x14198d662  4889ea                         movq	%rbp, %rdx
// 0x14198d665  4c89f1                         movq	%r14, %rcx
// 0x14198d668  e8f32336ff                     callq	0x140cefa60 <.text+0xceea60>
// Frames: F103 -> F104 -> F105 -> F105 -> F105 -> F106
// 0x14198d66d  488b95f8010000                 movq	0x1f8(%rbp), %rdx
// 0x14198d674  4885d2                         testq	%rdx, %rdx
// 0x14198d677  7412                           je	0x14198d68b <.text+0x198c68b>
// Frames: F107 -> F108 -> F109 -> F105 -> F105 -> F105 -> F106
// 0x14198d679  41b801000000                   movl	$0x1, %r8d
// 0x14198d67f  488b8d38010000                 movq	0x138(%rbp), %rcx
// 0x14198d686  e805d1a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F110
// 0x14198d68b  f30f107640                     movss	0x40(%rsi), %xmm6
// 0x14198d690  c6851702000000                 movb	$0x0, 0x217(%rbp)
// Frames: F111
// 0x14198d697  488d8de0000000                 leaq	0xe0(%rbp), %rcx
// 0x14198d69e  e8ed8ccffe                     callq	0x140686390 <.text+0x685390>
// 0x14198d6a3  488b8d00030000                 movq	0x300(%rbp), %rcx
// 0x14198d6aa  4c8bb5f8020000                 movq	0x2f8(%rbp), %r14
// 0x14198d6b1  4c8bbde8020000                 movq	0x2e8(%rbp), %r15
// 0x14198d6b8  04fe                           addb	$-0x2, %al
// 0x14198d6ba  0fb6c0                         movzbl	%al, %eax
// 0x14198d6bd  3c03                           cmpb	$0x3, %al
// 0x14198d6bf  ba03000000                     movl	$0x3, %edx
// 0x14198d6c4  0f42d0                         cmovbl	%eax, %edx
// 0x14198d6c7  0fb6c2                         movzbl	%dl, %eax
// 0x14198d6ca  488d154b64d600                 leaq	0xd6644b(%rip), %rdx    # 0x1426f3b1c
// 0x14198d6d1  48630482                       movslq	(%rdx,%rax,4), %rax
// 0x14198d6d5  4801d0                         addq	%rdx, %rax
// 0x14198d6d8  ffe0                           jmpq	*%rax
// Frames: F112 -> F113
// 0x14198d6da  488b11                         movq	(%rcx), %rdx
// 0x14198d6dd  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198d6e4  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x14198d6eb  e88070c5fe                     callq	0x1405e4770 <.text+0x5e3770>
// Frames: F114 -> F113
// 0x14198d6f0  498b06                         movq	(%r14), %rax
// 0x14198d6f3  f30f1008                       movss	(%rax), %xmm1
// 0x14198d6f7  f30f105004                     movss	0x4(%rax), %xmm2
// 0x14198d6fc  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198d703  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x14198d70a  e89170c5fe                     callq	0x1405e47a0 <.text+0x5e37a0>
// Frames: F115 -> F113
// 0x14198d70f  0f28d1                         movaps	%xmm1, %xmm2
// 0x14198d712  f30f11bd60010000               movss	%xmm7, 0x160(%rbp)
// 0x14198d71a  f30f11b564010000               movss	%xmm6, 0x164(%rbp)
// 0x14198d722  f3440f118568010000             movss	%xmm8, 0x168(%rbp)
// 0x14198d72b  48b8000000000000803f           movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
// 0x14198d735  4889856c010000                 movq	%rax, 0x16c(%rbp)
// 0x14198d73c  c7857401000000000000           movl	$0x0, 0x174(%rbp)
// Frames: F116 -> F115 -> F113
// 0x14198d746  f20f100db25b8e00               movsd	0x8e5bb2(%rip), %xmm1   # 0x142273300
// 0x14198d74e  f20f118d40010000               movsd	%xmm1, 0x140(%rbp)
// 0x14198d756  c6854801000000                 movb	$0x0, 0x148(%rbp)
// 0x14198d75d  c6851702000000                 movb	$0x0, 0x217(%rbp)
// Frames: F117 -> F113
// 0x14198d764  48897c2430                     movq	%rdi, 0x30(%rsp)
// 0x14198d769  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x14198d76e  4c896c2420                     movq	%r13, 0x20(%rsp)
// 0x14198d773  4889e9                         movq	%rbp, %rcx
// 0x14198d776  4c8d8df0000000                 leaq	0xf0(%rbp), %r9
// 0x14198d77d  0f28c8                         movaps	%xmm0, %xmm1
// 0x14198d780  e86b70c5fe                     callq	0x1405e47f0 <.text+0x5e37f0>
// Frames: F118 -> F119 -> F120 -> F113
// 0x14198d785  488b8518030000                 movq	0x318(%rbp), %rax
// 0x14198d78c  f30f104500                     movss	(%rbp), %xmm0
// Frames: F118 -> F121 -> F120 -> F113
// 0x14198d791  f30f104d04                     movss	0x4(%rbp), %xmm1
// Frames: F118 -> F122 -> F120 -> F113
// 0x14198d796  f30f105508                     movss	0x8(%rbp), %xmm2
// Frames: F118 -> F119 -> F120 -> F113
// 0x14198d79b  f30f12df                       movsldup	%xmm7, %xmm3            # xmm3 = xmm7[0,0,2,2]
// 0x14198d79f  0f28e3                         movaps	%xmm3, %xmm4
// 0x14198d7a2  f30f10e0                       movss	%xmm0, %xmm4            # xmm4 = xmm0[0],xmm4[1,2,3]
// 0x14198d7a6  0f5ce3                         subps	%xmm3, %xmm4
// Frames: F118 -> F121 -> F120 -> F113
// 0x14198d7a9  f30f10f9                       movss	%xmm1, %xmm7            # xmm7 = xmm1[0],xmm7[1,2,3]
// 0x14198d7ad  f30f12c6                       movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
// 0x14198d7b1  0f5cf8                         subps	%xmm0, %xmm7
// Frames: F118 -> F122 -> F120 -> F113
// 0x14198d7b4  410f14d0                       unpcklps	%xmm8, %xmm2            # xmm2 = xmm2[0],xmm8[0],xmm2[1],xmm8[1]
// 0x14198d7b8  f3410f12c0                     movsldup	%xmm8, %xmm0            # xmm0 = xmm8[0,0,2,2]
// 0x14198d7bd  0f5cd0                         subps	%xmm0, %xmm2
// Frames: F123 -> F124 -> F120 -> F113
// 0x14198d7c0  0f59e4                         mulps	%xmm4, %xmm4
// 0x14198d7c3  0f59ff                         mulps	%xmm7, %xmm7
// 0x14198d7c6  0f58fc                         addps	%xmm4, %xmm7
// 0x14198d7c9  0f59d2                         mulps	%xmm2, %xmm2
// 0x14198d7cc  0f58d7                         addps	%xmm7, %xmm2
// Frames: F63 -> F125 -> F124 -> F120 -> F113
// 0x14198d7cf  0f51c2                         sqrtps	%xmm2, %xmm0
// Frames: F126
// 0x14198d7d2  f30f16c8                       movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
// 0x14198d7d6  f30f5cc1                       subss	%xmm1, %xmm0
// 0x14198d7da  488b8d30010000                 movq	0x130(%rbp), %rcx
// 0x14198d7e1  f30f1101                       movss	%xmm0, (%rcx)
// Frames: F77 -> F78 -> F127
// 0x14198d7e5  448b601c                       movl	0x1c(%rax), %r12d
// 0x14198d7e9  4c8b6810                       movq	0x10(%rax), %r13
// 0x14198d7ed  45896500                       movl	%r12d, (%r13)
// Frames: F80 -> F127
// 0x14198d7f1  488b38                         movq	(%rax), %rdi
// Frames: F128
// 0x14198d7f4  488b1e                         movq	(%rsi), %rbx
// 0x14198d7f7  c6851702000000                 movb	$0x0, 0x217(%rbp)
// Frames: F129 -> F128
// 0x14198d7fe  4889d9                         movq	%rbx, %rcx
// 0x14198d801  e87ad3c2fe                     callq	0x1405bab80 <.text+0x5b9b80>
// Frames: F130
// 0x14198d806  4989d0                         movq	%rdx, %r8
// 0x14198d809  488b8d20030000                 movq	0x320(%rbp), %rcx
// 0x14198d810  488b95d0020000                 movq	0x2d0(%rbp), %rdx
// 0x14198d817  4c8b31                         movq	(%rcx), %r14
// 0x14198d81a  c6851702000000                 movb	$0x0, 0x217(%rbp)
// Frames: F127
// 0x14198d821  488d0db062d600                 leaq	0xd662b0(%rip), %rcx    # 0x1426f3ad8
// 0x14198d828  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x14198d82d  4889542420                     movq	%rdx, 0x20(%rsp)
// 0x14198d832  48c744243009000000             movq	$0x9, 0x30(%rsp)
// 0x14198d83b  4889f9                         movq	%rdi, %rcx
// 0x14198d83e  4889c2                         movq	%rax, %rdx
// 0x14198d841  4d89f1                         movq	%r14, %r9
// 0x14198d844  e84746ffff                     callq	0x141981e90 <.text+0x1980e90>
// Frames: F77 -> F78 -> F131
// 0x14198d849  45896500                       movl	%r12d, (%r13)
// 0x14198d84d  c6851702000000                 movb	$0x0, 0x217(%rbp)
// Frames: F131
// 0x14198d854  4889f9                         movq	%rdi, %rcx
// 0x14198d857  4889da                         movq	%rbx, %rdx
// 0x14198d85a  4d89f0                         movq	%r14, %r8
// 0x14198d85d  e81ec4c2fe                     callq	0x1405b9c80 <.text+0x5b8c80>
// Frames: F132
// 0x14198d862  488b06                         movq	(%rsi), %rax
// Frames: F133
// 0x14198d865  0f104630                       movups	0x30(%rsi), %xmm0
// 0x14198d869  0f11856c010000                 movups	%xmm0, 0x16c(%rbp)
// 0x14198d870  0f104624                       movups	0x24(%rsi), %xmm0
// 0x14198d874  0f298560010000                 movaps	%xmm0, 0x160(%rbp)
// Frames: F77 -> F78 -> F134
// 0x14198d87b  418b4f1c                       movl	0x1c(%r15), %ecx
// 0x14198d87f  498b5710                       movq	0x10(%r15), %rdx
// 0x14198d883  890a                           movl	%ecx, (%rdx)
// Frames: F80 -> F134
// 0x14198d885  498b0f                         movq	(%r15), %rcx
// Frames: F134
// 0x14198d888  c70100000000                   movl	$0x0, (%rcx)
// 0x14198d88e  48894110                       movq	%rax, 0x10(%rcx)
// 0x14198d892  0f288560010000                 movaps	0x160(%rbp), %xmm0
// 0x14198d899  0f114118                       movups	%xmm0, 0x18(%rcx)
// 0x14198d89d  0f10856c010000                 movups	0x16c(%rbp), %xmm0
// 0x14198d8a4  0f114124                       movups	%xmm0, 0x24(%rcx)
// 0x14198d8a8  e9c1f7ffff                     jmp	0x14198d06e <.text+0x198c06e>
// Frames: F112 -> F135
// 0x14198d8ad  488b11                         movq	(%rcx), %rdx
// 0x14198d8b0  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198d8b7  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x14198d8be  e8ad6ec5fe                     callq	0x1405e4770 <.text+0x5e3770>
// Frames: F114 -> F135
// 0x14198d8c3  498b06                         movq	(%r14), %rax
// 0x14198d8c6  f30f1008                       movss	(%rax), %xmm1
// 0x14198d8ca  f30f105004                     movss	0x4(%rax), %xmm2
// 0x14198d8cf  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198d8d6  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x14198d8dd  e8be6ec5fe                     callq	0x1405e47a0 <.text+0x5e37a0>
// Frames: F115 -> F135
// 0x14198d8e2  0f28d1                         movaps	%xmm1, %xmm2
// 0x14198d8e5  f30f11bd60010000               movss	%xmm7, 0x160(%rbp)
// 0x14198d8ed  f30f11b564010000               movss	%xmm6, 0x164(%rbp)
// 0x14198d8f5  f3440f118568010000             movss	%xmm8, 0x168(%rbp)
// 0x14198d8fe  48b8000000000000803f           movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
// 0x14198d908  4889856c010000                 movq	%rax, 0x16c(%rbp)
// 0x14198d90f  c7857401000000000000           movl	$0x0, 0x174(%rbp)
// Frames: F116 -> F115 -> F135
// 0x14198d919  f20f100ddf598e00               movsd	0x8e59df(%rip), %xmm1   # 0x142273300
// 0x14198d921  f20f118d40010000               movsd	%xmm1, 0x140(%rbp)
// 0x14198d929  c6854801000000                 movb	$0x0, 0x148(%rbp)
// 0x14198d930  c6851702000000                 movb	$0x0, 0x217(%rbp)
// Frames: F117 -> F135
// 0x14198d937  48897c2430                     movq	%rdi, 0x30(%rsp)
// 0x14198d93c  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x14198d941  4c896c2420                     movq	%r13, 0x20(%rsp)
// 0x14198d946  4889e9                         movq	%rbp, %rcx
// 0x14198d949  4c8d8df0000000                 leaq	0xf0(%rbp), %r9
// 0x14198d950  0f28c8                         movaps	%xmm0, %xmm1
// 0x14198d953  e8986ec5fe                     callq	0x1405e47f0 <.text+0x5e37f0>
// 0x14198d958  f30f104500                     movss	(%rbp), %xmm0
// 0x14198d95d  f30f104d04                     movss	0x4(%rbp), %xmm1
// 0x14198d962  f30f105508                     movss	0x8(%rbp), %xmm2
// Frames: F118 -> F119 -> F120 -> F135
// 0x14198d967  f30f5cc7                       subss	%xmm7, %xmm0
// Frames: F118 -> F121 -> F120 -> F135
// 0x14198d96b  f30f5cce                       subss	%xmm6, %xmm1
// Frames: F118 -> F122 -> F120 -> F135
// 0x14198d96f  f3410f5cd0                     subss	%xmm8, %xmm2
// Frames: F123 -> F124 -> F120 -> F135
// 0x14198d974  f30f59c0                       mulss	%xmm0, %xmm0
// 0x14198d978  f30f59c9                       mulss	%xmm1, %xmm1
// 0x14198d97c  f30f58c8                       addss	%xmm0, %xmm1
// 0x14198d980  f30f59d2                       mulss	%xmm2, %xmm2
// 0x14198d984  f30f58d1                       addss	%xmm1, %xmm2
// Frames: F63 -> F125 -> F124 -> F120 -> F135
// 0x14198d988  f30f51d2                       sqrtss	%xmm2, %xmm2
// 0x14198d98c  488b8530010000                 movq	0x130(%rbp), %rax
// Frames: F135
// 0x14198d993  f30f5c10                       subss	(%rax), %xmm2
// 0x14198d997  c6851702000000                 movb	$0x0, 0x217(%rbp)
// Frames: F136
// 0x14198d99e  f30f10059e977f00               movss	0x7f979e(%rip), %xmm0   # 0x142187144
// 0x14198d9a6  f30f100d96288200               movss	0x822896(%rip), %xmm1   # 0x1421b0244
// 0x14198d9ae  e8cdbde4fe                     callq	0x1407d9780 <.text+0x7d8780>
// Frames: F137 -> F138
// 0x14198d9b3  488b8528030000                 movq	0x328(%rbp), %rax
// 0x14198d9ba  0f57c9                         xorps	%xmm1, %xmm1
// 0x14198d9bd  f30f5fc8                       maxss	%xmm0, %xmm1
// 0x14198d9c1  f30f1035f7ad7b00               movss	0x7badf7(%rip), %xmm6   # 0x1421487c0
// Frames: F139 -> F138
// 0x14198d9c9  f30f5df1                       minss	%xmm1, %xmm6
// Frames: F140
// 0x14198d9cd  488b3e                         movq	(%rsi), %rdi
// Frames: F141 -> F78 -> F142 -> F143
// 0x14198d9d0  8b481c                         movl	0x1c(%rax), %ecx
// 0x14198d9d3  488b5010                       movq	0x10(%rax), %rdx
// 0x14198d9d7  890a                           movl	%ecx, (%rdx)
// Frames: F144 -> F142 -> F143
// 0x14198d9d9  488b30                         movq	(%rax), %rsi
// Frames: F145 -> F146 -> F142 -> F143
// 0x14198d9dc  488b5630                       movq	0x30(%rsi), %rdx
// Frames: F147 -> F142 -> F143
// 0x14198d9e0  488b5e40                       movq	0x40(%rsi), %rbx
// Frames: F145 -> F146 -> F142 -> F143
// 0x14198d9e4  483b5620                       cmpq	0x20(%rsi), %rdx
// 0x14198d9e8  750d                           jne	0x14198d9f7 <.text+0x198c9f7>
// Frames: F146 -> F142 -> F143
// 0x14198d9ea  488d4e20                       leaq	0x20(%rsi), %rcx
// Frames: F148 -> F146 -> F142 -> F143
// 0x14198d9ee  e88dc48efe                     callq	0x140279e80 <.text+0x278e80>
// Frames: F149 -> F146 -> F142 -> F143
// 0x14198d9f3  488b5630                       movq	0x30(%rsi), %rdx
// Frames: F150 -> F151 -> F149 -> F146 -> F142 -> F143
// 0x14198d9f7  488b4628                       movq	0x28(%rsi), %rax
// Frames: F152 -> F149 -> F146 -> F142 -> F143
// 0x14198d9fb  48c1e206                       shlq	$0x6, %rdx
// Frames: F153 -> F154 -> F146 -> F142 -> F143
// 0x14198d9ff  c7041000000000                 movl	$0x0, (%rax,%rdx)
// 0x14198da06  c744100801000000               movl	$0x1, 0x8(%rax,%rdx)
// 0x14198da0e  f30f1174100c                   movss	%xmm6, 0xc(%rax,%rdx)
// 0x14198da14  c744101000000000               movl	$0x0, 0x10(%rax,%rdx)
// 0x14198da1c  c744101800000000               movl	$0x0, 0x18(%rax,%rdx)
// 0x14198da24  48897c1020                     movq	%rdi, 0x20(%rax,%rdx)
// 0x14198da29  c744102800000000               movl	$0x0, 0x28(%rax,%rdx)
// 0x14198da31  c644103400                     movb	$0x0, 0x34(%rax,%rdx)
// 0x14198da36  48895c1038                     movq	%rbx, 0x38(%rax,%rdx)
// Frames: F155 -> F146 -> F142 -> F143
// 0x14198da3b  48ff4630                       incq	0x30(%rsi)
// Frames: F156 -> F142 -> F143
// 0x14198da3f  48ff4640                       incq	0x40(%rsi)
// 0x14198da43  e926f6ffff                     jmp	0x14198d06e <.text+0x198c06e>
// Frames: F157
// 0x14198da48  498b07                         movq	(%r15), %rax
// Frames: F34 -> F157
// 0x14198da4b  833802                         cmpl	$0x2, (%rax)
// Frames: F157
// 0x14198da4e  0f841af6ffff                   je	0x14198d06e <.text+0x198c06e>
// Frames: F158 -> F159 -> F160
// 0x14198da54  f30f104018                     movss	0x18(%rax), %xmm0
// 0x14198da59  0f2e4624                       ucomiss	0x24(%rsi), %xmm0
// 0x14198da5d  7550                           jne	0x14198daaf <.text+0x198caaf>
// 0x14198da5f  7a4e                           jp	0x14198daaf <.text+0x198caaf>
// Frames: F161 -> F159 -> F160
// 0x14198da61  f30f10401c                     movss	0x1c(%rax), %xmm0
// 0x14198da66  0f2e4628                       ucomiss	0x28(%rsi), %xmm0
// 0x14198da6a  7543                           jne	0x14198daaf <.text+0x198caaf>
// 0x14198da6c  7a41                           jp	0x14198daaf <.text+0x198caaf>
// Frames: F162 -> F159 -> F160
// 0x14198da6e  f30f104020                     movss	0x20(%rax), %xmm0
// Frames: F163 -> F162 -> F159 -> F160
// 0x14198da73  0f2e462c                       ucomiss	0x2c(%rsi), %xmm0
// Frames: F162 -> F159 -> F160
// 0x14198da77  7536                           jne	0x14198daaf <.text+0x198caaf>
// 0x14198da79  7a34                           jp	0x14198daaf <.text+0x198caaf>
// 0x14198da7b  f30f104024                     movss	0x24(%rax), %xmm0
// 0x14198da80  0f2e4630                       ucomiss	0x30(%rsi), %xmm0
// 0x14198da84  7529                           jne	0x14198daaf <.text+0x198caaf>
// 0x14198da86  7a27                           jp	0x14198daaf <.text+0x198caaf>
// Frames: F164 -> F159 -> F160
// 0x14198da88  f30f104028                     movss	0x28(%rax), %xmm0
// 0x14198da8d  0f2e4634                       ucomiss	0x34(%rsi), %xmm0
// 0x14198da91  751c                           jne	0x14198daaf <.text+0x198caaf>
// 0x14198da93  7a1a                           jp	0x14198daaf <.text+0x198caaf>
// Frames: F165 -> F159 -> F160
// 0x14198da95  0fb64830                       movzbl	0x30(%rax), %ecx
// Frames: F166 -> F165 -> F159 -> F160
// 0x14198da99  3a4e3c                         cmpb	0x3c(%rsi), %cl
// Frames: F165 -> F159 -> F160
// 0x14198da9c  7511                           jne	0x14198daaf <.text+0x198caaf>
// Frames: F167 -> F159 -> F160
// 0x14198da9e  f30f10402c                     movss	0x2c(%rax), %xmm0
// 0x14198daa3  0f2e4638                       ucomiss	0x38(%rsi), %xmm0
// Frames: F160
// 0x14198daa7  7506                           jne	0x14198daaf <.text+0x198caaf>
// 0x14198daa9  0f8bbff5ffff                   jnp	0x14198d06e <.text+0x198c06e>
// Frames: F77 -> F78 -> F168
// 0x14198daaf  488b8de0020000                 movq	0x2e0(%rbp), %rcx
// 0x14198dab6  8b511c                         movl	0x1c(%rcx), %edx
// 0x14198dab9  4c8b4110                       movq	0x10(%rcx), %r8
// 0x14198dabd  418910                         movl	%edx, (%r8)
// Frames: F80 -> F168
// 0x14198dac0  488b39                         movq	(%rcx), %rdi
// Frames: F169
// 0x14198dac3  488b5810                       movq	0x10(%rax), %rbx
// Frames: F170
// 0x14198dac7  f20f107818                     movsd	0x18(%rax), %xmm7
// Frames: F171
// 0x14198dacc  4c8b7020                       movq	0x20(%rax), %r14
// Frames: F172
// 0x14198dad0  f20f107624                     movsd	0x24(%rsi), %xmm6
// Frames: F173
// 0x14198dad5  4c8b7e2c                       movq	0x2c(%rsi), %r15
// Frames: F174
// 0x14198dad9  f2440f104828                   movsd	0x28(%rax), %xmm9
// Frames: F175
// 0x14198dadf  f2440f104634                   movsd	0x34(%rsi), %xmm8
// Frames: F94 -> F95 -> F96 -> F97 -> F98 -> F99 -> F100 -> F168
// 0x14198dae5  0fb605d5cf2e01                 movzbl	0x12ecfd5(%rip), %eax   # 0x142c7aac1
// Frames: F101 -> F96 -> F97 -> F98 -> F99 -> F100 -> F168
// 0x14198daec  b950000000                     movl	$0x50, %ecx
// 0x14198daf1  ba08000000                     movl	$0x8, %edx
// 0x14198daf6  e885cca4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F98 -> F99 -> F100 -> F168
// 0x14198dafb  4885c0                         testq	%rax, %rax
// 0x14198dafe  0f84af000000                   je	0x14198dbb3 <.text+0x198cbb3>
// Frames: F99 -> F100 -> F168
// 0x14198db04  48c70001000000                 movq	$0x1, (%rax)
// 0x14198db0b  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x14198db13  c6401002                       movb	$0x2, 0x10(%rax)
// 0x14198db17  0fc6ffe1                       shufps	$0xe1, %xmm7, %xmm7     # xmm7 = xmm7[1,0,2,3]
// 0x14198db1b  0f137814                       movlps	%xmm7, 0x14(%rax)
// 0x14198db1f  440fc6ce14                     shufps	$0x14, %xmm6, %xmm9     # xmm9 = xmm9[0,1],xmm6[1,0]
// 0x14198db24  440f11481c                     movups	%xmm9, 0x1c(%rax)
// 0x14198db29  440f13402c                     movlps	%xmm8, 0x2c(%rax)
// 0x14198db2e  48895838                       movq	%rbx, 0x38(%rax)
// 0x14198db32  4c897040                       movq	%r14, 0x40(%rax)
// 0x14198db36  4c897848                       movq	%r15, 0x48(%rax)
// Frames: F168
// 0x14198db3a  48898568010000                 movq	%rax, 0x168(%rbp)
// 0x14198db41  48c7856001000005000000         movq	$0x5, 0x160(%rbp)
// 0x14198db4c  4c8d05955fd600                 leaq	0xd65f95(%rip), %r8     # 0x1426f3ae8
// 0x14198db53  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x14198db5a  4889f9                         movq	%rdi, %rcx
// 0x14198db5d  e8fe9ec6fe                     callq	0x1405f7a60 <.text+0x5f6a60>
// 0x14198db62  e907f5ffff                     jmp	0x14198d06e <.text+0x198c06e>
// 0x14198db67  31d2                           xorl	%edx, %edx
// Frames: F176 -> F6
// 0x14198db69  48899560010000                 movq	%rdx, 0x160(%rbp)
// 0x14198db70  48898568010000                 movq	%rax, 0x168(%rbp)
// 0x14198db77  488d05a25ed600                 leaq	0xd65ea2(%rip), %rax    # 0x1426f3a20
// 0x14198db7e  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198db83  488d0d865dd600                 leaq	0xd65d86(%rip), %rcx    # 0x1426f3910
// 0x14198db8a  4c8d0daf5dd600                 leaq	0xd65daf(%rip), %r9     # 0x1426f3940
// 0x14198db91  4c8d8560010000                 leaq	0x160(%rbp), %r8
// 0x14198db98  ba2b000000                     movl	$0x2b, %edx
// 0x14198db9d  e87e137200                     callq	0x1420aef20 <.text+0x20adf20>
// Frames: F177 -> F99 -> F100 -> F93
// 0x14198dba2  b910000000                     movl	$0x10, %ecx
// 0x14198dba7  ba30000000                     movl	$0x30, %edx
// 0x14198dbac  e8af0b7200                     callq	0x1420ae760 <.text+0x20ad760>
// 0x14198dbb1  0f0b                           ud2
// Frames: F177 -> F99 -> F100 -> F168
// 0x14198dbb3  b908000000                     movl	$0x8, %ecx
// 0x14198dbb8  ba50000000                     movl	$0x50, %edx
// 0x14198dbbd  e89e0b7200                     callq	0x1420ae760 <.text+0x20ad760>
// 0x14198dbc2  cc                             int3
// 0x14198dbc3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x14198dbd0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198dbd5  55                             pushq	%rbp
// 0x14198dbd6  4157                           pushq	%r15
// 0x14198dbd8  4156                           pushq	%r14
// 0x14198dbda  4155                           pushq	%r13
// 0x14198dbdc  4154                           pushq	%r12
// 0x14198dbde  56                             pushq	%rsi
// 0x14198dbdf  57                             pushq	%rdi
// 0x14198dbe0  53                             pushq	%rbx
// 0x14198dbe1  4883ec78                       subq	$0x78, %rsp
// 0x14198dbe5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198dbec  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x14198dbf2  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x14198dbf8  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x14198dbfd  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F103 -> F104 -> F105 -> F105 -> F105 -> F106
// 0x14198dc02  4883bdf801000000               cmpq	$0x0, 0x1f8(%rbp)
// 0x14198dc0a  7419                           je	0x14198dc25 <.text+0x198cc25>
// Frames: F107 -> F108 -> F109 -> F105 -> F105 -> F105 -> F106
// 0x14198dc0c  41b801000000                   movl	$0x1, %r8d
// 0x14198dc12  488b8d38010000                 movq	0x138(%rbp), %rcx
// 0x14198dc19  488b95f8010000                 movq	0x1f8(%rbp), %rdx
// 0x14198dc20  e86bcba4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F106
// 0x14198dc25  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198dc2c  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x14198dc31  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x14198dc36  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x14198dc3c  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x14198dc42  4883c478                       addq	$0x78, %rsp
// 0x14198dc46  5b                             popq	%rbx
// 0x14198dc47  5f                             popq	%rdi
// 0x14198dc48  5e                             popq	%rsi
// 0x14198dc49  415c                           popq	%r12
// 0x14198dc4b  415d                           popq	%r13
// 0x14198dc4d  415e                           popq	%r14
// 0x14198dc4f  415f                           popq	%r15
// 0x14198dc51  5d                             popq	%rbp
// 0x14198dc52  c3                             retq
// 0x14198dc53  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x14198dc60  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198dc65  55                             pushq	%rbp
// 0x14198dc66  4157                           pushq	%r15
// 0x14198dc68  4156                           pushq	%r14
// 0x14198dc6a  4155                           pushq	%r13
// 0x14198dc6c  4154                           pushq	%r12
// 0x14198dc6e  56                             pushq	%rsi
// 0x14198dc6f  57                             pushq	%rdi
// 0x14198dc70  53                             pushq	%rbx
// 0x14198dc71  4883ec78                       subq	$0x78, %rsp
// 0x14198dc75  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198dc7c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x14198dc82  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x14198dc88  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x14198dc8d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F178
// 0x14198dc92  80bd1602000000                 cmpb	$0x0, 0x216(%rbp)
// 0x14198dc99  744c                           je	0x14198dce7 <.text+0x198cce7>
// Frames: F105 -> F178
// 0x14198dc9b  488b9540010000                 movq	0x140(%rbp), %rdx
// Frames: F103 -> F104 -> F105 -> F105 -> F105 -> F178
// 0x14198dca2  4885d2                         testq	%rdx, %rdx
// 0x14198dca5  7412                           je	0x14198dcb9 <.text+0x198ccb9>
// Frames: F105 -> F178
// 0x14198dca7  488b8d48010000                 movq	0x148(%rbp), %rcx
// Frames: F107 -> F108 -> F109 -> F105 -> F105 -> F105 -> F178
// 0x14198dcae  41b801000000                   movl	$0x1, %r8d
// 0x14198dcb4  e8d7caa4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F178
// 0x14198dcb9  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198dcc0  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x14198dcc5  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x14198dcca  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x14198dcd0  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x14198dcd6  4883c478                       addq	$0x78, %rsp
// 0x14198dcda  5b                             popq	%rbx
// 0x14198dcdb  5f                             popq	%rdi
// 0x14198dcdc  5e                             popq	%rsi
// 0x14198dcdd  415c                           popq	%r12
// 0x14198dcdf  415d                           popq	%r13
// 0x14198dce1  415e                           popq	%r14
// 0x14198dce3  415f                           popq	%r15
// 0x14198dce5  5d                             popq	%rbp
// 0x14198dce6  c3                             retq
// 0x14198dce7  c6851702000000                 movb	$0x0, 0x217(%rbp)
// 0x14198dcee  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x14198dcf3  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x14198dcf8  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x14198dcfe  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x14198dd04  4883c478                       addq	$0x78, %rsp
// 0x14198dd08  5b                             popq	%rbx
// 0x14198dd09  5f                             popq	%rdi
// 0x14198dd0a  5e                             popq	%rsi
// 0x14198dd0b  415c                           popq	%r12
// 0x14198dd0d  415d                           popq	%r13
// 0x14198dd0f  415e                           popq	%r14
// 0x14198dd11  415f                           popq	%r15
// 0x14198dd13  5d                             popq	%rbp
// 0x14198dd14  c3                             retq
// 0x14198dd15  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x14198dd20  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198dd25  55                             pushq	%rbp
// 0x14198dd26  4157                           pushq	%r15
// 0x14198dd28  4156                           pushq	%r14
// 0x14198dd2a  4155                           pushq	%r13
// 0x14198dd2c  4154                           pushq	%r12
// 0x14198dd2e  56                             pushq	%rsi
// 0x14198dd2f  57                             pushq	%rdi
// 0x14198dd30  53                             pushq	%rbx
// 0x14198dd31  4883ec78                       subq	$0x78, %rsp
// 0x14198dd35  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198dd3c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x14198dd42  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x14198dd48  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x14198dd4d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F179
// 0x14198dd52  80bd1702000000                 cmpb	$0x0, 0x217(%rbp)
// 0x14198dd59  7426                           je	0x14198dd81 <.text+0x198cd81>
// Frames: F105 -> F179
// 0x14198dd5b  488b85a0000000                 movq	0xa0(%rbp), %rax
// Frames: F103 -> F104 -> F105 -> F105 -> F179
// 0x14198dd62  4885c0                         testq	%rax, %rax
// 0x14198dd65  741a                           je	0x14198dd81 <.text+0x198cd81>
// Frames: F105 -> F179
// 0x14198dd67  488b8da8000000                 movq	0xa8(%rbp), %rcx
// Frames: F180 -> F181 -> F104 -> F105 -> F105 -> F179
// 0x14198dd6e  48c1e002                       shlq	$0x2, %rax
// 0x14198dd72  488d14c0                       leaq	(%rax,%rax,8), %rdx
// Frames: F107 -> F108 -> F109 -> F105 -> F105 -> F179
// 0x14198dd76  41b804000000                   movl	$0x4, %r8d
// 0x14198dd7c  e80fcaa4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F1
// 0x14198dd81  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x14198dd86  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x14198dd8b  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x14198dd91  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x14198dd97  4883c478                       addq	$0x78, %rsp
// 0x14198dd9b  5b                             popq	%rbx
// 0x14198dd9c  5f                             popq	%rdi
// 0x14198dd9d  5e                             popq	%rsi
// 0x14198dd9e  415c                           popq	%r12
// 0x14198dda0  415d                           popq	%r13
// 0x14198dda2  415e                           popq	%r14
// 0x14198dda4  415f                           popq	%r15
// 0x14198dda6  5d                             popq	%rbp
// 0x14198dda7  c3                             retq
//
// RANGE rva_198ddb0_len_1d9: RVA 0x198ddb0, 473 PDB bytes; 109 instructions
// Evidence: _assembly/roof-v1/A/pack-003/rva_198ddb0_len_1d9.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3200:5896: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// Frames: F182
// 0x14198ddb0  55                             pushq	%rbp
// 0x14198ddb1  4156                           pushq	%r14
// 0x14198ddb3  56                             pushq	%rsi
// 0x14198ddb4  57                             pushq	%rdi
// 0x14198ddb5  53                             pushq	%rbx
// 0x14198ddb6  4881ec80010000                 subq	$0x180, %rsp            # imm = 0x180
// 0x14198ddbd  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14198ddc5  48c785f8000000feffffff         movq	$-0x2, 0xf8(%rbp)
// 0x14198ddd0  4c89cf                         movq	%r9, %rdi
// 0x14198ddd3  4c89c3                         movq	%r8, %rbx
// 0x14198ddd6  4989d6                         movq	%rdx, %r14
// 0x14198ddd9  4889ce                         movq	%rcx, %rsi
// Frames: F94 -> F95 -> F96 -> F97 -> F98 -> F183
// 0x14198dddc  0fb605decc2e01                 movzbl	0x12eccde(%rip), %eax   # 0x142c7aac1
// Frames: F101 -> F96 -> F97 -> F98 -> F183
// 0x14198dde3  b908000000                     movl	$0x8, %ecx
// 0x14198dde8  ba08000000                     movl	$0x8, %edx
// 0x14198dded  e88ec9a4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F98 -> F183
// 0x14198ddf2  4885c0                         testq	%rax, %rax
// 0x14198ddf5  0f842e010000                   je	0x14198df29 <.text+0x198cf29>
// Frames: F94 -> F95 -> F96 -> F97 -> F184 -> F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192 -> F193 -> F129 -> F194
// 0x14198ddfb  488985f0000000                 movq	%rax, 0xf0(%rbp)
// 0x14198de02  0fb605b8cc2e01                 movzbl	0x12eccb8(%rip), %eax   # 0x142c7aac1
// Frames: F101 -> F96 -> F97 -> F184 -> F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192 -> F193 -> F129 -> F194
// 0x14198de09  b91a000000                     movl	$0x1a, %ecx
// 0x14198de0e  ba01000000                     movl	$0x1, %edx
// 0x14198de13  e868c9a4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F195 -> F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192 -> F193 -> F129 -> F194
// 0x14198de18  4885c0                         testq	%rax, %rax
// 0x14198de1b  0f8417010000                   je	0x14198df38 <.text+0x198cf38>
// Frames: F196 -> F197 -> F198 -> F188 -> F189 -> F190 -> F191 -> F192 -> F193 -> F129 -> F194
// 0x14198de21  0f1005e25cd600                 movups	0xd65ce2(%rip), %xmm0   # 0x1426f3b0a
// 0x14198de28  0f11400a                       movups	%xmm0, 0xa(%rax)
// 0x14198de2c  0f1005cd5cd600                 movups	0xd65ccd(%rip), %xmm0   # 0x1426f3b00
// 0x14198de33  0f1100                         movups	%xmm0, (%rax)
// Frames: F183
// 0x14198de36  48c745401a000000               movq	$0x1a, 0x40(%rbp)
// 0x14198de3e  48894548                       movq	%rax, 0x48(%rbp)
// 0x14198de42  48c745501a000000               movq	$0x1a, 0x50(%rbp)
// 0x14198de4a  48b80000000000000080           movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
// 0x14198de54  48894558                       movq	%rax, 0x58(%rbp)
// 0x14198de58  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14198de5c  488d5540                       leaq	0x40(%rbp), %rdx
// 0x14198de60  e8eb8dccfe                     callq	0x140656c50 <.text+0x655c50>
// 0x14198de65  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198de69  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14198de6d  0f57d2                         xorps	%xmm2, %xmm2
// 0x14198de70  e8ab90ccfe                     callq	0x140656f20 <.text+0x655f20>
// Frames: F199
// 0x14198de75  410f2806                       movaps	(%r14), %xmm0
// Frames: F183
// 0x14198de79  0f2985d0000000                 movaps	%xmm0, 0xd0(%rbp)
// 0x14198de80  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14198de84  488d5540                       leaq	0x40(%rbp), %rdx
// 0x14198de88  4c8d85d0000000                 leaq	0xd0(%rbp), %r8
// 0x14198de8f  e82c90ccfe                     callq	0x140656ec0 <.text+0x655ec0>
// Frames: F200
// 0x14198de94  f3410f105610                   movss	0x10(%r14), %xmm2
// 0x14198de9a  f30f5915a2f18200               mulss	0x82f1a2(%rip), %xmm2   # 0x1421bd044
// Frames: F183
// 0x14198dea2  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198dea6  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14198deaa  e84190ccfe                     callq	0x140656ef0 <.text+0x655ef0>
// Frames: F201
// 0x14198deaf  418b461c                       movl	0x1c(%r14), %eax
// 0x14198deb3  8985e8000000                   movl	%eax, 0xe8(%rbp)
// 0x14198deb9  498b4614                       movq	0x14(%r14), %rax
// 0x14198debd  488985e0000000                 movq	%rax, 0xe0(%rbp)
// Frames: F183
// 0x14198dec4  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14198dec8  488d5540                       leaq	0x40(%rbp), %rdx
// 0x14198decc  4c8d85e0000000                 leaq	0xe0(%rbp), %r8
// 0x14198ded3  e8b88fccfe                     callq	0x140656e90 <.text+0x655e90>
// 0x14198ded8  488b8550010000                 movq	0x150(%rbp), %rax
// 0x14198dedf  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198dee4  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198dee8  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14198deec  4989d8                         movq	%rbx, %r8
// 0x14198deef  4989f9                         movq	%rdi, %r9
// 0x14198def2  e8e990ccfe                     callq	0x140656fe0 <.text+0x655fe0>
// 0x14198def7  488b4560                       movq	0x60(%rbp), %rax
// 0x14198defb  488b8df0000000                 movq	0xf0(%rbp), %rcx
// 0x14198df02  488901                         movq	%rax, (%rcx)
// Frames: F202 -> F203 -> F204 -> F183
// 0x14198df05  48c70601000000                 movq	$0x1, (%rsi)
// 0x14198df0c  48894e08                       movq	%rcx, 0x8(%rsi)
// 0x14198df10  48c7461001000000               movq	$0x1, 0x10(%rsi)
// Frames: F205
// 0x14198df18  4889f0                         movq	%rsi, %rax
// 0x14198df1b  4881c480010000                 addq	$0x180, %rsp            # imm = 0x180
// 0x14198df22  5b                             popq	%rbx
// 0x14198df23  5f                             popq	%rdi
// 0x14198df24  5e                             popq	%rsi
// 0x14198df25  415e                           popq	%r14
// 0x14198df27  5d                             popq	%rbp
// 0x14198df28  c3                             retq
// Frames: F177 -> F183
// 0x14198df29  b908000000                     movl	$0x8, %ecx
// 0x14198df2e  ba08000000                     movl	$0x8, %edx
// 0x14198df33  e828087200                     callq	0x1420ae760 <.text+0x20ad760>
// Frames: F206 -> F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192 -> F193 -> F129 -> F194
// 0x14198df38  b901000000                     movl	$0x1, %ecx
// 0x14198df3d  ba1a000000                     movl	$0x1a, %edx
// 0x14198df42  e819087200                     callq	0x1420ae760 <.text+0x20ad760>
// 0x14198df47  0f0b                           ud2
// 0x14198df49  0f1f8000000000                 nopl	(%rax)
// 0x14198df50  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198df55  55                             pushq	%rbp
// 0x14198df56  4156                           pushq	%r14
// 0x14198df58  56                             pushq	%rsi
// 0x14198df59  57                             pushq	%rdi
// 0x14198df5a  53                             pushq	%rbx
// 0x14198df5b  4883ec30                       subq	$0x30, %rsp
// 0x14198df5f  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F107 -> F108 -> F207 -> F105 -> F183
// 0x14198df66  ba08000000                     movl	$0x8, %edx
// 0x14198df6b  41b808000000                   movl	$0x8, %r8d
// 0x14198df71  488b8df0000000                 movq	0xf0(%rbp), %rcx
// 0x14198df78  e813c8a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F182
// 0x14198df7d  90                             nop
// 0x14198df7e  4883c430                       addq	$0x30, %rsp
// 0x14198df82  5b                             popq	%rbx
// 0x14198df83  5f                             popq	%rdi
// 0x14198df84  5e                             popq	%rsi
// 0x14198df85  415e                           popq	%r14
// 0x14198df87  5d                             popq	%rbp
// 0x14198df88  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:37:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F2: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:61:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F3: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:62:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F4: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:238:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:851:0 bevy_ecs::system::query::Query::get
// F6: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:66:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F7: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:740:0 bevy_ecs::entity::Entities::get
// F8: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:866:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F9: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:850:0 bevy_ecs::system::query::Query::get
// F10: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F11: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1488:0 core::cmp::impls::impl$24::ne
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/nonzero.rs:137:0 core::num::nonzero::impl$2::ne
// F14: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:741:0 bevy_ecs::entity::Entities::get
// F15: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:74:0 bevy_ecs::archetype::impl$24::eq
// F16: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:742:0 bevy_ecs::entity::Entities::get
// F17: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F18: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:870:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F19: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 core::bool::impl$0::then
// F20: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F22: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F23: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F24: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/bit.rs:178:0 fixedbitset::FixedBitSet::get_unchecked
// F26: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1952:0 core::result::impl$26::branch
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F28: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:886:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F29: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:888:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F30: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:693:0 bevy_ecs::storage::table::Table::get_column
// F31: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:955:0 bevy_ecs::query::fetch::impl$21::set_table
// F32: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:942:0 bevy_ecs::query::fetch::impl$21::set_archetype
// F33: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F34: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F35: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::impl$11::get::closure$0
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F37: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F38: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs:398:0 nonmax::NonMaxUsize::get
// F39: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:372:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F40: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:412:0 bevy_ecs::storage::blob_vec::BlobVec::get_slice
// F41: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:322:0 bevy_ecs::storage::table::Column::get_data_slice
// F42: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:957:0 bevy_ecs::query::fetch::impl$21::set_table
// F43: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs:506:0 bevy_ptr::ThinSlicePtr::get
// F44: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:973:0 bevy_ecs::query::fetch::impl$21::fetch
// F45: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:892:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F46: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:67:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F47: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:88:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F48: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:90:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F49: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:112:0 core::ops::arith::impl$12::add
// F50: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1083:0 glam::f32::vec3::impl$12::add
// F51: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1085:0 glam::f32::vec3::impl$12::add
// F52: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:509:0 core::ops::arith::impl$216::div
// F53: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:996:0 glam::f32::vec3::impl$4::div
// F54: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:998:0 glam::f32::vec3::impl$4::div
// F55: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:93:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F56: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1082:0 glam::f32::vec3::impl$12::add
// F57: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:95:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F58: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:816:0 glam::f32::vec2::Vec2::perp
// F59: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:158:0 glam::f32::vec2::Vec2::dot
// F60: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:381:0 glam::f32::vec2::Vec2::length
// F61: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:400:0 glam::f32::vec2::Vec2::length_recip
// F62: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:452:0 glam::f32::vec2::Vec2::normalize
// F63: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:442:0 std::f32::impl$0::sqrt
// F64: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs:188:0 glam::f32::vec2::Vec2::dot
// F65: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:868:0 glam::f32::vec2::Vec2::length
// F66: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:351:0 core::ops::arith::impl$124::mul
// F67: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:974:0 glam::f32::vec2::Vec2::length_recip
// F68: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:975:0 glam::f32::vec2::Vec2::length_recip
// F69: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:102:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F70: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:103:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F71: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:101:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F72: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:97:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F73: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:108:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F74: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:111:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F75: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:112:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F76: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:114:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F77: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$29::set_changed
// F78: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F79: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:117:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F80: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:328:0 bevy_ecs::change_detection::impl$30::deref_mut
// F81: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:118:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F82: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/fmt.rs:637:0 alloc::fmt::format::closure$0
// F83: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1179:0 core::option::Option::map_or_else
// F84: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/fmt.rs:637:0 alloc::fmt::format
// F85: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:121:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F86: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:120:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F87: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:123:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F88: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:119:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F89: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:125:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F90: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:130:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F91: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:136:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F92: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:138:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F93: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:142:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F94: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1665:0 core::ptr::read_volatile
// F95: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:98:0 alloc::alloc::alloc
// F96: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:183:0 alloc::alloc::Global::alloc_impl
// F97: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:243:0 alloc::alloc::impl$1::allocate
// F98: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:332:0 alloc::alloc::exchange_malloc
// F99: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/boxed.rs:219:0 alloc::boxed::impl$0::new
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:395:0 alloc::sync::Arc::new
// F101: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:100:0 alloc::alloc::alloc
// F102: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:137:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F103: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:280:0 alloc::raw_vec::RawVec::current_memory
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:554:0 alloc::raw_vec::impl$4::drop
// F105: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:515:0 core::ptr::drop_in_place
// F106: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:154:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F107: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:119:0 alloc::alloc::dealloc
// F108: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:256:0 alloc::alloc::impl$1::deallocate
// F109: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:555:0 alloc::raw_vec::impl$4::drop
// F110: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:157:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F111: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:159:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F112: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:76:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F113: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:164:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F114: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:77:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F115: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:80:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F116: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/range.rs:379:0 core::ops::range::RangeInclusive::new
// F117: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:79:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F118: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:221:0 core::ops::arith::impl$68::sub
// F119: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1137:0 glam::f32::vec3::impl$17::sub
// F120: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:463:0 glam::f32::vec3::Vec3::distance
// F121: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1138:0 glam::f32::vec3::impl$17::sub
// F122: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1139:0 glam::f32::vec3::impl$17::sub
// F123: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:196:0 glam::f32::vec3::Vec3::dot
// F124: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:437:0 glam::f32::vec3::impl$17::sub
// F125: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs:188:0 glam::f32::vec3::Vec3::dot
// F126: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:167:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F127: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:169:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F128: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:170:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F129: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/convert/mod.rs:759:0 core::convert::impl$3::into
// F130: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:171:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F131: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:176:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F132: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:211:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F133: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:212:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F134: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:210:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F135: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:180:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F136: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:183:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F137: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1521:0 core::f32::impl$0::clamp
// F138: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:184:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F139: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1524:0 core::f32::impl$0::clamp
// F140: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:197:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F141: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$29::set_changed
// F142: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:625:0 bevy_ecs::event::EventWriter::send
// F143: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:196:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F144: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$30::deref_mut
// F145: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1970:0 alloc::vec::Vec::push
// F146: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::event::Events::send
// F147: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::event::Events::send
// F148: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1971:0 alloc::vec::Vec::push
// F149: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1974:0 alloc::vec::Vec::push
// F150: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F151: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F152: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 alloc::vec::Vec::as_mut_ptr
// F153: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F154: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1975:0 alloc::vec::Vec::push
// F155: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1976:0 alloc::vec::Vec::push
// F156: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::event::Events::send
// F157: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:219:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F158: crates/country-core/src/resources/roofs/roof_shape.rs:219:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F159: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:264:0 core::cmp::PartialEq::ne
// F160: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:220:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F161: crates/country-core/src/resources/roofs/roof_shape.rs:221:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F162: crates/country-core/src/resources/roofs/roof_shape.rs:226:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F163: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:23:0 glam::f32::vec2::impl$46::eq
// F164: crates/country-core/src/resources/roofs/roof_shape.rs:231:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F165: crates/country-core/src/resources/roofs/roof_shape.rs:233:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F166: crates/country-core/src/resources/roofs/roof_shape.rs:239:0 country_core::resources::roofs::roof_shape::impl$27::eq
// F167: crates/country-core/src/resources/roofs/roof_shape.rs:236:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F168: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:221:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F169: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:222:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F170: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:224:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F171: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:225:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F172: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:227:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F173: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:228:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F174: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:229:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F175: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:230:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F176: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1077:0 core::result::Result::unwrap
// F177: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:334:0 alloc::alloc::exchange_malloc
// F178: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:127:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F179: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:192:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h0ed253f9439ac0c4
// F180: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::unchecked_mul
// F181: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:290:0 alloc::raw_vec::RawVec::current_memory
// F182: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:142:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F183: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:143:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F184: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:221:0 alloc::raw_vec::RawVec::try_allocate_in
// F185: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:155:0 alloc::raw_vec::RawVec::with_capacity_in
// F186: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:688:0 alloc::vec::Vec::with_capacity_in
// F187: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:162:0 alloc::slice::hack::impl$1::to_vec
// F188: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:111:0 alloc::slice::hack::to_vec
// F189: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:441:0 alloc::slice::impl$0::to_vec_in
// F190: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:416:0 alloc::slice::impl$0::to_vec
// F191: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:823:0 alloc::slice::impl$9::to_owned
// F192: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/str.rs:211:0 alloc::str::impl$4::to_owned
// F193: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/string.rs:2706:0 alloc::string::impl$44::from
// F194: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:144:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F195: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:225:0 alloc::raw_vec::RawVec::try_allocate_in
// F196: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2987:0 core::intrinsics::copy_nonoverlapping
// F197: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:1336:0 core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
// F198: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:167:0 alloc::slice::hack::impl$1::to_vec
// F199: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:147:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F200: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:148:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F201: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:149:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F202: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:820:0 alloc::vec::Vec::from_raw_parts_in
// F203: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:104:0 alloc::slice::hack::into_vec
// F204: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:463:0 alloc::slice::impl$0::into_vec
// F205: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:152:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F206: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:567:0 alloc::raw_vec::handle_reserve
// F207: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/boxed.rs:1248:0 alloc::boxed::impl$8::drop
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 138aaa38c083b5cc2c428181de27b9e738708f36942bdb4608ba23c82e1c1bd5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_edit_roof_height_focused.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3200
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3200,"record_offset":1260,"rva":26791920,"va":5395501040}
// 编译实例 RVAs: 0x198cff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22752864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8a0d1a4fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395502555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22753019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e805d1a4fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395502726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21100368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8b008befe","size":5,"soft_fail":false,"target_va":5374401888,"va":5395502251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20930950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e87a9ec0fe","size":5,"soft_fail":false,"target_va":5374570832,"va":5395501777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20786310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e87ad3c2fe","size":5,"soft_fail":false,"target_va":5374716800,"va":5395503105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18563635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8cdbde4fe","size":5,"soft_fail":false,"target_va":5376939904,"va":5395503534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cee5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13233795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e87d1136ff","size":5,"soft_fail":false,"target_va":5382268400,"va":5395502190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141981e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e84746ffff","size":5,"soft_fail":false,"target_va":5395455632,"va":5395503172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7474095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8af0b7200","size":5,"soft_fail":false,"target_va":5402978144,"va":5395504044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17972093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e883c4edfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5395501976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24198003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e88dc48efe","size":5,"soft_fail":false,"target_va":5371305600,"va":5395503598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7476094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e87e137200","size":5,"soft_fail":false,"target_va":5402980128,"va":5395504029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20537602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8fe9ec6fe","size":5,"soft_fail":false,"target_va":5374966368,"va":5395503965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20912785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e86fe5c0fe","size":5,"soft_fail":false,"target_va":5374588624,"va":5395501404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20912622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e812e6c0fe","size":5,"soft_fail":false,"target_va":5374588816,"va":5395501433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20790242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e81ec4c2fe","size":5,"soft_fail":false,"target_va":5374712960,"va":5395503197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20065125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e89bd4cdfe","size":5,"soft_fail":false,"target_va":5375436032,"va":5395501152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19951927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8c98ecffe","size":5,"soft_fail":false,"target_va":5375550352,"va":5395502274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19951908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8dc8ecffe","size":5,"soft_fail":false,"target_va":5375550320,"va":5395502223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::boop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19730319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e871f0d2fe","size":5,"soft_fail":false,"target_va":5375771504,"va":5395501818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19730250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8b6f0d2fe","size":5,"soft_fail":false,"target_va":5375771552,"va":5395501797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19730258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8aef0d2fe","size":5,"soft_fail":false,"target_va":5375771600,"va":5395501853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19730897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e82feed2fe","size":5,"soft_fail":false,"target_va":5375771168,"va":5395502060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19730819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e87deed2fe","size":5,"soft_fail":false,"target_va":5375771344,"va":5395502158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19730858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e856eed2fe","size":5,"soft_fail":false,"target_va":5375771280,"va":5395502133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20146913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e81f95ccfe","size":5,"soft_fail":false,"target_va":5375355744,"va":5395502652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13229069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e8f32336ff","size":5,"soft_fail":false,"target_va":5382273632,"va":5395502696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21215683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e83d46bcfe","size":5,"soft_fail":false,"target_va":5374286080,"va":5395501758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20615040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e88070c5fe","size":5,"soft_fail":false,"target_va":5374887792,"va":5395502827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20615023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e89170c5fe","size":5,"soft_fail":false,"target_va":5374887840,"va":5395502858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20615061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e86b70c5fe","size":5,"soft_fail":false,"target_va":5374887920,"va":5395502976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_side
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12339947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e815b543ff","size":5,"soft_fail":false,"target_va":5383161616,"va":5395501558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18558702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395501040,"raw_hex":"e812d1e4fe","size":5,"soft_fail":false,"target_va":5376942944,"va":5395501641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3200,"record_offset":3112,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3200,"record_offset":3280,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3200,"record_offset":3052,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3200,"record_offset":4512,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3200,"record_offset":1500,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3200,"record_offset":4804,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3200,"record_offset":4056,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3200,"record_offset":4480,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3200,"record_offset":4776,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3200,"record_offset":3496,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3200,"record_offset":5208,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3200,"record_offset":2744,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3200,"record_offset":2716,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3200,"record_offset":3764,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3200,"record_offset":2452,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3200,"record_offset":2540,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs
//     evidence={"module":3200,"record_offset":3696,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":101556,"source_line":75,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":1260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":1260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":1260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":3336,"parent_record_offset":3308,"ancestor_record_offset":1260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":3368,"parent_record_offset":3336,"ancestor_record_offset":1260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":3624,"parent_record_offset":3568,"ancestor_record_offset":1260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3200,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":1260,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3200,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":1260,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3200,"record_offset":3596,"parent_record_offset":3568,"ancestor_record_offset":1260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3200,"record_offset":4704,"parent_record_offset":4672,"ancestor_record_offset":1260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3200,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":1260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3200,"record_offset":4672,"parent_record_offset":4632,"ancestor_record_offset":1260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3200,"record_offset":4632,"parent_record_offset":4596,"ancestor_record_offset":1260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3200,"record_offset":1716,"parent_record_offset":1572,"ancestor_record_offset":1260,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3200,"record_offset":3080,"parent_record_offset":3052,"ancestor_record_offset":1260,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3200,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":1260,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3200,"record_offset":1572,"parent_record_offset":1528,"ancestor_record_offset":1260,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3200,"record_offset":4596,"parent_record_offset":4512,"ancestor_record_offset":1260,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3200,"record_offset":2360,"parent_record_offset":1528,"ancestor_record_offset":1260,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3200,"record_offset":1992,"parent_record_offset":1528,"ancestor_record_offset":1260,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3200,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":1260,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3200,"record_offset":1528,"parent_record_offset":1500,"ancestor_record_offset":1260,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3200,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":1260,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3200,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1260,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3200,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":1260,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3200,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":1260,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3200,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":1260,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3200,"record_offset":2296,"parent_record_offset":2016,"ancestor_record_offset":1260,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3200,"record_offset":2044,"parent_record_offset":2016,"ancestor_record_offset":1260,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3200,"record_offset":1964,"parent_record_offset":1528,"ancestor_record_offset":1260,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3200,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":1260,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3200,"record_offset":1780,"parent_record_offset":1748,"ancestor_record_offset":1260,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3200,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":1260,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3200,"record_offset":5760,"parent_record_offset":5728,"ancestor_record_offset":1260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3200,"record_offset":1660,"parent_record_offset":1572,"ancestor_record_offset":1260,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3200,"record_offset":2900,"parent_record_offset":2768,"ancestor_record_offset":1260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3200,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":1260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3200,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":1260,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3200,"record_offset":3828,"parent_record_offset":3788,"ancestor_record_offset":1260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3200,"record_offset":3732,"parent_record_offset":3696,"ancestor_record_offset":1260,"inlinee":5483,"source_line":378,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3200,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":1260,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3200,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3200,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":1260,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3200,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":1260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3200,"record_offset":1888,"parent_record_offset":1856,"ancestor_record_offset":1260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3200,"record_offset":3444,"parent_record_offset":3416,"ancestor_record_offset":1260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3200,"record_offset":4736,"parent_record_offset":4632,"ancestor_record_offset":1260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3200,"record_offset":1936,"parent_record_offset":1528,"ancestor_record_offset":1260,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3200,"record_offset":1604,"parent_record_offset":1572,"ancestor_record_offset":1260,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3200,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1260,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3200,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":1260,"inlinee":77167,"source_line":216,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$27::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3200,"record_offset":4896,"parent_record_offset":4828,"ancestor_record_offset":1260,"inlinee":77169,"source_line":239,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3200,"record_offset":1748,"parent_record_offset":1528,"ancestor_record_offset":1260,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3200,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1260,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3200,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":1260,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3200,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":1260,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3200,"record_offset":2832,"parent_record_offset":2800,"ancestor_record_offset":1260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3200,"record_offset":2800,"parent_record_offset":2768,"ancestor_record_offset":1260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3200,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":1260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3200,"record_offset":4868,"parent_record_offset":4828,"ancestor_record_offset":1260,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3200,"record_offset":3924,"parent_record_offset":3788,"ancestor_record_offset":1260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3200,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":1260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3200,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":1260,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3200,"record_offset":2864,"parent_record_offset":2832,"ancestor_record_offset":1260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18CB4
// 首条位置: {"module":3200,"inlinee":101556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3200,"record_offset":3732,"parent_record_offset":3696,"ancestor_record_offset":3696,"inlinee":5483,"source_line":378,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3200,"record_offset":5896,"rva":26795440,"va":5395504560}
// 编译实例 RVAs: 0x198ddb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22754930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e88ec9a4fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395504621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22755309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e813c8a4fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395505016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7473192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e828087200","size":5,"soft_fail":false,"target_va":5402978144,"va":5395504947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20148053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e8ab90ccfe","size":5,"soft_fail":false,"target_va":5375356704,"va":5395504752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20147991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e8e990ccfe","size":5,"soft_fail":false,"target_va":5375356896,"va":5395504882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20148757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e8eb8dccfe","size":5,"soft_fail":false,"target_va":5375355984,"va":5395504736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20148296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e8b88fccfe","size":5,"soft_fail":false,"target_va":5375356560,"va":5395504851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20148180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e82c90ccfe","size":5,"soft_fail":false,"target_va":5375356608,"va":5395504783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20148159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395504560,"raw_hex":"e84190ccfe","size":5,"soft_fail":false,"target_va":5375356656,"va":5395504810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":6096,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3200,"record_offset":6800,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3200,"record_offset":6244,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3200,"record_offset":6884,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":5896,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":5896,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":5896,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":6128,"parent_record_offset":6096,"ancestor_record_offset":5896,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3200,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":5896,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3200,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":5896,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3200,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":5896,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3200,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":5896,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3200,"record_offset":6676,"parent_record_offset":6504,"ancestor_record_offset":5896,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3200,"record_offset":6440,"parent_record_offset":6412,"ancestor_record_offset":5896,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3200,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":5896,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3200,"record_offset":6412,"parent_record_offset":6384,"ancestor_record_offset":5896,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3200,"record_offset":6356,"parent_record_offset":6328,"ancestor_record_offset":5896,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3200,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":5896,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3200,"record_offset":6328,"parent_record_offset":6300,"ancestor_record_offset":5896,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3200,"record_offset":6300,"parent_record_offset":6272,"ancestor_record_offset":5896,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3200,"record_offset":6272,"parent_record_offset":6244,"ancestor_record_offset":5896,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3200,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":5896,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3200,"record_offset":6476,"parent_record_offset":6440,"ancestor_record_offset":5896,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3200,"record_offset":6736,"parent_record_offset":6712,"ancestor_record_offset":5896,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3200,"record_offset":6712,"parent_record_offset":6440,"ancestor_record_offset":5896,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3200,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":5896,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03200.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_edit_roof_height_focused.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::SystemConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > >", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>", "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_edit_roof_height_focused.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_edit_roof_height_focused.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// 信息 ID: 6812d55a4f2e63af42bcaa56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 47c6405e8e70abf74f4bb199: void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> > @ 0x12d6ea2 -> 0x198cff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)> @ 0x12d9a92 -> 0x198cff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// 信息 ID: b46b40af1578b71b05fef039
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused; module=3200, inline_record=3696; 不是独立调用边。
// [PDB 内联宿主] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused; module=3200, inline_record=4200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// 信息 ID: 3bd056497c771482b163a8df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused", "captures": "unknown"}
// [闭包环境类型引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1 <- core::ops::function::FnOnce::call_once<system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1 <- core::ptr::drop_in_place<system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x198cf12 -> 0x198ddb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
