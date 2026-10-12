// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: delete_roof.rs.assembly.json.gz

// FUNCTION system_roof::delete_roof::process_invalid_roof_notifications
// Candidate interfaces: d3696816013ccc14dc345402
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 264:248 -> rva_3eee70_len_55a
//
// FUNCTION system_roof::delete_roof::mark_roof_for_deletion
// Candidate interfaces: 1ac3b09f94008d55466c958a
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 264:4828 -> rva_3ef3d0_len_45a
//
// FUNCTION system_roof::delete_roof::delete_roofs
// Candidate interfaces: 22f506c314bfeff5269f4bf0
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 264:8468 -> rva_3ef830_len_1ac
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_3eee70_len_55a: RVA 0x3eee70, 1370 PDB bytes; 355 instructions
// Evidence: _assembly/roof-v1/A/pack-000/rva_3eee70_len_55a.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 264:248: system_roof::delete_roof::process_invalid_roof_notifications
// Frames: F1
// 0x1403eee70  55                             pushq	%rbp
// 0x1403eee71  4157                           pushq	%r15
// 0x1403eee73  4156                           pushq	%r14
// 0x1403eee75  4155                           pushq	%r13
// 0x1403eee77  4154                           pushq	%r12
// 0x1403eee79  56                             pushq	%rsi
// 0x1403eee7a  57                             pushq	%rdi
// 0x1403eee7b  53                             pushq	%rbx
// 0x1403eee7c  4881ecd8000000                 subq	$0xd8, %rsp
// 0x1403eee83  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1403eee8b  48c74550feffffff               movq	$-0x2, 0x50(%rbp)
// 0x1403eee93  4c89cb                         movq	%r9, %rbx
// Frames: F2 -> F3 -> F4
// 0x1403eee96  498b00                         movq	(%r8), %rax
// Frames: F5 -> F3 -> F4
// 0x1403eee99  4d8b5008                       movq	0x8(%r8), %r10
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x1403eee9d  4c8b00                         movq	(%rax), %r8
// 0x1403eeea0  4531db                         xorl	%r11d, %r11d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x1403eeea3  4c89c6                         movq	%r8, %rsi
// 0x1403eeea6  492b7218                       subq	0x18(%r10), %rsi
// 0x1403eeeaa  490f42f3                       cmovbq	%r11, %rsi
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x1403eeeae  4d2b4238                       subq	0x38(%r10), %r8
// 0x1403eeeb2  4d0f42c3                       cmovbq	%r11, %r8
// Frames: F12 -> F7 -> F8 -> F3 -> F4
// 0x1403eeeb6  4d8b7210                       movq	0x10(%r10), %r14
// Frames: F13 -> F7 -> F8 -> F3 -> F4
// 0x1403eeeba  4d8b4a30                       movq	0x30(%r10), %r9
// Frames: F14 -> F12 -> F7 -> F8 -> F3 -> F4
// 0x1403eeebe  4d89f7                         movq	%r14, %r15
// 0x1403eeec1  4929f7                         subq	%rsi, %r15
// 0x1403eeec4  4d0f42fb                       cmovbq	%r11, %r15
// Frames: F14 -> F13 -> F7 -> F8 -> F3 -> F4
// 0x1403eeec8  4d89cc                         movq	%r9, %r12
// 0x1403eeecb  4d29c4                         subq	%r8, %r12
// 0x1403eeece  4d0f42e3                       cmovbq	%r11, %r12
// 0x1403eeed2  4889cf                         movq	%rcx, %rdi
// Frames: F15 -> F7 -> F8 -> F3 -> F4
// 0x1403eeed5  498b4a40                       movq	0x40(%r10), %rcx
// Frames: F16 -> F7 -> F8 -> F3 -> F4
// 0x1403eeed9  4d01fc                         addq	%r15, %r12
// Frames: F15 -> F7 -> F8 -> F3 -> F4
// 0x1403eeedc  4989ca                         movq	%rcx, %r10
// 0x1403eeedf  4d29e2                         subq	%r12, %r10
// 0x1403eeee2  4c8b9dc0000000                 movq	0xc0(%rbp), %r11
// 0x1403eeee9  4c8910                         movq	%r10, (%rax)
// Frames: F14 -> F12 -> F7 -> F8 -> F3 -> F4
// 0x1403eeeec  4939f6                         cmpq	%rsi, %r14
// Frames: F17 -> F18 -> F19 -> F4
// 0x1403eeeef  0f871b010000                   ja	0x1403ef010 <.text+0x3ee010>
// 0x1403eeef5  4d39c1                         cmpq	%r8, %r9
// 0x1403eeef8  0f8712010000                   ja	0x1403ef010 <.text+0x3ee010>
// Frames: F2 -> F3 -> F20
// 0x1403eeefe  488b02                         movq	(%rdx), %rax
// Frames: F5 -> F3 -> F20
// 0x1403eef01  488b5208                       movq	0x8(%rdx), %rdx
// Frames: F6 -> F7 -> F8 -> F3 -> F20
// 0x1403eef05  4c8b10                         movq	(%rax), %r10
// 0x1403eef08  4531ff                         xorl	%r15d, %r15d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F20
// 0x1403eef0b  4d89d3                         movq	%r10, %r11
// 0x1403eef0e  4c2b5a18                       subq	0x18(%rdx), %r11
// 0x1403eef12  4d0f42df                       cmovbq	%r15, %r11
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F20
// 0x1403eef16  4c2b5238                       subq	0x38(%rdx), %r10
// 0x1403eef1a  4d0f42d7                       cmovbq	%r15, %r10
// Frames: F21 -> F22 -> F23 -> F12 -> F7 -> F8 -> F3 -> F20
// 0x1403eef1e  4c89d9                         movq	%r11, %rcx
// 0x1403eef21  48c1e104                       shlq	$0x4, %rcx
// 0x1403eef25  48034a08                       addq	0x8(%rdx), %rcx
// Frames: F12 -> F7 -> F8 -> F3 -> F20
// 0x1403eef29  4c8b4a10                       movq	0x10(%rdx), %r9
// Frames: F13 -> F7 -> F8 -> F3 -> F20
// 0x1403eef2d  4c8b4230                       movq	0x30(%rdx), %r8
// Frames: F24 -> F12 -> F7 -> F8 -> F3 -> F20
// 0x1403eef31  4d29d9                         subq	%r11, %r9
// 0x1403eef34  4d0f42cf                       cmovbq	%r15, %r9
// 0x1403eef38  4c8d1d7193ee01                 leaq	0x1ee9371(%rip), %r11   # 0x1422d82b0
// 0x1403eef3f  490f42cb                       cmovbq	%r11, %rcx
// Frames: F21 -> F22 -> F23 -> F13 -> F7 -> F8 -> F3 -> F20
// 0x1403eef43  4c89d6                         movq	%r10, %rsi
// 0x1403eef46  48c1e604                       shlq	$0x4, %rsi
// 0x1403eef4a  48037228                       addq	0x28(%rdx), %rsi
// 0x1403eef4e  4d39c2                         cmpq	%r8, %r10
// Frames: F24 -> F13 -> F7 -> F8 -> F3 -> F20
// 0x1403eef51  490f47f3                       cmovaq	%r11, %rsi
// 0x1403eef55  4d29d0                         subq	%r10, %r8
// 0x1403eef58  4d0f42c7                       cmovbq	%r15, %r8
// Frames: F16 -> F7 -> F8 -> F3 -> F20
// 0x1403eef5c  4f8d1408                       leaq	(%r8,%r9), %r10
// Frames: F15 -> F7 -> F8 -> F3 -> F20
// 0x1403eef60  488b5240                       movq	0x40(%rdx), %rdx
// 0x1403eef64  4c29d2                         subq	%r10, %rdx
// 0x1403eef67  488910                         movq	%rdx, (%rax)
// Frames: F25 -> F26 -> F27 -> F28 -> F7 -> F8 -> F3 -> F20
// 0x1403eef6a  49c1e104                       shlq	$0x4, %r9
// 0x1403eef6e  4901c9                         addq	%rcx, %r9
// 0x1403eef71  49c1e004                       shlq	$0x4, %r8
// 0x1403eef75  4901f0                         addq	%rsi, %r8
// Frames: F29 -> F8 -> F3 -> F20
// 0x1403eef78  488945e0                       movq	%rax, -0x20(%rbp)
// 0x1403eef7c  48894de8                       movq	%rcx, -0x18(%rbp)
// 0x1403eef80  4c894df0                       movq	%r9, -0x10(%rbp)
// 0x1403eef84  488975f8                       movq	%rsi, -0x8(%rbp)
// 0x1403eef88  4c894500                       movq	%r8, (%rbp)
// 0x1403eef8c  4c895508                       movq	%r10, 0x8(%rbp)
// 0x1403eef90  488d4da8                       leaq	-0x58(%rbp), %rcx
// 0x1403eef94  488d55e0                       leaq	-0x20(%rbp), %rdx
// Frames: F30 -> F31 -> F20
// 0x1403eef98  e873c34d00                     callq	0x1408cb310 <.text+0x8ca310>
// Frames: F32 -> F33
// 0x1403eef9d  4c8b27                         movq	(%rdi), %r12
// Frames: F34 -> F33
// 0x1403eefa0  4c8b6f08                       movq	0x8(%rdi), %r13
// Frames: F35 -> F36 -> F37 -> F34 -> F33
// 0x1403eefa4  4d8bb424c8000000               movq	0xc8(%r12), %r14
// 0x1403eefac  498b8424d0000000               movq	0xd0(%r12), %rax
// Frames: F25 -> F26 -> F27 -> F35 -> F36 -> F37 -> F34 -> F33
// 0x1403eefb4  498d3486                       leaq	(%r14,%rax,4), %rsi
// 0x1403eefb8  8b431c                         movl	0x1c(%rbx), %eax
// 0x1403eefbb  894514                         movl	%eax, 0x14(%rbp)
// 0x1403eefbe  488b03                         movq	(%rbx), %rax
// 0x1403eefc1  488945d0                       movq	%rax, -0x30(%rbp)
// 0x1403eefc5  488b4310                       movq	0x10(%rbx), %rax
// 0x1403eefc9  488945c8                       movq	%rax, -0x38(%rbp)
// 0x1403eefcd  488b45b0                       movq	-0x50(%rbp), %rax
// 0x1403eefd1  48894538                       movq	%rax, 0x38(%rbp)
// 0x1403eefd5  488b45b8                       movq	-0x48(%rbp), %rax
// 0x1403eefd9  488b95c0000000                 movq	0xc0(%rbp), %rdx
// 0x1403eefe0  8b4a1c                         movl	0x1c(%rdx), %ecx
// 0x1403eefe3  894d10                         movl	%ecx, 0x10(%rbp)
// 0x1403eefe6  4c8b0a                         movq	(%rdx), %r9
// 0x1403eefe9  488b4a10                       movq	0x10(%rdx), %rcx
// 0x1403eefed  48894dc0                       movq	%rcx, -0x40(%rbp)
// 0x1403eeff1  4d8d5120                       leaq	0x20(%r9), %r10
// Frames: F33
// 0x1403eeff5  48c1e003                       shlq	$0x3, %rax
// 0x1403eeff9  48894518                       movq	%rax, 0x18(%rbp)
// 0x1403eeffd  31c9                           xorl	%ecx, %ecx
// 0x1403eefff  31c0                           xorl	%eax, %eax
// 0x1403ef001  48894530                       movq	%rax, 0x30(%rbp)
// 0x1403ef005  31ff                           xorl	%edi, %edi
// 0x1403ef007  4c894d48                       movq	%r9, 0x48(%rbp)
// 0x1403ef00b  e957010000                     jmp	0x1403ef167 <.text+0x3ee167>
// Frames: F38 -> F19 -> F4
// 0x1403ef010  488908                         movq	%rcx, (%rax)
// Frames: F32 -> F39
// 0x1403ef013  488b1f                         movq	(%rdi), %rbx
// Frames: F34 -> F39
// 0x1403ef016  488b7f08                       movq	0x8(%rdi), %rdi
// Frames: F35 -> F36 -> F37 -> F34 -> F39
// 0x1403ef01a  4c8bb3c8000000                 movq	0xc8(%rbx), %r14
// 0x1403ef021  488b83d0000000                 movq	0xd0(%rbx), %rax
// Frames: F25 -> F26 -> F27 -> F35 -> F36 -> F37 -> F34 -> F39
// 0x1403ef028  4d8d3c86                       leaq	(%r14,%rax,4), %r15
// 0x1403ef02c  458b4b1c                       movl	0x1c(%r11), %r9d
// 0x1403ef030  4d8b2b                         movq	(%r11), %r13
// 0x1403ef033  4d8b5310                       movq	0x10(%r11), %r10
// 0x1403ef037  498d4520                       leaq	0x20(%r13), %rax
// 0x1403ef03b  48894520                       movq	%rax, 0x20(%rbp)
// 0x1403ef03f  4531e4                         xorl	%r12d, %r12d
// 0x1403ef042  31f6                           xorl	%esi, %esi
// 0x1403ef044  4531c0                         xorl	%r8d, %r8d
// 0x1403ef047  44894d30                       movl	%r9d, 0x30(%rbp)
// 0x1403ef04b  4c895548                       movq	%r10, 0x48(%rbp)
// 0x1403ef04f  eb34                           jmp	0x1403ef085 <.text+0x3ee085>
// 0x1403ef051  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F45
// 0x1403ef060  498b4528                       movq	0x28(%r13), %rax
// Frames: F46 -> F42 -> F43 -> F44 -> F45
// 0x1403ef064  488d0c52                       leaq	(%rdx,%rdx,2), %rcx
// 0x1403ef068  488b5538                       movq	0x38(%rbp), %rdx
// Frames: F47 -> F48 -> F43 -> F44 -> F45
// 0x1403ef06c  488914c8                       movq	%rdx, (%rax,%rcx,8)
// 0x1403ef070  4c895cc808                     movq	%r11, 0x8(%rax,%rcx,8)
// 0x1403ef075  c644c81000                     movb	$0x0, 0x10(%rax,%rcx,8)
// Frames: F49 -> F43 -> F44 -> F45
// 0x1403ef07a  49ff4530                       incq	0x30(%r13)
// 0x1403ef07e  48ffc6                         incq	%rsi
// Frames: F50 -> F44 -> F45
// 0x1403ef081  49ff4540                       incq	0x40(%r13)
// Frames: F51 -> F52 -> F39
// 0x1403ef085  4c39e6                         cmpq	%r12, %rsi
// 0x1403ef088  7417                           je	0x1403ef0a1 <.text+0x3ee0a1>
// 0x1403ef08a  eb75                           jmp	0x1403ef101 <.text+0x3ee101>
// 0x1403ef08c  0f1f4000                       nopl	(%rax)
// Frames: F53 -> F54 -> F52 -> F39
// 0x1403ef090  4983c604                       addq	$0x4, %r14
// 0x1403ef094  488d0cd1                       leaq	(%rcx,%rdx,8), %rcx
// 0x1403ef098  4c8b6110                       movq	0x10(%rcx), %r12
// Frames: F51 -> F52 -> F39
// 0x1403ef09c  4d85e4                         testq	%r12, %r12
// 0x1403ef09f  753f                           jne	0x1403ef0e0 <.text+0x3ee0e0>
// Frames: F55 -> F56 -> F57 -> F52 -> F39
// 0x1403ef0a1  4d39fe                         cmpq	%r15, %r14
// Frames: F58 -> F57 -> F52 -> F39
// 0x1403ef0a4  0f84a2020000                   je	0x1403ef34c <.text+0x3ee34c>
// Frames: F57 -> F52 -> F39
// 0x1403ef0aa  418b06                         movl	(%r14), %eax
// Frames: F59 -> F60 -> F52 -> F39
// 0x1403ef0ad  488b8f28010000                 movq	0x128(%rdi), %rcx
// Frames: F61 -> F23 -> F59 -> F60 -> F52 -> F39
// 0x1403ef0b4  488d14c0                       leaq	(%rax,%rax,8), %rdx
// Frames: F62 -> F63 -> F64 -> F65 -> F66 -> F52 -> F39
// 0x1403ef0b8  488b44d138                     movq	0x38(%rcx,%rdx,8), %rax
// Frames: F67 -> F68 -> F65 -> F66 -> F52 -> F39
// 0x1403ef0bd  4c8b83e0000000                 movq	0xe0(%rbx), %r8
// Frames: F61 -> F23 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F65 -> F66 -> F52 -> F39
// 0x1403ef0c4  4c3944d140                     cmpq	%r8, 0x40(%rcx,%rdx,8)
// Frames: F74 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F65 -> F66 -> F52 -> F39
// 0x1403ef0c9  76c5                           jbe	0x1403ef090 <.text+0x3ee090>
// Frames: F75 -> F76 -> F77 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F65 -> F66 -> F52 -> F39
// 0x1403ef0cb  4a833cc000                     cmpq	$0x0, (%rax,%r8,8)
// 0x1403ef0d0  ebbe                           jmp	0x1403ef090 <.text+0x3ee090>
// 0x1403ef0d2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1403ef0e0  488b93d8000000                 movq	0xd8(%rbx), %rdx
// 0x1403ef0e7  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x1403ef0eb  48f7d0                         notq	%rax
// 0x1403ef0ee  488b4918                       movq	0x18(%rcx), %rcx
// 0x1403ef0f2  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403ef0f6  48c1e005                       shlq	$0x5, %rax
// 0x1403ef0fa  4c8b440110                     movq	0x10(%rcx,%rax), %r8
// 0x1403ef0ff  31f6                           xorl	%esi, %esi
// Frames: F78 -> F79 -> F80 -> F81 -> F52 -> F39
// 0x1403ef101  89f0                           movl	%esi, %eax
// Frames: F25 -> F82 -> F79 -> F80 -> F81 -> F52 -> F39
// 0x1403ef103  488d0480                       leaq	(%rax,%rax,4), %rax
// 0x1403ef107  48c1e004                       shlq	$0x4, %rax
// Frames: F83
// 0x1403ef10b  4d8b1c00                       movq	(%r8,%rax), %r11
// Frames: F84 -> F85 -> F44 -> F45
// 0x1403ef10f  45890a                         movl	%r9d, (%r10)
// Frames: F86 -> F43 -> F44 -> F45
// 0x1403ef112  498b5530                       movq	0x30(%r13), %rdx
// Frames: F87 -> F44 -> F45
// 0x1403ef116  498b4540                       movq	0x40(%r13), %rax
// 0x1403ef11a  48894538                       movq	%rax, 0x38(%rbp)
// Frames: F86 -> F43 -> F44 -> F45
// 0x1403ef11e  493b5520                       cmpq	0x20(%r13), %rdx
// 0x1403ef122  0f8538ffffff                   jne	0x1403ef060 <.text+0x3ee060>
// Frames: F88 -> F43 -> F44 -> F45
// 0x1403ef128  488b4d20                       movq	0x20(%rbp), %rcx
// 0x1403ef12c  4c894518                       movq	%r8, 0x18(%rbp)
// 0x1403ef130  4c895d40                       movq	%r11, 0x40(%rbp)
// 0x1403ef134  e867b2e8ff                     callq	0x14027a3a0 <.text+0x2793a0>
// 0x1403ef139  4c8b5d40                       movq	0x40(%rbp), %r11
// 0x1403ef13d  4c8b4518                       movq	0x18(%rbp), %r8
// 0x1403ef141  4c8b5548                       movq	0x48(%rbp), %r10
// 0x1403ef145  448b4d30                       movl	0x30(%rbp), %r9d
// Frames: F42 -> F43 -> F44 -> F45
// 0x1403ef149  498b5530                       movq	0x30(%r13), %rdx
// 0x1403ef14d  e90effffff                     jmp	0x1403ef060 <.text+0x3ee060>
// 0x1403ef152  4c8b55d8                       movq	-0x28(%rbp), %r10
// 0x1403ef156  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F89 -> F52 -> F33
// 0x1403ef160  488b4d40                       movq	0x40(%rbp), %rcx
// 0x1403ef164  48ffc1                         incq	%rcx
// Frames: F51 -> F52 -> F33
// 0x1403ef167  48894d40                       movq	%rcx, 0x40(%rbp)
// 0x1403ef16b  4c39f9                         cmpq	%r15, %rcx
// 0x1403ef16e  742a                           je	0x1403ef19a <.text+0x3ee19a>
// Frames: F90 -> F80 -> F81 -> F52 -> F33
// 0x1403ef170  4084ff                         testb	%dil, %dil
// Frames: F91 -> F92 -> F80 -> F81 -> F52 -> F33
// 0x1403ef173  75eb                           jne	0x1403ef160 <.text+0x3ee160>
// 0x1403ef175  e989000000                     jmp	0x1403ef203 <.text+0x3ee203>
// 0x1403ef17a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F75 -> F76 -> F93 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F65 -> F66 -> F52 -> F33
// 0x1403ef180  4a833cc000                     cmpq	$0x0, (%rax,%r8,8)
// 0x1403ef185  400f95c7                       setne	%dil
// Frames: F53 -> F54 -> F52 -> F33
// 0x1403ef189  4983c604                       addq	$0x4, %r14
// 0x1403ef18d  488d0cd1                       leaq	(%rcx,%rdx,8), %rcx
// 0x1403ef191  4c8b7910                       movq	0x10(%rcx), %r15
// Frames: F51 -> F52 -> F33
// 0x1403ef195  4d85ff                         testq	%r15, %r15
// 0x1403ef198  7536                           jne	0x1403ef1d0 <.text+0x3ee1d0>
// Frames: F55 -> F56 -> F57 -> F52 -> F33
// 0x1403ef19a  4939f6                         cmpq	%rsi, %r14
// Frames: F58 -> F57 -> F52 -> F33
// 0x1403ef19d  0f848d010000                   je	0x1403ef330 <.text+0x3ee330>
// Frames: F57 -> F52 -> F33
// 0x1403ef1a3  418b06                         movl	(%r14), %eax
// Frames: F59 -> F60 -> F52 -> F33
// 0x1403ef1a6  498b8d28010000                 movq	0x128(%r13), %rcx
// Frames: F61 -> F23 -> F59 -> F60 -> F52 -> F33
// 0x1403ef1ad  488d14c0                       leaq	(%rax,%rax,8), %rdx
// Frames: F62 -> F63 -> F64 -> F65 -> F66 -> F52 -> F33
// 0x1403ef1b1  488b44d138                     movq	0x38(%rcx,%rdx,8), %rax
// Frames: F67 -> F68 -> F65 -> F66 -> F52 -> F33
// 0x1403ef1b6  4d8b8424e0000000               movq	0xe0(%r12), %r8
// Frames: F61 -> F94 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F65 -> F66 -> F52 -> F33
// 0x1403ef1be  4c3944d140                     cmpq	%r8, 0x40(%rcx,%rdx,8)
// Frames: F95 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F65 -> F66 -> F52 -> F33
// 0x1403ef1c3  77bb                           ja	0x1403ef180 <.text+0x3ee180>
// 0x1403ef1c5  31ff                           xorl	%edi, %edi
// 0x1403ef1c7  ebc0                           jmp	0x1403ef189 <.text+0x3ee189>
// 0x1403ef1c9  0f1f8000000000                 nopl	(%rax)
// Frames: F90 -> F80 -> F81 -> F52 -> F33
// 0x1403ef1d0  498b9424d8000000               movq	0xd8(%r12), %rdx
// 0x1403ef1d8  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x1403ef1dc  48f7d0                         notq	%rax
// 0x1403ef1df  488b4918                       movq	0x18(%rcx), %rcx
// 0x1403ef1e3  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403ef1e7  48c1e005                       shlq	$0x5, %rax
// 0x1403ef1eb  488b440110                     movq	0x10(%rcx,%rax), %rax
// 0x1403ef1f0  48894530                       movq	%rax, 0x30(%rbp)
// 0x1403ef1f4  31c0                           xorl	%eax, %eax
// 0x1403ef1f6  48894540                       movq	%rax, 0x40(%rbp)
// 0x1403ef1fa  4084ff                         testb	%dil, %dil
// Frames: F91 -> F92 -> F80 -> F81 -> F52 -> F33
// 0x1403ef1fd  0f855dffffff                   jne	0x1403ef160 <.text+0x3ee160>
// 0x1403ef203  8b4540                         movl	0x40(%rbp), %eax
// 0x1403ef206  488d0480                       leaq	(%rax,%rax,4), %rax
// 0x1403ef20a  48c1e004                       shlq	$0x4, %rax
// 0x1403ef20e  48034530                       addq	0x30(%rbp), %rax
// 0x1403ef212  48894520                       movq	%rax, 0x20(%rbp)
// 0x1403ef216  488b18                         movq	(%rax), %rbx
// 0x1403ef219  31c0                           xorl	%eax, %eax
// 0x1403ef21b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F55 -> F56 -> F96 -> F97 -> F98 -> F99
// 0x1403ef220  48394518                       cmpq	%rax, 0x18(%rbp)
// Frames: F58 -> F96 -> F97 -> F98 -> F99
// 0x1403ef224  0f8436ffffff                   je	0x1403ef160 <.text+0x3ee160>
// Frames: F100 -> F97 -> F98 -> F99
// 0x1403ef22a  488d4808                       leaq	0x8(%rax), %rcx
// 0x1403ef22e  488b5538                       movq	0x38(%rbp), %rdx
// Frames: F101 -> F102 -> F100 -> F97 -> F98 -> F99
// 0x1403ef232  483b1c02                       cmpq	(%rdx,%rax), %rbx
// 0x1403ef236  4889c8                         movq	%rcx, %rax
// Frames: F100 -> F97 -> F98 -> F99
// 0x1403ef239  75e5                           jne	0x1403ef220 <.text+0x3ee220>
// Frames: F84 -> F85 -> F44 -> F103
// 0x1403ef23b  8b4510                         movl	0x10(%rbp), %eax
// 0x1403ef23e  488b4dc0                       movq	-0x40(%rbp), %rcx
// 0x1403ef242  8901                           movl	%eax, (%rcx)
// 0x1403ef244  488b4548                       movq	0x48(%rbp), %rax
// Frames: F86 -> F43 -> F44 -> F103
// 0x1403ef248  488b5030                       movq	0x30(%rax), %rdx
// Frames: F87 -> F44 -> F103
// 0x1403ef24c  488b4840                       movq	0x40(%rax), %rcx
// 0x1403ef250  48894d28                       movq	%rcx, 0x28(%rbp)
// Frames: F86 -> F43 -> F44 -> F103
// 0x1403ef254  483b5020                       cmpq	0x20(%rax), %rdx
// 0x1403ef258  4c8955d8                       movq	%r10, -0x28(%rbp)
// 0x1403ef25c  7512                           jne	0x1403ef270 <.text+0x3ee270>
// Frames: F88 -> F43 -> F44 -> F103
// 0x1403ef25e  4c89d1                         movq	%r10, %rcx
// 0x1403ef261  e83ab1e8ff                     callq	0x14027a3a0 <.text+0x2793a0>
// Frames: F42 -> F43 -> F44 -> F103
// 0x1403ef266  4c8b4548                       movq	0x48(%rbp), %r8
// 0x1403ef26a  498b5030                       movq	0x30(%r8), %rdx
// 0x1403ef26e  eb04                           jmp	0x1403ef274 <.text+0x3ee274>
// 0x1403ef270  4c8b4548                       movq	0x48(%rbp), %r8
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F103
// 0x1403ef274  498b4028                       movq	0x28(%r8), %rax
// Frames: F46 -> F42 -> F43 -> F44 -> F103
// 0x1403ef278  488d0c52                       leaq	(%rdx,%rdx,2), %rcx
// 0x1403ef27c  488b5528                       movq	0x28(%rbp), %rdx
// Frames: F47 -> F48 -> F43 -> F44 -> F103
// 0x1403ef280  488914c8                       movq	%rdx, (%rax,%rcx,8)
// 0x1403ef284  48895cc808                     movq	%rbx, 0x8(%rax,%rcx,8)
// 0x1403ef289  c644c81000                     movb	$0x0, 0x10(%rax,%rcx,8)
// Frames: F49 -> F43 -> F44 -> F103
// 0x1403ef28e  49ff4030                       incq	0x30(%r8)
// Frames: F50 -> F44 -> F103
// 0x1403ef292  49ff4040                       incq	0x40(%r8)
// 0x1403ef296  8b4514                         movl	0x14(%rbp), %eax
// 0x1403ef299  488b4dc8                       movq	-0x38(%rbp), %rcx
// Frames: F104 -> F85 -> F105
// 0x1403ef29d  8901                           movl	%eax, (%rcx)
// 0x1403ef29f  488b4d20                       movq	0x20(%rbp), %rcx
// Frames: F106
// 0x1403ef2a3  8b4144                         movl	0x44(%rcx), %eax
// 0x1403ef2a6  894528                         movl	%eax, 0x28(%rbp)
// Frames: F105
// 0x1403ef2a9  0f104124                       movups	0x24(%rcx), %xmm0
// 0x1403ef2ad  488b4134                       movq	0x34(%rcx), %rax
// 0x1403ef2b1  8b493c                         movl	0x3c(%rcx), %ecx
// 0x1403ef2b4  894df8                         movl	%ecx, -0x8(%rbp)
// 0x1403ef2b7  488945f0                       movq	%rax, -0x10(%rbp)
// 0x1403ef2bb  0f2945e0                       movaps	%xmm0, -0x20(%rbp)
// Frames: F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F113 -> F105
// 0x1403ef2bf  0fb605fbb78802                 movzbl	0x288b7fb(%rip), %eax   # 0x142c7aac1
// Frames: F114 -> F109 -> F110 -> F111 -> F112 -> F113 -> F105
// 0x1403ef2c6  b950000000                     movl	$0x50, %ecx
// 0x1403ef2cb  ba08000000                     movl	$0x8, %edx
// 0x1403ef2d0  e8abb4feff                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F111 -> F112 -> F113 -> F105
// 0x1403ef2d5  4885c0                         testq	%rax, %rax
// 0x1403ef2d8  0f8483000000                   je	0x1403ef361 <.text+0x3ee361>
// Frames: F112 -> F113 -> F105
// 0x1403ef2de  48c70001000000                 movq	$0x1, (%rax)
// 0x1403ef2e5  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x1403ef2ed  c6401001                       movb	$0x1, 0x10(%rax)
// 0x1403ef2f1  8b4d28                         movl	0x28(%rbp), %ecx
// 0x1403ef2f4  894814                         movl	%ecx, 0x14(%rax)
// 0x1403ef2f7  0f2845e0                       movaps	-0x20(%rbp), %xmm0
// 0x1403ef2fb  0f114018                       movups	%xmm0, 0x18(%rax)
// 0x1403ef2ff  0f1045ec                       movups	-0x14(%rbp), %xmm0
// 0x1403ef303  0f114024                       movups	%xmm0, 0x24(%rax)
// 0x1403ef307  48895838                       movq	%rbx, 0x38(%rax)
// Frames: F105
// 0x1403ef30b  488945e8                       movq	%rax, -0x18(%rbp)
// 0x1403ef30f  48c745e005000000               movq	$0x5, -0x20(%rbp)
// 0x1403ef317  488b4dd0                       movq	-0x30(%rbp), %rcx
// 0x1403ef31b  488d55e0                       leaq	-0x20(%rbp), %rdx
// 0x1403ef31f  4c8d054a88dd01                 leaq	0x1dd884a(%rip), %r8    # 0x1421c7b70
// 0x1403ef326  e835872000                     callq	0x1405f7a60 <.text+0x5f6a60>
// 0x1403ef32b  e922feffff                     jmp	0x1403ef152 <.text+0x3ee152>
// Frames: F115 -> F116
// 0x1403ef330  488b55a8                       movq	-0x58(%rbp), %rdx
// Frames: F117 -> F118 -> F115 -> F115 -> F116
// 0x1403ef334  4885d2                         testq	%rdx, %rdx
// 0x1403ef337  7413                           je	0x1403ef34c <.text+0x3ee34c>
// Frames: F119 -> F120 -> F118 -> F115 -> F115 -> F116
// 0x1403ef339  48c1e203                       shlq	$0x3, %rdx
// Frames: F121 -> F122 -> F123 -> F115 -> F115 -> F116
// 0x1403ef33d  41b808000000                   movl	$0x8, %r8d
// 0x1403ef343  488b4d38                       movq	0x38(%rbp), %rcx
// 0x1403ef347  e844b4feff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F124
// 0x1403ef34c  90                             nop
// 0x1403ef34d  4881c4d8000000                 addq	$0xd8, %rsp
// 0x1403ef354  5b                             popq	%rbx
// 0x1403ef355  5f                             popq	%rdi
// 0x1403ef356  5e                             popq	%rsi
// 0x1403ef357  415c                           popq	%r12
// 0x1403ef359  415d                           popq	%r13
// 0x1403ef35b  415e                           popq	%r14
// 0x1403ef35d  415f                           popq	%r15
// 0x1403ef35f  5d                             popq	%rbp
// 0x1403ef360  c3                             retq
// Frames: F125 -> F112 -> F113 -> F105
// 0x1403ef361  b908000000                     movl	$0x8, %ecx
// 0x1403ef366  ba50000000                     movl	$0x50, %edx
// 0x1403ef36b  e8f0f3cb01                     callq	0x1420ae760 <.text+0x20ad760>
// 0x1403ef370  0f0b                           ud2
// 0x1403ef372  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1403ef380  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403ef385  55                             pushq	%rbp
// 0x1403ef386  4157                           pushq	%r15
// 0x1403ef388  4156                           pushq	%r14
// 0x1403ef38a  4155                           pushq	%r13
// 0x1403ef38c  4154                           pushq	%r12
// 0x1403ef38e  56                             pushq	%rsi
// 0x1403ef38f  57                             pushq	%rdi
// 0x1403ef390  53                             pushq	%rbx
// 0x1403ef391  4883ec28                       subq	$0x28, %rsp
// 0x1403ef395  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F115 -> F116
// 0x1403ef39c  488b55a8                       movq	-0x58(%rbp), %rdx
// Frames: F117 -> F118 -> F115 -> F115 -> F116
// 0x1403ef3a0  4885d2                         testq	%rdx, %rdx
// 0x1403ef3a3  7413                           je	0x1403ef3b8 <.text+0x3ee3b8>
// Frames: F119 -> F120 -> F118 -> F115 -> F115 -> F116
// 0x1403ef3a5  48c1e203                       shlq	$0x3, %rdx
// Frames: F121 -> F122 -> F123 -> F115 -> F115 -> F116
// 0x1403ef3a9  41b808000000                   movl	$0x8, %r8d
// 0x1403ef3af  488b4d38                       movq	0x38(%rbp), %rcx
// 0x1403ef3b3  e8d8b3feff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F1
// 0x1403ef3b8  90                             nop
// 0x1403ef3b9  4883c428                       addq	$0x28, %rsp
// 0x1403ef3bd  5b                             popq	%rbx
// 0x1403ef3be  5f                             popq	%rdi
// 0x1403ef3bf  5e                             popq	%rsi
// 0x1403ef3c0  415c                           popq	%r12
// 0x1403ef3c2  415d                           popq	%r13
// 0x1403ef3c4  415e                           popq	%r14
// 0x1403ef3c6  415f                           popq	%r15
// 0x1403ef3c8  5d                             popq	%rbp
// 0x1403ef3c9  c3                             retq
//
// RANGE rva_3ef3d0_len_45a: RVA 0x3ef3d0, 1114 PDB bytes; 303 instructions
// Evidence: _assembly/roof-v1/A/pack-000/rva_3ef3d0_len_45a.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 264:4828: system_roof::delete_roof::mark_roof_for_deletion
// Frames: F126
// 0x1403ef3d0  55                             pushq	%rbp
// 0x1403ef3d1  4157                           pushq	%r15
// 0x1403ef3d3  4156                           pushq	%r14
// 0x1403ef3d5  4155                           pushq	%r13
// 0x1403ef3d7  4154                           pushq	%r12
// 0x1403ef3d9  56                             pushq	%rsi
// 0x1403ef3da  57                             pushq	%rdi
// 0x1403ef3db  53                             pushq	%rbx
// 0x1403ef3dc  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1403ef3e3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1403ef3eb  48c74540feffffff               movq	$-0x2, 0x40(%rbp)
// 0x1403ef3f3  4c894d28                       movq	%r9, 0x28(%rbp)
// 0x1403ef3f7  4c894530                       movq	%r8, 0x30(%rbp)
// 0x1403ef3fb  48895538                       movq	%rdx, 0x38(%rbp)
// Frames: F127 -> F128
// 0x1403ef3ff  48c7451000000000               movq	$0x0, 0x10(%rbp)
// 0x1403ef407  48c7451808000000               movq	$0x8, 0x18(%rbp)
// 0x1403ef40f  48c7452000000000               movq	$0x0, 0x20(%rbp)
// Frames: F2 -> F3 -> F129
// 0x1403ef417  4c8b31                         movq	(%rcx), %r14
// Frames: F5 -> F3 -> F129
// 0x1403ef41a  4c8b4108                       movq	0x8(%rcx), %r8
// Frames: F6 -> F7 -> F8 -> F3 -> F129
// 0x1403ef41e  4d8b0e                         movq	(%r14), %r9
// 0x1403ef421  31d2                           xorl	%edx, %edx
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F129
// 0x1403ef423  4c89c9                         movq	%r9, %rcx
// 0x1403ef426  492b4818                       subq	0x18(%r8), %rcx
// 0x1403ef42a  480f42ca                       cmovbq	%rdx, %rcx
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F129
// 0x1403ef42e  4d2b4838                       subq	0x38(%r8), %r9
// 0x1403ef432  4c0f42ca                       cmovbq	%rdx, %r9
// Frames: F21 -> F22 -> F23 -> F12 -> F7 -> F8 -> F3 -> F129
// 0x1403ef436  488d0449                       leaq	(%rcx,%rcx,2), %rax
// 0x1403ef43a  48c1e003                       shlq	$0x3, %rax
// 0x1403ef43e  49034008                       addq	0x8(%r8), %rax
// Frames: F12 -> F7 -> F8 -> F3 -> F129
// 0x1403ef442  4d8b5010                       movq	0x10(%r8), %r10
// Frames: F24 -> F12 -> F7 -> F8 -> F3 -> F129
// 0x1403ef446  4929ca                         subq	%rcx, %r10
// 0x1403ef449  4c0f42d2                       cmovbq	%rdx, %r10
// Frames: F13 -> F7 -> F8 -> F3 -> F129
// 0x1403ef44d  4d8b5830                       movq	0x30(%r8), %r11
// Frames: F24 -> F12 -> F7 -> F8 -> F3 -> F129
// 0x1403ef451  488d35588eee01                 leaq	0x1ee8e58(%rip), %rsi   # 0x1422d82b0
// 0x1403ef458  480f42c6                       cmovbq	%rsi, %rax
// Frames: F21 -> F22 -> F23 -> F13 -> F7 -> F8 -> F3 -> F129
// 0x1403ef45c  4b8d0c49                       leaq	(%r9,%r9,2), %rcx
// 0x1403ef460  48c1e103                       shlq	$0x3, %rcx
// 0x1403ef464  49034828                       addq	0x28(%r8), %rcx
// 0x1403ef468  4d39d9                         cmpq	%r11, %r9
// Frames: F24 -> F13 -> F7 -> F8 -> F3 -> F129
// 0x1403ef46b  480f47ce                       cmovaq	%rsi, %rcx
// 0x1403ef46f  4d29cb                         subq	%r9, %r11
// 0x1403ef472  4c0f42da                       cmovbq	%rdx, %r11
// Frames: F15 -> F7 -> F8 -> F3 -> F129
// 0x1403ef476  4d8b4040                       movq	0x40(%r8), %r8
// 0x1403ef47a  4d29d8                         subq	%r11, %r8
// 0x1403ef47d  4d29d0                         subq	%r10, %r8
// 0x1403ef480  4d8906                         movq	%r8, (%r14)
// Frames: F25 -> F26 -> F27 -> F28 -> F7 -> F8 -> F3 -> F129
// 0x1403ef483  4f8d0452                       leaq	(%r10,%r10,2), %r8
// 0x1403ef487  4e8d24c0                       leaq	(%rax,%r8,8), %r12
// 0x1403ef48b  4f8d045b                       leaq	(%r11,%r11,2), %r8
// 0x1403ef48f  4e8d2cc1                       leaq	(%rcx,%r8,8), %r13
// 0x1403ef493  488d7d10                       leaq	0x10(%rbp), %rdi
// 0x1403ef497  eb20                           jmp	0x1403ef4b9 <.text+0x3ee4b9>
// 0x1403ef499  0f1f8000000000                 nopl	(%rax)
// Frames: F40 -> F41 -> F42 -> F130
// 0x1403ef4a0  488b4518                       movq	0x18(%rbp), %rax
// Frames: F47 -> F48 -> F130
// 0x1403ef4a4  48891cd0                       movq	%rbx, (%rax,%rdx,8)
// Frames: F49 -> F130
// 0x1403ef4a8  488b5520                       movq	0x20(%rbp), %rdx
// 0x1403ef4ac  48ffc2                         incq	%rdx
// 0x1403ef4af  48895520                       movq	%rdx, 0x20(%rbp)
// 0x1403ef4b3  4c89f8                         movq	%r15, %rax
// 0x1403ef4b6  4889f1                         movq	%rsi, %rcx
// Frames: F131 -> F132 -> F133 -> F134 -> F135 -> F129
// 0x1403ef4b9  4885c0                         testq	%rax, %rax
// 0x1403ef4bc  7422                           je	0x1403ef4e0 <.text+0x3ee4e0>
// 0x1403ef4be  4c39e0                         cmpq	%r12, %rax
// 0x1403ef4c1  741d                           je	0x1403ef4e0 <.text+0x3ee4e0>
// Frames: F136 -> F135 -> F129
// 0x1403ef4c3  4c8d7818                       leaq	0x18(%rax), %r15
// 0x1403ef4c7  4889ce                         movq	%rcx, %rsi
// 0x1403ef4ca  49ff06                         incq	(%r14)
// Frames: F130
// 0x1403ef4cd  488b5808                       movq	0x8(%rax), %rbx
// Frames: F86 -> F130
// 0x1403ef4d1  483b5510                       cmpq	0x10(%rbp), %rdx
// 0x1403ef4d5  75c9                           jne	0x1403ef4a0 <.text+0x3ee4a0>
// 0x1403ef4d7  eb28                           jmp	0x1403ef501 <.text+0x3ee501>
// 0x1403ef4d9  0f1f8000000000                 nopl	(%rax)
// Frames: F131 -> F137 -> F138 -> F133 -> F134 -> F135 -> F129
// 0x1403ef4e0  4885c9                         testq	%rcx, %rcx
// 0x1403ef4e3  742a                           je	0x1403ef50f <.text+0x3ee50f>
// 0x1403ef4e5  4c39e9                         cmpq	%r13, %rcx
// 0x1403ef4e8  7425                           je	0x1403ef50f <.text+0x3ee50f>
// Frames: F139 -> F140 -> F141 -> F137 -> F138 -> F133 -> F134 -> F135 -> F129
// 0x1403ef4ea  488d7118                       leaq	0x18(%rcx), %rsi
// 0x1403ef4ee  4531ff                         xorl	%r15d, %r15d
// 0x1403ef4f1  4889c8                         movq	%rcx, %rax
// Frames: F136 -> F135 -> F129
// 0x1403ef4f4  49ff06                         incq	(%r14)
// Frames: F130
// 0x1403ef4f7  488b5808                       movq	0x8(%rax), %rbx
// Frames: F86 -> F130
// 0x1403ef4fb  483b5510                       cmpq	0x10(%rbp), %rdx
// 0x1403ef4ff  759f                           jne	0x1403ef4a0 <.text+0x3ee4a0>
// Frames: F88 -> F130
// 0x1403ef501  4889f9                         movq	%rdi, %rcx
// 0x1403ef504  e857a7e8ff                     callq	0x140279c60 <.text+0x278c60>
// Frames: F42 -> F130
// 0x1403ef509  488b5520                       movq	0x20(%rbp), %rdx
// 0x1403ef50d  eb91                           jmp	0x1403ef4a0 <.text+0x3ee4a0>
// Frames: F142 -> F143
// 0x1403ef50f  488b4d38                       movq	0x38(%rbp), %rcx
// 0x1403ef513  488b01                         movq	(%rcx), %rax
// Frames: F144 -> F143
// 0x1403ef516  488b7108                       movq	0x8(%rcx), %rsi
// Frames: F35 -> F36 -> F37 -> F144 -> F143
// 0x1403ef51a  488bb8c8000000                 movq	0xc8(%rax), %rdi
// 0x1403ef521  488945d8                       movq	%rax, -0x28(%rbp)
// 0x1403ef525  488b80d0000000                 movq	0xd0(%rax), %rax
// Frames: F25 -> F26 -> F27 -> F35 -> F36 -> F37 -> F144 -> F143
// 0x1403ef52c  4c8d3487                       leaq	(%rdi,%rax,4), %r14
// 0x1403ef530  488b4d30                       movq	0x30(%rbp), %rcx
// 0x1403ef534  488b01                         movq	(%rcx), %rax
// 0x1403ef537  488945c8                       movq	%rax, -0x38(%rbp)
// 0x1403ef53b  4c8b5108                       movq	0x8(%rcx), %r10
// 0x1403ef53f  488b4110                       movq	0x10(%rcx), %rax
// 0x1403ef543  488945e8                       movq	%rax, -0x18(%rbp)
// 0x1403ef547  488b4d28                       movq	0x28(%rbp), %rcx
// 0x1403ef54b  8b411c                         movl	0x1c(%rcx), %eax
// 0x1403ef54e  8945fc                         movl	%eax, -0x4(%rbp)
// 0x1403ef551  4c8b09                         movq	(%rcx), %r9
// 0x1403ef554  488b4110                       movq	0x10(%rcx), %rax
// 0x1403ef558  488945d0                       movq	%rax, -0x30(%rbp)
// 0x1403ef55c  498d4120                       leaq	0x20(%r9), %rax
// 0x1403ef560  488945e0                       movq	%rax, -0x20(%rbp)
// 0x1403ef564  488d05a593ee01                 leaq	0x1ee93a5(%rip), %rax   # 0x1422d8910
// 0x1403ef56b  48894530                       movq	%rax, 0x30(%rbp)
// 0x1403ef56f  31c0                           xorl	%eax, %eax
// 0x1403ef571  48894538                       movq	%rax, 0x38(%rbp)
// 0x1403ef575  31db                           xorl	%ebx, %ebx
// 0x1403ef577  31c9                           xorl	%ecx, %ecx
// 0x1403ef579  4c895508                       movq	%r10, 0x8(%rbp)
// 0x1403ef57d  4c894d00                       movq	%r9, (%rbp)
// Frames: F51 -> F52 -> F143
// 0x1403ef581  4839d9                         cmpq	%rbx, %rcx
// 0x1403ef584  7578                           jne	0x1403ef5fe <.text+0x3ee5fe>
// 0x1403ef586  eb18                           jmp	0x1403ef5a0 <.text+0x3ee5a0>
// 0x1403ef588  488b4d28                       movq	0x28(%rbp), %rcx
// 0x1403ef58c  4839d9                         cmpq	%rbx, %rcx
// 0x1403ef58f  756d                           jne	0x1403ef5fe <.text+0x3ee5fe>
// 0x1403ef591  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F55 -> F56 -> F57 -> F52 -> F143
// 0x1403ef5a0  4c39f7                         cmpq	%r14, %rdi
// Frames: F58 -> F57 -> F52 -> F143
// 0x1403ef5a3  0f84f0010000                   je	0x1403ef799 <.text+0x3ee799>
// Frames: F57 -> F52 -> F143
// 0x1403ef5a9  8b0f                           movl	(%rdi), %ecx
// Frames: F139 -> F140 -> F141 -> F57 -> F52 -> F143
// 0x1403ef5ab  4883c704                       addq	$0x4, %rdi
// Frames: F59 -> F60 -> F52 -> F143
// 0x1403ef5af  488b8628010000                 movq	0x128(%rsi), %rax
// Frames: F61 -> F23 -> F59 -> F60 -> F52 -> F143
// 0x1403ef5b6  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F53 -> F54 -> F52 -> F143
// 0x1403ef5ba  488b5cc810                     movq	0x10(%rax,%rcx,8), %rbx
// Frames: F51 -> F52 -> F143
// 0x1403ef5bf  4885db                         testq	%rbx, %rbx
// 0x1403ef5c2  74dc                           je	0x1403ef5a0 <.text+0x3ee5a0>
// 0x1403ef5c4  488d14c8                       leaq	(%rax,%rcx,8), %rdx
// 0x1403ef5c8  4c8b45d8                       movq	-0x28(%rbp), %r8
// 0x1403ef5cc  4d8b80d8000000                 movq	0xd8(%r8), %r8
// 0x1403ef5d3  488b44c838                     movq	0x38(%rax,%rcx,8), %rax
// 0x1403ef5d8  4a8b04c0                       movq	(%rax,%r8,8), %rax
// 0x1403ef5dc  48f7d0                         notq	%rax
// 0x1403ef5df  488b4a08                       movq	0x8(%rdx), %rcx
// 0x1403ef5e3  48894d30                       movq	%rcx, 0x30(%rbp)
// 0x1403ef5e7  488b4a18                       movq	0x18(%rdx), %rcx
// 0x1403ef5eb  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403ef5ef  48c1e005                       shlq	$0x5, %rax
// 0x1403ef5f3  488b440110                     movq	0x10(%rcx,%rax), %rax
// 0x1403ef5f8  48894538                       movq	%rax, 0x38(%rbp)
// 0x1403ef5fc  31c9                           xorl	%ecx, %ecx
// Frames: F81 -> F52 -> F143
// 0x1403ef5fe  488b4530                       movq	0x30(%rbp), %rax
// 0x1403ef602  4c8b2cc8                       movq	(%rax,%rcx,8), %r13
// Frames: F143
// 0x1403ef606  4c89e8                         movq	%r13, %rax
// 0x1403ef609  48c1e820                       shrq	$0x20, %rax
// 0x1403ef60d  0f8486010000                   je	0x1403ef799 <.text+0x3ee799>
// Frames: F145
// 0x1403ef613  89c8                           movl	%ecx, %eax
// 0x1403ef615  4c8d3c80                       leaq	(%rax,%rax,4), %r15
// 0x1403ef619  49c1e704                       shlq	$0x4, %r15
// 0x1403ef61d  4c037d38                       addq	0x38(%rbp), %r15
// 0x1403ef621  48ffc1                         incq	%rcx
// 0x1403ef624  48894d28                       movq	%rcx, 0x28(%rbp)
// 0x1403ef628  488b4518                       movq	0x18(%rbp), %rax
// 0x1403ef62c  488b4d20                       movq	0x20(%rbp), %rcx
// 0x1403ef630  498b17                         movq	(%r15), %rdx
// Frames: F96 -> F97 -> F98 -> F145
// 0x1403ef633  48c1e103                       shlq	$0x3, %rcx
// 0x1403ef637  4531c0                         xorl	%r8d, %r8d
// 0x1403ef63a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F55 -> F56 -> F96 -> F97 -> F98 -> F145
// 0x1403ef640  4c39c1                         cmpq	%r8, %rcx
// Frames: F58 -> F96 -> F97 -> F98 -> F145
// 0x1403ef643  0f843fffffff                   je	0x1403ef588 <.text+0x3ee588>
// Frames: F100 -> F97 -> F98 -> F145
// 0x1403ef649  4d8d4808                       leaq	0x8(%r8), %r9
// Frames: F101 -> F102 -> F100 -> F97 -> F98 -> F145
// 0x1403ef64d  4a3b1400                       cmpq	(%rax,%r8), %rdx
// 0x1403ef651  4d89c8                         movq	%r9, %r8
// Frames: F100 -> F97 -> F98 -> F145
// 0x1403ef654  75ea                           jne	0x1403ef640 <.text+0x3ee640>
// Frames: F146 -> F147 -> F148
// 0x1403ef656  488b4dc8                       movq	-0x38(%rbp), %rcx
// 0x1403ef65a  4c89ea                         movq	%r13, %rdx
// 0x1403ef65d  e80e4a4300                     callq	0x140824070 <.text+0x823070>
// Frames: F149 -> F146 -> F147 -> F148
// 0x1403ef662  84c0                           testb	%al, %al
// 0x1403ef664  0f8460010000                   je	0x1403ef7ca <.text+0x3ee7ca>
// Frames: F150 -> F151 -> F152 -> F153 -> F148
// 0x1403ef66a  488b4d08                       movq	0x8(%rbp), %rcx
// 0x1403ef66e  4885c9                         testq	%rcx, %rcx
// 0x1403ef671  7444                           je	0x1403ef6b7 <.text+0x3ee6b7>
// Frames: F154 -> F155 -> F156 -> F157 -> F158 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef673  488b01                         movq	(%rcx), %rax
// Frames: F160 -> F161 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef676  4c8b6110                       movq	0x10(%rcx), %r12
// Frames: F162 -> F155 -> F156 -> F157 -> F158 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef67a  4c29e0                         subq	%r12, %rax
// Frames: F155 -> F156 -> F157 -> F158 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef67d  4883f80f                       cmpq	$0xf, %rax
// Frames: F156 -> F157 -> F158 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef681  7712                           ja	0x1403ef695 <.text+0x3ee695>
// Frames: F163 -> F157 -> F158 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef683  41b810000000                   movl	$0x10, %r8d
// 0x1403ef689  488b4d08                       movq	0x8(%rbp), %rcx
// 0x1403ef68d  4c89e2                         movq	%r12, %rdx
// 0x1403ef690  e80b0acb01                     callq	0x1420a00a0 <.text+0x209f0a0>
// Frames: F40 -> F41 -> F164 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef695  488b4d08                       movq	0x8(%rbp), %rcx
// 0x1403ef699  488b4108                       movq	0x8(%rcx), %rax
// Frames: F165 -> F166 -> F167 -> F168 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef69d  488d157cc94d00                 leaq	0x4dc97c(%rip), %rdx    # 0x1408cc020 <.text+0x8cb020>
// 0x1403ef6a4  4a891420                       movq	%rdx, (%rax,%r12)
// 0x1403ef6a8  4e896c2008                     movq	%r13, 0x8(%rax,%r12)
// Frames: F169 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6ad  4983c410                       addq	$0x10, %r12
// Frames: F170 -> F169 -> F159 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6b1  4c896110                       movq	%r12, 0x10(%rcx)
// 0x1403ef6b5  eb56                           jmp	0x1403ef70d <.text+0x3ee70d>
// Frames: F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6b7  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x1403ef6bb  488b55e8                       movq	-0x18(%rbp), %rdx
// 0x1403ef6bf  e87c4e4300                     callq	0x140824540 <.text+0x823540>
// 0x1403ef6c4  488b55a0                       movq	-0x60(%rbp), %rdx
// Frames: F154 -> F155 -> F156 -> F157 -> F158 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6c8  488b02                         movq	(%rdx), %rax
// Frames: F160 -> F161 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6cb  4c8b6210                       movq	0x10(%rdx), %r12
// Frames: F162 -> F155 -> F156 -> F157 -> F158 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6cf  4c29e0                         subq	%r12, %rax
// Frames: F155 -> F156 -> F157 -> F158 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6d2  4883f80f                       cmpq	$0xf, %rax
// Frames: F156 -> F157 -> F158 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6d6  7719                           ja	0x1403ef6f1 <.text+0x3ee6f1>
// Frames: F163 -> F157 -> F158 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6d8  41b810000000                   movl	$0x10, %r8d
// 0x1403ef6de  4889d1                         movq	%rdx, %rcx
// 0x1403ef6e1  488955f0                       movq	%rdx, -0x10(%rbp)
// 0x1403ef6e5  4c89e2                         movq	%r12, %rdx
// 0x1403ef6e8  e8b309cb01                     callq	0x1420a00a0 <.text+0x209f0a0>
// 0x1403ef6ed  488b55f0                       movq	-0x10(%rbp), %rdx
// Frames: F40 -> F41 -> F164 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6f1  488b4208                       movq	0x8(%rdx), %rax
// Frames: F165 -> F166 -> F167 -> F168 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef6f5  488d0d24c94d00                 leaq	0x4dc924(%rip), %rcx    # 0x1408cc020 <.text+0x8cb020>
// 0x1403ef6fc  4a890c20                       movq	%rcx, (%rax,%r12)
// 0x1403ef700  4e896c2008                     movq	%r13, 0x8(%rax,%r12)
// Frames: F169 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef705  4983c410                       addq	$0x10, %r12
// Frames: F170 -> F169 -> F171 -> F172 -> F151 -> F152 -> F153 -> F148
// 0x1403ef709  4c896210                       movq	%r12, 0x10(%rdx)
// Frames: F173
// 0x1403ef70d  488d4dbc                       leaq	-0x44(%rbp), %rcx
// 0x1403ef711  4c89fa                         movq	%r15, %rdx
// 0x1403ef714  e8d7ac2c00                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F174
// 0x1403ef719  8b45c4                         movl	-0x3c(%rbp), %eax
// 0x1403ef71c  488d4daf                       leaq	-0x51(%rbp), %rcx
// 0x1403ef720  894108                         movl	%eax, 0x8(%rcx)
// 0x1403ef723  488b45bc                       movq	-0x44(%rbp), %rax
// 0x1403ef727  488901                         movq	%rax, (%rcx)
// 0x1403ef72a  8b45fc                         movl	-0x4(%rbp), %eax
// 0x1403ef72d  488b4dd0                       movq	-0x30(%rbp), %rcx
// Frames: F84 -> F85 -> F44 -> F174
// 0x1403ef731  8901                           movl	%eax, (%rcx)
// 0x1403ef733  488b4500                       movq	(%rbp), %rax
// Frames: F86 -> F43 -> F44 -> F174
// 0x1403ef737  488b5030                       movq	0x30(%rax), %rdx
// Frames: F87 -> F44 -> F174
// 0x1403ef73b  4c8b7840                       movq	0x40(%rax), %r15
// Frames: F86 -> F43 -> F44 -> F174
// 0x1403ef73f  483b5020                       cmpq	0x20(%rax), %rdx
// 0x1403ef743  7513                           jne	0x1403ef758 <.text+0x3ee758>
// Frames: F88 -> F43 -> F44 -> F174
// 0x1403ef745  488b4de0                       movq	-0x20(%rbp), %rcx
// 0x1403ef749  e8c2ade8ff                     callq	0x14027a510 <.text+0x279510>
// Frames: F42 -> F43 -> F44 -> F174
// 0x1403ef74e  4c8b4d00                       movq	(%rbp), %r9
// 0x1403ef752  498b5130                       movq	0x30(%r9), %rdx
// 0x1403ef756  eb04                           jmp	0x1403ef75c <.text+0x3ee75c>
// 0x1403ef758  4c8b4d00                       movq	(%rbp), %r9
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F174
// 0x1403ef75c  498b4128                       movq	0x28(%r9), %rax
// Frames: F46 -> F42 -> F43 -> F44 -> F174
// 0x1403ef760  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F47 -> F48 -> F43 -> F44 -> F174
// 0x1403ef764  4c893cc8                       movq	%r15, (%rax,%rcx,8)
// 0x1403ef768  c644c80801                     movb	$0x1, 0x8(%rax,%rcx,8)
// 0x1403ef76d  0f1045a0                       movups	-0x60(%rbp), %xmm0
// 0x1403ef771  0f104dab                       movups	-0x55(%rbp), %xmm1
// 0x1403ef775  0f1144c809                     movups	%xmm0, 0x9(%rax,%rcx,8)
// 0x1403ef77a  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// Frames: F49 -> F43 -> F44 -> F174
// 0x1403ef77f  49ff4130                       incq	0x30(%r9)
// Frames: F50 -> F44 -> F174
// 0x1403ef783  49ff4140                       incq	0x40(%r9)
// 0x1403ef787  488b4d28                       movq	0x28(%rbp), %rcx
// Frames: F51 -> F52 -> F143
// 0x1403ef78b  4839d9                         cmpq	%rbx, %rcx
// 0x1403ef78e  0f840cfeffff                   je	0x1403ef5a0 <.text+0x3ee5a0>
// 0x1403ef794  e965feffff                     jmp	0x1403ef5fe <.text+0x3ee5fe>
// Frames: F115 -> F175
// 0x1403ef799  488b5510                       movq	0x10(%rbp), %rdx
// Frames: F117 -> F118 -> F115 -> F115 -> F175
// 0x1403ef79d  4885d2                         testq	%rdx, %rdx
// 0x1403ef7a0  7413                           je	0x1403ef7b5 <.text+0x3ee7b5>
// Frames: F115 -> F175
// 0x1403ef7a2  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F119 -> F120 -> F118 -> F115 -> F115 -> F175
// 0x1403ef7a6  48c1e203                       shlq	$0x3, %rdx
// Frames: F121 -> F122 -> F123 -> F115 -> F115 -> F175
// 0x1403ef7aa  41b808000000                   movl	$0x8, %r8d
// 0x1403ef7b0  e8dbaffeff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F175
// 0x1403ef7b5  90                             nop
// 0x1403ef7b6  4881c4c8000000                 addq	$0xc8, %rsp
// 0x1403ef7bd  5b                             popq	%rbx
// 0x1403ef7be  5f                             popq	%rdi
// 0x1403ef7bf  5e                             popq	%rsi
// 0x1403ef7c0  415c                           popq	%r12
// 0x1403ef7c2  415d                           popq	%r13
// 0x1403ef7c4  415e                           popq	%r14
// 0x1403ef7c6  415f                           popq	%r15
// 0x1403ef7c8  5d                             popq	%rbp
// 0x1403ef7c9  c3                             retq
// Frames: F176 -> F148
// 0x1403ef7ca  488d15b783dd01                 leaq	0x1dd83b7(%rip), %rdx   # 0x1421c7b88
// 0x1403ef7d1  4c89e9                         movq	%r13, %rcx
// 0x1403ef7d4  e8d7d7cb01                     callq	0x1420acfb0 <.text+0x20abfb0>
// 0x1403ef7d9  0f0b                           ud2
// 0x1403ef7db  0f1f440000                     nopl	(%rax,%rax)
// 0x1403ef7e0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403ef7e5  55                             pushq	%rbp
// 0x1403ef7e6  4157                           pushq	%r15
// 0x1403ef7e8  4156                           pushq	%r14
// 0x1403ef7ea  4155                           pushq	%r13
// 0x1403ef7ec  4154                           pushq	%r12
// 0x1403ef7ee  56                             pushq	%rsi
// 0x1403ef7ef  57                             pushq	%rdi
// 0x1403ef7f0  53                             pushq	%rbx
// 0x1403ef7f1  4883ec28                       subq	$0x28, %rsp
// 0x1403ef7f5  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F115 -> F175
// 0x1403ef7fc  488b5510                       movq	0x10(%rbp), %rdx
// Frames: F117 -> F118 -> F115 -> F115 -> F175
// 0x1403ef800  4885d2                         testq	%rdx, %rdx
// 0x1403ef803  7413                           je	0x1403ef818 <.text+0x3ee818>
// Frames: F115 -> F175
// 0x1403ef805  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F119 -> F120 -> F118 -> F115 -> F115 -> F175
// 0x1403ef809  48c1e203                       shlq	$0x3, %rdx
// Frames: F121 -> F122 -> F123 -> F115 -> F115 -> F175
// 0x1403ef80d  41b808000000                   movl	$0x8, %r8d
// 0x1403ef813  e878affeff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F126
// 0x1403ef818  90                             nop
// 0x1403ef819  4883c428                       addq	$0x28, %rsp
// 0x1403ef81d  5b                             popq	%rbx
// 0x1403ef81e  5f                             popq	%rdi
// 0x1403ef81f  5e                             popq	%rsi
// 0x1403ef820  415c                           popq	%r12
// 0x1403ef822  415d                           popq	%r13
// 0x1403ef824  415e                           popq	%r14
// 0x1403ef826  415f                           popq	%r15
// 0x1403ef828  5d                             popq	%rbp
// 0x1403ef829  c3                             retq
//
// RANGE rva_3ef830_len_1ac: RVA 0x3ef830, 428 PDB bytes; 112 instructions
// Evidence: _assembly/roof-v1/A/pack-000/rva_3ef830_len_1ac.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 264:8468: system_roof::delete_roof::delete_roofs
// Frames: F177
// 0x1403ef830  4157                           pushq	%r15
// 0x1403ef832  4156                           pushq	%r14
// 0x1403ef834  4155                           pushq	%r13
// 0x1403ef836  4154                           pushq	%r12
// 0x1403ef838  56                             pushq	%rsi
// 0x1403ef839  57                             pushq	%rdi
// 0x1403ef83a  55                             pushq	%rbp
// 0x1403ef83b  53                             pushq	%rbx
// 0x1403ef83c  4881ec98000000                 subq	$0x98, %rsp
// 0x1403ef843  0f29b42480000000               movaps	%xmm6, 0x80(%rsp)
// Frames: F142 -> F178
// 0x1403ef84b  4c8b31                         movq	(%rcx), %r14
// Frames: F144 -> F178
// 0x1403ef84e  4c8b7908                       movq	0x8(%rcx), %r15
// Frames: F35 -> F36 -> F37 -> F144 -> F178
// 0x1403ef852  4d8ba6c8000000                 movq	0xc8(%r14), %r12
// 0x1403ef859  498b86d0000000                 movq	0xd0(%r14), %rax
// Frames: F25 -> F26 -> F27 -> F35 -> F36 -> F37 -> F144 -> F178
// 0x1403ef860  4d8d2c84                       leaq	(%r12,%rax,4), %r13
// 0x1403ef864  488b1a                         movq	(%rdx), %rbx
// 0x1403ef867  488b4208                       movq	0x8(%rdx), %rax
// 0x1403ef86b  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1403ef870  488b4210                       movq	0x10(%rdx), %rax
// 0x1403ef874  4889442438                     movq	%rax, 0x38(%rsp)
// 0x1403ef879  488b4218                       movq	0x18(%rdx), %rax
// 0x1403ef87d  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1403ef882  4c8d0d8790ee01                 leaq	0x1ee9087(%rip), %r9    # 0x1422d8910
// 0x1403ef889  31d2                           xorl	%edx, %edx
// 0x1403ef88b  0f57f6                         xorps	%xmm6, %xmm6
// 0x1403ef88e  31ff                           xorl	%edi, %edi
// 0x1403ef890  31f6                           xorl	%esi, %esi
// 0x1403ef892  31ed                           xorl	%ebp, %ebp
// Frames: F51 -> F52 -> F178
// 0x1403ef894  4839f5                         cmpq	%rsi, %rbp
// 0x1403ef897  750a                           jne	0x1403ef8a3 <.text+0x3ee8a3>
// 0x1403ef899  eb2f                           jmp	0x1403ef8ca <.text+0x3ee8ca>
// Frames: F89 -> F52 -> F178
// 0x1403ef89b  48ffc5                         incq	%rbp
// Frames: F51 -> F52 -> F178
// 0x1403ef89e  4839f5                         cmpq	%rsi, %rbp
// 0x1403ef8a1  7427                           je	0x1403ef8ca <.text+0x3ee8ca>
// Frames: F90 -> F179 -> F81 -> F52 -> F178
// 0x1403ef8a3  4084ff                         testb	%dil, %dil
// 0x1403ef8a6  757f                           jne	0x1403ef927 <.text+0x3ee927>
// 0x1403ef8a8  ebf1                           jmp	0x1403ef89b <.text+0x3ee89b>
// 0x1403ef8aa  660f1f440000                   nopw	(%rax,%rax)
// Frames: F75 -> F76 -> F93 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F180 -> F66 -> F52 -> F178
// 0x1403ef8b0  4a833cc000                     cmpq	$0x0, (%rax,%r8,8)
// 0x1403ef8b5  400f95c7                       setne	%dil
// Frames: F53 -> F54 -> F52 -> F178
// 0x1403ef8b9  4983c404                       addq	$0x4, %r12
// 0x1403ef8bd  4d8d04d1                       leaq	(%r9,%rdx,8), %r8
// 0x1403ef8c1  498b7010                       movq	0x10(%r8), %rsi
// Frames: F51 -> F52 -> F178
// 0x1403ef8c5  4885f6                         testq	%rsi, %rsi
// 0x1403ef8c8  752f                           jne	0x1403ef8f9 <.text+0x3ee8f9>
// Frames: F55 -> F56 -> F57 -> F52 -> F178
// 0x1403ef8ca  4d39ec                         cmpq	%r13, %r12
// Frames: F58 -> F57 -> F52 -> F178
// 0x1403ef8cd  0f84db000000                   je	0x1403ef9ae <.text+0x3ee9ae>
// Frames: F57 -> F52 -> F178
// 0x1403ef8d3  418b0424                       movl	(%r12), %eax
// Frames: F59 -> F60 -> F52 -> F178
// 0x1403ef8d7  4d8b8f28010000                 movq	0x128(%r15), %r9
// Frames: F61 -> F23 -> F59 -> F60 -> F52 -> F178
// 0x1403ef8de  488d14c0                       leaq	(%rax,%rax,8), %rdx
// Frames: F62 -> F63 -> F64 -> F180 -> F66 -> F52 -> F178
// 0x1403ef8e2  498b44d138                     movq	0x38(%r9,%rdx,8), %rax
// Frames: F67 -> F68 -> F180 -> F66 -> F52 -> F178
// 0x1403ef8e7  4d8b86e0000000                 movq	0xe0(%r14), %r8
// Frames: F61 -> F94 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F180 -> F66 -> F52 -> F178
// 0x1403ef8ee  4d3944d140                     cmpq	%r8, 0x40(%r9,%rdx,8)
// Frames: F95 -> F69 -> F70 -> F71 -> F72 -> F73 -> F68 -> F180 -> F66 -> F52 -> F178
// 0x1403ef8f3  77bb                           ja	0x1403ef8b0 <.text+0x3ee8b0>
// 0x1403ef8f5  31ff                           xorl	%edi, %edi
// 0x1403ef8f7  ebc0                           jmp	0x1403ef8b9 <.text+0x3ee8b9>
// Frames: F53 -> F54 -> F52 -> F178
// 0x1403ef8f9  498b96e8000000                 movq	0xe8(%r14), %rdx
// 0x1403ef900  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x1403ef904  48f7d0                         notq	%rax
// 0x1403ef907  4d8b4808                       movq	0x8(%r8), %r9
// 0x1403ef90b  498b5018                       movq	0x18(%r8), %rdx
// Frames: F181 -> F182 -> F183 -> F180 -> F66 -> F52 -> F178
// 0x1403ef90f  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403ef913  48c1e005                       shlq	$0x5, %rax
// 0x1403ef917  488b540210                     movq	0x10(%rdx,%rax), %rdx
// 0x1403ef91c  31ed                           xorl	%ebp, %ebp
// Frames: F90 -> F179 -> F81 -> F52 -> F178
// 0x1403ef91e  4084ff                         testb	%dil, %dil
// Frames: F91 -> F92 -> F179 -> F81 -> F52 -> F178
// 0x1403ef921  0f8474ffffff                   je	0x1403ef89b <.text+0x3ee89b>
// Frames: F78 -> F79 -> F179 -> F81 -> F52 -> F178
// 0x1403ef927  89e8                           movl	%ebp, %eax
// Frames: F184
// 0x1403ef929  f30f1004c2                     movss	(%rdx,%rax,8), %xmm0
// 0x1403ef92e  0f2ec6                         ucomiss	%xmm6, %xmm0
// 0x1403ef931  0f8564ffffff                   jne	0x1403ef89b <.text+0x3ee89b>
// 0x1403ef937  0f8a5effffff                   jp	0x1403ef89b <.text+0x3ee89b>
// Frames: F146 -> F147 -> F185
// 0x1403ef93d  4889542448                     movq	%rdx, 0x48(%rsp)
// 0x1403ef942  4c894c2450                     movq	%r9, 0x50(%rsp)
// 0x1403ef947  498b14e9                       movq	(%r9,%rbp,8), %rdx
// 0x1403ef94b  4889d9                         movq	%rbx, %rcx
// 0x1403ef94e  4889542428                     movq	%rdx, 0x28(%rsp)
// 0x1403ef953  e818474300                     callq	0x140824070 <.text+0x823070>
// Frames: F149 -> F146 -> F147 -> F185
// 0x1403ef958  84c0                           testb	%al, %al
// 0x1403ef95a  746e                           je	0x1403ef9ca <.text+0x3ee9ca>
// Frames: F186 -> F185
// 0x1403ef95c  48895c2458                     movq	%rbx, 0x58(%rsp)
// 0x1403ef961  488b442440                     movq	0x40(%rsp), %rax
// 0x1403ef966  4889442460                     movq	%rax, 0x60(%rsp)
// 0x1403ef96b  488b442438                     movq	0x38(%rsp), %rax
// 0x1403ef970  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1403ef975  488b442430                     movq	0x30(%rsp), %rax
// 0x1403ef97a  4889442470                     movq	%rax, 0x70(%rsp)
// 0x1403ef97f  488b442428                     movq	0x28(%rsp), %rax
// 0x1403ef984  4889442478                     movq	%rax, 0x78(%rsp)
// Frames: F185
// 0x1403ef989  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1403ef98e  e83d324400                     callq	0x140832bd0 <.text+0x831bd0>
// 0x1403ef993  4c8b4c2450                     movq	0x50(%rsp), %r9
// 0x1403ef998  488b542448                     movq	0x48(%rsp), %rdx
// Frames: F89 -> F52 -> F178
// 0x1403ef99d  48ffc5                         incq	%rbp
// Frames: F51 -> F52 -> F178
// 0x1403ef9a0  4839f5                         cmpq	%rsi, %rbp
// 0x1403ef9a3  0f8421ffffff                   je	0x1403ef8ca <.text+0x3ee8ca>
// 0x1403ef9a9  e9f5feffff                     jmp	0x1403ef8a3 <.text+0x3ee8a3>
// Frames: F187
// 0x1403ef9ae  0f28b42480000000               movaps	0x80(%rsp), %xmm6
// 0x1403ef9b6  4881c498000000                 addq	$0x98, %rsp
// 0x1403ef9bd  5b                             popq	%rbx
// 0x1403ef9be  5d                             popq	%rbp
// 0x1403ef9bf  5f                             popq	%rdi
// 0x1403ef9c0  5e                             popq	%rsi
// 0x1403ef9c1  415c                           popq	%r12
// 0x1403ef9c3  415d                           popq	%r13
// 0x1403ef9c5  415e                           popq	%r14
// 0x1403ef9c7  415f                           popq	%r15
// 0x1403ef9c9  c3                             retq
// Frames: F176 -> F185
// 0x1403ef9ca  488d15cf81dd01                 leaq	0x1dd81cf(%rip), %rdx   # 0x1421c7ba0
// 0x1403ef9d1  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x1403ef9d6  e8d5d5cb01                     callq	0x1420acfb0 <.text+0x20abfb0>
// 0x1403ef9db  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/delete_roof.rs:12:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F2: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:806:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:469:0 bevy_ecs::event::EventReader::read
// F4: crates/systems/roof/src/delete_roof.rs:20:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:547:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F6: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:811:0 bevy_ecs::event::EventIteratorWithId::new
// F7: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:723:0 bevy_ecs::event::ManualEventReader::read_with_id
// F8: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:718:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:813:0 bevy_ecs::event::EventIteratorWithId::new
// F11: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:816:0 bevy_ecs::event::EventIteratorWithId::new
// F12: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:817:0 bevy_ecs::event::EventIteratorWithId::new
// F13: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:818:0 bevy_ecs::event::EventIteratorWithId::new
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1007:0 core::option::Option::unwrap_or_default
// F15: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:823:0 bevy_ecs::event::EventIteratorWithId::new
// F16: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:820:0 bevy_ecs::event::EventIteratorWithId::new
// F17: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:2486:0 core::option::impl$40::branch
// F18: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:871:0 bevy_ecs::event::impl$23::last
// F19: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:786:0 bevy_ecs::event::impl$20::last
// F20: crates/systems/roof/src/delete_roof.rs:32:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:354:0 core::slice::index::impl$4::get
// F22: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:480:0 core::slice::index::impl$6::get
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1007:0 core::slice::impl$0::get
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F26: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F27: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F28: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:825:0 bevy_ecs::event::EventIteratorWithId::new
// F29: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:836:0 bevy_ecs::event::ManualEventReader::read_with_id
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2894:0 alloc::vec::impl$14::from_iter
// F31: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/traits/iterator.rs:2003:0 core::iter::traits::iterator::Iterator::collect
// F32: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:440:0 bevy_ecs::system::query::Query::iter
// F33: crates/systems/roof/src/delete_roof.rs:34:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F34: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:442:0 bevy_ecs::system::query::Query::iter
// F35: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1533:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F36: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:49:0 bevy_ecs::query::iter::QueryIter::new
// F37: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:1246:0 bevy_ecs::query::state::QueryState::iter_unchecked_manual
// F38: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:872:0 bevy_ecs::event::impl$23::last
// F39: crates/systems/roof/src/delete_roof.rs:25:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F42: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1974:0 alloc::vec::Vec::push
// F43: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::event::Events::send
// F44: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:625:0 bevy_ecs::event::EventWriter::send
// F45: crates/systems/roof/src/delete_roof.rs:26:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 alloc::vec::Vec::as_mut_ptr
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F48: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1975:0 alloc::vec::Vec::push
// F49: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1976:0 alloc::vec::Vec::push
// F50: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::event::Events::send
// F51: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1596:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F52: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:886:0 bevy_ecs::query::iter::impl$1::next
// F53: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:569:0 bevy_ecs::storage::table::Table::entities
// F54: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1605:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F55: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:1798:0 core::ptr::non_null::impl$12::eq
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$181::next
// F57: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1597:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F58: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:162:0 core::slice::iter::impl$181::next
// F59: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F60: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1598:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F61: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F62: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F63: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:693:0 bevy_ecs::storage::table::Table::get_column
// F64: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:955:0 bevy_ecs::query::fetch::impl$21::set_table
// F65: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:196:0 bevy_ecs::query::world_query::impl$2::set_table
// F66: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1602:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F67: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1007:0 bevy_ecs::query::fetch::impl$21::matches_component_set
// F68: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1548:0 bevy_ecs::query::fetch::impl$36::set_table
// F69: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:49:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
// F70: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:416:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
// F71: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:714:0 bevy_ecs::storage::table::Table::has_column
// F72: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1548:0 bevy_ecs::query::fetch::impl$36::set_table::closure$0
// F73: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1010:0 bevy_ecs::query::fetch::impl$21::matches_component_set
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::slice::impl$0::get
// F75: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:608:0 core::option::Option::is_some
// F76: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:49:0 bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
// F77: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::slice::impl$0::get
// F78: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:127:0 bevy_ecs::storage::table::TableRow::as_usize
// F79: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:973:0 bevy_ecs::query::fetch::impl$21::fetch
// F80: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:208:0 bevy_ecs::query::world_query::impl$2::fetch
// F81: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1625:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F82: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs:506:0 bevy_ecs::storage::table::TableRow::as_usize
// F83: crates/systems/roof/src/delete_roof.rs:28:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F84: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$29::set_changed
// F85: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F86: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1970:0 alloc::vec::Vec::push
// F87: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::event::Events::send
// F88: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1971:0 alloc::vec::Vec::push
// F89: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1627:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F90: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1563:0 bevy_ecs::query::fetch::impl$36::fetch
// F91: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 core::bool::impl$0::then
// F92: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1566:0 bevy_ecs::query::fetch::impl$36::fetch
// F93: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F94: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::option::Option::map
// F95: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::option::Option::map
// F96: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:284:0 core::slice::iter::impl$181::any
// F97: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/cmp.rs:216:0 core::slice::cmp::impl$10::slice_contains
// F98: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:2528:0 core::slice::impl$0::contains
// F99: crates/systems/roof/src/delete_roof.rs:35:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:285:0 core::slice::iter::impl$181::any
// F101: crates/country-core/src/resources/walls/mod.rs:66:0 country_core::resources::walls::impl$9::eq
// F102: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/cmp.rs:216:0 core::slice::cmp::impl$10::slice_contains::closure$0
// F103: crates/systems/roof/src/delete_roof.rs:36:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F104: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$29::set_changed
// F105: crates/systems/roof/src/delete_roof.rs:41:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F106: crates/systems/roof/src/delete_roof.rs:44:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F107: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1665:0 core::ptr::read_volatile
// F108: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:98:0 alloc::alloc::alloc
// F109: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:183:0 alloc::alloc::Global::alloc_impl
// F110: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:243:0 alloc::alloc::impl$1::allocate
// F111: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:332:0 alloc::alloc::exchange_malloc
// F112: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/boxed.rs:219:0 alloc::boxed::impl$0::new
// F113: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:395:0 alloc::sync::Arc::new
// F114: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:100:0 alloc::alloc::alloc
// F115: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:515:0 core::ptr::drop_in_place
// F116: crates/systems/roof/src/delete_roof.rs:48:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F117: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:280:0 alloc::raw_vec::RawVec::current_memory
// F118: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:554:0 alloc::raw_vec::impl$4::drop
// F119: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::unchecked_mul
// F120: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:290:0 alloc::raw_vec::RawVec::current_memory
// F121: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:119:0 alloc::alloc::dealloc
// F122: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:256:0 alloc::alloc::impl$1::deallocate
// F123: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:555:0 alloc::raw_vec::impl$4::drop
// F124: crates/systems/roof/src/delete_roof.rs:49:0 system_roof::delete_roof::process_invalid_roof_notifications::h572bb3f24918da70
// F125: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:334:0 alloc::alloc::exchange_malloc
// F126: crates/systems/roof/src/delete_roof.rs:51:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F127: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:423:0 alloc::vec::Vec::new
// F128: crates/systems/roof/src/delete_roof.rs:57:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F129: crates/systems/roof/src/delete_roof.rs:58:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F130: crates/systems/roof/src/delete_roof.rs:59:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F131: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:705:0 core::option::Option::as_mut
// F132: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:300:0 core::iter::adapters::chain::and_then_or_clear
// F133: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next
// F134: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:845:0 bevy_ecs::event::impl$23::next
// F135: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:771:0 bevy_ecs::event::impl$20::next
// F136: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:850:0 bevy_ecs::event::impl$23::next
// F137: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/chain.rs:51:0 core::iter::adapters::chain::impl$1::next::closure$0
// F138: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1513:0 core::option::Option::or_else
// F139: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F140: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F141: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F142: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:476:0 bevy_ecs::system::query::Query::iter_mut
// F143: crates/systems/roof/src/delete_roof.rs:62:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F144: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:477:0 bevy_ecs::system::query::Query::iter_mut
// F145: crates/systems/roof/src/delete_roof.rs:63:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F146: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:448:0 bevy_ecs::system::commands::Commands::get_entity
// F147: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:408:0 bevy_ecs::system::commands::Commands::entity
// F148: crates/systems/roof/src/delete_roof.rs:64:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F149: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:34:0 core::bool::impl$0::then_some
// F150: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:502:0 bevy_ecs::system::commands::Commands::push
// F151: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:749:0 bevy_ecs::system::commands::Commands::add
// F152: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:1101:0 bevy_ecs::system::commands::EntityCommands::add
// F153: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:946:0 bevy_ecs::system::commands::EntityCommands::insert
// F154: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:271:0 alloc::raw_vec::RawVec::capacity
// F155: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:415:0 alloc::raw_vec::RawVec::needs_to_grow
// F156: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:332:0 alloc::raw_vec::RawVec::reserve
// F157: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:945:0 alloc::vec::Vec::reserve
// F158: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:200:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F159: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:510:0 bevy_ecs::system::commands::Commands::push
// F160: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2203:0 alloc::vec::Vec::len
// F161: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:197:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F162: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 alloc::raw_vec::RawVec::capacity
// F163: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:333:0 alloc::raw_vec::RawVec::reserve
// F164: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:204:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F165: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2987:0 core::intrinsics::copy_nonoverlapping
// F166: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1584:0 core::ptr::write_unaligned
// F167: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1551:0 core::ptr::mut_ptr::impl$0::write_unaligned
// F168: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:213:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F169: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:220:0 bevy_ecs::world::command_queue::RawCommandQueue::push
// F170: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1458:0 alloc::vec::Vec::set_len
// F171: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs:84:0 bevy_ecs::world::command_queue::CommandQueue::push
// F172: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:504:0 bevy_ecs::system::commands::Commands::push
// F173: crates/systems/roof/src/delete_roof.rs:67:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F174: crates/systems/roof/src/delete_roof.rs:65:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F175: crates/systems/roof/src/delete_roof.rs:71:0 system_roof::delete_roof::mark_roof_for_deletion::hd93d9e080193cb66
// F176: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:410:0 bevy_ecs::system::commands::Commands::entity
// F177: crates/systems/roof/src/delete_roof.rs:73:0 system_roof::delete_roof::delete_roofs::he458806e2d067d84
// F178: crates/systems/roof/src/delete_roof.rs:82:0 system_roof::delete_roof::delete_roofs::he458806e2d067d84
// F179: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:208:0 bevy_ecs::query::world_query::impl$4::fetch
// F180: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:196:0 bevy_ecs::query::world_query::impl$4::set_table
// F181: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:412:0 bevy_ecs::storage::blob_vec::BlobVec::get_slice
// F182: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:322:0 bevy_ecs::storage::table::Column::get_data_slice
// F183: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:957:0 bevy_ecs::query::fetch::impl$21::set_table
// F184: crates/systems/roof/src/delete_roof.rs:86:0 system_roof::delete_roof::delete_roofs::he458806e2d067d84
// F185: crates/systems/roof/src/delete_roof.rs:87:0 system_roof::delete_roof::delete_roofs::he458806e2d067d84
// F186: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs:409:0 bevy_ecs::system::commands::Commands::entity
// F187: crates/systems/roof/src/delete_roof.rs:90:0 system_roof::delete_roof::delete_roofs::he458806e2d067d84
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/delete_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 08cb8cc9e4703c38b7a630bd3097875755d24dd9877ca1113f0862dccab44edb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: delete_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 264
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::process_invalid_roof_notifications
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":264,"record_offset":248,"rva":4124272,"va":5372833392}
// 编译实例 RVAs: 0x3eee70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-84821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372833392,"raw_hex":"e8abb4feff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372834512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-84924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372833392,"raw_hex":"e844b4feff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372834631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cb310
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5096307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372833392,"raw_hex":"e873c34d00","size":5,"soft_fail":false,"target_va":5377930000,"va":5372833688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30143472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372833392,"raw_hex":"e8f0f3cb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372834667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1527193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372833392,"raw_hex":"e867b2e8ff","size":5,"soft_fail":false,"target_va":5371306912,"va":5372834100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2131765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372833392,"raw_hex":"e835872000","size":5,"soft_fail":false,"target_va":5374966368,"va":5372834598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":264,"record_offset":4192,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":264,"record_offset":4136,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":428,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":1800,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":700,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":2076,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":264,"record_offset":1400,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":264,"record_offset":1344,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":264,"record_offset":4408,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":264,"record_offset":3668,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":264,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":248,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":264,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":248,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":264,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":248,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":264,"record_offset":4248,"parent_record_offset":4220,"ancestor_record_offset":248,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":264,"record_offset":4280,"parent_record_offset":4248,"ancestor_record_offset":248,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":264,"record_offset":4548,"parent_record_offset":4456,"ancestor_record_offset":248,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":264,"record_offset":4220,"parent_record_offset":4192,"ancestor_record_offset":248,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":4484,"parent_record_offset":4456,"ancestor_record_offset":248,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":1948,"parent_record_offset":1916,"ancestor_record_offset":248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":248,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":1916,"parent_record_offset":1872,"ancestor_record_offset":248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":1872,"parent_record_offset":1828,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":248,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":264,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":264,"record_offset":2016,"parent_record_offset":1800,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":248,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":1828,"parent_record_offset":1800,"ancestor_record_offset":248,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":488,"parent_record_offset":452,"ancestor_record_offset":248,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":248,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":248,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":248,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":248,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":2964,"parent_record_offset":2936,"ancestor_record_offset":248,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":2428,"parent_record_offset":2320,"ancestor_record_offset":248,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":248,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":248,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":248,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":248,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":264,"record_offset":1428,"parent_record_offset":1400,"ancestor_record_offset":248,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":264,"record_offset":2728,"parent_record_offset":2100,"ancestor_record_offset":248,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":264,"record_offset":2320,"parent_record_offset":2100,"ancestor_record_offset":248,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":264,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":248,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":264,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":248,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":264,"record_offset":2392,"parent_record_offset":2368,"ancestor_record_offset":248,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":264,"record_offset":2640,"parent_record_offset":2576,"ancestor_record_offset":248,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":2144,"parent_record_offset":2100,"ancestor_record_offset":248,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":248,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":248,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":248,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":2236,"parent_record_offset":2100,"ancestor_record_offset":248,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":264,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":248,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":264,"record_offset":3000,"parent_record_offset":2964,"ancestor_record_offset":248,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":264,"record_offset":560,"parent_record_offset":512,"ancestor_record_offset":248,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":264,"record_offset":4512,"parent_record_offset":4484,"ancestor_record_offset":248,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":264,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":248,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":264,"record_offset":3240,"parent_record_offset":3212,"ancestor_record_offset":248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":264,"record_offset":624,"parent_record_offset":512,"ancestor_record_offset":248,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":264,"record_offset":760,"parent_record_offset":728,"ancestor_record_offset":248,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":264,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":264,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":264,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":264,"record_offset":4356,"parent_record_offset":4328,"ancestor_record_offset":248,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":264,"record_offset":1980,"parent_record_offset":1872,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":264,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":248,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":264,"record_offset":3808,"parent_record_offset":3716,"ancestor_record_offset":248,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":264,"record_offset":1000,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":264,"record_offset":1188,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":264,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":248,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":264,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":248,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":264,"record_offset":1036,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":264,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":264,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":248,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":264,"record_offset":2172,"parent_record_offset":2100,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":264,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":248,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::mark_roof_for_deletion
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":264,"record_offset":4828,"rva":4125648,"va":5372834768}
// 编译实例 RVAs: 0x3ef3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-86053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372834768,"raw_hex":"e8dbaffeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372835760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30083595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372834768,"raw_hex":"e80b0acb01","size":5,"soft_fail":false,"target_va":5402919072,"va":5372835472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1530025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372834768,"raw_hex":"e857a7e8ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5372835076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4409870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372834768,"raw_hex":"e80e4a4300","size":5,"soft_fail":false,"target_va":5377245296,"va":5372835421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30136279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372834768,"raw_hex":"e8d7d7cb01","size":5,"soft_fail":false,"target_va":5402972080,"va":5372835796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4411004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372834768,"raw_hex":"e87c4e4300","size":5,"soft_fail":false,"target_va":5377246528,"va":5372835519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2927831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372834768,"raw_hex":"e8d7ac2c00","size":5,"soft_fail":false,"target_va":5375763440,"va":5372835604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":5000,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":5568,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":5028,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":7764,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":5700,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":6232,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":264,"record_offset":6764,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":264,"record_offset":6856,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":264,"record_offset":6032,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":264,"record_offset":8024,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":264,"record_offset":6556,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":264,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":4828,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":264,"record_offset":8176,"parent_record_offset":8076,"ancestor_record_offset":4828,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":7088,"parent_record_offset":7060,"ancestor_record_offset":4828,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":8108,"parent_record_offset":8076,"ancestor_record_offset":4828,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":7060,"parent_record_offset":7028,"ancestor_record_offset":4828,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":4828,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":7028,"parent_record_offset":7000,"ancestor_record_offset":4828,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":264,"record_offset":8076,"parent_record_offset":8048,"ancestor_record_offset":4828,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":5612,"parent_record_offset":5568,"ancestor_record_offset":4828,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":7140,"parent_record_offset":6960,"ancestor_record_offset":4828,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":7880,"parent_record_offset":7844,"ancestor_record_offset":4828,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":7000,"parent_record_offset":6960,"ancestor_record_offset":4828,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":264,"record_offset":7308,"parent_record_offset":6960,"ancestor_record_offset":4828,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":264,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":4828,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":264,"record_offset":7788,"parent_record_offset":7764,"ancestor_record_offset":4828,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":4828,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":7844,"parent_record_offset":7764,"ancestor_record_offset":4828,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":5092,"parent_record_offset":5052,"ancestor_record_offset":4828,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":264,"record_offset":5728,"parent_record_offset":5700,"ancestor_record_offset":4828,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":6084,"parent_record_offset":6060,"ancestor_record_offset":4828,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":4828,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":6264,"parent_record_offset":6232,"ancestor_record_offset":4828,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":264,"record_offset":6060,"parent_record_offset":6032,"ancestor_record_offset":4828,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":6520,"parent_record_offset":6264,"ancestor_record_offset":4828,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":6436,"parent_record_offset":6264,"ancestor_record_offset":4828,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":264,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":4828,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":264,"record_offset":6796,"parent_record_offset":6764,"ancestor_record_offset":4828,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":264,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":4828,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":264,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":4828,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":264,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":4828,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":264,"record_offset":7340,"parent_record_offset":6928,"ancestor_record_offset":4828,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":264,"record_offset":6960,"parent_record_offset":6928,"ancestor_record_offset":4828,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":264,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":4828,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":264,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":4828,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":264,"record_offset":5796,"parent_record_offset":5768,"ancestor_record_offset":4828,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":264,"record_offset":5768,"parent_record_offset":5728,"ancestor_record_offset":4828,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":264,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":4828,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":264,"record_offset":5160,"parent_record_offset":5116,"ancestor_record_offset":4828,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":264,"record_offset":8140,"parent_record_offset":8108,"ancestor_record_offset":4828,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":264,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":4828,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":264,"record_offset":5852,"parent_record_offset":5768,"ancestor_record_offset":4828,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":264,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":4828,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":264,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":4828,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":264,"record_offset":7224,"parent_record_offset":6960,"ancestor_record_offset":4828,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":264,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":4828,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":264,"record_offset":6348,"parent_record_offset":6312,"ancestor_record_offset":4828,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":264,"record_offset":5668,"parent_record_offset":5568,"ancestor_record_offset":4828,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":264,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":4828,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":264,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":4828,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":264,"record_offset":6696,"parent_record_offset":6604,"ancestor_record_offset":4828,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":264,"record_offset":5224,"parent_record_offset":5116,"ancestor_record_offset":4828,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":264,"record_offset":5412,"parent_record_offset":5116,"ancestor_record_offset":4828,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":264,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":4828,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":264,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":4828,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":264,"record_offset":5260,"parent_record_offset":5224,"ancestor_record_offset":4828,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":264,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":4828,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":264,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":4828,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":264,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":4828,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":264,"record_offset":5928,"parent_record_offset":5876,"ancestor_record_offset":4828,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":264,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":4828,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::delete_roofs
// 记录形式: compiled_procedure; instances=1
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":264,"record_offset":8468,"rva":4126768,"va":5372835888}
// 编译实例 RVAs: 0x3ef830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4409112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372835888,"raw_hex":"e818474300","size":5,"soft_fail":false,"target_va":5377245296,"va":5372836179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30135765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372835888,"raw_hex":"e8d5d5cb01","size":5,"soft_fail":false,"target_va":5402972080,"va":5372836310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4469309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372835888,"raw_hex":"e83d324400","size":5,"soft_fail":false,"target_va":5377305552,"va":5372836238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":8796,"parent_record_offset":8468,"ancestor_record_offset":8468,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":264,"record_offset":9768,"parent_record_offset":8468,"ancestor_record_offset":8468,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":264,"record_offset":8600,"parent_record_offset":8468,"ancestor_record_offset":8468,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":8988,"parent_record_offset":8900,"ancestor_record_offset":8468,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":9108,"parent_record_offset":9080,"ancestor_record_offset":8468,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":9408,"parent_record_offset":9048,"ancestor_record_offset":8468,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":8928,"parent_record_offset":8900,"ancestor_record_offset":8468,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":9080,"parent_record_offset":9048,"ancestor_record_offset":8468,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":264,"record_offset":9140,"parent_record_offset":9108,"ancestor_record_offset":8468,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":8468,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":8468,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":264,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":8468,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":264,"record_offset":8624,"parent_record_offset":8600,"ancestor_record_offset":8468,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":264,"record_offset":8900,"parent_record_offset":8824,"ancestor_record_offset":8468,"inlinee":14639,"source_line":201,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":264,"record_offset":9048,"parent_record_offset":8824,"ancestor_record_offset":8468,"inlinee":14640,"source_line":192,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":264,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":8468,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":264,"record_offset":9224,"parent_record_offset":9196,"ancestor_record_offset":8468,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":264,"record_offset":9196,"parent_record_offset":9168,"ancestor_record_offset":8468,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":264,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":8468,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":264,"record_offset":9296,"parent_record_offset":9252,"ancestor_record_offset":8468,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":9516,"parent_record_offset":9492,"ancestor_record_offset":8468,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":9580,"parent_record_offset":8824,"ancestor_record_offset":8468,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":8468,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":9168,"parent_record_offset":9140,"ancestor_record_offset":8468,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":9012,"parent_record_offset":8988,"ancestor_record_offset":8468,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":264,"record_offset":9676,"parent_record_offset":8824,"ancestor_record_offset":8468,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":264,"record_offset":9804,"parent_record_offset":9768,"ancestor_record_offset":8468,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":264,"record_offset":8956,"parent_record_offset":8928,"ancestor_record_offset":8468,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":264,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":8468,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":264,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":8468,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":264,"record_offset":9252,"parent_record_offset":9224,"ancestor_record_offset":8468,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":264,"record_offset":8744,"parent_record_offset":8720,"ancestor_record_offset":8468,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":264,"record_offset":9644,"parent_record_offset":9612,"ancestor_record_offset":8468,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":264,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":8468,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":264,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":8468,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":264,"record_offset":9352,"parent_record_offset":9252,"ancestor_record_offset":8468,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":264,"record_offset":8720,"parent_record_offset":8696,"ancestor_record_offset":8468,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":264,"record_offset":9612,"parent_record_offset":8824,"ancestor_record_offset":8468,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00264.jsonl.gz"}
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
// 完整结构及原始类型记录: delete_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: delete_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/binary/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/delete_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: delete_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>", "bevy_ecs::event::EventReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::delete_roofs
// 信息 ID: 1f894a37924b9c597c96002f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 22f506c314bfeff5269f4bf0: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)> @ 0x8e59dd -> 0x3ef830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0x12d8bdb -> 0x3ef830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::mark_roof_for_deletion
// 信息 ID: 856c1b07bc9fd01cde782603
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> @ 0x8e57f5 -> 0x3ef3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > @ 0x12d4964 -> 0x3ef3d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::process_invalid_roof_notifications
// 信息 ID: 3b763853103525592d673ad5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d3696816013ccc14dc345402: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)> @ 0x4dc60 -> 0x3eee70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)> > @ 0xd0b3d -> 0x3eee70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
