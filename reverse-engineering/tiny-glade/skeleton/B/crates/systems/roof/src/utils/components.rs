// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: components.rs.assembly.json.gz

// FUNCTION system_roof::utils::components::impl$0::register_required_components
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 3; complete records in enriched/assembly sidecars.
// Compiled procedure 4336:2188 -> rva_258d1f0_len_6f0
//
// FUNCTION system_roof::utils::components::impl$1::register_required_components
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 4336:6724 -> rva_258d8e0_len_6f0
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_258d1f0_len_6f0: RVA 0x258d1f0, 1776 PDB bytes; 420 instructions
// Evidence: _assembly/roof-v1/B/pack-003/rva_258d1f0_len_6f0.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 4336:2188: system_roof::utils::components::impl$0::register_required_components
// Frames: F1
// 0x14258d1f0  55                             pushq	%rbp
// 0x14258d1f1  4157                           pushq	%r15
// 0x14258d1f3  4156                           pushq	%r14
// 0x14258d1f5  4155                           pushq	%r13
// 0x14258d1f7  4154                           pushq	%r12
// 0x14258d1f9  56                             pushq	%rsi
// 0x14258d1fa  57                             pushq	%rdi
// 0x14258d1fb  53                             pushq	%rbx
// 0x14258d1fc  4881ece8020000                 subq	$0x2e8, %rsp            # imm = 0x2E8
// 0x14258d203  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14258d20b  48c78560020000feffffff         movq	$-0x2, 0x260(%rbp)
// 0x14258d216  4989d6                         movq	%rdx, %r14
// 0x14258d219  488bb5d0020000                 movq	0x2d0(%rbp), %rsi
// 0x14258d220  4c8b3a                         movq	(%rdx), %r15
// 0x14258d223  488b5e08                       movq	0x8(%rsi), %rbx
// 0x14258d227  488b7e10                       movq	0x10(%rsi), %rdi
// 0x14258d22b  4c89f9                         movq	%r15, %rcx
// 0x14258d22e  4889da                         movq	%rbx, %rdx
// 0x14258d231  4989f8                         movq	%rdi, %r8
// 0x14258d234  e8373b7bfe                     callq	0x140d40d70 <.text+0xd3fd70>
// Frames: F2 -> F3 -> F1
// 0x14258d239  48c7853002000000000000         movq	$0x0, 0x230(%rbp)
// 0x14258d244  48c7853802000008000000         movq	$0x8, 0x238(%rbp)
// 0x14258d24f  48c7854002000000000000         movq	$0x0, 0x240(%rbp)
// Frames: F4 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d25a  49837f3000                     cmpq	$0x0, 0x30(%r15)
// 0x14258d25f  48b806275cd45221f511           movabsq	$0x11f52152d45c2706, %rax # imm = 0x11F52152D45C2706
// 0x14258d269  48b975df370294852a99           movabsq	$-0x66d57a6bfdc8208b, %rcx # imm = 0x992A85940237DF75
// 0x14258d273  0f849a000000                   je	0x14258d313 <.text+0x258c313>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d279  498b5718                       movq	0x18(%r15), %rdx
// 0x14258d27d  4d8b4720                       movq	0x20(%r15), %r8
// 0x14258d281  4d89c1                         movq	%r8, %r9
// 0x14258d284  4921c1                         andq	%rax, %r9
// 0x14258d287  4531d2                         xorl	%r10d, %r10d
// 0x14258d28a  660f6f050e333c00               movdqa	0x3c330e(%rip), %xmm0   # 0x1429505a0
// 0x14258d292  660f76c9                       pcmpeqd	%xmm1, %xmm1
// Frames: F13 -> F14 -> F15 -> F16 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d296  f3420f6f140a                   movdqu	(%rdx,%r9), %xmm2
// Frames: F17 -> F18 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d29c  660f6fda                       movdqa	%xmm2, %xmm3
// 0x14258d2a0  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x14258d2a4  66440fd7db                     pmovmskb	%xmm3, %r11d
// Frames: F20 -> F21 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2a9  4585db                         testl	%r11d, %r11d
// 0x14258d2ac  752f                           jne	0x14258d2dd <.text+0x258c2dd>
// Frames: F17 -> F18 -> F22 -> F23 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2ae  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x14258d2b2  66440fd7da                     pmovmskb	%xmm2, %r11d
// Frames: F23 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2b7  4585db                         testl	%r11d, %r11d
// 0x14258d2ba  7557                           jne	0x14258d313 <.text+0x258c313>
// Frames: F24 -> F25 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2bc  4d01d1                         addq	%r10, %r9
// 0x14258d2bf  4983c110                       addq	$0x10, %r9
// Frames: F26 -> F25 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2c3  4983c210                       addq	$0x10, %r10
// Frames: F27 -> F25 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2c7  4d21c1                         andq	%r8, %r9
// Frames: F28 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2ca  ebca                           jmp	0x14258d296 <.text+0x258c296>
// 0x14258d2cc  0f1f4000                       nopl	(%rax)
// Frames: F29 -> F30 -> F31 -> F32 -> F21 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2d0  458d63ff                       leal	-0x1(%r11), %r12d
// Frames: F20 -> F21 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2d4  664521dc                       andw	%r11w, %r12w
// 0x14258d2d8  4589e3                         movl	%r12d, %r11d
// 0x14258d2db  74d1                           je	0x14258d2ae <.text+0x258c2ae>
// Frames: F30 -> F31 -> F32 -> F21 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2dd  f3450fbce3                     tzcntl	%r11d, %r12d
// Frames: F33 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2e2  4d01cc                         addq	%r9, %r12
// 0x14258d2e5  4d21c4                         andq	%r8, %r12
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2e8  49f7dc                         negq	%r12
// 0x14258d2eb  4f8d2464                       leaq	(%r12,%r12,2), %r12
// Frames: F39 -> F40 -> F41 -> F42 -> F43 -> F37 -> F38 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2ef  4a394ce2e8                     cmpq	%rcx, -0x18(%rdx,%r12,8)
// Frames: F38 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F1
// 0x14258d2f4  75da                           jne	0x14258d2d0 <.text+0x258c2d0>
// 0x14258d2f6  4a3944e2f0                     cmpq	%rax, -0x10(%rdx,%r12,8)
// 0x14258d2fb  75d3                           jne	0x14258d2d0 <.text+0x258c2d0>
// Frames: F44 -> F3 -> F1
// 0x14258d2fd  4a8d04e2                       leaq	(%rdx,%r12,8), %rax
// 0x14258d301  4c8b60f8                       movq	-0x8(%rax), %r12
// Frames: F45 -> F1
// 0x14258d305  483b3e                         cmpq	(%rsi), %rdi
// 0x14258d308  0f8452030000                   je	0x14258d660 <.text+0x258c660>
// 0x14258d30e  e9f6030000                     jmp	0x14258d709 <.text+0x258c709>
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F3 -> F1
// 0x14258d313  48899d48020000                 movq	%rbx, 0x248(%rbp)
// 0x14258d31a  410fb65760                     movzbl	0x60(%r15), %edx
// 0x14258d31f  498b9780000000                 movq	0x80(%r15), %rdx
// Frames: F8 -> F9 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d326  4d8b8f88000000                 movq	0x88(%r15), %r9
// 0x14258d32d  4d89ca                         movq	%r9, %r10
// 0x14258d330  4921c2                         andq	%rax, %r10
// 0x14258d333  4531db                         xorl	%r11d, %r11d
// 0x14258d336  660f6f0562323c00               movdqa	0x3c3262(%rip), %xmm0   # 0x1429505a0
// 0x14258d33e  660f76c9                       pcmpeqd	%xmm1, %xmm1
// Frames: F13 -> F14 -> F15 -> F16 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d342  f3420f6f1412                   movdqu	(%rdx,%r10), %xmm2
// Frames: F17 -> F18 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d348  660f6fda                       movdqa	%xmm2, %xmm3
// 0x14258d34c  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x14258d350  66440fd7e3                     pmovmskb	%xmm3, %r12d
// Frames: F20 -> F21 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d355  4585e4                         testl	%r12d, %r12d
// 0x14258d358  7534                           jne	0x14258d38e <.text+0x258c38e>
// Frames: F17 -> F18 -> F22 -> F23 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d35a  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x14258d35e  66440fd7c2                     pmovmskb	%xmm2, %r8d
// Frames: F23 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d363  4585c0                         testl	%r8d, %r8d
// 0x14258d366  0f85f2000000                   jne	0x14258d45e <.text+0x258c45e>
// Frames: F24 -> F25 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d36c  4d01da                         addq	%r11, %r10
// 0x14258d36f  4983c210                       addq	$0x10, %r10
// Frames: F26 -> F25 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d373  4983c310                       addq	$0x10, %r11
// Frames: F27 -> F25 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d377  4d21ca                         andq	%r9, %r10
// Frames: F28 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d37a  ebc6                           jmp	0x14258d342 <.text+0x258c342>
// 0x14258d37c  0f1f4000                       nopl	(%rax)
// Frames: F29 -> F30 -> F31 -> F32 -> F21 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d380  458d4424ff                     leal	-0x1(%r12), %r8d
// Frames: F20 -> F21 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d385  664521e0                       andw	%r12w, %r8w
// 0x14258d389  4589c4                         movl	%r8d, %r12d
// 0x14258d38c  74cc                           je	0x14258d35a <.text+0x258c35a>
// Frames: F30 -> F31 -> F32 -> F21 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d38e  f3450fbcec                     tzcntl	%r12d, %r13d
// Frames: F33 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d393  4d01d5                         addq	%r10, %r13
// 0x14258d396  4d21cd                         andq	%r9, %r13
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d399  4d89e8                         movq	%r13, %r8
// 0x14258d39c  49f7d8                         negq	%r8
// 0x14258d39f  4a8d1ced00000000               leaq	(,%r13,8), %rbx
// 0x14258d3a7  49c1e007                       shlq	$0x7, %r8
// 0x14258d3ab  4929d8                         subq	%rbx, %r8
// Frames: F39 -> F40 -> F41 -> F42 -> F43 -> F37 -> F38 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3ae  4a398c0278ffffff               cmpq	%rcx, -0x88(%rdx,%r8)
// Frames: F38 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3b6  75c8                           jne	0x14258d380 <.text+0x258c380>
// 0x14258d3b8  4a39440280                     cmpq	%rax, -0x80(%rdx,%r8)
// 0x14258d3bd  75c1                           jne	0x14258d380 <.text+0x258c380>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3bf  4d89ea                         movq	%r13, %r10
// 0x14258d3c2  49c1e207                       shlq	$0x7, %r10
// 0x14258d3c6  4f8d1cea                       leaq	(%r10,%r13,8), %r11
// 0x14258d3ca  49c1fb03                       sarq	$0x3, %r11
// 0x14258d3ce  49baf1f0f0f0f0f0f0f0           movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
// 0x14258d3d8  4d0fafd3                       imulq	%r11, %r10
// Frames: F64 -> F65 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3dc  4d8d5af0                       leaq	-0x10(%r10), %r11
// Frames: F65 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3e0  4d21cb                         andq	%r9, %r11
// Frames: F13 -> F14 -> F15 -> F67 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3e3  f3420f6f041a                   movdqu	(%rdx,%r11), %xmm0
// Frames: F68 -> F69 -> F70 -> F67 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3e9  660f76c9                       pcmpeqd	%xmm1, %xmm1
// Frames: F17 -> F18 -> F70 -> F67 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3ed  660f74c1                       pcmpeqb	%xmm1, %xmm0
// 0x14258d3f1  66440fd7e0                     pmovmskb	%xmm0, %r12d
// Frames: F13 -> F14 -> F15 -> F71 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3f6  f3420f6f0412                   movdqu	(%rdx,%r10), %xmm0
// Frames: F17 -> F18 -> F70 -> F71 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d3fc  660f74c1                       pcmpeqb	%xmm1, %xmm0
// 0x14258d400  66440fd7c8                     pmovmskb	%xmm0, %r9d
// Frames: F72 -> F73 -> F74 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d405  4585e4                         testl	%r12d, %r12d
// 0x14258d408  740b                           je	0x14258d415 <.text+0x258c415>
// 0x14258d40a  66450fbde4                     bsrw	%r12w, %r12w
// 0x14258d40f  4183f40f                       xorl	$0xf, %r12d
// 0x14258d413  eb05                           jmp	0x14258d41a <.text+0x258c41a>
// 0x14258d415  6641bc1000                     movw	$0x10, %r12w
// Frames: F75 -> F76 -> F74 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d41a  4901d0                         addq	%rdx, %r8
// 0x14258d41d  4181c900000100                 orl	$0x10000, %r9d          # imm = 0x10000
// 0x14258d424  f3450fbcc9                     tzcntl	%r9d, %r9d
// Frames: F74 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d429  4501cc                         addl	%r9d, %r12d
// 0x14258d42c  41b180                         movb	$-0x80, %r9b
// 0x14258d42f  664183fc0f                     cmpw	$0xf, %r12w
// 0x14258d434  770a                           ja	0x14258d440 <.text+0x258c440>
// Frames: F77 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d436  49ff8790000000                 incq	0x90(%r15)
// 0x14258d43d  41b1ff                         movb	$-0x1, %r9b
// Frames: F78 -> F79 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d440  46880c12                       movb	%r9b, (%rdx,%r10)
// Frames: F80 -> F79 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d444  46884c1a10                     movb	%r9b, 0x10(%rdx,%r11)
// Frames: F81 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d449  49ff8f98000000                 decq	0x98(%r15)
// Frames: F82 -> F83 -> F84 -> F85 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d450  498b5088                       movq	-0x78(%r8), %rdx
// Frames: F54 -> F55 -> F56 -> F3 -> F1
// 0x14258d454  4883fa02                       cmpq	$0x2, %rdx
// 0x14258d458  0f8526020000                   jne	0x14258d684 <.text+0x258c684>
// Frames: F86 -> F3 -> F1
// 0x14258d45e  498b5608                       movq	0x8(%r14), %rdx
// Frames: F87 -> F86 -> F3 -> F1
// 0x14258d462  4c8b22                         movq	(%rdx), %r12
// Frames: F88 -> F86 -> F3 -> F1
// 0x14258d465  4d8d6c2401                     leaq	0x1(%r12), %r13
// 0x14258d46a  4c892a                         movq	%r13, (%rdx)
// 0x14258d46d  48ba0000000000000080           movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
// Frames: F89 -> F90 -> F91 -> F3 -> F1
// 0x14258d477  48899508020000                 movq	%rdx, 0x208(%rbp)
// 0x14258d47e  488d15b9f79100                 leaq	0x91f7b9(%rip), %rdx    # 0x142eacc3e
// 0x14258d485  48899510020000                 movq	%rdx, 0x210(%rbp)
// 0x14258d48c  48c7851802000029000000         movq	$0x29, 0x218(%rbp)
// 0x14258d497  66c785280200000001             movw	$0x100, 0x228(%rbp)     # imm = 0x100
// 0x14258d4a0  48c785d001000001000000         movq	$0x1, 0x1d0(%rbp)
// 0x14258d4ab  48898dd8010000                 movq	%rcx, 0x1d8(%rbp)
// 0x14258d4b2  488985e0010000                 movq	%rax, 0x1e0(%rbp)
// 0x14258d4b9  48c785f801000001000000         movq	$0x1, 0x1f8(%rbp)
// 0x14258d4c4  48c7850002000000000000         movq	$0x0, 0x200(%rbp)
// 0x14258d4cf  48c7852002000000000000         movq	$0x0, 0x220(%rbp)
// 0x14258d4da  c6852a02000001                 movb	$0x1, 0x22a(%rbp)
// 0x14258d4e1  48c785e801000000000000         movq	$0x0, 0x1e8(%rbp)
// Frames: F92 -> F90 -> F91 -> F3 -> F1
// 0x14258d4ec  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x14258d4f3  4c8d85d0010000                 leaq	0x1d0(%rbp), %r8
// 0x14258d4fa  4c89e2                         movq	%r12, %rdx
// 0x14258d4fd  e86e237bfe                     callq	0x140d3f870 <.text+0xd3e870>
// Frames: F93 -> F90 -> F91 -> F3 -> F1
// 0x14258d502  4d396f10                       cmpq	%r13, 0x10(%r15)
// 0x14258d506  7319                           jae	0x14258d521 <.text+0x258c521>
// Frames: F94 -> F90 -> F91 -> F3 -> F1
// 0x14258d508  c6855f02000001                 movb	$0x1, 0x25f(%rbp)
// 0x14258d50f  4c8d053a4cb700                 leaq	0xb74c3a(%rip), %r8     # 0x143102150
// 0x14258d516  4c89f9                         movq	%r15, %rcx
// 0x14258d519  4c89ea                         movq	%r13, %rdx
// 0x14258d51c  e8af0884fe                     callq	0x140dcddd0 <.text+0xdccdd0>
// Frames: F95 -> F96 -> F97 -> F90 -> F91 -> F3 -> F1
// 0x14258d521  498b5f08                       movq	0x8(%r15), %rbx
// 0x14258d525  4d69fcd0000000                 imulq	$0xd0, %r12, %r15
// 0x14258d52c  4a8d043b                       leaq	(%rbx,%r15), %rax
// 0x14258d530  48898550020000                 movq	%rax, 0x250(%rbp)
// 0x14258d537  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14258d53b  488d9580000000                 leaq	0x80(%rbp), %rdx
// Frames: F98 -> F90 -> F91 -> F3 -> F1
// 0x14258d542  41b8d0000000                   movl	$0xd0, %r8d
// 0x14258d548  e8b3663900                     callq	0x142923c00 <.text+0x2922c00>
// Frames: F99 -> F98 -> F90 -> F91 -> F3 -> F1
// 0x14258d54d  42833c3b02                     cmpl	$0x2, (%rbx,%r15)
// 0x14258d552  740c                           je	0x14258d560 <.text+0x258c560>
// 0x14258d554  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14258d55b  e8f03506fe                     callq	0x1405f0b50 <.text+0x5efb50>
// Frames: F98 -> F90 -> F91 -> F3 -> F1
// 0x14258d560  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14258d564  41b8d0000000                   movl	$0xd0, %r8d
// 0x14258d56a  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14258d571  e88a663900                     callq	0x142923c00 <.text+0x2922c00>
// Frames: F100 -> F101 -> F91 -> F3 -> F1
// 0x14258d576  498b0e                         movq	(%r14), %rcx
// Frames: F102 -> F101 -> F91 -> F3 -> F1
// 0x14258d579  4883c118                       addq	$0x18, %rcx
// 0x14258d57d  48ba75df370294852a99           movabsq	$-0x66d57a6bfdc8208b, %rdx # imm = 0x992A85940237DF75
// 0x14258d587  49b806275cd45221f511           movabsq	$0x11f52152d45c2706, %r8 # imm = 0x11F52152D45C2706
// 0x14258d591  4d89e1                         movq	%r12, %r9
// 0x14258d594  e88756affd                     callq	0x140082c20 <.text+0x81c20>
// Frames: F103 -> F104 -> F91 -> F3 -> F1
// 0x14258d599  0f1005404bb700                 movups	0xb74b40(%rip), %xmm0   # 0x1431020e0
// 0x14258d5a0  0f298590000000                 movaps	%xmm0, 0x90(%rbp)
// 0x14258d5a7  f30f6f05214bb700               movdqu	0xb74b21(%rip), %xmm0   # 0x1431020d0
// 0x14258d5af  660f7f8580000000               movdqa	%xmm0, 0x80(%rbp)
// 0x14258d5b7  c6855e02000001                 movb	$0x1, 0x25e(%rbp)
// Frames: F105 -> F91 -> F3 -> F1
// 0x14258d5be  488d8530020000                 leaq	0x230(%rbp), %rax
// 0x14258d5c5  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14258d5ca  4c89f2                         movq	%r14, %rdx
// 0x14258d5cd  e81efcffff                     callq	0x14258d1f0 <.text+0x258c1f0>
// Frames: F106 -> F91 -> F3 -> F1
// 0x14258d5d2  498b06                         movq	(%r14), %rax
// Frames: F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F106 -> F91 -> F3 -> F1
// 0x14258d5d5  488b4008                       movq	0x8(%rax), %rax
// Frames: F113 -> F91 -> F3 -> F1
// 0x14258d5d9  0f288580000000                 movaps	0x80(%rbp), %xmm0
// 0x14258d5e0  0f288d90000000                 movaps	0x90(%rbp), %xmm1
// 0x14258d5e7  0f294dc0                       movaps	%xmm1, -0x40(%rbp)
// 0x14258d5eb  0f2945b0                       movaps	%xmm0, -0x50(%rbp)
// 0x14258d5ef  4a8d4c3860                     leaq	0x60(%rax,%r15), %rcx
// Frames: F114 -> F99 -> F99 -> F99 -> F99 -> F113 -> F91 -> F3 -> F1
// 0x14258d5f4  4a8d943880000000               leaq	0x80(%rax,%r15), %rdx
// Frames: F115 -> F99 -> F99 -> F99 -> F99 -> F113 -> F91 -> F3 -> F1
// 0x14258d5fc  41b820000000                   movl	$0x20, %r8d
// 0x14258d602  41b910000000                   movl	$0x10, %r9d
// 0x14258d608  48898d50020000                 movq	%rcx, 0x250(%rbp)
// 0x14258d60f  e82c3dacfd                     callq	0x140051340 <.text+0x50340>
// 0x14258d614  488b9d48020000                 movq	0x248(%rbp), %rbx
// Frames: F113 -> F91 -> F3 -> F1
// 0x14258d61b  660f6f45b0                     movdqa	-0x50(%rbp), %xmm0
// 0x14258d620  660f6f4dc0                     movdqa	-0x40(%rbp), %xmm1
// 0x14258d625  488b8550020000                 movq	0x250(%rbp), %rax
// 0x14258d62c  f30f7f4810                     movdqu	%xmm1, 0x10(%rax)
// 0x14258d631  f30f7f00                       movdqu	%xmm0, (%rax)
// Frames: F116 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d635  488b9530020000                 movq	0x230(%rbp), %rdx
// Frames: F118 -> F116 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d63c  4885d2                         testq	%rdx, %rdx
// 0x14258d63f  7416                           je	0x14258d657 <.text+0x258c657>
// Frames: F116 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d641  488b8d38020000                 movq	0x238(%rbp), %rcx
// Frames: F119 -> F120 -> F116 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d648  48c1e203                       shlq	$0x3, %rdx
// Frames: F121 -> F122 -> F123 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d64c  41b808000000                   movl	$0x8, %r8d
// 0x14258d652  e8c96508fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F45 -> F1
// 0x14258d657  483b3e                         cmpq	(%rsi), %rdi
// 0x14258d65a  0f85a9000000                   jne	0x14258d709 <.text+0x258c709>
// Frames: F124 -> F1
// 0x14258d660  488d15314bb700                 leaq	0xb74b31(%rip), %rdx    # 0x143102198
// 0x14258d667  4889f1                         movq	%rsi, %rcx
// 0x14258d66a  e881475ffe                     callq	0x140b81df0 <.text+0xb80df0>
// Frames: F107 -> F108 -> F109 -> F110 -> F125 -> F1
// 0x14258d66f  488b4608                       movq	0x8(%rsi), %rax
// Frames: F126 -> F127 -> F1
// 0x14258d673  4c8924f8                       movq	%r12, (%rax,%rdi,8)
// Frames: F128 -> F1
// 0x14258d677  488d4701                       leaq	0x1(%rdi), %rax
// 0x14258d67b  48894610                       movq	%rax, 0x10(%rsi)
// Frames: F129 -> F1
// 0x14258d67f  e995000000                     jmp	0x14258d719 <.text+0x258c719>
// Frames: F130 -> F55 -> F56 -> F3 -> F1
// 0x14258d684  410f1040f0                     movups	-0x10(%r8), %xmm0
// 0x14258d689  0f1185c0010000                 movups	%xmm0, 0x1c0(%rbp)
// 0x14258d690  410f1040e0                     movups	-0x20(%r8), %xmm0
// 0x14258d695  0f1185b0010000                 movups	%xmm0, 0x1b0(%rbp)
// 0x14258d69c  410f1040d0                     movups	-0x30(%r8), %xmm0
// 0x14258d6a1  0f1185a0010000                 movups	%xmm0, 0x1a0(%rbp)
// 0x14258d6a8  f3410f6f4090                   movdqu	-0x70(%r8), %xmm0
// 0x14258d6ae  f3410f6f48a0                   movdqu	-0x60(%r8), %xmm1
// 0x14258d6b4  f3410f6f50b0                   movdqu	-0x50(%r8), %xmm2
// 0x14258d6ba  f3410f6f58c0                   movdqu	-0x40(%r8), %xmm3
// 0x14258d6c0  f30f7f9d90010000               movdqu	%xmm3, 0x190(%rbp)
// 0x14258d6c8  f30f7f9580010000               movdqu	%xmm2, 0x180(%rbp)
// 0x14258d6d0  f30f7f8d70010000               movdqu	%xmm1, 0x170(%rbp)
// 0x14258d6d8  f30f7f8560010000               movdqu	%xmm0, 0x160(%rbp)
// Frames: F51 -> F3 -> F1
// 0x14258d6e0  48899558010000                 movq	%rdx, 0x158(%rbp)
// Frames: F131 -> F3 -> F1
// 0x14258d6e7  488d8d58010000                 leaq	0x158(%rbp), %rcx
// 0x14258d6ee  4c89f2                         movq	%r14, %rdx
// 0x14258d6f1  e84a237bfe                     callq	0x140d3fa40 <.text+0xd3ea40>
// Frames: F45 -> F1
// 0x14258d6f6  4989c4                         movq	%rax, %r12
// 0x14258d6f9  488b9d48020000                 movq	0x248(%rbp), %rbx
// 0x14258d700  483b3e                         cmpq	(%rsi), %rdi
// 0x14258d703  0f8457ffffff                   je	0x14258d660 <.text+0x258c660>
// Frames: F126 -> F127 -> F1
// 0x14258d709  4c8924fb                       movq	%r12, (%rbx,%rdi,8)
// Frames: F128 -> F1
// 0x14258d70d  4889f8                         movq	%rdi, %rax
// 0x14258d710  48ffc0                         incq	%rax
// 0x14258d713  48894610                       movq	%rax, 0x10(%rsi)
// Frames: F129 -> F1
// 0x14258d717  7404                           je	0x14258d71d <.text+0x258c71d>
// Frames: F132 -> F1
// 0x14258d719  48897e10                       movq	%rdi, 0x10(%rsi)
// Frames: F1
// 0x14258d71d  4881c4e8020000                 addq	$0x2e8, %rsp            # imm = 0x2E8
// 0x14258d724  5b                             popq	%rbx
// 0x14258d725  5f                             popq	%rdi
// 0x14258d726  5e                             popq	%rsi
// 0x14258d727  415c                           popq	%r12
// 0x14258d729  415d                           popq	%r13
// 0x14258d72b  415e                           popq	%r14
// 0x14258d72d  415f                           popq	%r15
// 0x14258d72f  5d                             popq	%rbp
// 0x14258d730  c3                             retq
// 0x14258d731  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// 0x14258d740  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258d745  55                             pushq	%rbp
// 0x14258d746  4157                           pushq	%r15
// 0x14258d748  4156                           pushq	%r14
// 0x14258d74a  4155                           pushq	%r13
// 0x14258d74c  4154                           pushq	%r12
// 0x14258d74e  56                             pushq	%rsi
// 0x14258d74f  57                             pushq	%rdi
// 0x14258d750  53                             pushq	%rbx
// 0x14258d751  4883ec28                       subq	$0x28, %rsp
// 0x14258d755  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14258d75c  488d55b0                       leaq	-0x50(%rbp), %rdx
// Frames: F98 -> F90 -> F91 -> F3 -> F1
// 0x14258d760  41b8d0000000                   movl	$0xd0, %r8d
// 0x14258d766  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14258d76d  e88e643900                     callq	0x142923c00 <.text+0x2922c00>
// 0x14258d772  c6855f02000000                 movb	$0x0, 0x25f(%rbp)
// 0x14258d779  4883c428                       addq	$0x28, %rsp
// 0x14258d77d  5b                             popq	%rbx
// 0x14258d77e  5f                             popq	%rdi
// 0x14258d77f  5e                             popq	%rsi
// 0x14258d780  415c                           popq	%r12
// 0x14258d782  415d                           popq	%r13
// 0x14258d784  415e                           popq	%r14
// 0x14258d786  415f                           popq	%r15
// 0x14258d788  5d                             popq	%rbp
// 0x14258d789  c3                             retq
// 0x14258d78a  660f1f440000                   nopw	(%rax,%rax)
// 0x14258d790  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258d795  55                             pushq	%rbp
// 0x14258d796  4157                           pushq	%r15
// 0x14258d798  4156                           pushq	%r14
// 0x14258d79a  4155                           pushq	%r13
// 0x14258d79c  4154                           pushq	%r12
// 0x14258d79e  56                             pushq	%rsi
// 0x14258d79f  57                             pushq	%rdi
// 0x14258d7a0  53                             pushq	%rbx
// 0x14258d7a1  4883ec28                       subq	$0x28, %rsp
// 0x14258d7a5  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F133 -> F90 -> F91 -> F3 -> F1
// 0x14258d7ac  80bd5f02000000                 cmpb	$0x0, 0x25f(%rbp)
// 0x14258d7b3  740c                           je	0x14258d7c1 <.text+0x258c7c1>
// 0x14258d7b5  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x14258d7bc  e88f3306fe                     callq	0x1405f0b50 <.text+0x5efb50>
// 0x14258d7c1  90                             nop
// 0x14258d7c2  4883c428                       addq	$0x28, %rsp
// 0x14258d7c6  5b                             popq	%rbx
// 0x14258d7c7  5f                             popq	%rdi
// 0x14258d7c8  5e                             popq	%rsi
// 0x14258d7c9  415c                           popq	%r12
// 0x14258d7cb  415d                           popq	%r13
// 0x14258d7cd  415e                           popq	%r14
// 0x14258d7cf  415f                           popq	%r15
// 0x14258d7d1  5d                             popq	%rbp
// 0x14258d7d2  c3                             retq
// 0x14258d7d3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x14258d7e0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258d7e5  55                             pushq	%rbp
// 0x14258d7e6  4157                           pushq	%r15
// 0x14258d7e8  4156                           pushq	%r14
// 0x14258d7ea  4155                           pushq	%r13
// 0x14258d7ec  4154                           pushq	%r12
// 0x14258d7ee  56                             pushq	%rsi
// 0x14258d7ef  57                             pushq	%rdi
// 0x14258d7f0  53                             pushq	%rbx
// 0x14258d7f1  4883ec28                       subq	$0x28, %rsp
// 0x14258d7f5  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F113 -> F91 -> F3 -> F1
// 0x14258d7fc  660f6f45b0                     movdqa	-0x50(%rbp), %xmm0
// 0x14258d801  660f6f4dc0                     movdqa	-0x40(%rbp), %xmm1
// 0x14258d806  488b8550020000                 movq	0x250(%rbp), %rax
// 0x14258d80d  f30f7f4810                     movdqu	%xmm1, 0x10(%rax)
// 0x14258d812  f30f7f00                       movdqu	%xmm0, (%rax)
// 0x14258d816  c6855e02000000                 movb	$0x0, 0x25e(%rbp)
// 0x14258d81d  4883c428                       addq	$0x28, %rsp
// 0x14258d821  5b                             popq	%rbx
// 0x14258d822  5f                             popq	%rdi
// 0x14258d823  5e                             popq	%rsi
// 0x14258d824  415c                           popq	%r12
// 0x14258d826  415d                           popq	%r13
// 0x14258d828  415e                           popq	%r14
// 0x14258d82a  415f                           popq	%r15
// 0x14258d82c  5d                             popq	%rbp
// 0x14258d82d  c3                             retq
// 0x14258d82e  6690                           nop
// 0x14258d830  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258d835  55                             pushq	%rbp
// 0x14258d836  4157                           pushq	%r15
// 0x14258d838  4156                           pushq	%r14
// 0x14258d83a  4155                           pushq	%r13
// 0x14258d83c  4154                           pushq	%r12
// 0x14258d83e  56                             pushq	%rsi
// 0x14258d83f  57                             pushq	%rdi
// 0x14258d840  53                             pushq	%rbx
// 0x14258d841  4883ec28                       subq	$0x28, %rsp
// 0x14258d845  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F134 -> F91 -> F3 -> F1
// 0x14258d84c  80bd5e02000000                 cmpb	$0x0, 0x25e(%rbp)
// 0x14258d853  741f                           je	0x14258d874 <.text+0x258c874>
// Frames: F114 -> F99 -> F99 -> F99 -> F99 -> F134 -> F91 -> F3 -> F1
// 0x14258d855  488d95a0000000                 leaq	0xa0(%rbp), %rdx
// 0x14258d85c  488d8d80000000                 leaq	0x80(%rbp), %rcx
// Frames: F115 -> F99 -> F99 -> F99 -> F99 -> F134 -> F91 -> F3 -> F1
// 0x14258d863  41b820000000                   movl	$0x20, %r8d
// 0x14258d869  41b910000000                   movl	$0x10, %r9d
// 0x14258d86f  e8cc3aacfd                     callq	0x140051340 <.text+0x50340>
// 0x14258d874  90                             nop
// 0x14258d875  4883c428                       addq	$0x28, %rsp
// 0x14258d879  5b                             popq	%rbx
// 0x14258d87a  5f                             popq	%rdi
// 0x14258d87b  5e                             popq	%rsi
// 0x14258d87c  415c                           popq	%r12
// 0x14258d87e  415d                           popq	%r13
// 0x14258d880  415e                           popq	%r14
// 0x14258d882  415f                           popq	%r15
// 0x14258d884  5d                             popq	%rbp
// 0x14258d885  c3                             retq
// 0x14258d886  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x14258d890  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258d895  55                             pushq	%rbp
// 0x14258d896  4157                           pushq	%r15
// 0x14258d898  4156                           pushq	%r14
// 0x14258d89a  4155                           pushq	%r13
// 0x14258d89c  4154                           pushq	%r12
// 0x14258d89e  56                             pushq	%rsi
// 0x14258d89f  57                             pushq	%rdi
// 0x14258d8a0  53                             pushq	%rbx
// 0x14258d8a1  4883ec28                       subq	$0x28, %rsp
// 0x14258d8a5  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F116 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d8ac  488b9530020000                 movq	0x230(%rbp), %rdx
// Frames: F118 -> F116 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d8b3  4885d2                         testq	%rdx, %rdx
// 0x14258d8b6  7416                           je	0x14258d8ce <.text+0x258c8ce>
// Frames: F116 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d8b8  488b8d38020000                 movq	0x238(%rbp), %rcx
// Frames: F119 -> F120 -> F116 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d8bf  48c1e203                       shlq	$0x3, %rdx
// Frames: F121 -> F122 -> F123 -> F117 -> F99 -> F99 -> F3 -> F1
// 0x14258d8c3  41b808000000                   movl	$0x8, %r8d
// 0x14258d8c9  e8526308fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F135 -> F1
// 0x14258d8ce  90                             nop
// 0x14258d8cf  4883c428                       addq	$0x28, %rsp
// 0x14258d8d3  5b                             popq	%rbx
// 0x14258d8d4  5f                             popq	%rdi
// 0x14258d8d5  5e                             popq	%rsi
// 0x14258d8d6  415c                           popq	%r12
// 0x14258d8d8  415d                           popq	%r13
// 0x14258d8da  415e                           popq	%r14
// 0x14258d8dc  415f                           popq	%r15
// 0x14258d8de  5d                             popq	%rbp
// 0x14258d8df  c3                             retq
//
// RANGE rva_258d8e0_len_6f0: RVA 0x258d8e0, 1776 PDB bytes; 420 instructions
// Evidence: _assembly/roof-v1/B/pack-003/rva_258d8e0_len_6f0.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 4336:6724: system_roof::utils::components::impl$1::register_required_components
// Frames: F136
// 0x14258d8e0  55                             pushq	%rbp
// 0x14258d8e1  4157                           pushq	%r15
// 0x14258d8e3  4156                           pushq	%r14
// 0x14258d8e5  4155                           pushq	%r13
// 0x14258d8e7  4154                           pushq	%r12
// 0x14258d8e9  56                             pushq	%rsi
// 0x14258d8ea  57                             pushq	%rdi
// 0x14258d8eb  53                             pushq	%rbx
// 0x14258d8ec  4881ece8020000                 subq	$0x2e8, %rsp            # imm = 0x2E8
// 0x14258d8f3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14258d8fb  48c78560020000feffffff         movq	$-0x2, 0x260(%rbp)
// 0x14258d906  4989d6                         movq	%rdx, %r14
// 0x14258d909  488bb5d0020000                 movq	0x2d0(%rbp), %rsi
// 0x14258d910  4c8b3a                         movq	(%rdx), %r15
// 0x14258d913  488b5e08                       movq	0x8(%rsi), %rbx
// 0x14258d917  488b7e10                       movq	0x10(%rsi), %rdi
// 0x14258d91b  4c89f9                         movq	%r15, %rcx
// 0x14258d91e  4889da                         movq	%rbx, %rdx
// 0x14258d921  4989f8                         movq	%rdi, %r8
// 0x14258d924  e847347bfe                     callq	0x140d40d70 <.text+0xd3fd70>
// Frames: F2 -> F3 -> F136
// 0x14258d929  48c7853002000000000000         movq	$0x0, 0x230(%rbp)
// 0x14258d934  48c7853802000008000000         movq	$0x8, 0x238(%rbp)
// 0x14258d93f  48c7854002000000000000         movq	$0x0, 0x240(%rbp)
// Frames: F4 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d94a  49837f3000                     cmpq	$0x0, 0x30(%r15)
// 0x14258d94f  48b858ca46d1a27e5a36           movabsq	$0x365a7ea2d146ca58, %rax # imm = 0x365A7EA2D146CA58
// 0x14258d959  48b9d8585636c0c1b41b           movabsq	$0x1bb4c1c0365658d8, %rcx # imm = 0x1BB4C1C0365658D8
// 0x14258d963  0f849a000000                   je	0x14258da03 <.text+0x258ca03>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d969  498b5718                       movq	0x18(%r15), %rdx
// 0x14258d96d  4d8b4720                       movq	0x20(%r15), %r8
// 0x14258d971  4d89c1                         movq	%r8, %r9
// 0x14258d974  4921c1                         andq	%rax, %r9
// 0x14258d977  4531d2                         xorl	%r10d, %r10d
// 0x14258d97a  660f6f05be2c3c00               movdqa	0x3c2cbe(%rip), %xmm0   # 0x142950640
// 0x14258d982  660f76c9                       pcmpeqd	%xmm1, %xmm1
// Frames: F13 -> F14 -> F15 -> F16 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d986  f3420f6f140a                   movdqu	(%rdx,%r9), %xmm2
// Frames: F17 -> F18 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d98c  660f6fda                       movdqa	%xmm2, %xmm3
// 0x14258d990  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x14258d994  66440fd7db                     pmovmskb	%xmm3, %r11d
// Frames: F20 -> F21 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d999  4585db                         testl	%r11d, %r11d
// 0x14258d99c  752f                           jne	0x14258d9cd <.text+0x258c9cd>
// Frames: F17 -> F18 -> F22 -> F23 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d99e  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x14258d9a2  66440fd7da                     pmovmskb	%xmm2, %r11d
// Frames: F23 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9a7  4585db                         testl	%r11d, %r11d
// 0x14258d9aa  7557                           jne	0x14258da03 <.text+0x258ca03>
// Frames: F24 -> F25 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9ac  4d01d1                         addq	%r10, %r9
// 0x14258d9af  4983c110                       addq	$0x10, %r9
// Frames: F26 -> F25 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9b3  4983c210                       addq	$0x10, %r10
// Frames: F27 -> F25 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9b7  4d21c1                         andq	%r8, %r9
// Frames: F28 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9ba  ebca                           jmp	0x14258d986 <.text+0x258c986>
// 0x14258d9bc  0f1f4000                       nopl	(%rax)
// Frames: F29 -> F30 -> F31 -> F32 -> F21 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9c0  458d63ff                       leal	-0x1(%r11), %r12d
// Frames: F20 -> F21 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9c4  664521dc                       andw	%r11w, %r12w
// 0x14258d9c8  4589e3                         movl	%r12d, %r11d
// 0x14258d9cb  74d1                           je	0x14258d99e <.text+0x258c99e>
// Frames: F30 -> F31 -> F32 -> F21 -> F19 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9cd  f3450fbce3                     tzcntl	%r11d, %r12d
// Frames: F33 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9d2  4d01cc                         addq	%r9, %r12
// 0x14258d9d5  4d21c4                         andq	%r8, %r12
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9d8  49f7dc                         negq	%r12
// 0x14258d9db  4f8d2464                       leaq	(%r12,%r12,2), %r12
// Frames: F39 -> F40 -> F41 -> F42 -> F43 -> F37 -> F38 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9df  4a394ce2e8                     cmpq	%rcx, -0x18(%rdx,%r12,8)
// Frames: F38 -> F10 -> F11 -> F12 -> F5 -> F6 -> F7 -> F3 -> F136
// 0x14258d9e4  75da                           jne	0x14258d9c0 <.text+0x258c9c0>
// 0x14258d9e6  4a3944e2f0                     cmpq	%rax, -0x10(%rdx,%r12,8)
// 0x14258d9eb  75d3                           jne	0x14258d9c0 <.text+0x258c9c0>
// Frames: F44 -> F3 -> F136
// 0x14258d9ed  4a8d04e2                       leaq	(%rdx,%r12,8), %rax
// 0x14258d9f1  4c8b60f8                       movq	-0x8(%rax), %r12
// Frames: F45 -> F136
// 0x14258d9f5  483b3e                         cmpq	(%rsi), %rdi
// 0x14258d9f8  0f8452030000                   je	0x14258dd50 <.text+0x258cd50>
// 0x14258d9fe  e9f6030000                     jmp	0x14258ddf9 <.text+0x258cdf9>
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F3 -> F136
// 0x14258da03  48899d48020000                 movq	%rbx, 0x248(%rbp)
// 0x14258da0a  410fb65760                     movzbl	0x60(%r15), %edx
// 0x14258da0f  498b9780000000                 movq	0x80(%r15), %rdx
// Frames: F8 -> F9 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da16  4d8b8f88000000                 movq	0x88(%r15), %r9
// 0x14258da1d  4d89ca                         movq	%r9, %r10
// 0x14258da20  4921c2                         andq	%rax, %r10
// 0x14258da23  4531db                         xorl	%r11d, %r11d
// 0x14258da26  660f6f05122c3c00               movdqa	0x3c2c12(%rip), %xmm0   # 0x142950640
// 0x14258da2e  660f76c9                       pcmpeqd	%xmm1, %xmm1
// Frames: F13 -> F14 -> F15 -> F16 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da32  f3420f6f1412                   movdqu	(%rdx,%r10), %xmm2
// Frames: F17 -> F18 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da38  660f6fda                       movdqa	%xmm2, %xmm3
// 0x14258da3c  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x14258da40  66440fd7e3                     pmovmskb	%xmm3, %r12d
// Frames: F20 -> F21 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da45  4585e4                         testl	%r12d, %r12d
// 0x14258da48  7534                           jne	0x14258da7e <.text+0x258ca7e>
// Frames: F17 -> F18 -> F22 -> F23 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da4a  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x14258da4e  66440fd7c2                     pmovmskb	%xmm2, %r8d
// Frames: F23 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da53  4585c0                         testl	%r8d, %r8d
// 0x14258da56  0f85f2000000                   jne	0x14258db4e <.text+0x258cb4e>
// Frames: F24 -> F25 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da5c  4d01da                         addq	%r11, %r10
// 0x14258da5f  4983c210                       addq	$0x10, %r10
// Frames: F26 -> F25 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da63  4983c310                       addq	$0x10, %r11
// Frames: F27 -> F25 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da67  4d21ca                         andq	%r9, %r10
// Frames: F28 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da6a  ebc6                           jmp	0x14258da32 <.text+0x258ca32>
// 0x14258da6c  0f1f4000                       nopl	(%rax)
// Frames: F29 -> F30 -> F31 -> F32 -> F21 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da70  458d4424ff                     leal	-0x1(%r12), %r8d
// Frames: F20 -> F21 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da75  664521e0                       andw	%r12w, %r8w
// 0x14258da79  4589c4                         movl	%r8d, %r12d
// 0x14258da7c  74cc                           je	0x14258da4a <.text+0x258ca4a>
// Frames: F30 -> F31 -> F32 -> F21 -> F19 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da7e  f3450fbcec                     tzcntl	%r12d, %r13d
// Frames: F33 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da83  4d01d5                         addq	%r10, %r13
// 0x14258da86  4d21cd                         andq	%r9, %r13
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da89  4d89e8                         movq	%r13, %r8
// 0x14258da8c  49f7d8                         negq	%r8
// 0x14258da8f  4a8d1ced00000000               leaq	(,%r13,8), %rbx
// 0x14258da97  49c1e007                       shlq	$0x7, %r8
// 0x14258da9b  4929d8                         subq	%rbx, %r8
// Frames: F39 -> F40 -> F41 -> F42 -> F43 -> F37 -> F38 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258da9e  4a398c0278ffffff               cmpq	%rcx, -0x88(%rdx,%r8)
// Frames: F38 -> F10 -> F52 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258daa6  75c8                           jne	0x14258da70 <.text+0x258ca70>
// 0x14258daa8  4a39440280                     cmpq	%rax, -0x80(%rdx,%r8)
// 0x14258daad  75c1                           jne	0x14258da70 <.text+0x258ca70>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258daaf  4d89ea                         movq	%r13, %r10
// 0x14258dab2  49c1e207                       shlq	$0x7, %r10
// 0x14258dab6  4f8d1cea                       leaq	(%r10,%r13,8), %r11
// 0x14258daba  49c1fb03                       sarq	$0x3, %r11
// 0x14258dabe  49baf1f0f0f0f0f0f0f0           movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
// 0x14258dac8  4d0fafd3                       imulq	%r11, %r10
// Frames: F64 -> F65 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258dacc  4d8d5af0                       leaq	-0x10(%r10), %r11
// Frames: F65 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258dad0  4d21cb                         andq	%r9, %r11
// Frames: F13 -> F14 -> F15 -> F67 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258dad3  f3420f6f041a                   movdqu	(%rdx,%r11), %xmm0
// Frames: F68 -> F69 -> F70 -> F67 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258dad9  660f76c9                       pcmpeqd	%xmm1, %xmm1
// Frames: F17 -> F18 -> F70 -> F67 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258dadd  660f74c1                       pcmpeqb	%xmm1, %xmm0
// 0x14258dae1  66440fd7e0                     pmovmskb	%xmm0, %r12d
// Frames: F13 -> F14 -> F15 -> F71 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258dae6  f3420f6f0412                   movdqu	(%rdx,%r10), %xmm0
// Frames: F17 -> F18 -> F70 -> F71 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258daec  660f74c1                       pcmpeqb	%xmm1, %xmm0
// 0x14258daf0  66440fd7c8                     pmovmskb	%xmm0, %r9d
// Frames: F72 -> F73 -> F74 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258daf5  4585e4                         testl	%r12d, %r12d
// 0x14258daf8  740b                           je	0x14258db05 <.text+0x258cb05>
// 0x14258dafa  66450fbde4                     bsrw	%r12w, %r12w
// 0x14258daff  4183f40f                       xorl	$0xf, %r12d
// 0x14258db03  eb05                           jmp	0x14258db0a <.text+0x258cb0a>
// 0x14258db05  6641bc1000                     movw	$0x10, %r12w
// Frames: F75 -> F76 -> F74 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258db0a  4901d0                         addq	%rdx, %r8
// 0x14258db0d  4181c900000100                 orl	$0x10000, %r9d          # imm = 0x10000
// 0x14258db14  f3450fbcc9                     tzcntl	%r9d, %r9d
// Frames: F74 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258db19  4501cc                         addl	%r9d, %r12d
// 0x14258db1c  41b180                         movb	$-0x80, %r9b
// 0x14258db1f  664183fc0f                     cmpw	$0xf, %r12w
// 0x14258db24  770a                           ja	0x14258db30 <.text+0x258cb30>
// Frames: F77 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258db26  49ff8790000000                 incq	0x90(%r15)
// 0x14258db2d  41b1ff                         movb	$-0x1, %r9b
// Frames: F78 -> F79 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258db30  46880c12                       movb	%r9b, (%rdx,%r10)
// Frames: F80 -> F79 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258db34  46884c1a10                     movb	%r9b, 0x10(%rdx,%r11)
// Frames: F81 -> F66 -> F62 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258db39  49ff8f98000000                 decq	0x98(%r15)
// Frames: F82 -> F83 -> F84 -> F85 -> F63 -> F53 -> F54 -> F55 -> F56 -> F3 -> F136
// 0x14258db40  498b5088                       movq	-0x78(%r8), %rdx
// Frames: F54 -> F55 -> F56 -> F3 -> F136
// 0x14258db44  4883fa02                       cmpq	$0x2, %rdx
// 0x14258db48  0f8526020000                   jne	0x14258dd74 <.text+0x258cd74>
// Frames: F86 -> F3 -> F136
// 0x14258db4e  498b5608                       movq	0x8(%r14), %rdx
// Frames: F87 -> F86 -> F3 -> F136
// 0x14258db52  4c8b22                         movq	(%rdx), %r12
// Frames: F88 -> F86 -> F3 -> F136
// 0x14258db55  4d8d6c2401                     leaq	0x1(%r12), %r13
// 0x14258db5a  4c892a                         movq	%r13, (%rdx)
// 0x14258db5d  48ba0000000000000080           movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
// Frames: F89 -> F90 -> F91 -> F3 -> F136
// 0x14258db67  48899508020000                 movq	%rdx, 0x208(%rbp)
// 0x14258db6e  488d156cf19100                 leaq	0x91f16c(%rip), %rdx    # 0x142eacce1
// 0x14258db75  48899510020000                 movq	%rdx, 0x210(%rbp)
// 0x14258db7c  48c7851802000030000000         movq	$0x30, 0x218(%rbp)
// 0x14258db87  66c785280200000001             movw	$0x100, 0x228(%rbp)     # imm = 0x100
// 0x14258db90  48c785d001000001000000         movq	$0x1, 0x1d0(%rbp)
// 0x14258db9b  48898dd8010000                 movq	%rcx, 0x1d8(%rbp)
// 0x14258dba2  488985e0010000                 movq	%rax, 0x1e0(%rbp)
// 0x14258dba9  48c785f801000001000000         movq	$0x1, 0x1f8(%rbp)
// 0x14258dbb4  48c7850002000000000000         movq	$0x0, 0x200(%rbp)
// 0x14258dbbf  48c7852002000000000000         movq	$0x0, 0x220(%rbp)
// 0x14258dbca  c6852a02000001                 movb	$0x1, 0x22a(%rbp)
// 0x14258dbd1  48c785e801000000000000         movq	$0x0, 0x1e8(%rbp)
// Frames: F92 -> F90 -> F91 -> F3 -> F136
// 0x14258dbdc  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x14258dbe3  4c8d85d0010000                 leaq	0x1d0(%rbp), %r8
// 0x14258dbea  4c89e2                         movq	%r12, %rdx
// 0x14258dbed  e87e1c7bfe                     callq	0x140d3f870 <.text+0xd3e870>
// Frames: F93 -> F90 -> F91 -> F3 -> F136
// 0x14258dbf2  4d396f10                       cmpq	%r13, 0x10(%r15)
// 0x14258dbf6  7319                           jae	0x14258dc11 <.text+0x258cc11>
// Frames: F94 -> F90 -> F91 -> F3 -> F136
// 0x14258dbf8  c6855f02000001                 movb	$0x1, 0x25f(%rbp)
// 0x14258dbff  4c8d054a45b700                 leaq	0xb7454a(%rip), %r8     # 0x143102150
// 0x14258dc06  4c89f9                         movq	%r15, %rcx
// 0x14258dc09  4c89ea                         movq	%r13, %rdx
// 0x14258dc0c  e8bf0184fe                     callq	0x140dcddd0 <.text+0xdccdd0>
// Frames: F95 -> F96 -> F97 -> F90 -> F91 -> F3 -> F136
// 0x14258dc11  498b5f08                       movq	0x8(%r15), %rbx
// 0x14258dc15  4d69fcd0000000                 imulq	$0xd0, %r12, %r15
// 0x14258dc1c  4a8d043b                       leaq	(%rbx,%r15), %rax
// 0x14258dc20  48898550020000                 movq	%rax, 0x250(%rbp)
// 0x14258dc27  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14258dc2b  488d9580000000                 leaq	0x80(%rbp), %rdx
// Frames: F98 -> F90 -> F91 -> F3 -> F136
// 0x14258dc32  41b8d0000000                   movl	$0xd0, %r8d
// 0x14258dc38  e8c35f3900                     callq	0x142923c00 <.text+0x2922c00>
// Frames: F99 -> F98 -> F90 -> F91 -> F3 -> F136
// 0x14258dc3d  42833c3b02                     cmpl	$0x2, (%rbx,%r15)
// 0x14258dc42  740c                           je	0x14258dc50 <.text+0x258cc50>
// 0x14258dc44  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14258dc4b  e8002f06fe                     callq	0x1405f0b50 <.text+0x5efb50>
// Frames: F98 -> F90 -> F91 -> F3 -> F136
// 0x14258dc50  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14258dc54  41b8d0000000                   movl	$0xd0, %r8d
// 0x14258dc5a  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14258dc61  e89a5f3900                     callq	0x142923c00 <.text+0x2922c00>
// Frames: F100 -> F101 -> F91 -> F3 -> F136
// 0x14258dc66  498b0e                         movq	(%r14), %rcx
// Frames: F102 -> F101 -> F91 -> F3 -> F136
// 0x14258dc69  4883c118                       addq	$0x18, %rcx
// 0x14258dc6d  48bad8585636c0c1b41b           movabsq	$0x1bb4c1c0365658d8, %rdx # imm = 0x1BB4C1C0365658D8
// 0x14258dc77  49b858ca46d1a27e5a36           movabsq	$0x365a7ea2d146ca58, %r8 # imm = 0x365A7EA2D146CA58
// 0x14258dc81  4d89e1                         movq	%r12, %r9
// 0x14258dc84  e8974faffd                     callq	0x140082c20 <.text+0x81c20>
// Frames: F103 -> F104 -> F91 -> F3 -> F136
// 0x14258dc89  0f10055044b700                 movups	0xb74450(%rip), %xmm0   # 0x1431020e0
// 0x14258dc90  0f298590000000                 movaps	%xmm0, 0x90(%rbp)
// 0x14258dc97  f30f6f053144b700               movdqu	0xb74431(%rip), %xmm0   # 0x1431020d0
// 0x14258dc9f  660f7f8580000000               movdqa	%xmm0, 0x80(%rbp)
// 0x14258dca7  c6855e02000001                 movb	$0x1, 0x25e(%rbp)
// Frames: F105 -> F91 -> F3 -> F136
// 0x14258dcae  488d8530020000                 leaq	0x230(%rbp), %rax
// 0x14258dcb5  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14258dcba  4c89f2                         movq	%r14, %rdx
// 0x14258dcbd  e81efcffff                     callq	0x14258d8e0 <.text+0x258c8e0>
// Frames: F106 -> F91 -> F3 -> F136
// 0x14258dcc2  498b06                         movq	(%r14), %rax
// Frames: F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F106 -> F91 -> F3 -> F136
// 0x14258dcc5  488b4008                       movq	0x8(%rax), %rax
// Frames: F113 -> F91 -> F3 -> F136
// 0x14258dcc9  0f288580000000                 movaps	0x80(%rbp), %xmm0
// 0x14258dcd0  0f288d90000000                 movaps	0x90(%rbp), %xmm1
// 0x14258dcd7  0f294dc0                       movaps	%xmm1, -0x40(%rbp)
// 0x14258dcdb  0f2945b0                       movaps	%xmm0, -0x50(%rbp)
// 0x14258dcdf  4a8d4c3860                     leaq	0x60(%rax,%r15), %rcx
// Frames: F114 -> F99 -> F99 -> F99 -> F99 -> F113 -> F91 -> F3 -> F136
// 0x14258dce4  4a8d943880000000               leaq	0x80(%rax,%r15), %rdx
// Frames: F115 -> F99 -> F99 -> F99 -> F99 -> F113 -> F91 -> F3 -> F136
// 0x14258dcec  41b820000000                   movl	$0x20, %r8d
// 0x14258dcf2  41b910000000                   movl	$0x10, %r9d
// 0x14258dcf8  48898d50020000                 movq	%rcx, 0x250(%rbp)
// 0x14258dcff  e83c36acfd                     callq	0x140051340 <.text+0x50340>
// 0x14258dd04  488b9d48020000                 movq	0x248(%rbp), %rbx
// Frames: F113 -> F91 -> F3 -> F136
// 0x14258dd0b  660f6f45b0                     movdqa	-0x50(%rbp), %xmm0
// 0x14258dd10  660f6f4dc0                     movdqa	-0x40(%rbp), %xmm1
// 0x14258dd15  488b8550020000                 movq	0x250(%rbp), %rax
// 0x14258dd1c  f30f7f4810                     movdqu	%xmm1, 0x10(%rax)
// 0x14258dd21  f30f7f00                       movdqu	%xmm0, (%rax)
// Frames: F116 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dd25  488b9530020000                 movq	0x230(%rbp), %rdx
// Frames: F118 -> F116 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dd2c  4885d2                         testq	%rdx, %rdx
// 0x14258dd2f  7416                           je	0x14258dd47 <.text+0x258cd47>
// Frames: F116 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dd31  488b8d38020000                 movq	0x238(%rbp), %rcx
// Frames: F119 -> F120 -> F116 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dd38  48c1e203                       shlq	$0x3, %rdx
// Frames: F121 -> F122 -> F123 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dd3c  41b808000000                   movl	$0x8, %r8d
// 0x14258dd42  e8d95e08fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F45 -> F136
// 0x14258dd47  483b3e                         cmpq	(%rsi), %rdi
// 0x14258dd4a  0f85a9000000                   jne	0x14258ddf9 <.text+0x258cdf9>
// Frames: F124 -> F136
// 0x14258dd50  488d155944b700                 leaq	0xb74459(%rip), %rdx    # 0x1431021b0
// 0x14258dd57  4889f1                         movq	%rsi, %rcx
// 0x14258dd5a  e891405ffe                     callq	0x140b81df0 <.text+0xb80df0>
// Frames: F107 -> F108 -> F109 -> F110 -> F125 -> F136
// 0x14258dd5f  488b4608                       movq	0x8(%rsi), %rax
// Frames: F126 -> F127 -> F136
// 0x14258dd63  4c8924f8                       movq	%r12, (%rax,%rdi,8)
// Frames: F128 -> F136
// 0x14258dd67  488d4701                       leaq	0x1(%rdi), %rax
// 0x14258dd6b  48894610                       movq	%rax, 0x10(%rsi)
// Frames: F129 -> F136
// 0x14258dd6f  e995000000                     jmp	0x14258de09 <.text+0x258ce09>
// Frames: F130 -> F55 -> F56 -> F3 -> F136
// 0x14258dd74  410f1040f0                     movups	-0x10(%r8), %xmm0
// 0x14258dd79  0f1185c0010000                 movups	%xmm0, 0x1c0(%rbp)
// 0x14258dd80  410f1040e0                     movups	-0x20(%r8), %xmm0
// 0x14258dd85  0f1185b0010000                 movups	%xmm0, 0x1b0(%rbp)
// 0x14258dd8c  410f1040d0                     movups	-0x30(%r8), %xmm0
// 0x14258dd91  0f1185a0010000                 movups	%xmm0, 0x1a0(%rbp)
// 0x14258dd98  f3410f6f4090                   movdqu	-0x70(%r8), %xmm0
// 0x14258dd9e  f3410f6f48a0                   movdqu	-0x60(%r8), %xmm1
// 0x14258dda4  f3410f6f50b0                   movdqu	-0x50(%r8), %xmm2
// 0x14258ddaa  f3410f6f58c0                   movdqu	-0x40(%r8), %xmm3
// 0x14258ddb0  f30f7f9d90010000               movdqu	%xmm3, 0x190(%rbp)
// 0x14258ddb8  f30f7f9580010000               movdqu	%xmm2, 0x180(%rbp)
// 0x14258ddc0  f30f7f8d70010000               movdqu	%xmm1, 0x170(%rbp)
// 0x14258ddc8  f30f7f8560010000               movdqu	%xmm0, 0x160(%rbp)
// Frames: F51 -> F3 -> F136
// 0x14258ddd0  48899558010000                 movq	%rdx, 0x158(%rbp)
// Frames: F131 -> F3 -> F136
// 0x14258ddd7  488d8d58010000                 leaq	0x158(%rbp), %rcx
// 0x14258ddde  4c89f2                         movq	%r14, %rdx
// 0x14258dde1  e85a1c7bfe                     callq	0x140d3fa40 <.text+0xd3ea40>
// Frames: F45 -> F136
// 0x14258dde6  4989c4                         movq	%rax, %r12
// 0x14258dde9  488b9d48020000                 movq	0x248(%rbp), %rbx
// 0x14258ddf0  483b3e                         cmpq	(%rsi), %rdi
// 0x14258ddf3  0f8457ffffff                   je	0x14258dd50 <.text+0x258cd50>
// Frames: F126 -> F127 -> F136
// 0x14258ddf9  4c8924fb                       movq	%r12, (%rbx,%rdi,8)
// Frames: F128 -> F136
// 0x14258ddfd  4889f8                         movq	%rdi, %rax
// 0x14258de00  48ffc0                         incq	%rax
// 0x14258de03  48894610                       movq	%rax, 0x10(%rsi)
// Frames: F129 -> F136
// 0x14258de07  7404                           je	0x14258de0d <.text+0x258ce0d>
// Frames: F132 -> F136
// 0x14258de09  48897e10                       movq	%rdi, 0x10(%rsi)
// Frames: F136
// 0x14258de0d  4881c4e8020000                 addq	$0x2e8, %rsp            # imm = 0x2E8
// 0x14258de14  5b                             popq	%rbx
// 0x14258de15  5f                             popq	%rdi
// 0x14258de16  5e                             popq	%rsi
// 0x14258de17  415c                           popq	%r12
// 0x14258de19  415d                           popq	%r13
// 0x14258de1b  415e                           popq	%r14
// 0x14258de1d  415f                           popq	%r15
// 0x14258de1f  5d                             popq	%rbp
// 0x14258de20  c3                             retq
// 0x14258de21  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// 0x14258de30  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258de35  55                             pushq	%rbp
// 0x14258de36  4157                           pushq	%r15
// 0x14258de38  4156                           pushq	%r14
// 0x14258de3a  4155                           pushq	%r13
// 0x14258de3c  4154                           pushq	%r12
// 0x14258de3e  56                             pushq	%rsi
// 0x14258de3f  57                             pushq	%rdi
// 0x14258de40  53                             pushq	%rbx
// 0x14258de41  4883ec28                       subq	$0x28, %rsp
// 0x14258de45  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14258de4c  488d55b0                       leaq	-0x50(%rbp), %rdx
// Frames: F98 -> F90 -> F91 -> F3 -> F136
// 0x14258de50  41b8d0000000                   movl	$0xd0, %r8d
// 0x14258de56  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14258de5d  e89e5d3900                     callq	0x142923c00 <.text+0x2922c00>
// 0x14258de62  c6855f02000000                 movb	$0x0, 0x25f(%rbp)
// 0x14258de69  4883c428                       addq	$0x28, %rsp
// 0x14258de6d  5b                             popq	%rbx
// 0x14258de6e  5f                             popq	%rdi
// 0x14258de6f  5e                             popq	%rsi
// 0x14258de70  415c                           popq	%r12
// 0x14258de72  415d                           popq	%r13
// 0x14258de74  415e                           popq	%r14
// 0x14258de76  415f                           popq	%r15
// 0x14258de78  5d                             popq	%rbp
// 0x14258de79  c3                             retq
// 0x14258de7a  660f1f440000                   nopw	(%rax,%rax)
// 0x14258de80  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258de85  55                             pushq	%rbp
// 0x14258de86  4157                           pushq	%r15
// 0x14258de88  4156                           pushq	%r14
// 0x14258de8a  4155                           pushq	%r13
// 0x14258de8c  4154                           pushq	%r12
// 0x14258de8e  56                             pushq	%rsi
// 0x14258de8f  57                             pushq	%rdi
// 0x14258de90  53                             pushq	%rbx
// 0x14258de91  4883ec28                       subq	$0x28, %rsp
// 0x14258de95  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F133 -> F90 -> F91 -> F3 -> F136
// 0x14258de9c  80bd5f02000000                 cmpb	$0x0, 0x25f(%rbp)
// 0x14258dea3  740c                           je	0x14258deb1 <.text+0x258ceb1>
// 0x14258dea5  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x14258deac  e89f2c06fe                     callq	0x1405f0b50 <.text+0x5efb50>
// 0x14258deb1  90                             nop
// 0x14258deb2  4883c428                       addq	$0x28, %rsp
// 0x14258deb6  5b                             popq	%rbx
// 0x14258deb7  5f                             popq	%rdi
// 0x14258deb8  5e                             popq	%rsi
// 0x14258deb9  415c                           popq	%r12
// 0x14258debb  415d                           popq	%r13
// 0x14258debd  415e                           popq	%r14
// 0x14258debf  415f                           popq	%r15
// 0x14258dec1  5d                             popq	%rbp
// 0x14258dec2  c3                             retq
// 0x14258dec3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x14258ded0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258ded5  55                             pushq	%rbp
// 0x14258ded6  4157                           pushq	%r15
// 0x14258ded8  4156                           pushq	%r14
// 0x14258deda  4155                           pushq	%r13
// 0x14258dedc  4154                           pushq	%r12
// 0x14258dede  56                             pushq	%rsi
// 0x14258dedf  57                             pushq	%rdi
// 0x14258dee0  53                             pushq	%rbx
// 0x14258dee1  4883ec28                       subq	$0x28, %rsp
// 0x14258dee5  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F113 -> F91 -> F3 -> F136
// 0x14258deec  660f6f45b0                     movdqa	-0x50(%rbp), %xmm0
// 0x14258def1  660f6f4dc0                     movdqa	-0x40(%rbp), %xmm1
// 0x14258def6  488b8550020000                 movq	0x250(%rbp), %rax
// 0x14258defd  f30f7f4810                     movdqu	%xmm1, 0x10(%rax)
// 0x14258df02  f30f7f00                       movdqu	%xmm0, (%rax)
// 0x14258df06  c6855e02000000                 movb	$0x0, 0x25e(%rbp)
// 0x14258df0d  4883c428                       addq	$0x28, %rsp
// 0x14258df11  5b                             popq	%rbx
// 0x14258df12  5f                             popq	%rdi
// 0x14258df13  5e                             popq	%rsi
// 0x14258df14  415c                           popq	%r12
// 0x14258df16  415d                           popq	%r13
// 0x14258df18  415e                           popq	%r14
// 0x14258df1a  415f                           popq	%r15
// 0x14258df1c  5d                             popq	%rbp
// 0x14258df1d  c3                             retq
// 0x14258df1e  6690                           nop
// 0x14258df20  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258df25  55                             pushq	%rbp
// 0x14258df26  4157                           pushq	%r15
// 0x14258df28  4156                           pushq	%r14
// 0x14258df2a  4155                           pushq	%r13
// 0x14258df2c  4154                           pushq	%r12
// 0x14258df2e  56                             pushq	%rsi
// 0x14258df2f  57                             pushq	%rdi
// 0x14258df30  53                             pushq	%rbx
// 0x14258df31  4883ec28                       subq	$0x28, %rsp
// 0x14258df35  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F134 -> F91 -> F3 -> F136
// 0x14258df3c  80bd5e02000000                 cmpb	$0x0, 0x25e(%rbp)
// 0x14258df43  741f                           je	0x14258df64 <.text+0x258cf64>
// Frames: F114 -> F99 -> F99 -> F99 -> F99 -> F134 -> F91 -> F3 -> F136
// 0x14258df45  488d95a0000000                 leaq	0xa0(%rbp), %rdx
// 0x14258df4c  488d8d80000000                 leaq	0x80(%rbp), %rcx
// Frames: F115 -> F99 -> F99 -> F99 -> F99 -> F134 -> F91 -> F3 -> F136
// 0x14258df53  41b820000000                   movl	$0x20, %r8d
// 0x14258df59  41b910000000                   movl	$0x10, %r9d
// 0x14258df5f  e8dc33acfd                     callq	0x140051340 <.text+0x50340>
// 0x14258df64  90                             nop
// 0x14258df65  4883c428                       addq	$0x28, %rsp
// 0x14258df69  5b                             popq	%rbx
// 0x14258df6a  5f                             popq	%rdi
// 0x14258df6b  5e                             popq	%rsi
// 0x14258df6c  415c                           popq	%r12
// 0x14258df6e  415d                           popq	%r13
// 0x14258df70  415e                           popq	%r14
// 0x14258df72  415f                           popq	%r15
// 0x14258df74  5d                             popq	%rbp
// 0x14258df75  c3                             retq
// 0x14258df76  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x14258df80  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14258df85  55                             pushq	%rbp
// 0x14258df86  4157                           pushq	%r15
// 0x14258df88  4156                           pushq	%r14
// 0x14258df8a  4155                           pushq	%r13
// 0x14258df8c  4154                           pushq	%r12
// 0x14258df8e  56                             pushq	%rsi
// 0x14258df8f  57                             pushq	%rdi
// 0x14258df90  53                             pushq	%rbx
// 0x14258df91  4883ec28                       subq	$0x28, %rsp
// 0x14258df95  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F116 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258df9c  488b9530020000                 movq	0x230(%rbp), %rdx
// Frames: F118 -> F116 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dfa3  4885d2                         testq	%rdx, %rdx
// 0x14258dfa6  7416                           je	0x14258dfbe <.text+0x258cfbe>
// Frames: F116 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dfa8  488b8d38020000                 movq	0x238(%rbp), %rcx
// Frames: F119 -> F120 -> F116 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dfaf  48c1e203                       shlq	$0x3, %rdx
// Frames: F121 -> F122 -> F123 -> F117 -> F99 -> F99 -> F3 -> F136
// 0x14258dfb3  41b808000000                   movl	$0x8, %r8d
// 0x14258dfb9  e8625c08fe                     callq	0x140613c20 <.text+0x612c20>
// Frames: F135 -> F136
// 0x14258dfbe  90                             nop
// 0x14258dfbf  4883c428                       addq	$0x28, %rsp
// 0x14258dfc3  5b                             popq	%rbx
// 0x14258dfc4  5f                             popq	%rdi
// 0x14258dfc5  5e                             popq	%rsi
// 0x14258dfc6  415c                           popq	%r12
// 0x14258dfc8  415d                           popq	%r13
// 0x14258dfca  415e                           popq	%r14
// 0x14258dfcc  415f                           popq	%r15
// 0x14258dfce  5d                             popq	%rbp
// 0x14258dfcf  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/utils/components.rs:3:0 _$LT$system_roof..utils..components..RoofSpire$u20$as$u20$bevy_ecs..component..Component$GT$::register_required_components::h4b99bb84469127f1
// F2: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:423:0 alloc::vec::Vec::new
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1716:0 bevy_ecs::component::ComponentsRegistrator::register_component
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs:1351:0 hashbrown::map::HashMap::get_inner
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs:1309:0 hashbrown::map::HashMap::get
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs:937:0 bevy_platform::collections::hash_map::HashMap::get
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1726:0 bevy_ecs::component::ComponentsRegistrator::register_component_checked
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:2305:0 hashbrown::raw::RawTableInner::probe_seq
// F9: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1864:0 hashbrown::raw::RawTableInner::find_inner
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1164:0 hashbrown::raw::RawTable::find
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1181:0 hashbrown::raw::RawTable::get
// F12: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs:1355:0 hashbrown::map::HashMap::get_inner
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:4441:0 core::intrinsics::copy_nonoverlapping
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1229:0 core::core_arch::x86::sse2::_mm_loadu_si128
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs:52:0 hashbrown::control::group::sse2::Group::load
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1881:0 hashbrown::raw::RawTableInner::find_inner
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1436:0 core::core_arch::x86::sse2::_mm_movemask_epi8
// F18: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs:87:0 hashbrown::control::group::sse2::Group::match_tag
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1883:0 hashbrown::raw::RawTableInner::find_inner
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs:50:0 hashbrown::control::bitmask::BitMask::lowest_set_bit
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs:113:0 hashbrown::control::bitmask::impl$2::next
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs:95:0 hashbrown::control::group::sse2::Group::match_empty
// F23: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1893:0 hashbrown::raw::RawTableInner::find_inner
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:89:0 hashbrown::raw::ProbeSeq::move_next
// F25: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1897:0 hashbrown::raw::RawTableInner::find_inner
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:88:0 hashbrown::raw::ProbeSeq::move_next
// F27: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:90:0 hashbrown::raw::ProbeSeq::move_next
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1866:0 hashbrown::raw::RawTableInner::find_inner
// F29: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:468:0 core::num::nonzero::NonZero::get
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:604:0 core::num::nonzero::NonZero::trailing_zeros
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs:80:0 hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs:51:0 hashbrown::control::bitmask::BitMask::lowest_set_bit
// F33: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1886:0 hashbrown::raw::RawTableInner::find_inner
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1168:0 core::ptr::mut_ptr::impl$0::sub
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:288:0 hashbrown::raw::Bucket::from_base_index
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:761:0 hashbrown::raw::RawTable::bucket
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1166:0 hashbrown::raw::impl$8::find::closure$0
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:1888:0 hashbrown::raw::RawTableInner::find_inner
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1740:0 core::cmp::impls::impl$25::eq
// F40: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/tuple.rs:31:0 core::tuple::impl$10::eq
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/any.rs:720:0 core::any::impl$7::eq
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs:89:0 equivalent::impl$0::equivalent
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs:224:0 hashbrown::map::equivalent_key::closure$0
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1727:0 bevy_ecs::component::ComponentsRegistrator::register_component_checked
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2422:0 alloc::vec::Vec::push
// F46: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3731:0 core::sync::atomic::atomic_load
// F47: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:626:0 core::sync::atomic::AtomicBool::load
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison.rs:145:0 std::sync::poison::Flag::get
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison.rs:117:0 std::sync::poison::Flag::borrow
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison/rwlock.rs:633:0 std::sync::poison::rwlock::RwLock::get_mut
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1730:0 bevy_ecs::component::ComponentsRegistrator::register_component_checked
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:799:0 hashbrown::raw::RawTable::remove_entry
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs:1998:0 hashbrown::map::HashMap::remove_entry
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs:1959:0 hashbrown::map::HashMap::remove
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs:1145:0 bevy_platform::collections::hash_map::HashMap::remove
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1736:0 bevy_ecs::component::ComponentsRegistrator::register_component_checked
// F57: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:689:0 core::ptr::const_ptr::impl$0::offset_from
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:19:0 hashbrown::raw::offset_from
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:360:0 hashbrown::raw::Bucket::to_base_index
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:706:0 hashbrown::raw::RawTable::bucket_index
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:767:0 hashbrown::raw::RawTable::erase_no_drop
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:786:0 hashbrown::raw::RawTable::remove
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:800:0 hashbrown::raw::RawTable::remove_entry
// F64: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:2022:0 core::num::impl$11::wrapping_sub
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3067:0 hashbrown::raw::RawTableInner::erase
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:768:0 hashbrown::raw::RawTable::erase_no_drop
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3071:0 hashbrown::raw::RawTableInner::erase
// F68: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:843:0 core::core_arch::x86::sse2::_mm_cmpeq_epi8
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs:86:0 hashbrown::control::group::sse2::Group::match_tag
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs:95:0 hashbrown::control::group::sse2::Group::load
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3072:0 hashbrown::raw::RawTableInner::erase
// F72: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:136:0 core::num::impl$7::leading_zeros
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs:87:0 hashbrown::control::bitmask::BitMask::leading_zeros
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3110:0 hashbrown::raw::RawTableInner::erase
// F75: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:162:0 core::num::impl$7::trailing_zeros
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs:68:0 hashbrown::control::bitmask::BitMask::leading_zeros
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3113:0 hashbrown::raw::RawTableInner::erase
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:2447:0 hashbrown::raw::RawTableInner::set_ctrl
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3117:0 hashbrown::raw::RawTableInner::erase
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:2448:0 hashbrown::raw::RawTableInner::set_ctrl
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3118:0 hashbrown::raw::RawTableInner::erase
// F82: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1380:0 core::ptr::read
// F83: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1359:0 core::ptr::mut_ptr::impl$0::read
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:489:0 hashbrown::raw::Bucket::read
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:788:0 hashbrown::raw::RawTable::remove
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1743:0 bevy_ecs::component::ComponentsRegistrator::register_component_checked
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1336:0 bevy_ecs::component::ComponentIds::next_mut
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1337:0 bevy_ecs::component::ComponentIds::next_mut
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1102:0 bevy_ecs::component::ComponentDescriptor::new
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1762:0 bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1746:0 bevy_ecs::component::ComponentsRegistrator::register_component_checked
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1977:0 bevy_ecs::component::Components::register_component_inner
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1979:0 bevy_ecs::component::Components::register_component_inner
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1980:0 bevy_ecs::component::Components::register_component_inner
// F95: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:233:0 core::slice::index::impl$2::get_mut
// F96: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:623:0 core::slice::impl$0::get_mut
// F97: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1983:0 bevy_ecs::component::Components::register_component_inner
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1986:0 bevy_ecs::component::Components::register_component_inner
// F99: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:523:0 core::ptr::drop_in_place
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1615:0 bevy_ecs::component::impl$17::deref_mut
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1765:0 bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
// F102: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs:1100:0 bevy_ecs::component::impl$17::deref_mut
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:2812:0 bevy_ecs::component::impl$90::default
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1768:0 bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1769:0 bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1778:0 bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
// F107: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F108: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F109: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F110: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1734:0 alloc::vec::Vec::as_mut_ptr
// F111: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1607:0 alloc::vec::Vec::as_mut_slice
// F112: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3274:0 alloc::vec::impl$9::deref_mut
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1794:0 bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3333:0 hashbrown::raw::impl$17::drop
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs:3332:0 hashbrown::raw::impl$17::drop
// F116: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:751:0 alloc::raw_vec::RawVecInner::deallocate
// F117: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:402:0 alloc::raw_vec::impl$3::drop
// F118: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:523:0 alloc::raw_vec::RawVecInner::current_memory
// F119: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:945:0 core::num::impl$11::unchecked_mul
// F120: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:531:0 alloc::raw_vec::RawVecInner::current_memory
// F121: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:116:0 alloc::alloc::dealloc
// F122: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:269:0 alloc::alloc::impl$1::deallocate
// F123: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:753:0 alloc::raw_vec::RawVecInner::deallocate
// F124: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2423:0 alloc::vec::Vec::push
// F125: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2426:0 alloc::vec::Vec::push
// F126: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F127: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2427:0 alloc::vec::Vec::push
// F128: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2428:0 alloc::vec::Vec::push
// F129: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2504:0 alloc::vec::Vec::pop
// F130: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs:1960:0 hashbrown::map::HashMap::remove
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1740:0 bevy_ecs::component::ComponentsRegistrator::register_component_checked
// F132: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2508:0 alloc::vec::Vec::pop
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1987:0 bevy_ecs::component::Components::register_component_inner
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1795:0 bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs:1715:0 bevy_ecs::component::ComponentsRegistrator::register_component
// F136: crates/systems/roof/src/utils/components.rs:7:0 _$LT$system_roof..utils..components..RoofSupportBeams$u20$as$u20$bevy_ecs..component..Component$GT$::register_required_components::hbeb95b505e6d89e4
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/utils/components.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 0caaef554ea43c5bd13d0d4c9bfbe7e28493ccf2bd77776b65df1f652b452c42
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: components.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 4336
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::utils::components::impl$0::register_required_components
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4336,"record_offset":2188,"rva":39375344,"va":5408084464}
// 编译实例 RVAs: 0x258d1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140051340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39043796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e82c3dacfd","size":5,"soft_fail":false,"target_va":5369041728,"va":5408085519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33147408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e8f03506fe","size":5,"soft_fail":false,"target_va":5374937936,"va":5408085339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33004087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e8c96508fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5408085586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcddd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24901457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e8af0884fe","size":5,"soft_fail":false,"target_va":5383183824,"va":5408085276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14258d1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e81efcffff","size":5,"soft_fail":false,"target_va":5408084464,"va":5408085453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3761843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e8b3663900","size":5,"soft_fail":false,"target_va":5411847168,"va":5408085320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27310207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e881475ffe","size":5,"soft_fail":false,"target_va":5380775408,"va":5408085610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::ComponentInfo::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25484434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e86e237bfe","size":5,"soft_fail":false,"target_va":5382600816,"va":5408085245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::QueuedRegistration::register
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25484470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e84a237bfe","size":5,"soft_fail":false,"target_va":5382601280,"va":5408085745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::enforce_no_required_components_recursion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25478345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e8373b7bfe","size":5,"soft_fail":false,"target_va":5382606192,"va":5408084532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-38840697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408084464,"raw_hex":"e88756affd","size":5,"soft_fail":false,"target_va":5369244704,"va":5408085396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":6684,"parent_record_offset":2188,"ancestor_record_offset":2188,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":6488,"parent_record_offset":2188,"ancestor_record_offset":2188,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::component::ComponentsRegistrator::register_component
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":2348,"parent_record_offset":2188,"ancestor_record_offset":2188,"inlinee":5611,"source_line":1715,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4336,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":2188,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4336,"record_offset":6176,"parent_record_offset":6084,"ancestor_record_offset":2188,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":2188,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":6112,"parent_record_offset":6084,"ancestor_record_offset":2188,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":6084,"parent_record_offset":6060,"ancestor_record_offset":2188,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":2188,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":2188,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":2188,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":2188,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":5572,"parent_record_offset":5548,"ancestor_record_offset":2188,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":2392,"parent_record_offset":2348,"ancestor_record_offset":2188,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":5548,"parent_record_offset":5260,"ancestor_record_offset":2188,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentDescriptor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":5320,"parent_record_offset":5260,"ancestor_record_offset":2188,"inlinee":5647,"source_line":1101,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentIds::next_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":5228,"parent_record_offset":2420,"ancestor_record_offset":2188,"inlinee":5644,"source_line":1334,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::register_component_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":5348,"parent_record_offset":5260,"ancestor_record_offset":2188,"inlinee":5648,"source_line":1972,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_checked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":2420,"parent_record_offset":2348,"ancestor_record_offset":2188,"inlinee":5612,"source_line":1721,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":5260,"parent_record_offset":2420,"ancestor_record_offset":2188,"inlinee":5645,"source_line":1755,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$17::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":5484,"parent_record_offset":5260,"ancestor_record_offset":2188,"inlinee":13108,"source_line":1614,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$90::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":5520,"parent_record_offset":5260,"ancestor_record_offset":2188,"inlinee":5662,"source_line":2812,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":4336,"record_offset":2488,"parent_record_offset":2420,"ancestor_record_offset":2188,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":4336,"record_offset":3528,"parent_record_offset":2420,"ancestor_record_offset":2188,"inlinee":5624,"source_line":1141,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":4336,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":2188,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4336,"record_offset":3316,"parent_record_offset":3292,"ancestor_record_offset":2188,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_cmpeq_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4336,"record_offset":4788,"parent_record_offset":4760,"ancestor_record_offset":2188,"inlinee":5636,"source_line":842,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4336,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2188,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4336,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2188,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4336,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2188,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4336,"record_offset":6144,"parent_record_offset":6112,"ancestor_record_offset":2188,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4336,"record_offset":4648,"parent_record_offset":4600,"ancestor_record_offset":2188,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4336,"record_offset":5024,"parent_record_offset":4996,"ancestor_record_offset":2188,"inlinee":5639,"source_line":135,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4336,"record_offset":5052,"parent_record_offset":4996,"ancestor_record_offset":2188,"inlinee":5640,"source_line":161,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":4336,"record_offset":2952,"parent_record_offset":2924,"ancestor_record_offset":2188,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":4336,"record_offset":2924,"parent_record_offset":2896,"ancestor_record_offset":2188,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4336,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":2188,"inlinee":5634,"source_line":682,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4336,"record_offset":5452,"parent_record_offset":5348,"ancestor_record_offset":2188,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4336,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":2188,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4336,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2188,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4336,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":2188,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4336,"record_offset":6648,"parent_record_offset":6488,"ancestor_record_offset":2188,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4336,"record_offset":5396,"parent_record_offset":5348,"ancestor_record_offset":2188,"inlinee":5649,"source_line":619,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4336,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":2188,"inlinee":5650,"source_line":232,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":4336,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":2188,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":4336,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":2188,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":4336,"record_offset":3292,"parent_record_offset":3268,"ancestor_record_offset":2188,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":4336,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":2188,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::leading_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":4336,"record_offset":4996,"parent_record_offset":4600,"ancestor_record_offset":2188,"inlinee":5637,"source_line":86,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":4336,"record_offset":2860,"parent_record_offset":2832,"ancestor_record_offset":2188,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":4336,"record_offset":2896,"parent_record_offset":2860,"ancestor_record_offset":2188,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":4336,"record_offset":2832,"parent_record_offset":2612,"ancestor_record_offset":2188,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":4336,"record_offset":2692,"parent_record_offset":2612,"ancestor_record_offset":2188,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":4336,"record_offset":2996,"parent_record_offset":2612,"ancestor_record_offset":2188,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":4336,"record_offset":2776,"parent_record_offset":2612,"ancestor_record_offset":2188,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":2188,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":2188,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":3560,"parent_record_offset":3528,"ancestor_record_offset":2188,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":3592,"parent_record_offset":3560,"ancestor_record_offset":2188,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":3220,"parent_record_offset":3112,"ancestor_record_offset":2188,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2188,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":5124,"parent_record_offset":4432,"ancestor_record_offset":2188,"inlinee":5642,"source_line":488,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":4512,"parent_record_offset":4488,"ancestor_record_offset":2188,"inlinee":5631,"source_line":334,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":3080,"parent_record_offset":2612,"ancestor_record_offset":2188,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2188,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":4488,"parent_record_offset":4460,"ancestor_record_offset":2188,"inlinee":5630,"source_line":705,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":4460,"parent_record_offset":4432,"ancestor_record_offset":2188,"inlinee":5629,"source_line":766,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":2188,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":2564,"parent_record_offset":2536,"ancestor_record_offset":2188,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":4432,"parent_record_offset":3616,"ancestor_record_offset":2188,"inlinee":5628,"source_line":785,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":2188,"inlinee":5627,"source_line":797,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":4600,"parent_record_offset":4460,"ancestor_record_offset":2188,"inlinee":5635,"source_line":3062,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":2188,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":2664,"parent_record_offset":2612,"ancestor_record_offset":2188,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":5084,"parent_record_offset":4600,"ancestor_record_offset":2188,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":2188,"inlinee":4995,"source_line":3323,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":3112,"parent_record_offset":2612,"ancestor_record_offset":2188,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":2188,"inlinee":5633,"source_line":18,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":4336,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":2188,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":4336,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":2188,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":4336,"record_offset":3388,"parent_record_offset":2420,"ancestor_record_offset":2188,"inlinee":5618,"source_line":631,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::utils::components::impl$1::register_required_components
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4336,"record_offset":6724,"rva":39377120,"va":5408086240}
// 编译实例 RVAs: 0x258d8e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140051340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39045572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e83c36acfd","size":5,"soft_fail":false,"target_va":5369041728,"va":5408087295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33149184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e8002f06fe","size":5,"soft_fail":false,"target_va":5374937936,"va":5408087115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33005863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e8d95e08fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5408087362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcddd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24903233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e8bf0184fe","size":5,"soft_fail":false,"target_va":5383183824,"va":5408087052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14258d8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e81efcffff","size":5,"soft_fail":false,"target_va":5408086240,"va":5408087229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: components.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3760067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e8c35f3900","size":5,"soft_fail":false,"target_va":5411847168,"va":5408087096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27311983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e891405ffe","size":5,"soft_fail":false,"target_va":5380775408,"va":5408087386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::ComponentInfo::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25486210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e87e1c7bfe","size":5,"soft_fail":false,"target_va":5382600816,"va":5408087021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::QueuedRegistration::register
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25486246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e85a1c7bfe","size":5,"soft_fail":false,"target_va":5382601280,"va":5408087521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::enforce_no_required_components_recursion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25480121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e847347bfe","size":5,"soft_fail":false,"target_va":5382606192,"va":5408086308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-38842473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408086240,"raw_hex":"e8974faffd","size":5,"soft_fail":false,"target_va":5369244704,"va":5408087172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":11220,"parent_record_offset":6724,"ancestor_record_offset":6724,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":11024,"parent_record_offset":6724,"ancestor_record_offset":6724,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::component::ComponentsRegistrator::register_component
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":6884,"parent_record_offset":6724,"ancestor_record_offset":6724,"inlinee":5611,"source_line":1715,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4336,"record_offset":10736,"parent_record_offset":10712,"ancestor_record_offset":6724,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4336,"record_offset":10712,"parent_record_offset":10620,"ancestor_record_offset":6724,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":6724,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":10648,"parent_record_offset":10620,"ancestor_record_offset":6724,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":6724,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":6724,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":6724,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4336,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":6724,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":10132,"parent_record_offset":10108,"ancestor_record_offset":6724,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":10108,"parent_record_offset":10084,"ancestor_record_offset":6724,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":6928,"parent_record_offset":6884,"ancestor_record_offset":6724,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4336,"record_offset":10084,"parent_record_offset":9796,"ancestor_record_offset":6724,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentDescriptor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":9856,"parent_record_offset":9796,"ancestor_record_offset":6724,"inlinee":5647,"source_line":1101,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentIds::next_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":9764,"parent_record_offset":6956,"ancestor_record_offset":6724,"inlinee":5644,"source_line":1334,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::register_component_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":9884,"parent_record_offset":9796,"ancestor_record_offset":6724,"inlinee":5648,"source_line":1972,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_checked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":6956,"parent_record_offset":6884,"ancestor_record_offset":6724,"inlinee":5612,"source_line":1721,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":9796,"parent_record_offset":6956,"ancestor_record_offset":6724,"inlinee":5645,"source_line":1755,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$17::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":10020,"parent_record_offset":9796,"ancestor_record_offset":6724,"inlinee":13108,"source_line":1614,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$90::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":4336,"record_offset":10056,"parent_record_offset":9796,"ancestor_record_offset":6724,"inlinee":5662,"source_line":2812,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":4336,"record_offset":7024,"parent_record_offset":6956,"ancestor_record_offset":6724,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":4336,"record_offset":8064,"parent_record_offset":6956,"ancestor_record_offset":6724,"inlinee":5624,"source_line":1141,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":4336,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":6724,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4336,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":6724,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_cmpeq_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4336,"record_offset":9324,"parent_record_offset":9296,"ancestor_record_offset":6724,"inlinee":5636,"source_line":842,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4336,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":6724,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4336,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":6724,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4336,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":6724,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4336,"record_offset":10680,"parent_record_offset":10648,"ancestor_record_offset":6724,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4336,"record_offset":9184,"parent_record_offset":9136,"ancestor_record_offset":6724,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4336,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":6724,"inlinee":5639,"source_line":135,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4336,"record_offset":9588,"parent_record_offset":9532,"ancestor_record_offset":6724,"inlinee":5640,"source_line":161,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":4336,"record_offset":7488,"parent_record_offset":7460,"ancestor_record_offset":6724,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":4336,"record_offset":7460,"parent_record_offset":7432,"ancestor_record_offset":6724,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4336,"record_offset":9096,"parent_record_offset":9072,"ancestor_record_offset":6724,"inlinee":5634,"source_line":682,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4336,"record_offset":9988,"parent_record_offset":9884,"ancestor_record_offset":6724,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4336,"record_offset":9684,"parent_record_offset":9660,"ancestor_record_offset":6724,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4336,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":6724,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4336,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":6724,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4336,"record_offset":11184,"parent_record_offset":11024,"ancestor_record_offset":6724,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4336,"record_offset":9932,"parent_record_offset":9884,"ancestor_record_offset":6724,"inlinee":5649,"source_line":619,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4336,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":6724,"inlinee":5650,"source_line":232,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":4336,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":6724,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":4336,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":6724,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":4336,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":6724,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":4336,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":6724,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::leading_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":4336,"record_offset":9532,"parent_record_offset":9136,"ancestor_record_offset":6724,"inlinee":5637,"source_line":86,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":4336,"record_offset":7396,"parent_record_offset":7368,"ancestor_record_offset":6724,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":4336,"record_offset":7432,"parent_record_offset":7396,"ancestor_record_offset":6724,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":4336,"record_offset":7368,"parent_record_offset":7148,"ancestor_record_offset":6724,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":4336,"record_offset":7228,"parent_record_offset":7148,"ancestor_record_offset":6724,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":4336,"record_offset":7532,"parent_record_offset":7148,"ancestor_record_offset":6724,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":4336,"record_offset":7312,"parent_record_offset":7148,"ancestor_record_offset":6724,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":6724,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":6724,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":8096,"parent_record_offset":8064,"ancestor_record_offset":6724,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":8128,"parent_record_offset":8096,"ancestor_record_offset":6724,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":4336,"record_offset":7756,"parent_record_offset":7648,"ancestor_record_offset":6724,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":6724,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":9660,"parent_record_offset":8968,"ancestor_record_offset":6724,"inlinee":5642,"source_line":488,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":6724,"inlinee":5631,"source_line":334,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":7616,"parent_record_offset":7148,"ancestor_record_offset":6724,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":7672,"parent_record_offset":7648,"ancestor_record_offset":6724,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":9024,"parent_record_offset":8996,"ancestor_record_offset":6724,"inlinee":5630,"source_line":705,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":8996,"parent_record_offset":8968,"ancestor_record_offset":6724,"inlinee":5629,"source_line":766,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6724,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":7100,"parent_record_offset":7072,"ancestor_record_offset":6724,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":8968,"parent_record_offset":8152,"ancestor_record_offset":6724,"inlinee":5628,"source_line":785,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":6724,"inlinee":5627,"source_line":797,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":9136,"parent_record_offset":8996,"ancestor_record_offset":6724,"inlinee":5635,"source_line":3062,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":6724,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":7200,"parent_record_offset":7148,"ancestor_record_offset":6724,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":9620,"parent_record_offset":9136,"ancestor_record_offset":6724,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":10348,"parent_record_offset":10324,"ancestor_record_offset":6724,"inlinee":4995,"source_line":3323,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":7648,"parent_record_offset":7148,"ancestor_record_offset":6724,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":4336,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":6724,"inlinee":5633,"source_line":18,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":4336,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":6724,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":4336,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":6724,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/04336.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":4336,"record_offset":7924,"parent_record_offset":6956,"ancestor_record_offset":6724,"inlinee":5618,"source_line":631,"archive":"_pdb/B/modules/04336.jsonl.gz"}
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
// 完整结构及原始类型记录: components.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_roof::utils::components::RoofSpire
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 56
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>", "constructor": "bevy_ecs::query::filter::With", "arguments": ["system_roof::utils::components::RoofSpire"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::iter::QueryIter", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::impl$2::next<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>", "constructor": "core::iter::adapters::filter_map::impl$2::next", "arguments": ["country_core::resources::roof_tile_ssbo::RoofSpireSsbo", "bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "system_roof::visual::place_spires::place_roof_spires::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "system_roof::visual::place_spires::place_roof_spires::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::roof_tile_ssbo::RoofSpireSsbo", "core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "constructor": "tuple$", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::system::query::Query", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)", "void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["tuple$<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::config::impl$4::into_configs", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::impl$2::update_archetype_component_access::closure_env$0<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::impl$2::update_archetype_component_access::closure_env$0", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<bevy_ecs::query::access::ComponentAccessKind<bevy_ecs::component::ComponentId> >,bevy_ecs::query::state::impl$2::update_archetype_component_access::closure_env$0<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["enum2$<bevy_ecs::query::access::ComponentAccessKind<bevy_ecs::component::ComponentId> >", "bevy_ecs::query::state::impl$2::update_archetype_component_access::closure_env$0<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<bevy_ecs::query::access::ComponentAccessKind<bevy_ecs::component::ComponentId> >,bevy_ecs::query::state::impl$2::update_archetype_component_access::closure_env$0<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > > >", "constructor": "ref_mut$", "arguments": ["core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<bevy_ecs::query::access::ComponentAccessKind<bevy_ecs::component::ComponentId> >,bevy_ecs::query::state::impl$2::update_archetype_component_access::closure_env$0<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<enum2$<bevy_ecs::query::access::ComponentAccessKind<bevy_ecs::component::ComponentId> >,fixedbitset::Ones,bevy_ecs::query::access::impl$5::try_iter_component_access::closure_env$0<bevy_ecs::component::ComponentId>,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<bevy_ecs::query::access::ComponentAccessKind<bevy_ecs::component::ComponentId> >,bevy_ecs::query::state::impl$2::update_archetype_component_access::closure_env$0<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["enum2$<bevy_ecs::query::access::ComponentAccessKind<bevy_ecs::component::ComponentId> >", "fixedbitset::Ones", "bevy_ecs::query::access::impl$5::try_iter_component_access::closure_env$0<bevy_ecs::component::ComponentId>", "tuple$<>", "ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<bevy_ecs::query::access::ComponentAccessKind<bevy_ecs::component::ComponentId> >,bevy_ecs::query::state::impl$2::update_archetype_component_access::closure_env$0<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::BundleInfo::write_components<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::BundleInfo::write_components", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new_with_access<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState::new_with_access", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new_archetype_internal<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState::new_archetype_internal", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::update_archetype_component_access<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState::update_archetype_component_access", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::init_query_param<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::system::system_param::init_query_param", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$33::component_ids<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$33::component_ids", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::filter::impl$0::init_state<system_roof::utils::components::RoofSpire>", "constructor": "bevy_ecs::query::filter::impl$0::init_state", "arguments": ["system_roof::utils::components::RoofSpire"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSpire,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSpire", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSupportBeams,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSupportBeams", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_roof::utils::components::RoofSupportBeams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "constructor": "tuple$", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::BundleInfo::write_components<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::BundleInfo::write_components", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$33::component_ids<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$33::component_ids", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSpire,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSpire", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSupportBeams,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSupportBeams", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: components.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-runtime/B-cache/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_roof::utils::components::RoofSupportBeams", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "system_roof::utils::components::RoofSpire", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: components.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_roof::utils::components::impl$0::register_required_components
// 信息 ID: b090ec85f30f6f051c591bb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::query::filter::impl$0::init_state<system_roof::utils::components::RoofSpire> @ 0x219e50b -> 0x258d1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSpire,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > @ 0x258c6b8 -> 0x258d1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::components::impl$0::register_required_components @ 0x258d5cd -> 0x258d1f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::utils::components::impl$1::register_required_components
// 信息 ID: b9e2e3a617c64f757a64f272
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSupportBeams,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > @ 0x258cd68 -> 0x258d8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::components::impl$1::register_required_components @ 0x258dcbd -> 0x258d8e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
