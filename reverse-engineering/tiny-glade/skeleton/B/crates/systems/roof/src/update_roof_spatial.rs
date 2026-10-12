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
// Inline site 3465:1508 -> host 3465:260 / rva_21a7710_len_5b3; exact_symbolizer_function_frame
//   Function-frame RVA span [0x21a784a, 0x21a7965), 65 instructions
//
// FUNCTION system_roof::update_roof_spatial::get_shape_params
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3465:4116 -> host 3465:260 / rva_21a7710_len_5b3; exact_symbolizer_function_frame
//   Function-frame RVA span [0x21a7a6d, 0x21a7a9d), 8 instructions
//
// FUNCTION system_roof::update_roof_spatial::update_roof_spatial
// Candidate interfaces: 3a48aeb128215955ee03632b
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3465:260 -> rva_21a7710_len_5b3
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21a7710_len_5b3: RVA 0x21a7710, 1459 PDB bytes; 342 instructions
// Evidence: _assembly/roof-v1/B/pack-001/rva_21a7710_len_5b3.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3465:260: system_roof::update_roof_spatial::update_roof_spatial
// Frames: F1
// 0x1421a7710  4157                           pushq	%r15
// 0x1421a7712  4156                           pushq	%r14
// 0x1421a7714  4155                           pushq	%r13
// 0x1421a7716  4154                           pushq	%r12
// 0x1421a7718  56                             pushq	%rsi
// 0x1421a7719  57                             pushq	%rdi
// 0x1421a771a  55                             pushq	%rbp
// 0x1421a771b  53                             pushq	%rbx
// 0x1421a771c  4881ecf8000000                 subq	$0xf8, %rsp
// 0x1421a7723  0f29b424e0000000               movaps	%xmm6, 0xe0(%rsp)
// Frames: F2 -> F3 -> F4
// 0x1421a772b  4d8b30                         movq	(%r8), %r14
// Frames: F5 -> F3 -> F4
// 0x1421a772e  498b4008                       movq	0x8(%r8), %rax
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x1421a7732  4d8b1e                         movq	(%r14), %r11
// 0x1421a7735  4531d2                         xorl	%r10d, %r10d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x1421a7738  4c89de                         movq	%r11, %rsi
// 0x1421a773b  482b7018                       subq	0x18(%rax), %rsi
// 0x1421a773f  490f42f2                       cmovbq	%r10, %rsi
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x1421a7743  4c2b5838                       subq	0x38(%rax), %r11
// 0x1421a7747  4d0f42da                       cmovbq	%r10, %r11
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421a774b  4989f0                         movq	%rsi, %r8
// 0x1421a774e  49c1e004                       shlq	$0x4, %r8
// 0x1421a7752  4c034008                       addq	0x8(%rax), %r8
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421a7756  4c8b7810                       movq	0x10(%rax), %r15
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421a775a  4929f7                         subq	%rsi, %r15
// 0x1421a775d  4d0f42fa                       cmovbq	%r10, %r15
// 0x1421a7761  be08000000                     movl	$0x8, %esi
// 0x1421a7766  4c0f42c6                       cmovbq	%rsi, %r8
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421a776a  4d89dd                         movq	%r11, %r13
// 0x1421a776d  49c1e504                       shlq	$0x4, %r13
// 0x1421a7771  4c036828                       addq	0x28(%rax), %r13
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421a7775  488b6830                       movq	0x30(%rax), %rbp
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421a7779  4c29dd                         subq	%r11, %rbp
// 0x1421a777c  490f42ea                       cmovbq	%r10, %rbp
// 0x1421a7780  4c0f42ee                       cmovbq	%rsi, %r13
// Frames: F20 -> F7 -> F8 -> F3 -> F4
// 0x1421a7784  488b4040                       movq	0x40(%rax), %rax
// 0x1421a7788  4829e8                         subq	%rbp, %rax
// 0x1421a778b  4c29f8                         subq	%r15, %rax
// 0x1421a778e  498906                         movq	%rax, (%r14)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F4
// 0x1421a7791  49c1e704                       shlq	$0x4, %r15
// 0x1421a7795  4d01c7                         addq	%r8, %r15
// 0x1421a7798  48c1e504                       shlq	$0x4, %rbp
// 0x1421a779c  4c01ed                         addq	%r13, %rbp
// 0x1421a779f  488b32                         movq	(%rdx), %rsi
// 0x1421a77a2  488b01                         movq	(%rcx), %rax
// 0x1421a77a5  4889442450                     movq	%rax, 0x50(%rsp)
// 0x1421a77aa  488b4108                       movq	0x8(%rcx), %rax
// 0x1421a77ae  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1421a77b3  8b4114                         movl	0x14(%rcx), %eax
// 0x1421a77b6  89442428                       movl	%eax, 0x28(%rsp)
// 0x1421a77ba  418b411c                       movl	0x1c(%r9), %eax
// 0x1421a77be  8944242c                       movl	%eax, 0x2c(%rsp)
// 0x1421a77c2  498b01                         movq	(%r9), %rax
// 0x1421a77c5  498b4910                       movq	0x10(%r9), %rcx
// 0x1421a77c9  48894c2468                     movq	%rcx, 0x68(%rsp)
// 0x1421a77ce  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1421a77d3  4883c020                       addq	$0x20, %rax
// 0x1421a77d7  4889442460                     movq	%rax, 0x60(%rsp)
// 0x1421a77dc  f30f103594508d00               movss	0x8d5094(%rip), %xmm6   # 0x142a7c878
// 0x1421a77e4  48896c2470                     movq	%rbp, 0x70(%rsp)
// 0x1421a77e9  4c897c2478                     movq	%r15, 0x78(%rsp)
// 0x1421a77ee  6690                           nop
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F4
// 0x1421a77f0  4d85c0                         testq	%r8, %r8
// 0x1421a77f3  741b                           je	0x1421a7810 <.text+0x21a6810>
// 0x1421a77f5  4d39f8                         cmpq	%r15, %r8
// 0x1421a77f8  7416                           je	0x1421a7810 <.text+0x21a6810>
// 0x1421a77fa  4d8d6010                       leaq	0x10(%r8), %r12
// 0x1421a77fe  4c89eb                         movq	%r13, %rbx
// 0x1421a7801  4c89c7                         movq	%r8, %rdi
// 0x1421a7804  eb29                           jmp	0x1421a782f <.text+0x21a682f>
// 0x1421a7806  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F25 -> F30 -> F31 -> F27 -> F28 -> F29 -> F4
// 0x1421a7810  4d85ed                         testq	%r13, %r13
// 0x1421a7813  0f841f040000                   je	0x1421a7c38 <.text+0x21a6c38>
// 0x1421a7819  4939ed                         cmpq	%rbp, %r13
// 0x1421a781c  0f8416040000                   je	0x1421a7c38 <.text+0x21a6c38>
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F27 -> F28 -> F29 -> F4
// 0x1421a7822  498d5d10                       leaq	0x10(%r13), %rbx
// 0x1421a7826  4531e4                         xorl	%r12d, %r12d
// 0x1421a7829  4c89ef                         movq	%r13, %rdi
// 0x1421a782c  4989dd                         movq	%rbx, %r13
// Frames: F36 -> F29 -> F4
// 0x1421a782f  49ff06                         incq	(%r14)
// Frames: F37
// 0x1421a7832  488b5708                       movq	0x8(%rdi), %rdx
// Frames: F38
// 0x1421a7836  4889f1                         movq	%rsi, %rcx
// 0x1421a7839  e872aa8efe                     callq	0x140a922b0 <.text+0xa912b0>
// Frames: F39 -> F38
// 0x1421a783e  4889c1                         movq	%rax, %rcx
// 0x1421a7841  48c1e920                       shrq	$0x20, %rcx
// 0x1421a7845  4d89e0                         movq	%r12, %r8
// 0x1421a7848  74a6                           je	0x1421a77f0 <.text+0x21a67f0>
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F38
// 0x1421a784a  89c2                           movl	%eax, %edx
// 0x1421a784c  4989dd                         movq	%rbx, %r13
// 0x1421a784f  4d89e0                         movq	%r12, %r8
// 0x1421a7852  4c8b4c2458                     movq	0x58(%rsp), %r9
// Frames: F45 -> F15 -> F40 -> F41 -> F42 -> F43 -> F44 -> F38
// 0x1421a7857  493b5110                       cmpq	0x10(%r9), %rdx
// 0x1421a785b  7393                           jae	0x1421a77f0 <.text+0x21a67f0>
// Frames: F46 -> F47 -> F48 -> F49 -> F17 -> F50 -> F40 -> F41 -> F42 -> F43 -> F44 -> F38
// 0x1421a785d  4c8b442458                     movq	0x58(%rsp), %r8
// 0x1421a7862  4d8b4808                       movq	0x8(%r8), %r9
// Frames: F51 -> F45 -> F15 -> F40 -> F41 -> F42 -> F43 -> F44 -> F38
// 0x1421a7866  488d1492                       leaq	(%rdx,%rdx,4), %rdx
// 0x1421a786a  4989dd                         movq	%rbx, %r13
// 0x1421a786d  4d89e0                         movq	%r12, %r8
// Frames: F52 -> F53 -> F54 -> F41 -> F42 -> F43 -> F44 -> F38
// 0x1421a7870  41390c91                       cmpl	%ecx, (%r9,%rdx,4)
// Frames: F54 -> F41 -> F42 -> F43 -> F44 -> F38
// 0x1421a7874  0f8576ffffff                   jne	0x1421a77f0 <.text+0x21a67f0>
// Frames: F55 -> F56 -> F41 -> F42 -> F43 -> F44 -> F38
// 0x1421a787a  498d0c91                       leaq	(%r9,%rdx,4), %rcx
// 0x1421a787e  8b5104                         movl	0x4(%rcx), %edx
// 0x1421a7881  4989dd                         movq	%rbx, %r13
// 0x1421a7884  4d89e0                         movq	%r12, %r8
// 0x1421a7887  41b9ffffffff                   movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
// 0x1421a788d  4c39ca                         cmpq	%r9, %rdx
// Frames: F56 -> F41 -> F42 -> F43 -> F44 -> F38
// 0x1421a7890  0f845affffff                   je	0x1421a77f0 <.text+0x21a67f0>
// Frames: F57 -> F58 -> F42 -> F43 -> F44 -> F38
// 0x1421a7896  4989dd                         movq	%rbx, %r13
// 0x1421a7899  4d89e0                         movq	%r12, %r8
// 0x1421a789c  4c8b4c2450                     movq	0x50(%rsp), %r9
// 0x1421a78a1  49395138                       cmpq	%rdx, 0x38(%r9)
// Frames: F59 -> F60 -> F58 -> F42 -> F43 -> F44 -> F38
// 0x1421a78a5  0f8645ffffff                   jbe	0x1421a77f0 <.text+0x21a67f0>
// Frames: F61 -> F62 -> F63 -> F59 -> F60 -> F58 -> F42 -> F43 -> F44 -> F38
// 0x1421a78ab  4c8b442450                     movq	0x50(%rsp), %r8
// 0x1421a78b0  4d8b4028                       movq	0x28(%r8), %r8
// Frames: F21 -> F61 -> F62 -> F63 -> F59 -> F60 -> F58 -> F42 -> F43 -> F44 -> F38
// 0x1421a78b4  4189d1                         movl	%edx, %r9d
// 0x1421a78b7  41c1e906                       shrl	$0x6, %r9d
// Frames: F64 -> F62 -> F63 -> F59 -> F60 -> F58 -> F42 -> F43 -> F44 -> F38
// 0x1421a78bb  4f8b04c8                       movq	(%r8,%r9,8), %r8
// Frames: F62 -> F63 -> F59 -> F60 -> F58 -> F42 -> F43 -> F44 -> F38
// 0x1421a78bf  490fa3d0                       btq	%rdx, %r8
// 0x1421a78c3  4989dd                         movq	%rbx, %r13
// 0x1421a78c6  4d89e0                         movq	%r12, %r8
// Frames: F65 -> F42 -> F43 -> F44 -> F38
// 0x1421a78c9  0f8321ffffff                   jae	0x1421a77f0 <.text+0x21a67f0>
// Frames: F46 -> F47 -> F48 -> F49 -> F17 -> F18 -> F66 -> F67 -> F42 -> F43 -> F44 -> F38
// 0x1421a78cf  4889442438                     movq	%rax, 0x38(%rsp)
// 0x1421a78d4  4883c708                       addq	$0x8, %rdi
// 0x1421a78d8  8b410c                         movl	0xc(%rcx), %eax
// 0x1421a78db  8b6910                         movl	0x10(%rcx), %ebp
// 0x1421a78de  488b4c2458                     movq	0x58(%rsp), %rcx
// 0x1421a78e3  488b89a8010000                 movq	0x1a8(%rcx), %rcx
// Frames: F51 -> F45 -> F68 -> F66 -> F67 -> F42 -> F43 -> F44 -> F38
// 0x1421a78ea  488d04c0                       leaq	(%rax,%rax,8), %rax
// 0x1421a78ee  4c8b442450                     movq	0x50(%rsp), %r8
// Frames: F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a78f3  498b9010010000                 movq	0x110(%r8), %rdx
// 0x1421a78fa  4d8b8018010000                 movq	0x118(%r8), %r8
// Frames: F70 -> F71 -> F72 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a7901  4c8b4cc118                     movq	0x18(%rcx,%rax,8), %r9
// 0x1421a7906  488b44c138                     movq	0x38(%rcx,%rax,8), %rax
// Frames: F75 -> F76 -> F44 -> F77 -> F70 -> F71 -> F72 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a790b  488b0cd0                       movq	(%rax,%rdx,8), %rcx
// Frames: F78 -> F79 -> F80 -> F70 -> F71 -> F72 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a790f  48f7d1                         notq	%rcx
// Frames: F51 -> F81 -> F82 -> F79 -> F80 -> F70 -> F71 -> F72 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a7912  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1421a7916  48c1e104                       shlq	$0x4, %rcx
// Frames: F83 -> F84 -> F85 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a791a  498b540928                     movq	0x28(%r9,%rcx), %rdx
// 0x1421a791f  4889542448                     movq	%rdx, 0x48(%rsp)
// Frames: F75 -> F76 -> F44 -> F77 -> F70 -> F71 -> F72 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a7924  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F78 -> F79 -> F80 -> F70 -> F71 -> F72 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a7928  48f7d0                         notq	%rax
// Frames: F51 -> F81 -> F82 -> F79 -> F80 -> F70 -> F71 -> F72 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a792b  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1421a792f  48c1e004                       shlq	$0x4, %rax
// Frames: F83 -> F84 -> F85 -> F73 -> F74 -> F69 -> F42 -> F43 -> F44 -> F38
// 0x1421a7933  498b540128                     movq	0x28(%r9,%rax), %rdx
// 0x1421a7938  4889942480000000               movq	%rdx, 0x80(%rsp)
// Frames: F21 -> F86 -> F87 -> F88 -> F89 -> F90 -> F91 -> F42 -> F43 -> F44 -> F38
// 0x1421a7940  4c6bfd58                       imulq	$0x58, %rbp, %r15
// 0x1421a7944  4d037c0910                     addq	0x10(%r9,%rcx), %r15
// 0x1421a7949  488d0c6d00000000               leaq	(,%rbp,2), %rcx
// 0x1421a7951  4801e9                         addq	%rbp, %rcx
// 0x1421a7954  48c1e104                       shlq	$0x4, %rcx
// 0x1421a7958  49034c0110                     addq	0x10(%r9,%rax), %rcx
// 0x1421a795d  48898c2488000000               movq	%rcx, 0x88(%rsp)
// Frames: F92
// 0x1421a7965  488b17                         movq	(%rdi), %rdx
// 0x1421a7968  4889f1                         movq	%rsi, %rcx
// 0x1421a796b  4c8d05bef3e400                 leaq	0xe4f3be(%rip), %r8     # 0x142ff6d30
// 0x1421a7972  e879a58efe                     callq	0x140a91ef0 <.text+0xa90ef0>
// 0x1421a7977  4885c0                         testq	%rax, %rax
// 0x1421a797a  0f84d4020000                   je	0x1421a7c54 <.text+0x21a6c54>
// Frames: F93
// 0x1421a7980  4989c5                         movq	%rax, %r13
// 0x1421a7983  4889c1                         movq	%rax, %rcx
// 0x1421a7986  e8952c98fe                     callq	0x140b2a620 <.text+0xb29620>
// 0x1421a798b  8b442428                       movl	0x28(%rsp), %eax
// 0x1421a798f  48896c2440                     movq	%rbp, 0x40(%rsp)
// 0x1421a7994  488b4c2448                     movq	0x48(%rsp), %rcx
// Frames: F94 -> F95 -> F93
// 0x1421a7999  8904a9                         movl	%eax, (%rcx,%rbp,4)
// Frames: F93
// 0x1421a799c  f3410f11474c                   movss	%xmm0, 0x4c(%r15)
// Frames: F96
// 0x1421a79a2  4c89ed                         movq	%r13, %rbp
// 0x1421a79a5  4881c599020000                 addq	$0x299, %rbp            # imm = 0x299
// 0x1421a79ac  4889e9                         movq	%rbp, %rcx
// 0x1421a79af  e8bcdc71fe                     callq	0x1408c5670 <.text+0x8c4670>
// Frames: F97 -> F98 -> F96
// 0x1421a79b4  41384754                       cmpb	%al, 0x54(%r15)
// 0x1421a79b8  4d89f9                         movq	%r15, %r9
// Frames: F96
// 0x1421a79bb  0f84ac000000                   je	0x1421a7a6d <.text+0x21a6a6d>
// Frames: F99
// 0x1421a79c1  4889e9                         movq	%rbp, %rcx
// 0x1421a79c4  4d89cf                         movq	%r9, %r15
// 0x1421a79c7  e8a4dc71fe                     callq	0x1408c5670 <.text+0x8c4670>
// 0x1421a79cc  4d89f9                         movq	%r15, %r9
// 0x1421a79cf  8b4c2428                       movl	0x28(%rsp), %ecx
// 0x1421a79d3  488b542440                     movq	0x40(%rsp), %rdx
// 0x1421a79d8  4c8b442448                     movq	0x48(%rsp), %r8
// Frames: F94 -> F95 -> F99
// 0x1421a79dd  41890c90                       movl	%ecx, (%r8,%rdx,4)
// Frames: F99
// 0x1421a79e1  41884754                       movb	%al, 0x54(%r15)
// Frames: F100
// 0x1421a79e5  4c8b17                         movq	(%rdi), %r10
// 0x1421a79e8  8b44242c                       movl	0x2c(%rsp), %eax
// 0x1421a79ec  488b4c2468                     movq	0x68(%rsp), %rcx
// Frames: F101 -> F102 -> F103 -> F104
// 0x1421a79f1  8901                           movl	%eax, (%rcx)
// 0x1421a79f3  488b442430                     movq	0x30(%rsp), %rax
// Frames: F105 -> F106 -> F107 -> F103 -> F104
// 0x1421a79f8  488b6830                       movq	0x30(%rax), %rbp
// Frames: F108 -> F107 -> F103 -> F104
// 0x1421a79fc  4c8b5840                       movq	0x40(%rax), %r11
// Frames: F109 -> F106 -> F107 -> F103 -> F104
// 0x1421a7a00  483b6820                       cmpq	0x20(%rax), %rbp
// 0x1421a7a04  7534                           jne	0x1421a7a3a <.text+0x21a6a3a>
// Frames: F110 -> F106 -> F107 -> F103 -> F104
// 0x1421a7a06  488b4c2460                     movq	0x60(%rsp), %rcx
// 0x1421a7a0b  488d155eb39c00                 leaq	0x9cb35e(%rip), %rdx    # 0x142b72d70
// 0x1421a7a12  4c89942498000000               movq	%r10, 0x98(%rsp)
// 0x1421a7a1a  4c899c2490000000               movq	%r11, 0x90(%rsp)
// 0x1421a7a22  e809b23bfe                     callq	0x140562c30 <.text+0x561c30>
// 0x1421a7a27  4c8b9c2490000000               movq	0x90(%rsp), %r11
// 0x1421a7a2f  4c8b942498000000               movq	0x98(%rsp), %r10
// 0x1421a7a37  4d89f9                         movq	%r15, %r9
// Frames: F46 -> F47 -> F48 -> F111 -> F112 -> F106 -> F107 -> F103 -> F104
// 0x1421a7a3a  4c8b442430                     movq	0x30(%rsp), %r8
// 0x1421a7a3f  498b4828                       movq	0x28(%r8), %rcx
// Frames: F113 -> F112 -> F106 -> F107 -> F103 -> F104
// 0x1421a7a43  4889ea                         movq	%rbp, %rdx
// 0x1421a7a46  48c1e205                       shlq	$0x5, %rdx
// Frames: F114 -> F115 -> F106 -> F107 -> F103 -> F104
// 0x1421a7a4a  4c891c11                       movq	%r11, (%rcx,%rdx)
// 0x1421a7a4e  488b442438                     movq	0x38(%rsp), %rax
// 0x1421a7a53  4889441108                     movq	%rax, 0x8(%rcx,%rdx)
// 0x1421a7a58  4c89541110                     movq	%r10, 0x10(%rcx,%rdx)
// 0x1421a7a5d  c644111802                     movb	$0x2, 0x18(%rcx,%rdx)
// Frames: F116 -> F106 -> F107 -> F103 -> F104
// 0x1421a7a62  48ffc5                         incq	%rbp
// 0x1421a7a65  49896830                       movq	%rbp, 0x30(%r8)
// Frames: F117 -> F107 -> F103 -> F104
// 0x1421a7a69  49ff4040                       incq	0x40(%r8)
// Frames: F118 -> F119
// 0x1421a7a6d  418b8d58020000                 movl	0x258(%r13), %ecx
// 0x1421a7a74  83f901                         cmpl	$0x1, %ecx
// 0x1421a7a77  488b442438                     movq	0x38(%rsp), %rax
// 0x1421a7a7c  488b6c2440                     movq	0x40(%rsp), %rbp
// 0x1421a7a81  4c8b7c2478                     movq	0x78(%rsp), %r15
// 0x1421a7a86  0f8710010000                   ja	0x1421a7b9c <.text+0x21a6b9c>
// Frames: F120 -> F119
// 0x1421a7a8c  410f10855c020000               movups	0x25c(%r13), %xmm0
// 0x1421a7a94  f2410f108d6c020000             movsd	0x26c(%r13), %xmm1
// Frames: F121 -> F98 -> F122
// 0x1421a7a9d  413b4908                       cmpl	0x8(%r9), %ecx
// 0x1421a7aa1  7569                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7aa3  f30f16d0                       movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
// 0x1421a7aa7  f6c101                         testb	$0x1, %cl
// 0x1421a7aaa  742a                           je	0x1421a7ad6 <.text+0x21a6ad6>
// Frames: F123 -> F124 -> F125 -> F121 -> F98 -> F122
// 0x1421a7aac  410f2e410c                     ucomiss	0xc(%r9), %xmm0
// Frames: F124 -> F125 -> F121 -> F98 -> F122
// 0x1421a7ab1  7559                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7ab3  7a57                           jp	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7ab5  410f2e5110                     ucomiss	0x10(%r9), %xmm2
// 0x1421a7aba  7550                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7abc  7a4e                           jp	0x1421a7b0c <.text+0x21a6b0c>
// Frames: F126 -> F125 -> F121 -> F98 -> F122
// 0x1421a7abe  410f2e491c                     ucomiss	0x1c(%r9), %xmm1
// 0x1421a7ac3  7547                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7ac5  7a45                           jp	0x1421a7b0c <.text+0x21a6b0c>
// Frames: F127 -> F125 -> F121 -> F98 -> F122
// 0x1421a7ac7  f30f16d1                       movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
// 0x1421a7acb  410f2e5120                     ucomiss	0x20(%r9), %xmm2
// 0x1421a7ad0  753a                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7ad2  7b14                           jnp	0x1421a7ae8 <.text+0x21a6ae8>
// 0x1421a7ad4  eb36                           jmp	0x1421a7b0c <.text+0x21a6b0c>
// Frames: F123 -> F128 -> F125 -> F129 -> F98 -> F122
// 0x1421a7ad6  410f2e410c                     ucomiss	0xc(%r9), %xmm0
// Frames: F128 -> F125 -> F129 -> F98 -> F122
// 0x1421a7adb  752f                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7add  7a2d                           jp	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7adf  410f2e5110                     ucomiss	0x10(%r9), %xmm2
// 0x1421a7ae4  7526                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7ae6  7a24                           jp	0x1421a7b0c <.text+0x21a6b0c>
// Frames: F122
// 0x1421a7ae8  0f28d0                         movaps	%xmm0, %xmm2
// 0x1421a7aeb  660f15d0                       unpckhpd	%xmm0, %xmm2            # xmm2 = xmm2[1],xmm0[1]
// 0x1421a7aef  410f2e5114                     ucomiss	0x14(%r9), %xmm2
// 0x1421a7af4  7516                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7af6  7a14                           jp	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7af8  0f28d0                         movaps	%xmm0, %xmm2
// 0x1421a7afb  0fc6d0ff                       shufps	$0xff, %xmm0, %xmm2     # xmm2 = xmm2[3,3],xmm0[3,3]
// 0x1421a7aff  410f2e5118                     ucomiss	0x18(%r9), %xmm2
// 0x1421a7b04  7506                           jne	0x1421a7b0c <.text+0x21a6b0c>
// 0x1421a7b06  0f8b90000000                   jnp	0x1421a7b9c <.text+0x21a6b9c>
// Frames: F94 -> F95 -> F130
// 0x1421a7b0c  8b542428                       movl	0x28(%rsp), %edx
// 0x1421a7b10  4c8b442448                     movq	0x48(%rsp), %r8
// 0x1421a7b15  418914a8                       movl	%edx, (%r8,%rbp,4)
// Frames: F130
// 0x1421a7b19  41894908                       movl	%ecx, 0x8(%r9)
// 0x1421a7b1d  410f11410c                     movups	%xmm0, 0xc(%r9)
// 0x1421a7b22  410f13491c                     movlps	%xmm1, 0x1c(%r9)
// Frames: F131
// 0x1421a7b27  4c8b2f                         movq	(%rdi), %r13
// 0x1421a7b2a  8b4c242c                       movl	0x2c(%rsp), %ecx
// 0x1421a7b2e  488b542468                     movq	0x68(%rsp), %rdx
// Frames: F101 -> F102 -> F103 -> F132
// 0x1421a7b33  890a                           movl	%ecx, (%rdx)
// 0x1421a7b35  488b4c2430                     movq	0x30(%rsp), %rcx
// Frames: F105 -> F106 -> F107 -> F103 -> F132
// 0x1421a7b3a  488b7930                       movq	0x30(%rcx), %rdi
// Frames: F108 -> F107 -> F103 -> F132
// 0x1421a7b3e  488b6940                       movq	0x40(%rcx), %rbp
// Frames: F109 -> F106 -> F107 -> F103 -> F132
// 0x1421a7b42  483b7920                       cmpq	0x20(%rcx), %rdi
// 0x1421a7b46  7521                           jne	0x1421a7b69 <.text+0x21a6b69>
// Frames: F110 -> F106 -> F107 -> F103 -> F132
// 0x1421a7b48  488b4c2460                     movq	0x60(%rsp), %rcx
// 0x1421a7b4d  488d151cb29c00                 leaq	0x9cb21c(%rip), %rdx    # 0x142b72d70
// 0x1421a7b54  4d89cf                         movq	%r9, %r15
// 0x1421a7b57  e8d4b03bfe                     callq	0x140562c30 <.text+0x561c30>
// 0x1421a7b5c  4d89f9                         movq	%r15, %r9
// 0x1421a7b5f  4c8b7c2478                     movq	0x78(%rsp), %r15
// 0x1421a7b64  488b442438                     movq	0x38(%rsp), %rax
// Frames: F46 -> F47 -> F48 -> F111 -> F112 -> F106 -> F107 -> F103 -> F132
// 0x1421a7b69  4c8b442430                     movq	0x30(%rsp), %r8
// 0x1421a7b6e  498b4828                       movq	0x28(%r8), %rcx
// Frames: F113 -> F112 -> F106 -> F107 -> F103 -> F132
// 0x1421a7b72  4889fa                         movq	%rdi, %rdx
// 0x1421a7b75  48c1e205                       shlq	$0x5, %rdx
// Frames: F114 -> F115 -> F106 -> F107 -> F103 -> F132
// 0x1421a7b79  48892c11                       movq	%rbp, (%rcx,%rdx)
// 0x1421a7b7d  4889441108                     movq	%rax, 0x8(%rcx,%rdx)
// 0x1421a7b82  4c896c1110                     movq	%r13, 0x10(%rcx,%rdx)
// 0x1421a7b87  c644111802                     movb	$0x2, 0x18(%rcx,%rdx)
// Frames: F116 -> F106 -> F107 -> F103 -> F132
// 0x1421a7b8c  48ffc7                         incq	%rdi
// 0x1421a7b8f  49897830                       movq	%rdi, 0x30(%r8)
// Frames: F117 -> F107 -> F103 -> F132
// 0x1421a7b93  49ff4040                       incq	0x40(%r8)
// 0x1421a7b97  488b6c2440                     movq	0x40(%rsp), %rbp
// Frames: F133
// 0x1421a7b9c  4c89c9                         movq	%r9, %rcx
// 0x1421a7b9f  4c89cf                         movq	%r9, %rdi
// 0x1421a7ba2  e8a9b673fe                     callq	0x1408e3250 <.text+0x8e2250>
// 0x1421a7ba7  84c0                           testb	%al, %al
// 0x1421a7ba9  7428                           je	0x1421a7bd3 <.text+0x21a6bd3>
// Frames: F94 -> F95 -> F134
// 0x1421a7bab  8b442428                       movl	0x28(%rsp), %eax
// 0x1421a7baf  488b8c2480000000               movq	0x80(%rsp), %rcx
// 0x1421a7bb7  8904a9                         movl	%eax, (%rcx,%rbp,4)
// 0x1421a7bba  488b842488000000               movq	0x88(%rsp), %rax
// Frames: F134
// 0x1421a7bc2  c7401800000000                 movl	$0x0, 0x18(%rax)
// 0x1421a7bc9  48c7401000000000               movq	$0x0, 0x10(%rax)
// 0x1421a7bd1  eb55                           jmp	0x1421a7c28 <.text+0x21a6c28>
// Frames: F135
// 0x1421a7bd3  488d8c24a0000000               leaq	0xa0(%rsp), %rcx
// 0x1421a7bdb  4889fa                         movq	%rdi, %rdx
// 0x1421a7bde  e83db673fe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F136 -> F135
// 0x1421a7be3  f30f108424a0000000             movss	0xa0(%rsp), %xmm0
// 0x1421a7bec  f30f108c24a4000000             movss	0xa4(%rsp), %xmm1
// Frames: F137 -> F138 -> F135
// 0x1421a7bf5  f30f58ce                       addss	%xmm6, %xmm1
// Frames: F139 -> F135
// 0x1421a7bf9  f30f109424a8000000             movss	0xa8(%rsp), %xmm2
// 0x1421a7c02  8b442428                       movl	0x28(%rsp), %eax
// 0x1421a7c06  488b8c2480000000               movq	0x80(%rsp), %rcx
// Frames: F94 -> F95 -> F135
// 0x1421a7c0e  8904a9                         movl	%eax, (%rcx,%rbp,4)
// 0x1421a7c11  488b842488000000               movq	0x88(%rsp), %rax
// Frames: F135
// 0x1421a7c19  f30f114010                     movss	%xmm0, 0x10(%rax)
// 0x1421a7c1e  f30f114814                     movss	%xmm1, 0x14(%rax)
// 0x1421a7c23  f30f115018                     movss	%xmm2, 0x18(%rax)
// Frames: F140
// 0x1421a7c28  4989dd                         movq	%rbx, %r13
// 0x1421a7c2b  4d89e0                         movq	%r12, %r8
// 0x1421a7c2e  488b6c2470                     movq	0x70(%rsp), %rbp
// 0x1421a7c33  e9b8fbffff                     jmp	0x1421a77f0 <.text+0x21a67f0>
// Frames: F141
// 0x1421a7c38  0f28b424e0000000               movaps	0xe0(%rsp), %xmm6
// 0x1421a7c40  4881c4f8000000                 addq	$0xf8, %rsp
// 0x1421a7c47  5b                             popq	%rbx
// 0x1421a7c48  5d                             popq	%rbp
// 0x1421a7c49  5f                             popq	%rdi
// 0x1421a7c4a  5e                             popq	%rsi
// 0x1421a7c4b  415c                           popq	%r12
// 0x1421a7c4d  415d                           popq	%r13
// 0x1421a7c4f  415e                           popq	%r14
// 0x1421a7c51  415f                           popq	%r15
// 0x1421a7c53  c3                             retq
// Frames: F142
// 0x1421a7c54  4889bc24d0000000               movq	%rdi, 0xd0(%rsp)
// 0x1421a7c5c  488d055dd298fe                 leaq	-0x1672da3(%rip), %rax  # 0x140b34ec0 <.text+0xb33ec0>
// 0x1421a7c63  48898424d8000000               movq	%rax, 0xd8(%rsp)
// Frames: F143 -> F142
// 0x1421a7c6b  488d0526f1e400                 leaq	0xe4f126(%rip), %rax    # 0x142ff6d98
// 0x1421a7c72  48898424a0000000               movq	%rax, 0xa0(%rsp)
// 0x1421a7c7a  48c78424a800000002000000       movq	$0x2, 0xa8(%rsp)
// 0x1421a7c86  48c78424c000000000000000       movq	$0x0, 0xc0(%rsp)
// 0x1421a7c92  488d8424d0000000               leaq	0xd0(%rsp), %rax
// 0x1421a7c9a  48898424b0000000               movq	%rax, 0xb0(%rsp)
// 0x1421a7ca2  48c78424b800000001000000       movq	$0x1, 0xb8(%rsp)
// Frames: F142
// 0x1421a7cae  488d1503f1e400                 leaq	0xe4f103(%rip), %rdx    # 0x142ff6db8
// 0x1421a7cb5  488d8c24a0000000               leaq	0xa0(%rsp), %rcx
// 0x1421a7cbd  e8ce167300                     callq	0x1428d9390 <.text+0x28d8390>
// 0x1421a7cc2  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/update_roof_spatial.rs:12:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1235:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs:28:0 bevy_ecs::event::reader::EventReader::read
// F4: crates/systems/roof/src/update_roof_spatial.rs:19:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
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
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:729:0 core::option::Option::as_mut
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:333:0 core::iter::adapters::chain::and_then_or_clear
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:89:0 bevy_ecs::event::iterators::impl$3::next
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:15:0 bevy_ecs::event::iterators::impl$0::next
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next::closure$0
// F31: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1552:0 core::option::Option::or_else
// F32: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:617:0 core::ptr::non_null::NonNull::add
// F33: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:108:0 core::slice::iter::Iter::post_inc_start
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:443:0 core::slice::iter::impl$186::next_unchecked
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:165:0 core::slice::iter::impl$182::next
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:95:0 bevy_ecs::event::iterators::impl$3::next
// F37: crates/systems/roof/src/update_roof_spatial.rs:21:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F38: crates/systems/roof/src/update_roof_spatial.rs:20:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1451:0 core::option::Option::and_then
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:843:0 bevy_ecs::entity::Entities::get
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1595:0 bevy_ecs::system::query::Query::get_inner
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1577:0 bevy_ecs::system::query::Query::get_mut
// F43: crates/systems/roof/src/update_roof_spatial.rs:22:0 system_roof::update_roof_spatial::update_roof_spatial::closure$0
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::option::Option::and_then
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F46: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F47: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 core::slice::impl$0::get
// F51: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F52: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1742:0 core::cmp::impls::impl$24::ne
// F53: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:201:0 core::num::nonzero::impl$2::ne
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:844:0 bevy_ecs::entity::Entities::get
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:76:0 bevy_ecs::archetype::impl$25::eq
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:845:0 bevy_ecs::entity::Entities::get
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1603:0 bevy_ecs::system::query::Query::get_inner
// F59: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/bool.rs:62:0 core::bool::impl$0::then
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F64: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/internal_macros.rs:36:0 fixedbitset::FixedBitSet::get_unchecked
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1600:0 bevy_ecs::system::query::Query::get_inner
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1632:0 bevy_ecs::system::query::Query::get_inner
// F68: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 alloc::vec::impl$8::deref
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1634:0 bevy_ecs::system::query::Query::get_inner
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1544:0 bevy_ecs::query::fetch::impl$36::set_table
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1533:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$3::set_archetype
// F75: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F76: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F80: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F81: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:256:0 core::slice::index::impl$2::get_unchecked
// F82: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 nonmax::NonMaxUsize::get
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs:305:0 bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:320:0 bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1548:0 bevy_ecs::query::fetch::impl$36::set_table
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1608:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1601:0 bevy_ecs::query::fetch::impl$37::fetch
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2090:0 bevy_ecs::query::fetch::impl$63::fetch
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1638:0 bevy_ecs::system::query::Query::get_inner
// F92: crates/systems/roof/src/update_roof_spatial.rs:24:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F93: crates/systems/roof/src/update_roof_spatial.rs:25:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F96: crates/systems/roof/src/update_roof_spatial.rs:28:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F97: crates/country-core/src/resources/roofs/roof_shape.rs:48:0 country_core::resources::roofs::roof_shape::impl$16::eq
// F98: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:262:0 core::cmp::PartialEq::ne
// F99: crates/systems/roof/src/update_roof_spatial.rs:29:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F100: crates/systems/roof/src/update_roof_spatial.rs:33:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F102: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs:75:0 bevy_ecs::event::writer::EventWriter::write
// F104: crates/systems/roof/src/update_roof_spatial.rs:30:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F105: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2419:0 alloc::vec::Vec::push
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:139:0 bevy_ecs::event::collections::Events::send_with_caller
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::change_detection::impl$38::deref_mut
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:130:0 bevy_ecs::event::collections::Events::send_with_caller
// F109: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2422:0 alloc::vec::Vec::push
// F110: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2423:0 alloc::vec::Vec::push
// F111: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1734:0 alloc::vec::Vec::as_mut_ptr
// F112: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2426:0 alloc::vec::Vec::push
// F113: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 alloc::vec::Vec::as_mut_ptr
// F114: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F115: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2427:0 alloc::vec::Vec::push
// F116: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2428:0 alloc::vec::Vec::push
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:140:0 bevy_ecs::event::collections::Events::send_with_caller
// F118: crates/systems/roof/src/update_roof_spatial.rs:76:0 system_roof::update_roof_spatial::get_shape_params
// F119: crates/systems/roof/src/update_roof_spatial.rs:38:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F120: crates/systems/roof/src/update_roof_spatial.rs:78:0 system_roof::update_roof_spatial::get_shape_params
// F121: crates/country-core/src/systems/wall/valid_enclosure/mod.rs:67:0 country_core::systems::wall::valid_enclosure::impl$9::eq
// F122: crates/systems/roof/src/update_roof_spatial.rs:40:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:22:0 glam::f32::vec2::impl$102::eq
// F124: crates/utils/src/geometry/rectangle.rs:118:0 utils::geometry::rectangle::impl$10::eq
// F125: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1922:0 core::cmp::impls::impl$9::eq
// F126: crates/utils/src/geometry/rectangle.rs:119:0 utils::geometry::rectangle::impl$10::eq
// F127: crates/utils/src/geometry/rectangle.rs:120:0 utils::geometry::rectangle::impl$10::eq
// F128: crates/utils/src/geometry/circle.rs:19:0 utils::geometry::circle::impl$6::eq
// F129: crates/country-core/src/systems/wall/valid_enclosure/mod.rs:66:0 country_core::systems::wall::valid_enclosure::impl$9::eq
// F130: crates/systems/roof/src/update_roof_spatial.rs:42:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F131: crates/systems/roof/src/update_roof_spatial.rs:46:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F132: crates/systems/roof/src/update_roof_spatial.rs:43:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F133: crates/systems/roof/src/update_roof_spatial.rs:55:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F134: crates/systems/roof/src/update_roof_spatial.rs:62:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F135: crates/systems/roof/src/update_roof_spatial.rs:58:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1475:0 glam::f32::vec3::impl$50::sub
// F137: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$77::sub
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1476:0 glam::f32::vec3::impl$50::sub
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs:1477:0 glam::f32::vec3::impl$50::sub
// F140: crates/systems/roof/src/update_roof_spatial.rs:60:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F141: crates/systems/roof/src/update_roof_spatial.rs:73:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F142: crates/systems/roof/src/update_roof_spatial.rs:66:0 system_roof::update_roof_spatial::update_roof_spatial::h8b4c1d2ae89e59b9
// F143: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/mod.rs:613:0 core::fmt::Arguments::new_v1
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/update_roof_spatial.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 21316421b98a6be055e752bed68f2b51b59b709de8aec533e60c395c8077c0af
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: update_roof_spatial.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3465
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::update_roof_spatial::update_roof_spatial
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3465,"record_offset":260,"rva":35288848,"va":5403997968}
// 编译实例 RVAs: 0x21a7710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_roof_spatial.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26092356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403997968,"raw_hex":"e8bcdc71fe","size":5,"soft_fail":false,"target_va":5377906288,"va":5403998639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-29642231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403997968,"raw_hex":"e809b23bfe","size":5,"soft_fail":false,"target_va":5374356528,"va":5403998754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7542478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403997968,"raw_hex":"e8ce167300","size":5,"soft_fail":false,"target_va":5411541904,"va":5403999421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25971139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403997968,"raw_hex":"e83db673fe","size":5,"soft_fail":false,"target_va":5378028064,"va":5403999198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25971031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403997968,"raw_hex":"e8a9b673fe","size":5,"soft_fail":false,"target_va":5378028112,"va":5403999138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23581547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403997968,"raw_hex":"e8952c98fe","size":5,"soft_fail":false,"target_va":5380417056,"va":5403998598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24205959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403997968,"raw_hex":"e879a58efe","size":5,"soft_fail":false,"target_va":5379792624,"va":5403998578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24204686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403997968,"raw_hex":"e872aa8efe","size":5,"soft_fail":false,"target_va":5379793584,"va":5403998265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3465,"record_offset":3620,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3465,"record_offset":1140,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3465,"record_offset":432,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3465,"record_offset":3788,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3465,"record_offset":3676,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3465,"record_offset":4948,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":1480,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3465,"record_offset":4828,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_direct] system_roof::update_roof_spatial::get_shape_params
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/update_roof_spatial.rs
//     evidence={"module":3465,"record_offset":4116,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":116683,"source_line":75,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3465,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3465,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3465,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3465,"record_offset":3952,"parent_record_offset":3908,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3465,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":260,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3465,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3465,"record_offset":3908,"parent_record_offset":3872,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3465,"record_offset":744,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3465,"record_offset":1932,"parent_record_offset":1600,"ancestor_record_offset":260,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3465,"record_offset":3844,"parent_record_offset":3812,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3465,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3465,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":260,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3465,"record_offset":1600,"parent_record_offset":1556,"ancestor_record_offset":260,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3465,"record_offset":3872,"parent_record_offset":3812,"ancestor_record_offset":260,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3465,"record_offset":496,"parent_record_offset":456,"ancestor_record_offset":260,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3465,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":260,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3465,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":260,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":260,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":260,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":260,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":260,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":260,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":3296,"parent_record_offset":1556,"ancestor_record_offset":260,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3465,"record_offset":2404,"parent_record_offset":1556,"ancestor_record_offset":260,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3465,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":260,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3465,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":260,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3465,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":260,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3465,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":260,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3465,"record_offset":2144,"parent_record_offset":1556,"ancestor_record_offset":260,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3465,"record_offset":2796,"parent_record_offset":2452,"ancestor_record_offset":260,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3465,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":260,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3465,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":260,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3465,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":260,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3465,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":260,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3465,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":260,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3465,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":260,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3465,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":260,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3465,"record_offset":4204,"parent_record_offset":4172,"ancestor_record_offset":260,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3465,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":260,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3465,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":260,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3465,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":260,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3465,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":260,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3465,"record_offset":560,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3465,"record_offset":1876,"parent_record_offset":1600,"ancestor_record_offset":260,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3465,"record_offset":4860,"parent_record_offset":4828,"ancestor_record_offset":260,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3465,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":260,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":260,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":260,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":260,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":2640,"parent_record_offset":2504,"ancestor_record_offset":260,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":1272,"parent_record_offset":1192,"ancestor_record_offset":260,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3465,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3465,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3465,"record_offset":4072,"parent_record_offset":3908,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3465,"record_offset":616,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3465,"record_offset":984,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3465,"record_offset":1700,"parent_record_offset":1672,"ancestor_record_offset":260,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3465,"record_offset":1672,"parent_record_offset":1632,"ancestor_record_offset":260,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3465,"record_offset":2720,"parent_record_offset":2688,"ancestor_record_offset":260,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3465,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":260,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3465,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":260,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3465,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3465,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":260,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3465,"record_offset":1348,"parent_record_offset":1296,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3465,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":260,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3465,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":260,"inlinee":116681,"source_line":48,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":3465,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":260,"inlinee":97894,"source_line":62,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3465,"record_offset":1964,"parent_record_offset":1556,"ancestor_record_offset":260,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3465,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":260,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3465,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":260,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3465,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":260,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3465,"record_offset":4260,"parent_record_offset":4228,"ancestor_record_offset":260,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3465,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":260,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::update_roof_spatial::update_roof_spatial::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/update_roof_spatial.rs
//     evidence={"module":3465,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":260,"inlinee":116679,"source_line":22,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3465,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":260,"inlinee":97898,"source_line":16,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3465,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":260,"inlinee":97896,"source_line":114,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::update_roof_spatial::update_roof_spatial::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C7C7
// 首条位置: {"module":3465,"inlinee":116679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3465,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":1508,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3465,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":1508,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3465,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1508,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3465,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":1508,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3465,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1508,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3465,"record_offset":1732,"parent_record_offset":1632,"ancestor_record_offset":1508,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3465,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1508,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3465,"record_offset":1932,"parent_record_offset":1600,"ancestor_record_offset":1508,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3465,"record_offset":1600,"parent_record_offset":1556,"ancestor_record_offset":1508,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":1508,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":1508,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1508,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":1508,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":1508,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3465,"record_offset":3296,"parent_record_offset":1556,"ancestor_record_offset":1508,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3465,"record_offset":2404,"parent_record_offset":1556,"ancestor_record_offset":1508,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3465,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":1508,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3465,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":1508,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3465,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":1508,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3465,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":1508,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3465,"record_offset":2144,"parent_record_offset":1556,"ancestor_record_offset":1508,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3465,"record_offset":2796,"parent_record_offset":2452,"ancestor_record_offset":1508,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3465,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":1508,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3465,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":1508,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3465,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":1508,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3465,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":1508,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3465,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1508,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3465,"record_offset":1876,"parent_record_offset":1600,"ancestor_record_offset":1508,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3465,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":1508,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":1508,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":1508,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3465,"record_offset":2640,"parent_record_offset":2504,"ancestor_record_offset":1508,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3465,"record_offset":2096,"parent_record_offset":2064,"ancestor_record_offset":1508,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3465,"record_offset":1632,"parent_record_offset":1600,"ancestor_record_offset":1508,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3465,"record_offset":1700,"parent_record_offset":1672,"ancestor_record_offset":1508,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3465,"record_offset":1672,"parent_record_offset":1632,"ancestor_record_offset":1508,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3465,"record_offset":2720,"parent_record_offset":2688,"ancestor_record_offset":1508,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3465,"record_offset":1964,"parent_record_offset":1556,"ancestor_record_offset":1508,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3465,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1508,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3465,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1508,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3465,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":1508,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3465,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":1508,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03465.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::update_roof_spatial::get_shape_params
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C7CB
// 首条位置: {"module":3465,"inlinee":116683}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::get_shape_params
// 信息 ID: 61b7301f6b7696a76dddc9ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3465, inline_record=4116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::update_roof_spatial
// 信息 ID: d6dd6e91d1bba9f9a76c8bc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3a48aeb128215955ee03632b: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)> > > @ 0x1a2809e -> 0x21a7710 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)> > @ 0x1a350b6 -> 0x21a7710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::update_roof_spatial::closure$0
// 信息 ID: 56ac0033586a19ca553db42b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::update_roof_spatial::update_roof_spatial", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3465, inline_record=1508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
