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
// Inline site 2437:6400 -> host 2437:268 / rva_1f367a0_len_1431; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x1f3707e, 0x1f3712f), 31 instructions
//   Function-frame RVA span [0x1f37292, 0x1f37343), 31 instructions
// Inline site 2437:6952 -> host 2437:268 / rva_1f367a0_len_1431; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x1f3707e, 0x1f3712f), 31 instructions
//   Function-frame RVA span [0x1f37292, 0x1f37343), 31 instructions
//
// FUNCTION system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// Candidate interfaces: 497b97607894768c7492cd7f
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 2437:268 -> rva_1f367a0_len_1431
//
// FUNCTION system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 2437:10508 -> rva_1f37be0_len_1d9
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1f367a0_len_1431: RVA 0x1f367a0, 5169 PDB bytes; 1164 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f367a0_len_1431.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2437:268: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// Frames: F1
// 0x141f367a0  55                             pushq	%rbp
// 0x141f367a1  4157                           pushq	%r15
// 0x141f367a3  4156                           pushq	%r14
// 0x141f367a5  4155                           pushq	%r13
// 0x141f367a7  4154                           pushq	%r12
// 0x141f367a9  56                             pushq	%rsi
// 0x141f367aa  57                             pushq	%rdi
// 0x141f367ab  53                             pushq	%rbx
// 0x141f367ac  4881ec68030000                 subq	$0x368, %rsp            # imm = 0x368
// 0x141f367b3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x141f367bb  440f298dd0020000               movaps	%xmm9, 0x2d0(%rbp)
// 0x141f367c3  440f2985c0020000               movaps	%xmm8, 0x2c0(%rbp)
// 0x141f367cb  0f29bdb0020000                 movaps	%xmm7, 0x2b0(%rbp)
// 0x141f367d2  0f29b5a0020000                 movaps	%xmm6, 0x2a0(%rbp)
// 0x141f367d9  48c78598020000feffffff         movq	$-0x2, 0x298(%rbp)
// 0x141f367e4  4d89ce                         movq	%r9, %r14
// 0x141f367e7  4d89c4                         movq	%r8, %r12
// 0x141f367ea  4889d6                         movq	%rdx, %rsi
// 0x141f367ed  48898d38020000                 movq	%rcx, 0x238(%rbp)
// 0x141f367f4  488b8570030000                 movq	0x370(%rbp), %rax
// Frames: F2
// 0x141f367fb  4c8b38                         movq	(%rax), %r15
// 0x141f367fe  488b4820                       movq	0x20(%rax), %rcx
// 0x141f36802  48898d88020000                 movq	%rcx, 0x288(%rbp)
// 0x141f36809  488b4040                       movq	0x40(%rax), %rax
// 0x141f3680d  48898530020000                 movq	%rax, 0x230(%rbp)
// Frames: F3
// 0x141f36814  e8a758f5fe                     callq	0x140e8c0c0 <.text+0xe8b0c0>
// 0x141f36819  89c3                           movl	%eax, %ebx
// 0x141f3681b  84c0                           testb	%al, %al
// 0x141f3681d  0f8408010000                   je	0x141f3692b <.text+0x1f3592b>
// Frames: F4 -> F3
// 0x141f36823  bf5b000000                     movl	$0x5b, %edi
// 0x141f36828  b95b000000                     movl	$0x5b, %ecx
// 0x141f3682d  e89e150000                     callq	0x141f37dd0 <.text+0x1f36dd0>
// 0x141f36832  4c8d2d0af94d01                 leaq	0x14df90a(%rip), %r13   # 0x143416143
// 0x141f36839  4883f801                       cmpq	$0x1, %rax
// 0x141f3683d  7539                           jne	0x141f36878 <.text+0x1f35878>
// Frames: F5 -> F6 -> F7 -> F8 -> F9 -> F3
// 0x141f3683f  4989d1                         movq	%rdx, %r9
// 0x141f36842  4883fa5b                       cmpq	$0x5b, %rdx
// 0x141f36846  0f87560f0000                   ja	0x141f377a2 <.text+0x1f367a2>
// Frames: F9 -> F3
// 0x141f3684c  4c89c9                         movq	%r9, %rcx
// 0x141f3684f  e87c150000                     callq	0x141f37dd0 <.text+0x1f36dd0>
// 0x141f36854  a801                           testb	$0x1, %al
// 0x141f36856  7420                           je	0x141f36878 <.text+0x1f35878>
// Frames: F10 -> F3
// 0x141f36858  4c8d4202                       leaq	0x2(%rdx), %r8
// Frames: F5 -> F11 -> F12 -> F8 -> F10 -> F3
// 0x141f3685c  4983f85b                       cmpq	$0x5b, %r8
// 0x141f36860  0f875c0f0000                   ja	0x141f377c2 <.text+0x1f367c2>
// Frames: F13 -> F14 -> F15 -> F12 -> F8 -> F10 -> F3
// 0x141f36866  bf59000000                     movl	$0x59, %edi
// 0x141f3686b  4829d7                         subq	%rdx, %rdi
// Frames: F16 -> F17 -> F14 -> F15 -> F12 -> F8 -> F10 -> F3
// 0x141f3686e  4c8d2dcef84d01                 leaq	0x14df8ce(%rip), %r13   # 0x143416143
// 0x141f36875  4d01c5                         addq	%r8, %r13
// 0x141f36878  4c89bd70020000                 movq	%r15, 0x270(%rbp)
// 0x141f3687f  4c8d0525f74d01                 leaq	0x14df725(%rip), %r8    # 0x143415fab
// 0x141f36886  4c8d3dddf64d01                 leaq	0x14df6dd(%rip), %r15   # 0x143415f6a
// 0x141f3688d  4889b578020000                 movq	%rsi, 0x278(%rbp)
// 0x141f36894  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368a0  410fbe40ff                     movsbl	-0x1(%r8), %eax
// 0x141f368a5  85c0                           testl	%eax, %eax
// 0x141f368a7  7817                           js	0x141f368c0 <.text+0x1f358c0>
// Frames: F26 -> F27 -> F28 -> F29 -> F30 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368a9  49ffc8                         decq	%r8
// 0x141f368ac  83f85c                         cmpl	$0x5c, %eax
// 0x141f368af  7563                           jne	0x141f36914 <.text+0x1f35914>
// 0x141f368b1  e98f000000                     jmp	0x141f36945 <.text+0x1f35945>
// 0x141f368b6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F31 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368c0  410fb648fe                     movzbl	-0x2(%r8), %ecx
// Frames: F32 -> F33 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368c5  80f9c0                         cmpb	$-0x40, %cl
// Frames: F33 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368c8  7d23                           jge	0x141f368ed <.text+0x1f358ed>
// Frames: F34 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368ca  410fb650fd                     movzbl	-0x3(%r8), %edx
// Frames: F32 -> F35 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368cf  80fac0                         cmpb	$-0x40, %dl
// Frames: F35 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368d2  7d22                           jge	0x141f368f6 <.text+0x1f358f6>
// Frames: F36 -> F37 -> F38 -> F39 -> F40 -> F41 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368d4  450fb648fc                     movzbl	-0x4(%r8), %r9d
// 0x141f368d9  4983c0fc                       addq	$-0x4, %r8
// Frames: F42 -> F43 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368dd  4183e107                       andl	$0x7, %r9d
// Frames: F44 -> F45 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368e1  41c1e106                       shll	$0x6, %r9d
// 0x141f368e5  83e23f                         andl	$0x3f, %edx
// 0x141f368e8  4409ca                         orl	%r9d, %edx
// 0x141f368eb  eb10                           jmp	0x141f368fd <.text+0x1f358fd>
// Frames: F33 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368ed  4983c0fe                       addq	$-0x2, %r8
// 0x141f368f1  83e11f                         andl	$0x1f, %ecx
// 0x141f368f4  eb0f                           jmp	0x141f36905 <.text+0x1f35905>
// 0x141f368f6  4983c0fd                       addq	$-0x3, %r8
// 0x141f368fa  83e20f                         andl	$0xf, %edx
// Frames: F44 -> F46 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f368fd  c1e206                         shll	$0x6, %edx
// 0x141f36900  83e13f                         andl	$0x3f, %ecx
// 0x141f36903  09d1                           orl	%edx, %ecx
// Frames: F44 -> F47 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f36905  c1e106                         shll	$0x6, %ecx
// 0x141f36908  243f                           andb	$0x3f, %al
// 0x141f3690a  0fb6c0                         movzbl	%al, %eax
// 0x141f3690d  09c8                           orl	%ecx, %eax
// Frames: F26 -> F27 -> F28 -> F29 -> F30 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f3690f  83f85c                         cmpl	$0x5c, %eax
// 0x141f36912  7431                           je	0x141f36945 <.text+0x1f35945>
// 0x141f36914  83f82f                         cmpl	$0x2f, %eax
// 0x141f36917  742c                           je	0x141f36945 <.text+0x1f35945>
// Frames: F48 -> F49 -> F50 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f36919  4d39f8                         cmpq	%r15, %r8
// Frames: F51 -> F50 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F3
// 0x141f3691c  7582                           jne	0x141f368a0 <.text+0x1f358a0>
// 0x141f3691e  4c89f6                         movq	%r14, %rsi
// 0x141f36921  4189de                         movl	%ebx, %r14d
// 0x141f36924  bb41000000                     movl	$0x41, %ebx
// 0x141f36929  eb42                           jmp	0x141f3696d <.text+0x1f3596d>
// Frames: F3
// 0x141f3692b  31c0                           xorl	%eax, %eax
// 0x141f3692d  488985d0000000                 movq	%rax, 0xd0(%rbp)
// Frames: F52
// 0x141f36934  488b06                         movq	(%rsi), %rax
// 0x141f36937  803800                         cmpb	$0x0, (%rax)
// 0x141f3693a  0f85e00c0000                   jne	0x141f37620 <.text+0x1f36620>
// 0x141f36940  e9bd000000                     jmp	0x141f36a02 <.text+0x1f35a02>
// Frames: F53 -> F3
// 0x141f36945  4c8d3d1ef64d01                 leaq	0x14df61e(%rip), %r15   # 0x143415f6a
// 0x141f3694c  4d29f8                         subq	%r15, %r8
// 0x141f3694f  49ffc0                         incq	%r8
// Frames: F5 -> F11 -> F12 -> F8 -> F53 -> F3
// 0x141f36952  4983f841                       cmpq	$0x41, %r8
// 0x141f36956  0f87230e0000                   ja	0x141f3777f <.text+0x1f3677f>
// Frames: F13 -> F14 -> F15 -> F12 -> F8 -> F53 -> F3
// 0x141f3695c  4c89f6                         movq	%r14, %rsi
// 0x141f3695f  4189de                         movl	%ebx, %r14d
// 0x141f36962  bb41000000                     movl	$0x41, %ebx
// 0x141f36967  4c29c3                         subq	%r8, %rbx
// Frames: F16 -> F17 -> F14 -> F15 -> F12 -> F8 -> F53 -> F3
// 0x141f3696a  4d01c7                         addq	%r8, %r15
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F3
// 0x141f3696d  e80e7cf5fe                     callq	0x140e8e580 <.text+0xe8d580>
// Frames: F60 -> F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F3
// 0x141f36972  80786801                       cmpb	$0x1, 0x68(%rax)
// 0x141f36976  0f85240d0000                   jne	0x141f376a0 <.text+0x1f366a0>
// Frames: F61 -> F62 -> F63 -> F64 -> F65 -> F57 -> F58 -> F59 -> F3
// 0x141f3697c  48833800                       cmpq	$0x0, (%rax)
// 0x141f36980  0f850e0d0000                   jne	0x141f37694 <.text+0x1f36694>
// Frames: F66 -> F67 -> F68 -> F62 -> F63 -> F64 -> F65 -> F57 -> F58 -> F59 -> F3
// 0x141f36986  48c700ffffffff                 movq	$-0x1, (%rax)
// Frames: F69 -> F70 -> F63 -> F64 -> F65 -> F57 -> F58 -> F59 -> F3
// 0x141f3698d  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F71 -> F64 -> F65 -> F57 -> F58 -> F59 -> F3
// 0x141f36991  48895c2420                     movq	%rbx, 0x20(%rsp)
// 0x141f36996  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x141f3699f  48c744242801000000             movq	$0x1, 0x28(%rsp)
// 0x141f369a8  4c89ea                         movq	%r13, %rdx
// 0x141f369ab  4989f8                         movq	%rdi, %r8
// 0x141f369ae  4d89f9                         movq	%r15, %r9
// 0x141f369b1  48898580020000                 movq	%rax, 0x280(%rbp)
// 0x141f369b8  e8d35af5fe                     callq	0x140e8c490 <.text+0xe8b490>
// 0x141f369bd  90                             nop
// Frames: F66 -> F67 -> F72 -> F73 -> F73 -> F64 -> F65 -> F57 -> F58 -> F59 -> F3
// 0x141f369be  488b8d80020000                 movq	0x280(%rbp), %rcx
// 0x141f369c5  48ff01                         incq	(%rcx)
// 0x141f369c8  48898580020000                 movq	%rax, 0x280(%rbp)
// Frames: F3
// 0x141f369cf  488985d8000000                 movq	%rax, 0xd8(%rbp)
// 0x141f369d6  b801000000                     movl	$0x1, %eax
// 0x141f369db  4489f3                         movl	%r14d, %ebx
// 0x141f369de  4c8bbd70020000                 movq	0x270(%rbp), %r15
// 0x141f369e5  4989f6                         movq	%rsi, %r14
// 0x141f369e8  488bb578020000                 movq	0x278(%rbp), %rsi
// 0x141f369ef  488985d0000000                 movq	%rax, 0xd0(%rbp)
// Frames: F52
// 0x141f369f6  488b06                         movq	(%rsi), %rax
// 0x141f369f9  803800                         cmpb	$0x0, (%rax)
// 0x141f369fc  0f851e0c0000                   jne	0x141f37620 <.text+0x1f36620>
// 0x141f36a02  8b4808                         movl	0x8(%rax), %ecx
// 0x141f36a05  ffc9                           decl	%ecx
// 0x141f36a07  83f901                         cmpl	$0x1, %ecx
// 0x141f36a0a  0f87100c0000                   ja	0x141f37620 <.text+0x1f36620>
// 0x141f36a10  889d70020000                   movb	%bl, 0x270(%rbp)
// 0x141f36a16  488b5010                       movq	0x10(%rax), %rdx
// 0x141f36a1a  48899538010000                 movq	%rdx, 0x138(%rbp)
// Frames: F74
// 0x141f36a21  498b0c24                       movq	(%r12), %rcx
// 0x141f36a25  e806a4c6fe                     callq	0x140ba0e30 <.text+0xb9fe30>
// 0x141f36a2a  90                             nop
// 0x141f36a2b  b9ffffffff                     movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
// 0x141f36a30  4989c0                         movq	%rax, %r8
// 0x141f36a33  4921c8                         andq	%rcx, %r8
// 0x141f36a36  0f8462020000                   je	0x141f36c9e <.text+0x1f35c9e>
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F80
// 0x141f36a3c  498b5608                       movq	0x8(%r14), %rdx
// Frames: F81 -> F82 -> F83 -> F79 -> F80
// 0x141f36a40  4931c8                         xorq	%rcx, %r8
// 0x141f36a43  b901000000                     movl	$0x1, %ecx
// Frames: F84 -> F85 -> F81 -> F82 -> F83 -> F79 -> F80
// 0x141f36a48  4c3b4210                       cmpq	0x10(%rdx), %r8
// Frames: F86 -> F87 -> F82 -> F83 -> F79 -> F80
// 0x141f36a4c  0f83eb0c0000                   jae	0x141f3773d <.text+0x1f3673d>
// Frames: F88 -> F89 -> F90 -> F87 -> F82 -> F83 -> F79 -> F80
// 0x141f36a52  4f8d0c40                       leaq	(%r8,%r8,2), %r9
// 0x141f36a56  49c1e103                       shlq	$0x3, %r9
// 0x141f36a5a  4c034a08                       addq	0x8(%rdx), %r9
// 0x141f36a5e  4989c0                         movq	%rax, %r8
// 0x141f36a61  49c1e820                       shrq	$0x20, %r8
// 0x141f36a65  45394110                       cmpl	%r8d, 0x10(%r9)
// Frames: F91 -> F87 -> F82 -> F83 -> F79 -> F80
// 0x141f36a69  0f85ce0c0000                   jne	0x141f3773d <.text+0x1f3673d>
// Frames: F92 -> F93 -> F79 -> F80
// 0x141f36a6f  41833900                       cmpl	$0x0, (%r9)
// 0x141f36a73  0f84c40c0000                   je	0x141f3773d <.text+0x1f3673d>
// Frames: F94 -> F95 -> F79 -> F80
// 0x141f36a79  4d8b16                         movq	(%r14), %r10
// 0x141f36a7c  4d8b4108                       movq	0x8(%r9), %r8
// 0x141f36a80  4589c3                         movl	%r8d, %r11d
// 0x141f36a83  31c9                           xorl	%ecx, %ecx
// Frames: F96 -> F95 -> F79 -> F80
// 0x141f36a85  4d3b5a38                       cmpq	0x38(%r10), %r11
// Frames: F97 -> F98 -> F95 -> F79 -> F80
// 0x141f36a89  0f83ae0c0000                   jae	0x141f3773d <.text+0x1f3673d>
// Frames: F99 -> F100 -> F101 -> F97 -> F98 -> F95 -> F79 -> F80
// 0x141f36a8f  498b7a28                       movq	0x28(%r10), %rdi
// Frames: F102 -> F99 -> F100 -> F101 -> F97 -> F98 -> F95 -> F79 -> F80
// 0x141f36a93  41c1eb06                       shrl	$0x6, %r11d
// Frames: F103 -> F100 -> F101 -> F97 -> F98 -> F95 -> F79 -> F80
// 0x141f36a97  4e8b1cdf                       movq	(%rdi,%r11,8), %r11
// Frames: F100 -> F101 -> F97 -> F98 -> F95 -> F79 -> F80
// 0x141f36a9b  4d0fa3c3                       btq	%r8, %r11
// Frames: F104 -> F79 -> F80
// 0x141f36a9f  0f83980c0000                   jae	0x141f3773d <.text+0x1f3673d>
// Frames: F105 -> F106 -> F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F79 -> F80
// 0x141f36aa5  418b4104                       movl	0x4(%r9), %eax
// 0x141f36aa9  49c1e820                       shrq	$0x20, %r8
// 0x141f36aad  488b92a0010000                 movq	0x1a0(%rdx), %rdx
// Frames: F113 -> F114 -> F111 -> F112 -> F79 -> F80
// 0x141f36ab4  4f8d04c0                       leaq	(%r8,%r8,8), %r8
// Frames: F115 -> F79 -> F80
// 0x141f36ab8  498b8a10010000                 movq	0x110(%r10), %rcx
// Frames: F84 -> F85 -> F116 -> F117 -> F118 -> F119 -> F120 -> F121 -> F115 -> F79 -> F80
// 0x141f36abf  4a3b4cc240                     cmpq	0x40(%rdx,%r8,8), %rcx
// 0x141f36ac4  7327                           jae	0x141f36aed <.text+0x1f35aed>
// Frames: F117 -> F118 -> F119 -> F120 -> F121 -> F115 -> F79 -> F80
// 0x141f36ac6  4a8d14c2                       leaq	(%rdx,%r8,8), %rdx
// 0x141f36aca  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F122 -> F55 -> F123 -> F116 -> F117 -> F118 -> F119 -> F120 -> F121 -> F115 -> F79 -> F80
// 0x141f36ace  498b0cc8                       movq	(%r8,%rcx,8), %rcx
// 0x141f36ad2  4885c9                         testq	%rcx, %rcx
// 0x141f36ad5  7416                           je	0x141f36aed <.text+0x1f35aed>
// Frames: F117 -> F118 -> F119 -> F120 -> F121 -> F115 -> F79 -> F80
// 0x141f36ad7  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F124 -> F125 -> F126 -> F117 -> F118 -> F119 -> F120 -> F121 -> F115 -> F79 -> F80
// 0x141f36adb  48f7d1                         notq	%rcx
// Frames: F127 -> F128 -> F125 -> F126 -> F117 -> F118 -> F119 -> F120 -> F121 -> F115 -> F79 -> F80
// 0x141f36ade  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x141f36ae2  48c1e104                       shlq	$0x4, %rcx
// Frames: F129 -> F130 -> F131 -> F126 -> F132 -> F120 -> F121 -> F115 -> F79 -> F80
// 0x141f36ae6  488b4c0a10                     movq	0x10(%rdx,%rcx), %rcx
// 0x141f36aeb  eb02                           jmp	0x141f36aef <.text+0x1f35aef>
// 0x141f36aed  31c9                           xorl	%ecx, %ecx
// Frames: F133 -> F134 -> F135 -> F136 -> F137 -> F138 -> F79 -> F80
// 0x141f36aef  f7d0                           notl	%eax
// Frames: F102 -> F139 -> F135 -> F136 -> F137 -> F138 -> F79 -> F80
// 0x141f36af1  4c6bf058                       imulq	$0x58, %rax, %r14
// 0x141f36af5  4901ce                         addq	%rcx, %r14
// Frames: F140
// 0x141f36af8  4c89f1                         movq	%r14, %rcx
// 0x141f36afb  e8b083b8fe                     callq	0x140abeeb0 <.text+0xabdeb0>
// 0x141f36b00  90                             nop
// 0x141f36b01  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x141f36b08  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x141f36b0f  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x141f36b16  4d89f0                         movq	%r14, %r8
// 0x141f36b19  e8b279b8fe                     callq	0x140abe4d0 <.text+0xabd4d0>
// 0x141f36b1e  90                             nop
// Frames: F141
// 0x141f36b1f  f20f10bd10010000               movsd	0x110(%rbp), %xmm7
// 0x141f36b27  f3440f108518010000             movss	0x118(%rbp), %xmm8
// 0x141f36b30  83bd0c01000001                 cmpl	$0x1, 0x10c(%rbp)
// 0x141f36b37  7524                           jne	0x141f36b5d <.text+0x1f35b5d>
// Frames: F142
// 0x141f36b39  f20f10851c010000               movsd	0x11c(%rbp), %xmm0
// Frames: F143 -> F144 -> F142
// 0x141f36b41  0f58f8                         addps	%xmm0, %xmm7
// Frames: F143 -> F145 -> F142
// 0x141f36b44  f3440f588524010000             addss	0x124(%rbp), %xmm8
// Frames: F146 -> F147 -> F142
// 0x141f36b4d  0f593decdefc00                 mulps	0xfcdeec(%rip), %xmm7   # 0x142f04a40
// Frames: F146 -> F148 -> F142
// 0x141f36b54  f3440f590517fbfc00             mulss	0xfcfb17(%rip), %xmm8   # 0x142f06674
// Frames: F143 -> F144 -> F149
// 0x141f36b5d  f20f1005cbec0b01               movsd	0x10beccb(%rip), %xmm0  # 0x142ff5830
// 0x141f36b65  0f58c7                         addps	%xmm7, %xmm0
// 0x141f36b68  0f57c9                         xorps	%xmm1, %xmm1
// Frames: F143 -> F145 -> F149
// 0x141f36b6b  f3410f58c8                     addss	%xmm8, %xmm1
// Frames: F150 -> F149
// 0x141f36b70  0f138560020000                 movlps	%xmm0, 0x260(%rbp)
// 0x141f36b77  f30f118d68020000               movss	%xmm1, 0x268(%rbp)
// Frames: F151
// 0x141f36b7f  488dbd60010000                 leaq	0x160(%rbp), %rdi
// 0x141f36b86  4889f9                         movq	%rdi, %rcx
// 0x141f36b89  4c89fa                         movq	%r15, %rdx
// 0x141f36b8c  e88ff0ecfe                     callq	0x140e05c20 <.text+0xe04c20>
// 0x141f36b91  90                             nop
// 0x141f36b92  f30f108560010000               movss	0x160(%rbp), %xmm0
// 0x141f36b9a  f30f109568010000               movss	0x168(%rbp), %xmm2
// 0x141f36ba2  0f280d8776fc00                 movaps	0xfc7687(%rip), %xmm1   # 0x142efe230
// Frames: F152 -> F151
// 0x141f36ba9  0f57ca                         xorps	%xmm2, %xmm1
// Frames: F153 -> F154 -> F155 -> F156 -> F151
// 0x141f36bac  f30f59d2                       mulss	%xmm2, %xmm2
// 0x141f36bb0  0f28d8                         movaps	%xmm0, %xmm3
// 0x141f36bb3  f30f59d8                       mulss	%xmm0, %xmm3
// 0x141f36bb7  f30f58da                       addss	%xmm2, %xmm3
// Frames: F157 -> F158 -> F159 -> F154 -> F155 -> F156 -> F151
// 0x141f36bbb  f30f51db                       sqrtss	%xmm3, %xmm3
// 0x141f36bbf  f30f10155ddefc00               movss	0xfcde5d(%rip), %xmm2   # 0x142f04a24
// Frames: F160 -> F155 -> F156 -> F151
// 0x141f36bc7  f30f5ed3                       divss	%xmm3, %xmm2
// Frames: F161 -> F162 -> F156 -> F151
// 0x141f36bcb  f30f59ca                       mulss	%xmm2, %xmm1
// Frames: F161 -> F163 -> F156 -> F151
// 0x141f36bcf  f30f59d0                       mulss	%xmm0, %xmm2
// Frames: F151
// 0x141f36bd3  4c8dade0000000                 leaq	0xe0(%rbp), %r13
// 0x141f36bda  4c89e9                         movq	%r13, %rcx
// 0x141f36bdd  e8ded8eafe                     callq	0x140de44c0 <.text+0xde34c0>
// 0x141f36be2  90                             nop
// Frames: F143 -> F145 -> F164
// 0x141f36be3  0f57c0                         xorps	%xmm0, %xmm0
// 0x141f36be6  f30f588568020000               addss	0x268(%rbp), %xmm0
// Frames: F164
// 0x141f36bee  f20f108d60020000               movsd	0x260(%rbp), %xmm1
// Frames: F143 -> F144 -> F164
// 0x141f36bf6  0f580df3900001                 addps	0x10090f3(%rip), %xmm1  # 0x142f3fcf0
// Frames: F165
// 0x141f36bfd  0f138d70010000                 movlps	%xmm1, 0x170(%rbp)
// 0x141f36c04  f30f118578010000               movss	%xmm0, 0x178(%rbp)
// 0x141f36c0c  0f2885e0000000                 movaps	0xe0(%rbp), %xmm0
// 0x141f36c13  0f298560010000                 movaps	%xmm0, 0x160(%rbp)
// 0x141f36c1a  48b80000803f0000803f           movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
// 0x141f36c24  4889857c010000                 movq	%rax, 0x17c(%rbp)
// 0x141f36c2b  c785840100000000803f           movl	$0x3f800000, 0x184(%rbp) # imm = 0x3F800000
// Frames: F166
// 0x141f36c35  48897c2420                     movq	%rdi, 0x20(%rsp)
// 0x141f36c3a  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x141f36c41  f30f100df7bdfe00               movss	0xfebdf7(%rip), %xmm1   # 0x142f22a40
// 0x141f36c49  f30f1035d3ddfc00               movss	0xfcddd3(%rip), %xmm6   # 0x142f04a24
// 0x141f36c51  f30f101d1bfafc00               movss	0xfcfa1b(%rip), %xmm3   # 0x142f06674
// 0x141f36c59  0f28d6                         movaps	%xmm6, %xmm2
// 0x141f36c5c  e85fa6c0fe                     callq	0x140b412c0 <.text+0xb402c0>
// 0x141f36c61  90                             nop
// Frames: F167
// 0x141f36c62  4c89fb                         movq	%r15, %rbx
// 0x141f36c65  488b8578030000                 movq	0x378(%rbp), %rax
// 0x141f36c6c  488b00                         movq	(%rax), %rax
// 0x141f36c6f  8b4810                         movl	0x10(%rax), %ecx
// 0x141f36c72  f30f104814                     movss	0x14(%rax), %xmm1
// 0x141f36c77  c6859702000001                 movb	$0x1, 0x297(%rbp)
// 0x141f36c7e  e80d85a0fe                     callq	0x14093f190 <.text+0x93e190>
// 0x141f36c83  90                             nop
// Frames: F168
// 0x141f36c84  84c0                           testb	%al, %al
// 0x141f36c86  7460                           je	0x141f36ce8 <.text+0x1f35ce8>
// Frames: F169
// 0x141f36c88  c6859702000001                 movb	$0x1, 0x297(%rbp)
// 0x141f36c8f  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x141f36c93  e8683fb9fe                     callq	0x140acac00 <.text+0xac9c00>
// 0x141f36c98  90                             nop
// 0x141f36c99  e989000000                     jmp	0x141f36d27 <.text+0x1f35d27>
// Frames: F73 -> F170
// 0x141f36c9e  80bd7002000000                 cmpb	$0x0, 0x270(%rbp)
// 0x141f36ca5  0f84b7090000                   je	0x141f37662 <.text+0x1f36662>
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f36cab  e8d078f5fe                     callq	0x140e8e580 <.text+0xe8d580>
// Frames: F60 -> F54 -> F55 -> F56 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f36cb0  80786801                       cmpb	$0x1, 0x68(%rax)
// 0x141f36cb4  0f85300a0000                   jne	0x141f376ea <.text+0x1f366ea>
// 0x141f36cba  488b9580020000                 movq	0x280(%rbp), %rdx
// Frames: F61 -> F62 -> F63 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f36cc1  48833800                       cmpq	$0x0, (%rax)
// 0x141f36cc5  0f85c9090000                   jne	0x141f37694 <.text+0x1f36694>
// Frames: F66 -> F67 -> F68 -> F62 -> F63 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f36ccb  48c700ffffffff                 movq	$-0x1, (%rax)
// 0x141f36cd2  48898588020000                 movq	%rax, 0x288(%rbp)
// Frames: F69 -> F70 -> F63 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f36cd9  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F172 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f36cdd  e80e5cf5fe                     callq	0x140e8c8f0 <.text+0xe8b8f0>
// 0x141f36ce2  90                             nop
// 0x141f36ce3  e970090000                     jmp	0x141f37658 <.text+0x1f36658>
// Frames: F173
// 0x141f36ce8  c6859702000001                 movb	$0x1, 0x297(%rbp)
// 0x141f36cef  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x141f36cf6  e8d53eb9fe                     callq	0x140acabd0 <.text+0xac9bd0>
// 0x141f36cfb  90                             nop
// 0x141f36cfc  48c7452802000000               movq	$0x2, 0x28(%rbp)
// 0x141f36d04  c7452001000000                 movl	$0x1, 0x20(%rbp)
// 0x141f36d0b  c6859702000001                 movb	$0x1, 0x297(%rbp)
// 0x141f36d12  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x141f36d16  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x141f36d1d  4c8d4520                       leaq	0x20(%rbp), %r8
// 0x141f36d21  e80a3eb9fe                     callq	0x140acab30 <.text+0xac9b30>
// 0x141f36d26  90                             nop
// Frames: F174
// 0x141f36d27  488b8558030000                 movq	0x358(%rbp), %rax
// 0x141f36d2e  488b38                         movq	(%rax), %rdi
// Frames: F175
// 0x141f36d31  4c897520                       movq	%r14, 0x20(%rbp)
// 0x141f36d35  4c8d3db4140000                 leaq	0x14b4(%rip), %r15      # 0x141f381f0 <.text+0x1f371f0>
// 0x141f36d3c  4c897d28                       movq	%r15, 0x28(%rbp)
// 0x141f36d40  c6859702000001                 movb	$0x1, 0x297(%rbp)
// Frames: F176 -> F177 -> F178 -> F175
// 0x141f36d47  488d157af24d01                 leaq	0x14df27a(%rip), %rdx   # 0x143415fc8
// 0x141f36d4e  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x141f36d55  4c8d4520                       leaq	0x20(%rbp), %r8
// 0x141f36d59  e8920502ff                     callq	0x140f572f0 <.text+0xf562f0>
// 0x141f36d5e  90                             nop
// Frames: F179 -> F175
// 0x141f36d5f  0f108560010000                 movups	0x160(%rbp), %xmm0
// 0x141f36d66  0f298540010000                 movaps	%xmm0, 0x140(%rbp)
// 0x141f36d6d  488b8570010000                 movq	0x170(%rbp), %rax
// 0x141f36d74  48898550010000                 movq	%rax, 0x150(%rbp)
// Frames: F180
// 0x141f36d7b  488b8500010000                 movq	0x100(%rbp), %rax
// 0x141f36d82  48894538                       movq	%rax, 0x38(%rbp)
// 0x141f36d86  0f1085f0000000                 movups	0xf0(%rbp), %xmm0
// 0x141f36d8d  0f114528                       movups	%xmm0, 0x28(%rbp)
// Frames: F181
// 0x141f36d91  c6452002                       movb	$0x2, 0x20(%rbp)
// Frames: F182
// 0x141f36d95  c6851002000000                 movb	$0x0, 0x210(%rbp)
// 0x141f36d9c  c6859602000001                 movb	$0x1, 0x296(%rbp)
// Frames: F183
// 0x141f36da3  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x141f36daa  488d5520                       leaq	0x20(%rbp), %rdx
// 0x141f36dae  4c8da510020000                 leaq	0x210(%rbp), %r12
// 0x141f36db5  4d89e0                         movq	%r12, %r8
// 0x141f36db8  e8133cb9fe                     callq	0x140aca9d0 <.text+0xac99d0>
// 0x141f36dbd  90                             nop
// Frames: F184
// 0x141f36dbe  c6859602000001                 movb	$0x1, 0x296(%rbp)
// 0x141f36dc5  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x141f36dc9  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x141f36dd0  4c8d45c0                       leaq	-0x40(%rbp), %r8
// 0x141f36dd4  e8673cb9fe                     callq	0x140acaa40 <.text+0xac9a40>
// 0x141f36dd9  90                             nop
// Frames: F185
// 0x141f36dda  c6859602000001                 movb	$0x1, 0x296(%rbp)
// 0x141f36de1  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x141f36de8  488d5520                       leaq	0x20(%rbp), %rdx
// 0x141f36dec  4531c0                         xorl	%r8d, %r8d
// 0x141f36def  e80c3bb9fe                     callq	0x140aca900 <.text+0xac9900>
// 0x141f36df4  90                             nop
// Frames: F174
// 0x141f36df5  c6859602000000                 movb	$0x0, 0x296(%rbp)
// 0x141f36dfc  488d9540010000                 leaq	0x140(%rbp), %rdx
// 0x141f36e03  4c8d8560010000                 leaq	0x160(%rbp), %r8
// 0x141f36e0a  4889f9                         movq	%rdi, %rcx
// 0x141f36e0d  e86e9c8e00                     callq	0x142820a80 <.text+0x281fa80>
// 0x141f36e12  90                             nop
// 0x141f36e13  48898520020000                 movq	%rax, 0x220(%rbp)
// 0x141f36e1a  48899528020000                 movq	%rdx, 0x228(%rbp)
// 0x141f36e21  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F186
// 0x141f36e28  488d8d20020000                 leaq	0x220(%rbp), %rcx
// 0x141f36e2f  e83c2e9cfe                     callq	0x1408f9c70 <.text+0x8f8c70>
// 0x141f36e34  90                             nop
// 0x141f36e35  888560010000                   movb	%al, 0x160(%rbp)
// 0x141f36e3b  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f36e42  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x141f36e49  e8d22c9cfe                     callq	0x1408f9b20 <.text+0x8f8b20>
// 0x141f36e4e  90                             nop
// 0x141f36e4f  84c0                           testb	%al, %al
// 0x141f36e51  7518                           jne	0x141f36e6b <.text+0x1f35e6b>
// 0x141f36e53  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f36e5a  488d8d20020000                 leaq	0x220(%rbp), %rcx
// 0x141f36e61  e8ea2d9cfe                     callq	0x1408f9c50 <.text+0x8f8c50>
// 0x141f36e66  90                             nop
// 0x141f36e67  3c02                           cmpb	$0x2, %al
// 0x141f36e69  7408                           je	0x141f36e73 <.text+0x1f35e73>
// 0x141f36e6b  f30f1035a51bff00               movss	0xff1ba5(%rip), %xmm6   # 0x142f28a18
// Frames: F187
// 0x141f36e73  488b8580030000                 movq	0x380(%rbp), %rax
// 0x141f36e7a  f30f11b54c020000               movss	%xmm6, 0x24c(%rbp)
// 0x141f36e82  488b38                         movq	(%rax), %rdi
// 0x141f36e85  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f36e8c  488d8d20020000                 leaq	0x220(%rbp), %rcx
// 0x141f36e93  e8f8017ffe                     callq	0x140727090 <.text+0x726090>
// 0x141f36e98  90                             nop
// 0x141f36e99  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f36ea0  488d8d40010000                 leaq	0x140(%rbp), %rcx
// 0x141f36ea7  4889fa                         movq	%rdi, %rdx
// 0x141f36eaa  4989c0                         movq	%rax, %r8
// 0x141f36ead  e88e16bcfe                     callq	0x140af8540 <.text+0xaf7540>
// 0x141f36eb2  90                             nop
// Frames: F188
// 0x141f36eb3  488d8538010000                 leaq	0x138(%rbp), %rax
// 0x141f36eba  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x141f36ec1  4c89bd68010000                 movq	%r15, 0x168(%rbp)
// 0x141f36ec8  4c89ad70010000                 movq	%r13, 0x170(%rbp)
// 0x141f36ecf  488d05ea33fffe                 leaq	-0x100cc16(%rip), %rax  # 0x140f2a2c0 <.text+0xf292c0>
// 0x141f36ed6  48898578010000                 movq	%rax, 0x178(%rbp)
// 0x141f36edd  488d8560020000                 leaq	0x260(%rbp), %rax
// 0x141f36ee4  48898580010000                 movq	%rax, 0x180(%rbp)
// 0x141f36eeb  488d051e3afffe                 leaq	-0x100c5e2(%rip), %rax  # 0x140f2a910 <.text+0xf29910>
// 0x141f36ef2  48898588010000                 movq	%rax, 0x188(%rbp)
// 0x141f36ef9  488d854c020000                 leaq	0x24c(%rbp), %rax
// 0x141f36f00  48898590010000                 movq	%rax, 0x190(%rbp)
// 0x141f36f07  488d05220c03ff                 leaq	-0xfcf3de(%rip), %rax   # 0x140f67b30 <.text+0xf66b30>
// 0x141f36f0e  48898598010000                 movq	%rax, 0x198(%rbp)
// 0x141f36f15  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F176 -> F177 -> F178 -> F188
// 0x141f36f1c  488d15b2f04d01                 leaq	0x14df0b2(%rip), %rdx   # 0x143415fd5
// 0x141f36f23  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x141f36f27  4c8d8560010000                 leaq	0x160(%rbp), %r8
// 0x141f36f2e  e8bd0302ff                     callq	0x140f572f0 <.text+0xf562f0>
// 0x141f36f33  90                             nop
// Frames: F179 -> F188
// 0x141f36f34  488b4520                       movq	0x20(%rbp), %rax
// 0x141f36f38  48898578020000                 movq	%rax, 0x278(%rbp)
// 0x141f36f3f  488b7528                       movq	0x28(%rbp), %rsi
// 0x141f36f43  4c8b7d30                       movq	0x30(%rbp), %r15
// Frames: F189
// 0x141f36f47  440f288de0000000               movaps	0xe0(%rbp), %xmm9
// 0x141f36f4f  488b8560020000                 movq	0x260(%rbp), %rax
// 0x141f36f56  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x141f36f5d  8b8568020000                   movl	0x268(%rbp), %eax
// 0x141f36f63  898568010000                   movl	%eax, 0x168(%rbp)
// Frames: F190 -> F191 -> F192 -> F193 -> F194 -> F195 -> F196 -> F189
// 0x141f36f69  e8a2c30cfe                     callq	0x140003310 <.text+0x2310>
// Frames: F197 -> F191 -> F192 -> F193 -> F194 -> F195 -> F196 -> F189
// 0x141f36f6e  b930000000                     movl	$0x30, %ecx
// 0x141f36f73  ba10000000                     movl	$0x10, %edx
// 0x141f36f78  e863e976fe                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F194 -> F195 -> F196 -> F189
// 0x141f36f7d  4885c0                         testq	%rax, %rax
// 0x141f36f80  4889b540020000                 movq	%rsi, 0x240(%rbp)
// 0x141f36f87  0f8448070000                   je	0x141f376d5 <.text+0x1f366d5>
// Frames: F198 -> F196 -> F189
// 0x141f36f8d  48c70001000000                 movq	$0x1, (%rax)
// 0x141f36f94  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x141f36f9c  440f294810                     movaps	%xmm9, 0x10(%rax)
// 0x141f36fa1  f30f117020                     movss	%xmm6, 0x20(%rax)
// 0x141f36fa6  488b8d60010000                 movq	0x160(%rbp), %rcx
// 0x141f36fad  48894824                       movq	%rcx, 0x24(%rax)
// 0x141f36fb1  8b8d68010000                   movl	0x168(%rbp), %ecx
// 0x141f36fb7  89482c                         movl	%ecx, 0x2c(%rax)
// Frames: F199
// 0x141f36fba  488d0d2ff04d01                 leaq	0x14df02f(%rip), %rcx   # 0x143415ff0
// 0x141f36fc1  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x141f36fc6  488dbd60010000                 leaq	0x160(%rbp), %rdi
// 0x141f36fcd  4889f9                         movq	%rdi, %rcx
// 0x141f36fd0  4889f2                         movq	%rsi, %rdx
// 0x141f36fd3  4d89f8                         movq	%r15, %r8
// 0x141f36fd6  4989c1                         movq	%rax, %r9
// 0x141f36fd9  e8e29ba8fe                     callq	0x1409c0bc0 <.text+0x9bfbc0>
// 0x141f36fde  90                             nop
// Frames: F200
// 0x141f36fdf  488b8580010000                 movq	0x180(%rbp), %rax
// 0x141f36fe6  48894540                       movq	%rax, 0x40(%rbp)
// 0x141f36fea  0f108560010000                 movups	0x160(%rbp), %xmm0
// 0x141f36ff1  0f108d70010000                 movups	0x170(%rbp), %xmm1
// 0x141f36ff8  0f294d30                       movaps	%xmm1, 0x30(%rbp)
// 0x141f36ffc  0f294520                       movaps	%xmm0, 0x20(%rbp)
// 0x141f37000  488d8d40010000                 leaq	0x140(%rbp), %rcx
// 0x141f37007  488d5520                       leaq	0x20(%rbp), %rdx
// 0x141f3700b  e84015bcfe                     callq	0x140af8550 <.text+0xaf7550>
// 0x141f37010  90                             nop
// Frames: F201 -> F202 -> F203 -> F73 -> F73 -> F73 -> F204
// 0x141f37011  488b9578020000                 movq	0x278(%rbp), %rdx
// 0x141f37018  4885d2                         testq	%rdx, %rdx
// 0x141f3701b  7412                           je	0x141f3702f <.text+0x1f3602f>
// Frames: F205 -> F206 -> F207 -> F208 -> F209 -> F203 -> F73 -> F73 -> F73 -> F204
// 0x141f3701d  41b801000000                   movl	$0x1, %r8d
// 0x141f37023  488b8d40020000                 movq	0x240(%rbp), %rcx
// 0x141f3702a  e8c1e876fe                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F210
// 0x141f3702f  f3410f10764c                   movss	0x4c(%r14), %xmm6
// 0x141f37035  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F211
// 0x141f3703c  488d8d20020000                 leaq	0x220(%rbp), %rcx
// 0x141f37043  e8082c9cfe                     callq	0x1408f9c50 <.text+0x8f8c50>
// 0x141f37048  90                             nop
// 0x141f37049  4c8bada8030000                 movq	0x3a8(%rbp), %r13
// 0x141f37050  4c8bbda0030000                 movq	0x3a0(%rbp), %r15
// 0x141f37057  488bb568030000                 movq	0x368(%rbp), %rsi
// 0x141f3705e  2c02                           subb	$0x2, %al
// 0x141f37060  0fb6c0                         movzbl	%al, %eax
// 0x141f37063  b903000000                     movl	$0x3, %ecx
// 0x141f37068  0f43c8                         cmovael	%eax, %ecx
// 0x141f3706b  0fb6c1                         movzbl	%cl, %eax
// 0x141f3706e  488d0dcff24d01                 leaq	0x14df2cf(%rip), %rcx   # 0x143416344
// 0x141f37075  48630481                       movslq	(%rcx,%rax,4), %rax
// 0x141f37079  4801c8                         addq	%rcx, %rax
// 0x141f3707c  ffe0                           jmpq	*%rax
// Frames: F212 -> F213
// 0x141f3707e  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f37085  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x141f37089  488b9530020000                 movq	0x230(%rbp), %rdx
// 0x141f37090  e82b2dcbfe                     callq	0x140be9dc0 <.text+0xbe8dc0>
// 0x141f37095  90                             nop
// Frames: F214 -> F213
// 0x141f37096  488b8588020000                 movq	0x288(%rbp), %rax
// 0x141f3709d  f30f1008                       movss	(%rax), %xmm1
// 0x141f370a1  f30f105004                     movss	0x4(%rax), %xmm2
// 0x141f370a6  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f370ad  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x141f370b1  e83a2dcbfe                     callq	0x140be9df0 <.text+0xbe8df0>
// 0x141f370b6  90                             nop
// Frames: F215 -> F213
// 0x141f370b7  0f28d1                         movaps	%xmm1, %xmm2
// 0x141f370ba  f30f11bd60010000               movss	%xmm7, 0x160(%rbp)
// 0x141f370c2  f30f11b564010000               movss	%xmm6, 0x164(%rbp)
// 0x141f370ca  f3440f118568010000             movss	%xmm8, 0x168(%rbp)
// 0x141f370d3  48b8000000000000803f           movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
// 0x141f370dd  4889856c010000                 movq	%rax, 0x16c(%rbp)
// 0x141f370e4  c7857401000000000000           movl	$0x0, 0x174(%rbp)
// Frames: F216 -> F215 -> F213
// 0x141f370ee  f20f100d4af61001               movsd	0x110f64a(%rip), %xmm1  # 0x143046740
// 0x141f370f6  f20f118d10020000               movsd	%xmm1, 0x210(%rbp)
// 0x141f370fe  c6851802000000                 movb	$0x0, 0x218(%rbp)
// 0x141f37105  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F215 -> F213
// 0x141f3710c  4c89642430                     movq	%r12, 0x30(%rsp)
// 0x141f37111  48897c2428                     movq	%rdi, 0x28(%rsp)
// 0x141f37116  48895c2420                     movq	%rbx, 0x20(%rsp)
// 0x141f3711b  488d8d50020000                 leaq	0x250(%rbp), %rcx
// 0x141f37122  4c8d4d20                       leaq	0x20(%rbp), %r9
// 0x141f37126  0f28c8                         movaps	%xmm0, %xmm1
// 0x141f37129  e8122dcbfe                     callq	0x140be9e40 <.text+0xbe8e40>
// 0x141f3712e  90                             nop
// Frames: F217 -> F213
// 0x141f3712f  488b8588030000                 movq	0x388(%rbp), %rax
// 0x141f37136  f20f108550020000               movsd	0x250(%rbp), %xmm0
// Frames: F218 -> F219 -> F217 -> F213
// 0x141f3713e  f30f108d58020000               movss	0x258(%rbp), %xmm1
// Frames: F218 -> F220 -> F217 -> F213
// 0x141f37146  0f28d0                         movaps	%xmm0, %xmm2
// 0x141f37149  0f14d7                         unpcklps	%xmm7, %xmm2            # xmm2 = xmm2[0],xmm7[0],xmm2[1],xmm7[1]
// 0x141f3714c  f30f12df                       movsldup	%xmm7, %xmm3            # xmm3 = xmm7[0,0,2,2]
// 0x141f37150  0f5cd3                         subps	%xmm3, %xmm2
// Frames: F218 -> F221 -> F217 -> F213
// 0x141f37153  0fc6f811                       shufps	$0x11, %xmm0, %xmm7     # xmm7 = xmm7[1,0],xmm0[1,0]
// 0x141f37157  0fc6f8e2                       shufps	$0xe2, %xmm0, %xmm7     # xmm7 = xmm7[2,0],xmm0[2,3]
// 0x141f3715b  f30f12c6                       movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
// 0x141f3715f  0f5cf8                         subps	%xmm0, %xmm7
// Frames: F218 -> F219 -> F217 -> F213
// 0x141f37162  410f14c8                       unpcklps	%xmm8, %xmm1            # xmm1 = xmm1[0],xmm8[0],xmm1[1],xmm8[1]
// 0x141f37166  f3410f12c0                     movsldup	%xmm8, %xmm0            # xmm0 = xmm8[0,0,2,2]
// 0x141f3716b  0f5cc8                         subps	%xmm0, %xmm1
// Frames: F222 -> F223 -> F217 -> F213
// 0x141f3716e  0f59d2                         mulps	%xmm2, %xmm2
// 0x141f37171  0f59ff                         mulps	%xmm7, %xmm7
// 0x141f37174  0f58fa                         addps	%xmm2, %xmm7
// 0x141f37177  0f59c9                         mulps	%xmm1, %xmm1
// 0x141f3717a  0f58cf                         addps	%xmm7, %xmm1
// Frames: F157 -> F158 -> F224 -> F223 -> F217 -> F213
// 0x141f3717d  0f51c1                         sqrtps	%xmm1, %xmm0
// Frames: F225
// 0x141f37180  f30f16c8                       movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
// 0x141f37184  f30f5cc1                       subss	%xmm1, %xmm0
// 0x141f37188  488b8d38020000                 movq	0x238(%rbp), %rcx
// 0x141f3718f  f30f1101                       movss	%xmm0, (%rcx)
// Frames: F226 -> F227 -> F228
// 0x141f37193  448b601c                       movl	0x1c(%rax), %r12d
// 0x141f37197  4c8b6810                       movq	0x10(%rax), %r13
// 0x141f3719b  45896500                       movl	%r12d, (%r13)
// Frames: F229 -> F228
// 0x141f3719f  488b18                         movq	(%rax), %rbx
// Frames: F230
// 0x141f371a2  498b3e                         movq	(%r14), %rdi
// 0x141f371a5  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F231 -> F230
// 0x141f371ac  4889f9                         movq	%rdi, %rcx
// 0x141f371af  e83c63a1fe                     callq	0x14094d4f0 <.text+0x94c4f0>
// 0x141f371b4  90                             nop
// Frames: F232
// 0x141f371b5  4989d0                         movq	%rdx, %r8
// 0x141f371b8  488b8d90030000                 movq	0x390(%rbp), %rcx
// 0x141f371bf  488b9550030000                 movq	0x350(%rbp), %rdx
// 0x141f371c6  4c8b39                         movq	(%rcx), %r15
// 0x141f371c9  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F228
// 0x141f371d0  488d0d49ee4d01                 leaq	0x14dee49(%rip), %rcx   # 0x143416020
// 0x141f371d7  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x141f371dc  4889542420                     movq	%rdx, 0x20(%rsp)
// 0x141f371e1  48c744243009000000             movq	$0x9, 0x30(%rsp)
// 0x141f371ea  4889d9                         movq	%rbx, %rcx
// 0x141f371ed  4889c2                         movq	%rax, %rdx
// 0x141f371f0  4d89f9                         movq	%r15, %r9
// 0x141f371f3  e8c8238f00                     callq	0x1428295c0 <.text+0x28285c0>
// 0x141f371f8  90                             nop
// Frames: F226 -> F227 -> F233
// 0x141f371f9  45896500                       movl	%r12d, (%r13)
// 0x141f371fd  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F233
// 0x141f37204  4889d9                         movq	%rbx, %rcx
// 0x141f37207  4889fa                         movq	%rdi, %rdx
// 0x141f3720a  4d89f8                         movq	%r15, %r8
// 0x141f3720d  e8ee60b1fe                     callq	0x140a4d300 <.text+0xa4c300>
// 0x141f37212  90                             nop
// Frames: F234
// 0x141f37213  488b85a0030000                 movq	0x3a0(%rbp), %rax
// 0x141f3721a  488b08                         movq	(%rax), %rcx
// 0x141f3721d  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f37224  4889fa                         movq	%rdi, %rdx
// 0x141f37227  e8d4d0ccfe                     callq	0x140c04300 <.text+0xc03300>
// 0x141f3722c  90                             nop
// Frames: F235 -> F234
// 0x141f3722d  4885c0                         testq	%rax, %rax
// 0x141f37230  0f84d0040000                   je	0x141f37706 <.text+0x1f36706>
// Frames: F234
// 0x141f37236  f30f104054                     movss	0x54(%rax), %xmm0
// 0x141f3723b  488b85a8030000                 movq	0x3a8(%rbp), %rax
// 0x141f37242  f30f1100                       movss	%xmm0, (%rax)
// Frames: F236
// 0x141f37246  498b06                         movq	(%r14), %rax
// Frames: F237
// 0x141f37249  410f104630                     movups	0x30(%r14), %xmm0
// 0x141f3724e  0f11856c010000                 movups	%xmm0, 0x16c(%rbp)
// 0x141f37255  410f104624                     movups	0x24(%r14), %xmm0
// 0x141f3725a  0f298560010000                 movaps	%xmm0, 0x160(%rbp)
// Frames: F226 -> F227 -> F238
// 0x141f37261  8b4e1c                         movl	0x1c(%rsi), %ecx
// 0x141f37264  488b5610                       movq	0x10(%rsi), %rdx
// 0x141f37268  890a                           movl	%ecx, (%rdx)
// Frames: F229 -> F238
// 0x141f3726a  488b0e                         movq	(%rsi), %rcx
// Frames: F238
// 0x141f3726d  c70100000000                   movl	$0x0, (%rcx)
// 0x141f37273  48894110                       movq	%rax, 0x10(%rcx)
// 0x141f37277  0f288560010000                 movaps	0x160(%rbp), %xmm0
// 0x141f3727e  0f114118                       movups	%xmm0, 0x18(%rcx)
// 0x141f37282  0f10856c010000                 movups	0x16c(%rbp), %xmm0
// 0x141f37289  0f114124                       movups	%xmm0, 0x24(%rcx)
// 0x141f3728d  e987030000                     jmp	0x141f37619 <.text+0x1f36619>
// Frames: F212 -> F239
// 0x141f37292  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f37299  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x141f3729d  488b9530020000                 movq	0x230(%rbp), %rdx
// 0x141f372a4  e8172bcbfe                     callq	0x140be9dc0 <.text+0xbe8dc0>
// 0x141f372a9  90                             nop
// Frames: F214 -> F239
// 0x141f372aa  488b8588020000                 movq	0x288(%rbp), %rax
// 0x141f372b1  f30f1008                       movss	(%rax), %xmm1
// 0x141f372b5  f30f105004                     movss	0x4(%rax), %xmm2
// 0x141f372ba  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f372c1  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x141f372c5  e8262bcbfe                     callq	0x140be9df0 <.text+0xbe8df0>
// 0x141f372ca  90                             nop
// Frames: F215 -> F239
// 0x141f372cb  0f28d1                         movaps	%xmm1, %xmm2
// 0x141f372ce  f30f11bd60010000               movss	%xmm7, 0x160(%rbp)
// 0x141f372d6  f30f11b564010000               movss	%xmm6, 0x164(%rbp)
// 0x141f372de  f3440f118568010000             movss	%xmm8, 0x168(%rbp)
// 0x141f372e7  48b8000000000000803f           movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
// 0x141f372f1  4889856c010000                 movq	%rax, 0x16c(%rbp)
// 0x141f372f8  c7857401000000000000           movl	$0x0, 0x174(%rbp)
// Frames: F216 -> F215 -> F239
// 0x141f37302  f20f100d36f41001               movsd	0x110f436(%rip), %xmm1  # 0x143046740
// 0x141f3730a  f20f118d10020000               movsd	%xmm1, 0x210(%rbp)
// 0x141f37312  c6851802000000                 movb	$0x0, 0x218(%rbp)
// 0x141f37319  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F215 -> F239
// 0x141f37320  4c89642430                     movq	%r12, 0x30(%rsp)
// 0x141f37325  48897c2428                     movq	%rdi, 0x28(%rsp)
// 0x141f3732a  48895c2420                     movq	%rbx, 0x20(%rsp)
// 0x141f3732f  488d8d50020000                 leaq	0x250(%rbp), %rcx
// 0x141f37336  4c8d4d20                       leaq	0x20(%rbp), %r9
// 0x141f3733a  0f28c8                         movaps	%xmm0, %xmm1
// 0x141f3733d  e8fe2acbfe                     callq	0x140be9e40 <.text+0xbe8e40>
// 0x141f37342  90                             nop
// Frames: F217 -> F239
// 0x141f37343  f30f108550020000               movss	0x250(%rbp), %xmm0
// 0x141f3734b  f30f108d54020000               movss	0x254(%rbp), %xmm1
// 0x141f37353  f30f109558020000               movss	0x258(%rbp), %xmm2
// Frames: F218 -> F220 -> F217 -> F239
// 0x141f3735b  f30f5cc7                       subss	%xmm7, %xmm0
// Frames: F218 -> F221 -> F217 -> F239
// 0x141f3735f  f30f5cce                       subss	%xmm6, %xmm1
// Frames: F218 -> F219 -> F217 -> F239
// 0x141f37363  f3410f5cd0                     subss	%xmm8, %xmm2
// Frames: F222 -> F223 -> F217 -> F239
// 0x141f37368  f30f59c0                       mulss	%xmm0, %xmm0
// 0x141f3736c  f30f59c9                       mulss	%xmm1, %xmm1
// 0x141f37370  f30f58c8                       addss	%xmm0, %xmm1
// 0x141f37374  f30f59d2                       mulss	%xmm2, %xmm2
// 0x141f37378  f30f58d1                       addss	%xmm1, %xmm2
// Frames: F157 -> F158 -> F224 -> F223 -> F217 -> F239
// 0x141f3737c  f30f51d2                       sqrtss	%xmm2, %xmm2
// 0x141f37380  488b8538020000                 movq	0x238(%rbp), %rax
// Frames: F239
// 0x141f37387  f30f5c10                       subss	(%rax), %xmm2
// 0x141f3738b  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F240
// 0x141f37392  f30f10052afafe00               movss	0xfefa2a(%rip), %xmm0   # 0x142f26dc4
// 0x141f3739a  f30f100db617ff00               movss	0xff17b6(%rip), %xmm1   # 0x142f28b58
// 0x141f373a2  e849caeafe                     callq	0x140de3df0 <.text+0xde2df0>
// 0x141f373a7  90                             nop
// Frames: F241 -> F242
// 0x141f373a8  0f57c9                         xorps	%xmm1, %xmm1
// 0x141f373ab  f30f5fc8                       maxss	%xmm0, %xmm1
// 0x141f373af  f30f10356dd6fc00               movss	0xfcd66d(%rip), %xmm6   # 0x142f04a24
// Frames: F243 -> F242
// 0x141f373b7  f30f5df1                       minss	%xmm1, %xmm6
// Frames: F226 -> F227 -> F244
// 0x141f373bb  418b471c                       movl	0x1c(%r15), %eax
// 0x141f373bf  498b4f10                       movq	0x10(%r15), %rcx
// 0x141f373c3  8901                           movl	%eax, (%rcx)
// Frames: F245 -> F246
// 0x141f373c5  f3410f104500                   movss	(%r13), %xmm0
// 0x141f373cb  f30f100d39bb0b01               movss	0x10bbb39(%rip), %xmm1  # 0x142ff2f0c
// 0x141f373d3  0f2ec8                         ucomiss	%xmm0, %xmm1
// 0x141f373d6  7668                           jbe	0x141f37440 <.text+0x1f36440>
// Frames: F247
// 0x141f373d8  498b0f                         movq	(%r15), %rcx
// 0x141f373db  f3410f105e30                   movss	0x30(%r14), %xmm3
// 0x141f373e1  0f28d6                         movaps	%xmm6, %xmm2
// 0x141f373e4  f30f5cd3                       subss	%xmm3, %xmm2
// Frames: F248 -> F246
// 0x141f373e8  f30f58d3                       addss	%xmm3, %xmm2
// Frames: F249 -> F246
// 0x141f373ec  0f2e1d41b6fe00                 ucomiss	0xfeb641(%rip), %xmm3   # 0x142f22a34
// 0x141f373f3  760d                           jbe	0x141f37402 <.text+0x1f36402>
// 0x141f373f5  f30f102537b6fe00               movss	0xfeb637(%rip), %xmm4   # 0x142f22a34
// 0x141f373fd  0f2ee2                         ucomiss	%xmm2, %xmm4
// 0x141f37400  731e                           jae	0x141f37420 <.text+0x1f36420>
// Frames: F250 -> F246
// 0x141f37402  f30f100d2ab6fe00               movss	0xfeb62a(%rip), %xmm1   # 0x142f22a34
// 0x141f3740a  0f2ecb                         ucomiss	%xmm3, %xmm1
// 0x141f3740d  7231                           jb	0x141f37440 <.text+0x1f36440>
// 0x141f3740f  0f2e151eb6fe00                 ucomiss	0xfeb61e(%rip), %xmm2   # 0x142f22a34
// 0x141f37416  7628                           jbe	0x141f37440 <.text+0x1f36440>
// 0x141f37418  f30f100d0415ff00               movss	0xff1504(%rip), %xmm1   # 0x142f28924
// Frames: F251 -> F246
// 0x141f37420  498b16                         movq	(%r14), %rdx
// Frames: F241 -> F252 -> F246
// 0x141f37423  f30f5fc8                       maxss	%xmm0, %xmm1
// 0x141f37427  f30f1015d19e0b01               movss	0x10b9ed1(%rip), %xmm2  # 0x142ff1300
// Frames: F243 -> F252 -> F246
// 0x141f3742f  f30f5dd1                       minss	%xmm1, %xmm2
// 0x141f37433  c6859702000000                 movb	$0x0, 0x297(%rbp)
// Frames: F253 -> F246
// 0x141f3743a  e891c9ccfe                     callq	0x140c03dd0 <.text+0xc02dd0>
// 0x141f3743f  90                             nop
// Frames: F254
// 0x141f37440  488b8598030000                 movq	0x398(%rbp), %rax
// 0x141f37447  4d8b36                         movq	(%r14), %r14
// Frames: F226 -> F227 -> F255 -> F256
// 0x141f3744a  8b481c                         movl	0x1c(%rax), %ecx
// 0x141f3744d  488b5010                       movq	0x10(%rax), %rdx
// 0x141f37451  890a                           movl	%ecx, (%rdx)
// Frames: F229 -> F255 -> F256
// 0x141f37453  488b38                         movq	(%rax), %rdi
// Frames: F257 -> F258 -> F259 -> F260 -> F255 -> F256
// 0x141f37456  488b5f30                       movq	0x30(%rdi), %rbx
// Frames: F261 -> F260 -> F255 -> F256
// 0x141f3745a  4c8b7f40                       movq	0x40(%rdi), %r15
// Frames: F262 -> F258 -> F259 -> F260 -> F255 -> F256
// 0x141f3745e  483b5f20                       cmpq	0x20(%rdi), %rbx
// 0x141f37462  750a                           jne	0x141f3746e <.text+0x1f3646e>
// Frames: F259 -> F260 -> F255 -> F256
// 0x141f37464  488d4f20                       leaq	0x20(%rdi), %rcx
// Frames: F263 -> F258 -> F259 -> F260 -> F255 -> F256
// 0x141f37468  e8a382e300                     callq	0x142d6f710 <.text+0x2d6e710>
// 0x141f3746d  90                             nop
// Frames: F105 -> F106 -> F107 -> F264 -> F265 -> F258 -> F259 -> F260 -> F255 -> F256
// 0x141f3746e  488b4728                       movq	0x28(%rdi), %rax
// Frames: F266 -> F265 -> F258 -> F259 -> F260 -> F255 -> F256
// 0x141f37472  4889d9                         movq	%rbx, %rcx
// 0x141f37475  48c1e106                       shlq	$0x6, %rcx
// Frames: F267 -> F268 -> F258 -> F259 -> F260 -> F255 -> F256
// 0x141f37479  c7040800000000                 movl	$0x0, (%rax,%rcx)
// 0x141f37480  c744080801000000               movl	$0x1, 0x8(%rax,%rcx)
// 0x141f37488  f30f1174080c                   movss	%xmm6, 0xc(%rax,%rcx)
// 0x141f3748e  c744081000000000               movl	$0x0, 0x10(%rax,%rcx)
// 0x141f37496  c744081800000000               movl	$0x0, 0x18(%rax,%rcx)
// 0x141f3749e  4c89740820                     movq	%r14, 0x20(%rax,%rcx)
// 0x141f374a3  c744082800000000               movl	$0x0, 0x28(%rax,%rcx)
// 0x141f374ab  c644083400                     movb	$0x0, 0x34(%rax,%rcx)
// 0x141f374b0  4c897c0838                     movq	%r15, 0x38(%rax,%rcx)
// Frames: F269 -> F258 -> F259 -> F260 -> F255 -> F256
// 0x141f374b5  48ffc3                         incq	%rbx
// 0x141f374b8  48895f30                       movq	%rbx, 0x30(%rdi)
// Frames: F270 -> F260 -> F255 -> F256
// 0x141f374bc  48ff4740                       incq	0x40(%rdi)
// Frames: F271
// 0x141f374c0  e954010000                     jmp	0x141f37619 <.text+0x1f36619>
// Frames: F272
// 0x141f374c5  488b06                         movq	(%rsi), %rax
// Frames: F122 -> F272
// 0x141f374c8  833802                         cmpl	$0x2, (%rax)
// Frames: F272
// 0x141f374cb  0f8448010000                   je	0x141f37619 <.text+0x1f36619>
// Frames: F273 -> F274 -> F275
// 0x141f374d1  f30f104020                     movss	0x20(%rax), %xmm0
// 0x141f374d6  410f2e462c                     ucomiss	0x2c(%r14), %xmm0
// 0x141f374db  7560                           jne	0x141f3753d <.text+0x1f3653d>
// 0x141f374dd  7a5e                           jp	0x141f3753d <.text+0x1f3653d>
// Frames: F276 -> F274 -> F275
// 0x141f374df  f30f104024                     movss	0x24(%rax), %xmm0
// 0x141f374e4  410f2e4630                     ucomiss	0x30(%r14), %xmm0
// 0x141f374e9  7552                           jne	0x141f3753d <.text+0x1f3653d>
// 0x141f374eb  7a50                           jp	0x141f3753d <.text+0x1f3653d>
// Frames: F277 -> F274 -> F275
// 0x141f374ed  f30f104028                     movss	0x28(%rax), %xmm0
// 0x141f374f2  410f2e4634                     ucomiss	0x34(%r14), %xmm0
// 0x141f374f7  7544                           jne	0x141f3753d <.text+0x1f3653d>
// 0x141f374f9  7a42                           jp	0x141f3753d <.text+0x1f3653d>
// Frames: F278 -> F274 -> F275
// 0x141f374fb  f30f10402c                     movss	0x2c(%rax), %xmm0
// 0x141f37500  410f2e4638                     ucomiss	0x38(%r14), %xmm0
// 0x141f37505  7536                           jne	0x141f3753d <.text+0x1f3653d>
// 0x141f37507  7a34                           jp	0x141f3753d <.text+0x1f3653d>
// Frames: F279 -> F274 -> F275
// 0x141f37509  f30f104018                     movss	0x18(%rax), %xmm0
// Frames: F280 -> F279 -> F274 -> F275
// 0x141f3750e  410f2e4624                     ucomiss	0x24(%r14), %xmm0
// Frames: F279 -> F274 -> F275
// 0x141f37513  7528                           jne	0x141f3753d <.text+0x1f3653d>
// 0x141f37515  7a26                           jp	0x141f3753d <.text+0x1f3653d>
// 0x141f37517  f30f10401c                     movss	0x1c(%rax), %xmm0
// 0x141f3751c  410f2e4628                     ucomiss	0x28(%r14), %xmm0
// 0x141f37521  751a                           jne	0x141f3753d <.text+0x1f3653d>
// 0x141f37523  7a18                           jp	0x141f3753d <.text+0x1f3653d>
// Frames: F281 -> F274 -> F275
// 0x141f37525  0fb64830                       movzbl	0x30(%rax), %ecx
// Frames: F282 -> F281 -> F274 -> F275
// 0x141f37529  413a4e3c                       cmpb	0x3c(%r14), %cl
// Frames: F281 -> F274 -> F275
// 0x141f3752d  750e                           jne	0x141f3753d <.text+0x1f3653d>
// Frames: F283 -> F274 -> F275
// 0x141f3752f  0fb64831                       movzbl	0x31(%rax), %ecx
// Frames: F284 -> F283 -> F274 -> F275
// 0x141f37533  413a4e3d                       cmpb	0x3d(%r14), %cl
// Frames: F275
// 0x141f37537  0f84dc000000                   je	0x141f37619 <.text+0x1f36619>
// Frames: F226 -> F227 -> F285
// 0x141f3753d  488b8d60030000                 movq	0x360(%rbp), %rcx
// 0x141f37544  448b791c                       movl	0x1c(%rcx), %r15d
// 0x141f37548  4c8b6110                       movq	0x10(%rcx), %r12
// 0x141f3754c  45893c24                       movl	%r15d, (%r12)
// Frames: F229 -> F285
// 0x141f37550  488b39                         movq	(%rcx), %rdi
// Frames: F286
// 0x141f37553  0f107010                       movups	0x10(%rax), %xmm6
// Frames: F287
// 0x141f37557  440f104020                     movups	0x20(%rax), %xmm8
// Frames: F288
// 0x141f3755c  498b5e24                       movq	0x24(%r14), %rbx
// Frames: F289
// 0x141f37560  410f107e2c                     movups	0x2c(%r14), %xmm7
// Frames: F190 -> F191 -> F192 -> F193 -> F194 -> F195 -> F196 -> F285
// 0x141f37565  e8a6bd0cfe                     callq	0x140003310 <.text+0x2310>
// Frames: F197 -> F191 -> F192 -> F193 -> F194 -> F195 -> F196 -> F285
// 0x141f3756a  b950000000                     movl	$0x50, %ecx
// 0x141f3756f  ba08000000                     movl	$0x8, %edx
// 0x141f37574  e867e376fe                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F194 -> F195 -> F196 -> F285
// 0x141f37579  4885c0                         testq	%rax, %rax
// 0x141f3757c  0f849a010000                   je	0x141f3771c <.text+0x1f3671c>
// Frames: F198 -> F196 -> F285
// 0x141f37582  48c70001000000                 movq	$0x1, (%rax)
// 0x141f37589  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x141f37591  c6401002                       movb	$0x2, 0x10(%rax)
// 0x141f37595  450fc6c0e1                     shufps	$0xe1, %xmm8, %xmm8     # xmm8 = xmm8[1,0,2,3]
// 0x141f3759a  440f114014                     movups	%xmm8, 0x14(%rax)
// 0x141f3759f  0fc6ffe1                       shufps	$0xe1, %xmm7, %xmm7     # xmm7 = xmm7[1,0,2,3]
// 0x141f375a3  0f117824                       movups	%xmm7, 0x24(%rax)
// 0x141f375a7  0f117038                       movups	%xmm6, 0x38(%rax)
// 0x141f375ab  48895848                       movq	%rbx, 0x48(%rax)
// Frames: F285
// 0x141f375af  48898568010000                 movq	%rax, 0x168(%rbp)
// 0x141f375b6  48c7856001000008000000         movq	$0x8, 0x160(%rbp)
// 0x141f375c1  4c8d0580ea4d01                 leaq	0x14dea80(%rip), %r8    # 0x143416048
// 0x141f375c8  488d9560010000                 leaq	0x160(%rbp), %rdx
// 0x141f375cf  4889f9                         movq	%rdi, %rcx
// 0x141f375d2  e84940c1fe                     callq	0x140b4b620 <.text+0xb4a620>
// 0x141f375d7  90                             nop
// 0x141f375d8  488b85a0030000                 movq	0x3a0(%rbp), %rax
// Frames: F290
// 0x141f375df  488b08                         movq	(%rax), %rcx
// 0x141f375e2  498b1e                         movq	(%r14), %rbx
// 0x141f375e5  4889da                         movq	%rbx, %rdx
// 0x141f375e8  e813cdccfe                     callq	0x140c04300 <.text+0xc03300>
// 0x141f375ed  90                             nop
// Frames: F235 -> F290
// 0x141f375ee  4885c0                         testq	%rax, %rax
// 0x141f375f1  0f8437010000                   je	0x141f3772e <.text+0x1f3672e>
// Frames: F290
// 0x141f375f7  f30f105854                     movss	0x54(%rax), %xmm3
// Frames: F291
// 0x141f375fc  f3410f105500                   movss	(%r13), %xmm2
// 0x141f37602  0f2ed3                         ucomiss	%xmm3, %xmm2
// 0x141f37605  7502                           jne	0x141f37609 <.text+0x1f36609>
// 0x141f37607  7b10                           jnp	0x141f37619 <.text+0x1f36619>
// Frames: F226 -> F227 -> F292
// 0x141f37609  45893c24                       movl	%r15d, (%r12)
// Frames: F292
// 0x141f3760d  4889f9                         movq	%rdi, %rcx
// 0x141f37610  4889da                         movq	%rbx, %rdx
// 0x141f37613  e8b8f6c0fe                     callq	0x140b46cd0 <.text+0xb45cd0>
// 0x141f37618  90                             nop
// 0x141f37619  0fb69d70020000                 movzbl	0x270(%rbp), %ebx
// Frames: F73 -> F170
// 0x141f37620  84db                           testb	%bl, %bl
// 0x141f37622  743e                           je	0x141f37662 <.text+0x1f36662>
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37624  e8576ff5fe                     callq	0x140e8e580 <.text+0xe8d580>
// Frames: F60 -> F54 -> F55 -> F56 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37629  80786801                       cmpb	$0x1, 0x68(%rax)
// 0x141f3762d  0f8582000000                   jne	0x141f376b5 <.text+0x1f366b5>
// Frames: F61 -> F62 -> F63 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37633  48833800                       cmpq	$0x0, (%rax)
// 0x141f37637  488b9580020000                 movq	0x280(%rbp), %rdx
// 0x141f3763e  7554                           jne	0x141f37694 <.text+0x1f36694>
// Frames: F66 -> F67 -> F68 -> F62 -> F63 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37640  48c700ffffffff                 movq	$-0x1, (%rax)
// 0x141f37647  48898588020000                 movq	%rax, 0x288(%rbp)
// Frames: F69 -> F70 -> F63 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f3764e  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F172 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37652  e89952f5fe                     callq	0x140e8c8f0 <.text+0xe8b8f0>
// 0x141f37657  90                             nop
// Frames: F66 -> F67 -> F72 -> F73 -> F73 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37658  488b8588020000                 movq	0x288(%rbp), %rax
// 0x141f3765f  48ff00                         incq	(%rax)
// Frames: F170
// 0x141f37662  0f28b5a0020000                 movaps	0x2a0(%rbp), %xmm6
// 0x141f37669  0f28bdb0020000                 movaps	0x2b0(%rbp), %xmm7
// 0x141f37670  440f2885c0020000               movaps	0x2c0(%rbp), %xmm8
// 0x141f37678  440f288dd0020000               movaps	0x2d0(%rbp), %xmm9
// 0x141f37680  4881c468030000                 addq	$0x368, %rsp            # imm = 0x368
// 0x141f37687  5b                             popq	%rbx
// 0x141f37688  5f                             popq	%rdi
// 0x141f37689  5e                             popq	%rsi
// 0x141f3768a  415c                           popq	%r12
// 0x141f3768c  415d                           popq	%r13
// 0x141f3768e  415e                           popq	%r14
// 0x141f37690  415f                           popq	%r15
// 0x141f37692  5d                             popq	%rbp
// 0x141f37693  c3                             retq
// 0x141f37694  488d0db50b6501                 leaq	0x1650bb5(%rip), %rcx   # 0x143588250
// 0x141f3769b  e8c09ee700                     callq	0x142db1560 <.text+0x2db0560>
// Frames: F293 -> F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F3
// 0x141f376a0  4889c1                         movq	%rax, %rcx
// 0x141f376a3  31d2                           xorl	%edx, %edx
// 0x141f376a5  e886b3ed00                     callq	0x142e12a30 <.text+0x2e11a30>
// Frames: F294 -> F56 -> F57 -> F58 -> F59 -> F3
// 0x141f376aa  4885c0                         testq	%rax, %rax
// 0x141f376ad  0f85c9f2ffff                   jne	0x141f3697c <.text+0x1f3597c>
// 0x141f376b3  eb13                           jmp	0x141f376c8 <.text+0x1f366c8>
// Frames: F293 -> F54 -> F55 -> F56 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f376b5  4889c1                         movq	%rax, %rcx
// 0x141f376b8  31d2                           xorl	%edx, %edx
// 0x141f376ba  e871b3ed00                     callq	0x142e12a30 <.text+0x2e11a30>
// Frames: F294 -> F56 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f376bf  4885c0                         testq	%rax, %rax
// 0x141f376c2  0f856bffffff                   jne	0x141f37633 <.text+0x1f36633>
// 0x141f376c8  488d0d110b6501                 leaq	0x1650b11(%rip), %rcx   # 0x1435881e0
// 0x141f376cf  e88c96e700                     callq	0x142db0d60 <.text+0x2dafd60>
// 0x141f376d4  90                             nop
// Frames: F295 -> F195 -> F196 -> F189
// 0x141f376d5  b910000000                     movl	$0x10, %ecx
// 0x141f376da  ba30000000                     movl	$0x30, %edx
// 0x141f376df  e8fd9be700                     callq	0x142db12e1 <.text+0x2db02e1>
// 0x141f376e4  90                             nop
// 0x141f376e5  e993000000                     jmp	0x141f3777d <.text+0x1f3677d>
// Frames: F293 -> F54 -> F55 -> F56 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f376ea  4889c1                         movq	%rax, %rcx
// 0x141f376ed  31d2                           xorl	%edx, %edx
// 0x141f376ef  e83cb3ed00                     callq	0x142e12a30 <.text+0x2e11a30>
// Frames: F294 -> F56 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f376f4  4885c0                         testq	%rax, %rax
// 0x141f376f7  488b9580020000                 movq	0x280(%rbp), %rdx
// 0x141f376fe  0f85bdf5ffff                   jne	0x141f36cc1 <.text+0x1f35cc1>
// 0x141f37704  ebc2                           jmp	0x141f376c8 <.text+0x1f366c8>
// Frames: F296 -> F234
// 0x141f37706  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f3770d  488d0d1ce94d01                 leaq	0x14de91c(%rip), %rcx   # 0x143416030
// 0x141f37714  e8079fe700                     callq	0x142db1620 <.text+0x2db0620>
// 0x141f37719  90                             nop
// 0x141f3771a  eb61                           jmp	0x141f3777d <.text+0x1f3677d>
// Frames: F295 -> F195 -> F196 -> F285
// 0x141f3771c  b908000000                     movl	$0x8, %ecx
// 0x141f37721  ba50000000                     movl	$0x50, %edx
// 0x141f37726  e8b69be700                     callq	0x142db12e1 <.text+0x2db02e1>
// 0x141f3772b  90                             nop
// 0x141f3772c  eb4f                           jmp	0x141f3777d <.text+0x1f3677d>
// Frames: F296 -> F290
// 0x141f3772e  488d0d2be94d01                 leaq	0x14de92b(%rip), %rcx   # 0x143416060
// 0x141f37735  e8e69ee700                     callq	0x142db1620 <.text+0x2db0620>
// 0x141f3773a  90                             nop
// 0x141f3773b  eb40                           jmp	0x141f3777d <.text+0x1f3677d>
// Frames: F297 -> F80
// 0x141f3773d  898d60010000                   movl	%ecx, 0x160(%rbp)
// 0x141f37743  44898564010000                 movl	%r8d, 0x164(%rbp)
// 0x141f3774a  48898568010000                 movq	%rax, 0x168(%rbp)
// 0x141f37751  488d0558e84d01                 leaq	0x14de858(%rip), %rax   # 0x143415fb0
// 0x141f37758  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3775d  488d0db4e94d01                 leaq	0x14de9b4(%rip), %rcx   # 0x143416118
// 0x141f37764  4c8d0d8de94d01                 leaq	0x14de98d(%rip), %r9    # 0x1434160f8
// 0x141f3776b  4c8d8560010000                 leaq	0x160(%rbp), %r8
// 0x141f37772  ba2b000000                     movl	$0x2b, %edx
// 0x141f37777  e8c49ee700                     callq	0x142db1640 <.text+0x2db0640>
// 0x141f3777c  90                             nop
// 0x141f3777d  0f0b                           ud2
// Frames: F298 -> F8 -> F53 -> F3
// 0x141f3777f  488d0582ea4d01                 leaq	0x14dea82(%rip), %rax   # 0x143416208
// 0x141f37786  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3778b  488d0dd8e74d01                 leaq	0x14de7d8(%rip), %rcx   # 0x143415f6a
// 0x141f37792  ba41000000                     movl	$0x41, %edx
// 0x141f37797  41b941000000                   movl	$0x41, %r9d
// 0x141f3779d  e89e9de700                     callq	0x142db1540 <.text+0x2db0540>
// Frames: F299 -> F8 -> F9 -> F3
// 0x141f377a2  488d057fea4d01                 leaq	0x14dea7f(%rip), %rax   # 0x143416228
// 0x141f377a9  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f377ae  488d0d8ee94d01                 leaq	0x14de98e(%rip), %rcx   # 0x143416143
// 0x141f377b5  ba5b000000                     movl	$0x5b, %edx
// 0x141f377ba  4531c0                         xorl	%r8d, %r8d
// 0x141f377bd  e87e9de700                     callq	0x142db1540 <.text+0x2db0540>
// Frames: F298 -> F8 -> F10 -> F3
// 0x141f377c2  488d0577ea4d01                 leaq	0x14dea77(%rip), %rax   # 0x143416240
// 0x141f377c9  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f377ce  488d0d6ee94d01                 leaq	0x14de96e(%rip), %rcx   # 0x143416143
// 0x141f377d5  ba5b000000                     movl	$0x5b, %edx
// 0x141f377da  41b95b000000                   movl	$0x5b, %r9d
// 0x141f377e0  e85b9de700                     callq	0x142db1540 <.text+0x2db0540>
// 0x141f377e5  cc                             int3
// 0x141f377e6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x141f377f0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f377f5  55                             pushq	%rbp
// 0x141f377f6  4157                           pushq	%r15
// 0x141f377f8  4156                           pushq	%r14
// 0x141f377fa  4155                           pushq	%r13
// 0x141f377fc  4154                           pushq	%r12
// 0x141f377fe  56                             pushq	%rsi
// 0x141f377ff  57                             pushq	%rdi
// 0x141f37800  53                             pushq	%rbx
// 0x141f37801  4883ec78                       subq	$0x78, %rsp
// 0x141f37805  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3780c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141f37812  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141f37818  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141f3781d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// 0x141f37822  488b8588020000                 movq	0x288(%rbp), %rax
// Frames: F66 -> F67 -> F72 -> F73 -> F73 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37829  48ff00                         incq	(%rax)
// 0x141f3782c  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f37831  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f37836  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f3783c  440f284c2430                   movaps	0x30(%rsp), %xmm9
// Frames: F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37842  4883c478                       addq	$0x78, %rsp
// 0x141f37846  5b                             popq	%rbx
// 0x141f37847  5f                             popq	%rdi
// 0x141f37848  5e                             popq	%rsi
// 0x141f37849  415c                           popq	%r12
// 0x141f3784b  415d                           popq	%r13
// 0x141f3784d  415e                           popq	%r14
// 0x141f3784f  415f                           popq	%r15
// 0x141f37851  5d                             popq	%rbp
// 0x141f37852  c3                             retq
// 0x141f37853  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x141f37860  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f37865  55                             pushq	%rbp
// 0x141f37866  4157                           pushq	%r15
// 0x141f37868  4156                           pushq	%r14
// 0x141f3786a  4155                           pushq	%r13
// 0x141f3786c  4154                           pushq	%r12
// 0x141f3786e  56                             pushq	%rsi
// 0x141f3786f  57                             pushq	%rdi
// 0x141f37870  53                             pushq	%rbx
// 0x141f37871  4883ec78                       subq	$0x78, %rsp
// 0x141f37875  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3787c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141f37882  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141f37888  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141f3788d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// 0x141f37892  488b8588020000                 movq	0x288(%rbp), %rax
// Frames: F66 -> F67 -> F72 -> F73 -> F73 -> F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37899  48ff00                         incq	(%rax)
// 0x141f3789c  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f378a1  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f378a6  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f378ac  440f284c2430                   movaps	0x30(%rsp), %xmm9
// Frames: F64 -> F65 -> F57 -> F58 -> F171 -> F73 -> F73 -> F170
// 0x141f378b2  4883c478                       addq	$0x78, %rsp
// 0x141f378b6  5b                             popq	%rbx
// 0x141f378b7  5f                             popq	%rdi
// 0x141f378b8  5e                             popq	%rsi
// 0x141f378b9  415c                           popq	%r12
// 0x141f378bb  415d                           popq	%r13
// 0x141f378bd  415e                           popq	%r14
// 0x141f378bf  415f                           popq	%r15
// 0x141f378c1  5d                             popq	%rbp
// 0x141f378c2  c3                             retq
// 0x141f378c3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x141f378d0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f378d5  55                             pushq	%rbp
// 0x141f378d6  4157                           pushq	%r15
// 0x141f378d8  4156                           pushq	%r14
// 0x141f378da  4155                           pushq	%r13
// 0x141f378dc  4154                           pushq	%r12
// 0x141f378de  56                             pushq	%rsi
// 0x141f378df  57                             pushq	%rdi
// 0x141f378e0  53                             pushq	%rbx
// 0x141f378e1  4883ec78                       subq	$0x78, %rsp
// 0x141f378e5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f378ec  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141f378f2  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141f378f8  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141f378fd  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F300
// 0x141f37902  80bd9602000000                 cmpb	$0x0, 0x296(%rbp)
// 0x141f37909  744c                           je	0x141f37957 <.text+0x1f36957>
// Frames: F202 -> F203 -> F73 -> F73 -> F73 -> F300
// 0x141f3790b  488b9540010000                 movq	0x140(%rbp), %rdx
// Frames: F201 -> F202 -> F203 -> F73 -> F73 -> F73 -> F300
// 0x141f37912  4885d2                         testq	%rdx, %rdx
// 0x141f37915  7412                           je	0x141f37929 <.text+0x1f36929>
// Frames: F202 -> F203 -> F73 -> F73 -> F73 -> F300
// 0x141f37917  488b8d48010000                 movq	0x148(%rbp), %rcx
// Frames: F205 -> F206 -> F207 -> F208 -> F209 -> F203 -> F73 -> F73 -> F73 -> F300
// 0x141f3791e  41b801000000                   movl	$0x1, %r8d
// 0x141f37924  e8c7df76fe                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F300
// 0x141f37929  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f37930  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f37935  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f3793a  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f37940  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x141f37946  4883c478                       addq	$0x78, %rsp
// 0x141f3794a  5b                             popq	%rbx
// 0x141f3794b  5f                             popq	%rdi
// 0x141f3794c  5e                             popq	%rsi
// 0x141f3794d  415c                           popq	%r12
// 0x141f3794f  415d                           popq	%r13
// 0x141f37951  415e                           popq	%r14
// 0x141f37953  415f                           popq	%r15
// 0x141f37955  5d                             popq	%rbp
// 0x141f37956  c3                             retq
// 0x141f37957  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f3795e  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f37963  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f37968  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f3796e  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x141f37974  4883c478                       addq	$0x78, %rsp
// 0x141f37978  5b                             popq	%rbx
// 0x141f37979  5f                             popq	%rdi
// 0x141f3797a  5e                             popq	%rsi
// 0x141f3797b  415c                           popq	%r12
// 0x141f3797d  415d                           popq	%r13
// 0x141f3797f  415e                           popq	%r14
// 0x141f37981  415f                           popq	%r15
// 0x141f37983  5d                             popq	%rbp
// 0x141f37984  c3                             retq
// 0x141f37985  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x141f37990  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f37995  55                             pushq	%rbp
// 0x141f37996  4157                           pushq	%r15
// 0x141f37998  4156                           pushq	%r14
// 0x141f3799a  4155                           pushq	%r13
// 0x141f3799c  4154                           pushq	%r12
// 0x141f3799e  56                             pushq	%rsi
// 0x141f3799f  57                             pushq	%rdi
// 0x141f379a0  53                             pushq	%rbx
// 0x141f379a1  4883ec78                       subq	$0x78, %rsp
// 0x141f379a5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f379ac  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141f379b2  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141f379b8  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141f379bd  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// 0x141f379c2  488b8580020000                 movq	0x280(%rbp), %rax
// Frames: F66 -> F67 -> F72 -> F73 -> F73 -> F64 -> F65 -> F57 -> F58 -> F59 -> F3
// 0x141f379c9  48ff00                         incq	(%rax)
// 0x141f379cc  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f379d1  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f379d6  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f379dc  440f284c2430                   movaps	0x30(%rsp), %xmm9
// Frames: F64 -> F65 -> F57 -> F58 -> F59 -> F3
// 0x141f379e2  4883c478                       addq	$0x78, %rsp
// 0x141f379e6  5b                             popq	%rbx
// 0x141f379e7  5f                             popq	%rdi
// 0x141f379e8  5e                             popq	%rsi
// 0x141f379e9  415c                           popq	%r12
// 0x141f379eb  415d                           popq	%r13
// 0x141f379ed  415e                           popq	%r14
// 0x141f379ef  415f                           popq	%r15
// 0x141f379f1  5d                             popq	%rbp
// 0x141f379f2  c3                             retq
// 0x141f379f3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x141f37a00  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f37a05  55                             pushq	%rbp
// 0x141f37a06  4157                           pushq	%r15
// 0x141f37a08  4156                           pushq	%r14
// 0x141f37a0a  4155                           pushq	%r13
// 0x141f37a0c  4154                           pushq	%r12
// 0x141f37a0e  56                             pushq	%rsi
// 0x141f37a0f  57                             pushq	%rdi
// 0x141f37a10  53                             pushq	%rbx
// 0x141f37a11  4883ec78                       subq	$0x78, %rsp
// 0x141f37a15  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f37a1c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141f37a22  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141f37a28  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141f37a2d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F201 -> F202 -> F203 -> F73 -> F73 -> F73 -> F204
// 0x141f37a32  4883bd7802000000               cmpq	$0x0, 0x278(%rbp)
// 0x141f37a3a  7419                           je	0x141f37a55 <.text+0x1f36a55>
// Frames: F205 -> F206 -> F207 -> F208 -> F209 -> F203 -> F73 -> F73 -> F73 -> F204
// 0x141f37a3c  41b801000000                   movl	$0x1, %r8d
// 0x141f37a42  488b8d40020000                 movq	0x240(%rbp), %rcx
// 0x141f37a49  488b9578020000                 movq	0x278(%rbp), %rdx
// 0x141f37a50  e89bde76fe                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F204
// 0x141f37a55  c6859702000000                 movb	$0x0, 0x297(%rbp)
// 0x141f37a5c  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f37a61  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f37a66  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f37a6c  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x141f37a72  4883c478                       addq	$0x78, %rsp
// 0x141f37a76  5b                             popq	%rbx
// 0x141f37a77  5f                             popq	%rdi
// 0x141f37a78  5e                             popq	%rsi
// 0x141f37a79  415c                           popq	%r12
// 0x141f37a7b  415d                           popq	%r13
// 0x141f37a7d  415e                           popq	%r14
// 0x141f37a7f  415f                           popq	%r15
// 0x141f37a81  5d                             popq	%rbp
// 0x141f37a82  c3                             retq
// 0x141f37a83  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x141f37a90  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f37a95  55                             pushq	%rbp
// 0x141f37a96  4157                           pushq	%r15
// 0x141f37a98  4156                           pushq	%r14
// 0x141f37a9a  4155                           pushq	%r13
// 0x141f37a9c  4154                           pushq	%r12
// 0x141f37a9e  56                             pushq	%rsi
// 0x141f37a9f  57                             pushq	%rdi
// 0x141f37aa0  53                             pushq	%rbx
// 0x141f37aa1  4883ec78                       subq	$0x78, %rsp
// 0x141f37aa5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f37aac  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141f37ab2  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141f37ab8  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141f37abd  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F301
// 0x141f37ac2  80bd9702000000                 cmpb	$0x0, 0x297(%rbp)
// 0x141f37ac9  744d                           je	0x141f37b18 <.text+0x1f36b18>
// Frames: F202 -> F203 -> F73 -> F73 -> F301
// 0x141f37acb  488b85f0000000                 movq	0xf0(%rbp), %rax
// Frames: F201 -> F202 -> F203 -> F73 -> F73 -> F301
// 0x141f37ad2  4885c0                         testq	%rax, %rax
// 0x141f37ad5  741a                           je	0x141f37af1 <.text+0x1f36af1>
// Frames: F202 -> F203 -> F73 -> F73 -> F301
// 0x141f37ad7  488b8df8000000                 movq	0xf8(%rbp), %rcx
// Frames: F302 -> F303 -> F202 -> F203 -> F73 -> F73 -> F301
// 0x141f37ade  48c1e002                       shlq	$0x2, %rax
// 0x141f37ae2  488d14c0                       leaq	(%rax,%rax,8), %rdx
// Frames: F205 -> F206 -> F207 -> F208 -> F209 -> F203 -> F73 -> F73 -> F301
// 0x141f37ae6  41b804000000                   movl	$0x4, %r8d
// 0x141f37aec  e8ffdd76fe                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F301
// 0x141f37af1  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f37af6  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f37afb  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f37b01  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x141f37b07  4883c478                       addq	$0x78, %rsp
// 0x141f37b0b  5b                             popq	%rbx
// 0x141f37b0c  5f                             popq	%rdi
// 0x141f37b0d  5e                             popq	%rsi
// 0x141f37b0e  415c                           popq	%r12
// 0x141f37b10  415d                           popq	%r13
// 0x141f37b12  415e                           popq	%r14
// 0x141f37b14  415f                           popq	%r15
// 0x141f37b16  5d                             popq	%rbp
// 0x141f37b17  c3                             retq
// 0x141f37b18  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f37b1d  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f37b22  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f37b28  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x141f37b2e  4883c478                       addq	$0x78, %rsp
// 0x141f37b32  5b                             popq	%rbx
// 0x141f37b33  5f                             popq	%rdi
// 0x141f37b34  5e                             popq	%rsi
// 0x141f37b35  415c                           popq	%r12
// 0x141f37b37  415d                           popq	%r13
// 0x141f37b39  415e                           popq	%r14
// 0x141f37b3b  415f                           popq	%r15
// 0x141f37b3d  5d                             popq	%rbp
// 0x141f37b3e  c3                             retq
// 0x141f37b3f  90                             nop
// 0x141f37b40  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f37b45  55                             pushq	%rbp
// 0x141f37b46  4157                           pushq	%r15
// 0x141f37b48  4156                           pushq	%r14
// 0x141f37b4a  4155                           pushq	%r13
// 0x141f37b4c  4154                           pushq	%r12
// 0x141f37b4e  56                             pushq	%rsi
// 0x141f37b4f  57                             pushq	%rdi
// 0x141f37b50  53                             pushq	%rbx
// 0x141f37b51  4883ec78                       subq	$0x78, %rsp
// 0x141f37b55  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f37b5c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x141f37b62  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x141f37b68  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x141f37b6d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F73 -> F170
// 0x141f37b72  80bd7002000000                 cmpb	$0x0, 0x270(%rbp)
// 0x141f37b79  742f                           je	0x141f37baa <.text+0x1f36baa>
// 0x141f37b7b  488d85d8000000                 leaq	0xd8(%rbp), %rax
// 0x141f37b82  48898530010000                 movq	%rax, 0x130(%rbp)
// 0x141f37b89  488d8530010000                 leaq	0x130(%rbp), %rax
// 0x141f37b90  48898528010000                 movq	%rax, 0x128(%rbp)
// Frames: F58 -> F171 -> F73 -> F73 -> F170
// 0x141f37b97  488d0d0ae64d01                 leaq	0x14de60a(%rip), %rcx   # 0x1434161a8
// 0x141f37b9e  488d9528010000                 leaq	0x128(%rbp), %rdx
// 0x141f37ba5  e886918e00                     callq	0x142820d30 <.text+0x281fd30>
// Frames: F1
// 0x141f37baa  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x141f37baf  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x141f37bb4  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x141f37bba  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x141f37bc0  4883c478                       addq	$0x78, %rsp
// 0x141f37bc4  5b                             popq	%rbx
// 0x141f37bc5  5f                             popq	%rdi
// 0x141f37bc6  5e                             popq	%rsi
// 0x141f37bc7  415c                           popq	%r12
// 0x141f37bc9  415d                           popq	%r13
// 0x141f37bcb  415e                           popq	%r14
// 0x141f37bcd  415f                           popq	%r15
// 0x141f37bcf  5d                             popq	%rbp
// 0x141f37bd0  c3                             retq
//
// RANGE rva_1f37be0_len_1d9: RVA 0x1f37be0, 473 PDB bytes; 117 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f37be0_len_1d9.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2437:10508: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// Frames: F304
// 0x141f37be0  55                             pushq	%rbp
// 0x141f37be1  4156                           pushq	%r14
// 0x141f37be3  56                             pushq	%rsi
// 0x141f37be4  57                             pushq	%rdi
// 0x141f37be5  53                             pushq	%rbx
// 0x141f37be6  4881ec70010000                 subq	$0x170, %rsp            # imm = 0x170
// 0x141f37bed  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x141f37bf5  48c785e8000000feffffff         movq	$-0x2, 0xe8(%rbp)
// 0x141f37c00  4c89cf                         movq	%r9, %rdi
// 0x141f37c03  4c89c3                         movq	%r8, %rbx
// 0x141f37c06  4989d6                         movq	%rdx, %r14
// 0x141f37c09  4889ce                         movq	%rcx, %rsi
// Frames: F190 -> F191 -> F192 -> F193 -> F194 -> F305 -> F306
// 0x141f37c0c  e8ffb60cfe                     callq	0x140003310 <.text+0x2310>
// Frames: F197 -> F191 -> F192 -> F193 -> F194 -> F305 -> F306
// 0x141f37c11  b908000000                     movl	$0x8, %ecx
// 0x141f37c16  ba08000000                     movl	$0x8, %edx
// 0x141f37c1b  e8c0dc76fe                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F194 -> F305 -> F306
// 0x141f37c20  4885c0                         testq	%rax, %rax
// 0x141f37c23  0f8432010000                   je	0x141f37d5b <.text+0x1f36d5b>
// Frames: F197 -> F191 -> F192 -> F193 -> F307 -> F308 -> F309 -> F310 -> F311 -> F312 -> F313 -> F314 -> F315 -> F316 -> F231 -> F317
// 0x141f37c29  488985e0000000                 movq	%rax, 0xe0(%rbp)
// 0x141f37c30  e8dbb60cfe                     callq	0x140003310 <.text+0x2310>
// 0x141f37c35  b91a000000                     movl	$0x1a, %ecx
// 0x141f37c3a  ba01000000                     movl	$0x1, %edx
// 0x141f37c3f  e89cdc76fe                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F318 -> F308 -> F309 -> F310 -> F311 -> F312 -> F313 -> F314 -> F315 -> F316 -> F231 -> F317
// 0x141f37c44  4885c0                         testq	%rax, %rax
// 0x141f37c47  0f841e010000                   je	0x141f37d6b <.text+0x1f36d6b>
// Frames: F319 -> F320 -> F321 -> F312 -> F313 -> F314 -> F315 -> F316 -> F231 -> F317
// 0x141f37c4d  0f10052ee44d01                 movups	0x14de42e(%rip), %xmm0  # 0x143416082
// 0x141f37c54  0f11400a                       movups	%xmm0, 0xa(%rax)
// 0x141f37c58  0f100519e44d01                 movups	0x14de419(%rip), %xmm0  # 0x143416078
// 0x141f37c5f  0f1100                         movups	%xmm0, (%rax)
// Frames: F322
// 0x141f37c62  48c745481a000000               movq	$0x1a, 0x48(%rbp)
// 0x141f37c6a  48894550                       movq	%rax, 0x50(%rbp)
// 0x141f37c6e  48c745581a000000               movq	$0x1a, 0x58(%rbp)
// 0x141f37c76  48b80000000000000080           movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
// 0x141f37c80  48894540                       movq	%rax, 0x40(%rbp)
// 0x141f37c84  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x141f37c88  488d5540                       leaq	0x40(%rbp), %rdx
// 0x141f37c8c  e88f91a8fe                     callq	0x1409c0e20 <.text+0x9bfe20>
// 0x141f37c91  90                             nop
// Frames: F323
// 0x141f37c92  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x141f37c96  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x141f37c9a  0f57d2                         xorps	%xmm2, %xmm2
// 0x141f37c9d  e82e91a8fe                     callq	0x1409c0dd0 <.text+0x9bfdd0>
// 0x141f37ca2  90                             nop
// Frames: F324
// 0x141f37ca3  410f2806                       movaps	(%r14), %xmm0
// Frames: F322
// 0x141f37ca7  0f2985d0000000                 movaps	%xmm0, 0xd0(%rbp)
// Frames: F324
// 0x141f37cae  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x141f37cb2  488d5540                       leaq	0x40(%rbp), %rdx
// 0x141f37cb6  4c8d85d0000000                 leaq	0xd0(%rbp), %r8
// 0x141f37cbd  e82ebaa8fe                     callq	0x1409c36f0 <.text+0x9c26f0>
// 0x141f37cc2  90                             nop
// Frames: F325
// 0x141f37cc3  f3410f105610                   movss	0x10(%r14), %xmm2
// 0x141f37cc9  f30f5915ef80ef00               mulss	0xef80ef(%rip), %xmm2   # 0x142e2fdc0
// 0x141f37cd1  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x141f37cd5  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x141f37cd9  e882b9a8fe                     callq	0x1409c3660 <.text+0x9c2660>
// 0x141f37cde  90                             nop
// Frames: F326
// 0x141f37cdf  418b461c                       movl	0x1c(%r14), %eax
// 0x141f37ce3  8985d8000000                   movl	%eax, 0xd8(%rbp)
// 0x141f37ce9  498b4614                       movq	0x14(%r14), %rax
// 0x141f37ced  488985d0000000                 movq	%rax, 0xd0(%rbp)
// 0x141f37cf4  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x141f37cf8  488d5540                       leaq	0x40(%rbp), %rdx
// 0x141f37cfc  4c8d85d0000000                 leaq	0xd0(%rbp), %r8
// 0x141f37d03  e8b8b9a8fe                     callq	0x1409c36c0 <.text+0x9c26c0>
// 0x141f37d08  90                             nop
// Frames: F327
// 0x141f37d09  488b8540010000                 movq	0x140(%rbp), %rax
// 0x141f37d10  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f37d15  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x141f37d19  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x141f37d1d  4989d8                         movq	%rbx, %r8
// 0x141f37d20  4989f9                         movq	%rdi, %r9
// 0x141f37d23  e8489aa8fe                     callq	0x1409c1770 <.text+0x9c0770>
// 0x141f37d28  90                             nop
// Frames: F328
// 0x141f37d29  488b4540                       movq	0x40(%rbp), %rax
// 0x141f37d2d  488b8de0000000                 movq	0xe0(%rbp), %rcx
// Frames: F306
// 0x141f37d34  488901                         movq	%rax, (%rcx)
// Frames: F329 -> F330 -> F331 -> F306
// 0x141f37d37  48c70601000000                 movq	$0x1, (%rsi)
// 0x141f37d3e  48894e08                       movq	%rcx, 0x8(%rsi)
// 0x141f37d42  48c7461001000000               movq	$0x1, 0x10(%rsi)
// Frames: F332
// 0x141f37d4a  4889f0                         movq	%rsi, %rax
// 0x141f37d4d  4881c470010000                 addq	$0x170, %rsp            # imm = 0x170
// 0x141f37d54  5b                             popq	%rbx
// 0x141f37d55  5f                             popq	%rdi
// 0x141f37d56  5e                             popq	%rsi
// 0x141f37d57  415e                           popq	%r14
// 0x141f37d59  5d                             popq	%rbp
// 0x141f37d5a  c3                             retq
// Frames: F295 -> F305 -> F306
// 0x141f37d5b  b908000000                     movl	$0x8, %ecx
// 0x141f37d60  ba08000000                     movl	$0x8, %edx
// 0x141f37d65  e87795e700                     callq	0x142db12e1 <.text+0x2db02e1>
// 0x141f37d6a  90                             nop
// Frames: F333 -> F309 -> F310 -> F311 -> F312 -> F313 -> F314 -> F315 -> F316 -> F231 -> F317
// 0x141f37d6b  b901000000                     movl	$0x1, %ecx
// 0x141f37d70  ba1a000000                     movl	$0x1a, %edx
// 0x141f37d75  e88095e700                     callq	0x142db12fa <.text+0x2db02fa>
// 0x141f37d7a  90                             nop
// 0x141f37d7b  0f0b                           ud2
// 0x141f37d7d  0f1f00                         nopl	(%rax)
// 0x141f37d80  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f37d85  55                             pushq	%rbp
// 0x141f37d86  4156                           pushq	%r14
// 0x141f37d88  56                             pushq	%rsi
// 0x141f37d89  57                             pushq	%rdi
// 0x141f37d8a  53                             pushq	%rbx
// 0x141f37d8b  4883ec30                       subq	$0x30, %rsp
// 0x141f37d8f  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F205 -> F206 -> F207 -> F208 -> F334 -> F73 -> F306
// 0x141f37d96  ba08000000                     movl	$0x8, %edx
// 0x141f37d9b  41b808000000                   movl	$0x8, %r8d
// 0x141f37da1  488b8de0000000                 movq	0xe0(%rbp), %rcx
// 0x141f37da8  e843db76fe                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x141f37dad  90                             nop
// Frames: F304
// 0x141f37dae  4883c430                       addq	$0x30, %rsp
// 0x141f37db2  5b                             popq	%rbx
// 0x141f37db3  5f                             popq	%rdi
// 0x141f37db4  5e                             popq	%rsi
// 0x141f37db5  415e                           popq	%r14
// 0x141f37db7  5d                             popq	%rbp
// 0x141f37db8  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:41:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F2: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:54:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F3: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:72:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F5: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:382:0 core::str::impl$0::is_char_boundary
// F6: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:439:0 core::str::traits::impl$10::get
// F7: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:470:0 core::str::traits::impl$10::index
// F8: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:0 core::str::traits::impl$4::index
// F9: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F11: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:508:0 core::str::traits::impl$11::get
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:541:0 core::str::traits::impl$11::index
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:211:0 core::str::traits::impl$7::get_unchecked
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:530:0 core::str::traits::impl$11::get_unchecked
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:511:0 core::str::traits::impl$11::get
// F16: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:863:0 core::ptr::const_ptr::impl$0::add
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:212:0 core::str::traits::impl$7::get_unchecked
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F19: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/iter.rs:128:0 core::str::iter::impl$2::next_back
// F20: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/iter.rs:216:0 core::str::iter::impl$6::next_back
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:714:0 core::str::pattern::impl$11::next_back
// F22: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:317:0 core::str::pattern::ReverseSearcher::next_match_back
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:799:0 core::str::pattern::impl$22::next_match_back
// F24: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:1531:0 core::str::impl$0::rfind
// F25: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:332:0 core::slice::iter::impl$171::any
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/cmp.rs:417:0 core::slice::cmp::impl$49::slice_contains
// F28: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:2593:0 core::slice::impl$0::contains
// F29: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:660:0 core::str::pattern::impl$8::matches
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:717:0 core::str::pattern::impl$11::next_back
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:93:0 core::str::validations::next_code_point_reverse
// F32: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:24:0 core::str::validations::utf8_is_cont_byte
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:95:0 core::str::validations::next_code_point_reverse
// F34: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:98:0 core::str::validations::next_code_point_reverse
// F35: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:100:0 core::str::validations::next_code_point_reverse
// F36: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:583:0 core::ptr::non_null::NonNull::offset
// F37: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:739:0 core::ptr::non_null::NonNull::sub
// F38: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:132:0 core::slice::iter::Iter::pre_dec_end
// F39: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:84:0 core::slice::iter::Iter::next_back_unchecked
// F40: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:447:0 core::slice::iter::impl$172::next_back
// F41: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:103:0 core::str::validations::next_code_point_reverse
// F42: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:11:0 core::str::validations::utf8_first_byte
// F43: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F44: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:17:0 core::str::validations::utf8_acc_cont_byte
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:107:0 core::str::validations::next_code_point_reverse
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:109:0 core::str::validations::next_code_point_reverse
// F48: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$172::next_back
// F50: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:83:0 core::str::validations::next_code_point_reverse
// F51: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$172::next_back
// F52: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:74:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F54: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/mod.rs:97:0 puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
// F55: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:512:0 std::thread::local::LocalKey::try_with
// F57: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:477:0 std::thread::local::LocalKey::with
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F60: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/lazy.rs:59:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F61: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2066:0 core::cell::BorrowRefMut::new
// F62: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1256:0 core::cell::RefCell::try_borrow_mut
// F63: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1222:0 core::cell::RefCell::borrow_mut
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F65: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:513:0 std::thread::local::LocalKey::try_with
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:902:0 core::mem::replace
// F67: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:513:0 core::cell::Cell::replace
// F68: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2068:0 core::cell::BorrowRefMut::new
// F69: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2447:0 core::cell::UnsafeCell::get
// F70: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1264:0 core::cell::RefCell::try_borrow_mut
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F72: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2055:0 core::cell::impl$52::drop
// F73: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:805:0 core::ptr::drop_in_place
// F74: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:75:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:528:0 bevy_ecs::system::query::Query::new
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:643:0 bevy_ecs::system::query::Query::copy_unsafe
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:625:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:548:0 bevy_ecs::system::query::Query::as_readonly
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1389:0 bevy_ecs::system::query::Query::get
// F80: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:79:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:957:0 bevy_ecs::entity::Entities::get_id_location
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:949:0 bevy_ecs::entity::Entities::get
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1546:0 bevy_ecs::system::query::Query::get_inner
// F84: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F85: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F86: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1583:0 core::option::Option::filter
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::Entities::get_id_location
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:214:0 bevy_ecs::entity::impl$37::eq
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::impl$18::get_id_location::closure$0
// F90: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1584:0 core::option::Option::filter
// F91: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1589:0 core::option::Option::filter
// F92: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1338:0 core::option::Option::ok_or
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1547:0 bevy_ecs::system::query::Query::get_inner
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:123:0 bevy_ecs::archetype::ArchetypeId::index
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1551:0 bevy_ecs::system::query::Query::get_inner
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 bevy_ecs::archetype::ArchetypeId::index
// F97: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 bevy_ecs::archetype::ArchetypeId::index
// F99: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F102: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F103: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/internal_macros.rs:28:0 fixedbitset::FixedBitSet::get_unchecked
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1548:0 bevy_ecs::system::query::Query::get_inner
// F105: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F106: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F107: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F108: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F109: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F110: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1580:0 bevy_ecs::system::query::Query::get_inner
// F113: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:221:0 core::slice::index::impl$2::get
// F114: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 alloc::vec::impl$9::deref
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1582:0 bevy_ecs::system::query::Query::get_inner
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:381:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1552:0 bevy_ecs::query::fetch::impl$36::set_table
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1539:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F122: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F123: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::slice::impl$0::get
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F126: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F127: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F128: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F130: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:298:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1609:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1604:0 bevy_ecs::query::fetch::impl$37::fetch
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1591:0 bevy_ecs::system::query::Query::get_inner
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ecs::storage::table::TableRow::index
// F140: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:80:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F141: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:101:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F142: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:103:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F143: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1591:0 glam::f32::vec3::impl$34::add
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1593:0 glam::f32::vec3::impl$34::add
// F146: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:517:0 core::ops::arith::impl$241::div
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1363:0 glam::f32::vec3::impl$8::div
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1365:0 glam::f32::vec3::impl$8::div
// F149: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:106:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1590:0 glam::f32::vec3::impl$34::add
// F151: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:109:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F152: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:996:0 glam::f32::vec2::Vec2::perp
// F153: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:186:0 glam::f32::vec2::Vec2::dot
// F154: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:462:0 glam::f32::vec2::Vec2::length
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:481:0 glam::f32::vec2::Vec2::length_recip
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:533:0 glam::f32::vec2::Vec2::normalize
// F157: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:2041:0 core::f32::math::sqrt
// F158: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:385:0 std::f32::impl$0::sqrt
// F159: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs:193:0 glam::f32::vec2::Vec2::dot
// F160: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:908:0 glam::f32::vec2::Vec2::length
// F161: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:352:0 core::ops::arith::impl$141::mul
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1311:0 glam::f32::vec2::Vec2::length_recip
// F163: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:1312:0 glam::f32::vec2::Vec2::length_recip
// F164: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:116:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F165: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:115:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F166: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:111:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F167: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:122:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F168: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:125:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F169: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:126:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F170: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:269:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F171: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop
// F172: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop::closure$0
// F173: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:128:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F174: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:131:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F175: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:132:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F176: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/fmt.rs:659:0 alloc::fmt::format::closure$0
// F177: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1278:0 core::option::Option::map_or_else
// F178: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/fmt.rs:659:0 alloc::fmt::format
// F179: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/hint.rs:614:0 core::hint::must_use
// F180: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:135:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F181: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:134:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F182: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:137:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F183: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:133:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F184: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:139:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F185: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:140:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F186: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:144:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F187: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:150:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F188: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:153:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F189: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:157:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F190: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:93:0 alloc::alloc::alloc
// F191: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:190:0 alloc::alloc::Global::alloc_impl_runtime
// F192: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:312:0 alloc::alloc::Global::alloc_impl
// F193: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:429:0 alloc::alloc::impl$1::allocate
// F194: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:248:0 alloc::boxed::box_new_uninit
// F195: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:286:0 alloc::boxed::impl$0::new
// F196: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:422:0 alloc::sync::Arc::new
// F197: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:95:0 alloc::alloc::alloc
// F198: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:289:0 alloc::boxed::impl$0::new
// F199: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:152:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F200: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:151:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F201: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:632:0 alloc::raw_vec::RawVecInner::current_memory
// F202: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:872:0 alloc::raw_vec::RawVecInner::deallocate
// F203: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:423:0 alloc::raw_vec::impl$3::drop
// F204: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:171:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F205: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:115:0 alloc::alloc::dealloc
// F206: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:209:0 alloc::alloc::Global::deallocate_impl_runtime
// F207: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:324:0 alloc::alloc::Global::deallocate_impl
// F208: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:442:0 alloc::alloc::impl$1::deallocate
// F209: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:874:0 alloc::raw_vec::RawVecInner::deallocate
// F210: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:174:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F211: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:176:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F212: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:89:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F213: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:181:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F214: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:90:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F215: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:93:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// F216: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/range.rs:391:0 core::ops::range::RangeInclusive::new
// F217: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:579:0 glam::f32::vec3::Vec3::distance
// F218: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:218:0 core::ops::arith::impl$77::sub
// F219: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1733:0 glam::f32::vec3::impl$50::sub
// F220: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1731:0 glam::f32::vec3::impl$50::sub
// F221: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:1732:0 glam::f32::vec3::impl$50::sub
// F222: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:249:0 glam::f32::vec3::Vec3::dot
// F223: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs:553:0 glam::f32::vec3::impl$50::sub
// F224: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs:193:0 glam::f32::vec3::Vec3::dot
// F225: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:184:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F226: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F227: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F228: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:186:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F229: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:437:0 bevy_ecs::change_detection::impl$38::deref_mut
// F230: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:187:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F231: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:778:0 core::convert::impl$3::into
// F232: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:188:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F233: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:193:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F234: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:195:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F235: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:0 core::option::Option::unwrap
// F236: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:237:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F237: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:238:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F238: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:236:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F239: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:199:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F240: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:202:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F241: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1512:0 core::f32::impl$0::clamp
// F242: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:203:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F243: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1515:0 core::f32::impl$0::clamp
// F244: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:208:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F245: crates/systems/roof/src/ui_edit_roof.rs:247:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F246: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:207:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F247: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:211:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F248: crates/systems/roof/src/ui_edit_roof.rs:249:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F249: crates/systems/roof/src/ui_edit_roof.rs:251:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F250: crates/systems/roof/src/ui_edit_roof.rs:253:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F251: crates/systems/roof/src/ui_edit_roof.rs:260:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F252: crates/systems/roof/src/ui_edit_roof.rs:261:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F253: crates/systems/roof/src/ui_edit_roof.rs:259:0 system_roof::ui_edit_roof::update_wall_height_if_needed
// F254: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:223:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F255: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:70:0 bevy_ecs::message::message_writer::MessageWriter::write
// F256: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:222:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F257: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1025:0 alloc::vec::Vec::push_mut
// F258: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:992:0 alloc::vec::Vec::push
// F259: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:143:0 bevy_ecs::message::messages::Messages::write_with_caller
// F260: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::change_detection::impl$38::deref_mut
// F261: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:131:0 bevy_ecs::message::messages::Messages::write_with_caller
// F262: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1028:0 alloc::vec::Vec::push_mut
// F263: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1029:0 alloc::vec::Vec::push_mut
// F264: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2000:0 alloc::vec::Vec::as_mut_ptr
// F265: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1032:0 alloc::vec::Vec::push_mut
// F266: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 alloc::vec::Vec::as_mut_ptr
// F267: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1917:0 core::ptr::write
// F268: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:0 alloc::vec::Vec::push_mut
// F269: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1034:0 alloc::vec::Vec::push_mut
// F270: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:144:0 bevy_ecs::message::messages::Messages::write_with_caller
// F271: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:234:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F272: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:245:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F273: crates/country-core/src/resources/roofs/roof_shape.rs:338:0 country_core::resources::roofs::roof_shape::impl$49::eq
// F274: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:265:0 core::cmp::PartialEq::ne
// F275: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:246:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F276: crates/country-core/src/resources/roofs/roof_shape.rs:340:0 country_core::resources::roofs::roof_shape::impl$49::eq
// F277: crates/country-core/src/resources/roofs/roof_shape.rs:350:0 country_core::resources::roofs::roof_shape::impl$49::eq
// F278: crates/country-core/src/resources/roofs/roof_shape.rs:355:0 country_core::resources::roofs::roof_shape::impl$49::eq
// F279: crates/country-core/src/resources/roofs/roof_shape.rs:345:0 country_core::resources::roofs::roof_shape::impl$49::eq
// F280: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs:30:0 glam::f32::vec2::impl$103::eq
// F281: crates/country-core/src/resources/roofs/roof_shape.rs:352:0 country_core::resources::roofs::roof_shape::impl$49::eq
// F282: crates/country-core/src/resources/roofs/roof_shape.rs:360:0 country_core::resources::roofs::roof_shape::impl$55::eq
// F283: crates/country-core/src/resources/roofs/roof_shape.rs:357:0 country_core::resources::roofs::roof_shape::impl$49::eq
// F284: crates/country-core/src/resources/roofs/roof_shape.rs:147:0 country_core::resources::roofs::roof_shape::impl$40::eq
// F285: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:247:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F286: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:248:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F287: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:250:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F288: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:254:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F289: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:253:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F290: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:261:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F291: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:262:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F292: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:263:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F293: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/lazy.rs:62:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F294: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:269:0 core::ptr::const_ptr::impl$0::as_ref
// F295: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:250:0 alloc::boxed::box_new_uninit
// F296: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1016:0 core::option::Option::unwrap
// F297: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1233:0 core::result::Result::unwrap
// F298: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:543:0 core::str::traits::impl$11::index
// F299: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:472:0 core::str::traits::impl$10::index
// F300: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:141:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F301: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:218:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::h81498e9d2e2534e7
// F302: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:1232:0 core::num::impl$11::unchecked_mul
// F303: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:640:0 alloc::raw_vec::RawVecInner::current_memory
// F304: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:157:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F305: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:317:0 alloc::boxed::impl$0::new_uninit
// F306: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:158:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F307: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:464:0 alloc::raw_vec::RawVecInner::try_allocate_in
// F308: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:433:0 alloc::raw_vec::RawVecInner::with_capacity_in
// F309: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:0 alloc::raw_vec::RawVec::with_capacity_in
// F310: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:965:0 alloc::vec::Vec::with_capacity_in
// F311: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:448:0 alloc::slice::impl$0::to_vec_in::impl$1::to_vec
// F312: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:400:0 alloc::slice::impl$0::to_vec_in
// F313: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:376:0 alloc::slice::impl$0::to_vec
// F314: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:842:0 alloc::slice::impl$9::to_owned
// F315: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/str.rs:211:0 alloc::str::impl$4::to_owned
// F316: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:3114:0 alloc::string::impl$48::from
// F317: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:160:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F318: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:468:0 alloc::raw_vec::RawVecInner::try_allocate_in
// F319: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:547:0 core::ptr::copy_nonoverlapping
// F320: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1252:0 core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
// F321: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:454:0 alloc::slice::impl$0::to_vec_in::impl$1::to_vec
// F322: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:159:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F323: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:162:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F324: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:163:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F325: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:164:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F326: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:165:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F327: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:166:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F328: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:167:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F329: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1187:0 alloc::vec::Vec::from_raw_parts_in
// F330: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:484:0 alloc::slice::impl$0::into_vec
// F331: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:265:0 alloc::boxed::box_assume_init_into_vec_unsafe
// F332: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs:169:0 system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// F333: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:441:0 alloc::raw_vec::RawVecInner::with_capacity_in
// F334: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:1921:0 alloc::boxed::impl$10::drop
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f41985d8cc2ec34d84630e31a4a73f315943e1643e5d9f7fba9c051255f3ee1f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_edit_roof_height_focused.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2437
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2437,"record_offset":268,"rva":32728992,"va":5401438112}
// 编译实例 RVAs: 0x1f367a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-25761437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e863e976fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5401440120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-25761599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8c1e876fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5401440298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32717918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8a2c30cfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5401440105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25230856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8f8017ffe","size":5,"soft_fail":false,"target_va":5376209040,"va":5401439891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23319342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8d22c9cfe","size":5,"soft_fail":false,"target_va":5378120480,"va":5401439817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9c50
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23319062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8ea2d9cfe","size":5,"soft_fail":false,"target_va":5378120784,"va":5401439841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22977732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e83c63a1fe","size":5,"soft_fail":false,"target_va":5378462960,"va":5401440687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b46cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20908360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8b8f6c0fe","size":5,"soft_fail":false,"target_va":5380533456,"va":5401441811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18167223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e849caeafe","size":5,"soft_fail":false,"target_va":5383273968,"va":5401441186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142820a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9346158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e86e9c8e00","size":5,"soft_fail":false,"target_va":5410785920,"va":5401439757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428295c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9380808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8c8238f00","size":5,"soft_fail":false,"target_va":5410821568,"va":5401440755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14910115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8a382e300","size":5,"soft_fail":false,"target_va":5416351504,"va":5401441384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15178749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8fd9be700","size":5,"soft_fail":false,"target_va":5416620769,"va":5401442015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16644718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8920502ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5401439577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15179456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8c09ee700","size":5,"soft_fail":false,"target_va":5416621408,"va":5401441947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15179527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8079fe700","size":5,"soft_fail":false,"target_va":5416621600,"va":5401442068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15179460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8c49ee700","size":5,"soft_fail":false,"target_va":5416621632,"va":5401442167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e89e150000","size":5,"soft_fail":false,"target_va":5401443792,"va":5401438253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15179166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e89e9de700","size":5,"soft_fail":false,"target_va":5416621376,"va":5401442205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::pressed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23034611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e80d85a0fe","size":5,"soft_fail":false,"target_va":5378404752,"va":5401439358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20889527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e84940c1fe","size":5,"soft_fail":false,"target_va":5380552224,"va":5401441746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21462096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8b083b8fe","size":5,"soft_fail":false,"target_va":5379976880,"va":5401438971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21464654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8b279b8fe","size":5,"soft_fail":false,"target_va":5379974352,"va":5401439001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21929746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8ee60b1fe","size":5,"soft_fail":false,"target_va":5379511040,"va":5401440781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20131628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8d4d0ccfe","size":5,"soft_fail":false,"target_va":5381309184,"va":5401440807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20133487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e891c9ccfe","size":5,"soft_fail":false,"target_va":5381307856,"va":5401441338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20536314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e806a4c6fe","size":5,"soft_fail":false,"target_va":5380902448,"va":5401438757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23318980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e83c2e9cfe","size":5,"soft_fail":false,"target_va":5378120816,"va":5401439791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::boop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21414187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8d53eb9fe","size":5,"soft_fail":false,"target_va":5380025296,"va":5401439478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21414040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8683fb9fe","size":5,"soft_fail":false,"target_va":5380025344,"va":5401439379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21414390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e80a3eb9fe","size":5,"soft_fail":false,"target_va":5380025136,"va":5401439521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21414893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8133cb9fe","size":5,"soft_fail":false,"target_va":5380024784,"va":5401439672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21415156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e80c3bb9fe","size":5,"soft_fail":false,"target_va":5380024576,"va":5401439727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21414809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8673cb9fe","size":5,"soft_fail":false,"target_va":5380024896,"va":5401439700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22504478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8e29ba8fe","size":5,"soft_fail":false,"target_va":5378935744,"va":5401440217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21228224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e84015bcfe","size":5,"soft_fail":false,"target_va":5380212048,"va":5401440267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::owned_by
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21227890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e88e16bcfe","size":5,"soft_fail":false,"target_va":5380212032,"va":5401439917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20928929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e85fa6c0fe","size":5,"soft_fail":false,"target_va":5380510400,"va":5401439324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20239061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e82b2dcbfe","size":5,"soft_fail":false,"target_va":5381201344,"va":5401440400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20239046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e83a2dcbfe","size":5,"soft_fail":false,"target_va":5381201392,"va":5401440433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20239086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8122dcbfe","size":5,"soft_fail":false,"target_va":5381201472,"va":5401440553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17474861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8d35af5fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5401438648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17466354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e80e7cf5fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5401438573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17474546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e80e5cf5fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5401439453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17475417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8a758f5fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5401438228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::camera::CameraMatrices::eye_right
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-18026353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e88ff0ecfe","size":5,"soft_fail":false,"target_va":5383412768,"va":5401439116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15577990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e886b3ed00","size":5,"soft_fail":false,"target_va":5417019952,"va":5401441957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9343366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e886918e00","size":5,"soft_fail":false,"target_va":5410786608,"va":5401443237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15177356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e88c96e700","size":5,"soft_fail":false,"target_va":5416619360,"va":5401441999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18163490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401438112,"raw_hex":"e8ded8eafe","size":5,"soft_fail":false,"target_va":5383275712,"va":5401439197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2437,"record_offset":5668,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2437,"record_offset":5892,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2437,"record_offset":6712,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2437,"record_offset":7428,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2437,"record_offset":2740,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2437,"record_offset":7824,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2437,"record_offset":6772,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2437,"record_offset":7256,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2437,"record_offset":5752,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2437,"record_offset":7796,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2437,"record_offset":6856,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2437,"record_offset":4924,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2437,"record_offset":9428,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2437,"record_offset":4588,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2437,"record_offset":4560,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2437,"record_offset":6464,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2437,"record_offset":4296,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2437,"record_offset":4384,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2437,"record_offset":1932,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2437,"record_offset":532,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2437,"record_offset":920,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] system_roof::ui_edit_roof::update_wall_height_if_needed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/ui_edit_roof.rs
//     evidence={"module":2437,"record_offset":7344,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":105093,"source_line":241,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs
//     evidence={"module":2437,"record_offset":6400,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":105101,"source_line":88,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":268,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":268,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":6292,"parent_record_offset":6268,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":5988,"parent_record_offset":5956,"ancestor_record_offset":268,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":6268,"parent_record_offset":6212,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2437,"record_offset":5956,"parent_record_offset":5920,"ancestor_record_offset":268,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2437,"record_offset":5920,"parent_record_offset":5892,"ancestor_record_offset":268,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2437,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":268,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":6240,"parent_record_offset":6212,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2437,"record_offset":7628,"parent_record_offset":7584,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2437,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2437,"record_offset":3404,"parent_record_offset":3372,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2437,"record_offset":7552,"parent_record_offset":7516,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2437,"record_offset":7584,"parent_record_offset":7552,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2437,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2437,"record_offset":3160,"parent_record_offset":2876,"ancestor_record_offset":268,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2437,"record_offset":6740,"parent_record_offset":6712,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2437,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2437,"record_offset":2924,"parent_record_offset":2876,"ancestor_record_offset":268,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2437,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":268,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2437,"record_offset":3064,"parent_record_offset":3032,"ancestor_record_offset":268,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2437,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":268,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2437,"record_offset":7516,"parent_record_offset":7452,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2437,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2437,"record_offset":3580,"parent_record_offset":2876,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2437,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2437,"record_offset":4112,"parent_record_offset":2876,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2437,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2437,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2437,"record_offset":3704,"parent_record_offset":3680,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2437,"record_offset":3680,"parent_record_offset":3656,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2437,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2437,"record_offset":3656,"parent_record_offset":3628,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2437,"record_offset":3628,"parent_record_offset":3604,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2437,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2437,"record_offset":3348,"parent_record_offset":2876,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2437,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2437,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2437,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":268,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2437,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":268,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2437,"record_offset":2876,"parent_record_offset":2740,"ancestor_record_offset":268,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2437,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":268,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2437,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":268,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":2437,"record_offset":3196,"parent_record_offset":3160,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2437,"record_offset":2260,"parent_record_offset":2232,"ancestor_record_offset":268,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2437,"record_offset":2288,"parent_record_offset":2260,"ancestor_record_offset":268,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2437,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":268,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2437,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":268,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2437,"record_offset":2348,"parent_record_offset":2232,"ancestor_record_offset":268,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2437,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":268,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2437,"record_offset":7392,"parent_record_offset":7344,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2437,"record_offset":4732,"parent_record_offset":4708,"ancestor_record_offset":268,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2437,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":268,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2437,"record_offset":10156,"parent_record_offset":10124,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2437,"record_offset":4324,"parent_record_offset":4296,"ancestor_record_offset":268,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2437,"record_offset":4772,"parent_record_offset":4612,"ancestor_record_offset":268,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2437,"record_offset":4412,"parent_record_offset":4384,"ancestor_record_offset":268,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2437,"record_offset":6524,"parent_record_offset":6488,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2437,"record_offset":2080,"parent_record_offset":2040,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2437,"record_offset":6432,"parent_record_offset":6400,"ancestor_record_offset":268,"inlinee":8913,"source_line":390,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2437,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2437,"record_offset":3032,"parent_record_offset":2948,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2437,"record_offset":3860,"parent_record_offset":3680,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2437,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":268,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2437,"record_offset":3132,"parent_record_offset":2876,"ancestor_record_offset":268,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2437,"record_offset":868,"parent_record_offset":840,"ancestor_record_offset":268,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2437,"record_offset":2696,"parent_record_offset":2040,"ancestor_record_offset":268,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2437,"record_offset":2412,"parent_record_offset":2180,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2437,"record_offset":3300,"parent_record_offset":3268,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2437,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":268,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2437,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":268,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2437,"record_offset":1524,"parent_record_offset":1496,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2437,"record_offset":7748,"parent_record_offset":7584,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2437,"record_offset":1624,"parent_record_offset":1596,"ancestor_record_offset":268,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2437,"record_offset":1596,"parent_record_offset":1568,"ancestor_record_offset":268,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2437,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2437,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2437,"record_offset":3940,"parent_record_offset":3908,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2437,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":268,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2437,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":268,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2437,"record_offset":1652,"parent_record_offset":1624,"ancestor_record_offset":268,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2437,"record_offset":1244,"parent_record_offset":1128,"ancestor_record_offset":268,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2437,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":268,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2437,"record_offset":956,"parent_record_offset":920,"ancestor_record_offset":268,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2437,"record_offset":1096,"parent_record_offset":1064,"ancestor_record_offset":268,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2437,"record_offset":1064,"parent_record_offset":1028,"ancestor_record_offset":268,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2437,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":268,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2437,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":268,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2437,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":268,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2437,"record_offset":1568,"parent_record_offset":1028,"ancestor_record_offset":268,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2437,"record_offset":636,"parent_record_offset":604,"ancestor_record_offset":268,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2437,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":268,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2437,"record_offset":760,"parent_record_offset":728,"ancestor_record_offset":268,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2437,"record_offset":816,"parent_record_offset":760,"ancestor_record_offset":268,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2437,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":268,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2437,"record_offset":576,"parent_record_offset":532,"ancestor_record_offset":268,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2437,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":268,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2437,"record_offset":1128,"parent_record_offset":1096,"ancestor_record_offset":268,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2437,"record_offset":1412,"parent_record_offset":1128,"ancestor_record_offset":268,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2437,"record_offset":1384,"parent_record_offset":1128,"ancestor_record_offset":268,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2437,"record_offset":1188,"parent_record_offset":1128,"ancestor_record_offset":268,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":2437,"record_offset":7952,"parent_record_offset":7848,"ancestor_record_offset":268,"inlinee":27145,"source_line":147,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$49::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":2437,"record_offset":7848,"parent_record_offset":7824,"ancestor_record_offset":268,"inlinee":27141,"source_line":334,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$55::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":2437,"record_offset":7924,"parent_record_offset":7848,"ancestor_record_offset":268,"inlinee":27143,"source_line":360,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2437,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":268,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2437,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":268,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2437,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":268,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2437,"record_offset":4676,"parent_record_offset":4644,"ancestor_record_offset":268,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2437,"record_offset":4644,"parent_record_offset":4612,"ancestor_record_offset":268,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2437,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":268,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2437,"record_offset":7896,"parent_record_offset":7848,"ancestor_record_offset":268,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2437,"record_offset":6612,"parent_record_offset":6488,"ancestor_record_offset":268,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2437,"record_offset":6488,"parent_record_offset":6464,"ancestor_record_offset":268,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2437,"record_offset":4216,"parent_record_offset":4184,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2437,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2437,"record_offset":1968,"parent_record_offset":1932,"ancestor_record_offset":268,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2437,"record_offset":4988,"parent_record_offset":4956,"ancestor_record_offset":268,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2437,"record_offset":5444,"parent_record_offset":5240,"ancestor_record_offset":268,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2437,"record_offset":2108,"parent_record_offset":2080,"ancestor_record_offset":268,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2437,"record_offset":2180,"parent_record_offset":2040,"ancestor_record_offset":268,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2437,"record_offset":2384,"parent_record_offset":2180,"ancestor_record_offset":268,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2437,"record_offset":4708,"parent_record_offset":4676,"ancestor_record_offset":268,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2437,"record_offset":2136,"parent_record_offset":2108,"ancestor_record_offset":268,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2437,"record_offset":2040,"parent_record_offset":2004,"ancestor_record_offset":268,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2437,"record_offset":2004,"parent_record_offset":1968,"ancestor_record_offset":268,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19A8D
// 首条位置: {"module":2437,"inlinee":105101}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2437,"record_offset":6432,"parent_record_offset":6400,"ancestor_record_offset":6400,"inlinee":8913,"source_line":390,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2437,"record_offset":10508,"rva":32734176,"va":5401443296}
// 编译实例 RVAs: 0x1f37be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-25764672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e8c0dc76fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5401443355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25765053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e843db76fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5401443752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: ui_edit_roof_height_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32721153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e8ffb60cfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5401443340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15177079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e87795e700","size":5,"soft_fail":false,"target_va":5416620769,"va":5401443685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15177088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e88095e700","size":5,"soft_fail":false,"target_va":5416620794,"va":5401443701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22507218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e82e91a8fe","size":5,"soft_fail":false,"target_va":5378936272,"va":5401443485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22504888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e8489aa8fe","size":5,"soft_fail":false,"target_va":5378938736,"va":5401443619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22507121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e88f91a8fe","size":5,"soft_fail":false,"target_va":5378936352,"va":5401443468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22496840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e8b8b9a8fe","size":5,"soft_fail":false,"target_va":5378946752,"va":5401443587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22496722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e82ebaa8fe","size":5,"soft_fail":false,"target_va":5378946800,"va":5401443517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22496894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401443296,"raw_hex":"e882b9a8fe","size":5,"soft_fail":false,"target_va":5378946656,"va":5401443545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2437,"record_offset":11420,"parent_record_offset":10508,"ancestor_record_offset":10508,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2437,"record_offset":10708,"parent_record_offset":10508,"ancestor_record_offset":10508,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2437,"record_offset":10888,"parent_record_offset":10508,"ancestor_record_offset":10508,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2437,"record_offset":11504,"parent_record_offset":10508,"ancestor_record_offset":10508,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":10508,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":10508,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":11576,"parent_record_offset":11552,"ancestor_record_offset":10508,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":11600,"parent_record_offset":11576,"ancestor_record_offset":10508,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":10508,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":10508,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":10768,"parent_record_offset":10736,"ancestor_record_offset":10508,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2437,"record_offset":11552,"parent_record_offset":11528,"ancestor_record_offset":10508,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2437,"record_offset":10736,"parent_record_offset":10708,"ancestor_record_offset":10508,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2437,"record_offset":11528,"parent_record_offset":11504,"ancestor_record_offset":10508,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":11120,"parent_record_offset":11092,"ancestor_record_offset":10508,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":11180,"parent_record_offset":11148,"ancestor_record_offset":10508,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2437,"record_offset":11148,"parent_record_offset":11120,"ancestor_record_offset":10508,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2437,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":10508,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2437,"record_offset":11000,"parent_record_offset":10972,"ancestor_record_offset":10508,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2437,"record_offset":11028,"parent_record_offset":11000,"ancestor_record_offset":10508,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2437,"record_offset":11056,"parent_record_offset":11028,"ancestor_record_offset":10508,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2437,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":10508,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2437,"record_offset":10944,"parent_record_offset":10916,"ancestor_record_offset":10508,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2437,"record_offset":10916,"parent_record_offset":10888,"ancestor_record_offset":10508,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2437,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10508,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2437,"record_offset":11092,"parent_record_offset":11056,"ancestor_record_offset":10508,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2437,"record_offset":11336,"parent_record_offset":11056,"ancestor_record_offset":10508,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2437,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":10508,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02437.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 28
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)", "tuple$<>", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$134::init_access<bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight> >", "constructor": "bevy_ecs::system::system_param::impl$134::init_access", "arguments": ["bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused
// 信息 ID: 995be9b38b43cacfb492cda0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 497b97607894768c7492cd7f: void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> @ 0xfcfee0 -> 0x1f367a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>),tuple$<>,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::OriginalWallHeight>)> > @ 0x1f28bb4 -> 0x1f367a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$0
// 信息 ID: fd77a9c7fd05c76a499d7f09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused; module=2437, inline_record=6400; 不是独立调用边。
// [PDB 内联宿主] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused; module=2437, inline_record=6952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure$1
// 信息 ID: 5e34b43dd77477f5da40f4ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused", "captures": "unknown"}
// [闭包环境类型引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1 <- core::ops::function::FnOnce::call_once<system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1f37dc0 -> 0x1f37be0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
