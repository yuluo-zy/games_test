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
// Inline site 3779:1460 -> host 3779:268 / rva_28273d0_len_6b9; exact_symbolizer_function_frame
//   Function-frame RVA span [0x2827596, 0x28276a0), 63 instructions
//
// FUNCTION system_roof::update_roof_spatial::get_shape_params
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3779:4440 -> host 3779:268 / rva_28273d0_len_6b9; exact_symbolizer_function_frame
//   Function-frame RVA span [0x282783b, 0x2827861), 6 instructions
//
// FUNCTION system_roof::update_roof_spatial::update_roof_spatial
// Candidate interfaces: 9ac2be146dab86716e231d9b
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3779:268 -> rva_28273d0_len_6b9
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_28273d0_len_6b9: RVA 0x28273d0, 1721 PDB bytes; 393 instructions
// Evidence: _assembly/roof-v1/C/pack-002/rva_28273d0_len_6b9.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3779:268: system_roof::update_roof_spatial::update_roof_spatial
// Frames: F1
// 0x1428273d0  4157                           pushq	%r15
// 0x1428273d2  4156                           pushq	%r14
// 0x1428273d4  4155                           pushq	%r13
// 0x1428273d6  4154                           pushq	%r12
// 0x1428273d8  56                             pushq	%rsi
// 0x1428273d9  57                             pushq	%rdi
// 0x1428273da  55                             pushq	%rbp
// 0x1428273db  53                             pushq	%rbx
// 0x1428273dc  4881ec18010000                 subq	$0x118, %rsp            # imm = 0x118
// 0x1428273e3  440f298c2400010000             movaps	%xmm9, 0x100(%rsp)
// 0x1428273ec  440f298424f0000000             movaps	%xmm8, 0xf0(%rsp)
// 0x1428273f5  0f29bc24e0000000               movaps	%xmm7, 0xe0(%rsp)
// 0x1428273fd  0f29b424d0000000               movaps	%xmm6, 0xd0(%rsp)
// 0x142827405  488b842488010000               movq	0x188(%rsp), %rax
// Frames: F2 -> F3 -> F4
// 0x14282740d  4d8b30                         movq	(%r8), %r14
// Frames: F5 -> F3 -> F4
// 0x142827410  4d8b5808                       movq	0x8(%r8), %r11
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x142827414  498b3e                         movq	(%r14), %rdi
// 0x142827417  31f6                           xorl	%esi, %esi
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x142827419  4889fb                         movq	%rdi, %rbx
// 0x14282741c  492b5b18                       subq	0x18(%r11), %rbx
// 0x142827420  480f42de                       cmovbq	%rsi, %rbx
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x142827424  492b7b38                       subq	0x38(%r11), %rdi
// 0x142827428  4c8b942480010000               movq	0x180(%rsp), %r10
// 0x142827430  480f42fe                       cmovbq	%rsi, %rdi
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142827434  4989d8                         movq	%rbx, %r8
// 0x142827437  49c1e004                       shlq	$0x4, %r8
// 0x14282743b  4d034308                       addq	0x8(%r11), %r8
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x14282743f  4d8b6310                       movq	0x10(%r11), %r12
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142827443  4d8b6b30                       movq	0x30(%r11), %r13
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142827447  4929dc                         subq	%rbx, %r12
// 0x14282744a  4c0f42e6                       cmovbq	%rsi, %r12
// 0x14282744e  41bf08000000                   movl	$0x8, %r15d
// 0x142827454  4d0f42c7                       cmovbq	%r15, %r8
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142827458  4889fb                         movq	%rdi, %rbx
// 0x14282745b  48c1e304                       shlq	$0x4, %rbx
// 0x14282745f  49035b28                       addq	0x28(%r11), %rbx
// 0x142827463  4929fd                         subq	%rdi, %r13
// 0x142827466  490f42df                       cmovbq	%r15, %rbx
// 0x14282746a  4c0f42ee                       cmovbq	%rsi, %r13
// Frames: F20 -> F7 -> F8 -> F3 -> F4
// 0x14282746e  4d8b5b40                       movq	0x40(%r11), %r11
// 0x142827472  4d29eb                         subq	%r13, %r11
// 0x142827475  4d29e3                         subq	%r12, %r11
// 0x142827478  4d891e                         movq	%r11, (%r14)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F4
// 0x14282747b  49c1e404                       shlq	$0x4, %r12
// 0x14282747f  4d01c4                         addq	%r8, %r12
// 0x142827482  4c89a424c8000000               movq	%r12, 0xc8(%rsp)
// 0x14282748a  49c1e504                       shlq	$0x4, %r13
// 0x14282748e  4901dd                         addq	%rbx, %r13
// 0x142827491  4c89ac24c0000000               movq	%r13, 0xc0(%rsp)
// 0x142827499  488b32                         movq	(%rdx), %rsi
// 0x14282749c  488b11                         movq	(%rcx), %rdx
// 0x14282749f  4889542450                     movq	%rdx, 0x50(%rsp)
// 0x1428274a4  488b7908                       movq	0x8(%rcx), %rdi
// 0x1428274a8  8b4914                         movl	0x14(%rcx), %ecx
// 0x1428274ab  894c2420                       movl	%ecx, 0x20(%rsp)
// 0x1428274af  418b4a1c                       movl	0x1c(%r10), %ecx
// 0x1428274b3  894c245c                       movl	%ecx, 0x5c(%rsp)
// 0x1428274b7  498b0a                         movq	(%r10), %rcx
// 0x1428274ba  48898c24b8000000               movq	%rcx, 0xb8(%rsp)
// 0x1428274c2  498b4a10                       movq	0x10(%r10), %rcx
// 0x1428274c6  48898c24b0000000               movq	%rcx, 0xb0(%rsp)
// 0x1428274ce  8b481c                         movl	0x1c(%rax), %ecx
// 0x1428274d1  894c2458                       movl	%ecx, 0x58(%rsp)
// 0x1428274d5  488b08                         movq	(%rax), %rcx
// 0x1428274d8  488b4010                       movq	0x10(%rax), %rax
// 0x1428274dc  48898424a8000000               movq	%rax, 0xa8(%rsp)
// 0x1428274e4  418b411c                       movl	0x1c(%r9), %eax
// 0x1428274e8  89442424                       movl	%eax, 0x24(%rsp)
// 0x1428274ec  498b01                         movq	(%r9), %rax
// 0x1428274ef  498b5110                       movq	0x10(%r9), %rdx
// 0x1428274f3  4889542468                     movq	%rdx, 0x68(%rsp)
// 0x1428274f8  f20f103da0147000               movsd	0x7014a0(%rip), %xmm7   # 0x142f289a0
// 0x142827500  f2440f10059713d600             movsd	0xd61397(%rip), %xmm8   # 0x1435888a0
// 0x142827509  48894c2470                     movq	%rcx, 0x70(%rsp)
// 0x14282750e  4883c120                       addq	$0x20, %rcx
// 0x142827512  48898c24a0000000               movq	%rcx, 0xa0(%rsp)
// 0x14282751a  4889442428                     movq	%rax, 0x28(%rsp)
// 0x14282751f  4883c020                       addq	$0x20, %rax
// 0x142827523  4889442460                     movq	%rax, 0x60(%rsp)
// 0x142827528  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F4
// 0x142827530  4d85c0                         testq	%r8, %r8
// 0x142827533  741b                           je	0x142827550 <.text+0x2826550>
// 0x142827535  4c3b8424c8000000               cmpq	0xc8(%rsp), %r8
// 0x14282753d  7411                           je	0x142827550 <.text+0x2826550>
// 0x14282753f  4d8d7810                       leaq	0x10(%r8), %r15
// 0x142827543  4989dc                         movq	%rbx, %r12
// 0x142827546  4c89c5                         movq	%r8, %rbp
// 0x142827549  eb29                           jmp	0x142827574 <.text+0x2826574>
// 0x14282754b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F25 -> F30 -> F31 -> F27 -> F28 -> F29 -> F4
// 0x142827550  4885db                         testq	%rbx, %rbx
// 0x142827553  0f84c3040000                   je	0x142827a1c <.text+0x2826a1c>
// 0x142827559  483b9c24c0000000               cmpq	0xc0(%rsp), %rbx
// 0x142827561  0f84b5040000                   je	0x142827a1c <.text+0x2826a1c>
// Frames: F32 -> F33 -> F30 -> F31 -> F27 -> F28 -> F29 -> F4
// 0x142827567  4c8d6310                       leaq	0x10(%rbx), %r12
// 0x14282756b  4531ff                         xorl	%r15d, %r15d
// 0x14282756e  4889dd                         movq	%rbx, %rbp
// 0x142827571  4c89e3                         movq	%r12, %rbx
// Frames: F34 -> F29 -> F4
// 0x142827574  49ff06                         incq	(%r14)
// Frames: F35
// 0x142827577  4c8b6d08                       movq	0x8(%rbp), %r13
// 0x14282757b  4889f1                         movq	%rsi, %rcx
// 0x14282757e  4c89ea                         movq	%r13, %rdx
// 0x142827581  e8aa9837fe                     callq	0x140ba0e30 <.text+0xb9fe30>
// Frames: F36 -> F37
// 0x142827586  4889c1                         movq	%rax, %rcx
// 0x142827589  4d89f8                         movq	%r15, %r8
// 0x14282758c  baffffffff                     movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
// 0x142827591  4821d1                         andq	%rdx, %rcx
// 0x142827594  749a                           je	0x142827530 <.text+0x2826530>
// Frames: F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F37
// 0x142827596  4831d1                         xorq	%rdx, %rcx
// 0x142827599  4d89f8                         movq	%r15, %r8
// 0x14282759c  4c89e3                         movq	%r12, %rbx
// Frames: F44 -> F15 -> F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F37
// 0x14282759f  483b4f10                       cmpq	0x10(%rdi), %rcx
// Frames: F45 -> F46 -> F39 -> F40 -> F41 -> F42 -> F43 -> F37
// 0x1428275a3  738b                           jae	0x142827530 <.text+0x2826530>
// Frames: F47 -> F48 -> F49 -> F46 -> F39 -> F40 -> F41 -> F42 -> F43 -> F37
// 0x1428275a5  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1428275a9  48c1e103                       shlq	$0x3, %rcx
// 0x1428275ad  48034f08                       addq	0x8(%rdi), %rcx
// 0x1428275b1  4889c2                         movq	%rax, %rdx
// 0x1428275b4  48c1ea20                       shrq	$0x20, %rdx
// 0x1428275b8  4d89f8                         movq	%r15, %r8
// 0x1428275bb  4c89e3                         movq	%r12, %rbx
// 0x1428275be  395110                         cmpl	%edx, 0x10(%rcx)
// Frames: F50 -> F46 -> F39 -> F40 -> F41 -> F42 -> F43 -> F37
// 0x1428275c1  0f8569ffffff                   jne	0x142827530 <.text+0x2826530>
// Frames: F51 -> F52 -> F41 -> F42 -> F43 -> F37
// 0x1428275c7  833900                         cmpl	$0x0, (%rcx)
// 0x1428275ca  4d89f8                         movq	%r15, %r8
// 0x1428275cd  4c89e3                         movq	%r12, %rbx
// 0x1428275d0  0f845affffff                   je	0x142827530 <.text+0x2826530>
// Frames: F53 -> F54 -> F41 -> F42 -> F43 -> F37
// 0x1428275d6  488b5108                       movq	0x8(%rcx), %rdx
// 0x1428275da  4189d1                         movl	%edx, %r9d
// 0x1428275dd  4d89f8                         movq	%r15, %r8
// 0x1428275e0  4c89e3                         movq	%r12, %rbx
// 0x1428275e3  4c8b542450                     movq	0x50(%rsp), %r10
// Frames: F55 -> F54 -> F41 -> F42 -> F43 -> F37
// 0x1428275e8  4d3b4a38                       cmpq	0x38(%r10), %r9
// Frames: F56 -> F57 -> F54 -> F41 -> F42 -> F43 -> F37
// 0x1428275ec  0f833effffff                   jae	0x142827530 <.text+0x2826530>
// Frames: F58 -> F59 -> F60 -> F56 -> F57 -> F54 -> F41 -> F42 -> F43 -> F37
// 0x1428275f2  4c8b442450                     movq	0x50(%rsp), %r8
// 0x1428275f7  4d8b4028                       movq	0x28(%r8), %r8
// Frames: F21 -> F58 -> F59 -> F60 -> F56 -> F57 -> F54 -> F41 -> F42 -> F43 -> F37
// 0x1428275fb  41c1e906                       shrl	$0x6, %r9d
// Frames: F61 -> F59 -> F60 -> F56 -> F57 -> F54 -> F41 -> F42 -> F43 -> F37
// 0x1428275ff  4f8b04c8                       movq	(%r8,%r9,8), %r8
// Frames: F59 -> F60 -> F56 -> F57 -> F54 -> F41 -> F42 -> F43 -> F37
// 0x142827603  490fa3d0                       btq	%rdx, %r8
// 0x142827607  4d89f8                         movq	%r15, %r8
// 0x14282760a  4c89e3                         movq	%r12, %rbx
// Frames: F62 -> F41 -> F42 -> F43 -> F37
// 0x14282760d  0f831dffffff                   jae	0x142827530 <.text+0x2826530>
// Frames: F63 -> F64 -> F65 -> F66 -> F17 -> F18 -> F67 -> F68 -> F41 -> F42 -> F43 -> F37
// 0x142827613  4889442438                     movq	%rax, 0x38(%rsp)
// 0x142827618  448b5104                       movl	0x4(%rcx), %r10d
// 0x14282761c  48c1ea20                       shrq	$0x20, %rdx
// 0x142827620  488b87a0010000                 movq	0x1a0(%rdi), %rax
// Frames: F69 -> F70 -> F67 -> F68 -> F41 -> F42 -> F43 -> F37
// 0x142827627  488d0cd2                       leaq	(%rdx,%rdx,8), %rcx
// 0x14282762b  4c8b442450                     movq	0x50(%rsp), %r8
// Frames: F71 -> F41 -> F42 -> F43 -> F37
// 0x142827630  498b9010010000                 movq	0x110(%r8), %rdx
// 0x142827637  4d8b8018010000                 movq	0x118(%r8), %r8
// Frames: F72 -> F73 -> F74 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x14282763e  4c8b4cc818                     movq	0x18(%rax,%rcx,8), %r9
// 0x142827643  488b44c838                     movq	0x38(%rax,%rcx,8), %rax
// Frames: F77 -> F78 -> F43 -> F79 -> F72 -> F73 -> F74 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x142827648  488b0cd0                       movq	(%rax,%rdx,8), %rcx
// Frames: F80 -> F81 -> F82 -> F72 -> F73 -> F74 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x14282764c  48f7d1                         notq	%rcx
// Frames: F83 -> F84 -> F81 -> F82 -> F72 -> F73 -> F74 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x14282764f  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x142827653  48c1e104                       shlq	$0x4, %rcx
// Frames: F85 -> F86 -> F87 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x142827657  498b540928                     movq	0x28(%r9,%rcx), %rdx
// 0x14282765c  4889542440                     movq	%rdx, 0x40(%rsp)
// Frames: F77 -> F78 -> F43 -> F79 -> F72 -> F73 -> F74 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x142827661  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F80 -> F81 -> F82 -> F72 -> F73 -> F74 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x142827665  48f7d0                         notq	%rax
// Frames: F83 -> F84 -> F81 -> F82 -> F72 -> F73 -> F74 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x142827668  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x14282766c  48c1e004                       shlq	$0x4, %rax
// Frames: F85 -> F86 -> F87 -> F75 -> F76 -> F71 -> F41 -> F42 -> F43 -> F37
// 0x142827670  498b540128                     movq	0x28(%r9,%rax), %rdx
// 0x142827675  4889942480000000               movq	%rdx, 0x80(%rsp)
// Frames: F88 -> F89 -> F90 -> F91 -> F92 -> F93 -> F94 -> F41 -> F42 -> F43 -> F37
// 0x14282767d  41f7d2                         notl	%r10d
// Frames: F21 -> F95 -> F90 -> F91 -> F92 -> F93 -> F94 -> F41 -> F42 -> F43 -> F37
// 0x142827680  496bda58                       imulq	$0x58, %r10, %rbx
// 0x142827684  49035c0910                     addq	0x10(%r9,%rcx), %rbx
// 0x142827689  4c89542448                     movq	%r10, 0x48(%rsp)
// Frames: F21 -> F96 -> F90 -> F91 -> F92 -> F93 -> F94 -> F41 -> F42 -> F43 -> F37
// 0x14282768e  4b8d0c52                       leaq	(%r10,%r10,2), %rcx
// 0x142827692  48c1e104                       shlq	$0x4, %rcx
// 0x142827696  49034c0110                     addq	0x10(%r9,%rax), %rcx
// 0x14282769b  48894c2478                     movq	%rcx, 0x78(%rsp)
// Frames: F97
// 0x1428276a0  4889f1                         movq	%rsi, %rcx
// 0x1428276a3  4c89ea                         movq	%r13, %rdx
// 0x1428276a6  4c8d053312d600                 leaq	0xd61233(%rip), %r8     # 0x1435888e0
// 0x1428276ad  e8be9b37fe                     callq	0x140ba1270 <.text+0xba0270>
// 0x1428276b2  4885c0                         testq	%rax, %rax
// 0x1428276b5  0f8497030000                   je	0x142827a52 <.text+0x2826a52>
// Frames: F98
// 0x1428276bb  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x1428276c3  4889c1                         movq	%rax, %rcx
// 0x1428276c6  e8e5a83cfe                     callq	0x140bf1fb0 <.text+0xbf0fb0>
// 0x1428276cb  0f28f0                         movaps	%xmm0, %xmm6
// Frames: F99
// 0x1428276ce  f30f10434c                     movss	0x4c(%rbx), %xmm0
// 0x1428276d3  0f2ec6                         ucomiss	%xmm6, %xmm0
// 0x1428276d6  48895c2430                     movq	%rbx, 0x30(%rsp)
// 0x1428276db  7506                           jne	0x1428276e3 <.text+0x28266e3>
// 0x1428276dd  0f8ba0000000                   jnp	0x142827783 <.text+0x2826783>
// Frames: F100 -> F101 -> F102
// 0x1428276e3  440f28ce                       movaps	%xmm6, %xmm9
// 0x1428276e7  440fc6ce00                     shufps	$0x0, %xmm6, %xmm9      # xmm9 = xmm9[0,0],xmm6[0,0]
// 0x1428276ec  440f59cf                       mulps	%xmm7, %xmm9
// 0x1428276f0  8b44245c                       movl	0x5c(%rsp), %eax
// 0x1428276f4  488b8c24b0000000               movq	0xb0(%rsp), %rcx
// Frames: F103 -> F104 -> F105
// 0x1428276fc  8901                           movl	%eax, (%rcx)
// Frames: F105
// 0x1428276fe  4c8b03                         movq	(%rbx), %r8
// 0x142827701  ba01000000                     movl	$0x1, %edx
// 0x142827706  488b8c24b8000000               movq	0xb8(%rsp), %rcx
// 0x14282770e  e84d292c00                     callq	0x142aea060 <.text+0x2ae9060>
// 0x142827713  0f28054612d600                 movaps	0xd61246(%rip), %xmm0   # 0x143588960
// 0x14282771a  0f2900                         movaps	%xmm0, (%rax)
// 0x14282771d  0f28054c12d600                 movaps	0xd6124c(%rip), %xmm0   # 0x143588970
// 0x142827724  0f294010                       movaps	%xmm0, 0x10(%rax)
// 0x142827728  0f28055112d600                 movaps	0xd61251(%rip), %xmm0   # 0x143588980
// 0x14282772f  0f294020                       movaps	%xmm0, 0x20(%rax)
// 0x142827733  440f294830                     movaps	%xmm9, 0x30(%rax)
// 0x142827738  8b442458                       movl	0x58(%rsp), %eax
// 0x14282773c  488b8c24a8000000               movq	0xa8(%rsp), %rcx
// Frames: F103 -> F104 -> F106 -> F107
// 0x142827744  8901                           movl	%eax, (%rcx)
// 0x142827746  488b442470                     movq	0x70(%rsp), %rax
// Frames: F108 -> F109 -> F110 -> F111 -> F112 -> F106 -> F107
// 0x14282774b  488b6830                       movq	0x30(%rax), %rbp
// Frames: F113 -> F111 -> F112 -> F106 -> F107
// 0x14282774f  488b5840                       movq	0x40(%rax), %rbx
// Frames: F114 -> F109 -> F110 -> F111 -> F112 -> F106 -> F107
// 0x142827753  483b6820                       cmpq	0x20(%rax), %rbp
// 0x142827757  750d                           jne	0x142827766 <.text+0x2826766>
// Frames: F115 -> F109 -> F110 -> F111 -> F112 -> F106 -> F107
// 0x142827759  488b8c24a0000000               movq	0xa0(%rsp), %rcx
// 0x142827761  e89a7c5400                     callq	0x142d6f400 <.text+0x2d6e400>
// Frames: F63 -> F64 -> F65 -> F116 -> F117 -> F109 -> F110 -> F111 -> F112 -> F106 -> F107
// 0x142827766  488b4c2470                     movq	0x70(%rsp), %rcx
// 0x14282776b  488b4128                       movq	0x28(%rcx), %rax
// Frames: F118 -> F119 -> F109 -> F110 -> F111 -> F112 -> F106 -> F107
// 0x14282776f  48891ce8                       movq	%rbx, (%rax,%rbp,8)
// Frames: F120 -> F109 -> F110 -> F111 -> F112 -> F106 -> F107
// 0x142827773  48ffc5                         incq	%rbp
// 0x142827776  48896930                       movq	%rbp, 0x30(%rcx)
// Frames: F121 -> F111 -> F112 -> F106 -> F107
// 0x14282777a  48ff4140                       incq	0x40(%rcx)
// 0x14282777e  488b5c2430                     movq	0x30(%rsp), %rbx
// Frames: F122 -> F123 -> F124
// 0x142827783  8b442420                       movl	0x20(%rsp), %eax
// 0x142827787  488b6c2448                     movq	0x48(%rsp), %rbp
// 0x14282778c  488b4c2440                     movq	0x40(%rsp), %rcx
// 0x142827791  8904a9                         movl	%eax, (%rcx,%rbp,4)
// Frames: F124
// 0x142827794  f30f11734c                     movss	%xmm6, 0x4c(%rbx)
// 0x142827799  488b842488000000               movq	0x88(%rsp), %rax
// Frames: F125
// 0x1428277a1  4889c1                         movq	%rax, %rcx
// 0x1428277a4  4881c19f020000                 addq	$0x29f, %rcx            # imm = 0x29F
// 0x1428277ab  0fb690a3020000                 movzbl	0x2a3(%rax), %edx
// 0x1428277b2  e8b9df42fe                     callq	0x140c55770 <.text+0xc54770>
// 0x1428277b7  488b4c2430                     movq	0x30(%rsp), %rcx
// Frames: F126 -> F127 -> F128
// 0x1428277bc  384154                         cmpb	%al, 0x54(%rcx)
// 0x1428277bf  4c8b4c2438                     movq	0x38(%rsp), %r9
// Frames: F128
// 0x1428277c4  7475                           je	0x14282783b <.text+0x282683b>
// Frames: F122 -> F123 -> F129
// 0x1428277c6  8b542420                       movl	0x20(%rsp), %edx
// 0x1428277ca  4c8b442440                     movq	0x40(%rsp), %r8
// 0x1428277cf  418914a8                       movl	%edx, (%r8,%rbp,4)
// Frames: F129
// 0x1428277d3  884154                         movb	%al, 0x54(%rcx)
// 0x1428277d6  8b442424                       movl	0x24(%rsp), %eax
// 0x1428277da  488b542468                     movq	0x68(%rsp), %rdx
// Frames: F103 -> F104 -> F130 -> F131
// 0x1428277df  8902                           movl	%eax, (%rdx)
// 0x1428277e1  488b442428                     movq	0x28(%rsp), %rax
// Frames: F108 -> F109 -> F110 -> F132 -> F130 -> F131
// 0x1428277e6  488b6830                       movq	0x30(%rax), %rbp
// Frames: F113 -> F132 -> F130 -> F131
// 0x1428277ea  488b5840                       movq	0x40(%rax), %rbx
// Frames: F114 -> F109 -> F110 -> F132 -> F130 -> F131
// 0x1428277ee  483b6820                       cmpq	0x20(%rax), %rbp
// 0x1428277f2  7514                           jne	0x142827808 <.text+0x2826808>
// Frames: F115 -> F109 -> F110 -> F132 -> F130 -> F131
// 0x1428277f4  488b4c2460                     movq	0x60(%rsp), %rcx
// 0x1428277f9  e8b27a5400                     callq	0x142d6f2b0 <.text+0x2d6e2b0>
// 0x1428277fe  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x142827803  4c8b4c2438                     movq	0x38(%rsp), %r9
// Frames: F63 -> F64 -> F65 -> F116 -> F117 -> F109 -> F110 -> F132 -> F130 -> F131
// 0x142827808  4c8b442428                     movq	0x28(%rsp), %r8
// 0x14282780d  498b4028                       movq	0x28(%r8), %rax
// Frames: F133 -> F117 -> F109 -> F110 -> F132 -> F130 -> F131
// 0x142827811  4889ea                         movq	%rbp, %rdx
// 0x142827814  48c1e205                       shlq	$0x5, %rdx
// Frames: F118 -> F119 -> F109 -> F110 -> F132 -> F130 -> F131
// 0x142827818  48891c10                       movq	%rbx, (%rax,%rdx)
// 0x14282781c  4c894c1008                     movq	%r9, 0x8(%rax,%rdx)
// 0x142827821  4c896c1010                     movq	%r13, 0x10(%rax,%rdx)
// 0x142827826  c644101802                     movb	$0x2, 0x18(%rax,%rdx)
// Frames: F120 -> F109 -> F110 -> F132 -> F130 -> F131
// 0x14282782b  48ffc5                         incq	%rbp
// 0x14282782e  49896830                       movq	%rbp, 0x30(%r8)
// Frames: F121 -> F132 -> F130 -> F131
// 0x142827832  49ff4040                       incq	0x40(%r8)
// 0x142827836  488b6c2448                     movq	0x48(%rsp), %rbp
// Frames: F134 -> F135
// 0x14282783b  488b942488000000               movq	0x88(%rsp), %rdx
// 0x142827843  8b8258020000                   movl	0x258(%rdx), %eax
// 0x142827849  83f801                         cmpl	$0x1, %eax
// 0x14282784c  0f8785000000                   ja	0x1428278d7 <.text+0x28268d7>
// Frames: F136 -> F135
// 0x142827852  0f10825c020000                 movups	0x25c(%rdx), %xmm0
// 0x142827859  f20f108a6c020000               movsd	0x26c(%rdx), %xmm1
// Frames: F137 -> F127 -> F138
// 0x142827861  3b4108                         cmpl	0x8(%rcx), %eax
// 0x142827864  0f85d7000000                   jne	0x142827941 <.text+0x2826941>
// 0x14282786a  83f801                         cmpl	$0x1, %eax
// 0x14282786d  0f859c000000                   jne	0x14282790f <.text+0x282690f>
// Frames: F139 -> F140 -> F137 -> F127 -> F138
// 0x142827873  0f2e491c                       ucomiss	0x1c(%rcx), %xmm1
// 0x142827877  0f85c4000000                   jne	0x142827941 <.text+0x2826941>
// 0x14282787d  0f8abe000000                   jp	0x142827941 <.text+0x2826941>
// Frames: F141 -> F140 -> F137 -> F127 -> F138
// 0x142827883  f30f16d1                       movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
// 0x142827887  0f2e5120                       ucomiss	0x20(%rcx), %xmm2
// 0x14282788b  0f85b0000000                   jne	0x142827941 <.text+0x2826941>
// 0x142827891  0f8aaa000000                   jp	0x142827941 <.text+0x2826941>
// Frames: F142 -> F143 -> F140 -> F137 -> F127 -> F138
// 0x142827897  0f2e410c                       ucomiss	0xc(%rcx), %xmm0
// Frames: F143 -> F140 -> F137 -> F127 -> F138
// 0x14282789b  0f85a0000000                   jne	0x142827941 <.text+0x2826941>
// 0x1428278a1  0f8a9a000000                   jp	0x142827941 <.text+0x2826941>
// 0x1428278a7  f30f16d0                       movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
// 0x1428278ab  0f2e5110                       ucomiss	0x10(%rcx), %xmm2
// 0x1428278af  0f858c000000                   jne	0x142827941 <.text+0x2826941>
// 0x1428278b5  0f8a86000000                   jp	0x142827941 <.text+0x2826941>
// Frames: F144 -> F140 -> F137 -> F127 -> F138
// 0x1428278bb  f20f105114                     movsd	0x14(%rcx), %xmm2
// Frames: F142 -> F144 -> F140 -> F137 -> F127 -> F138
// 0x1428278c0  0f28d8                         movaps	%xmm0, %xmm3
// 0x1428278c3  660f15d8                       unpckhpd	%xmm0, %xmm3            # xmm3 = xmm3[1],xmm0[1]
// 0x1428278c7  0fc2da00                       cmpeqps	%xmm2, %xmm3
// 0x1428278cb  0f14db                         unpcklps	%xmm3, %xmm3            # xmm3 = xmm3[0,0,1,1]
// 0x1428278ce  660f50d3                       movmskpd	%xmm3, %edx
// 0x1428278d2  83fa03                         cmpl	$0x3, %edx
// Frames: F138
// 0x1428278d5  756a                           jne	0x142827941 <.text+0x2826941>
// Frames: F145
// 0x1428278d7  4889cb                         movq	%rcx, %rbx
// 0x1428278da  e8f1133cfe                     callq	0x140be8cd0 <.text+0xbe7cd0>
// 0x1428278df  84c0                           testb	%al, %al
// 0x1428278e1  0f84e7000000                   je	0x1428279ce <.text+0x28269ce>
// Frames: F122 -> F123 -> F146
// 0x1428278e7  8b442420                       movl	0x20(%rsp), %eax
// 0x1428278eb  488b8c2480000000               movq	0x80(%rsp), %rcx
// 0x1428278f3  8904a9                         movl	%eax, (%rcx,%rbp,4)
// 0x1428278f6  488b442478                     movq	0x78(%rsp), %rax
// Frames: F146
// 0x1428278fb  c7401800000000                 movl	$0x0, 0x18(%rax)
// 0x142827902  48c7401000000000               movq	$0x0, 0x10(%rax)
// 0x14282790a  e902010000                     jmp	0x142827a11 <.text+0x2826a11>
// Frames: F147 -> F140 -> F148 -> F127 -> F138
// 0x14282790f  0f28d0                         movaps	%xmm0, %xmm2
// 0x142827912  660f15d0                       unpckhpd	%xmm0, %xmm2            # xmm2 = xmm2[1],xmm0[1]
// 0x142827916  0f2e5114                       ucomiss	0x14(%rcx), %xmm2
// 0x14282791a  7525                           jne	0x142827941 <.text+0x2826941>
// 0x14282791c  7a23                           jp	0x142827941 <.text+0x2826941>
// Frames: F142 -> F149 -> F140 -> F148 -> F127 -> F138
// 0x14282791e  0f2e410c                       ucomiss	0xc(%rcx), %xmm0
// Frames: F149 -> F140 -> F148 -> F127 -> F138
// 0x142827922  751d                           jne	0x142827941 <.text+0x2826941>
// 0x142827924  7a1b                           jp	0x142827941 <.text+0x2826941>
// 0x142827926  f30f16d0                       movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
// 0x14282792a  0f2e5110                       ucomiss	0x10(%rcx), %xmm2
// 0x14282792e  7511                           jne	0x142827941 <.text+0x2826941>
// 0x142827930  7a0f                           jp	0x142827941 <.text+0x2826941>
// Frames: F150 -> F151 -> F140 -> F148 -> F127 -> F138
// 0x142827932  0f28d0                         movaps	%xmm0, %xmm2
// 0x142827935  0fc6d0ff                       shufps	$0xff, %xmm0, %xmm2     # xmm2 = xmm2[3,3],xmm0[3,3]
// 0x142827939  0f2e5118                       ucomiss	0x18(%rcx), %xmm2
// Frames: F138
// 0x14282793d  7502                           jne	0x142827941 <.text+0x2826941>
// 0x14282793f  7b96                           jnp	0x1428278d7 <.text+0x28268d7>
// Frames: F122 -> F123 -> F152
// 0x142827941  8b542420                       movl	0x20(%rsp), %edx
// 0x142827945  4c8b442440                     movq	0x40(%rsp), %r8
// 0x14282794a  418914a8                       movl	%edx, (%r8,%rbp,4)
// Frames: F152
// 0x14282794e  894108                         movl	%eax, 0x8(%rcx)
// 0x142827951  0f11410c                       movups	%xmm0, 0xc(%rcx)
// 0x142827955  0f13491c                       movlps	%xmm1, 0x1c(%rcx)
// 0x142827959  8b442424                       movl	0x24(%rsp), %eax
// 0x14282795d  488b542468                     movq	0x68(%rsp), %rdx
// Frames: F103 -> F104 -> F130 -> F153
// 0x142827962  8902                           movl	%eax, (%rdx)
// 0x142827964  488b442428                     movq	0x28(%rsp), %rax
// Frames: F108 -> F109 -> F110 -> F132 -> F130 -> F153
// 0x142827969  488b5830                       movq	0x30(%rax), %rbx
// Frames: F113 -> F132 -> F130 -> F153
// 0x14282796d  488b6840                       movq	0x40(%rax), %rbp
// Frames: F114 -> F109 -> F110 -> F132 -> F130 -> F153
// 0x142827971  483b5820                       cmpq	0x20(%rax), %rbx
// 0x142827975  7514                           jne	0x14282798b <.text+0x282698b>
// Frames: F115 -> F109 -> F110 -> F132 -> F130 -> F153
// 0x142827977  488b4c2460                     movq	0x60(%rsp), %rcx
// 0x14282797c  e82f795400                     callq	0x142d6f2b0 <.text+0x2d6e2b0>
// 0x142827981  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x142827986  4c8b4c2438                     movq	0x38(%rsp), %r9
// Frames: F63 -> F64 -> F65 -> F116 -> F117 -> F109 -> F110 -> F132 -> F130 -> F153
// 0x14282798b  4c8b442428                     movq	0x28(%rsp), %r8
// 0x142827990  498b4028                       movq	0x28(%r8), %rax
// Frames: F133 -> F117 -> F109 -> F110 -> F132 -> F130 -> F153
// 0x142827994  4889da                         movq	%rbx, %rdx
// 0x142827997  48c1e205                       shlq	$0x5, %rdx
// Frames: F118 -> F119 -> F109 -> F110 -> F132 -> F130 -> F153
// 0x14282799b  48892c10                       movq	%rbp, (%rax,%rdx)
// 0x14282799f  4c894c1008                     movq	%r9, 0x8(%rax,%rdx)
// 0x1428279a4  4c896c1010                     movq	%r13, 0x10(%rax,%rdx)
// 0x1428279a9  c644101802                     movb	$0x2, 0x18(%rax,%rdx)
// Frames: F120 -> F109 -> F110 -> F132 -> F130 -> F153
// 0x1428279ae  48ffc3                         incq	%rbx
// 0x1428279b1  49895830                       movq	%rbx, 0x30(%r8)
// Frames: F121 -> F132 -> F130 -> F153
// 0x1428279b5  49ff4040                       incq	0x40(%r8)
// 0x1428279b9  488b6c2448                     movq	0x48(%rsp), %rbp
// 0x1428279be  4889cb                         movq	%rcx, %rbx
// Frames: F145
// 0x1428279c1  e80a133cfe                     callq	0x140be8cd0 <.text+0xbe7cd0>
// 0x1428279c6  84c0                           testb	%al, %al
// 0x1428279c8  0f8519ffffff                   jne	0x1428278e7 <.text+0x28268e7>
// Frames: F154
// 0x1428279ce  488d8c2490000000               leaq	0x90(%rsp), %rcx
// 0x1428279d6  4889da                         movq	%rbx, %rdx
// 0x1428279d9  e8f2143cfe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// Frames: F155 -> F154
// 0x1428279de  f30f10842498000000             movss	0x98(%rsp), %xmm0
// Frames: F156 -> F154
// 0x1428279e7  f20f108c2490000000             movsd	0x90(%rsp), %xmm1
// Frames: F157 -> F158 -> F154
// 0x1428279f0  410f58c8                       addps	%xmm8, %xmm1
// 0x1428279f4  8b442420                       movl	0x20(%rsp), %eax
// 0x1428279f8  488b8c2480000000               movq	0x80(%rsp), %rcx
// Frames: F122 -> F123 -> F154
// 0x142827a00  8904a9                         movl	%eax, (%rcx,%rbp,4)
// 0x142827a03  488b442478                     movq	0x78(%rsp), %rax
// Frames: F154
// 0x142827a08  0f134810                       movlps	%xmm1, 0x10(%rax)
// 0x142827a0c  f30f114018                     movss	%xmm0, 0x18(%rax)
// Frames: F159
// 0x142827a11  4d89f8                         movq	%r15, %r8
// 0x142827a14  4c89e3                         movq	%r12, %rbx
// 0x142827a17  e914fbffff                     jmp	0x142827530 <.text+0x2826530>
// Frames: F160
// 0x142827a1c  0f28b424d0000000               movaps	0xd0(%rsp), %xmm6
// 0x142827a24  0f28bc24e0000000               movaps	0xe0(%rsp), %xmm7
// 0x142827a2c  440f288424f0000000             movaps	0xf0(%rsp), %xmm8
// 0x142827a35  440f288c2400010000             movaps	0x100(%rsp), %xmm9
// 0x142827a3e  4881c418010000                 addq	$0x118, %rsp            # imm = 0x118
// 0x142827a45  5b                             popq	%rbx
// 0x142827a46  5d                             popq	%rbp
// 0x142827a47  5f                             popq	%rdi
// 0x142827a48  5e                             popq	%rsi
// 0x142827a49  415c                           popq	%r12
// 0x142827a4b  415d                           popq	%r13
// 0x142827a4d  415e                           popq	%r14
// 0x142827a4f  415f                           popq	%r15
// 0x142827a51  c3                             retq
// Frames: F161
// 0x142827a52  4883c508                       addq	$0x8, %rbp
// 0x142827a56  4889ac2490000000               movq	%rbp, 0x90(%rsp)
// 0x142827a5e  488d056bf70efe                 leaq	-0x1f10895(%rip), %rax  # 0x1409171d0 <.text+0x9161d0>
// 0x142827a65  4889842498000000               movq	%rax, 0x98(%rsp)
// 0x142827a6d  488d0d840ed600                 leaq	0xd60e84(%rip), %rcx    # 0x1435888f8
// 0x142827a74  4c8d05cd0ed600                 leaq	0xd60ecd(%rip), %r8     # 0x143588948
// 0x142827a7b  488d942490000000               leaq	0x90(%rsp), %rdx
// 0x142827a83  e8189e5800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x142827a88  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/update_roof_spatial.rs:13:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:1065:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:28:0 bevy_ecs::message::message_reader::MessageReader::read
// F4: crates/systems/roof/src/update_roof_spatial.rs:22:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:55:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:93:0 bevy_ecs::message::message_cursor::MessageCursor::read_with_id
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:80:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F9: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2345:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:57:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:60:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:923:0 core::num::impl$11::checked_sub
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:368:0 core::slice::index::impl$4::get
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:546:0 core::slice::index::impl$7::get
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:61:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:62:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:67:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F22: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 core::slice::impl$0::iter
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:69:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:767:0 core::option::Option::as_mut
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:332:0 core::iter::adapters::chain::and_then_or_clear
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:89:0 bevy_ecs::message::iterators::impl$3::next
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:15:0 bevy_ecs::message::iterators::impl$0::next
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next::closure$0
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1656:0 core::option::Option::or_else
// F32: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:659:0 core::ptr::non_null::NonNull::add
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:185:0 core::slice::iter::impl$171::next
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:95:0 bevy_ecs::message::iterators::impl$3::next
// F35: crates/systems/roof/src/update_roof_spatial.rs:24:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F36: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1545:0 core::option::Option::and_then
// F37: crates/systems/roof/src/update_roof_spatial.rs:25:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:957:0 bevy_ecs::entity::Entities::get_id_location
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:949:0 bevy_ecs::entity::Entities::get
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1546:0 bevy_ecs::system::query::Query::get_inner
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1525:0 bevy_ecs::system::query::Query::get_mut
// F42: crates/systems/roof/src/update_roof_spatial.rs:25:0 system_roof::update_roof_spatial::update_roof_spatial::closure$0
// F43: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::option::Option::and_then
// F44: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1583:0 core::option::Option::filter
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::Entities::get_id_location
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:214:0 bevy_ecs::entity::impl$37::eq
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::impl$18::get_id_location::closure$0
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1584:0 core::option::Option::filter
// F50: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1589:0 core::option::Option::filter
// F51: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1338:0 core::option::Option::ok_or
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1547:0 bevy_ecs::system::query::Query::get_inner
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:123:0 bevy_ecs::archetype::ArchetypeId::index
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1551:0 bevy_ecs::system::query::Query::get_inner
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 bevy_ecs::archetype::ArchetypeId::index
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 bevy_ecs::archetype::ArchetypeId::index
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F61: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/internal_macros.rs:28:0 fixedbitset::FixedBitSet::get_unchecked
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1548:0 bevy_ecs::system::query::Query::get_inner
// F63: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F64: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F65: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1580:0 bevy_ecs::system::query::Query::get_inner
// F69: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:221:0 core::slice::index::impl$2::get
// F70: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 alloc::vec::impl$9::deref
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1582:0 bevy_ecs::system::query::Query::get_inner
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1932:0 bevy_ecs::query::fetch::impl$48::set_table
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1921:0 bevy_ecs::query::fetch::impl$48::set_archetype
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$3::set_archetype
// F77: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F78: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F82: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F83: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F84: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs:305:0 bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:314:0 bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1939:0 bevy_ecs::query::fetch::impl$48::set_table
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2000:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1993:0 bevy_ecs::query::fetch::impl$49::fetch
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2529:0 bevy_ecs::query::fetch::impl$92::fetch
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1591:0 bevy_ecs::system::query::Query::get_inner
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ecs::storage::table::TableRow::index
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ptr::ThinSlicePtr::get
// F97: crates/systems/roof/src/update_roof_spatial.rs:27:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F98: crates/systems/roof/src/update_roof_spatial.rs:28:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F99: crates/systems/roof/src/update_roof_spatial.rs:33:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F100: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:352:0 core::ops::arith::impl$141::mul
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1503:0 glam::f32::vec3::impl$24::mul
// F102: crates/systems/roof/src/update_roof_spatial.rs:35:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F105: crates/systems/roof/src/update_roof_spatial.rs:34:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:94:0 bevy_ecs::message::message_writer::MessageWriter::write_default
// F107: crates/systems/roof/src/update_roof_spatial.rs:36:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F108: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1025:0 alloc::vec::Vec::push_mut
// F109: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:992:0 alloc::vec::Vec::push
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:143:0 bevy_ecs::message::messages::Messages::write_with_caller
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::message::messages::Messages::write
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:172:0 bevy_ecs::message::messages::Messages::write_default
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:131:0 bevy_ecs::message::messages::Messages::write_with_caller
// F114: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1028:0 alloc::vec::Vec::push_mut
// F115: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1029:0 alloc::vec::Vec::push_mut
// F116: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2000:0 alloc::vec::Vec::as_mut_ptr
// F117: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1032:0 alloc::vec::Vec::push_mut
// F118: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1917:0 core::ptr::write
// F119: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:0 alloc::vec::Vec::push_mut
// F120: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1034:0 alloc::vec::Vec::push_mut
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:144:0 bevy_ecs::message::messages::Messages::write_with_caller
// F122: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F124: crates/systems/roof/src/update_roof_spatial.rs:39:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F125: crates/systems/roof/src/update_roof_spatial.rs:42:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F126: crates/country-core/src/resources/roofs/roof_shape.rs:48:0 country_core::resources::roofs::roof_shape::impl$18::eq
// F127: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:265:0 core::cmp::PartialEq::ne
// F128: crates/systems/roof/src/update_roof_spatial.rs:43:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F129: crates/systems/roof/src/update_roof_spatial.rs:44:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F130: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:70:0 bevy_ecs::message::message_writer::MessageWriter::write
// F131: crates/systems/roof/src/update_roof_spatial.rs:45:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::change_detection::impl$38::deref_mut
// F133: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 alloc::vec::Vec::as_mut_ptr
// F134: crates/systems/roof/src/update_roof_spatial.rs:91:0 system_roof::update_roof_spatial::get_shape_params
// F135: crates/systems/roof/src/update_roof_spatial.rs:53:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F136: crates/systems/roof/src/update_roof_spatial.rs:93:0 system_roof::update_roof_spatial::get_shape_params
// F137: crates/country-core/src/systems/wall/valid_enclosure/mod.rs:67:0 country_core::systems::wall::valid_enclosure::impl$10::eq
// F138: crates/systems/roof/src/update_roof_spatial.rs:55:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F139: crates/utils/src/geometry/rectangle.rs:119:0 utils::geometry::rectangle::impl$12::eq
// F140: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:2123:0 core::cmp::impls::impl$9::eq
// F141: crates/utils/src/geometry/rectangle.rs:120:0 utils::geometry::rectangle::impl$12::eq
// F142: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:30:0 glam::f32::vec2::impl$103::eq
// F143: crates/utils/src/geometry/rectangle.rs:118:0 utils::geometry::rectangle::impl$12::eq
// F144: crates/utils/src/geometry/rectangle.rs:122:0 utils::geometry::rectangle::impl$12::eq
// F145: crates/systems/roof/src/update_roof_spatial.rs:70:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F146: crates/systems/roof/src/update_roof_spatial.rs:77:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F147: crates/utils/src/geometry/circle.rs:20:0 utils::geometry::circle::impl$7::eq
// F148: crates/country-core/src/systems/wall/valid_enclosure/mod.rs:66:0 country_core::systems::wall::valid_enclosure::impl$10::eq
// F149: crates/utils/src/geometry/circle.rs:19:0 utils::geometry::circle::impl$7::eq
// F150: crates/utils/src/circular_f32.rs:11:0 utils::circular_f32::impl$7::eq
// F151: crates/utils/src/geometry/circle.rs:24:0 utils::geometry::circle::impl$7::eq
// F152: crates/systems/roof/src/update_roof_spatial.rs:57:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F153: crates/systems/roof/src/update_roof_spatial.rs:58:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F154: crates/systems/roof/src/update_roof_spatial.rs:73:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1733:0 glam::f32::vec3::impl$50::sub
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1731:0 glam::f32::vec3::impl$50::sub
// F157: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:218:0 core::ops::arith::impl$77::sub
// F158: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1732:0 glam::f32::vec3::impl$50::sub
// F159: crates/systems/roof/src/update_roof_spatial.rs:75:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F160: crates/systems/roof/src/update_roof_spatial.rs:88:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// F161: crates/systems/roof/src/update_roof_spatial.rs:81:0 system_roof::update_roof_spatial::update_roof_spatial::hf829ebc6034ecfa6
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/update_roof_spatial.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b476060773ac29759430497473fab6eec038098a6a2bf3de1500dd8bd7947778
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: update_roof_spatial.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3779
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3779,"record_offset":268,"rva":42103760,"va":5410812880}
// 编译实例 RVAs: 0x28273d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142aea060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_roof_spatial.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2894157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e84d292c00","size":5,"soft_fail":false,"target_va":5413707872,"va":5410813710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_roof_spatial.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5536434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e8b27a5400","size":5,"soft_fail":false,"target_va":5416350384,"va":5410813945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_roof_spatial.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5536922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e89a7c5400","size":5,"soft_fail":false,"target_va":5416350720,"va":5410813793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5807640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e8189e5800","size":5,"soft_fail":false,"target_va":5416622240,"va":5410814595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29616910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e8f2143cfe","size":5,"soft_fail":false,"target_va":5381197520,"va":5410814425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29617167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e8f1133cfe","size":5,"soft_fail":false,"target_va":5381197008,"va":5410814170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_wall_state::WallMaterial::gable_roof_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29171783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e8b9df42fe","size":5,"soft_fail":false,"target_va":5381642096,"va":5410813874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29579035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e8e5a83cfe","size":5,"soft_fail":false,"target_va":5381234608,"va":5410813638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29910082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e8be9b37fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5410813613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29910870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410812880,"raw_hex":"e8aa9837fe","size":5,"soft_fail":false,"target_va":5380902448,"va":5410813313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3779,"record_offset":3464,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3779,"record_offset":3912,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3779,"record_offset":1148,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3779,"record_offset":448,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3779,"record_offset":4080,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3779,"record_offset":3520,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3779,"record_offset":3968,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":1432,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3779,"record_offset":3408,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3779,"record_offset":5256,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_direct] system_roof::update_roof_spatial::get_shape_params
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/update_roof_spatial.rs
//     evidence={"module":3779,"record_offset":4440,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":133048,"source_line":90,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3779,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3779,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3779,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":3748,"parent_record_offset":3712,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":3684,"parent_record_offset":3648,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":3712,"parent_record_offset":3684,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":768,"parent_record_offset":536,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3779,"record_offset":1792,"parent_record_offset":1508,"ancestor_record_offset":268,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3779,"record_offset":3488,"parent_record_offset":3464,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3779,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3779,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":268,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3779,"record_offset":1556,"parent_record_offset":1508,"ancestor_record_offset":268,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3779,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":268,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3779,"record_offset":1696,"parent_record_offset":1664,"ancestor_record_offset":268,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3779,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":268,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3779,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3779,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3779,"record_offset":512,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3779,"record_offset":3624,"parent_record_offset":3600,"ancestor_record_offset":268,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3779,"record_offset":3600,"parent_record_offset":3520,"ancestor_record_offset":268,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3779,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":268,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":268,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":268,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":268,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":3048,"parent_record_offset":1508,"ancestor_record_offset":268,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3779,"record_offset":2212,"parent_record_offset":1508,"ancestor_record_offset":268,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3779,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3779,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3779,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3779,"record_offset":2288,"parent_record_offset":2260,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3779,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3779,"record_offset":1980,"parent_record_offset":1508,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3779,"record_offset":2576,"parent_record_offset":2260,"ancestor_record_offset":268,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":3779,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":268,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3779,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3779,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":268,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3779,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":268,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3779,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":268,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3779,"record_offset":1828,"parent_record_offset":1792,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3779,"record_offset":4532,"parent_record_offset":4500,"ancestor_record_offset":268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3779,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3779,"record_offset":1200,"parent_record_offset":1172,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3779,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3779,"record_offset":724,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3779,"record_offset":584,"parent_record_offset":536,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3779,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":268,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3779,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3779,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":1664,"parent_record_offset":1580,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":2448,"parent_record_offset":2312,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":1764,"parent_record_offset":1508,"ancestor_record_offset":268,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":1280,"parent_record_offset":1200,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3779,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3779,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3779,"record_offset":3860,"parent_record_offset":3712,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3779,"record_offset":640,"parent_record_offset":536,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3779,"record_offset":992,"parent_record_offset":536,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3779,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3779,"record_offset":2528,"parent_record_offset":2496,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3779,"record_offset":696,"parent_record_offset":668,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3779,"record_offset":668,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3779,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3779,"record_offset":1356,"parent_record_offset":1304,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3779,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":268,"inlinee":133046,"source_line":48,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":3779,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":268,"inlinee":35232,"source_line":62,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3779,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":268,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3779,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":268,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3779,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":268,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3779,"record_offset":4592,"parent_record_offset":4556,"ancestor_record_offset":268,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3779,"record_offset":3176,"parent_record_offset":3144,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3779,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::update_roof_spatial::update_roof_spatial::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/update_roof_spatial.rs
//     evidence={"module":3779,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":268,"inlinee":133044,"source_line":25,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":3779,"record_offset":4740,"parent_record_offset":4680,"ancestor_record_offset":268,"inlinee":10600,"source_line":7,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3779,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":268,"inlinee":35238,"source_line":16,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3779,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":268,"inlinee":35234,"source_line":114,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::update_roof_spatial::update_roof_spatial::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x207B4
// 首条位置: {"module":3779,"inlinee":133044}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3779,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3779,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1460,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3779,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1460,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3779,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1460,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1460,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":2036,"parent_record_offset":2004,"ancestor_record_offset":1460,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3779,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":1460,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3779,"record_offset":1792,"parent_record_offset":1508,"ancestor_record_offset":1460,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3779,"record_offset":1556,"parent_record_offset":1508,"ancestor_record_offset":1460,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3779,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":1460,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3779,"record_offset":1696,"parent_record_offset":1664,"ancestor_record_offset":1460,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3779,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":1460,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":1460,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1460,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":1460,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":1460,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":1460,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3779,"record_offset":3048,"parent_record_offset":1508,"ancestor_record_offset":1460,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3779,"record_offset":2212,"parent_record_offset":1508,"ancestor_record_offset":1460,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3779,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":1460,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3779,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":1460,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3779,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":1460,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3779,"record_offset":2288,"parent_record_offset":2260,"ancestor_record_offset":1460,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3779,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":1460,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3779,"record_offset":1980,"parent_record_offset":1508,"ancestor_record_offset":1460,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3779,"record_offset":2576,"parent_record_offset":2260,"ancestor_record_offset":1460,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":3779,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":1460,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3779,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":1460,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3779,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":1460,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3779,"record_offset":1828,"parent_record_offset":1792,"ancestor_record_offset":1460,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3779,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":1460,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1460,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":1460,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":1664,"parent_record_offset":1580,"ancestor_record_offset":1460,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":2448,"parent_record_offset":2312,"ancestor_record_offset":1460,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3779,"record_offset":1764,"parent_record_offset":1508,"ancestor_record_offset":1460,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3779,"record_offset":1932,"parent_record_offset":1900,"ancestor_record_offset":1460,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3779,"record_offset":1608,"parent_record_offset":1580,"ancestor_record_offset":1460,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3779,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":1460,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3779,"record_offset":2528,"parent_record_offset":2496,"ancestor_record_offset":1460,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3779,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":1460,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3779,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1460,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3779,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1460,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3779,"record_offset":3176,"parent_record_offset":3144,"ancestor_record_offset":1460,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3779,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":1460,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::update_roof_spatial::get_shape_params
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x207B8
// 首条位置: {"module":3779,"inlinee":133048}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::get_shape_params
// 信息 ID: e3b2fb0ac95261a9ce59641a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3779, inline_record=4440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::update_roof_spatial
// 信息 ID: 96ed0843c228546447598402
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9ac2be146dab86716e231d9b: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)> > @ 0x1f26452 -> 0x28273d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>)> @ 0x1f3cc18 -> 0x28273d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::update_roof_spatial::update_roof_spatial::closure$0
// 信息 ID: 142df686eaf213e2192010ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::update_roof_spatial::update_roof_spatial", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3779, inline_record=1460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
