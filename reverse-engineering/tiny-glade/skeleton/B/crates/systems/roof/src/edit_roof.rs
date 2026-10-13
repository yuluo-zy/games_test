// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: edit_roof.rs.assembly.json.gz

// FUNCTION system_roof::edit_roof::edit_roof::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3468:2352 -> host 3468:492 / rva_21ab7c0_len_c67; exact_symbolizer_function_frame
//   Function-frame RVA span [0x21ac3ca, 0x21ac427), 14 instructions
//
// FUNCTION system_roof::edit_roof::edit_roof
// Candidate interfaces: 23b41926ba248e49c4540b0b
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3468:492 -> rva_21ab7c0_len_c67
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21ab7c0_len_c67: RVA 0x21ab7c0, 3175 PDB bytes; 694 instructions
// Evidence: _assembly/roof-v1/B/pack-002/rva_21ab7c0_len_c67.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3468:492: system_roof::edit_roof::edit_roof
// Frames: F1
// 0x1421ab7c0  4157                           pushq	%r15
// 0x1421ab7c2  4156                           pushq	%r14
// 0x1421ab7c4  4155                           pushq	%r13
// 0x1421ab7c6  4154                           pushq	%r12
// 0x1421ab7c8  56                             pushq	%rsi
// 0x1421ab7c9  57                             pushq	%rdi
// 0x1421ab7ca  55                             pushq	%rbp
// 0x1421ab7cb  53                             pushq	%rbx
// 0x1421ab7cc  4881ece8010000                 subq	$0x1e8, %rsp            # imm = 0x1E8
// 0x1421ab7d3  440f29bc24d0010000             movaps	%xmm15, 0x1d0(%rsp)
// 0x1421ab7dc  440f29b424c0010000             movaps	%xmm14, 0x1c0(%rsp)
// 0x1421ab7e5  440f29ac24b0010000             movaps	%xmm13, 0x1b0(%rsp)
// 0x1421ab7ee  440f29a424a0010000             movaps	%xmm12, 0x1a0(%rsp)
// 0x1421ab7f7  440f299c2490010000             movaps	%xmm11, 0x190(%rsp)
// 0x1421ab800  440f29942480010000             movaps	%xmm10, 0x180(%rsp)
// 0x1421ab809  440f298c2470010000             movaps	%xmm9, 0x170(%rsp)
// 0x1421ab812  440f29842460010000             movaps	%xmm8, 0x160(%rsp)
// 0x1421ab81b  0f29bc2450010000               movaps	%xmm7, 0x150(%rsp)
// 0x1421ab823  0f29b42440010000               movaps	%xmm6, 0x140(%rsp)
// Frames: F2 -> F3 -> F4
// 0x1421ab82b  4d8b30                         movq	(%r8), %r14
// Frames: F5 -> F3 -> F4
// 0x1421ab82e  4d8b5808                       movq	0x8(%r8), %r11
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x1421ab832  498b3e                         movq	(%r14), %rdi
// 0x1421ab835  31f6                           xorl	%esi, %esi
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x1421ab837  4889f8                         movq	%rdi, %rax
// 0x1421ab83a  492b4318                       subq	0x18(%r11), %rax
// 0x1421ab83e  480f42c6                       cmovbq	%rsi, %rax
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x1421ab842  492b7b38                       subq	0x38(%r11), %rdi
// 0x1421ab846  4c8b942460020000               movq	0x260(%rsp), %r10
// 0x1421ab84e  480f42fe                       cmovbq	%rsi, %rdi
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421ab852  4989c0                         movq	%rax, %r8
// 0x1421ab855  49c1e006                       shlq	$0x6, %r8
// 0x1421ab859  4d034308                       addq	0x8(%r11), %r8
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421ab85d  4d8b7b10                       movq	0x10(%r11), %r15
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421ab861  4d8b6330                       movq	0x30(%r11), %r12
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x1421ab865  4929c7                         subq	%rax, %r15
// 0x1421ab868  4c0f42fe                       cmovbq	%rsi, %r15
// 0x1421ab86c  bb08000000                     movl	$0x8, %ebx
// 0x1421ab871  4c0f42c3                       cmovbq	%rbx, %r8
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x1421ab875  4889f8                         movq	%rdi, %rax
// 0x1421ab878  48c1e006                       shlq	$0x6, %rax
// 0x1421ab87c  49034328                       addq	0x28(%r11), %rax
// 0x1421ab880  4929fc                         subq	%rdi, %r12
// 0x1421ab883  4c0f42e6                       cmovbq	%rsi, %r12
// 0x1421ab887  480f42c3                       cmovbq	%rbx, %rax
// 0x1421ab88b  488bb42458020000               movq	0x258(%rsp), %rsi
// Frames: F20 -> F7 -> F8 -> F3 -> F4
// 0x1421ab893  4d8b5b40                       movq	0x40(%r11), %r11
// 0x1421ab897  4d29e3                         subq	%r12, %r11
// 0x1421ab89a  4d29fb                         subq	%r15, %r11
// 0x1421ab89d  4c89b42420010000               movq	%r14, 0x120(%rsp)
// 0x1421ab8a5  4d891e                         movq	%r11, (%r14)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F4
// 0x1421ab8a8  49c1e706                       shlq	$0x6, %r15
// 0x1421ab8ac  4d01c7                         addq	%r8, %r15
// 0x1421ab8af  4c89bc2400010000               movq	%r15, 0x100(%rsp)
// 0x1421ab8b7  49c1e406                       shlq	$0x6, %r12
// 0x1421ab8bb  4901c4                         addq	%rax, %r12
// 0x1421ab8be  4c89a424f8000000               movq	%r12, 0xf8(%rsp)
// 0x1421ab8c6  488b09                         movq	(%rcx), %rcx
// 0x1421ab8c9  48898c2418010000               movq	%rcx, 0x118(%rsp)
// 0x1421ab8d1  4c8b22                         movq	(%rdx), %r12
// 0x1421ab8d4  4c8b6a08                       movq	0x8(%rdx), %r13
// 0x1421ab8d8  8b4a14                         movl	0x14(%rdx), %ecx
// 0x1421ab8db  894c2428                       movl	%ecx, 0x28(%rsp)
// 0x1421ab8df  8b4e1c                         movl	0x1c(%rsi), %ecx
// 0x1421ab8e2  898c2494000000                 movl	%ecx, 0x94(%rsp)
// 0x1421ab8e9  488b16                         movq	(%rsi), %rdx
// 0x1421ab8ec  488b4e10                       movq	0x10(%rsi), %rcx
// 0x1421ab8f0  48898c24f0000000               movq	%rcx, 0xf0(%rsp)
// 0x1421ab8f8  418b4a1c                       movl	0x1c(%r10), %ecx
// 0x1421ab8fc  894c242c                       movl	%ecx, 0x2c(%rsp)
// 0x1421ab900  4d8b1a                         movq	(%r10), %r11
// 0x1421ab903  498b4a10                       movq	0x10(%r10), %rcx
// 0x1421ab907  48894c2478                     movq	%rcx, 0x78(%rsp)
// 0x1421ab90c  488b8c2450020000               movq	0x250(%rsp), %rcx
// 0x1421ab914  448b511c                       movl	0x1c(%rcx), %r10d
// 0x1421ab918  44899424a4000000               movl	%r10d, 0xa4(%rsp)
// 0x1421ab920  4c8b11                         movq	(%rcx), %r10
// 0x1421ab923  488b4910                       movq	0x10(%rcx), %rcx
// 0x1421ab927  48898c2410010000               movq	%rcx, 0x110(%rsp)
// 0x1421ab92f  418b491c                       movl	0x1c(%r9), %ecx
// 0x1421ab933  894c243c                       movl	%ecx, 0x3c(%rsp)
// 0x1421ab937  498b09                         movq	(%r9), %rcx
// 0x1421ab93a  4d8b4910                       movq	0x10(%r9), %r9
// 0x1421ab93e  4c898c24a8000000               movq	%r9, 0xa8(%rsp)
// 0x1421ab946  450f57db                       xorps	%xmm11, %xmm11
// 0x1421ab94a  f3440f102531a07700             movss	0x77a031(%rip), %xmm12  # 0x142925984
// 0x1421ab953  f3440f1035a89f7700             movss	0x779fa8(%rip), %xmm14  # 0x142925904
// 0x1421ab95c  48899424b0000000               movq	%rdx, 0xb0(%rsp)
// 0x1421ab964  488d5220                       leaq	0x20(%rdx), %rdx
// 0x1421ab968  48899424d8000000               movq	%rdx, 0xd8(%rsp)
// 0x1421ab970  4c895c2430                     movq	%r11, 0x30(%rsp)
// 0x1421ab975  498d5320                       leaq	0x20(%r11), %rdx
// 0x1421ab979  4889542470                     movq	%rdx, 0x70(%rsp)
// 0x1421ab97e  4c899424c0000000               movq	%r10, 0xc0(%rsp)
// 0x1421ab986  498d5220                       leaq	0x20(%r10), %rdx
// 0x1421ab98a  48899424e8000000               movq	%rdx, 0xe8(%rsp)
// 0x1421ab992  48898c2480000000               movq	%rcx, 0x80(%rsp)
// 0x1421ab99a  4883c120                       addq	$0x20, %rcx
// 0x1421ab99e  48898c24e0000000               movq	%rcx, 0xe0(%rsp)
// 0x1421ab9a6  4c89ac24b8000000               movq	%r13, 0xb8(%rsp)
// 0x1421ab9ae  eb4c                           jmp	0x1421ab9fc <.text+0x21aa9fc>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32 -> F33
// 0x1421ab9b0  4c8b842480000000               movq	0x80(%rsp), %r8
// 0x1421ab9b8  498b4828                       movq	0x28(%r8), %rcx
// Frames: F34 -> F29 -> F30 -> F31 -> F32 -> F33
// 0x1421ab9bc  4889f2                         movq	%rsi, %rdx
// 0x1421ab9bf  48c1e205                       shlq	$0x5, %rdx
// Frames: F35 -> F36 -> F30 -> F31 -> F32 -> F33
// 0x1421ab9c3  4c893c11                       movq	%r15, (%rcx,%rdx)
// 0x1421ab9c7  48897c1108                     movq	%rdi, 0x8(%rcx,%rdx)
// 0x1421ab9cc  4c8b8c2488000000               movq	0x88(%rsp), %r9
// 0x1421ab9d4  4c894c1110                     movq	%r9, 0x10(%rcx,%rdx)
// 0x1421ab9d9  885c1118                       movb	%bl, 0x18(%rcx,%rdx)
// 0x1421ab9dd  88441119                       movb	%al, 0x19(%rcx,%rdx)
// Frames: F37 -> F30 -> F31 -> F32 -> F33
// 0x1421ab9e1  48ffc6                         incq	%rsi
// 0x1421ab9e4  49897030                       movq	%rsi, 0x30(%r8)
// Frames: F38 -> F31 -> F32 -> F33
// 0x1421ab9e8  49ff4040                       incq	0x40(%r8)
// 0x1421ab9ec  488b8424d0000000               movq	0xd0(%rsp), %rax
// 0x1421ab9f4  4c8b8424c8000000               movq	0xc8(%rsp), %r8
// Frames: F39 -> F40 -> F41 -> F42 -> F43 -> F4
// 0x1421ab9fc  4d85c0                         testq	%r8, %r8
// 0x1421ab9ff  741f                           je	0x1421aba20 <.text+0x21aaa20>
// 0x1421aba01  4c3b842400010000               cmpq	0x100(%rsp), %r8
// 0x1421aba09  7415                           je	0x1421aba20 <.text+0x21aaa20>
// 0x1421aba0b  498d7840                       leaq	0x40(%r8), %rdi
// 0x1421aba0f  4889c6                         movq	%rax, %rsi
// 0x1421aba12  4c89c3                         movq	%r8, %rbx
// 0x1421aba15  eb29                           jmp	0x1421aba40 <.text+0x21aaa40>
// 0x1421aba17  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F39 -> F44 -> F45 -> F41 -> F42 -> F43 -> F4
// 0x1421aba20  4885c0                         testq	%rax, %rax
// 0x1421aba23  0f84d7080000                   je	0x1421ac300 <.text+0x21ab300>
// 0x1421aba29  483b8424f8000000               cmpq	0xf8(%rsp), %rax
// 0x1421aba31  0f84c9080000                   je	0x1421ac300 <.text+0x21ab300>
// Frames: F46 -> F47 -> F48 -> F49 -> F44 -> F45 -> F41 -> F42 -> F43 -> F4
// 0x1421aba37  488d7040                       leaq	0x40(%rax), %rsi
// 0x1421aba3b  31ff                           xorl	%edi, %edi
// 0x1421aba3d  4889c3                         movq	%rax, %rbx
// Frames: F50 -> F43 -> F4
// 0x1421aba40  488b842420010000               movq	0x120(%rsp), %rax
// 0x1421aba48  48ff00                         incq	(%rax)
// Frames: F51
// 0x1421aba4b  4c8d7320                       leaq	0x20(%rbx), %r14
// 0x1421aba4f  488b5320                       movq	0x20(%rbx), %rdx
// 0x1421aba53  488b8c2418010000               movq	0x118(%rsp), %rcx
// Frames: F52
// 0x1421aba5b  e850688efe                     callq	0x140a922b0 <.text+0xa912b0>
// 0x1421aba60  4989c2                         movq	%rax, %r10
// Frames: F53 -> F52
// 0x1421aba63  48c1e820                       shrq	$0x20, %rax
// 0x1421aba67  0f845d090000                   je	0x1421ac3ca <.text+0x21ab3ca>
// Frames: F54 -> F55 -> F56 -> F57
// 0x1421aba6d  4489d1                         movl	%r10d, %ecx
// Frames: F58 -> F15 -> F54 -> F55 -> F56 -> F57
// 0x1421aba70  493b4d10                       cmpq	0x10(%r13), %rcx
// 0x1421aba74  0f83f2080000                   jae	0x1421ac36c <.text+0x21ab36c>
// Frames: F25 -> F26 -> F27 -> F59 -> F17 -> F60 -> F54 -> F55 -> F56 -> F57
// 0x1421aba7a  498b5508                       movq	0x8(%r13), %rdx
// 0x1421aba7e  4c89d0                         movq	%r10, %rax
// 0x1421aba81  48c1e820                       shrq	$0x20, %rax
// Frames: F61 -> F58 -> F15 -> F54 -> F55 -> F56 -> F57
// 0x1421aba85  4c8d0489                       leaq	(%rcx,%rcx,4), %r8
// Frames: F62 -> F63 -> F64 -> F55 -> F56 -> F57
// 0x1421aba89  42390482                       cmpl	%eax, (%rdx,%r8,4)
// Frames: F64 -> F55 -> F56 -> F57
// 0x1421aba8d  0f85d9080000                   jne	0x1421ac36c <.text+0x21ab36c>
// Frames: F65 -> F66 -> F55 -> F56 -> F57
// 0x1421aba93  4e8d0482                       leaq	(%rdx,%r8,4), %r8
// 0x1421aba97  418b5004                       movl	0x4(%r8), %edx
// 0x1421aba9b  41b9ffffffff                   movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
// 0x1421abaa1  4c39ca                         cmpq	%r9, %rdx
// Frames: F66 -> F55 -> F56 -> F57
// 0x1421abaa4  0f84c2080000                   je	0x1421ac36c <.text+0x21ab36c>
// Frames: F67 -> F68 -> F56 -> F57
// 0x1421abaaa  4889bc24c8000000               movq	%rdi, 0xc8(%rsp)
// 0x1421abab2  4889b424d0000000               movq	%rsi, 0xd0(%rsp)
// 0x1421ababa  4939542438                     cmpq	%rdx, 0x38(%r12)
// Frames: F69 -> F70 -> F68 -> F56 -> F57
// 0x1421ababf  0f86ce080000                   jbe	0x1421ac393 <.text+0x21ab393>
// Frames: F71 -> F72 -> F73 -> F69 -> F70 -> F68 -> F56 -> F57
// 0x1421abac5  498b442428                     movq	0x28(%r12), %rax
// Frames: F21 -> F71 -> F72 -> F73 -> F69 -> F70 -> F68 -> F56 -> F57
// 0x1421abaca  89d1                           movl	%edx, %ecx
// 0x1421abacc  c1e906                         shrl	$0x6, %ecx
// Frames: F74 -> F72 -> F73 -> F69 -> F70 -> F68 -> F56 -> F57
// 0x1421abacf  488b04c8                       movq	(%rax,%rcx,8), %rax
// Frames: F72 -> F73 -> F69 -> F70 -> F68 -> F56 -> F57
// 0x1421abad3  480fa3d0                       btq	%rdx, %rax
// Frames: F75 -> F56 -> F57
// 0x1421abad7  0f83b6080000                   jae	0x1421ac393 <.text+0x21ab393>
// Frames: F25 -> F26 -> F27 -> F59 -> F17 -> F18 -> F76 -> F77 -> F56 -> F57
// 0x1421abadd  418b400c                       movl	0xc(%r8), %eax
// 0x1421abae1  418b7810                       movl	0x10(%r8), %edi
// 0x1421abae5  498b8da8010000                 movq	0x1a8(%r13), %rcx
// Frames: F61 -> F58 -> F78 -> F76 -> F77 -> F56 -> F57
// 0x1421abaec  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F79 -> F56 -> F57
// 0x1421abaf0  498b942410010000               movq	0x110(%r12), %rdx
// Frames: F80 -> F81 -> F82 -> F83 -> F79 -> F56 -> F57
// 0x1421abaf8  4c8b44c118                     movq	0x18(%rcx,%rax,8), %r8
// 0x1421abafd  488b44c138                     movq	0x38(%rcx,%rax,8), %rax
// Frames: F84 -> F85 -> F86 -> F87 -> F80 -> F81 -> F82 -> F83 -> F79 -> F56 -> F57
// 0x1421abb02  488b04d0                       movq	(%rax,%rdx,8), %rax
// Frames: F88 -> F89 -> F90 -> F80 -> F81 -> F82 -> F83 -> F79 -> F56 -> F57
// 0x1421abb06  48f7d0                         notq	%rax
// Frames: F61 -> F91 -> F92 -> F89 -> F90 -> F80 -> F81 -> F82 -> F83 -> F79 -> F56 -> F57
// 0x1421abb09  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1421abb0d  48c1e004                       shlq	$0x4, %rax
// Frames: F21 -> F93 -> F94 -> F95 -> F96 -> F97 -> F56 -> F57
// 0x1421abb11  4c6bff58                       imulq	$0x58, %rdi, %r15
// 0x1421abb15  4d037c0010                     addq	0x10(%r8,%rax), %r15
// Frames: F98 -> F99 -> F100 -> F83 -> F79 -> F56 -> F57
// 0x1421abb1a  498b740028                     movq	0x28(%r8,%rax), %rsi
// Frames: F21 -> F93 -> F101 -> F95 -> F96 -> F97 -> F56 -> F57
// 0x1421abb1f  488d0cbd00000000               leaq	(,%rdi,4), %rcx
// 0x1421abb27  49034c0020                     addq	0x20(%r8,%rax), %rcx
// Frames: F102
// 0x1421abb2c  8b03                           movl	(%rbx), %eax
// 0x1421abb2e  0f57f6                         xorps	%xmm6, %xmm6
// 0x1421abb31  0f57ff                         xorps	%xmm7, %xmm7
// 0x1421abb34  85c0                           testl	%eax, %eax
// 0x1421abb36  7463                           je	0x1421abb9b <.text+0x21aab9b>
// 0x1421abb38  83f801                         cmpl	$0x1, %eax
// 0x1421abb3b  7533                           jne	0x1421abb70 <.text+0x21aab70>
// Frames: F103
// 0x1421abb3d  f30f107b04                     movss	0x4(%rbx), %xmm7
// Frames: F104 -> F105
// 0x1421abb42  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421abb45  f30f5fc7                       maxss	%xmm7, %xmm0
// Frames: F106 -> F105
// 0x1421abb49  410f28ce                       movaps	%xmm14, %xmm1
// 0x1421abb4d  f30f5dc8                       minss	%xmm0, %xmm1
// Frames: F107
// 0x1421abb51  f3410f5c7f2c                   subss	0x2c(%r15), %xmm7
// 0x1421abb57  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F105
// 0x1421abb5b  8904be                         movl	%eax, (%rsi,%rdi,4)
// Frames: F105
// 0x1421abb5e  f3410f114f2c                   movss	%xmm1, 0x2c(%r15)
// Frames: F110
// 0x1421abb64  eb35                           jmp	0x1421abb9b <.text+0x21aab9b>
// 0x1421abb66  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F111
// 0x1421abb70  f30f107b04                     movss	0x4(%rbx), %xmm7
// 0x1421abb75  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F112
// 0x1421abb79  8904be                         movl	%eax, (%rsi,%rdi,4)
// 0x1421abb7c  f3410f10472c                   movss	0x2c(%r15), %xmm0
// Frames: F112
// 0x1421abb82  f30f58c7                       addss	%xmm7, %xmm0
// Frames: F104 -> F113
// 0x1421abb86  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421abb89  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F106 -> F113
// 0x1421abb8d  410f28c6                       movaps	%xmm14, %xmm0
// 0x1421abb91  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F113
// 0x1421abb95  f3410f11472c                   movss	%xmm0, 0x2c(%r15)
// Frames: F114
// 0x1421abb9b  f3450f106f30                   movss	0x30(%r15), %xmm13
// Frames: F115
// 0x1421abba1  8b4308                         movl	0x8(%rbx), %eax
// 0x1421abba4  85c0                           testl	%eax, %eax
// 0x1421abba6  4c89942488000000               movq	%r10, 0x88(%rsp)
// 0x1421abbae  4c89b42428010000               movq	%r14, 0x128(%rsp)
// 0x1421abbb6  7438                           je	0x1421abbf0 <.text+0x21aabf0>
// 0x1421abbb8  83f802                         cmpl	$0x2, %eax
// 0x1421abbbb  7543                           jne	0x1421abc00 <.text+0x21aac00>
// Frames: F116
// 0x1421abbbd  f30f10730c                     movss	0xc(%rbx), %xmm6
// 0x1421abbc2  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F117
// 0x1421abbc6  8904be                         movl	%eax, (%rsi,%rdi,4)
// 0x1421abbc9  f3410f104730                   movss	0x30(%r15), %xmm0
// Frames: F117
// 0x1421abbcf  f30f58c6                       addss	%xmm6, %xmm0
// Frames: F104 -> F118
// 0x1421abbd3  410f28cc                       movaps	%xmm12, %xmm1
// 0x1421abbd7  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F106 -> F118
// 0x1421abbdb  450f28fe                       movaps	%xmm14, %xmm15
// 0x1421abbdf  f3440f5df9                     minss	%xmm1, %xmm15
// Frames: F118
// 0x1421abbe4  f3450f117f30                   movss	%xmm15, 0x30(%r15)
// 0x1421abbea  eb3b                           jmp	0x1421abc27 <.text+0x21aac27>
// 0x1421abbec  0f1f4000                       nopl	(%rax)
// Frames: F115
// 0x1421abbf0  31ed                           xorl	%ebp, %ebp
// 0x1421abbf2  450f28fd                       movaps	%xmm13, %xmm15
// 0x1421abbf6  eb32                           jmp	0x1421abc2a <.text+0x21aac2a>
// 0x1421abbf8  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F119
// 0x1421abc00  f30f10730c                     movss	0xc(%rbx), %xmm6
// Frames: F104 -> F120
// 0x1421abc05  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421abc08  f30f5fc6                       maxss	%xmm6, %xmm0
// Frames: F106 -> F120
// 0x1421abc0c  450f28fe                       movaps	%xmm14, %xmm15
// 0x1421abc10  f3440f5df8                     minss	%xmm0, %xmm15
// 0x1421abc15  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F120
// 0x1421abc19  8904be                         movl	%eax, (%rsi,%rdi,4)
// Frames: F120
// 0x1421abc1c  f3450f117f30                   movss	%xmm15, 0x30(%r15)
// Frames: F121
// 0x1421abc22  f3410f5cf5                     subss	%xmm13, %xmm6
// 0x1421abc27  40b501                         movb	$0x1, %bpl
// Frames: F122
// 0x1421abc2a  8b4328                         movl	0x28(%rbx), %eax
// 0x1421abc2d  450f57c0                       xorps	%xmm8, %xmm8
// 0x1421abc31  450f57d2                       xorps	%xmm10, %xmm10
// 0x1421abc35  85c0                           testl	%eax, %eax
// 0x1421abc37  0f84e6000000                   je	0x1421abd23 <.text+0x21aad23>
// 0x1421abc3d  83f802                         cmpl	$0x2, %eax
// 0x1421abc40  756e                           jne	0x1421abcb0 <.text+0x21aacb0>
// Frames: F123
// 0x1421abc42  f20f10432c                     movsd	0x2c(%rbx), %xmm0
// 0x1421abc47  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F124
// 0x1421abc4b  8904be                         movl	%eax, (%rsi,%rdi,4)
// Frames: F125 -> F126 -> F124
// 0x1421abc4e  f2410f104f24                   movsd	0x24(%r15), %xmm1
// 0x1421abc54  0f58c8                         addps	%xmm0, %xmm1
// Frames: F127 -> F128 -> F129 -> F130
// 0x1421abc57  0f28d1                         movaps	%xmm1, %xmm2
// 0x1421abc5a  0f59d1                         mulps	%xmm1, %xmm2
// 0x1421abc5d  f30f16da                       movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
// 0x1421abc61  f30f58da                       addss	%xmm2, %xmm3
// Frames: F131 -> F130
// 0x1421abc65  31c0                           xorl	%eax, %eax
// 0x1421abc67  410f2ede                       ucomiss	%xmm14, %xmm3
// 0x1421abc6b  0f97c0                         seta	%al
// 0x1421abc6e  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421abc71  f30f51d3                       sqrtss	%xmm3, %xmm2
// 0x1421abc75  f30f12d2                       movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
// 0x1421abc79  0f28d9                         movaps	%xmm1, %xmm3
// 0x1421abc7c  0f5eda                         divps	%xmm2, %xmm3
// 0x1421abc7f  660f6ed0                       movd	%eax, %xmm2
// 0x1421abc83  660f70d250                     pshufd	$0x50, %xmm2, %xmm2     # xmm2 = xmm2[0,0,1,1]
// 0x1421abc88  660f72f21f                     pslld	$0x1f, %xmm2
// 0x1421abc8d  660f72e21f                     psrad	$0x1f, %xmm2
// 0x1421abc92  0f54da                         andps	%xmm2, %xmm3
// 0x1421abc95  0f55d1                         andnps	%xmm1, %xmm2
// 0x1421abc98  0f56d3                         orps	%xmm3, %xmm2
// Frames: F130
// 0x1421abc9b  410f135724                     movlps	%xmm2, 0x24(%r15)
// 0x1421abca0  eb6d                           jmp	0x1421abd0f <.text+0x21aad0f>
// 0x1421abca2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F132
// 0x1421abcb0  f20f10432c                     movsd	0x2c(%rbx), %xmm0
// Frames: F133
// 0x1421abcb5  f2410f104f24                   movsd	0x24(%r15), %xmm1
// Frames: F127 -> F128 -> F129 -> F133
// 0x1421abcbb  0f28d1                         movaps	%xmm1, %xmm2
// 0x1421abcbe  0f59d1                         mulps	%xmm1, %xmm2
// 0x1421abcc1  f30f16da                       movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
// 0x1421abcc5  f30f58da                       addss	%xmm2, %xmm3
// Frames: F131 -> F133
// 0x1421abcc9  31c0                           xorl	%eax, %eax
// 0x1421abccb  410f2ede                       ucomiss	%xmm14, %xmm3
// 0x1421abccf  0f97c0                         seta	%al
// 0x1421abcd2  0f57d2                         xorps	%xmm2, %xmm2
// 0x1421abcd5  f30f51d3                       sqrtss	%xmm3, %xmm2
// 0x1421abcd9  f30f12d2                       movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
// 0x1421abcdd  0f28d9                         movaps	%xmm1, %xmm3
// 0x1421abce0  0f5eda                         divps	%xmm2, %xmm3
// 0x1421abce3  660f6ed0                       movd	%eax, %xmm2
// 0x1421abce7  660f70d250                     pshufd	$0x50, %xmm2, %xmm2     # xmm2 = xmm2[0,0,1,1]
// 0x1421abcec  660f72f21f                     pslld	$0x1f, %xmm2
// 0x1421abcf1  660f72e21f                     psrad	$0x1f, %xmm2
// 0x1421abcf6  0f54da                         andps	%xmm2, %xmm3
// 0x1421abcf9  0f55d1                         andnps	%xmm1, %xmm2
// 0x1421abcfc  0f56d3                         orps	%xmm3, %xmm2
// 0x1421abcff  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F134
// 0x1421abd03  8904be                         movl	%eax, (%rsi,%rdi,4)
// Frames: F134
// 0x1421abd06  f2410f114724                   movsd	%xmm0, 0x24(%r15)
// Frames: F135 -> F136 -> F137
// 0x1421abd0c  0f5cc2                         subps	%xmm2, %xmm0
// Frames: F127 -> F138 -> F33
// 0x1421abd0f  0f59c0                         mulps	%xmm0, %xmm0
// 0x1421abd12  f30f16c8                       movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
// 0x1421abd16  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F139 -> F140 -> F138 -> F33
// 0x1421abd1a  450f57d2                       xorps	%xmm10, %xmm10
// 0x1421abd1e  f3440f51d1                     sqrtss	%xmm1, %xmm10
// Frames: F141
// 0x1421abd23  8b4318                         movl	0x18(%rbx), %eax
// 0x1421abd26  85c0                           testl	%eax, %eax
// 0x1421abd28  745f                           je	0x1421abd89 <.text+0x21aad89>
// 0x1421abd2a  83f802                         cmpl	$0x2, %eax
// 0x1421abd2d  7531                           jne	0x1421abd60 <.text+0x21aad60>
// Frames: F142
// 0x1421abd2f  f3440f10431c                   movss	0x1c(%rbx), %xmm8
// 0x1421abd35  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F143
// 0x1421abd39  8904be                         movl	%eax, (%rsi,%rdi,4)
// 0x1421abd3c  f3410f104738                   movss	0x38(%r15), %xmm0
// Frames: F143
// 0x1421abd42  f3410f58c0                     addss	%xmm8, %xmm0
// Frames: F104 -> F144
// 0x1421abd47  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421abd4a  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F106 -> F144
// 0x1421abd4e  410f28c6                       movaps	%xmm14, %xmm0
// 0x1421abd52  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F144
// 0x1421abd56  f3410f114738                   movss	%xmm0, 0x38(%r15)
// 0x1421abd5c  eb2b                           jmp	0x1421abd89 <.text+0x21aad89>
// 0x1421abd5e  6690                           nop
// Frames: F145
// 0x1421abd60  f3440f10431c                   movss	0x1c(%rbx), %xmm8
// Frames: F104 -> F146
// 0x1421abd66  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421abd69  f3410f5f4738                   maxss	0x38(%r15), %xmm0
// Frames: F106 -> F146
// 0x1421abd6f  410f28ce                       movaps	%xmm14, %xmm1
// 0x1421abd73  f30f5dc8                       minss	%xmm0, %xmm1
// 0x1421abd77  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F147
// 0x1421abd7b  8904be                         movl	%eax, (%rsi,%rdi,4)
// Frames: F147
// 0x1421abd7e  f3450f114738                   movss	%xmm8, 0x38(%r15)
// Frames: F148
// 0x1421abd84  f3440f5cc1                     subss	%xmm1, %xmm8
// Frames: F149
// 0x1421abd89  48898c2408010000               movq	%rcx, 0x108(%rsp)
// 0x1421abd91  4d89e5                         movq	%r12, %r13
// 0x1421abd94  4c89f9                         movq	%r15, %rcx
// 0x1421abd97  e8147673fe                     callq	0x1408e33b0 <.text+0x8e23b0>
// 0x1421abd9c  4189c4                         movl	%eax, %r12d
// Frames: F150
// 0x1421abd9f  8b4310                         movl	0x10(%rbx), %eax
// 0x1421abda2  85c0                           testl	%eax, %eax
// 0x1421abda4  744a                           je	0x1421abdf0 <.text+0x21aadf0>
// 0x1421abda6  83f802                         cmpl	$0x2, %eax
// 0x1421abda9  7555                           jne	0x1421abe00 <.text+0x21aae00>
// Frames: F151
// 0x1421abdab  f3440f104b14                   movss	0x14(%rbx), %xmm9
// 0x1421abdb1  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F152
// 0x1421abdb5  8904be                         movl	%eax, (%rsi,%rdi,4)
// 0x1421abdb8  f3410f104734                   movss	0x34(%r15), %xmm0
// Frames: F152
// 0x1421abdbe  f3410f58c1                     addss	%xmm9, %xmm0
// Frames: F104 -> F153
// 0x1421abdc3  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421abdc6  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F106 -> F153
// 0x1421abdca  410f28c6                       movaps	%xmm14, %xmm0
// 0x1421abdce  f30f5dc1                       minss	%xmm1, %xmm0
// Frames: F153
// 0x1421abdd2  f3410f114734                   movss	%xmm0, 0x34(%r15)
// Frames: F154
// 0x1421abdd8  4c89f9                         movq	%r15, %rcx
// 0x1421abddb  e8d07573fe                     callq	0x1408e33b0 <.text+0x8e23b0>
// 0x1421abde0  89c7                           movl	%eax, %edi
// 0x1421abde2  eb64                           jmp	0x1421abe48 <.text+0x21aae48>
// 0x1421abde4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F150
// 0x1421abdf0  450f57c9                       xorps	%xmm9, %xmm9
// 0x1421abdf4  4489e7                         movl	%r12d, %edi
// 0x1421abdf7  eb4f                           jmp	0x1421abe48 <.text+0x21aae48>
// 0x1421abdf9  0f1f8000000000                 nopl	(%rax)
// Frames: F155
// 0x1421abe00  450f28dc                       movaps	%xmm12, %xmm11
// 0x1421abe04  450f28e2                       movaps	%xmm10, %xmm12
// 0x1421abe08  f3440f104b14                   movss	0x14(%rbx), %xmm9
// Frames: F104 -> F156
// 0x1421abe0e  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421abe11  f3410f5f4734                   maxss	0x34(%r15), %xmm0
// Frames: F106 -> F156
// 0x1421abe17  450f28d6                       movaps	%xmm14, %xmm10
// 0x1421abe1b  f3440f5dd0                     minss	%xmm0, %xmm10
// 0x1421abe20  8b442428                       movl	0x28(%rsp), %eax
// Frames: F108 -> F109 -> F157
// 0x1421abe24  8904be                         movl	%eax, (%rsi,%rdi,4)
// Frames: F157
// 0x1421abe27  f3450f114f34                   movss	%xmm9, 0x34(%r15)
// Frames: F158
// 0x1421abe2d  4c89f9                         movq	%r15, %rcx
// 0x1421abe30  e87b7573fe                     callq	0x1408e33b0 <.text+0x8e23b0>
// 0x1421abe35  89c7                           movl	%eax, %edi
// Frames: F159
// 0x1421abe37  f3450f5cca                     subss	%xmm10, %xmm9
// 0x1421abe3c  450f28d4                       movaps	%xmm12, %xmm10
// 0x1421abe40  450f28e3                       movaps	%xmm11, %xmm12
// 0x1421abe44  450f57db                       xorps	%xmm11, %xmm11
// Frames: F160
// 0x1421abe48  410f2efb                       ucomiss	%xmm11, %xmm7
// 0x1421abe4c  0f95c1                         setne	%cl
// 0x1421abe4f  450f2ec3                       ucomiss	%xmm11, %xmm8
// 0x1421abe53  0f95c0                         setne	%al
// 0x1421abe56  4138fc                         cmpb	%dil, %r12b
// 0x1421abe59  7555                           jne	0x1421abeb0 <.text+0x21aaeb0>
// 0x1421abe5b  4008e9                         orb	%bpl, %cl
// 0x1421abe5e  4020f9                         andb	%dil, %cl
// 0x1421abe61  754d                           jne	0x1421abeb0 <.text+0x21aaeb0>
// 0x1421abe63  4020f8                         andb	%dil, %al
// 0x1421abe66  7548                           jne	0x1421abeb0 <.text+0x21aaeb0>
// Frames: F161
// 0x1421abe68  4584e4                         testb	%r12b, %r12b
// 0x1421abe6b  0f84ad000000                   je	0x1421abf1e <.text+0x21aaf1e>
// Frames: F162
// 0x1421abe71  4084ff                         testb	%dil, %dil
// 0x1421abe74  4d89ec                         movq	%r13, %r12
// 0x1421abe77  4c8bac24b8000000               movq	0xb8(%rsp), %r13
// 0x1421abe7f  488bac2488000000               movq	0x88(%rsp), %rbp
// 0x1421abe87  0f851a010000                   jne	0x1421abfa7 <.text+0x21aafa7>
// Frames: F163
// 0x1421abe8d  488d4c2450                     leaq	0x50(%rsp), %rcx
// 0x1421abe92  4c89fa                         movq	%r15, %rdx
// 0x1421abe95  e8867373fe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F164
// 0x1421abe9a  c64424400b                     movb	$0xb, 0x40(%rsp)
// 0x1421abe9f  e9a4000000                     jmp	0x1421abf48 <.text+0x21aaf48>
// 0x1421abea4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F165
// 0x1421abeb0  498b2f                         movq	(%r15), %rbp
// 0x1421abeb3  8b842494000000                 movl	0x94(%rsp), %eax
// 0x1421abeba  488b8c24f0000000               movq	0xf0(%rsp), %rcx
// Frames: F166 -> F167 -> F32 -> F165
// 0x1421abec2  8901                           movl	%eax, (%rcx)
// 0x1421abec4  488b8424b0000000               movq	0xb0(%rsp), %rax
// Frames: F168 -> F30 -> F169 -> F32 -> F165
// 0x1421abecc  488b7030                       movq	0x30(%rax), %rsi
// Frames: F170 -> F169 -> F32 -> F165
// 0x1421abed0  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F171 -> F30 -> F169 -> F32 -> F165
// 0x1421abed4  483b7020                       cmpq	0x20(%rax), %rsi
// 0x1421abed8  7514                           jne	0x1421abeee <.text+0x21aaeee>
// Frames: F172 -> F30 -> F169 -> F32 -> F165
// 0x1421abeda  488b8c24d8000000               movq	0xd8(%rsp), %rcx
// 0x1421abee2  488d15876e9c00                 leaq	0x9c6e87(%rip), %rdx    # 0x142b72d70
// 0x1421abee9  e842673bfe                     callq	0x140562630 <.text+0x561630>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F169 -> F32 -> F165
// 0x1421abeee  488b9424b0000000               movq	0xb0(%rsp), %rdx
// 0x1421abef6  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F169 -> F32 -> F165
// 0x1421abefa  4889f1                         movq	%rsi, %rcx
// 0x1421abefd  48c1e104                       shlq	$0x4, %rcx
// Frames: F35 -> F36 -> F30 -> F169 -> F32 -> F165
// 0x1421abf01  4c893408                       movq	%r14, (%rax,%rcx)
// 0x1421abf05  48896c0808                     movq	%rbp, 0x8(%rax,%rcx)
// Frames: F37 -> F30 -> F169 -> F32 -> F165
// 0x1421abf0a  48ffc6                         incq	%rsi
// 0x1421abf0d  48897230                       movq	%rsi, 0x30(%rdx)
// Frames: F38 -> F169 -> F32 -> F165
// 0x1421abf11  48ff4240                       incq	0x40(%rdx)
// Frames: F161
// 0x1421abf15  4584e4                         testb	%r12b, %r12b
// 0x1421abf18  0f8553ffffff                   jne	0x1421abe71 <.text+0x21aae71>
// 0x1421abf1e  4084ff                         testb	%dil, %dil
// 0x1421abf21  4d89ec                         movq	%r13, %r12
// 0x1421abf24  4c8bac24b8000000               movq	0xb8(%rsp), %r13
// 0x1421abf2c  488bac2488000000               movq	0x88(%rsp), %rbp
// 0x1421abf34  7471                           je	0x1421abfa7 <.text+0x21aafa7>
// Frames: F173
// 0x1421abf36  488d4c2450                     leaq	0x50(%rsp), %rcx
// 0x1421abf3b  4c89fa                         movq	%r15, %rdx
// 0x1421abf3e  e8dd7273fe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F174
// 0x1421abf43  c64424400a                     movb	$0xa, 0x40(%rsp)
// Frames: F166 -> F167 -> F32 -> F33
// 0x1421abf48  8b44242c                       movl	0x2c(%rsp), %eax
// 0x1421abf4c  488b4c2478                     movq	0x78(%rsp), %rcx
// 0x1421abf51  8901                           movl	%eax, (%rcx)
// 0x1421abf53  488b442430                     movq	0x30(%rsp), %rax
// Frames: F168 -> F30 -> F169 -> F32 -> F33
// 0x1421abf58  488b7030                       movq	0x30(%rax), %rsi
// Frames: F170 -> F169 -> F32 -> F33
// 0x1421abf5c  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F171 -> F30 -> F169 -> F32 -> F33
// 0x1421abf60  483b7020                       cmpq	0x20(%rax), %rsi
// 0x1421abf64  7511                           jne	0x1421abf77 <.text+0x21aaf77>
// Frames: F172 -> F30 -> F169 -> F32 -> F33
// 0x1421abf66  488b4c2470                     movq	0x70(%rsp), %rcx
// 0x1421abf6b  488d15fe6d9c00                 leaq	0x9c6dfe(%rip), %rdx    # 0x142b72d70
// 0x1421abf72  e8396b3bfe                     callq	0x140562ab0 <.text+0x561ab0>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F169 -> F32 -> F33
// 0x1421abf77  488b542430                     movq	0x30(%rsp), %rdx
// 0x1421abf7c  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F169 -> F32 -> F33
// 0x1421abf80  488d0cb6                       leaq	(%rsi,%rsi,4), %rcx
// Frames: F35 -> F36 -> F30 -> F169 -> F32 -> F33
// 0x1421abf84  4c8934c8                       movq	%r14, (%rax,%rcx,8)
// 0x1421abf88  0f10442440                     movups	0x40(%rsp), %xmm0
// 0x1421abf8d  0f104c244c                     movups	0x4c(%rsp), %xmm1
// 0x1421abf92  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1421abf97  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// Frames: F37 -> F30 -> F169 -> F32 -> F33
// 0x1421abf9c  48ffc6                         incq	%rsi
// 0x1421abf9f  48897230                       movq	%rsi, 0x30(%rdx)
// Frames: F38 -> F169 -> F32 -> F33
// 0x1421abfa3  48ff4240                       incq	0x40(%rdx)
// Frames: F175
// 0x1421abfa7  450f2ec3                       ucomiss	%xmm11, %xmm8
// 0x1421abfab  7523                           jne	0x1421abfd0 <.text+0x21aafd0>
// 0x1421abfad  410f2efb                       ucomiss	%xmm11, %xmm7
// 0x1421abfb1  751d                           jne	0x1421abfd0 <.text+0x21aafd0>
// 0x1421abfb3  410f2ef3                       ucomiss	%xmm11, %xmm6
// 0x1421abfb7  7517                           jne	0x1421abfd0 <.text+0x21aafd0>
// 0x1421abfb9  450f2ecb                       ucomiss	%xmm11, %xmm9
// 0x1421abfbd  7511                           jne	0x1421abfd0 <.text+0x21aafd0>
// Frames: F176
// 0x1421abfbf  450f2ed3                       ucomiss	%xmm11, %xmm10
// 0x1421abfc3  0f875c010000                   ja	0x1421ac125 <.text+0x21ab125>
// 0x1421abfc9  e9f4010000                     jmp	0x1421ac1c2 <.text+0x21ab1c2>
// 0x1421abfce  6690                           nop
// Frames: F177 -> F175
// 0x1421abfd0  f3440f11942498000000           movss	%xmm10, 0x98(%rsp)
// 0x1421abfda  f3440f11bc249c000000           movss	%xmm15, 0x9c(%rsp)
// 0x1421abfe4  f3440f11ac24a0000000           movss	%xmm13, 0xa0(%rsp)
// 0x1421abfee  450f28ee                       movaps	%xmm14, %xmm13
// 0x1421abff2  450f28f4                       movaps	%xmm12, %xmm14
// 0x1421abff6  440f283d82297800               movaps	0x782982(%rip), %xmm15  # 0x14292e980
// 0x1421abffe  410f54f7                       andps	%xmm15, %xmm6
// Frames: F178 -> F179
// 0x1421ac002  0f28c6                         movaps	%xmm6, %xmm0
// 0x1421ac005  e8567e7700                     callq	0x142923e60 <.text+0x2922e60>
// 0x1421ac00a  440f28d0                       movaps	%xmm0, %xmm10
// Frames: F177 -> F180
// 0x1421ac00e  410f54ff                       andps	%xmm15, %xmm7
// Frames: F178 -> F180
// 0x1421ac012  0f28c7                         movaps	%xmm7, %xmm0
// 0x1421ac015  e8467e7700                     callq	0x142923e60 <.text+0x2922e60>
// 0x1421ac01a  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F177 -> F180
// 0x1421ac01e  450f54cf                       andps	%xmm15, %xmm9
// Frames: F178 -> F180
// 0x1421ac022  410f28c1                       movaps	%xmm9, %xmm0
// 0x1421ac026  e8357e7700                     callq	0x142923e60 <.text+0x2922e60>
// 0x1421ac02b  440f28e0                       movaps	%xmm0, %xmm12
// Frames: F180
// 0x1421ac02f  f3450f58e3                     addss	%xmm11, %xmm12
// Frames: F177 -> F180
// 0x1421ac034  450f54c7                       andps	%xmm15, %xmm8
// Frames: F178 -> F180
// 0x1421ac038  410f28c0                       movaps	%xmm8, %xmm0
// 0x1421ac03c  e81f7e7700                     callq	0x142923e60 <.text+0x2922e60>
// 0x1421ac041  440f28d8                       movaps	%xmm0, %xmm11
// Frames: F180
// 0x1421ac045  f3450f58dc                     addss	%xmm12, %xmm11
// Frames: F181
// 0x1421ac04a  f3440f58cf                     addss	%xmm7, %xmm9
// 0x1421ac04f  f3450f58c8                     addss	%xmm8, %xmm9
// 0x1421ac054  0f57ff                         xorps	%xmm7, %xmm7
// 0x1421ac057  f3410fc2fa01                   cmpltss	%xmm10, %xmm7
// 0x1421ac05d  0f54f7                         andps	%xmm7, %xmm6
// 0x1421ac060  410f55f9                       andnps	%xmm9, %xmm7
// 0x1421ac064  0f56fe                         orps	%xmm6, %xmm7
// Frames: F182
// 0x1421ac067  488d4c2450                     leaq	0x50(%rsp), %rcx
// 0x1421ac06c  4c89fa                         movq	%r15, %rdx
// 0x1421ac06f  e8ac7173fe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F183
// 0x1421ac074  c644244002                     movb	$0x2, 0x40(%rsp)
// 0x1421ac079  40887c2441                     movb	%dil, 0x41(%rsp)
// 0x1421ac07e  f3440f11542444                 movss	%xmm10, 0x44(%rsp)
// 0x1421ac085  f3440f115c2448                 movss	%xmm11, 0x48(%rsp)
// 0x1421ac08c  f30f117c244c                   movss	%xmm7, 0x4c(%rsp)
// 0x1421ac092  8b44242c                       movl	0x2c(%rsp), %eax
// 0x1421ac096  488b4c2478                     movq	0x78(%rsp), %rcx
// Frames: F166 -> F167 -> F32 -> F183
// 0x1421ac09b  8901                           movl	%eax, (%rcx)
// 0x1421ac09d  488b442430                     movq	0x30(%rsp), %rax
// Frames: F168 -> F30 -> F169 -> F32 -> F183
// 0x1421ac0a2  488b7030                       movq	0x30(%rax), %rsi
// Frames: F170 -> F169 -> F32 -> F183
// 0x1421ac0a6  488b7840                       movq	0x40(%rax), %rdi
// Frames: F171 -> F30 -> F169 -> F32 -> F183
// 0x1421ac0aa  483b7020                       cmpq	0x20(%rax), %rsi
// 0x1421ac0ae  7511                           jne	0x1421ac0c1 <.text+0x21ab0c1>
// Frames: F172 -> F30 -> F169 -> F32 -> F183
// 0x1421ac0b0  488b4c2470                     movq	0x70(%rsp), %rcx
// 0x1421ac0b5  488d15b46c9c00                 leaq	0x9c6cb4(%rip), %rdx    # 0x142b72d70
// 0x1421ac0bc  e8ef693bfe                     callq	0x140562ab0 <.text+0x561ab0>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F169 -> F32 -> F183
// 0x1421ac0c1  488b542430                     movq	0x30(%rsp), %rdx
// 0x1421ac0c6  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F169 -> F32 -> F183
// 0x1421ac0ca  488d0cb6                       leaq	(%rsi,%rsi,4), %rcx
// Frames: F35 -> F36 -> F30 -> F169 -> F32 -> F183
// 0x1421ac0ce  48893cc8                       movq	%rdi, (%rax,%rcx,8)
// 0x1421ac0d2  0f10442440                     movups	0x40(%rsp), %xmm0
// 0x1421ac0d7  0f104c244c                     movups	0x4c(%rsp), %xmm1
// 0x1421ac0dc  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1421ac0e1  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// Frames: F37 -> F30 -> F169 -> F32 -> F183
// 0x1421ac0e6  48ffc6                         incq	%rsi
// 0x1421ac0e9  48897230                       movq	%rsi, 0x30(%rdx)
// Frames: F38 -> F169 -> F32 -> F183
// 0x1421ac0ed  48ff4240                       incq	0x40(%rdx)
// 0x1421ac0f1  450f57db                       xorps	%xmm11, %xmm11
// 0x1421ac0f5  450f28e6                       movaps	%xmm14, %xmm12
// 0x1421ac0f9  450f28f5                       movaps	%xmm13, %xmm14
// 0x1421ac0fd  f3440f10ac24a0000000           movss	0xa0(%rsp), %xmm13
// 0x1421ac107  f3440f10bc249c000000           movss	0x9c(%rsp), %xmm15
// 0x1421ac111  f3440f10942498000000           movss	0x98(%rsp), %xmm10
// Frames: F176
// 0x1421ac11b  450f2ed3                       ucomiss	%xmm11, %xmm10
// 0x1421ac11f  0f869d000000                   jbe	0x1421ac1c2 <.text+0x21ab1c2>
// Frames: F184
// 0x1421ac125  f3410f104724                   movss	0x24(%r15), %xmm0
// 0x1421ac12b  f3410f104f28                   movss	0x28(%r15), %xmm1
// Frames: F127 -> F138 -> F184
// 0x1421ac131  f30f59c0                       mulss	%xmm0, %xmm0
// 0x1421ac135  f30f59c9                       mulss	%xmm1, %xmm1
// 0x1421ac139  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F139 -> F140 -> F138 -> F184
// 0x1421ac13d  0f57f6                         xorps	%xmm6, %xmm6
// 0x1421ac140  f30f51f1                       sqrtss	%xmm1, %xmm6
// Frames: F185
// 0x1421ac144  488d4c2450                     leaq	0x50(%rsp), %rcx
// 0x1421ac149  4c89fa                         movq	%r15, %rdx
// 0x1421ac14c  e8cf7073fe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F186
// 0x1421ac151  c644244003                     movb	$0x3, 0x40(%rsp)
// 0x1421ac156  f30f11742444                   movss	%xmm6, 0x44(%rsp)
// 0x1421ac15c  f3440f11542448                 movss	%xmm10, 0x48(%rsp)
// 0x1421ac163  8b44242c                       movl	0x2c(%rsp), %eax
// 0x1421ac167  488b4c2478                     movq	0x78(%rsp), %rcx
// Frames: F166 -> F167 -> F32 -> F186
// 0x1421ac16c  8901                           movl	%eax, (%rcx)
// 0x1421ac16e  488b442430                     movq	0x30(%rsp), %rax
// Frames: F168 -> F30 -> F169 -> F32 -> F186
// 0x1421ac173  488b7030                       movq	0x30(%rax), %rsi
// Frames: F170 -> F169 -> F32 -> F186
// 0x1421ac177  488b7840                       movq	0x40(%rax), %rdi
// Frames: F171 -> F30 -> F169 -> F32 -> F186
// 0x1421ac17b  483b7020                       cmpq	0x20(%rax), %rsi
// 0x1421ac17f  7511                           jne	0x1421ac192 <.text+0x21ab192>
// Frames: F172 -> F30 -> F169 -> F32 -> F186
// 0x1421ac181  488b4c2470                     movq	0x70(%rsp), %rcx
// 0x1421ac186  488d15e36b9c00                 leaq	0x9c6be3(%rip), %rdx    # 0x142b72d70
// 0x1421ac18d  e81e693bfe                     callq	0x140562ab0 <.text+0x561ab0>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F169 -> F32 -> F186
// 0x1421ac192  488b542430                     movq	0x30(%rsp), %rdx
// 0x1421ac197  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F169 -> F32 -> F186
// 0x1421ac19b  488d0cb6                       leaq	(%rsi,%rsi,4), %rcx
// Frames: F35 -> F36 -> F30 -> F169 -> F32 -> F186
// 0x1421ac19f  48893cc8                       movq	%rdi, (%rax,%rcx,8)
// 0x1421ac1a3  0f10442440                     movups	0x40(%rsp), %xmm0
// 0x1421ac1a8  0f104c244c                     movups	0x4c(%rsp), %xmm1
// 0x1421ac1ad  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1421ac1b2  0f1144c808                     movups	%xmm0, 0x8(%rax,%rcx,8)
// Frames: F37 -> F30 -> F169 -> F32 -> F186
// 0x1421ac1b7  48ffc6                         incq	%rsi
// 0x1421ac1ba  48897230                       movq	%rsi, 0x30(%rdx)
// Frames: F38 -> F169 -> F32 -> F186
// 0x1421ac1be  48ff4240                       incq	0x40(%rdx)
// Frames: F187
// 0x1421ac1c2  488b842428010000               movq	0x128(%rsp), %rax
// 0x1421ac1ca  488b38                         movq	(%rax), %rdi
// 0x1421ac1cd  8b8424a4000000                 movl	0xa4(%rsp), %eax
// 0x1421ac1d4  488b8c2410010000               movq	0x110(%rsp), %rcx
// Frames: F166 -> F167 -> F32 -> F188
// 0x1421ac1dc  8901                           movl	%eax, (%rcx)
// 0x1421ac1de  488b8424c0000000               movq	0xc0(%rsp), %rax
// Frames: F168 -> F30 -> F169 -> F32 -> F188
// 0x1421ac1e6  488b7030                       movq	0x30(%rax), %rsi
// Frames: F170 -> F169 -> F32 -> F188
// 0x1421ac1ea  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F171 -> F30 -> F169 -> F32 -> F188
// 0x1421ac1ee  483b7020                       cmpq	0x20(%rax), %rsi
// 0x1421ac1f2  7514                           jne	0x1421ac208 <.text+0x21ab208>
// Frames: F172 -> F30 -> F169 -> F32 -> F188
// 0x1421ac1f4  488b8c24e8000000               movq	0xe8(%rsp), %rcx
// 0x1421ac1fc  488d156d6b9c00                 leaq	0x9c6b6d(%rip), %rdx    # 0x142b72d70
// 0x1421ac203  e8286a3bfe                     callq	0x140562c30 <.text+0x561c30>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F169 -> F32 -> F188
// 0x1421ac208  488b9424c0000000               movq	0xc0(%rsp), %rdx
// 0x1421ac210  488b4228                       movq	0x28(%rdx), %rax
// Frames: F34 -> F29 -> F30 -> F169 -> F32 -> F188
// 0x1421ac214  4889f1                         movq	%rsi, %rcx
// 0x1421ac217  48c1e105                       shlq	$0x5, %rcx
// Frames: F35 -> F36 -> F30 -> F169 -> F32 -> F188
// 0x1421ac21b  4c893408                       movq	%r14, (%rax,%rcx)
// 0x1421ac21f  48896c0808                     movq	%rbp, 0x8(%rax,%rcx)
// 0x1421ac224  48897c0810                     movq	%rdi, 0x10(%rax,%rcx)
// 0x1421ac229  c644081801                     movb	$0x1, 0x18(%rax,%rcx)
// Frames: F37 -> F30 -> F169 -> F32 -> F188
// 0x1421ac22e  48ffc6                         incq	%rsi
// 0x1421ac231  48897230                       movq	%rsi, 0x30(%rdx)
// Frames: F38 -> F169 -> F32 -> F188
// 0x1421ac235  48ff4240                       incq	0x40(%rdx)
// Frames: F189
// 0x1421ac239  450f2eec                       ucomiss	%xmm12, %xmm13
// 0x1421ac23d  7641                           jbe	0x1421ac280 <.text+0x21ab280>
// 0x1421ac23f  450f2ee7                       ucomiss	%xmm15, %xmm12
// 0x1421ac243  723b                           jb	0x1421ac280 <.text+0x21ab280>
// Frames: F190
// 0x1421ac245  488b7b20                       movq	0x20(%rbx), %rdi
// Frames: F191
// 0x1421ac249  0fb65b34                       movzbl	0x34(%rbx), %ebx
// 0x1421ac24d  8b44243c                       movl	0x3c(%rsp), %eax
// 0x1421ac251  488b8c24a8000000               movq	0xa8(%rsp), %rcx
// Frames: F166 -> F167 -> F32 -> F192
// 0x1421ac259  8901                           movl	%eax, (%rcx)
// 0x1421ac25b  488b8c2480000000               movq	0x80(%rsp), %rcx
// Frames: F168 -> F30 -> F169 -> F32 -> F192
// 0x1421ac263  488b7130                       movq	0x30(%rcx), %rsi
// Frames: F170 -> F169 -> F32 -> F192
// 0x1421ac267  4c8b7940                       movq	0x40(%rcx), %r15
// 0x1421ac26b  31ed                           xorl	%ebp, %ebp
// 0x1421ac26d  b800000000                     movl	$0x0, %eax
// Frames: F171 -> F30 -> F31 -> F32 -> F33
// 0x1421ac272  483b7120                       cmpq	0x20(%rcx), %rsi
// 0x1421ac276  0f8534f7ffff                   jne	0x1421ab9b0 <.text+0x21aa9b0>
// 0x1421ac27c  eb67                           jmp	0x1421ac2e5 <.text+0x21ab2e5>
// 0x1421ac27e  6690                           nop
// Frames: F193
// 0x1421ac280  450f2ee5                       ucomiss	%xmm13, %xmm12
// 0x1421ac284  488b8c24d0000000               movq	0xd0(%rsp), %rcx
// 0x1421ac28c  4889c8                         movq	%rcx, %rax
// 0x1421ac28f  488b9424c8000000               movq	0xc8(%rsp), %rdx
// 0x1421ac297  4989d0                         movq	%rdx, %r8
// 0x1421ac29a  0f825cf7ffff                   jb	0x1421ab9fc <.text+0x21aa9fc>
// 0x1421ac2a0  450f2efc                       ucomiss	%xmm12, %xmm15
// 0x1421ac2a4  4889c8                         movq	%rcx, %rax
// 0x1421ac2a7  4989d0                         movq	%rdx, %r8
// 0x1421ac2aa  0f864cf7ffff                   jbe	0x1421ab9fc <.text+0x21aa9fc>
// Frames: F194
// 0x1421ac2b0  488b7b20                       movq	0x20(%rbx), %rdi
// Frames: F195
// 0x1421ac2b4  0fb65b34                       movzbl	0x34(%rbx), %ebx
// 0x1421ac2b8  8b44243c                       movl	0x3c(%rsp), %eax
// 0x1421ac2bc  488b8c24a8000000               movq	0xa8(%rsp), %rcx
// Frames: F166 -> F167 -> F32 -> F196
// 0x1421ac2c4  8901                           movl	%eax, (%rcx)
// 0x1421ac2c6  488b8c2480000000               movq	0x80(%rsp), %rcx
// Frames: F168 -> F30 -> F169 -> F32 -> F196
// 0x1421ac2ce  488b7130                       movq	0x30(%rcx), %rsi
// Frames: F170 -> F169 -> F32 -> F196
// 0x1421ac2d2  4c8b7940                       movq	0x40(%rcx), %r15
// 0x1421ac2d6  40b501                         movb	$0x1, %bpl
// 0x1421ac2d9  b001                           movb	$0x1, %al
// Frames: F171 -> F30 -> F31 -> F32 -> F33
// 0x1421ac2db  483b7120                       cmpq	0x20(%rcx), %rsi
// 0x1421ac2df  0f85cbf6ffff                   jne	0x1421ab9b0 <.text+0x21aa9b0>
// Frames: F172 -> F30 -> F31 -> F32 -> F33
// 0x1421ac2e5  488b8c24e0000000               movq	0xe0(%rsp), %rcx
// 0x1421ac2ed  488d157c6a9c00                 leaq	0x9c6a7c(%rip), %rdx    # 0x142b72d70
// 0x1421ac2f4  e837693bfe                     callq	0x140562c30 <.text+0x561c30>
// 0x1421ac2f9  89e8                           movl	%ebp, %eax
// 0x1421ac2fb  e9b0f6ffff                     jmp	0x1421ab9b0 <.text+0x21aa9b0>
// Frames: F197
// 0x1421ac300  0f28b42440010000               movaps	0x140(%rsp), %xmm6
// 0x1421ac308  0f28bc2450010000               movaps	0x150(%rsp), %xmm7
// 0x1421ac310  440f28842460010000             movaps	0x160(%rsp), %xmm8
// 0x1421ac319  440f288c2470010000             movaps	0x170(%rsp), %xmm9
// 0x1421ac322  440f28942480010000             movaps	0x180(%rsp), %xmm10
// 0x1421ac32b  440f289c2490010000             movaps	0x190(%rsp), %xmm11
// 0x1421ac334  440f28a424a0010000             movaps	0x1a0(%rsp), %xmm12
// 0x1421ac33d  440f28ac24b0010000             movaps	0x1b0(%rsp), %xmm13
// 0x1421ac346  440f28b424c0010000             movaps	0x1c0(%rsp), %xmm14
// 0x1421ac34f  440f28bc24d0010000             movaps	0x1d0(%rsp), %xmm15
// 0x1421ac358  4881c4e8010000                 addq	$0x1e8, %rsp            # imm = 0x1E8
// 0x1421ac35f  5b                             popq	%rbx
// 0x1421ac360  5d                             popq	%rbp
// 0x1421ac361  5f                             popq	%rdi
// 0x1421ac362  5e                             popq	%rsi
// 0x1421ac363  415c                           popq	%r12
// 0x1421ac365  415d                           popq	%r13
// 0x1421ac367  415e                           popq	%r14
// 0x1421ac369  415f                           popq	%r15
// 0x1421ac36b  c3                             retq
// Frames: F198 -> F55 -> F56 -> F57
// 0x1421ac36c  48ba00000000ffffffff           movabsq	$-0x100000000, %rdx # imm = 0xFFFFFFFF00000000
// 0x1421ac376  4c8b842408010000               movq	0x108(%rsp), %r8
// 0x1421ac37e  4921d0                         andq	%rdx, %r8
// 0x1421ac381  4c89c2                         movq	%r8, %rdx
// 0x1421ac384  48ffc2                         incq	%rdx
// 0x1421ac387  48c1e020                       shlq	$0x20, %rax
// 0x1421ac38b  4809c8                         orq	%rcx, %rax
// 0x1421ac38e  4989c2                         movq	%rax, %r10
// 0x1421ac391  eb04                           jmp	0x1421ac397 <.text+0x21ab397>
// Frames: F199 -> F56 -> F57
// 0x1421ac393  48c1e220                       shlq	$0x20, %rdx
// Frames: F200 -> F57
// 0x1421ac397  4889542440                     movq	%rdx, 0x40(%rsp)
// 0x1421ac39c  4c89542448                     movq	%r10, 0x48(%rsp)
// 0x1421ac3a1  488d0588b2e400                 leaq	0xe4b288(%rip), %rax    # 0x142ff7630
// 0x1421ac3a8  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421ac3ad  488d0d3cb1e400                 leaq	0xe4b13c(%rip), %rcx    # 0x142ff74f0
// 0x1421ac3b4  4c8d0d15b1e400                 leaq	0xe4b115(%rip), %r9     # 0x142ff74d0
// 0x1421ac3bb  4c8d442440                     leaq	0x40(%rsp), %r8
// 0x1421ac3c0  ba2b000000                     movl	$0x2b, %edx
// 0x1421ac3c5  e896d37200                     callq	0x1428d9760 <.text+0x28d8760>
// Frames: F201 -> F202 -> F52
// 0x1421ac3ca  4c89b42430010000               movq	%r14, 0x130(%rsp)
// 0x1421ac3d2  488d05e7f2ffff                 leaq	-0xd19(%rip), %rax      # 0x1421ab6c0 <.text+0x21aa6c0>
// 0x1421ac3d9  4889842438010000               movq	%rax, 0x138(%rsp)
// Frames: F203 -> F201 -> F202 -> F52
// 0x1421ac3e1  488d0580b2e400                 leaq	0xe4b280(%rip), %rax    # 0x142ff7668
// 0x1421ac3e8  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1421ac3ed  48c744244801000000             movq	$0x1, 0x48(%rsp)
// 0x1421ac3f6  48c744246000000000             movq	$0x0, 0x60(%rsp)
// 0x1421ac3ff  488d842430010000               leaq	0x130(%rsp), %rax
// 0x1421ac407  4889442450                     movq	%rax, 0x50(%rsp)
// 0x1421ac40c  48c744245801000000             movq	$0x1, 0x58(%rsp)
// Frames: F201 -> F202 -> F52
// 0x1421ac415  488d155cb2e400                 leaq	0xe4b25c(%rip), %rdx    # 0x142ff7678
// 0x1421ac41c  488d4c2440                     leaq	0x40(%rsp), %rcx
// 0x1421ac421  e86acf7200                     callq	0x1428d9390 <.text+0x28d8390>
// 0x1421ac426  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/edit_roof.rs:10:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1235:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs:28:0 bevy_ecs::event::reader::EventReader::read
// F4: crates/systems/roof/src/edit_roof.rs:23:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
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
// F33: crates/systems/roof/src/edit_roof.rs:0:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
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
// F51: crates/systems/roof/src/edit_roof.rs:25:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F52: crates/systems/roof/src/edit_roof.rs:24:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F53: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1021:0 core::option::Option::unwrap_or_else
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:843:0 bevy_ecs::entity::Entities::get
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1595:0 bevy_ecs::system::query::Query::get_inner
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1577:0 bevy_ecs::system::query::Query::get_mut
// F57: crates/systems/roof/src/edit_roof.rs:28:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F58: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F59: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F60: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 core::slice::impl$0::get
// F61: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F62: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1742:0 core::cmp::impls::impl$24::ne
// F63: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:201:0 core::num::nonzero::impl$2::ne
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:844:0 bevy_ecs::entity::Entities::get
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:76:0 bevy_ecs::archetype::impl$25::eq
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:845:0 bevy_ecs::entity::Entities::get
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1603:0 bevy_ecs::system::query::Query::get_inner
// F69: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/bool.rs:62:0 core::bool::impl$0::then
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F74: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/internal_macros.rs:36:0 fixedbitset::FixedBitSet::get_unchecked
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1600:0 bevy_ecs::system::query::Query::get_inner
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1632:0 bevy_ecs::system::query::Query::get_inner
// F78: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 alloc::vec::impl$8::deref
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1634:0 bevy_ecs::system::query::Query::get_inner
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1544:0 bevy_ecs::query::fetch::impl$36::set_table
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1533:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F84: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F85: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F86: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::option::Option::and_then
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F90: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F91: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:256:0 core::slice::index::impl$2::get_unchecked
// F92: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 nonmax::NonMaxUsize::get
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1608:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1601:0 bevy_ecs::query::fetch::impl$37::fetch
// F97: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1638:0 bevy_ecs::system::query::Query::get_inner
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs:305:0 bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
// F99: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:320:0 bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1548:0 bevy_ecs::query::fetch::impl$36::set_table
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1610:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F102: crates/systems/roof/src/edit_roof.rs:30:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F103: crates/systems/roof/src/edit_roof.rs:32:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F104: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1418:0 core::f32::impl$0::clamp
// F105: crates/systems/roof/src/edit_roof.rs:34:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F106: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1421:0 core::f32::impl$0::clamp
// F107: crates/systems/roof/src/edit_roof.rs:35:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F110: crates/systems/roof/src/edit_roof.rs:36:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F111: crates/systems/roof/src/edit_roof.rs:37:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F112: crates/systems/roof/src/edit_roof.rs:38:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F113: crates/systems/roof/src/edit_roof.rs:39:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F114: crates/systems/roof/src/edit_roof.rs:46:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F115: crates/systems/roof/src/edit_roof.rs:47:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F116: crates/systems/roof/src/edit_roof.rs:55:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F117: crates/systems/roof/src/edit_roof.rs:57:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F118: crates/systems/roof/src/edit_roof.rs:58:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F119: crates/systems/roof/src/edit_roof.rs:49:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F120: crates/systems/roof/src/edit_roof.rs:52:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F121: crates/systems/roof/src/edit_roof.rs:53:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F122: crates/systems/roof/src/edit_roof.rs:64:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F123: crates/systems/roof/src/edit_roof.rs:71:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F124: crates/systems/roof/src/edit_roof.rs:72:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F125: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:761:0 core::ops::arith::impl$353::add_assign
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1318:0 glam::f32::vec2::impl$38::add_assign
// F127: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:167:0 glam::f32::vec2::Vec2::dot
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:407:0 glam::f32::vec2::Vec2::length_squared
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:766:0 glam::f32::vec2::Vec2::clamp_length_max
// F130: crates/systems/roof/src/edit_roof.rs:73:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:767:0 glam::f32::vec2::Vec2::clamp_length_max
// F132: crates/systems/roof/src/edit_roof.rs:66:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F133: crates/systems/roof/src/edit_roof.rs:67:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F134: crates/systems/roof/src/edit_roof.rs:68:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F135: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$77::sub
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:1420:0 glam::f32::vec2::impl$50::sub
// F137: crates/systems/roof/src/edit_roof.rs:69:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs:397:0 glam::f32::vec2::Vec2::length
// F139: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:370:0 std::f32::impl$0::sqrt
// F140: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs:178:0 glam::f32::vec2::Vec2::dot
// F141: crates/systems/roof/src/edit_roof.rs:78:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F142: crates/systems/roof/src/edit_roof.rs:85:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F143: crates/systems/roof/src/edit_roof.rs:86:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F144: crates/systems/roof/src/edit_roof.rs:87:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F145: crates/systems/roof/src/edit_roof.rs:80:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F146: crates/systems/roof/src/edit_roof.rs:81:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F147: crates/systems/roof/src/edit_roof.rs:82:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F148: crates/systems/roof/src/edit_roof.rs:83:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F149: crates/systems/roof/src/edit_roof.rs:92:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F150: crates/systems/roof/src/edit_roof.rs:94:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F151: crates/systems/roof/src/edit_roof.rs:102:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F152: crates/systems/roof/src/edit_roof.rs:103:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F153: crates/systems/roof/src/edit_roof.rs:104:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F154: crates/systems/roof/src/edit_roof.rs:105:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F155: crates/systems/roof/src/edit_roof.rs:96:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F156: crates/systems/roof/src/edit_roof.rs:97:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F157: crates/systems/roof/src/edit_roof.rs:98:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F158: crates/systems/roof/src/edit_roof.rs:99:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F159: crates/systems/roof/src/edit_roof.rs:100:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F160: crates/systems/roof/src/edit_roof.rs:111:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F161: crates/systems/roof/src/edit_roof.rs:119:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F162: crates/systems/roof/src/edit_roof.rs:126:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F163: crates/systems/roof/src/edit_roof.rs:129:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F164: crates/systems/roof/src/edit_roof.rs:127:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F165: crates/systems/roof/src/edit_roof.rs:116:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F166: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F168: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2419:0 alloc::vec::Vec::push
// F169: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::change_detection::impl$38::deref_mut
// F170: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:130:0 bevy_ecs::event::collections::Events::send_with_caller
// F171: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2422:0 alloc::vec::Vec::push
// F172: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2423:0 alloc::vec::Vec::push
// F173: crates/systems/roof/src/edit_roof.rs:122:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F174: crates/systems/roof/src/edit_roof.rs:120:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F175: crates/systems/roof/src/edit_roof.rs:133:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F176: crates/systems/roof/src/edit_roof.rs:156:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F177: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1448:0 core::f32::impl$0::abs
// F178: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:71:0 std::f32::impl$0::ceil
// F179: crates/systems/roof/src/edit_roof.rs:139:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F180: crates/systems/roof/src/edit_roof.rs:140:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F181: crates/systems/roof/src/edit_roof.rs:145:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F182: crates/systems/roof/src/edit_roof.rs:152:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F183: crates/systems/roof/src/edit_roof.rs:141:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F184: crates/systems/roof/src/edit_roof.rs:159:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F185: crates/systems/roof/src/edit_roof.rs:162:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F186: crates/systems/roof/src/edit_roof.rs:157:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F187: crates/systems/roof/src/edit_roof.rs:169:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F188: crates/systems/roof/src/edit_roof.rs:166:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F189: crates/systems/roof/src/edit_roof.rs:173:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F190: crates/systems/roof/src/edit_roof.rs:177:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F191: crates/systems/roof/src/edit_roof.rs:178:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F192: crates/systems/roof/src/edit_roof.rs:174:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F193: crates/systems/roof/src/edit_roof.rs:180:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F194: crates/systems/roof/src/edit_roof.rs:184:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F195: crates/systems/roof/src/edit_roof.rs:185:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F196: crates/systems/roof/src/edit_roof.rs:181:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F197: crates/systems/roof/src/edit_roof.rs:189:0 system_roof::edit_roof::edit_roof::h4623f327534b9fd4
// F198: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:2014:0 core::result::impl$27::from_residual
// F199: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1605:0 bevy_ecs::system::query::Query::get_inner
// F200: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1109:0 core::result::Result::unwrap
// F201: crates/systems/roof/src/edit_roof.rs:26:0 system_roof::edit_roof::edit_roof::closure$0
// F202: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1023:0 core::option::Option::unwrap_or_else
// F203: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/mod.rs:613:0 core::fmt::Arguments::new_v1
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/edit_roof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 85d50f72e7226eefc3e1d7de595aafe2630b361aab262a9b03bdf91c02d7c8ac
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: edit_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3468
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
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::edit_roof::edit_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3468,"record_offset":492,"rva":35305408,"va":5404014528}
// 编译实例 RVAs: 0x21ab7c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: edit_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7831126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404014528,"raw_hex":"e8567e7700","size":5,"soft_fail":false,"target_va":5411847776,"va":5404016645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-29661374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404014528,"raw_hex":"e842673bfe","size":5,"soft_fail":false,"target_va":5374354992,"va":5404016361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7524202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404014528,"raw_hex":"e86acf7200","size":5,"soft_fail":false,"target_va":5411541904,"va":5404017697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7525270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404014528,"raw_hex":"e896d37200","size":5,"soft_fail":false,"target_va":5411542880,"va":5404017605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25987564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404014528,"raw_hex":"e8147673fe","size":5,"soft_fail":false,"target_va":5378028464,"va":5404016023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25988218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404014528,"raw_hex":"e8867373fe","size":5,"soft_fail":false,"target_va":5378028064,"va":5404016277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24221616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404014528,"raw_hex":"e850688efe","size":5,"soft_fail":false,"target_va":5379793584,"va":5404015195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3468,"record_offset":4028,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3468,"record_offset":1980,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3468,"record_offset":664,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3468,"record_offset":1364,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3468,"record_offset":2412,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3468,"record_offset":5516,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3468,"record_offset":3996,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":2320,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3468,"record_offset":7112,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3468,"record_offset":4460,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":28492,"source_line":764,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3468,"record_offset":4748,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3468,"record_offset":4404,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3468,"record_offset":4692,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3468,"record_offset":5544,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3468,"record_offset":1552,"parent_record_offset":1520,"ancestor_record_offset":492,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3468,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":492,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3468,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":492,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":1520,"parent_record_offset":1476,"ancestor_record_offset":492,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":492,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":492,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":1476,"parent_record_offset":1436,"ancestor_record_offset":492,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":984,"parent_record_offset":752,"ancestor_record_offset":492,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3468,"record_offset":2840,"parent_record_offset":2508,"ancestor_record_offset":492,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3468,"record_offset":1712,"parent_record_offset":1680,"ancestor_record_offset":492,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3468,"record_offset":1680,"parent_record_offset":1364,"ancestor_record_offset":492,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3468,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":492,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3468,"record_offset":2508,"parent_record_offset":2444,"ancestor_record_offset":492,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3468,"record_offset":1404,"parent_record_offset":1364,"ancestor_record_offset":492,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3468,"record_offset":1436,"parent_record_offset":1404,"ancestor_record_offset":492,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3468,"record_offset":728,"parent_record_offset":688,"ancestor_record_offset":492,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3468,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":492,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3468,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":492,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3468,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":492,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3468,"record_offset":3312,"parent_record_offset":2444,"ancestor_record_offset":492,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3468,"record_offset":3340,"parent_record_offset":3312,"ancestor_record_offset":492,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3468,"record_offset":3752,"parent_record_offset":2444,"ancestor_record_offset":492,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3468,"record_offset":3808,"parent_record_offset":3780,"ancestor_record_offset":492,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3468,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":492,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3468,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":492,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3468,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":492,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3468,"record_offset":3372,"parent_record_offset":3340,"ancestor_record_offset":492,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3468,"record_offset":3052,"parent_record_offset":2444,"ancestor_record_offset":492,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3468,"record_offset":3688,"parent_record_offset":3340,"ancestor_record_offset":492,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3468,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":492,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3468,"record_offset":2444,"parent_record_offset":2412,"ancestor_record_offset":492,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3468,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":492,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3468,"record_offset":3836,"parent_record_offset":3808,"ancestor_record_offset":492,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3468,"record_offset":2900,"parent_record_offset":2872,"ancestor_record_offset":492,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3468,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":492,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3468,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":492,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3468,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":492,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3468,"record_offset":2032,"parent_record_offset":2004,"ancestor_record_offset":492,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3468,"record_offset":2136,"parent_record_offset":2112,"ancestor_record_offset":492,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3468,"record_offset":940,"parent_record_offset":912,"ancestor_record_offset":492,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3468,"record_offset":800,"parent_record_offset":752,"ancestor_record_offset":492,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3468,"record_offset":2784,"parent_record_offset":2508,"ancestor_record_offset":492,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3468,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":492,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3468,"record_offset":4716,"parent_record_offset":4692,"ancestor_record_offset":492,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3468,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":492,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":492,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":492,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":492,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":3532,"parent_record_offset":3396,"ancestor_record_offset":492,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":2112,"parent_record_offset":2032,"ancestor_record_offset":492,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3468,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":492,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3468,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":492,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3468,"record_offset":1640,"parent_record_offset":1476,"ancestor_record_offset":492,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3468,"record_offset":3960,"parent_record_offset":2444,"ancestor_record_offset":492,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3468,"record_offset":856,"parent_record_offset":752,"ancestor_record_offset":492,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3468,"record_offset":1208,"parent_record_offset":752,"ancestor_record_offset":492,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3468,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":492,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3468,"record_offset":2580,"parent_record_offset":2540,"ancestor_record_offset":492,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3468,"record_offset":3612,"parent_record_offset":3580,"ancestor_record_offset":492,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3468,"record_offset":912,"parent_record_offset":884,"ancestor_record_offset":492,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3468,"record_offset":884,"parent_record_offset":856,"ancestor_record_offset":492,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3468,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":492,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3468,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":492,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3468,"record_offset":2188,"parent_record_offset":2136,"ancestor_record_offset":492,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3468,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":492,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3468,"record_offset":2872,"parent_record_offset":2444,"ancestor_record_offset":492,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3468,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":492,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3468,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":492,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3468,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":492,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3468,"record_offset":4512,"parent_record_offset":4488,"ancestor_record_offset":492,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3468,"record_offset":4488,"parent_record_offset":4460,"ancestor_record_offset":492,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3468,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":492,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3468,"record_offset":4804,"parent_record_offset":4772,"ancestor_record_offset":492,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::edit_roof::edit_roof::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/edit_roof.rs
//     evidence={"module":3468,"record_offset":2352,"parent_record_offset":2320,"ancestor_record_offset":492,"inlinee":116710,"source_line":26,"archive":"_pdb/B/modules/03468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::edit_roof::edit_roof::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C7E6
// 首条位置: {"module":3468,"inlinee":116710}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3468,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":2352,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03468.jsonl.gz"}
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
// 完整结构及原始类型记录: edit_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: edit_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: edit_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>

// ------------------------------------------------------------------
// 函数: system_roof::edit_roof::edit_roof
// 信息 ID: 2b019c5c60ab259989d35ca1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 23b41926ba248e49c4540b0b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > > @ 0x1a2c96f -> 0x21ab7c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > @ 0x1a344d6 -> 0x21ab7c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::edit_roof::edit_roof::closure$0
// 信息 ID: 65ad634b9e6ec87886146983
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::edit_roof::edit_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::edit_roof::edit_roof; module=3468, inline_record=2352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
