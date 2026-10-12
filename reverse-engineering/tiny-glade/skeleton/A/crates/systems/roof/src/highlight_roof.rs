// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: highlight_roof.rs.assembly.json.gz

// FUNCTION system_roof::highlight_roof::highlight_roof
// Candidate interfaces: 73f1a71d1d8226dabdf477ee
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 1927:287852 -> host 1927:283916 / rva_12d8260_len_456; exact_symbolizer_function_frame
//   Function-frame RVA span [0x12d83c3, 0x12d84ce), 67 instructions
// Inline site 1939:4580 -> host 1939:1216 / rva_12e04d0_len_37c; exact_symbolizer_function_frame
//   Function-frame RVA span [0x12e05de, 0x12e06ee), 67 instructions
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_12d8260_len_456: RVA 0x12d8260, 1110 PDB bytes; 233 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_12d8260_len_456.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 1927:283916: bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)> >
// Frames: F1
// 0x1412d8260  4157                           pushq	%r15
// 0x1412d8262  4156                           pushq	%r14
// 0x1412d8264  4155                           pushq	%r13
// 0x1412d8266  4154                           pushq	%r12
// 0x1412d8268  56                             pushq	%rsi
// 0x1412d8269  57                             pushq	%rdi
// 0x1412d826a  55                             pushq	%rbp
// 0x1412d826b  53                             pushq	%rbx
// 0x1412d826c  4881ec98000000                 subq	$0x98, %rsp
// 0x1412d8273  4889cf                         movq	%rcx, %rdi
// Frames: F2 -> F3 -> F4 -> F5
// 0x1412d8276  488b8240040000                 movq	0x440(%rdx), %rax
// Frames: F4 -> F5
// 0x1412d827d  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1412d8282  48c744242801000000             movq	$0x1, 0x28(%rsp)
// Frames: F6 -> F7 -> F8 -> F5
// 0x1412d828b  4883792800                     cmpq	$0x0, 0x28(%rcx)
// 0x1412d8290  0f8487030000                   je	0x1412d861d <.text+0x12d761d>
// 0x1412d8296  48394730                       cmpq	%rax, 0x30(%rdi)
// 0x1412d829a  0f857d030000                   jne	0x1412d861d <.text+0x12d761d>
// Frames: F9 -> F10 -> F11 -> F5
// 0x1412d82a0  488b82a8000000                 movq	0xa8(%rdx), %rax
// Frames: F12 -> F13 -> F11 -> F5
// 0x1412d82a7  8b8f10010000                   movl	0x110(%rdi), %ecx
// Frames: F14 -> F15 -> F11 -> F5
// 0x1412d82ad  898710010000                   movl	%eax, 0x110(%rdi)
// Frames: F16 -> F17 -> F18 -> F19 -> F20 -> F5
// 0x1412d82b3  4839c8                         cmpq	%rcx, %rax
// 0x1412d82b6  0f82ae030000                   jb	0x1412d866a <.text+0x12d766a>
// Frames: F21 -> F20 -> F5
// 0x1412d82bc  740a                           je	0x1412d82c8 <.text+0x12d72c8>
// 0x1412d82be  48833f00                       cmpq	$0x0, (%rdi)
// 0x1412d82c2  0f84b1030000                   je	0x1412d8679 <.text+0x12d7679>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27
// 0x1412d82c8  bb01000000                     movl	$0x1, %ebx
// 0x1412d82cd  f0                             lock
// 0x1412d82ce  0fc19a48040000                 xaddl	%ebx, 0x448(%rdx)
// Frames: F28 -> F29 -> F27
// 0x1412d82d5  48833f00                       cmpq	$0x0, (%rdi)
// 0x1412d82d9  0f84a6030000                   je	0x1412d8685 <.text+0x12d7685>
// Frames: F30 -> F31 -> F27
// 0x1412d82df  488b4718                       movq	0x18(%rdi), %rax
// Frames: F32 -> F33 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d82e3  48398298010000                 cmpq	%rax, 0x198(%rdx)
// Frames: F39 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d82ea  0f8602020000                   jbe	0x1412d84f2 <.text+0x12d74f2>
// Frames: F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d82f0  488b8a90010000                 movq	0x190(%rdx), %rcx
// Frames: F40 -> F41 -> F42 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d82f7  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1412d82fb  4885c0                         testq	%rax, %rax
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d82fe  0f84ee010000                   je	0x1412d84f2 <.text+0x12d74f2>
// Frames: F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d8304  4c8b9260010000                 movq	0x160(%rdx), %r10
// Frames: F44 -> F45 -> F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d830b  48f7d0                         notq	%rax
// Frames: F46 -> F47 -> F48 -> F45 -> F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d830e  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1412d8312  48c1e005                       shlq	$0x5, %rax
// Frames: F49 -> F50 -> F51 -> F52 -> F38 -> F30 -> F31 -> F27
// 0x1412d8316  49837c022000                   cmpq	$0x0, 0x20(%r10,%rax)
// Frames: F53 -> F51 -> F52 -> F38 -> F30 -> F31 -> F27
// 0x1412d831c  0f84d0010000                   je	0x1412d84f2 <.text+0x12d74f2>
// Frames: F54 -> F55 -> F56 -> F53 -> F51 -> F52 -> F38 -> F30 -> F31 -> F27
// 0x1412d8322  4901c2                         addq	%rax, %r10
// 0x1412d8325  4d8b7a10                       movq	0x10(%r10), %r15
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F38 -> F30 -> F31 -> F27
// 0x1412d8329  8b8248040000                   movl	0x448(%rdx), %eax
// Frames: F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d832f  488b4710                       movq	0x10(%rdi), %rax
// Frames: F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8333  4c8b8298010000                 movq	0x198(%rdx), %r8
// Frames: F32 -> F33 -> F34 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d833a  4939c0                         cmpq	%rax, %r8
// Frames: F39 -> F34 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d833d  0f86d0010000                   jbe	0x1412d8513 <.text+0x12d7513>
// Frames: F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8343  4c8b8a90010000                 movq	0x190(%rdx), %r9
// Frames: F40 -> F41 -> F42 -> F34 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d834a  498b04c1                       movq	(%r9,%rax,8), %rax
// 0x1412d834e  4885c0                         testq	%rax, %rax
// Frames: F34 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8351  0f84bc010000                   je	0x1412d8513 <.text+0x12d7513>
// Frames: F43 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8357  488b8a60010000                 movq	0x160(%rdx), %rcx
// Frames: F44 -> F45 -> F43 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d835e  48f7d0                         notq	%rax
// Frames: F46 -> F47 -> F48 -> F45 -> F43 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8361  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1412d8365  48c1e005                       shlq	$0x5, %rax
// Frames: F49 -> F50 -> F51 -> F66 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8369  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F53 -> F51 -> F66 -> F65 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d836f  0f849e010000                   je	0x1412d8513 <.text+0x12d7513>
// Frames: F30 -> F31 -> F27
// 0x1412d8375  4c89942490000000               movq	%r10, 0x90(%rsp)
// 0x1412d837d  4c8b5720                       movq	0x20(%rdi), %r10
// Frames: F32 -> F33 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d8381  4d39d0                         cmpq	%r10, %r8
// Frames: F39 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d8384  0f860e020000                   jbe	0x1412d8598 <.text+0x12d7598>
// Frames: F40 -> F41 -> F42 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d838a  4f8b04d1                       movq	(%r9,%r10,8), %r8
// 0x1412d838e  4d85c0                         testq	%r8, %r8
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d8391  0f8401020000                   je	0x1412d8598 <.text+0x12d7598>
// Frames: F44 -> F45 -> F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d8397  49f7d0                         notq	%r8
// Frames: F46 -> F47 -> F48 -> F45 -> F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F31 -> F27
// 0x1412d839a  4f8d2440                       leaq	(%r8,%r8,2), %r12
// 0x1412d839e  49c1e405                       shlq	$0x5, %r12
// Frames: F49 -> F50 -> F51 -> F52 -> F38 -> F30 -> F31 -> F27
// 0x1412d83a2  4a837c212000                   cmpq	$0x0, 0x20(%rcx,%r12)
// Frames: F53 -> F51 -> F52 -> F38 -> F30 -> F31 -> F27
// 0x1412d83a8  0f84ea010000                   je	0x1412d8598 <.text+0x12d7598>
// Frames: F30 -> F31 -> F27
// 0x1412d83ae  4801c8                         addq	%rcx, %rax
// 0x1412d83b1  488b4010                       movq	0x10(%rax), %rax
// 0x1412d83b5  4901cc                         addq	%rcx, %r12
// Frames: F54 -> F55 -> F56 -> F53 -> F51 -> F52 -> F38 -> F30 -> F31 -> F27
// 0x1412d83b8  4d8b6c2410                     movq	0x10(%r12), %r13
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F38 -> F30 -> F31 -> F27
// 0x1412d83bd  8b8a48040000                   movl	0x448(%rdx), %ecx
// Frames: F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83c3  41895c245c                     movl	%ebx, 0x5c(%r12)
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83c8  41c6450800                     movb	$0x0, 0x8(%r13)
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83cd  4c8b4708                       movq	0x8(%rdi), %r8
// 0x1412d83d1  31c9                           xorl	%ecx, %ecx
// Frames: F80 -> F81 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83d3  4d89c1                         movq	%r8, %r9
// 0x1412d83d6  4c2b4818                       subq	0x18(%rax), %r9
// 0x1412d83da  4c0f42c9                       cmovbq	%rcx, %r9
// Frames: F80 -> F82 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83de  4c2b4038                       subq	0x38(%rax), %r8
// 0x1412d83e2  4c0f42c1                       cmovbq	%rcx, %r8
// Frames: F83 -> F84 -> F33 -> F85 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83e6  4b8d1449                       leaq	(%r9,%r9,2), %rdx
// 0x1412d83ea  48c1e203                       shlq	$0x3, %rdx
// 0x1412d83ee  48035008                       addq	0x8(%rax), %rdx
// Frames: F85 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83f2  4c8b5010                       movq	0x10(%rax), %r10
// Frames: F86 -> F85 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83f6  4d29ca                         subq	%r9, %r10
// 0x1412d83f9  4c0f42d1                       cmovbq	%rcx, %r10
// Frames: F87 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d83fd  4c8b4830                       movq	0x30(%rax), %r9
// Frames: F86 -> F85 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8401  4c8d1da8feff00                 leaq	0xfffea8(%rip), %r11    # 0x1422d82b0
// 0x1412d8408  490f42d3                       cmovbq	%r11, %rdx
// Frames: F83 -> F84 -> F33 -> F87 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d840c  4b8d2c40                       leaq	(%r8,%r8,2), %rbp
// 0x1412d8410  48c1e503                       shlq	$0x3, %rbp
// 0x1412d8414  48036828                       addq	0x28(%rax), %rbp
// 0x1412d8418  4d39c8                         cmpq	%r9, %r8
// Frames: F86 -> F87 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d841b  490f47eb                       cmovaq	%r11, %rbp
// 0x1412d841f  4d29c1                         subq	%r8, %r9
// 0x1412d8422  4c0f42c9                       cmovbq	%rcx, %r9
// Frames: F88 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8426  488b4040                       movq	0x40(%rax), %rax
// 0x1412d842a  4c29c8                         subq	%r9, %rax
// 0x1412d842d  4c29d0                         subq	%r10, %rax
// 0x1412d8430  4889fe                         movq	%rdi, %rsi
// 0x1412d8433  48894708                       movq	%rax, 0x8(%rdi)
// Frames: F89 -> F90 -> F91 -> F92 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8437  4b8d0452                       leaq	(%r10,%r10,2), %rax
// 0x1412d843b  488d3cc2                       leaq	(%rdx,%rax,8), %rdi
// 0x1412d843f  4b8d0449                       leaq	(%r9,%r9,2), %rax
// 0x1412d8443  488d04c500000000               leaq	(,%rax,8), %rax
// 0x1412d844b  4801e8                         addq	%rbp, %rax
// 0x1412d844e  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x1412d8456  eb51                           jmp	0x1412d84a9 <.text+0x12d74a9>
// 0x1412d8458  0f1f840000000000               nopl	(%rax,%rax)
// 0x1412d8460  4c8d7218                       leaq	0x18(%rdx), %r14
// Frames: F93 -> F94 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8464  48ff4608                       incq	0x8(%rsi)
// Frames: F95 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8468  488b4208                       movq	0x8(%rdx), %rax
// 0x1412d846c  0fb64a10                       movzbl	0x10(%rdx), %ecx
// 0x1412d8470  4883c210                       addq	$0x10, %rdx
// Frames: F67 -> F68 -> F95 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8474  41895c245c                     movl	%ebx, 0x5c(%r12)
// Frames: F95 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8479  49894500                       movq	%rax, (%r13)
// 0x1412d847d  41884d08                       movb	%cl, 0x8(%r13)
// Frames: F96 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8481  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1412d8486  e845bf2fff                     callq	0x1405d43d0 <.text+0x5d33d0>
// 0x1412d848b  488b842490000000               movq	0x90(%rsp), %rax
// Frames: F67 -> F68 -> F96 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8493  89585c                         movl	%ebx, 0x5c(%rax)
// Frames: F96 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d8496  8b442460                       movl	0x60(%rsp), %eax
// 0x1412d849a  41894708                       movl	%eax, 0x8(%r15)
// 0x1412d849e  488b442458                     movq	0x58(%rsp), %rax
// 0x1412d84a3  498907                         movq	%rax, (%r15)
// 0x1412d84a6  4c89f2                         movq	%r14, %rdx
// Frames: F28 -> F97 -> F98 -> F99 -> F94 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d84a9  4885d2                         testq	%rdx, %rdx
// 0x1412d84ac  7405                           je	0x1412d84b3 <.text+0x12d74b3>
// 0x1412d84ae  4839fa                         cmpq	%rdi, %rdx
// 0x1412d84b1  75ad                           jne	0x1412d8460 <.text+0x12d7460>
// Frames: F28 -> F100 -> F101 -> F98 -> F99 -> F94 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d84b3  4885ed                         testq	%rbp, %rbp
// 0x1412d84b6  7416                           je	0x1412d84ce <.text+0x12d74ce>
// 0x1412d84b8  483bac2488000000               cmpq	0x88(%rsp), %rbp
// 0x1412d84c0  740c                           je	0x1412d84ce <.text+0x12d74ce>
// Frames: F102 -> F103 -> F104 -> F100 -> F101 -> F98 -> F99 -> F94 -> F79 -> F70 -> F71 -> F72 -> F73 -> F74 -> F27
// 0x1412d84c2  4889ea                         movq	%rbp, %rdx
// 0x1412d84c5  4883c518                       addq	$0x18, %rbp
// 0x1412d84c9  4531f6                         xorl	%r14d, %r14d
// 0x1412d84cc  eb96                           jmp	0x1412d8464 <.text+0x12d7464>
// Frames: F105 -> F27
// 0x1412d84ce  899e08010000                   movl	%ebx, 0x108(%rsi)
// Frames: F28 -> F106 -> F107
// 0x1412d84d4  48833e00                       cmpq	$0x0, (%rsi)
// 0x1412d84d8  0f84bf010000                   je	0x1412d869d <.text+0x12d769d>
// Frames: F108
// 0x1412d84de  4881c498000000                 addq	$0x98, %rsp
// 0x1412d84e5  5b                             popq	%rbx
// 0x1412d84e6  5d                             popq	%rbp
// 0x1412d84e7  5f                             popq	%rdi
// 0x1412d84e8  5e                             popq	%rsi
// 0x1412d84e9  415c                           popq	%r12
// 0x1412d84eb  415d                           popq	%r13
// 0x1412d84ed  415e                           popq	%r14
// 0x1412d84ef  415f                           popq	%r15
// 0x1412d84f1  c3                             retq
// Frames: F109 -> F110 -> F111 -> F30 -> F31 -> F27
// 0x1412d84f2  4881c7a0000000                 addq	$0xa0, %rdi
// Frames: F112 -> F113 -> F110 -> F111 -> F30 -> F31 -> F27
// 0x1412d84f9  488d0504962d01                 leaq	0x12d9604(%rip), %rax   # 0x1425b1b04
// 0x1412d8500  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1412d8505  48c74424503c000000             movq	$0x3c, 0x50(%rsp)
// 0x1412d850e  e9a1000000                     jmp	0x1412d85b4 <.text+0x12d75b4>
// Frames: F114 -> F110 -> F115 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8513  4881c7a0000000                 addq	$0xa0, %rdi
// Frames: F112 -> F116 -> F110 -> F115 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d851a  488d05cc000001                 leaq	0x10000cc(%rip), %rax   # 0x1422d85ed
// 0x1412d8521  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1412d8526  48c744245053000000             movq	$0x53, 0x50(%rsp)
// Frames: F114 -> F110 -> F115 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d852f  48897c2428                     movq	%rdi, 0x28(%rsp)
// 0x1412d8534  488d05b5bad2fe                 leaq	-0x12d454b(%rip), %rax  # 0x140003ff0 <.text+0x2ff0>
// 0x1412d853b  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1412d8540  488d442448                     leaq	0x48(%rsp), %rax
// 0x1412d8545  4889442438                     movq	%rax, 0x38(%rsp)
// 0x1412d854a  488d053f3ad9fe                 leaq	-0x126c5c1(%rip), %rax  # 0x14006bf90 <.text+0x6af90>
// 0x1412d8551  4889442440                     movq	%rax, 0x40(%rsp)
// Frames: F117 -> F118 -> F110 -> F115 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8556  488d055bfcff00                 leaq	0xfffc5b(%rip), %rax    # 0x1422d81b8
// 0x1412d855d  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1412d8562  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x1412d856b  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x1412d8574  488d442428                     leaq	0x28(%rsp), %rax
// 0x1412d8579  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1412d857e  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F114 -> F110 -> F115 -> F62 -> F63 -> F30 -> F31 -> F27
// 0x1412d8587  488d150afdff00                 leaq	0xfffd0a(%rip), %rdx    # 0x1422d8298
// 0x1412d858e  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1412d8593  e8e864dd00                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F109 -> F110 -> F111 -> F30 -> F31 -> F27
// 0x1412d8598  4881c7a0000000                 addq	$0xa0, %rdi
// Frames: F112 -> F113 -> F110 -> F111 -> F30 -> F31 -> F27
// 0x1412d859f  488d05a3942d01                 leaq	0x12d94a3(%rip), %rax   # 0x1425b1a49
// 0x1412d85a6  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1412d85ab  48c74424503d000000             movq	$0x3d, 0x50(%rsp)
// Frames: F30 -> F31 -> F27
// 0x1412d85b4  48897c2428                     movq	%rdi, 0x28(%rsp)
// 0x1412d85b9  488d0530bad2fe                 leaq	-0x12d45d0(%rip), %rax  # 0x140003ff0 <.text+0x2ff0>
// 0x1412d85c0  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1412d85c5  488d442448                     leaq	0x48(%rsp), %rax
// 0x1412d85ca  4889442438                     movq	%rax, 0x38(%rsp)
// 0x1412d85cf  488d05ba39d9fe                 leaq	-0x126c646(%rip), %rax  # 0x14006bf90 <.text+0x6af90>
// 0x1412d85d6  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1412d85db  488d057e902d01                 leaq	0x12d907e(%rip), %rax   # 0x1425b1660
// 0x1412d85e2  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1412d85e7  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x1412d85f0  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x1412d85f9  488d442428                     leaq	0x28(%rsp), %rax
// 0x1412d85fe  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1412d8603  48c744247002000000             movq	$0x2, 0x70(%rsp)
// 0x1412d860c  488d15dd932d01                 leaq	0x12d93dd(%rip), %rdx   # 0x1425b19f0
// 0x1412d8613  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1412d8618  e86364dd00                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F4 -> F5
// 0x1412d861d  488d059c9d2d01                 leaq	0x12d9d9c(%rip), %rax   # 0x1425b23c0
// 0x1412d8624  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1412d8629  48c744246001000000             movq	$0x1, 0x60(%rsp)
// 0x1412d8632  488d0507912d01                 leaq	0x12d9107(%rip), %rax   # 0x1425b1740
// 0x1412d8639  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1412d863e  0f57c0                         xorps	%xmm0, %xmm0
// 0x1412d8641  0f11442470                     movups	%xmm0, 0x70(%rsp)
// 0x1412d8646  4883c728                       addq	$0x28, %rdi
// 0x1412d864a  488d057f9d2d01                 leaq	0x12d9d7f(%rip), %rax   # 0x1425b23d0
// 0x1412d8651  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1412d8656  4c8d442428                     leaq	0x28(%rsp), %r8
// 0x1412d865b  4c8d4c2458                     leaq	0x58(%rsp), %r9
// 0x1412d8660  31c9                           xorl	%ecx, %ecx
// 0x1412d8662  4889fa                         movq	%rdi, %rdx
// 0x1412d8665  e83690dd00                     callq	0x1420b16a0 <.text+0x20b06a0>
// Frames: F119 -> F17 -> F18 -> F19 -> F20 -> F5
// 0x1412d866a  4c8d05779d2d01                 leaq	0x12d9d77(%rip), %r8    # 0x1425b23e8
// 0x1412d8671  4889c2                         movq	%rax, %rdx
// 0x1412d8674  e84769dd00                     callq	0x1420aefc0 <.text+0x20adfc0>
// Frames: F120 -> F121 -> F5
// 0x1412d8679  488d0d809d2d01                 leaq	0x12d9d80(%rip), %rcx   # 0x1425b2400
// 0x1412d8680  e8bb63dd00                     callq	0x1420aea40 <.text+0x20ada40>
// Frames: F122 -> F29 -> F27
// 0x1412d8685  488d0d54952d01                 leaq	0x12d9554(%rip), %rcx   # 0x1425b1be0
// 0x1412d868c  4c8d0515962d01                 leaq	0x12d9615(%rip), %r8    # 0x1425b1ca8
// 0x1412d8693  ba5f000000                     movl	$0x5f, %edx
// 0x1412d8698  e8c363dd00                     callq	0x1420aea60 <.text+0x20ada60>
// Frames: F122 -> F106 -> F107
// 0x1412d869d  488d0d3c952d01                 leaq	0x12d953c(%rip), %rcx   # 0x1425b1be0
// 0x1412d86a4  4c8d0515962d01                 leaq	0x12d9615(%rip), %r8    # 0x1425b1cc0
// 0x1412d86ab  ba5f000000                     movl	$0x5f, %edx
// 0x1412d86b0  e8ab63dd00                     callq	0x1420aea60 <.text+0x20ada60>
// 0x1412d86b5  cc                             int3
//
// RANGE rva_12e04d0_len_37c: RVA 0x12e04d0, 892 PDB bytes; 191 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_12e04d0_len_37c.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 1939:1216: bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>
// Frames: F123
// 0x1412e04d0  4157                           pushq	%r15
// 0x1412e04d2  4156                           pushq	%r14
// 0x1412e04d4  4155                           pushq	%r13
// 0x1412e04d6  4154                           pushq	%r12
// 0x1412e04d8  56                             pushq	%rsi
// 0x1412e04d9  57                             pushq	%rdi
// 0x1412e04da  55                             pushq	%rbp
// 0x1412e04db  53                             pushq	%rbx
// 0x1412e04dc  4881ec98000000                 subq	$0x98, %rsp
// 0x1412e04e3  bb01000000                     movl	$0x1, %ebx
// Frames: F22 -> F23 -> F24 -> F25 -> F124
// 0x1412e04e8  f0                             lock
// 0x1412e04e9  0fc19a48040000                 xaddl	%ebx, 0x448(%rdx)
// Frames: F28 -> F125
// 0x1412e04f0  48833900                       cmpq	$0x0, (%rcx)
// 0x1412e04f4  0f8439030000                   je	0x1412e0833 <.text+0x12df833>
// Frames: F30 -> F126
// 0x1412e04fa  488b4118                       movq	0x18(%rcx), %rax
// Frames: F32 -> F33 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e04fe  48398298010000                 cmpq	%rax, 0x198(%rdx)
// Frames: F39 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e0505  0f86fd010000                   jbe	0x1412e0708 <.text+0x12df708>
// Frames: F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e050b  4c8b8290010000                 movq	0x190(%rdx), %r8
// Frames: F40 -> F41 -> F42 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e0512  498b04c0                       movq	(%r8,%rax,8), %rax
// 0x1412e0516  4885c0                         testq	%rax, %rax
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e0519  0f84e9010000                   je	0x1412e0708 <.text+0x12df708>
// Frames: F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e051f  4c8b9260010000                 movq	0x160(%rdx), %r10
// Frames: F44 -> F45 -> F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e0526  48f7d0                         notq	%rax
// Frames: F46 -> F47 -> F48 -> F45 -> F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e0529  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1412e052d  48c1e005                       shlq	$0x5, %rax
// Frames: F49 -> F50 -> F51 -> F52 -> F38 -> F30 -> F126
// 0x1412e0531  49837c022000                   cmpq	$0x0, 0x20(%r10,%rax)
// Frames: F53 -> F51 -> F52 -> F38 -> F30 -> F126
// 0x1412e0537  0f84cb010000                   je	0x1412e0708 <.text+0x12df708>
// Frames: F54 -> F55 -> F56 -> F53 -> F51 -> F52 -> F38 -> F30 -> F126
// 0x1412e053d  4901c2                         addq	%rax, %r10
// 0x1412e0540  4d8b7a10                       movq	0x10(%r10), %r15
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F38 -> F30 -> F126
// 0x1412e0544  8b8248040000                   movl	0x448(%rdx), %eax
// Frames: F62 -> F63 -> F30 -> F126
// 0x1412e054a  488b4110                       movq	0x10(%rcx), %rax
// Frames: F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e054e  4c8b8298010000                 movq	0x198(%rdx), %r8
// Frames: F32 -> F33 -> F34 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e0555  4939c0                         cmpq	%rax, %r8
// Frames: F39 -> F34 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e0558  0f86cb010000                   jbe	0x1412e0729 <.text+0x12df729>
// Frames: F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e055e  4c8b8a90010000                 movq	0x190(%rdx), %r9
// Frames: F40 -> F41 -> F42 -> F34 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e0565  498b04c1                       movq	(%r9,%rax,8), %rax
// 0x1412e0569  4885c0                         testq	%rax, %rax
// Frames: F34 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e056c  0f84b7010000                   je	0x1412e0729 <.text+0x12df729>
// Frames: F43 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e0572  4c8b9a60010000                 movq	0x160(%rdx), %r11
// Frames: F44 -> F45 -> F43 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e0579  48f7d0                         notq	%rax
// Frames: F46 -> F47 -> F48 -> F45 -> F43 -> F35 -> F36 -> F64 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e057c  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1412e0580  48c1e005                       shlq	$0x5, %rax
// Frames: F49 -> F50 -> F51 -> F66 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e0584  49837c032000                   cmpq	$0x0, 0x20(%r11,%rax)
// Frames: F53 -> F51 -> F66 -> F65 -> F62 -> F63 -> F30 -> F126
// 0x1412e058a  0f8499010000                   je	0x1412e0729 <.text+0x12df729>
// Frames: F30 -> F126
// 0x1412e0590  4c89942490000000               movq	%r10, 0x90(%rsp)
// 0x1412e0598  4c8b5120                       movq	0x20(%rcx), %r10
// Frames: F32 -> F33 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e059c  4d39d0                         cmpq	%r10, %r8
// Frames: F39 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e059f  0f8609020000                   jbe	0x1412e07ae <.text+0x12df7ae>
// Frames: F40 -> F41 -> F42 -> F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e05a5  4f8b04d1                       movq	(%r9,%r10,8), %r8
// 0x1412e05a9  4d85c0                         testq	%r8, %r8
// Frames: F34 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e05ac  0f84fc010000                   je	0x1412e07ae <.text+0x12df7ae>
// Frames: F44 -> F45 -> F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e05b2  49f7d0                         notq	%r8
// Frames: F46 -> F47 -> F48 -> F45 -> F43 -> F35 -> F36 -> F37 -> F38 -> F30 -> F126
// 0x1412e05b5  4f8d2440                       leaq	(%r8,%r8,2), %r12
// 0x1412e05b9  49c1e405                       shlq	$0x5, %r12
// Frames: F49 -> F50 -> F51 -> F52 -> F38 -> F30 -> F126
// 0x1412e05bd  4b837c232000                   cmpq	$0x0, 0x20(%r11,%r12)
// Frames: F53 -> F51 -> F52 -> F38 -> F30 -> F126
// 0x1412e05c3  0f84e5010000                   je	0x1412e07ae <.text+0x12df7ae>
// Frames: F30 -> F126
// 0x1412e05c9  4c01d8                         addq	%r11, %rax
// 0x1412e05cc  488b4010                       movq	0x10(%rax), %rax
// 0x1412e05d0  4d01dc                         addq	%r11, %r12
// Frames: F54 -> F55 -> F56 -> F53 -> F51 -> F52 -> F38 -> F30 -> F126
// 0x1412e05d3  4d8b6c2410                     movq	0x10(%r12), %r13
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F38 -> F30 -> F126
// 0x1412e05d8  8b9248040000                   movl	0x448(%rdx), %edx
// Frames: F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e05de  41895c245c                     movl	%ebx, 0x5c(%r12)
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e05e3  41c6450800                     movb	$0x0, 0x8(%r13)
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e05e8  4c8b4108                       movq	0x8(%rcx), %r8
// 0x1412e05ec  31f6                           xorl	%esi, %esi
// Frames: F80 -> F81 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e05ee  4d89c1                         movq	%r8, %r9
// 0x1412e05f1  4c2b4818                       subq	0x18(%rax), %r9
// 0x1412e05f5  4c0f42ce                       cmovbq	%rsi, %r9
// Frames: F80 -> F82 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e05f9  4c2b4038                       subq	0x38(%rax), %r8
// 0x1412e05fd  4c0f42c6                       cmovbq	%rsi, %r8
// Frames: F83 -> F84 -> F33 -> F85 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0601  4b8d1449                       leaq	(%r9,%r9,2), %rdx
// 0x1412e0605  48c1e203                       shlq	$0x3, %rdx
// 0x1412e0609  48035008                       addq	0x8(%rax), %rdx
// Frames: F85 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e060d  4c8b5010                       movq	0x10(%rax), %r10
// Frames: F86 -> F85 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0611  4d29ca                         subq	%r9, %r10
// 0x1412e0614  4c0f42d6                       cmovbq	%rsi, %r10
// Frames: F87 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0618  4c8b4830                       movq	0x30(%rax), %r9
// Frames: F86 -> F85 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e061c  4c8d1d8d7cff00                 leaq	0xff7c8d(%rip), %r11    # 0x1422d82b0
// 0x1412e0623  490f42d3                       cmovbq	%r11, %rdx
// Frames: F83 -> F84 -> F33 -> F87 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0627  4b8d2c40                       leaq	(%r8,%r8,2), %rbp
// 0x1412e062b  48c1e503                       shlq	$0x3, %rbp
// 0x1412e062f  48036828                       addq	0x28(%rax), %rbp
// 0x1412e0633  4d39c8                         cmpq	%r9, %r8
// Frames: F86 -> F87 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0636  490f47eb                       cmovaq	%r11, %rbp
// 0x1412e063a  4d29c1                         subq	%r8, %r9
// 0x1412e063d  4c0f42ce                       cmovbq	%rsi, %r9
// Frames: F88 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0641  488b4040                       movq	0x40(%rax), %rax
// 0x1412e0645  4c29c8                         subq	%r9, %rax
// 0x1412e0648  4c29d0                         subq	%r10, %rax
// 0x1412e064b  4889cf                         movq	%rcx, %rdi
// 0x1412e064e  48894108                       movq	%rax, 0x8(%rcx)
// Frames: F89 -> F90 -> F91 -> F92 -> F76 -> F77 -> F78 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0652  4b8d0452                       leaq	(%r10,%r10,2), %rax
// 0x1412e0656  488d34c2                       leaq	(%rdx,%rax,8), %rsi
// 0x1412e065a  4b8d0449                       leaq	(%r9,%r9,2), %rax
// 0x1412e065e  488d04c500000000               leaq	(,%rax,8), %rax
// 0x1412e0666  4801e8                         addq	%rbp, %rax
// 0x1412e0669  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x1412e0671  eb56                           jmp	0x1412e06c9 <.text+0x12df6c9>
// 0x1412e0673  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1412e0680  4c8d7218                       leaq	0x18(%rdx), %r14
// Frames: F93 -> F94 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0684  48ff4708                       incq	0x8(%rdi)
// Frames: F95 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0688  488b4208                       movq	0x8(%rdx), %rax
// 0x1412e068c  0fb64a10                       movzbl	0x10(%rdx), %ecx
// 0x1412e0690  4883c210                       addq	$0x10, %rdx
// Frames: F67 -> F68 -> F95 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0694  41895c245c                     movl	%ebx, 0x5c(%r12)
// Frames: F95 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e0699  49894500                       movq	%rax, (%r13)
// 0x1412e069d  41884d08                       movb	%cl, 0x8(%r13)
// Frames: F96 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e06a1  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1412e06a6  e8253d2fff                     callq	0x1405d43d0 <.text+0x5d33d0>
// 0x1412e06ab  488b842490000000               movq	0x90(%rsp), %rax
// Frames: F67 -> F68 -> F96 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e06b3  89585c                         movl	%ebx, 0x5c(%rax)
// Frames: F96 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e06b6  8b442460                       movl	0x60(%rsp), %eax
// 0x1412e06ba  41894708                       movl	%eax, 0x8(%r15)
// 0x1412e06be  488b442458                     movq	0x58(%rsp), %rax
// 0x1412e06c3  498907                         movq	%rax, (%r15)
// 0x1412e06c6  4c89f2                         movq	%r14, %rdx
// Frames: F28 -> F97 -> F98 -> F99 -> F94 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e06c9  4885d2                         testq	%rdx, %rdx
// 0x1412e06cc  7405                           je	0x1412e06d3 <.text+0x12df6d3>
// 0x1412e06ce  4839f2                         cmpq	%rsi, %rdx
// 0x1412e06d1  75ad                           jne	0x1412e0680 <.text+0x12df680>
// Frames: F28 -> F100 -> F101 -> F98 -> F99 -> F94 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e06d3  4885ed                         testq	%rbp, %rbp
// 0x1412e06d6  7416                           je	0x1412e06ee <.text+0x12df6ee>
// 0x1412e06d8  483bac2488000000               cmpq	0x88(%rsp), %rbp
// 0x1412e06e0  740c                           je	0x1412e06ee <.text+0x12df6ee>
// Frames: F102 -> F103 -> F104 -> F100 -> F101 -> F98 -> F99 -> F94 -> F79 -> F70 -> F71 -> F72 -> F73 -> F127
// 0x1412e06e2  4889ea                         movq	%rbp, %rdx
// 0x1412e06e5  4883c518                       addq	$0x18, %rbp
// 0x1412e06e9  4531f6                         xorl	%r14d, %r14d
// 0x1412e06ec  eb96                           jmp	0x1412e0684 <.text+0x12df684>
// Frames: F128
// 0x1412e06ee  899f08010000                   movl	%ebx, 0x108(%rdi)
// Frames: F129
// 0x1412e06f4  4881c498000000                 addq	$0x98, %rsp
// 0x1412e06fb  5b                             popq	%rbx
// 0x1412e06fc  5d                             popq	%rbp
// 0x1412e06fd  5f                             popq	%rdi
// 0x1412e06fe  5e                             popq	%rsi
// 0x1412e06ff  415c                           popq	%r12
// 0x1412e0701  415d                           popq	%r13
// 0x1412e0703  415e                           popq	%r14
// 0x1412e0705  415f                           popq	%r15
// 0x1412e0707  c3                             retq
// Frames: F109 -> F110 -> F111 -> F30 -> F126
// 0x1412e0708  4881c1a0000000                 addq	$0xa0, %rcx
// Frames: F112 -> F113 -> F110 -> F111 -> F30 -> F126
// 0x1412e070f  488d0542912d01                 leaq	0x12d9142(%rip), %rax   # 0x1425b9858
// 0x1412e0716  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1412e071b  48c74424303c000000             movq	$0x3c, 0x30(%rsp)
// 0x1412e0724  e9a1000000                     jmp	0x1412e07ca <.text+0x12df7ca>
// Frames: F114 -> F110 -> F115 -> F62 -> F63 -> F30 -> F126
// 0x1412e0729  4881c1a0000000                 addq	$0xa0, %rcx
// Frames: F112 -> F116 -> F110 -> F115 -> F62 -> F63 -> F30 -> F126
// 0x1412e0730  488d05b67eff00                 leaq	0xff7eb6(%rip), %rax    # 0x1422d85ed
// 0x1412e0737  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1412e073c  48c744243053000000             movq	$0x53, 0x30(%rsp)
// Frames: F114 -> F110 -> F115 -> F62 -> F63 -> F30 -> F126
// 0x1412e0745  48894c2438                     movq	%rcx, 0x38(%rsp)
// 0x1412e074a  488d059f38d2fe                 leaq	-0x12dc761(%rip), %rax  # 0x140003ff0 <.text+0x2ff0>
// 0x1412e0751  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1412e0756  488d442428                     leaq	0x28(%rsp), %rax
// 0x1412e075b  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1412e0760  488d0529b8d8fe                 leaq	-0x12747d7(%rip), %rax  # 0x14006bf90 <.text+0x6af90>
// 0x1412e0767  4889442450                     movq	%rax, 0x50(%rsp)
// Frames: F117 -> F118 -> F110 -> F115 -> F62 -> F63 -> F30 -> F126
// 0x1412e076c  488d05457aff00                 leaq	0xff7a45(%rip), %rax    # 0x1422d81b8
// 0x1412e0773  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1412e0778  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x1412e0781  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x1412e078a  488d442438                     leaq	0x38(%rsp), %rax
// 0x1412e078f  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1412e0794  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F114 -> F110 -> F115 -> F62 -> F63 -> F30 -> F126
// 0x1412e079d  488d15f47aff00                 leaq	0xff7af4(%rip), %rdx    # 0x1422d8298
// 0x1412e07a4  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1412e07a9  e8d2e2dc00                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F109 -> F110 -> F111 -> F30 -> F126
// 0x1412e07ae  4881c1a0000000                 addq	$0xa0, %rcx
// Frames: F112 -> F113 -> F110 -> F111 -> F30 -> F126
// 0x1412e07b5  488d05dc8f2d01                 leaq	0x12d8fdc(%rip), %rax   # 0x1425b9798
// 0x1412e07bc  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1412e07c1  48c74424303d000000             movq	$0x3d, 0x30(%rsp)
// Frames: F30 -> F126
// 0x1412e07ca  48894c2438                     movq	%rcx, 0x38(%rsp)
// 0x1412e07cf  488d051a38d2fe                 leaq	-0x12dc7e6(%rip), %rax  # 0x140003ff0 <.text+0x2ff0>
// 0x1412e07d6  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1412e07db  488d442428                     leaq	0x28(%rsp), %rax
// 0x1412e07e0  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1412e07e5  488d05a4b7d8fe                 leaq	-0x127485c(%rip), %rax  # 0x14006bf90 <.text+0x6af90>
// 0x1412e07ec  4889442450                     movq	%rax, 0x50(%rsp)
// 0x1412e07f1  488d05808f2d01                 leaq	0x12d8f80(%rip), %rax   # 0x1425b9778
// 0x1412e07f8  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1412e07fd  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x1412e0806  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x1412e080f  488d442438                     leaq	0x38(%rsp), %rax
// 0x1412e0814  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1412e0819  48c744247002000000             movq	$0x2, 0x70(%rsp)
// 0x1412e0822  488d1517902d01                 leaq	0x12d9017(%rip), %rdx   # 0x1425b9840
// 0x1412e0829  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1412e082e  e84de2dc00                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F122 -> F125
// 0x1412e0833  488d0d36912d01                 leaq	0x12d9136(%rip), %rcx   # 0x1425b9970
// 0x1412e083a  4c8d058f912d01                 leaq	0x12d918f(%rip), %r8    # 0x1425b99d0
// 0x1412e0841  ba5f000000                     movl	$0x5f, %edx
// 0x1412e0846  e815e2dc00                     callq	0x1420aea60 <.text+0x20ada60>
// 0x1412e084b  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:77:0 bevy_ecs::system::system::System::run::hc95485909cbf31e7
// F2: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:184:0 bevy_ecs::world::World::id
// F3: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:201:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::id
// F4: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:567:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:79:0 bevy_ecs::system::system::System::run::hc95485909cbf31e7
// F6: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:2175:0 core::option::impl$15::eq
// F7: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:2155:0 core::option::impl$14::eq
// F8: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/macros/mod.rs:53:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2203:0 alloc::vec::Vec::len
// F10: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:734:0 bevy_ecs::archetype::Archetypes::generation
// F11: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:570:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1284:0 core::ptr::read
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:921:0 bevy_ecs::archetype::Archetypes::generation
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F15: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:922:0 bevy_ecs::archetype::Archetypes::generation
// F16: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:502:0 core::slice::index::impl$6::index
// F17: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:18:0 core::slice::index::impl$0::index
// F18: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2829:0 alloc::vec::impl$12::index
// F19: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:893:0 bevy_ecs::archetype::impl$10::index
// F20: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:572:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:162:0 core::slice::iter::impl$181::next
// F22: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3343:0 core::sync::atomic::atomic_add
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:2667:0 core::sync::atomic::AtomicU32::fetch_add
// F24: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:2065:0 bevy_ecs::world::World::increment_change_tick
// F25: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:279:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::increment_change_tick
// F26: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:519:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:83:0 bevy_ecs::system::system::System::run::hc95485909cbf31e7
// F28: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:705:0 core::option::Option::as_mut
// F29: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:528:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F30: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:1474:0 bevy_ecs::system::system_param::impl$85::get_param
// F31: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:527:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F34: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::SparseArray::get
// F35: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::SparseSet::get
// F36: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs:286:0 bevy_ecs::storage::resource::Resources::get
// F37: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:448:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F38: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:626:0 bevy_ecs::system::system_param::impl$7::get_param
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::slice::impl$0::get
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F41: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::impl$10::get::closure$0
// F42: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::slice::impl$0::get
// F43: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F44: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs:398:0 nonmax::NonMaxUsize::get
// F45: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:372:0 bevy_ecs::storage::sparse_set::impl$15::get::closure$0
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:964:0 core::ptr::const_ptr::impl$0::add
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:246:0 core::slice::index::impl$2::get_unchecked
// F48: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:686:0 nonmax::NonMaxUsize::get
// F49: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:93:0 bevy_ecs::storage::blob_vec::BlobVec::is_empty
// F50: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs:70:0 bevy_ecs::storage::resource::ResourceData::is_present
// F51: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs:113:0 bevy_ecs::storage::resource::ResourceData::get_with_ticks
// F52: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:449:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F53: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 bevy_ecs::storage::resource::ResourceData::is_present
// F54: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:396:0 bevy_ecs::storage::blob_vec::BlobVec::get_ptr
// F55: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:372:0 bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
// F56: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs:117:0 bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
// F57: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3308:0 core::sync::atomic::atomic_load
// F58: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3115:0 core::sync::atomic::AtomicU32::load
// F59: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:2075:0 bevy_ecs::world::World::read_change_tick
// F60: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:255:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
// F61: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:455:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F62: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:1474:0 bevy_ecs::system::system_param::impl$83::get_param
// F63: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:458:0 bevy_ecs::event::_::impl$0::get_param
// F64: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:543:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F65: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:531:0 bevy_ecs::system::system_param::impl$4::get_param
// F66: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:544:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F67: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$29::set_changed
// F68: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F69: crates/systems/roof/src/highlight_roof.rs:12:0 system_roof::highlight_roof::highlight_roof
// F70: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:166:0 core::ops::function::FnMut::call_mut
// F71: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:294:0 core::ops::function::impls::impl$3::call_mut
// F72: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:710:0 bevy_ecs::system::function_system::impl$16::run::call_inner
// F73: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:713:0 bevy_ecs::system::function_system::impl$16::run
// F74: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:534:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F75: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:811:0 bevy_ecs::event::EventIteratorWithId::new
// F76: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:723:0 bevy_ecs::event::ManualEventReader::read_with_id
// F77: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:718:0 bevy_ecs::event::ManualEventReader::read
// F78: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:469:0 bevy_ecs::event::EventReader::read
// F79: crates/systems/roof/src/highlight_roof.rs:14:0 system_roof::highlight_roof::highlight_roof
// F80: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::saturating_sub
// F81: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:813:0 bevy_ecs::event::EventIteratorWithId::new
// F82: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:816:0 bevy_ecs::event::EventIteratorWithId::new
// F83: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:354:0 core::slice::index::impl$4::get
// F84: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:480:0 core::slice::index::impl$6::get
// F85: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:817:0 bevy_ecs::event::EventIteratorWithId::new
// F86: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1007:0 core::slice::impl$0::get
// F87: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:818:0 bevy_ecs::event::EventIteratorWithId::new
// F88: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:823:0 bevy_ecs::event::EventIteratorWithId::new
// F89: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F90: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F91: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F92: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:825:0 bevy_ecs::event::EventIteratorWithId::new
// F93: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:850:0 bevy_ecs::event::impl$23::next
// F94: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:771:0 bevy_ecs::event::impl$20::next
// F95: crates/systems/roof/src/highlight_roof.rs:15:0 system_roof::highlight_roof::highlight_roof
// F96: crates/systems/roof/src/highlight_roof.rs:16:0 system_roof::highlight_roof::highlight_roof
// F97: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:300:0 core::iter::adapters::chain::and_then_or_clear
// F98: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next
// F99: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:845:0 bevy_ecs::event::impl$23::next
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next::closure$0
// F101: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1513:0 core::option::Option::or_else
// F102: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F103: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F105: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:535:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F106: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:541:0 bevy_ecs::system::function_system::impl$7::apply_deferred
// F107: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:84:0 bevy_ecs::system::system::System::run::hc95485909cbf31e7
// F108: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:86:0 bevy_ecs::system::system::System::run::hc95485909cbf31e7
// F109: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:629:0 bevy_ecs::system::system_param::impl$7::get_param::closure$0
// F110: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:978:0 core::option::Option::unwrap_or_else
// F111: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:628:0 bevy_ecs::system::system_param::impl$7::get_param
// F112: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/any.rs:700:0 core::any::type_name
// F113: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:632:0 bevy_ecs::system::system_param::impl$7::get_param::closure$0
// F114: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:534:0 bevy_ecs::system::system_param::impl$4::get_param::closure$0
// F115: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:533:0 bevy_ecs::system::system_param::impl$4::get_param
// F116: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:537:0 bevy_ecs::system::system_param::impl$4::get_param::closure$0
// F117: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:343:0 core::fmt::Arguments::new_v1
// F118: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panic.rs:106:0 bevy_ecs::system::system_param::impl$4::get_param::closure$0
// F119: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:503:0 core::slice::index::impl$6::index
// F120: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:933:0 core::option::Option::unwrap
// F121: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:573:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F122: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:896:0 core::option::Option::as_mut
// F123: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:515:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h319441bf3f0e832a (.llvm.3193233252864566565)
// F124: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:519:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h319441bf3f0e832a (.llvm.3193233252864566565)
// F125: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:528:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h319441bf3f0e832a (.llvm.3193233252864566565)
// F126: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:527:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h319441bf3f0e832a (.llvm.3193233252864566565)
// F127: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:534:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h319441bf3f0e832a (.llvm.3193233252864566565)
// F128: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:535:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h319441bf3f0e832a (.llvm.3193233252864566565)
// F129: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:537:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h319441bf3f0e832a (.llvm.3193233252864566565)
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/highlight_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f8a9b248f93cc5a5d938d50ea3703d71e714d3fb114130d460716881dcf7361a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: highlight_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1927, 1939
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::highlight_roof::highlight_roof
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12D7B
// 首条位置: {"module":1927,"inlinee":77179}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1927,"record_offset":287892,"parent_record_offset":287852,"ancestor_record_offset":287852,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1927,"record_offset":287948,"parent_record_offset":287852,"ancestor_record_offset":287852,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1927,"record_offset":288480,"parent_record_offset":287852,"ancestor_record_offset":287852,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1927,"record_offset":287916,"parent_record_offset":287892,"ancestor_record_offset":287852,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1927,"record_offset":288020,"parent_record_offset":287996,"ancestor_record_offset":287852,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1927,"record_offset":287972,"parent_record_offset":287948,"ancestor_record_offset":287852,"inlinee":6368,"source_line":717,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1927,"record_offset":287996,"parent_record_offset":287972,"ancestor_record_offset":287852,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1927,"record_offset":288508,"parent_record_offset":288480,"ancestor_record_offset":287852,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1927,"record_offset":288564,"parent_record_offset":288540,"ancestor_record_offset":287852,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1927,"record_offset":288540,"parent_record_offset":288508,"ancestor_record_offset":287852,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1927,"record_offset":288644,"parent_record_offset":288620,"ancestor_record_offset":287852,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1927,"record_offset":288068,"parent_record_offset":288020,"ancestor_record_offset":287852,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1927,"record_offset":288588,"parent_record_offset":288564,"ancestor_record_offset":287852,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1927,"record_offset":288620,"parent_record_offset":288540,"ancestor_record_offset":287852,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1927,"record_offset":288372,"parent_record_offset":288348,"ancestor_record_offset":287852,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1927,"record_offset":288744,"parent_record_offset":288720,"ancestor_record_offset":287852,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1927,"record_offset":288132,"parent_record_offset":288020,"ancestor_record_offset":287852,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1927,"record_offset":288324,"parent_record_offset":288020,"ancestor_record_offset":287852,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1927,"record_offset":288196,"parent_record_offset":288172,"ancestor_record_offset":287852,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1927,"record_offset":288172,"parent_record_offset":288132,"ancestor_record_offset":287852,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1927,"record_offset":288348,"parent_record_offset":288324,"ancestor_record_offset":287852,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1927,"record_offset":288720,"parent_record_offset":288696,"ancestor_record_offset":287852,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01927.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1927,"record_offset":288696,"parent_record_offset":288644,"ancestor_record_offset":287852,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01927.jsonl.gz"}
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
// 完整结构及原始类型记录: highlight_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: highlight_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: highlight_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 23; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >", "tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_roof::highlight_roof::highlight_roof
// 信息 ID: a71e08b28218ff08cf31a945
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 73f1a71d1d8226dabdf477ee: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)> >; module=1927, inline_record=287852; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>; module=1939, inline_record=4580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
