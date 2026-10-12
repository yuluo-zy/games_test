// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: assemble_roof_tiles.rs.assembly.json.gz

// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 2275:10232 -> host 2275:2152 / rva_1a36700_len_b95; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1a36ed3, 0x1a36ee1), 3 instructions
//
// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// Candidate interfaces: 8b9dad12a9b582e5eb314b57
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 2275:2152 -> rva_1a36700_len_b95
//
// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 7; complete records in enriched/assembly sidecars.
// Compiled procedure 3474:31176 -> rva_21b2ca0_len_3c3
// Compiled procedure 3475:10392 -> rva_21b4510_len_3c3
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1a36700_len_b95: RVA 0x1a36700, 2965 PDB bytes; 739 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_1a36700_len_b95.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2275:2152: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// Frames: F1
// 0x141a36700  55                             pushq	%rbp
// 0x141a36701  4157                           pushq	%r15
// 0x141a36703  4156                           pushq	%r14
// 0x141a36705  4155                           pushq	%r13
// 0x141a36707  4154                           pushq	%r12
// 0x141a36709  56                             pushq	%rsi
// 0x141a3670a  57                             pushq	%rdi
// 0x141a3670b  53                             pushq	%rbx
// 0x141a3670c  4881ec88010000                 subq	$0x188, %rsp            # imm = 0x188
// 0x141a36713  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x141a3671b  660f7fb5f0000000               movdqa	%xmm6, 0xf0(%rbp)
// 0x141a36723  48c785e8000000feffffff         movq	$-0x2, 0xe8(%rbp)
// 0x141a3672e  4d89cd                         movq	%r9, %r13
// 0x141a36731  4c89c6                         movq	%r8, %rsi
// 0x141a36734  4889d3                         movq	%rdx, %rbx
// 0x141a36737  4989cf                         movq	%rcx, %r15
// Frames: F2
// 0x141a3673a  e8c1021eff                     callq	0x140c16a00 <.text+0xc15a00>
// 0x141a3673f  84c0                           testb	%al, %al
// 0x141a36741  745c                           je	0x141a3679f <.text+0x1a3579f>
// Frames: F3 -> F2
// 0x141a36743  bf3d000000                     movl	$0x3d, %edi
// 0x141a36748  b93d000000                     movl	$0x3d, %ecx
// 0x141a3674d  e8befaffff                     callq	0x141a36210 <.text+0x1a35210>
// 0x141a36752  4c8d25b23e4801                 leaq	0x1483eb2(%rip), %r12   # 0x142eba60b
// 0x141a36759  4883f801                       cmpq	$0x1, %rax
// 0x141a3675d  0f85a7000000                   jne	0x141a3680a <.text+0x1a3580a>
// Frames: F4 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x141a36763  4989d1                         movq	%rdx, %r9
// 0x141a36766  4885d2                         testq	%rdx, %rdx
// 0x141a36769  743d                           je	0x141a367a8 <.text+0x1a357a8>
// Frames: F9 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x141a3676b  4983f93d                       cmpq	$0x3d, %r9
// 0x141a3676f  7335                           jae	0x141a367a6 <.text+0x1a357a6>
// Frames: F10 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x141a36771  488d05933e4801                 leaq	0x1483e93(%rip), %rax   # 0x142eba60b
// Frames: F11 -> F10 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x141a36778  41803c01bf                     cmpb	$-0x41, (%r9,%rax)
// Frames: F5 -> F6 -> F7 -> F8 -> F2
// 0x141a3677d  7f29                           jg	0x141a367a8 <.text+0x1a357a8>
// Frames: F12 -> F7 -> F8 -> F2
// 0x141a3677f  488d054a3f4801                 leaq	0x1483f4a(%rip), %rax   # 0x142eba6d0
// 0x141a36786  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a3678b  488d0d793e4801                 leaq	0x1483e79(%rip), %rcx   # 0x142eba60b
// 0x141a36792  ba3d000000                     movl	$0x3d, %edx
// 0x141a36797  4531c0                         xorl	%r8d, %r8d
// 0x141a3679a  e8a131ea00                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F2
// 0x141a3679f  31c0                           xorl	%eax, %eax
// 0x141a367a1  e9c4010000                     jmp	0x141a3696a <.text+0x1a3596a>
// Frames: F5 -> F6 -> F7 -> F8 -> F2
// 0x141a367a6  75d7                           jne	0x141a3677f <.text+0x1a3577f>
// Frames: F8 -> F2
// 0x141a367a8  4c89c9                         movq	%r9, %rcx
// 0x141a367ab  e860faffff                     callq	0x141a36210 <.text+0x1a35210>
// 0x141a367b0  4883f801                       cmpq	$0x1, %rax
// 0x141a367b4  7554                           jne	0x141a3680a <.text+0x1a3580a>
// Frames: F4 -> F13 -> F14 -> F7 -> F15 -> F2
// 0x141a367b6  4989d0                         movq	%rdx, %r8
// 0x141a367b9  4983c002                       addq	$0x2, %r8
// 0x141a367bd  7439                           je	0x141a367f8 <.text+0x1a357f8>
// Frames: F9 -> F13 -> F14 -> F7 -> F15 -> F2
// 0x141a367bf  4983f83d                       cmpq	$0x3d, %r8
// 0x141a367c3  7331                           jae	0x141a367f6 <.text+0x1a357f6>
// Frames: F10 -> F13 -> F14 -> F7 -> F15 -> F2
// 0x141a367c5  488d053f3e4801                 leaq	0x1483e3f(%rip), %rax   # 0x142eba60b
// Frames: F11 -> F10 -> F13 -> F14 -> F7 -> F15 -> F2
// 0x141a367cc  41803c00bf                     cmpb	$-0x41, (%r8,%rax)
// Frames: F13 -> F14 -> F7 -> F15 -> F2
// 0x141a367d1  7f25                           jg	0x141a367f8 <.text+0x1a357f8>
// Frames: F16 -> F7 -> F15 -> F2
// 0x141a367d3  488d050e3f4801                 leaq	0x1483f0e(%rip), %rax   # 0x142eba6e8
// 0x141a367da  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a367df  488d0d253e4801                 leaq	0x1483e25(%rip), %rcx   # 0x142eba60b
// 0x141a367e6  ba3d000000                     movl	$0x3d, %edx
// 0x141a367eb  41b93d000000                   movl	$0x3d, %r9d
// 0x141a367f1  e84a31ea00                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F13 -> F14 -> F7 -> F15 -> F2
// 0x141a367f6  75db                           jne	0x141a367d3 <.text+0x1a357d3>
// Frames: F17 -> F18 -> F19 -> F14 -> F7 -> F15 -> F2
// 0x141a367f8  bf3b000000                     movl	$0x3b, %edi
// 0x141a367fd  4829d7                         subq	%rdx, %rdi
// Frames: F20 -> F21 -> F18 -> F19 -> F14 -> F7 -> F15 -> F2
// 0x141a36800  4c8d25043e4801                 leaq	0x1483e04(%rip), %r12   # 0x142eba60b
// 0x141a36807  4d01c4                         addq	%r8, %r12
// 0x141a3680a  4c89ade0000000                 movq	%r13, 0xe0(%rbp)
// 0x141a36811  4c8d05fd3f4801                 leaq	0x1483ffd(%rip), %r8    # 0x142eba815
// 0x141a36818  4c8d2dc13f4801                 leaq	0x1483fc1(%rip), %r13   # 0x142eba7e0
// 0x141a3681f  90                             nop
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36820  410fbe40ff                     movsbl	-0x1(%r8), %eax
// 0x141a36825  85c0                           testl	%eax, %eax
// 0x141a36827  7817                           js	0x141a36840 <.text+0x1a35840>
// Frames: F30 -> F31 -> F32 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36829  49ffc8                         decq	%r8
// 0x141a3682c  83f85c                         cmpl	$0x5c, %eax
// 0x141a3682f  7563                           jne	0x141a36894 <.text+0x1a35894>
// 0x141a36831  eb73                           jmp	0x141a368a6 <.text+0x1a358a6>
// 0x141a36833  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F33 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36840  410fb648fe                     movzbl	-0x2(%r8), %ecx
// Frames: F34 -> F35 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36845  80f9c0                         cmpb	$-0x40, %cl
// Frames: F35 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36848  7d23                           jge	0x141a3686d <.text+0x1a3586d>
// Frames: F36 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a3684a  410fb650fd                     movzbl	-0x3(%r8), %edx
// Frames: F34 -> F37 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a3684f  80fac0                         cmpb	$-0x40, %dl
// Frames: F37 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36852  7d22                           jge	0x141a36876 <.text+0x1a35876>
// Frames: F38 -> F39 -> F40 -> F41 -> F42 -> F43 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36854  450fb648fc                     movzbl	-0x4(%r8), %r9d
// 0x141a36859  4983c0fc                       addq	$-0x4, %r8
// Frames: F44 -> F45 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a3685d  4183e107                       andl	$0x7, %r9d
// Frames: F46 -> F47 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36861  41c1e106                       shll	$0x6, %r9d
// 0x141a36865  83e23f                         andl	$0x3f, %edx
// 0x141a36868  4409ca                         orl	%r9d, %edx
// 0x141a3686b  eb10                           jmp	0x141a3687d <.text+0x1a3587d>
// Frames: F35 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a3686d  4983c0fe                       addq	$-0x2, %r8
// 0x141a36871  83e11f                         andl	$0x1f, %ecx
// 0x141a36874  eb0f                           jmp	0x141a36885 <.text+0x1a35885>
// 0x141a36876  4983c0fd                       addq	$-0x3, %r8
// 0x141a3687a  83e20f                         andl	$0xf, %edx
// Frames: F46 -> F48 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a3687d  c1e206                         shll	$0x6, %edx
// 0x141a36880  83e13f                         andl	$0x3f, %ecx
// 0x141a36883  09d1                           orl	%edx, %ecx
// Frames: F46 -> F49 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36885  c1e106                         shll	$0x6, %ecx
// 0x141a36888  243f                           andb	$0x3f, %al
// 0x141a3688a  0fb6c0                         movzbl	%al, %eax
// 0x141a3688d  09c8                           orl	%ecx, %eax
// Frames: F30 -> F31 -> F32 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a3688f  83f85c                         cmpl	$0x5c, %eax
// 0x141a36892  7412                           je	0x141a368a6 <.text+0x1a358a6>
// 0x141a36894  83f82f                         cmpl	$0x2f, %eax
// 0x141a36897  740d                           je	0x141a368a6 <.text+0x1a358a6>
// Frames: F50 -> F51 -> F52 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a36899  4d39e8                         cmpq	%r13, %r8
// Frames: F53 -> F52 -> F23 -> F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F2
// 0x141a3689c  7582                           jne	0x141a36820 <.text+0x1a35820>
// 0x141a3689e  41be35000000                   movl	$0x35, %r14d
// 0x141a368a4  eb4d                           jmp	0x141a368f3 <.text+0x1a358f3>
// Frames: F54 -> F2
// 0x141a368a6  4c8d2d333f4801                 leaq	0x1483f33(%rip), %r13   # 0x142eba7e0
// 0x141a368ad  4d29e8                         subq	%r13, %r8
// Frames: F4 -> F13 -> F14 -> F7 -> F54 -> F2
// 0x141a368b0  49ffc0                         incq	%r8
// 0x141a368b3  7432                           je	0x141a368e7 <.text+0x1a358e7>
// Frames: F9 -> F13 -> F14 -> F7 -> F54 -> F2
// 0x141a368b5  4983f835                       cmpq	$0x35, %r8
// 0x141a368b9  732a                           jae	0x141a368e5 <.text+0x1a358e5>
// Frames: F11 -> F10 -> F13 -> F14 -> F7 -> F54 -> F2
// 0x141a368bb  43803c28bf                     cmpb	$-0x41, (%r8,%r13)
// Frames: F13 -> F14 -> F7 -> F54 -> F2
// 0x141a368c0  7f25                           jg	0x141a368e7 <.text+0x1a358e7>
// Frames: F16 -> F7 -> F54 -> F2
// 0x141a368c2  488d05e73d4801                 leaq	0x1483de7(%rip), %rax   # 0x142eba6b0
// 0x141a368c9  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a368ce  488d0d0b3f4801                 leaq	0x1483f0b(%rip), %rcx   # 0x142eba7e0
// 0x141a368d5  ba35000000                     movl	$0x35, %edx
// 0x141a368da  41b935000000                   movl	$0x35, %r9d
// 0x141a368e0  e85b30ea00                     callq	0x1428d9940 <.text+0x28d8940>
// Frames: F13 -> F14 -> F7 -> F54 -> F2
// 0x141a368e5  75db                           jne	0x141a368c2 <.text+0x1a358c2>
// Frames: F17 -> F18 -> F19 -> F14 -> F7 -> F54 -> F2
// 0x141a368e7  41be35000000                   movl	$0x35, %r14d
// 0x141a368ed  4d29c6                         subq	%r8, %r14
// Frames: F20 -> F21 -> F18 -> F19 -> F14 -> F7 -> F54 -> F2
// 0x141a368f0  4d01c5                         addq	%r8, %r13
// Frames: F55 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x141a368f3  e838181eff                     callq	0x140c18130 <.text+0xc17130>
// 0x141a368f8  4889c1                         movq	%rax, %rcx
// Frames: F61 -> F55 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x141a368fb  488b00                         movq	(%rax), %rax
// 0x141a368fe  4883f801                       cmpq	$0x1, %rax
// 0x141a36902  0f85c0060000                   jne	0x141a36fc8 <.text+0x1a35fc8>
// Frames: F62 -> F55 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x141a36908  4883c108                       addq	$0x8, %rcx
// Frames: F63 -> F64 -> F65 -> F66 -> F67 -> F58 -> F59 -> F60 -> F2
// 0x141a3690c  48833900                       cmpq	$0x0, (%rcx)
// 0x141a36910  0f851a070000                   jne	0x141a37030 <.text+0x1a36030>
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F64 -> F65 -> F66 -> F67 -> F58 -> F59 -> F60 -> F2
// 0x141a36916  48c701ffffffff                 movq	$-0x1, (%rcx)
// 0x141a3691d  48898dd8000000                 movq	%rcx, 0xd8(%rbp)
// Frames: F73 -> F74 -> F65 -> F66 -> F67 -> F58 -> F59 -> F60 -> F2
// 0x141a36924  4883c108                       addq	$0x8, %rcx
// Frames: F75 -> F66 -> F67 -> F58 -> F59 -> F60 -> F2
// 0x141a36928  4c89742420                     movq	%r14, 0x20(%rsp)
// 0x141a3692d  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x141a36936  48c744242801000000             movq	$0x1, 0x28(%rsp)
// 0x141a3693f  4c89e2                         movq	%r12, %rdx
// 0x141a36942  4989f8                         movq	%rdi, %r8
// 0x141a36945  4d89e9                         movq	%r13, %r9
// 0x141a36948  e8f3031eff                     callq	0x140c16d40 <.text+0xc15d40>
// Frames: F68 -> F69 -> F70 -> F71 -> F76 -> F77 -> F77 -> F66 -> F67 -> F58 -> F59 -> F60 -> F2
// 0x141a3694d  488b8dd8000000                 movq	0xd8(%rbp), %rcx
// 0x141a36954  48ff01                         incq	(%rcx)
// Frames: F2
// 0x141a36957  488985b0000000                 movq	%rax, 0xb0(%rbp)
// 0x141a3695e  b801000000                     movl	$0x1, %eax
// 0x141a36963  4c8bade0000000                 movq	0xe0(%rbp), %r13
// 0x141a3696a  488985a8000000                 movq	%rax, 0xa8(%rbp)
// Frames: F78 -> F79 -> F80
// 0x141a36971  4d8b0f                         movq	(%r15), %r9
// Frames: F81 -> F79 -> F80
// 0x141a36974  498b4f08                       movq	0x8(%r15), %rcx
// Frames: F82 -> F83 -> F84 -> F79 -> F80
// 0x141a36978  498b11                         movq	(%r9), %rdx
// 0x141a3697b  4531d2                         xorl	%r10d, %r10d
// Frames: F85 -> F86 -> F83 -> F84 -> F79 -> F80
// 0x141a3697e  4889d0                         movq	%rdx, %rax
// 0x141a36981  482b4118                       subq	0x18(%rcx), %rax
// 0x141a36985  490f42c2                       cmovbq	%r10, %rax
// Frames: F85 -> F87 -> F83 -> F84 -> F79 -> F80
// 0x141a36989  482b5138                       subq	0x38(%rcx), %rdx
// 0x141a3698d  490f42d2                       cmovbq	%r10, %rdx
// Frames: F88 -> F89 -> F90 -> F91 -> F92 -> F83 -> F84 -> F79 -> F80
// 0x141a36991  4989c7                         movq	%rax, %r15
// 0x141a36994  49c1e705                       shlq	$0x5, %r15
// 0x141a36998  4c037908                       addq	0x8(%rcx), %r15
// Frames: F93 -> F94 -> F92 -> F83 -> F84 -> F79 -> F80
// 0x141a3699c  4c8b5910                       movq	0x10(%rcx), %r11
// Frames: F93 -> F94 -> F95 -> F83 -> F84 -> F79 -> F80
// 0x141a369a0  4c8b7130                       movq	0x30(%rcx), %r14
// Frames: F88 -> F89 -> F90 -> F91 -> F92 -> F83 -> F84 -> F79 -> F80
// 0x141a369a4  4929c3                         subq	%rax, %r11
// 0x141a369a7  4d0f42da                       cmovbq	%r10, %r11
// 0x141a369ab  41b808000000                   movl	$0x8, %r8d
// 0x141a369b1  4d0f42f8                       cmovbq	%r8, %r15
// Frames: F88 -> F89 -> F90 -> F91 -> F95 -> F83 -> F84 -> F79 -> F80
// 0x141a369b5  4889d0                         movq	%rdx, %rax
// 0x141a369b8  48c1e005                       shlq	$0x5, %rax
// 0x141a369bc  48034128                       addq	0x28(%rcx), %rax
// 0x141a369c0  4929d6                         subq	%rdx, %r14
// 0x141a369c3  4d0f42f2                       cmovbq	%r10, %r14
// 0x141a369c7  490f42c0                       cmovbq	%r8, %rax
// 0x141a369cb  488b9570010000                 movq	0x170(%rbp), %rdx
// Frames: F96 -> F83 -> F84 -> F79 -> F80
// 0x141a369d2  488b4940                       movq	0x40(%rcx), %rcx
// 0x141a369d6  4c29f1                         subq	%r14, %rcx
// 0x141a369d9  4c29d9                         subq	%r11, %rcx
// 0x141a369dc  4c894d68                       movq	%r9, 0x68(%rbp)
// 0x141a369e0  498909                         movq	%rcx, (%r9)
// Frames: F97 -> F98 -> F99 -> F100 -> F83 -> F84 -> F79 -> F80
// 0x141a369e3  49c1e305                       shlq	$0x5, %r11
// 0x141a369e7  4d01fb                         addq	%r15, %r11
// 0x141a369ea  4c899d80000000                 movq	%r11, 0x80(%rbp)
// 0x141a369f1  49c1e605                       shlq	$0x5, %r14
// 0x141a369f5  4901c6                         addq	%rax, %r14
// 0x141a369f8  4c89b588000000                 movq	%r14, 0x88(%rbp)
// 0x141a369ff  488b0e                         movq	(%rsi), %rcx
// 0x141a36a02  48894d70                       movq	%rcx, 0x70(%rbp)
// 0x141a36a06  4c8b23                         movq	(%rbx), %r12
// 0x141a36a09  488b4b08                       movq	0x8(%rbx), %rcx
// 0x141a36a0d  48894d78                       movq	%rcx, 0x78(%rbp)
// 0x141a36a11  8b4b14                         movl	0x14(%rbx), %ecx
// 0x141a36a14  898dd4000000                   movl	%ecx, 0xd4(%rbp)
// 0x141a36a1a  498b4d00                       movq	(%r13), %rcx
// 0x141a36a1e  48898d90000000                 movq	%rcx, 0x90(%rbp)
// 0x141a36a25  4883c120                       addq	$0x20, %rcx
// 0x141a36a29  48898d98000000                 movq	%rcx, 0x98(%rbp)
// 0x141a36a30  488b0a                         movq	(%rdx), %rcx
// 0x141a36a33  48898da0000000                 movq	%rcx, 0xa0(%rbp)
// 0x141a36a3a  660f76f6                       pcmpeqd	%xmm6, %xmm6
// 0x141a36a3e  eb0e                           jmp	0x141a36a4e <.text+0x1a35a4e>
// 0x141a36a40  4c8bbdd8000000                 movq	0xd8(%rbp), %r15
// 0x141a36a47  488b85e0000000                 movq	0xe0(%rbp), %rax
// Frames: F101 -> F102 -> F103 -> F104 -> F105 -> F80
// 0x141a36a4e  4d85ff                         testq	%r15, %r15
// 0x141a36a51  741d                           je	0x141a36a70 <.text+0x1a35a70>
// 0x141a36a53  4c3bbd80000000                 cmpq	0x80(%rbp), %r15
// 0x141a36a5a  7414                           je	0x141a36a70 <.text+0x1a35a70>
// 0x141a36a5c  498d4f20                       leaq	0x20(%r15), %rcx
// 0x141a36a60  48898dd8000000                 movq	%rcx, 0xd8(%rbp)
// 0x141a36a67  488985e0000000                 movq	%rax, 0xe0(%rbp)
// 0x141a36a6e  eb2f                           jmp	0x141a36a9f <.text+0x1a35a9f>
// Frames: F101 -> F106 -> F107 -> F103 -> F104 -> F105 -> F80
// 0x141a36a70  4885c0                         testq	%rax, %rax
// 0x141a36a73  0f84c7040000                   je	0x141a36f40 <.text+0x1a35f40>
// 0x141a36a79  483b8588000000                 cmpq	0x88(%rbp), %rax
// 0x141a36a80  0f84ba040000                   je	0x141a36f40 <.text+0x1a35f40>
// Frames: F108 -> F109 -> F110 -> F111 -> F106 -> F107 -> F103 -> F104 -> F105 -> F80
// 0x141a36a86  488d4820                       leaq	0x20(%rax), %rcx
// 0x141a36a8a  48898de0000000                 movq	%rcx, 0xe0(%rbp)
// 0x141a36a91  48c785d800000000000000         movq	$0x0, 0xd8(%rbp)
// 0x141a36a9c  4989c7                         movq	%rax, %r15
// Frames: F112 -> F105 -> F80
// 0x141a36a9f  488b4568                       movq	0x68(%rbp), %rax
// 0x141a36aa3  48ff00                         incq	(%rax)
// Frames: F113 -> F114 -> F115
// 0x141a36aa6  488b4d70                       movq	0x70(%rbp), %rcx
// 0x141a36aaa  488d150f484801                 leaq	0x148480f(%rip), %rdx   # 0x142ebb2c0
// 0x141a36ab1  e88a807700                     callq	0x1421aeb40 <.text+0x21adb40>
// 0x141a36ab6  4889c3                         movq	%rax, %rbx
// 0x141a36ab9  4885c0                         testq	%rax, %rax
// 0x141a36abc  0f847a050000                   je	0x141a3703c <.text+0x1a3603c>
// Frames: F116 -> F114 -> F115
// 0x141a36ac2  488d0d5f135c01                 leaq	0x15c135f(%rip), %rcx   # 0x142ff7e28
// 0x141a36ac9  48894df0                       movq	%rcx, -0x10(%rbp)
// 0x141a36acd  48c745f835000000               movq	$0x35, -0x8(%rbp)
// 0x141a36ad5  4989dd                         movq	%rbx, %r13
// 0x141a36ad8  4983c510                       addq	$0x10, %r13
// Frames: F117 -> F118 -> F119 -> F120 -> F119 -> F116 -> F114 -> F115
// 0x141a36adc  48837b1835                     cmpq	$0x35, 0x18(%rbx)
// 0x141a36ae1  0f851f050000                   jne	0x141a37006 <.text+0x1a36006>
// Frames: F116 -> F114 -> F115
// 0x141a36ae7  498b5500                       movq	(%r13), %rdx
// Frames: F121 -> F118 -> F119 -> F120 -> F119 -> F116 -> F114 -> F115
// 0x141a36aeb  41b835000000                   movl	$0x35, %r8d
// 0x141a36af1  e8fad0ee00                     callq	0x142923bf0 <.text+0x2922bf0>
// 0x141a36af6  85c0                           testl	%eax, %eax
// Frames: F116 -> F114 -> F115
// 0x141a36af8  0f8508050000                   jne	0x141a37006 <.text+0x1a36006>
// Frames: F122 -> F114 -> F115
// 0x141a36afe  488b0b                         movq	(%rbx), %rcx
// 0x141a36b01  488b4308                       movq	0x8(%rbx), %rax
// Frames: F114 -> F115
// 0x141a36b05  ff5018                         callq	*0x18(%rax)
// Frames: F115
// 0x141a36b08  4889c3                         movq	%rax, %rbx
// Frames: F123 -> F115
// 0x141a36b0b  4885c0                         testq	%rax, %rax
// 0x141a36b0e  0f8428050000                   je	0x141a3703c <.text+0x1a3603c>
// Frames: F124
// 0x141a36b14  498b4f08                       movq	0x8(%r15), %rcx
// Frames: F125 -> F126 -> F127 -> F124
// 0x141a36b18  89ca                           movl	%ecx, %edx
// 0x141a36b1a  4c8b5578                       movq	0x78(%rbp), %r10
// Frames: F128 -> F91 -> F125 -> F126 -> F127 -> F124
// 0x141a36b1e  493b5210                       cmpq	0x10(%r10), %rdx
// 0x141a36b22  0f832e050000                   jae	0x141a37056 <.text+0x1a36056>
// Frames: F129 -> F130 -> F131 -> F132 -> F93 -> F133 -> F125 -> F126 -> F127 -> F124
// 0x141a36b28  498b4208                       movq	0x8(%r10), %rax
// 0x141a36b2c  4989c8                         movq	%rcx, %r8
// 0x141a36b2f  49c1e820                       shrq	$0x20, %r8
// Frames: F134 -> F128 -> F91 -> F125 -> F126 -> F127 -> F124
// 0x141a36b33  4c8d0c92                       leaq	(%rdx,%rdx,4), %r9
// Frames: F135 -> F136 -> F137 -> F126 -> F127 -> F124
// 0x141a36b37  46390488                       cmpl	%r8d, (%rax,%r9,4)
// Frames: F137 -> F126 -> F127 -> F124
// 0x141a36b3b  0f851c050000                   jne	0x141a3705d <.text+0x1a3605d>
// Frames: F138 -> F139 -> F126 -> F127 -> F124
// 0x141a36b41  4a8d0488                       leaq	(%rax,%r9,4), %rax
// 0x141a36b45  448b4804                       movl	0x4(%rax), %r9d
// 0x141a36b49  41bbffffffff                   movl	$0xffffffff, %r11d      # imm = 0xFFFFFFFF
// 0x141a36b4f  4d39d9                         cmpq	%r11, %r9
// Frames: F139 -> F126 -> F127 -> F124
// 0x141a36b52  0f8405050000                   je	0x141a3705d <.text+0x1a3605d>
// Frames: F140 -> F141 -> F127 -> F124
// 0x141a36b58  4d394c2438                     cmpq	%r9, 0x38(%r12)
// Frames: F142 -> F143 -> F141 -> F127 -> F124
// 0x141a36b5d  0f86ea040000                   jbe	0x141a3704d <.text+0x1a3604d>
// Frames: F144 -> F145 -> F146 -> F142 -> F143 -> F141 -> F127 -> F124
// 0x141a36b63  498b542428                     movq	0x28(%r12), %rdx
// Frames: F97 -> F144 -> F145 -> F146 -> F142 -> F143 -> F141 -> F127 -> F124
// 0x141a36b68  4589c8                         movl	%r9d, %r8d
// 0x141a36b6b  41c1e806                       shrl	$0x6, %r8d
// Frames: F147 -> F145 -> F146 -> F142 -> F143 -> F141 -> F127 -> F124
// 0x141a36b6f  4a8b14c2                       movq	(%rdx,%r8,8), %rdx
// Frames: F145 -> F146 -> F142 -> F143 -> F141 -> F127 -> F124
// 0x141a36b73  4c0fa3ca                       btq	%r9, %rdx
// Frames: F148 -> F127 -> F124
// 0x141a36b77  0f83d0040000                   jae	0x141a3704d <.text+0x1a3604d>
// Frames: F129 -> F130 -> F131 -> F132 -> F93 -> F94 -> F149 -> F150 -> F127 -> F124
// 0x141a36b7d  8b480c                         movl	0xc(%rax), %ecx
// 0x141a36b80  4d8b8aa8010000                 movq	0x1a8(%r10), %r9
// Frames: F134 -> F128 -> F151 -> F149 -> F150 -> F127 -> F124
// 0x141a36b87  4c8d14c9                       leaq	(%rcx,%rcx,8), %r10
// Frames: F152 -> F127 -> F124
// 0x141a36b8b  4d8b842418010000               movq	0x118(%r12), %r8
// Frames: F153 -> F154 -> F155 -> F156 -> F157 -> F152 -> F127 -> F124
// 0x141a36b93  4b8b4cd118                     movq	0x18(%r9,%r10,8), %rcx
// 0x141a36b98  4b8b54d138                     movq	0x38(%r9,%r10,8), %rdx
// Frames: F128 -> F91 -> F158 -> F153 -> F154 -> F159 -> F160 -> F161 -> F157 -> F152 -> F127 -> F124
// 0x141a36b9d  4f3944d140                     cmpq	%r8, 0x40(%r9,%r10,8)
// 0x141a36ba2  761c                           jbe	0x141a36bc0 <.text+0x1a35bc0>
// Frames: F162 -> F56 -> F163 -> F158 -> F153 -> F154 -> F159 -> F160 -> F161 -> F157 -> F152 -> F127 -> F124
// 0x141a36ba4  4e8b04c2                       movq	(%rdx,%r8,8), %r8
// 0x141a36ba8  4d85c0                         testq	%r8, %r8
// Frames: F163 -> F158 -> F153 -> F154 -> F159 -> F160 -> F161 -> F157 -> F152 -> F127 -> F124
// 0x141a36bab  7413                           je	0x141a36bc0 <.text+0x1a35bc0>
// Frames: F164 -> F165 -> F166 -> F153 -> F154 -> F159 -> F160 -> F161 -> F157 -> F152 -> F127 -> F124
// 0x141a36bad  49f7d0                         notq	%r8
// Frames: F167 -> F168 -> F169 -> F166 -> F170 -> F160 -> F161 -> F157 -> F152 -> F127 -> F124
// 0x141a36bb0  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x141a36bb4  49c1e004                       shlq	$0x4, %r8
// 0x141a36bb8  4e8b6c0110                     movq	0x10(%rcx,%r8), %r13
// 0x141a36bbd  eb04                           jmp	0x141a36bc3 <.text+0x1a35bc3>
// 0x141a36bbf  90                             nop
// 0x141a36bc0  4531ed                         xorl	%r13d, %r13d
// Frames: F97 -> F171 -> F172 -> F173 -> F174 -> F175 -> F176 -> F127 -> F124
// 0x141a36bc3  448b7810                       movl	0x10(%rax), %r15d
// 0x141a36bc7  498b842410010000               movq	0x110(%r12), %rax
// 0x141a36bcf  488b04c2                       movq	(%rdx,%rax,8), %rax
// 0x141a36bd3  48f7d0                         notq	%rax
// 0x141a36bd6  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141a36bda  48c1e004                       shlq	$0x4, %rax
// 0x141a36bde  488b540110                     movq	0x10(%rcx,%rax), %rdx
// 0x141a36be3  488b740120                     movq	0x20(%rcx,%rax), %rsi
// 0x141a36be8  488b7c0128                     movq	0x28(%rcx,%rax), %rdi
// 0x141a36bed  4c89f8                         movq	%r15, %rax
// 0x141a36bf0  48c1e004                       shlq	$0x4, %rax
// 0x141a36bf4  4c8d3402                       leaq	(%rdx,%rax), %r14
// Frames: F177 -> F178 -> F179
// 0x141a36bf8  48833c0200                     cmpq	$0x0, (%rdx,%rax)
// Frames: F179
// 0x141a36bfd  0f85c5000000                   jne	0x141a36cc8 <.text+0x1a35cc8>
// Frames: F180 -> F181 -> F182 -> F183 -> F184 -> F185 -> F186 -> F187
// 0x141a36c03  0fb605ffdfbf01                 movzbl	0x1bfdfff(%rip), %eax   # 0x143634c09
// Frames: F188 -> F182 -> F183 -> F184 -> F185 -> F186 -> F187
// 0x141a36c0a  b920000000                     movl	$0x20, %ecx
// 0x141a36c0f  ba08000000                     movl	$0x8, %edx
// 0x141a36c14  e8f7cfbdfe                     callq	0x140613c10 <.text+0x612c10>
// Frames: F184 -> F185 -> F186 -> F187
// 0x141a36c19  4885c0                         testq	%rax, %rax
// 0x141a36c1c  0f84b0040000                   je	0x141a370d2 <.text+0x1a360d2>
// Frames: F185 -> F186 -> F187
// 0x141a36c22  4889c2                         movq	%rax, %rdx
// 0x141a36c25  48c70000000000                 movq	$0x0, (%rax)
// 0x141a36c2c  48c7400804000000               movq	$0x4, 0x8(%rax)
// 0x141a36c34  48c7401000000000               movq	$0x0, 0x10(%rax)
// 0x141a36c3c  c6401800                       movb	$0x0, 0x18(%rax)
// Frames: F189 -> F190 -> F191 -> F192 -> F187
// 0x141a36c40  4c8b4360                       movq	0x60(%rbx), %r8
// 0x141a36c44  eb0c                           jmp	0x141a36c52 <.text+0x1a35c52>
// 0x141a36c46  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F193 -> F194 -> F195 -> F192 -> F187
// 0x141a36c50  f390                           pause
// Frames: F196 -> F197 -> F198 -> F192 -> F187
// 0x141a36c52  498b4008                       movq	0x8(%r8), %rax
// 0x141a36c56  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F199 -> F192 -> F187
// 0x141a36c60  4883f8ff                       cmpq	$-0x1, %rax
// 0x141a36c64  74ea                           je	0x141a36c50 <.text+0x1a35c50>
// Frames: F200 -> F192 -> F187
// 0x141a36c66  4885c0                         testq	%rax, %rax
// 0x141a36c69  0f8841030000                   js	0x141a36fb0 <.text+0x1a35fb0>
// Frames: F201 -> F192 -> F187
// 0x141a36c6f  488d4801                       leaq	0x1(%rax), %rcx
// Frames: F202 -> F203 -> F201 -> F192 -> F187
// 0x141a36c73  f0                             lock
// 0x141a36c74  490fb14808                     cmpxchgq	%rcx, 0x8(%r8)
// Frames: F201 -> F192 -> F187
// 0x141a36c79  75e5                           jne	0x141a36c60 <.text+0x1a35c60>
// Frames: F204 -> F205 -> F187
// 0x141a36c7b  4c8985c0000000                 movq	%r8, 0xc0(%rbp)
// 0x141a36c82  488995b8000000                 movq	%rdx, 0xb8(%rbp)
// 0x141a36c89  8b85d4000000                   movl	0xd4(%rbp), %eax
// 0x141a36c8f  428904bf                       movl	%eax, (%rdi,%r15,4)
// Frames: F77 -> F187
// 0x141a36c93  49833e00                       cmpq	$0x0, (%r14)
// 0x141a36c97  4c89b5c8000000                 movq	%r14, 0xc8(%rbp)
// 0x141a36c9e  740c                           je	0x141a36cac <.text+0x1a35cac>
// 0x141a36ca0  488b8dc8000000                 movq	0xc8(%rbp), %rcx
// 0x141a36ca7  e8049039ff                     callq	0x140dcfcb0 <.text+0xdcecb0>
// Frames: F187
// 0x141a36cac  488b85c0000000                 movq	0xc0(%rbp), %rax
// 0x141a36cb3  4c8bb5c8000000                 movq	0xc8(%rbp), %r14
// 0x141a36cba  498906                         movq	%rax, (%r14)
// 0x141a36cbd  488b85b8000000                 movq	0xb8(%rbp), %rax
// 0x141a36cc4  49894608                       movq	%rax, 0x8(%r14)
// Frames: F204 -> F205 -> F206
// 0x141a36cc8  8b85d4000000                   movl	0xd4(%rbp), %eax
// 0x141a36cce  428904bf                       movl	%eax, (%rdi,%r15,4)
// Frames: F101 -> F206
// 0x141a36cd2  49833e00                       cmpq	$0x0, (%r14)
// Frames: F123 -> F206
// 0x141a36cd6  0f84cc030000                   je	0x141a370a8 <.text+0x1a360a8>
// Frames: F207 -> F208
// 0x141a36cdc  4d8b7608                       movq	0x8(%r14), %r14
// Frames: F123 -> F207 -> F208
// 0x141a36ce0  4d85f6                         testq	%r14, %r14
// 0x141a36ce3  0f84cd030000                   je	0x141a370b6 <.text+0x1a360b6>
// Frames: F209 -> F208
// 0x141a36ce9  498d7e18                       leaq	0x18(%r14), %rdi
// Frames: F202 -> F210 -> F211 -> F209 -> F208
// 0x141a36ced  31c0                           xorl	%eax, %eax
// 0x141a36cef  b101                           movb	$0x1, %cl
// 0x141a36cf1  f0                             lock
// 0x141a36cf2  410fb04e18                     cmpxchgb	%cl, 0x18(%r14)
// Frames: F212 -> F209 -> F208
// 0x141a36cf7  0f8530020000                   jne	0x141a36f2d <.text+0x1a35f2d>
// Frames: F213 -> F214 -> F209 -> F208
// 0x141a36cfd  4889f9                         movq	%rdi, %rcx
// 0x141a36d00  e86baf24ff                     callq	0x140c81c70 <.text+0xc80c70>
// Frames: F215 -> F208
// 0x141a36d05  498b4610                       movq	0x10(%r14), %rax
// Frames: F216 -> F215 -> F208
// 0x141a36d09  4885c0                         testq	%rax, %rax
// 0x141a36d0c  0f94c1                         sete	%cl
// 0x141a36d0f  48c1e002                       shlq	$0x2, %rax
// 0x141a36d13  49034608                       addq	0x8(%r14), %rax
// Frames: F217 -> F215 -> F208
// 0x141a36d17  4883c0fc                       addq	$-0x4, %rax
// 0x141a36d1b  0f94c2                         sete	%dl
// 0x141a36d1e  08ca                           orb	%cl, %dl
// 0x141a36d20  741e                           je	0x141a36d40 <.text+0x1a35d40>
// Frames: F218 -> F208
// 0x141a36d22  488b4328                       movq	0x28(%rbx), %rax
// Frames: F88 -> F219 -> F220 -> F215 -> F208
// 0x141a36d26  4885c0                         testq	%rax, %rax
// Frames: F123 -> F219 -> F220 -> F215 -> F208
// 0x141a36d29  0f8495030000                   je	0x141a370c4 <.text+0x1a360c4>
// Frames: F219 -> F220 -> F215 -> F208
// 0x141a36d2f  8d48ff                         leal	-0x1(%rax), %ecx
// 0x141a36d32  eb12                           jmp	0x141a36d46 <.text+0x1a35d46>
// 0x141a36d34  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F221 -> F215 -> F208
// 0x141a36d40  8b08                           movl	(%rax), %ecx
// Frames: F218 -> F208
// 0x141a36d42  488b4328                       movq	0x28(%rbx), %rax
// Frames: F222 -> F208
// 0x141a36d46  488b5320                       movq	0x20(%rbx), %rdx
// Frames: F223 -> F208
// 0x141a36d4a  f30f6f4330                     movdqu	0x30(%rbx), %xmm0
// Frames: F224 -> F208
// 0x141a36d4f  f30f6f4b40                     movdqu	0x40(%rbx), %xmm1
// Frames: F225 -> F208
// 0x141a36d54  f30f6f5350                     movdqu	0x50(%rbx), %xmm2
// Frames: F226 -> F208
// 0x141a36d59  894d50                         movl	%ecx, 0x50(%rbp)
// 0x141a36d5c  48895500                       movq	%rdx, (%rbp)
// 0x141a36d60  48894508                       movq	%rax, 0x8(%rbp)
// 0x141a36d64  f30f7f4510                     movdqu	%xmm0, 0x10(%rbp)
// 0x141a36d69  4c897520                       movq	%r14, 0x20(%rbp)
// 0x141a36d6d  f30f7f4d28                     movdqu	%xmm1, 0x28(%rbp)
// 0x141a36d72  48895d38                       movq	%rbx, 0x38(%rbp)
// 0x141a36d76  f30f7f5540                     movdqu	%xmm2, 0x40(%rbp)
// Frames: F227
// 0x141a36d7b  4889e9                         movq	%rbp, %rcx
// 0x141a36d7e  e8cd550000                     callq	0x141a3c350 <.text+0x1a3b350>
// Frames: F228
// 0x141a36d83  496bc758                       imulq	$0x58, %r15, %rax
// 0x141a36d87  4901c5                         addq	%rax, %r13
// 0x141a36d8a  4c89e9                         movq	%r13, %rcx
// 0x141a36d8d  e8eec5eafe                     callq	0x1408e3380 <.text+0x8e2380>
// 0x141a36d92  4a8d3cbe                       leaq	(%rsi,%r15,4), %rdi
// 0x141a36d96  84c0                           testb	%al, %al
// 0x141a36d98  7436                           je	0x141a36dd0 <.text+0x1a35dd0>
// Frames: F77 -> F229
// 0x141a36d9a  488b5d20                       movq	0x20(%rbp), %rbx
// Frames: F230 -> F77 -> F229
// 0x141a36d9e  4883c318                       addq	$0x18, %rbx
// Frames: F231 -> F232 -> F230 -> F77 -> F229
// 0x141a36da2  4889d9                         movq	%rbx, %rcx
// 0x141a36da5  e866b024ff                     callq	0x140c81e10 <.text+0xc80e10>
// Frames: F233 -> F234 -> F235 -> F230 -> F77 -> F229
// 0x141a36daa  b001                           movb	$0x1, %al
// 0x141a36dac  31c9                           xorl	%ecx, %ecx
// 0x141a36dae  f0                             lock
// 0x141a36daf  0fb00b                         cmpxchgb	%cl, (%rbx)
// Frames: F236 -> F230 -> F77 -> F229
// 0x141a36db2  0f8488fcffff                   je	0x141a36a40 <.text+0x1a35a40>
// Frames: F237 -> F230 -> F77 -> F229
// 0x141a36db8  4889d9                         movq	%rbx, %rcx
// 0x141a36dbb  31d2                           xorl	%edx, %edx
// 0x141a36dbd  e8ce6de900                     callq	0x1428cdb90 <.text+0x28ccb90>
// 0x141a36dc2  e979fcffff                     jmp	0x141a36a40 <.text+0x1a35a40>
// 0x141a36dc7  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F238 -> F239 -> F240 -> F241
// 0x141a36dd0  488b8590000000                 movq	0x90(%rbp), %rax
// 0x141a36dd7  4883781800                     cmpq	$0x0, 0x18(%rax)
// 0x141a36ddc  0f84ae000000                   je	0x141a36e90 <.text+0x1a35e90>
// Frames: F242 -> F243 -> F239 -> F240 -> F241
// 0x141a36de2  488b8d98000000                 movq	0x98(%rbp), %rcx
// 0x141a36de9  4c89ea                         movq	%r13, %rdx
// 0x141a36dec  4889c6                         movq	%rax, %rsi
// 0x141a36def  e87c9fb7fe                     callq	0x1405b0d70 <.text+0x5afd70>
// 0x141a36df4  488b0e                         movq	(%rsi), %rcx
// Frames: F244 -> F245 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36df7  488b5608                       movq	0x8(%rsi), %rdx
// 0x141a36dfb  4989d0                         movq	%rdx, %r8
// 0x141a36dfe  4921c0                         andq	%rax, %r8
// Frames: F249 -> F250 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e01  48c1e839                       shrq	$0x39, %rax
// 0x141a36e05  660f6ec0                       movd	%eax, %xmm0
// 0x141a36e09  660f60c0                       punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
// 0x141a36e0d  f20f70c000                     pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
// 0x141a36e12  660f70c000                     pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
// 0x141a36e17  488d41e0                       leaq	-0x20(%rcx), %rax
// 0x141a36e1b  4531c9                         xorl	%r9d, %r9d
// Frames: F251 -> F252 -> F253 -> F254 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e1e  f3420f6f0c01                   movdqu	(%rcx,%r8), %xmm1
// Frames: F255 -> F256 -> F257 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e24  660f6fd1                       movdqa	%xmm1, %xmm2
// 0x141a36e28  660f74d0                       pcmpeqb	%xmm0, %xmm2
// 0x141a36e2c  66440fd7d2                     pmovmskb	%xmm2, %r10d
// Frames: F258 -> F259 -> F257 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e31  4585d2                         testl	%r10d, %r10d
// 0x141a36e34  743a                           je	0x141a36e70 <.text+0x1a35e70>
// 0x141a36e36  4d8b5d00                       movq	(%r13), %r11
// Frames: F260 -> F261 -> F262 -> F259 -> F257 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e3a  f3410fbcf2                     tzcntl	%r10d, %esi
// Frames: F263 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e3f  4c01c6                         addq	%r8, %rsi
// 0x141a36e42  4821d6                         andq	%rdx, %rsi
// Frames: F264 -> F265 -> F266 -> F267 -> F268 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e45  4889f3                         movq	%rsi, %rbx
// 0x141a36e48  48c1e305                       shlq	$0x5, %rbx
// 0x141a36e4c  4989c6                         movq	%rax, %r14
// 0x141a36e4f  4929de                         subq	%rbx, %r14
// Frames: F269 -> F119 -> F270 -> F271 -> F267 -> F268 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e52  4d3b1e                         cmpq	(%r14), %r11
// Frames: F272 -> F268 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e55  7475                           je	0x141a36ecc <.text+0x1a35ecc>
// Frames: F273 -> F260 -> F261 -> F262 -> F259 -> F257 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e57  418d72ff                       leal	-0x1(%r10), %esi
// Frames: F258 -> F259 -> F257 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e5b  664421d6                       andw	%r10w, %si
// 0x141a36e5f  4189f2                         movl	%esi, %r10d
// 0x141a36e62  75d6                           jne	0x141a36e3a <.text+0x1a35e3a>
// 0x141a36e64  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F255 -> F256 -> F274 -> F275 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e70  660f74ce                       pcmpeqb	%xmm6, %xmm1
// 0x141a36e74  66440fd7d1                     pmovmskb	%xmm1, %r10d
// Frames: F276 -> F275 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e79  4585d2                         testl	%r10d, %r10d
// 0x141a36e7c  7512                           jne	0x141a36e90 <.text+0x1a35e90>
// Frames: F277 -> F278 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e7e  4d01c8                         addq	%r9, %r8
// 0x141a36e81  4983c010                       addq	$0x10, %r8
// Frames: F279 -> F278 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e85  4983c110                       addq	$0x10, %r9
// Frames: F280 -> F278 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e89  4921d0                         andq	%rdx, %r8
// Frames: F281 -> F246 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36e8c  eb90                           jmp	0x141a36e1e <.text+0x1a35e1e>
// 0x141a36e8e  6690                           nop
// Frames: F282
// 0x141a36e90  4531c0                         xorl	%r8d, %r8d
// 0x141a36e93  498d450c                       leaq	0xc(%r13), %rax
// 0x141a36e97  41f6450801                     testb	$0x1, 0x8(%r13)
// 0x141a36e9c  744a                           je	0x141a36ee8 <.text+0x1a35ee8>
// Frames: F283
// 0x141a36e9e  488b4810                       movq	0x10(%rax), %rcx
// 0x141a36ea2  48894dd0                       movq	%rcx, -0x30(%rbp)
// 0x141a36ea6  f30f6f00                       movdqu	(%rax), %xmm0
// 0x141a36eaa  660f7f45c0                     movdqa	%xmm0, -0x40(%rbp)
// Frames: F284
// 0x141a36eaf  4889e8                         movq	%rbp, %rax
// 0x141a36eb2  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a36eb7  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x141a36ebb  4c89ea                         movq	%r13, %rdx
// 0x141a36ebe  4c8b8da0000000                 movq	0xa0(%rbp), %r9
// 0x141a36ec5  e826807700                     callq	0x1421aeef0 <.text+0x21adef0>
// 0x141a36eca  eb34                           jmp	0x141a36f00 <.text+0x1a35f00>
// Frames: F285 -> F247 -> F248 -> F239 -> F240 -> F241
// 0x141a36ecc  48f7de                         negq	%rsi
// 0x141a36ecf  48c1e605                       shlq	$0x5, %rsi
// Frames: F286 -> F287 -> F288 -> F241
// 0x141a36ed3  48837c31f800                   cmpq	$0x0, -0x8(%rcx,%rsi)
// 0x141a36ed9  410f95c0                       setne	%r8b
// 0x141a36edd  498d450c                       leaq	0xc(%r13), %rax
// Frames: F282
// 0x141a36ee1  41f6450801                     testb	$0x1, 0x8(%r13)
// 0x141a36ee6  75b6                           jne	0x141a36e9e <.text+0x1a35e9e>
// Frames: F289
// 0x141a36ee8  f30f6f00                       movdqu	(%rax), %xmm0
// 0x141a36eec  660f7f45e0                     movdqa	%xmm0, -0x20(%rbp)
// Frames: F290
// 0x141a36ef1  488d4de0                       leaq	-0x20(%rbp), %rcx
// 0x141a36ef5  4c89ea                         movq	%r13, %rdx
// 0x141a36ef8  4989e9                         movq	%rbp, %r9
// 0x141a36efb  e820c27700                     callq	0x1421b3120 <.text+0x21b2120>
// Frames: F77 -> F229
// 0x141a36f00  488b5d20                       movq	0x20(%rbp), %rbx
// Frames: F230 -> F77 -> F229
// 0x141a36f04  4883c318                       addq	$0x18, %rbx
// Frames: F231 -> F232 -> F230 -> F77 -> F229
// 0x141a36f08  4889d9                         movq	%rbx, %rcx
// 0x141a36f0b  e800af24ff                     callq	0x140c81e10 <.text+0xc80e10>
// Frames: F233 -> F234 -> F235 -> F230 -> F77 -> F229
// 0x141a36f10  b001                           movb	$0x1, %al
// 0x141a36f12  31c9                           xorl	%ecx, %ecx
// 0x141a36f14  f0                             lock
// 0x141a36f15  0fb00b                         cmpxchgb	%cl, (%rbx)
// Frames: F236 -> F230 -> F77 -> F229
// 0x141a36f18  0f8422fbffff                   je	0x141a36a40 <.text+0x1a35a40>
// Frames: F237 -> F230 -> F77 -> F229
// 0x141a36f1e  4889d9                         movq	%rbx, %rcx
// 0x141a36f21  31d2                           xorl	%edx, %edx
// 0x141a36f23  e8686ce900                     callq	0x1428cdb90 <.text+0x28ccb90>
// 0x141a36f28  e913fbffff                     jmp	0x141a36a40 <.text+0x1a35a40>
// Frames: F291 -> F209 -> F208
// 0x141a36f2d  4889f9                         movq	%rdi, %rcx
// 0x141a36f30  41b800ca9a3b                   movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
// 0x141a36f36  e8d562e900                     callq	0x1428cd210 <.text+0x28cc210>
// 0x141a36f3b  e9bdfdffff                     jmp	0x141a36cfd <.text+0x1a35cfd>
// Frames: F77 -> F292
// 0x141a36f40  4883bda800000000               cmpq	$0x0, 0xa8(%rbp)
// 0x141a36f48  744b                           je	0x141a36f95 <.text+0x1a35f95>
// Frames: F55 -> F56 -> F57 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36f4a  e8e1111eff                     callq	0x140c18130 <.text+0xc17130>
// 0x141a36f4f  4889c1                         movq	%rax, %rcx
// Frames: F61 -> F55 -> F56 -> F57 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36f52  488b00                         movq	(%rax), %rax
// 0x141a36f55  4883f801                       cmpq	$0x1, %rax
// 0x141a36f59  0f8583000000                   jne	0x141a36fe2 <.text+0x1a35fe2>
// Frames: F62 -> F55 -> F56 -> F57 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36f5f  4883c108                       addq	$0x8, %rcx
// Frames: F63 -> F64 -> F65 -> F66 -> F67 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36f63  48833900                       cmpq	$0x0, (%rcx)
// 0x141a36f67  0f85c3000000                   jne	0x141a37030 <.text+0x1a36030>
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F64 -> F65 -> F66 -> F67 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36f6d  48c701ffffffff                 movq	$-0x1, (%rcx)
// 0x141a36f74  48898de0000000                 movq	%rcx, 0xe0(%rbp)
// Frames: F73 -> F74 -> F65 -> F66 -> F67 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36f7b  4883c108                       addq	$0x8, %rcx
// Frames: F294 -> F66 -> F67 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36f7f  488b95b0000000                 movq	0xb0(%rbp), %rdx
// 0x141a36f86  e865011eff                     callq	0x140c170f0 <.text+0xc160f0>
// Frames: F68 -> F69 -> F70 -> F71 -> F76 -> F77 -> F77 -> F66 -> F67 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36f8b  488b85e0000000                 movq	0xe0(%rbp), %rax
// 0x141a36f92  48ff00                         incq	(%rax)
// Frames: F292
// 0x141a36f95  0f28b5f0000000                 movaps	0xf0(%rbp), %xmm6
// 0x141a36f9c  4881c488010000                 addq	$0x188, %rsp            # imm = 0x188
// 0x141a36fa3  5b                             popq	%rbx
// 0x141a36fa4  5f                             popq	%rdi
// 0x141a36fa5  5e                             popq	%rsi
// 0x141a36fa6  415c                           popq	%r12
// 0x141a36fa8  415d                           popq	%r13
// 0x141a36faa  415e                           popq	%r14
// 0x141a36fac  415f                           popq	%r15
// 0x141a36fae  5d                             popq	%rbp
// 0x141a36faf  c3                             retq
// Frames: F200 -> F192 -> F187
// 0x141a36fb0  488d0dd9494701                 leaq	0x14749d9(%rip), %rcx   # 0x142eab990
// 0x141a36fb7  488d15324a4701                 leaq	0x1474a32(%rip), %rdx   # 0x142eab9f0
// 0x141a36fbe  e83d2ae700                     callq	0x1428a9a00 <.text+0x28a8a00>
// 0x141a36fc3  e919010000                     jmp	0x141a370e1 <.text+0x1a360e1>
// Frames: F61 -> F55 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x141a36fc8  4885c0                         testq	%rax, %rax
// 0x141a36fcb  752d                           jne	0x141a36ffa <.text+0x1a35ffa>
// Frames: F295 -> F55 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x141a36fcd  31d2                           xorl	%edx, %edx
// 0x141a36fcf  e86c13ee00                     callq	0x142918340 <.text+0x2917340>
// 0x141a36fd4  4889c1                         movq	%rax, %rcx
// Frames: F296 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x141a36fd7  4885c0                         testq	%rax, %rax
// 0x141a36fda  0f852cf9ffff                   jne	0x141a3690c <.text+0x1a3590c>
// 0x141a36fe0  eb18                           jmp	0x141a36ffa <.text+0x1a35ffa>
// Frames: F61 -> F55 -> F56 -> F57 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36fe2  4885c0                         testq	%rax, %rax
// 0x141a36fe5  7513                           jne	0x141a36ffa <.text+0x1a35ffa>
// Frames: F295 -> F55 -> F56 -> F57 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36fe7  31d2                           xorl	%edx, %edx
// 0x141a36fe9  e85213ee00                     callq	0x142918340 <.text+0x2917340>
// 0x141a36fee  4889c1                         movq	%rax, %rcx
// Frames: F296 -> F57 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a36ff1  4885c0                         testq	%rax, %rax
// 0x141a36ff4  0f8569ffffff                   jne	0x141a36f63 <.text+0x1a35f63>
// 0x141a36ffa  488d0de7f45b01                 leaq	0x15bf4e7(%rip), %rcx   # 0x142ff64e8
// 0x141a37001  e8ca10ea00                     callq	0x1428d80d0 <.text+0x28d70d0>
// Frames: F116 -> F114 -> F115
// 0x141a37006  48c7450000000000               movq	$0x0, (%rbp)
// 0x141a3700e  488d0533434801                 leaq	0x1484333(%rip), %rax   # 0x142ebb348
// 0x141a37015  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a3701a  488d55f0                       leaq	-0x10(%rbp), %rdx
// 0x141a3701e  4989e9                         movq	%rbp, %r9
// 0x141a37021  31c9                           xorl	%ecx, %ecx
// 0x141a37023  4d89e8                         movq	%r13, %r8
// 0x141a37026  e8163be900                     callq	0x1428cab41 <.text+0x28c9b41>
// 0x141a3702b  e9b1000000                     jmp	0x141a370e1 <.text+0x1a360e1>
// 0x141a37030  488d0d21f55b01                 leaq	0x15bf521(%rip), %rcx   # 0x142ff6558
// 0x141a37037  e81422ea00                     callq	0x1428d9250 <.text+0x28d8250>
// Frames: F297 -> F115
// 0x141a3703c  488d0dd5374801                 leaq	0x14837d5(%rip), %rcx   # 0x142eba818
// 0x141a37043  e8c822ea00                     callq	0x1428d9310 <.text+0x28d8310>
// 0x141a37048  e994000000                     jmp	0x141a370e1 <.text+0x1a360e1>
// Frames: F298 -> F127 -> F124
// 0x141a3704d  49c1e120                       shlq	$0x20, %r9
// 0x141a37051  4c89cf                         movq	%r9, %rdi
// 0x141a37054  eb21                           jmp	0x141a37077 <.text+0x1a36077>
// Frames: F299 -> F126 -> F127 -> F124
// 0x141a37056  48c1e920                       shrq	$0x20, %rcx
// 0x141a3705a  4989c8                         movq	%rcx, %r8
// Frames: F300 -> F126 -> F127 -> F124
// 0x141a3705d  48b800000000ffffffff           movabsq	$-0x100000000, %rax # imm = 0xFFFFFFFF00000000
// 0x141a37067  4821c7                         andq	%rax, %rdi
// 0x141a3706a  48ffc7                         incq	%rdi
// 0x141a3706d  49c1e020                       shlq	$0x20, %r8
// 0x141a37071  4909d0                         orq	%rdx, %r8
// 0x141a37074  4c89c1                         movq	%r8, %rcx
// Frames: F301 -> F124
// 0x141a37077  48897d00                       movq	%rdi, (%rbp)
// 0x141a3707b  48894d08                       movq	%rcx, 0x8(%rbp)
// 0x141a3707f  488d05aa374801                 leaq	0x14837aa(%rip), %rax   # 0x142eba830
// 0x141a37086  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a3708b  488d0d4e354801                 leaq	0x148354e(%rip), %rcx   # 0x142eba5e0
// 0x141a37092  4c8d0d27354801                 leaq	0x1483527(%rip), %r9    # 0x142eba5c0
// 0x141a37099  4989e8                         movq	%rbp, %r8
// 0x141a3709c  ba2b000000                     movl	$0x2b, %edx
// 0x141a370a1  e8ba26ea00                     callq	0x1428d9760 <.text+0x28d8760>
// 0x141a370a6  eb39                           jmp	0x141a370e1 <.text+0x1a360e1>
// Frames: F297 -> F206
// 0x141a370a8  488d0d99374801                 leaq	0x1483799(%rip), %rcx   # 0x142eba848
// 0x141a370af  e85c22ea00                     callq	0x1428d9310 <.text+0x28d8310>
// 0x141a370b4  eb2b                           jmp	0x141a370e1 <.text+0x1a360e1>
// Frames: F297 -> F207 -> F208
// 0x141a370b6  488d0d43434801                 leaq	0x1484343(%rip), %rcx   # 0x142ebb400
// 0x141a370bd  e84e22ea00                     callq	0x1428d9310 <.text+0x28d8310>
// 0x141a370c2  eb1d                           jmp	0x141a370e1 <.text+0x1a360e1>
// Frames: F297 -> F219 -> F220 -> F215 -> F208
// 0x141a370c4  488d0d4d434801                 leaq	0x148434d(%rip), %rcx   # 0x142ebb418
// 0x141a370cb  e84022ea00                     callq	0x1428d9310 <.text+0x28d8310>
// 0x141a370d0  eb0f                           jmp	0x141a370e1 <.text+0x1a360e1>
// Frames: F302 -> F185 -> F186 -> F187
// 0x141a370d2  b908000000                     movl	$0x8, %ecx
// 0x141a370d7  ba20000000                     movl	$0x20, %edx
// 0x141a370dc  e81f1fea00                     callq	0x1428d9000 <.text+0x28d8000>
// 0x141a370e1  0f0b                           ud2
// 0x141a370e3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x141a370f0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141a370f5  55                             pushq	%rbp
// 0x141a370f6  4157                           pushq	%r15
// 0x141a370f8  4156                           pushq	%r14
// 0x141a370fa  4155                           pushq	%r13
// 0x141a370fc  4154                           pushq	%r12
// 0x141a370fe  56                             pushq	%rsi
// 0x141a370ff  57                             pushq	%rdi
// 0x141a37100  53                             pushq	%rbx
// 0x141a37101  4883ec48                       subq	$0x48, %rsp
// 0x141a37105  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141a3710c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x141a37112  488b85e0000000                 movq	0xe0(%rbp), %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F76 -> F77 -> F77 -> F66 -> F67 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a37119  48ff00                         incq	(%rax)
// 0x141a3711c  0f28742430                     movaps	0x30(%rsp), %xmm6
// Frames: F66 -> F67 -> F58 -> F59 -> F293 -> F77 -> F77 -> F292
// 0x141a37121  4883c448                       addq	$0x48, %rsp
// 0x141a37125  5b                             popq	%rbx
// 0x141a37126  5f                             popq	%rdi
// 0x141a37127  5e                             popq	%rsi
// 0x141a37128  415c                           popq	%r12
// 0x141a3712a  415d                           popq	%r13
// 0x141a3712c  415e                           popq	%r14
// 0x141a3712e  415f                           popq	%r15
// 0x141a37130  5d                             popq	%rbp
// 0x141a37131  c3                             retq
// 0x141a37132  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x141a37140  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141a37145  55                             pushq	%rbp
// 0x141a37146  4157                           pushq	%r15
// 0x141a37148  4156                           pushq	%r14
// 0x141a3714a  4155                           pushq	%r13
// 0x141a3714c  4154                           pushq	%r12
// 0x141a3714e  56                             pushq	%rsi
// 0x141a3714f  57                             pushq	%rdi
// 0x141a37150  53                             pushq	%rbx
// 0x141a37151  4883ec48                       subq	$0x48, %rsp
// 0x141a37155  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141a3715c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x141a37162  488b85d8000000                 movq	0xd8(%rbp), %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F76 -> F77 -> F77 -> F66 -> F67 -> F58 -> F59 -> F60 -> F2
// 0x141a37169  48ff00                         incq	(%rax)
// 0x141a3716c  0f28742430                     movaps	0x30(%rsp), %xmm6
// Frames: F66 -> F67 -> F58 -> F59 -> F60 -> F2
// 0x141a37171  4883c448                       addq	$0x48, %rsp
// 0x141a37175  5b                             popq	%rbx
// 0x141a37176  5f                             popq	%rdi
// 0x141a37177  5e                             popq	%rsi
// 0x141a37178  415c                           popq	%r12
// 0x141a3717a  415d                           popq	%r13
// 0x141a3717c  415e                           popq	%r14
// 0x141a3717e  415f                           popq	%r15
// 0x141a37180  5d                             popq	%rbp
// 0x141a37181  c3                             retq
// 0x141a37182  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x141a37190  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141a37195  55                             pushq	%rbp
// 0x141a37196  4157                           pushq	%r15
// 0x141a37198  4156                           pushq	%r14
// 0x141a3719a  4155                           pushq	%r13
// 0x141a3719c  4154                           pushq	%r12
// 0x141a3719e  56                             pushq	%rsi
// 0x141a3719f  57                             pushq	%rdi
// 0x141a371a0  53                             pushq	%rbx
// 0x141a371a1  4883ec48                       subq	$0x48, %rsp
// 0x141a371a5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141a371ac  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x141a371b2  488b85c0000000                 movq	0xc0(%rbp), %rax
// 0x141a371b9  488b8dc8000000                 movq	0xc8(%rbp), %rcx
// Frames: F187
// 0x141a371c0  488901                         movq	%rax, (%rcx)
// 0x141a371c3  488b85b8000000                 movq	0xb8(%rbp), %rax
// 0x141a371ca  48894108                       movq	%rax, 0x8(%rcx)
// 0x141a371ce  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141a371d3  4883c448                       addq	$0x48, %rsp
// 0x141a371d7  5b                             popq	%rbx
// 0x141a371d8  5f                             popq	%rdi
// 0x141a371d9  5e                             popq	%rsi
// 0x141a371da  415c                           popq	%r12
// 0x141a371dc  415d                           popq	%r13
// 0x141a371de  415e                           popq	%r14
// 0x141a371e0  415f                           popq	%r15
// 0x141a371e2  5d                             popq	%rbp
// 0x141a371e3  c3                             retq
// 0x141a371e4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// 0x141a371f0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141a371f5  55                             pushq	%rbp
// 0x141a371f6  4157                           pushq	%r15
// 0x141a371f8  4156                           pushq	%r14
// 0x141a371fa  4155                           pushq	%r13
// 0x141a371fc  4154                           pushq	%r12
// 0x141a371fe  56                             pushq	%rsi
// 0x141a371ff  57                             pushq	%rdi
// 0x141a37200  53                             pushq	%rbx
// 0x141a37201  4883ec48                       subq	$0x48, %rsp
// 0x141a37205  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141a3720c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x141a37212  4889e9                         movq	%rbp, %rcx
// Frames: F229
// 0x141a37215  e8167abafe                     callq	0x1405dec30 <.text+0x5ddc30>
// 0x141a3721a  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141a3721f  4883c448                       addq	$0x48, %rsp
// 0x141a37223  5b                             popq	%rbx
// 0x141a37224  5f                             popq	%rdi
// 0x141a37225  5e                             popq	%rsi
// 0x141a37226  415c                           popq	%r12
// 0x141a37228  415d                           popq	%r13
// 0x141a3722a  415e                           popq	%r14
// 0x141a3722c  415f                           popq	%r15
// 0x141a3722e  5d                             popq	%rbp
// 0x141a3722f  c3                             retq
// 0x141a37230  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141a37235  55                             pushq	%rbp
// 0x141a37236  4157                           pushq	%r15
// 0x141a37238  4156                           pushq	%r14
// 0x141a3723a  4155                           pushq	%r13
// 0x141a3723c  4154                           pushq	%r12
// 0x141a3723e  56                             pushq	%rsi
// 0x141a3723f  57                             pushq	%rdi
// 0x141a37240  53                             pushq	%rbx
// 0x141a37241  4883ec48                       subq	$0x48, %rsp
// 0x141a37245  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141a3724c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// Frames: F77 -> F292
// 0x141a37252  4883bda800000000               cmpq	$0x0, 0xa8(%rbp)
// 0x141a3725a  7423                           je	0x141a3727f <.text+0x1a3627f>
// 0x141a3725c  488d85b0000000                 leaq	0xb0(%rbp), %rax
// 0x141a37263  48894560                       movq	%rax, 0x60(%rbp)
// 0x141a37267  488d4560                       leaq	0x60(%rbp), %rax
// 0x141a3726b  48894558                       movq	%rax, 0x58(%rbp)
// Frames: F59 -> F293 -> F77 -> F77 -> F292
// 0x141a3726f  488d0dda334801                 leaq	0x14833da(%rip), %rcx   # 0x142eba650
// 0x141a37276  488d5558                       leaq	0x58(%rbp), %rdx
// 0x141a3727a  e861917600                     callq	0x1421a03e0 <.text+0x219f3e0>
// Frames: F1
// 0x141a3727f  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141a37284  4883c448                       addq	$0x48, %rsp
// 0x141a37288  5b                             popq	%rbx
// 0x141a37289  5f                             popq	%rdi
// 0x141a3728a  5e                             popq	%rsi
// 0x141a3728b  415c                           popq	%r12
// 0x141a3728d  415d                           popq	%r13
// 0x141a3728f  415e                           popq	%r14
// 0x141a37291  415f                           popq	%r15
// 0x141a37293  5d                             popq	%rbp
// 0x141a37294  c3                             retq
//
// RANGE rva_21b2ca0_len_3c3: RVA 0x21b2ca0, 963 PDB bytes; 217 instructions
// Evidence: _assembly/roof-v1/B/pack-003/rva_21b2ca0_len_3c3.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3474:31176: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// Frames: F303
// 0x1421b2ca0  55                             pushq	%rbp
// 0x1421b2ca1  56                             pushq	%rsi
// 0x1421b2ca2  4881ece8000000                 subq	$0xe8, %rsp
// 0x1421b2ca9  488dac2480000000               leaq	0x80(%rsp), %rbp
// Frames: F304
// 0x1421b2cb1  48c74560feffffff               movq	$-0x2, 0x60(%rbp)
// 0x1421b2cb9  e8b29388ff                     callq	0x141a3c070 <.text+0x1a3b070>
// 0x1421b2cbe  84c0                           testb	%al, %al
// 0x1421b2cc0  0f84ee010000                   je	0x1421b2eb4 <.text+0x21b1eb4>
// Frames: F305 -> F306 -> F307 -> F308 -> F309
// 0x1421b2cc6  8b05ec7b4801                   movl	0x1487bec(%rip), %eax   # 0x14363a8b8
// Frames: F307 -> F308 -> F309
// 0x1421b2ccc  83f803                         cmpl	$0x3, %eax
// Frames: F308 -> F309
// 0x1421b2ccf  0f85e9010000                   jne	0x1421b2ebe <.text+0x21b1ebe>
// Frames: F309
// 0x1421b2cd5  488b15d47b4801                 movq	0x1487bd4(%rip), %rdx   # 0x14363a8b0
// 0x1421b2cdc  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x1421b2ce0  e8abfb38fe                     callq	0x140542890 <.text+0x541890>
// Frames: F310 -> F309
// 0x1421b2ce5  837db001                       cmpl	$0x1, -0x50(%rbp)
// 0x1421b2ce9  0f844c020000                   je	0x1421b2f3b <.text+0x21b1f3b>
// Frames: F311 -> F309
// 0x1421b2cef  488b45b8                       movq	-0x48(%rbp), %rax
// 0x1421b2cf3  48894538                       movq	%rax, 0x38(%rbp)
// 0x1421b2cf7  0fb645c0                       movzbl	-0x40(%rbp), %eax
// 0x1421b2cfb  88455f                         movb	%al, 0x5f(%rbp)
// 0x1421b2cfe  0fb605041f4801                 movzbl	0x1481f04(%rip), %eax   # 0x143634c09
// Frames: F188 -> F182 -> F183 -> F312 -> F313 -> F314 -> F315 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F309
// 0x1421b2d05  b90f000000                     movl	$0xf, %ecx
// 0x1421b2d0a  ba01000000                     movl	$0x1, %edx
// 0x1421b2d0f  e8fc0e46fe                     callq	0x140613c10 <.text+0x612c10>
// Frames: F323 -> F313 -> F314 -> F315 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F309
// 0x1421b2d14  4885c0                         testq	%rax, %rax
// 0x1421b2d17  0f8457020000                   je	0x1421b2f74 <.text+0x21b1f74>
// Frames: F251 -> F324 -> F325 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F309
// 0x1421b2d1d  48b97061746821202829           movabsq	$0x2928202168746170, %rcx # imm = 0x2928202168746170
// 0x1421b2d27  48894807                       movq	%rcx, 0x7(%rax)
// 0x1421b2d2b  48b96d6f64756c655f70           movabsq	$0x705f656c75646f6d, %rcx # imm = 0x705F656C75646F6D
// 0x1421b2d35  488908                         movq	%rcx, (%rax)
// Frames: F326 -> F321 -> F322 -> F309
// 0x1421b2d38  48c745b00f000000               movq	$0xf, -0x50(%rbp)
// 0x1421b2d40  488945b8                       movq	%rax, -0x48(%rbp)
// 0x1421b2d44  48c745c00f000000               movq	$0xf, -0x40(%rbp)
// Frames: F327 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2d4c  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421b2d55  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x1421b2d59  ba0f000000                     movl	$0xf, %edx
// 0x1421b2d5e  41b817000000                   movl	$0x17, %r8d
// 0x1421b2d64  41b901000000                   movl	$0x1, %r9d
// 0x1421b2d6a  e8617f7200                     callq	0x1428dacd0 <.text+0x28d9cd0>
// Frames: F129 -> F130 -> F131 -> F335 -> F336 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2d6f  488b45b8                       movq	-0x48(%rbp), %rax
// Frames: F337 -> F338 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2d73  488b55c0                       movq	-0x40(%rbp), %rdx
// Frames: F339 -> F336 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2d77  0f1005b555e400                 movups	0xe455b5(%rip), %xmm0   # 0x142ff8333
// 0x1421b2d7e  0f110410                       movups	%xmm0, (%rax,%rdx)
// 0x1421b2d82  48b9203a3a205761726e           movabsq	$0x6e726157203a3a20, %rcx # imm = 0x6E726157203A3A20
// 0x1421b2d8c  48894c100f                     movq	%rcx, 0xf(%rax,%rdx)
// Frames: F340 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2d91  4883c217                       addq	$0x17, %rdx
// Frames: F341 -> F309
// 0x1421b2d95  48895550                       movq	%rdx, 0x50(%rbp)
// 0x1421b2d99  0f1045b0                       movups	-0x50(%rbp), %xmm0
// 0x1421b2d9d  0f294540                       movaps	%xmm0, 0x40(%rbp)
// Frames: F342 -> F343 -> F344 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2da1  488b4540                       movq	0x40(%rbp), %rax
// Frames: F345 -> F343 -> F344 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2da5  4829d0                         subq	%rdx, %rax
// Frames: F343 -> F344 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2da8  4883f819                       cmpq	$0x19, %rax
// Frames: F344 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2dac  0f8642010000                   jbe	0x1421b2ef4 <.text+0x21b1ef4>
// Frames: F129 -> F130 -> F131 -> F335 -> F336 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2db2  488b4548                       movq	0x48(%rbp), %rax
// Frames: F339 -> F336 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2db6  0f10059755e400                 movups	0xe45597(%rip), %xmm0   # 0x142ff8354
// 0x1421b2dbd  0f1144100a                     movups	%xmm0, 0xa(%rax,%rdx)
// 0x1421b2dc2  0f10058155e400                 movups	0xe45581(%rip), %xmm0   # 0x142ff834a
// 0x1421b2dc9  0f110410                       movups	%xmm0, (%rax,%rdx)
// Frames: F340 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2dcd  4883c21a                       addq	$0x1a, %rdx
// 0x1421b2dd1  48895550                       movq	%rdx, 0x50(%rbp)
// Frames: F341 -> F309
// 0x1421b2dd5  488955c0                       movq	%rdx, -0x40(%rbp)
// 0x1421b2dd9  0f284540                       movaps	0x40(%rbp), %xmm0
// 0x1421b2ddd  0f2945b0                       movaps	%xmm0, -0x50(%rbp)
// 0x1421b2de1  488b4538                       movq	0x38(%rbp), %rax
// Frames: F73 -> F346 -> F309
// 0x1421b2de5  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F347 -> F309
// 0x1421b2de9  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x1421b2ded  e80e8e85ff                     callq	0x141a0bc00 <.text+0x1a0ac00>
// Frames: F309
// 0x1421b2df2  84c0                           testb	%al, %al
// 0x1421b2df4  0f8596000000                   jne	0x1421b2e90 <.text+0x21b1e90>
// Frames: F196 -> F197 -> F348 -> F309
// 0x1421b2dfa  488b05a7954701                 movq	0x14795a7(%rip), %rax   # 0x14362c3a8
// 0x1421b2e01  488b00                         movq	(%rax), %rax
// Frames: F349 -> F309
// 0x1421b2e04  4883f802                       cmpq	$0x2, %rax
// Frames: F309
// 0x1421b2e08  0f8282000000                   jb	0x1421b2e90 <.text+0x21b1e90>
// 0x1421b2e0e  488d0df354e400                 leaq	0xe454f3(%rip), %rcx    # 0x142ff8308
// 0x1421b2e15  e866af4cfe                     callq	0x14067dd80 <.text+0x67cd80>
// Frames: F350 -> F351 -> F352 -> F309
// 0x1421b2e1a  0f1000                         movups	(%rax), %xmm0
// Frames: F353 -> F354 -> F352 -> F309
// 0x1421b2e1d  8b4010                         movl	0x10(%rax), %eax
// Frames: F355 -> F356 -> F357 -> F352 -> F309
// 0x1421b2e20  48c745e002000000               movq	$0x2, -0x20(%rbp)
// 0x1421b2e28  488d154955e400                 leaq	0xe45549(%rip), %rdx    # 0x142ff8378
// 0x1421b2e2f  488955e8                       movq	%rdx, -0x18(%rbp)
// 0x1421b2e33  48c745f028000000               movq	$0x28, -0x10(%rbp)
// 0x1421b2e3b  488d0d2655e400                 leaq	0xe45526(%rip), %rcx    # 0x142ff8368
// 0x1421b2e42  48894d00                       movq	%rcx, (%rbp)
// 0x1421b2e46  48c7450801000000               movq	$0x1, 0x8(%rbp)
// 0x1421b2e4e  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x1421b2e52  48894d10                       movq	%rcx, 0x10(%rbp)
// 0x1421b2e56  48c745b000000000               movq	$0x0, -0x50(%rbp)
// 0x1421b2e5e  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421b2e61  0f114d18                       movups	%xmm1, 0x18(%rbp)
// 0x1421b2e65  488955b8                       movq	%rdx, -0x48(%rbp)
// 0x1421b2e69  48c745c028000000               movq	$0x28, -0x40(%rbp)
// 0x1421b2e71  48c745c800000000               movq	$0x0, -0x38(%rbp)
// 0x1421b2e79  0f1145d0                       movups	%xmm0, -0x30(%rbp)
// 0x1421b2e7d  c745f801000000                 movl	$0x1, -0x8(%rbp)
// 0x1421b2e84  8945fc                         movl	%eax, -0x4(%rbp)
// Frames: F357 -> F352 -> F309
// 0x1421b2e87  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x1421b2e8b  e8a074b9fe                     callq	0x140d4a330 <.text+0xd49330>
// Frames: F358 -> F359 -> F77 -> F309
// 0x1421b2e90  807d5f00                       cmpb	$0x0, 0x5f(%rbp)
// 0x1421b2e94  488b7538                       movq	0x38(%rbp), %rsi
// 0x1421b2e98  7512                           jne	0x1421b2eac <.text+0x21b1eac>
// Frames: F196 -> F197 -> F360 -> F361 -> F362 -> F358 -> F359 -> F77 -> F309
// 0x1421b2e9a  488b057f954701                 movq	0x147957f(%rip), %rax   # 0x14362c420
// 0x1421b2ea1  488b00                         movq	(%rax), %rax
// Frames: F360 -> F361 -> F362 -> F358 -> F359 -> F77 -> F309
// 0x1421b2ea4  48d1e0                         shlq	%rax
// 0x1421b2ea7  4885c0                         testq	%rax, %rax
// 0x1421b2eaa  7579                           jne	0x1421b2f25 <.text+0x21b1f25>
// Frames: F363 -> F364 -> F365 -> F366 -> F77 -> F309
// 0x1421b2eac  31c0                           xorl	%eax, %eax
// 0x1421b2eae  8606                           xchgb	%al, (%rsi)
// Frames: F365 -> F366 -> F77 -> F309
// 0x1421b2eb0  3c02                           cmpb	$0x2, %al
// 0x1421b2eb2  7467                           je	0x1421b2f1b <.text+0x21b1f1b>
// Frames: F367
// 0x1421b2eb4  4881c4e8000000                 addq	$0xe8, %rsp
// 0x1421b2ebb  5e                             popq	%rsi
// 0x1421b2ebc  5d                             popq	%rbp
// 0x1421b2ebd  c3                             retq
// Frames: F368 -> F309
// 0x1421b2ebe  c6454001                       movb	$0x1, 0x40(%rbp)
// 0x1421b2ec2  488d4540                       leaq	0x40(%rbp), %rax
// Frames: F369 -> F309
// 0x1421b2ec6  488945b0                       movq	%rax, -0x50(%rbp)
// 0x1421b2eca  488d053754e400                 leaq	0xe45437(%rip), %rax    # 0x142ff8308
// 0x1421b2ed1  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421b2ed6  488d0ddb794801                 leaq	0x14879db(%rip), %rcx   # 0x14363a8b8
// 0x1421b2edd  4c8d0d0c50e400                 leaq	0xe4500c(%rip), %r9     # 0x142ff7ef0
// 0x1421b2ee4  4c8d45b0                       leaq	-0x50(%rbp), %r8
// 0x1421b2ee8  31d2                           xorl	%edx, %edx
// 0x1421b2eea  e831597200                     callq	0x1428d8820 <.text+0x28d7820>
// 0x1421b2eef  e9e1fdffff                     jmp	0x1421b2cd5 <.text+0x21b1cd5>
// Frames: F327 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2ef4  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421b2efd  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x1421b2f01  41b81a000000                   movl	$0x1a, %r8d
// 0x1421b2f07  41b901000000                   movl	$0x1, %r9d
// 0x1421b2f0d  e8be7d7200                     callq	0x1428dacd0 <.text+0x28d9cd0>
// Frames: F337 -> F338 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b2f12  488b5550                       movq	0x50(%rbp), %rdx
// 0x1421b2f16  e997feffff                     jmp	0x1421b2db2 <.text+0x21b1db2>
// Frames: F370 -> F366 -> F77 -> F309
// 0x1421b2f1b  4889f1                         movq	%rsi, %rcx
// 0x1421b2f1e  e8ed587200                     callq	0x1428d8810 <.text+0x28d7810>
// 0x1421b2f23  eb8f                           jmp	0x1421b2eb4 <.text+0x21b1eb4>
// Frames: F371 -> F361 -> F362 -> F358 -> F359 -> F77 -> F309
// 0x1421b2f25  e886577200                     callq	0x1428d86b0 <.text+0x28d76b0>
// Frames: F358 -> F359 -> F77 -> F309
// 0x1421b2f2a  84c0                           testb	%al, %al
// 0x1421b2f2c  0f857affffff                   jne	0x1421b2eac <.text+0x21b1eac>
// Frames: F372 -> F373 -> F374 -> F359 -> F77 -> F309
// 0x1421b2f32  c6460101                       movb	$0x1, 0x1(%rsi)
// 0x1421b2f36  e971ffffff                     jmp	0x1421b2eac <.text+0x21b1eac>
// Frames: F375 -> F309
// 0x1421b2f3b  488b45b8                       movq	-0x48(%rbp), %rax
// 0x1421b2f3f  0fb64dc0                       movzbl	-0x40(%rbp), %ecx
// 0x1421b2f43  48894540                       movq	%rax, 0x40(%rbp)
// 0x1421b2f47  884d48                         movb	%cl, 0x48(%rbp)
// 0x1421b2f4a  488d05b753e400                 leaq	0xe453b7(%rip), %rax    # 0x142ff8308
// 0x1421b2f51  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421b2f56  488d0dc353e400                 leaq	0xe453c3(%rip), %rcx    # 0x142ff8320
// 0x1421b2f5d  4c8d0d9450e400                 leaq	0xe45094(%rip), %r9     # 0x142ff7ff8
// 0x1421b2f64  4c8d4540                       leaq	0x40(%rbp), %r8
// 0x1421b2f68  ba13000000                     movl	$0x13, %edx
// 0x1421b2f6d  e8ee677200                     callq	0x1428d9760 <.text+0x28d8760>
// 0x1421b2f72  eb16                           jmp	0x1421b2f8a <.text+0x21b1f8a>
// Frames: F376 -> F314 -> F315 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F309
// 0x1421b2f74  4c8d05ed50e400                 leaq	0xe450ed(%rip), %r8     # 0x142ff8068
// 0x1421b2f7b  b901000000                     movl	$0x1, %ecx
// 0x1421b2f80  ba0f000000                     movl	$0xf, %edx
// 0x1421b2f85  e859607200                     callq	0x1428d8fe3 <.text+0x28d7fe3>
// 0x1421b2f8a  0f0b                           ud2
// 0x1421b2f8c  0f1f4000                       nopl	(%rax)
// 0x1421b2f90  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b2f95  55                             pushq	%rbp
// 0x1421b2f96  56                             pushq	%rsi
// 0x1421b2f97  4883ec28                       subq	$0x28, %rsp
// 0x1421b2f9b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421b2fa2  488b4d38                       movq	0x38(%rbp), %rcx
// 0x1421b2fa6  0fb6555f                       movzbl	0x5f(%rbp), %edx
// Frames: F309
// 0x1421b2faa  e85100e9fd                     callq	0x140043000 <.text+0x42000>
// Frames: F303
// 0x1421b2faf  90                             nop
// 0x1421b2fb0  4883c428                       addq	$0x28, %rsp
// 0x1421b2fb4  5e                             popq	%rsi
// 0x1421b2fb5  5d                             popq	%rbp
// 0x1421b2fb6  c3                             retq
// 0x1421b2fb7  660f1f840000000000             nopw	(%rax,%rax)
// 0x1421b2fc0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b2fc5  55                             pushq	%rbp
// 0x1421b2fc6  56                             pushq	%rsi
// 0x1421b2fc7  4883ec28                       subq	$0x28, %rsp
// 0x1421b2fcb  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b2fd2  488b55b0                       movq	-0x50(%rbp), %rdx
// Frames: F380 -> F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b2fd6  4885d2                         testq	%rdx, %rdx
// 0x1421b2fd9  740f                           je	0x1421b2fea <.text+0x21b1fea>
// Frames: F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b2fdb  488b4db8                       movq	-0x48(%rbp), %rcx
// Frames: F381 -> F382 -> F383 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b2fdf  41b801000000                   movl	$0x1, %r8d
// 0x1421b2fe5  e8360c46fe                     callq	0x140613c20 <.text+0x612c20>
// 0x1421b2fea  90                             nop
// 0x1421b2feb  4883c428                       addq	$0x28, %rsp
// 0x1421b2fef  5e                             popq	%rsi
// 0x1421b2ff0  5d                             popq	%rbp
// 0x1421b2ff1  c3                             retq
// 0x1421b2ff2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1421b3000  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b3005  55                             pushq	%rbp
// 0x1421b3006  56                             pushq	%rsi
// 0x1421b3007  4883ec28                       subq	$0x28, %rsp
// 0x1421b300b  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b3012  488b5540                       movq	0x40(%rbp), %rdx
// Frames: F380 -> F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b3016  4885d2                         testq	%rdx, %rdx
// 0x1421b3019  740f                           je	0x1421b302a <.text+0x21b202a>
// Frames: F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b301b  488b4d48                       movq	0x48(%rbp), %rcx
// Frames: F381 -> F382 -> F383 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b301f  41b801000000                   movl	$0x1, %r8d
// 0x1421b3025  e8f60b46fe                     callq	0x140613c20 <.text+0x612c20>
// 0x1421b302a  90                             nop
// 0x1421b302b  4883c428                       addq	$0x28, %rsp
// 0x1421b302f  5e                             popq	%rsi
// 0x1421b3030  5d                             popq	%rbp
// 0x1421b3031  c3                             retq
// 0x1421b3032  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1421b3040  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b3045  55                             pushq	%rbp
// 0x1421b3046  56                             pushq	%rsi
// 0x1421b3047  4883ec28                       subq	$0x28, %rsp
// 0x1421b304b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421b3052  488d4d40                       leaq	0x40(%rbp), %rcx
// Frames: F375 -> F309
// 0x1421b3056  e815f841fe                     callq	0x1405d2870 <.text+0x5d1870>
// Frames: F384 -> F309
// 0x1421b305b  90                             nop
// 0x1421b305c  4883c428                       addq	$0x28, %rsp
// 0x1421b3060  5e                             popq	%rsi
// 0x1421b3061  5d                             popq	%rbp
// 0x1421b3062  c3                             retq
//
// RANGE rva_21b4510_len_3c3: RVA 0x21b4510, 963 PDB bytes; 217 instructions
// Evidence: _assembly/roof-v1/B/pack-003/rva_21b4510_len_3c3.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3475:10392: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// Frames: F303
// 0x1421b4510  55                             pushq	%rbp
// 0x1421b4511  56                             pushq	%rsi
// 0x1421b4512  4881ece8000000                 subq	$0xe8, %rsp
// 0x1421b4519  488dac2480000000               leaq	0x80(%rsp), %rbp
// Frames: F304
// 0x1421b4521  48c74560feffffff               movq	$-0x2, 0x60(%rbp)
// 0x1421b4529  e8427b88ff                     callq	0x141a3c070 <.text+0x1a3b070>
// 0x1421b452e  84c0                           testb	%al, %al
// 0x1421b4530  0f84ee010000                   je	0x1421b4724 <.text+0x21b3724>
// Frames: F305 -> F306 -> F307 -> F308 -> F309
// 0x1421b4536  8b057c634801                   movl	0x148637c(%rip), %eax   # 0x14363a8b8
// Frames: F307 -> F308 -> F309
// 0x1421b453c  83f803                         cmpl	$0x3, %eax
// Frames: F308 -> F309
// 0x1421b453f  0f85e9010000                   jne	0x1421b472e <.text+0x21b372e>
// Frames: F309
// 0x1421b4545  488b1564634801                 movq	0x1486364(%rip), %rdx   # 0x14363a8b0
// 0x1421b454c  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x1421b4550  e83be338fe                     callq	0x140542890 <.text+0x541890>
// Frames: F310 -> F309
// 0x1421b4555  837db001                       cmpl	$0x1, -0x50(%rbp)
// 0x1421b4559  0f844c020000                   je	0x1421b47ab <.text+0x21b37ab>
// Frames: F311 -> F309
// 0x1421b455f  488b45b8                       movq	-0x48(%rbp), %rax
// 0x1421b4563  48894538                       movq	%rax, 0x38(%rbp)
// 0x1421b4567  0fb645c0                       movzbl	-0x40(%rbp), %eax
// 0x1421b456b  88455f                         movb	%al, 0x5f(%rbp)
// 0x1421b456e  0fb60594064801                 movzbl	0x1480694(%rip), %eax   # 0x143634c09
// Frames: F188 -> F182 -> F183 -> F312 -> F313 -> F314 -> F315 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F309
// 0x1421b4575  b90f000000                     movl	$0xf, %ecx
// 0x1421b457a  ba01000000                     movl	$0x1, %edx
// 0x1421b457f  e88cf645fe                     callq	0x140613c10 <.text+0x612c10>
// Frames: F323 -> F313 -> F314 -> F315 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F309
// 0x1421b4584  4885c0                         testq	%rax, %rax
// 0x1421b4587  0f8457020000                   je	0x1421b47e4 <.text+0x21b37e4>
// Frames: F251 -> F324 -> F325 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F309
// 0x1421b458d  48b97061746821202829           movabsq	$0x2928202168746170, %rcx # imm = 0x2928202168746170
// 0x1421b4597  48894807                       movq	%rcx, 0x7(%rax)
// 0x1421b459b  48b96d6f64756c655f70           movabsq	$0x705f656c75646f6d, %rcx # imm = 0x705F656C75646F6D
// 0x1421b45a5  488908                         movq	%rcx, (%rax)
// Frames: F326 -> F321 -> F322 -> F309
// 0x1421b45a8  48c745b00f000000               movq	$0xf, -0x50(%rbp)
// 0x1421b45b0  488945b8                       movq	%rax, -0x48(%rbp)
// 0x1421b45b4  48c745c00f000000               movq	$0xf, -0x40(%rbp)
// Frames: F327 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b45bc  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421b45c5  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x1421b45c9  ba0f000000                     movl	$0xf, %edx
// 0x1421b45ce  41b817000000                   movl	$0x17, %r8d
// 0x1421b45d4  41b901000000                   movl	$0x1, %r9d
// 0x1421b45da  e8f1667200                     callq	0x1428dacd0 <.text+0x28d9cd0>
// Frames: F129 -> F130 -> F131 -> F335 -> F336 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b45df  488b45b8                       movq	-0x48(%rbp), %rax
// Frames: F337 -> F338 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b45e3  488b55c0                       movq	-0x40(%rbp), %rdx
// Frames: F339 -> F336 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b45e7  0f10055d40e400                 movups	0xe4405d(%rip), %xmm0   # 0x142ff864b
// 0x1421b45ee  0f110410                       movups	%xmm0, (%rax,%rdx)
// 0x1421b45f2  48b9203a3a205761726e           movabsq	$0x6e726157203a3a20, %rcx # imm = 0x6E726157203A3A20
// 0x1421b45fc  48894c100f                     movq	%rcx, 0xf(%rax,%rdx)
// Frames: F340 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b4601  4883c217                       addq	$0x17, %rdx
// Frames: F341 -> F309
// 0x1421b4605  48895550                       movq	%rdx, 0x50(%rbp)
// 0x1421b4609  0f1045b0                       movups	-0x50(%rbp), %xmm0
// 0x1421b460d  0f294540                       movaps	%xmm0, 0x40(%rbp)
// Frames: F342 -> F343 -> F344 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b4611  488b4540                       movq	0x40(%rbp), %rax
// Frames: F345 -> F343 -> F344 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b4615  4829d0                         subq	%rdx, %rax
// Frames: F343 -> F344 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b4618  4883f819                       cmpq	$0x19, %rax
// Frames: F344 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b461c  0f8642010000                   jbe	0x1421b4764 <.text+0x21b3764>
// Frames: F129 -> F130 -> F131 -> F335 -> F336 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b4622  488b4548                       movq	0x48(%rbp), %rax
// Frames: F339 -> F336 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b4626  0f10053f40e400                 movups	0xe4403f(%rip), %xmm0   # 0x142ff866c
// 0x1421b462d  0f1144100a                     movups	%xmm0, 0xa(%rax,%rdx)
// 0x1421b4632  0f10052940e400                 movups	0xe44029(%rip), %xmm0   # 0x142ff8662
// 0x1421b4639  0f110410                       movups	%xmm0, (%rax,%rdx)
// Frames: F340 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b463d  4883c21a                       addq	$0x1a, %rdx
// 0x1421b4641  48895550                       movq	%rdx, 0x50(%rbp)
// Frames: F341 -> F309
// 0x1421b4645  488955c0                       movq	%rdx, -0x40(%rbp)
// 0x1421b4649  0f284540                       movaps	0x40(%rbp), %xmm0
// 0x1421b464d  0f2945b0                       movaps	%xmm0, -0x50(%rbp)
// 0x1421b4651  488b4538                       movq	0x38(%rbp), %rax
// Frames: F73 -> F346 -> F309
// 0x1421b4655  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F347 -> F309
// 0x1421b4659  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x1421b465d  e89e7585ff                     callq	0x141a0bc00 <.text+0x1a0ac00>
// Frames: F309
// 0x1421b4662  84c0                           testb	%al, %al
// 0x1421b4664  0f8596000000                   jne	0x1421b4700 <.text+0x21b3700>
// Frames: F196 -> F197 -> F348 -> F309
// 0x1421b466a  488b05377d4701                 movq	0x1477d37(%rip), %rax   # 0x14362c3a8
// 0x1421b4671  488b00                         movq	(%rax), %rax
// Frames: F349 -> F309
// 0x1421b4674  4883f802                       cmpq	$0x2, %rax
// Frames: F309
// 0x1421b4678  0f8282000000                   jb	0x1421b4700 <.text+0x21b3700>
// 0x1421b467e  488d0d9b3fe400                 leaq	0xe43f9b(%rip), %rcx    # 0x142ff8620
// 0x1421b4685  e8f6964cfe                     callq	0x14067dd80 <.text+0x67cd80>
// Frames: F350 -> F351 -> F352 -> F309
// 0x1421b468a  0f1000                         movups	(%rax), %xmm0
// Frames: F353 -> F354 -> F352 -> F309
// 0x1421b468d  8b4010                         movl	0x10(%rax), %eax
// Frames: F355 -> F356 -> F357 -> F352 -> F309
// 0x1421b4690  48c745e002000000               movq	$0x2, -0x20(%rbp)
// 0x1421b4698  488d15f13fe400                 leaq	0xe43ff1(%rip), %rdx    # 0x142ff8690
// 0x1421b469f  488955e8                       movq	%rdx, -0x18(%rbp)
// 0x1421b46a3  48c745f028000000               movq	$0x28, -0x10(%rbp)
// 0x1421b46ab  488d0dce3fe400                 leaq	0xe43fce(%rip), %rcx    # 0x142ff8680
// 0x1421b46b2  48894d00                       movq	%rcx, (%rbp)
// 0x1421b46b6  48c7450801000000               movq	$0x1, 0x8(%rbp)
// 0x1421b46be  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x1421b46c2  48894d10                       movq	%rcx, 0x10(%rbp)
// 0x1421b46c6  48c745b000000000               movq	$0x0, -0x50(%rbp)
// 0x1421b46ce  0f57c9                         xorps	%xmm1, %xmm1
// 0x1421b46d1  0f114d18                       movups	%xmm1, 0x18(%rbp)
// 0x1421b46d5  488955b8                       movq	%rdx, -0x48(%rbp)
// 0x1421b46d9  48c745c028000000               movq	$0x28, -0x40(%rbp)
// 0x1421b46e1  48c745c800000000               movq	$0x0, -0x38(%rbp)
// 0x1421b46e9  0f1145d0                       movups	%xmm0, -0x30(%rbp)
// 0x1421b46ed  c745f801000000                 movl	$0x1, -0x8(%rbp)
// 0x1421b46f4  8945fc                         movl	%eax, -0x4(%rbp)
// Frames: F357 -> F352 -> F309
// 0x1421b46f7  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x1421b46fb  e8305cb9fe                     callq	0x140d4a330 <.text+0xd49330>
// Frames: F358 -> F359 -> F77 -> F309
// 0x1421b4700  807d5f00                       cmpb	$0x0, 0x5f(%rbp)
// 0x1421b4704  488b7538                       movq	0x38(%rbp), %rsi
// 0x1421b4708  7512                           jne	0x1421b471c <.text+0x21b371c>
// Frames: F196 -> F197 -> F360 -> F361 -> F362 -> F358 -> F359 -> F77 -> F309
// 0x1421b470a  488b050f7d4701                 movq	0x1477d0f(%rip), %rax   # 0x14362c420
// 0x1421b4711  488b00                         movq	(%rax), %rax
// Frames: F360 -> F361 -> F362 -> F358 -> F359 -> F77 -> F309
// 0x1421b4714  48d1e0                         shlq	%rax
// 0x1421b4717  4885c0                         testq	%rax, %rax
// 0x1421b471a  7579                           jne	0x1421b4795 <.text+0x21b3795>
// Frames: F363 -> F364 -> F365 -> F366 -> F77 -> F309
// 0x1421b471c  31c0                           xorl	%eax, %eax
// 0x1421b471e  8606                           xchgb	%al, (%rsi)
// Frames: F365 -> F366 -> F77 -> F309
// 0x1421b4720  3c02                           cmpb	$0x2, %al
// 0x1421b4722  7467                           je	0x1421b478b <.text+0x21b378b>
// Frames: F367
// 0x1421b4724  4881c4e8000000                 addq	$0xe8, %rsp
// 0x1421b472b  5e                             popq	%rsi
// 0x1421b472c  5d                             popq	%rbp
// 0x1421b472d  c3                             retq
// Frames: F368 -> F309
// 0x1421b472e  c6454001                       movb	$0x1, 0x40(%rbp)
// 0x1421b4732  488d4540                       leaq	0x40(%rbp), %rax
// Frames: F369 -> F309
// 0x1421b4736  488945b0                       movq	%rax, -0x50(%rbp)
// 0x1421b473a  488d05df3ee400                 leaq	0xe43edf(%rip), %rax    # 0x142ff8620
// 0x1421b4741  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421b4746  488d0d6b614801                 leaq	0x148616b(%rip), %rcx   # 0x14363a8b8
// 0x1421b474d  4c8d0d543ce400                 leaq	0xe43c54(%rip), %r9     # 0x142ff83a8
// 0x1421b4754  4c8d45b0                       leaq	-0x50(%rbp), %r8
// 0x1421b4758  31d2                           xorl	%edx, %edx
// 0x1421b475a  e8c1407200                     callq	0x1428d8820 <.text+0x28d7820>
// 0x1421b475f  e9e1fdffff                     jmp	0x1421b4545 <.text+0x21b3545>
// Frames: F327 -> F328 -> F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b4764  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1421b476d  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x1421b4771  41b81a000000                   movl	$0x1a, %r8d
// 0x1421b4777  41b901000000                   movl	$0x1, %r9d
// 0x1421b477d  e84e657200                     callq	0x1428dacd0 <.text+0x28d9cd0>
// Frames: F337 -> F338 -> F331 -> F332 -> F333 -> F334 -> F309
// 0x1421b4782  488b5550                       movq	0x50(%rbp), %rdx
// 0x1421b4786  e997feffff                     jmp	0x1421b4622 <.text+0x21b3622>
// Frames: F370 -> F366 -> F77 -> F309
// 0x1421b478b  4889f1                         movq	%rsi, %rcx
// 0x1421b478e  e87d407200                     callq	0x1428d8810 <.text+0x28d7810>
// 0x1421b4793  eb8f                           jmp	0x1421b4724 <.text+0x21b3724>
// Frames: F371 -> F361 -> F362 -> F358 -> F359 -> F77 -> F309
// 0x1421b4795  e8163f7200                     callq	0x1428d86b0 <.text+0x28d76b0>
// Frames: F358 -> F359 -> F77 -> F309
// 0x1421b479a  84c0                           testb	%al, %al
// 0x1421b479c  0f857affffff                   jne	0x1421b471c <.text+0x21b371c>
// Frames: F372 -> F373 -> F374 -> F359 -> F77 -> F309
// 0x1421b47a2  c6460101                       movb	$0x1, 0x1(%rsi)
// 0x1421b47a6  e971ffffff                     jmp	0x1421b471c <.text+0x21b371c>
// Frames: F375 -> F309
// 0x1421b47ab  488b45b8                       movq	-0x48(%rbp), %rax
// 0x1421b47af  0fb64dc0                       movzbl	-0x40(%rbp), %ecx
// 0x1421b47b3  48894540                       movq	%rax, 0x40(%rbp)
// 0x1421b47b7  884d48                         movb	%cl, 0x48(%rbp)
// 0x1421b47ba  488d055f3ee400                 leaq	0xe43e5f(%rip), %rax    # 0x142ff8620
// 0x1421b47c1  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1421b47c6  488d0d6b3ee400                 leaq	0xe43e6b(%rip), %rcx    # 0x142ff8638
// 0x1421b47cd  4c8d0d6c3ce400                 leaq	0xe43c6c(%rip), %r9     # 0x142ff8440
// 0x1421b47d4  4c8d4540                       leaq	0x40(%rbp), %r8
// 0x1421b47d8  ba13000000                     movl	$0x13, %edx
// 0x1421b47dd  e87e4f7200                     callq	0x1428d9760 <.text+0x28d8760>
// 0x1421b47e2  eb16                           jmp	0x1421b47fa <.text+0x21b37fa>
// Frames: F376 -> F314 -> F315 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F309
// 0x1421b47e4  4c8d05c53ce400                 leaq	0xe43cc5(%rip), %r8     # 0x142ff84b0
// 0x1421b47eb  b901000000                     movl	$0x1, %ecx
// 0x1421b47f0  ba0f000000                     movl	$0xf, %edx
// 0x1421b47f5  e8e9477200                     callq	0x1428d8fe3 <.text+0x28d7fe3>
// 0x1421b47fa  0f0b                           ud2
// 0x1421b47fc  0f1f4000                       nopl	(%rax)
// 0x1421b4800  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b4805  55                             pushq	%rbp
// 0x1421b4806  56                             pushq	%rsi
// 0x1421b4807  4883ec28                       subq	$0x28, %rsp
// 0x1421b480b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421b4812  488b4d38                       movq	0x38(%rbp), %rcx
// 0x1421b4816  0fb6555f                       movzbl	0x5f(%rbp), %edx
// Frames: F309
// 0x1421b481a  e8e1e7e8fd                     callq	0x140043000 <.text+0x42000>
// Frames: F303
// 0x1421b481f  90                             nop
// 0x1421b4820  4883c428                       addq	$0x28, %rsp
// 0x1421b4824  5e                             popq	%rsi
// 0x1421b4825  5d                             popq	%rbp
// 0x1421b4826  c3                             retq
// 0x1421b4827  660f1f840000000000             nopw	(%rax,%rax)
// 0x1421b4830  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b4835  55                             pushq	%rbp
// 0x1421b4836  56                             pushq	%rsi
// 0x1421b4837  4883ec28                       subq	$0x28, %rsp
// 0x1421b483b  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b4842  488b55b0                       movq	-0x50(%rbp), %rdx
// Frames: F380 -> F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b4846  4885d2                         testq	%rdx, %rdx
// 0x1421b4849  740f                           je	0x1421b485a <.text+0x21b385a>
// Frames: F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b484b  488b4db8                       movq	-0x48(%rbp), %rcx
// Frames: F381 -> F382 -> F383 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b484f  41b801000000                   movl	$0x1, %r8d
// 0x1421b4855  e8c6f345fe                     callq	0x140613c20 <.text+0x612c20>
// 0x1421b485a  90                             nop
// 0x1421b485b  4883c428                       addq	$0x28, %rsp
// 0x1421b485f  5e                             popq	%rsi
// 0x1421b4860  5d                             popq	%rbp
// 0x1421b4861  c3                             retq
// 0x1421b4862  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1421b4870  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b4875  55                             pushq	%rbp
// 0x1421b4876  56                             pushq	%rsi
// 0x1421b4877  4883ec28                       subq	$0x28, %rsp
// 0x1421b487b  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b4882  488b5540                       movq	0x40(%rbp), %rdx
// Frames: F380 -> F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b4886  4885d2                         testq	%rdx, %rdx
// 0x1421b4889  740f                           je	0x1421b489a <.text+0x21b389a>
// Frames: F377 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b488b  488b4d48                       movq	0x48(%rbp), %rcx
// Frames: F381 -> F382 -> F383 -> F378 -> F77 -> F77 -> F77 -> F379 -> F309
// 0x1421b488f  41b801000000                   movl	$0x1, %r8d
// 0x1421b4895  e886f345fe                     callq	0x140613c20 <.text+0x612c20>
// 0x1421b489a  90                             nop
// 0x1421b489b  4883c428                       addq	$0x28, %rsp
// 0x1421b489f  5e                             popq	%rsi
// 0x1421b48a0  5d                             popq	%rbp
// 0x1421b48a1  c3                             retq
// 0x1421b48a2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x1421b48b0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1421b48b5  55                             pushq	%rbp
// 0x1421b48b6  56                             pushq	%rsi
// 0x1421b48b7  4883ec28                       subq	$0x28, %rsp
// 0x1421b48bb  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1421b48c2  488d4d40                       leaq	0x40(%rbp), %rcx
// Frames: F375 -> F309
// 0x1421b48c6  e8a5df41fe                     callq	0x1405d2870 <.text+0x5d1870>
// Frames: F384 -> F309
// 0x1421b48cb  90                             nop
// 0x1421b48cc  4883c428                       addq	$0x28, %rsp
// 0x1421b48d0  5e                             popq	%rsi
// 0x1421b48d1  5d                             popq	%rbp
// 0x1421b48d2  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/assemble_roof_tiles.rs:10:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F2: crates/systems/roof/src/visual/assemble_roof_tiles.rs:19:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F4: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:371:0 core::str::impl$0::is_char_boundary
// F5: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:434:0 core::str::traits::impl$10::get
// F6: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:465:0 core::str::traits::impl$10::index
// F7: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:60:0 core::str::traits::impl$4::index
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:375:0 core::str::impl$0::is_char_boundary
// F10: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:387:0 core::str::impl$0::is_char_boundary
// F11: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/mod.rs:1046:0 core::num::impl$6::is_utf8_char_boundary
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:467:0 core::str::traits::impl$10::index
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:502:0 core::str::traits::impl$11::get
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:535:0 core::str::traits::impl$11::index
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F16: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:537:0 core::str::traits::impl$11::index
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:211:0 core::str::traits::impl$7::get_unchecked
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:524:0 core::str::traits::impl$11::get_unchecked
// F19: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:505:0 core::str::traits::impl$11::get
// F20: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:967:0 core::ptr::const_ptr::impl$0::add
// F21: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:212:0 core::str::traits::impl$7::get_unchecked
// F22: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:85:0 core::str::validations::next_code_point_reverse
// F23: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/iter.rs:131:0 core::str::iter::impl$2::next_back
// F24: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/iter.rs:219:0 core::str::iter::impl$6::next_back
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:714:0 core::str::pattern::impl$11::next_back
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:317:0 core::str::pattern::ReverseSearcher::next_match_back
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:799:0 core::str::pattern::impl$22::next_match_back
// F28: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/mod.rs:1492:0 core::str::impl$0::rfind
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:285:0 core::slice::iter::impl$182::any
// F31: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:660:0 core::str::pattern::impl$8::matches
// F32: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:717:0 core::str::pattern::impl$11::next_back
// F33: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:94:0 core::str::validations::next_code_point_reverse
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:25:0 core::str::validations::utf8_is_cont_byte
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:96:0 core::str::validations::next_code_point_reverse
// F36: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:99:0 core::str::validations::next_code_point_reverse
// F37: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:101:0 core::str::validations::next_code_point_reverse
// F38: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:541:0 core::ptr::non_null::NonNull::offset
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:697:0 core::ptr::non_null::NonNull::sub
// F40: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:131:0 core::slice::iter::Iter::pre_dec_end
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:83:0 core::slice::iter::Iter::next_back_unchecked
// F42: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:401:0 core::slice::iter::impl$183::next_back
// F43: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:12:0 core::str::validations::utf8_first_byte
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F46: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:18:0 core::str::validations::utf8_acc_cont_byte
// F47: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:106:0 core::str::validations::next_code_point_reverse
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:108:0 core::str::validations::next_code_point_reverse
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:110:0 core::str::validations::next_code_point_reverse
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:1623:0 core::ptr::non_null::impl$14::eq
// F51: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$183::next_back
// F52: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F53: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$183::next_back
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F55: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/mod.rs:94:0 puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F57: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:309:0 std::thread::local::LocalKey::try_with
// F58: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:274:0 std::thread::local::LocalKey::with
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F61: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:53:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F62: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:54:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F63: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1822:0 core::cell::BorrowRefMut::new
// F64: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1122:0 core::cell::RefCell::try_borrow_mut
// F65: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1090:0 core::cell::RefCell::borrow_mut
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F67: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/local.rs:310:0 std::thread::local::LocalKey::try_with
// F68: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F69: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:870:0 core::mem::replace
// F70: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:503:0 core::cell::Cell::replace
// F71: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:429:0 core::cell::Cell::set
// F72: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1824:0 core::cell::BorrowRefMut::new
// F73: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:2198:0 core::cell::UnsafeCell::get
// F74: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1130:0 core::cell::RefCell::try_borrow_mut
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F76: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cell.rs:1811:0 core::cell::impl$44::drop
// F77: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:523:0 core::ptr::drop_in_place
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1235:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs:28:0 bevy_ecs::event::reader::EventReader::read
// F80: crates/systems/roof/src/visual/assemble_roof_tiles.rs:21:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:55:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:89:0 bevy_ecs::event::event_cursor::EventCursor::read_with_id
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:79:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F85: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:1860:0 core::num::impl$11::saturating_sub
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:57:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:60:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F88: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:659:0 core::num::impl$11::checked_sub
// F89: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:366:0 core::slice::index::impl$4::get
// F90: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:538:0 core::slice::index::impl$7::get
// F91: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:61:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F93: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F94: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:62:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:67:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F97: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F98: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F99: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 core::slice::impl$0::iter
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:69:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F101: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:729:0 core::option::Option::as_mut
// F102: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:333:0 core::iter::adapters::chain::and_then_or_clear
// F103: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:89:0 bevy_ecs::event::iterators::impl$3::next
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:15:0 bevy_ecs::event::iterators::impl$0::next
// F106: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next::closure$0
// F107: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1552:0 core::option::Option::or_else
// F108: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:617:0 core::ptr::non_null::NonNull::add
// F109: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:108:0 core::slice::iter::Iter::post_inc_start
// F110: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:443:0 core::slice::iter::impl$186::next_unchecked
// F111: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:165:0 core::slice::iter::impl$182::next
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:95:0 bevy_ecs::event::iterators::impl$3::next
// F113: crates/country-core/src/asset_libraries/ssbo_library.rs:209:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F114: crates/country-core/src/asset_libraries/ssbo_library.rs:245:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
// F115: crates/systems/roof/src/visual/assemble_roof_tiles.rs:22:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F116: crates/country-core/src/asset_libraries/ssbo_library.rs:210:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F117: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/cmp.rs:83:0 core::slice::cmp::impl$5::equal
// F118: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/cmp.rs:15:0 core::slice::cmp::impl$0::eq
// F119: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1922:0 core::cmp::impls::impl$9::eq
// F120: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/traits.rs:29:0 core::str::traits::impl$1::eq
// F121: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/cmp.rs:91:0 core::slice::cmp::impl$5::equal
// F122: crates/country-core/src/asset_libraries/ssbo_library.rs:211:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F123: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:976:0 core::option::Option::unwrap
// F124: crates/systems/roof/src/visual/assemble_roof_tiles.rs:24:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:843:0 bevy_ecs::entity::Entities::get
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1595:0 bevy_ecs::system::query::Query::get_inner
// F127: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1577:0 bevy_ecs::system::query::Query::get_mut
// F128: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F129: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F130: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F131: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F132: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F133: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 core::slice::impl$0::get
// F134: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F135: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1742:0 core::cmp::impls::impl$24::ne
// F136: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:201:0 core::num::nonzero::impl$2::ne
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:844:0 bevy_ecs::entity::Entities::get
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:76:0 bevy_ecs::archetype::impl$25::eq
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs:845:0 bevy_ecs::entity::Entities::get
// F140: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F141: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1603:0 bevy_ecs::system::query::Query::get_inner
// F142: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/bool.rs:62:0 core::bool::impl$0::then
// F143: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F146: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F147: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/internal_macros.rs:36:0 fixedbitset::FixedBitSet::get_unchecked
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1600:0 bevy_ecs::system::query::Query::get_inner
// F149: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1632:0 bevy_ecs::system::query::Query::get_inner
// F151: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 alloc::vec::impl$8::deref
// F152: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1634:0 bevy_ecs::system::query::Query::get_inner
// F153: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F154: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1544:0 bevy_ecs::query::fetch::impl$36::set_table
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1533:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F157: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F158: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F159: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:367:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F160: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1181:0 bevy_ecs::query::fetch::impl$26::set_table
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1169:0 bevy_ecs::query::fetch::impl$26::set_archetype
// F162: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F163: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::slice::impl$0::get
// F164: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F165: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F166: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F168: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:304:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F169: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F170: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F171: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F172: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1608:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F173: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F174: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1601:0 bevy_ecs::query::fetch::impl$37::fetch
// F175: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2090:0 bevy_ecs::query::fetch::impl$61::fetch
// F176: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1638:0 bevy_ecs::system::query::Query::get_inner
// F177: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:608:0 core::option::Option::is_some
// F178: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:652:0 core::option::Option::is_none
// F179: crates/systems/roof/src/visual/assemble_roof_tiles.rs:26:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F180: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1752:0 core::ptr::read_volatile
// F181: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:94:0 alloc::alloc::alloc
// F182: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:192:0 alloc::alloc::Global::alloc_impl
// F183: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:254:0 alloc::alloc::impl$1::allocate
// F184: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:349:0 alloc::alloc::exchange_malloc
// F185: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/boxed.rs:274:0 alloc::boxed::impl$0::new
// F186: crates/country-core/src/render/sparse_instance_buffer.rs:121:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
// F187: crates/systems/roof/src/visual/assemble_roof_tiles.rs:27:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F188: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:96:0 alloc::alloc::alloc
// F189: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:429:0 core::ptr::non_null::NonNull::as_ref
// F190: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:1894:0 alloc::sync::Arc::inner
// F191: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:1715:0 alloc::sync::Arc::downgrade
// F192: crates/country-core/src/render/sparse_instance_buffer.rs:129:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
// F193: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:25:0 core::core_arch::x86::sse2::_mm_pause
// F194: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/hint.rs:278:0 core::hint::spin_loop
// F195: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:1720:0 alloc::sync::Arc::downgrade
// F196: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3731:0 core::sync::atomic::atomic_load
// F197: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2700:0 core::sync::atomic::AtomicUsize::load
// F198: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:0:0 alloc::sync::Arc::downgrade
// F199: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:1719:0 alloc::sync::Arc::downgrade
// F200: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:1726:0 alloc::sync::Arc::downgrade
// F201: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:1735:0 alloc::sync::Arc::downgrade
// F202: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3841:0 core::sync::atomic::atomic_compare_exchange_weak
// F203: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2927:0 core::sync::atomic::AtomicUsize::compare_exchange_weak
// F204: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F205: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F206: crates/systems/roof/src/visual/assemble_roof_tiles.rs:30:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F207: crates/country-core/src/render/sparse_instance_buffer.rs:166:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F208: crates/systems/roof/src/visual/assemble_roof_tiles.rs:31:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F209: crates/country-core/src/render/sparse_instance_buffer.rs:167:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F210: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2927:0 core::sync::atomic::AtomicU8::compare_exchange_weak
// F211: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs:69:0 parking_lot::raw_mutex::impl$0::lock
// F212: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs:67:0 parking_lot::raw_mutex::impl$0::lock
// F213: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs:1106:0 parking_lot_core::parking_lot::deadlock::acquire_resource
// F214: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs:74:0 parking_lot::raw_mutex::impl$0::lock
// F215: crates/country-core/src/render/sparse_instance_buffer.rs:170:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F216: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:280:0 core::slice::impl$0::last
// F217: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1898:0 core::option::Option::copied
// F218: crates/country-core/src/render/sparse_instance_buffer.rs:175:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F219: crates/country-core/src/render/sparse_instance_buffer.rs:175:0 country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
// F220: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1023:0 core::option::Option::unwrap_or_else
// F221: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1899:0 core::option::Option::copied
// F222: crates/country-core/src/render/sparse_instance_buffer.rs:176:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F223: crates/country-core/src/render/sparse_instance_buffer.rs:177:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F224: crates/country-core/src/render/sparse_instance_buffer.rs:179:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F225: crates/country-core/src/render/sparse_instance_buffer.rs:181:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F226: crates/country-core/src/render/sparse_instance_buffer.rs:169:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F227: crates/systems/roof/src/visual/assemble_roof_tiles.rs:32:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F228: crates/systems/roof/src/visual/assemble_roof_tiles.rs:34:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F229: crates/systems/roof/src/visual/assemble_roof_tiles.rs:62:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F230: crates/country-core/src/render/sparse_instance_buffer.rs:446:0 country_core::render::sparse_instance_buffer::impl$5::drop
// F231: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs:1122:0 parking_lot_core::parking_lot::deadlock::release_resource
// F232: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs:101:0 parking_lot::raw_mutex::impl$0::unlock
// F233: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3809:0 core::sync::atomic::atomic_compare_exchange
// F234: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2877:0 core::sync::atomic::AtomicU8::compare_exchange
// F235: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs:104:0 parking_lot::raw_mutex::impl$0::unlock
// F236: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs:102:0 parking_lot::raw_mutex::impl$0::unlock
// F237: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs:109:0 parking_lot::raw_mutex::impl$0::unlock
// F238: /rust/deps/hashbrown-0.15.2/src/map.rs:1351:0 hashbrown::map::HashMap::get_inner
// F239: /rust/deps/hashbrown-0.15.2/src/map.rs:1309:0 hashbrown::map::HashMap::get
// F240: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/collections/hash/map.rs:899:0 std::collections::hash::map::HashMap::get
// F241: crates/systems/roof/src/visual/assemble_roof_tiles.rs:45:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F242: /rust/deps/hashbrown-0.15.2/src/map.rs:258:0 hashbrown::map::make_hash
// F243: /rust/deps/hashbrown-0.15.2/src/map.rs:1354:0 hashbrown::map::HashMap::get_inner
// F244: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:2305:0 hashbrown::raw::RawTableInner::probe_seq
// F245: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1864:0 hashbrown::raw::RawTableInner::find_inner
// F246: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1164:0 hashbrown::raw::RawTable::find
// F247: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1181:0 hashbrown::raw::RawTable::get
// F248: /rust/deps/hashbrown-0.15.2/src/map.rs:1355:0 hashbrown::map::HashMap::get_inner
// F249: /rust/deps/hashbrown-0.15.2/src/control/tag.rs:48:0 hashbrown::control::tag::Tag::full
// F250: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1863:0 hashbrown::raw::RawTableInner::find_inner
// F251: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:4441:0 core::intrinsics::copy_nonoverlapping
// F252: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1229:0 core::core_arch::x86::sse2::_mm_loadu_si128
// F253: /rust/deps/hashbrown-0.15.2/src/control/group/sse2.rs:52:0 hashbrown::control::group::sse2::Group::load
// F254: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1881:0 hashbrown::raw::RawTableInner::find_inner
// F255: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1436:0 core::core_arch::x86::sse2::_mm_movemask_epi8
// F256: /rust/deps/hashbrown-0.15.2/src/control/group/sse2.rs:87:0 hashbrown::control::group::sse2::Group::match_tag
// F257: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1883:0 hashbrown::raw::RawTableInner::find_inner
// F258: /rust/deps/hashbrown-0.15.2/src/control/bitmask.rs:50:0 hashbrown::control::bitmask::BitMask::lowest_set_bit
// F259: /rust/deps/hashbrown-0.15.2/src/control/bitmask.rs:113:0 hashbrown::control::bitmask::impl$2::next
// F260: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:604:0 core::num::nonzero::NonZero::trailing_zeros
// F261: /rust/deps/hashbrown-0.15.2/src/control/bitmask.rs:80:0 hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
// F262: /rust/deps/hashbrown-0.15.2/src/control/bitmask.rs:51:0 hashbrown::control::bitmask::BitMask::lowest_set_bit
// F263: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1886:0 hashbrown::raw::RawTableInner::find_inner
// F264: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1168:0 core::ptr::mut_ptr::impl$0::sub
// F265: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:390:0 hashbrown::raw::Bucket::as_ptr
// F266: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:523:0 hashbrown::raw::Bucket::as_ref
// F267: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1166:0 hashbrown::raw::impl$8::find::closure$0
// F268: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1888:0 hashbrown::raw::RawTableInner::find_inner
// F269: crates/country-core/src/resources/walls/mod.rs:71:0 country_core::resources::walls::impl$9::eq
// F270: /rust/deps/hashbrown-0.15.2/src/lib.rs:165:0 hashbrown::impl$0::equivalent
// F271: /rust/deps/hashbrown-0.15.2/src/map.rs:224:0 hashbrown::map::equivalent_key::closure$0
// F272: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:1498:0 core::intrinsics::likely
// F273: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/nonzero.rs:468:0 core::num::nonzero::NonZero::get
// F274: /rust/deps/hashbrown-0.15.2/src/control/group/sse2.rs:95:0 hashbrown::control::group::sse2::Group::match_empty
// F275: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1893:0 hashbrown::raw::RawTableInner::find_inner
// F276: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:1498:0 hashbrown::control::group::sse2::Group::match_empty
// F277: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:89:0 hashbrown::raw::ProbeSeq::move_next
// F278: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1897:0 hashbrown::raw::RawTableInner::find_inner
// F279: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:88:0 hashbrown::raw::ProbeSeq::move_next
// F280: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:90:0 hashbrown::raw::ProbeSeq::move_next
// F281: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1866:0 hashbrown::raw::RawTableInner::find_inner
// F282: crates/systems/roof/src/visual/assemble_roof_tiles.rs:48:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F283: crates/systems/roof/src/visual/assemble_roof_tiles.rs:52:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F284: crates/systems/roof/src/visual/assemble_roof_tiles.rs:53:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F285: /rust/deps/hashbrown-0.15.2/src/raw/mod.rs:1169:0 hashbrown::raw::RawTable::find
// F286: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2706:0 alloc::vec::Vec::is_empty
// F287: crates/systems/roof/src/visual/assemble_roof_tiles.rs:45:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// F288: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:631:0 core::option::Option::is_some_and
// F289: crates/systems/roof/src/visual/assemble_roof_tiles.rs:49:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F290: crates/systems/roof/src/visual/assemble_roof_tiles.rs:50:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F291: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs:72:0 parking_lot::raw_mutex::impl$0::lock
// F292: crates/systems/roof/src/visual/assemble_roof_tiles.rs:63:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::h5f8b90cca464129c
// F293: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop
// F294: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop::closure$0
// F295: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/thread_local/native/lazy.rs:56:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F296: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:299:0 core::ptr::const_ptr::impl$0::as_ref
// F297: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:978:0 core::option::Option::unwrap
// F298: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:1605:0 bevy_ecs::system::query::Query::get_inner
// F299: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:2003:0 core::result::impl$26::branch
// F300: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:2014:0 core::result::impl$27::from_residual
// F301: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1109:0 core::result::Result::unwrap
// F302: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:351:0 alloc::alloc::exchange_malloc
// F303: crates/systems/roof/src/visual/assemble_roof_tiles.rs:38:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// F304: crates/systems/roof/src/visual/assemble_roof_tiles.rs:39:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// F305: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3732:0 core::sync::atomic::atomic_load
// F306: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2700:0 core::sync::atomic::AtomicU32::load
// F307: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/sync/once/futex.rs:80:0 std::sys::sync::once::futex::Once::is_completed
// F308: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison/once.rs:150:0 std::sync::poison::once::Once::call_once
// F309: crates/systems/roof/src/visual/assemble_roof_tiles.rs:40:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// F310: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1059:0 core::result::Result::expect
// F311: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1060:0 core::result::Result::expect
// F312: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:472:0 alloc::raw_vec::RawVecInner::try_allocate_in
// F313: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:418:0 alloc::raw_vec::RawVecInner::with_capacity_in
// F314: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:186:0 alloc::raw_vec::RawVec::with_capacity_in
// F315: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:803:0 alloc::vec::Vec::with_capacity_in
// F316: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:162:0 alloc::slice::hack::impl$1::to_vec
// F317: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:111:0 alloc::slice::hack::to_vec
// F318: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:505:0 alloc::slice::impl$0::to_vec_in
// F319: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:480:0 alloc::slice::impl$0::to_vec
// F320: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:888:0 alloc::slice::impl$9::to_owned
// F321: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/str.rs:211:0 alloc::str::impl$4::to_owned
// F322: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/string.rs:2957:0 alloc::string::impl$47::from
// F323: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:476:0 alloc::raw_vec::RawVecInner::try_allocate_in
// F324: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/const_ptr.rs:1356:0 core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
// F325: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/slice.rs:167:0 alloc::slice::hack::impl$1::to_vec
// F326: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/string.rs:1044:0 alloc::slice::impl$9::to_owned
// F327: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:559:0 alloc::raw_vec::RawVecInner::reserve
// F328: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:327:0 alloc::raw_vec::RawVec::reserve
// F329: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1272:0 alloc::vec::Vec::reserve
// F330: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2567:0 alloc::vec::Vec::append_elements
// F331: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/spec_extend.rs:61:0 alloc::vec::spec_extend::impl$4::spec_extend
// F332: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3034:0 alloc::vec::Vec::extend_from_slice
// F333: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/string.rs:1127:0 alloc::string::String::push_str
// F334: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/string.rs:2643:0 alloc::string::impl$26::add
// F335: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1734:0 alloc::vec::Vec::as_mut_ptr
// F336: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2569:0 alloc::vec::Vec::append_elements
// F337: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2681:0 alloc::vec::Vec::len
// F338: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2568:0 alloc::vec::Vec::append_elements
// F339: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:4441:0 alloc::vec::Vec::as_mut_ptr
// F340: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2570:0 alloc::vec::Vec::append_elements
// F341: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/string.rs:2644:0 alloc::string::impl$26::add
// F342: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:513:0 alloc::raw_vec::RawVecInner::capacity
// F343: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:623:0 alloc::raw_vec::RawVecInner::needs_to_grow
// F344: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:558:0 alloc::raw_vec::RawVecInner::reserve
// F345: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:2022:0 alloc::raw_vec::RawVecInner::capacity
// F346: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison/mutex.rs:664:0 std::sync::poison::mutex::impl$13::deref_mut
// F347: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/collections/btree/set.rs:909:0 alloc::collections::btree::set::BTreeSet::insert
// F348: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:1328:0 log::max_level
// F349: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/macros/mod.rs:538:0 core::cmp::PartialOrd::le
// F350: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panic/location.rs:130:0 core::panic::location::Location::file
// F351: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:75:0 log::__private_api::log_impl
// F352: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:94:0 log::__private_api::log
// F353: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panic/location.rs:155:0 core::panic::location::Location::line
// F354: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:76:0 log::__private_api::log_impl
// F355: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:766:0 log::impl$52::clone
// F356: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:1034:0 log::RecordBuilder::build
// F357: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:81:0 log::__private_api::log_impl
// F358: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison.rs:133:0 std::sync::poison::Flag::done
// F359: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison/mutex.rs:673:0 std::sync::poison::mutex::impl$14::drop
// F360: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/panicking.rs:470:0 std::panicking::panic_count::count_is_zero
// F361: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/panicking.rs:622:0 std::panicking::panicking
// F362: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/thread/mod.rs:815:0 std::thread::panicking
// F363: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3749:0 core::sync::atomic::atomic_swap
// F364: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2755:0 core::sync::atomic::AtomicU8::swap
// F365: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/sync/mutex/futex.rs:84:0 std::sys::sync::mutex::futex::Mutex::unlock
// F366: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison/mutex.rs:674:0 std::sync::poison::mutex::impl$14::drop
// F367: crates/systems/roof/src/visual/assemble_roof_tiles.rs:42:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// F368: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison/once.rs:154:0 std::sync::poison::once::Once::call_once
// F369: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison/once.rs:155:0 std::sync::poison::once::Once::call_once
// F370: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sys/sync/mutex/futex.rs:89:0 std::sys::sync::mutex::futex::Mutex::unlock
// F371: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/panicking.rs:482:0 std::panicking::panic_count::count_is_zero
// F372: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3716:0 core::sync::atomic::atomic_store
// F373: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:655:0 core::sync::atomic::AtomicBool::store
// F374: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/sync/poison.rs:134:0 std::sync::poison::Flag::done
// F375: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1061:0 core::result::Result::expect
// F376: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:426:0 alloc::raw_vec::RawVecInner::with_capacity_in
// F377: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:751:0 alloc::raw_vec::RawVecInner::deallocate
// F378: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:402:0 alloc::raw_vec::impl$3::drop
// F379: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/string.rs:2645:0 alloc::string::impl$26::add
// F380: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:523:0 alloc::raw_vec::RawVecInner::current_memory
// F381: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:116:0 alloc::alloc::dealloc
// F382: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:269:0 alloc::alloc::impl$1::deallocate
// F383: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:753:0 alloc::raw_vec::RawVecInner::deallocate
// F384: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/result.rs:1055:0 core::result::Result::expect
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/visual/assemble_roof_tiles.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: bf92cdb294c88c1d5e163e370ac9ff84c731c3a90de66c50cf20baa8cdc22785
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: assemble_roof_tiles.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2275, 3474, 3475
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2275,"record_offset":2152,"rva":27485952,"va":5396195072}
// 编译实例 RVAs: 0x1a36700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21520516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e87c9fb7fe","size":5,"soft_fail":false,"target_va":5374676336,"va":5396196847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21333482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8167abafe","size":5,"soft_fail":false,"target_va":5374864432,"va":5396197909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21114889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8f7cfbdfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5396196372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcfcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13004796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8049039ff","size":5,"soft_fail":false,"target_va":5383191728,"va":5396196519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a3c350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8cd550000","size":5,"soft_fail":false,"target_va":5396218704,"va":5396196734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421aeb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7831690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e88a807700","size":5,"soft_fail":false,"target_va":5404027712,"va":5396196017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421aeef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7831590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e826807700","size":5,"soft_fail":false,"target_va":5404028656,"va":5396197061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421b3120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7848480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e820c27700","size":5,"soft_fail":false,"target_va":5404045600,"va":5396197115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9a00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15149629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e83d2ae700","size":5,"soft_fail":false,"target_va":5411346944,"va":5396197310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15651066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8fad0ee00","size":5,"soft_fail":false,"target_va":5411847152,"va":5396196081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15343391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e81f1fea00","size":5,"soft_fail":false,"target_va":5411540992,"va":5396197596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15344148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e81422ea00","size":5,"soft_fail":false,"target_va":5411541584,"va":5396197431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t15344328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8c822ea00","size":5,"soft_fail":false,"target_va":5411541776,"va":5396197443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15285014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8163be900","size":5,"soft_fail":false,"target_va":5411482433,"va":5396197414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15345338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8ba26ea00","size":5,"soft_fail":false,"target_va":5411542880,"va":5396197537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8befaffff","size":5,"soft_fail":false,"target_va":5396193808,"va":5396195149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15348129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8a131ea00","size":5,"soft_fail":false,"target_va":5411543360,"va":5396195226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18168338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8eec5eafe","size":5,"soft_fail":false,"target_va":5378028416,"va":5396196749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15295189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8d562e900","size":5,"soft_fail":false,"target_va":5411492368,"va":5396197174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15297998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8ce6de900","size":5,"soft_fail":false,"target_va":5411494800,"va":5396196797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14373013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e86baf24ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5396196608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14372762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e866b024ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5396196773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14810125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8f3031eff","size":5,"soft_fail":false,"target_va":5381385536,"va":5396195656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14804936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e838181eff","size":5,"soft_fail":false,"target_va":5381390640,"va":5396195571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14810779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e865011eff","size":5,"soft_fail":false,"target_va":5381386480,"va":5396197254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14810431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8c1021eff","size":5,"soft_fail":false,"target_va":5381384704,"va":5396195130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15602540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e86c13ee00","size":5,"soft_fail":false,"target_va":5411799872,"va":5396197327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7770465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e861917600","size":5,"soft_fail":false,"target_va":5403968480,"va":5396198010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15339722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396195072,"raw_hex":"e8ca10ea00","size":5,"soft_fail":false,"target_va":5411537104,"va":5396197377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396195072,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396196101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2275,"record_offset":8408,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2275,"record_offset":5480,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":2275,"record_offset":4780,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2275,"record_offset":6096,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":8548,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":7844,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":10208,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":6060,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2275,"record_offset":8464,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2275,"record_offset":11508,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2275,"record_offset":5820,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2275,"record_offset":7900,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":24520,"source_line":117,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2275,"record_offset":8612,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2275,"record_offset":3796,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2275,"record_offset":2364,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2275,"record_offset":2820,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2275,"record_offset":9216,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2275,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":2152,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2275,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":2152,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2275,"record_offset":7968,"parent_record_offset":7940,"ancestor_record_offset":2152,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2275,"record_offset":8000,"parent_record_offset":7968,"ancestor_record_offset":2152,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2275,"record_offset":7940,"parent_record_offset":7900,"ancestor_record_offset":2152,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2275,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":2152,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2275,"record_offset":6408,"parent_record_offset":6384,"ancestor_record_offset":2152,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2275,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":2152,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2275,"record_offset":8124,"parent_record_offset":7900,"ancestor_record_offset":2152,"inlinee":12282,"source_line":1709,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2275,"record_offset":8176,"parent_record_offset":8124,"ancestor_record_offset":2152,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2275,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":2152,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2275,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":2152,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2275,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":2152,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2275,"record_offset":5100,"parent_record_offset":4868,"ancestor_record_offset":2152,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2275,"record_offset":6512,"parent_record_offset":6180,"ancestor_record_offset":2152,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2275,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":2152,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":2275,"record_offset":6180,"parent_record_offset":6128,"ancestor_record_offset":2152,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":2275,"record_offset":4844,"parent_record_offset":4804,"ancestor_record_offset":2152,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2275,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":2152,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2275,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":2152,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2275,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":2152,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2275,"record_offset":7120,"parent_record_offset":6984,"ancestor_record_offset":2152,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2275,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":2152,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2275,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":2152,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2275,"record_offset":7032,"parent_record_offset":7008,"ancestor_record_offset":2152,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2275,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":2152,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2275,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":2152,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2275,"record_offset":7612,"parent_record_offset":6128,"ancestor_record_offset":2152,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2275,"record_offset":6984,"parent_record_offset":6128,"ancestor_record_offset":2152,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":2275,"record_offset":7556,"parent_record_offset":7532,"ancestor_record_offset":2152,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2275,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":2152,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2275,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":2152,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2275,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":2152,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2275,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":2152,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2275,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":2152,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2275,"record_offset":6724,"parent_record_offset":6128,"ancestor_record_offset":2152,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2275,"record_offset":7532,"parent_record_offset":7508,"ancestor_record_offset":2152,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2275,"record_offset":7508,"parent_record_offset":7484,"ancestor_record_offset":2152,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2275,"record_offset":6128,"parent_record_offset":6096,"ancestor_record_offset":2152,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":2275,"record_offset":4804,"parent_record_offset":4780,"ancestor_record_offset":2152,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":2275,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":2152,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":2275,"record_offset":6572,"parent_record_offset":6544,"ancestor_record_offset":2152,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2275,"record_offset":4132,"parent_record_offset":4104,"ancestor_record_offset":2152,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2275,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":2152,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2275,"record_offset":4160,"parent_record_offset":4132,"ancestor_record_offset":2152,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2275,"record_offset":4080,"parent_record_offset":4052,"ancestor_record_offset":2152,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2275,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":2152,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2275,"record_offset":4276,"parent_record_offset":4104,"ancestor_record_offset":2152,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2275,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":2152,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2275,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":2152,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2275,"record_offset":5888,"parent_record_offset":5848,"ancestor_record_offset":2152,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2275,"record_offset":9532,"parent_record_offset":9508,"ancestor_record_offset":2152,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2275,"record_offset":9616,"parent_record_offset":9592,"ancestor_record_offset":2152,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2275,"record_offset":8256,"parent_record_offset":8232,"ancestor_record_offset":2152,"inlinee":11098,"source_line":22,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":2275,"record_offset":8232,"parent_record_offset":8124,"ancestor_record_offset":2152,"inlinee":11097,"source_line":268,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2275,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":2152,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2275,"record_offset":10032,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2275,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":2152,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2275,"record_offset":5532,"parent_record_offset":5504,"ancestor_record_offset":2152,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2275,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":2152,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2275,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":2152,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2275,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":2152,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2275,"record_offset":4916,"parent_record_offset":4868,"ancestor_record_offset":2152,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":2275,"record_offset":2520,"parent_record_offset":2488,"ancestor_record_offset":2152,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2275,"record_offset":9764,"parent_record_offset":9736,"ancestor_record_offset":2152,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2275,"record_offset":9736,"parent_record_offset":9708,"ancestor_record_offset":2152,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2275,"record_offset":6456,"parent_record_offset":6180,"ancestor_record_offset":2152,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2275,"record_offset":3948,"parent_record_offset":3904,"ancestor_record_offset":2152,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":2152,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":2152,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":8872,"parent_record_offset":8612,"ancestor_record_offset":2152,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":2152,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":7392,"parent_record_offset":7220,"ancestor_record_offset":2152,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":5612,"parent_record_offset":5532,"ancestor_record_offset":2152,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":8684,"parent_record_offset":8612,"ancestor_record_offset":2152,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2275,"record_offset":8908,"parent_record_offset":8612,"ancestor_record_offset":2152,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2275,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":2152,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2275,"record_offset":4736,"parent_record_offset":3904,"ancestor_record_offset":2152,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2275,"record_offset":4340,"parent_record_offset":4052,"ancestor_record_offset":2152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2275,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":2152,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2275,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":2152,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2275,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":2152,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2275,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":2152,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2275,"record_offset":3232,"parent_record_offset":3208,"ancestor_record_offset":2152,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2275,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":2152,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2275,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":2152,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2275,"record_offset":8076,"parent_record_offset":8048,"ancestor_record_offset":2152,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2275,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":2152,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2275,"record_offset":7780,"parent_record_offset":6128,"ancestor_record_offset":2152,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2275,"record_offset":7808,"parent_record_offset":6128,"ancestor_record_offset":2152,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2275,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":2152,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2275,"record_offset":6000,"parent_record_offset":5972,"ancestor_record_offset":2152,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2275,"record_offset":4972,"parent_record_offset":4868,"ancestor_record_offset":2152,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2275,"record_offset":5324,"parent_record_offset":4868,"ancestor_record_offset":2152,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2275,"record_offset":8844,"parent_record_offset":8612,"ancestor_record_offset":2152,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2275,"record_offset":6280,"parent_record_offset":6252,"ancestor_record_offset":2152,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2275,"record_offset":6252,"parent_record_offset":6212,"ancestor_record_offset":2152,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2275,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":2152,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2275,"record_offset":5000,"parent_record_offset":4972,"ancestor_record_offset":2152,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2275,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":2152,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2275,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2152,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2275,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":2152,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2275,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2152,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2275,"record_offset":3488,"parent_record_offset":3460,"ancestor_record_offset":2152,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2275,"record_offset":5688,"parent_record_offset":5636,"ancestor_record_offset":2152,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2275,"record_offset":3136,"parent_record_offset":3020,"ancestor_record_offset":2152,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2275,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":2152,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2275,"record_offset":2488,"parent_record_offset":2460,"ancestor_record_offset":2152,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2275,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":2152,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2275,"record_offset":2988,"parent_record_offset":2956,"ancestor_record_offset":2152,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2275,"record_offset":2956,"parent_record_offset":2920,"ancestor_record_offset":2152,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2275,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":2152,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2275,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":2152,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2275,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":2152,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2275,"record_offset":3460,"parent_record_offset":2920,"ancestor_record_offset":2152,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2275,"record_offset":2460,"parent_record_offset":2428,"ancestor_record_offset":2152,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2275,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":2152,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2275,"record_offset":2620,"parent_record_offset":2588,"ancestor_record_offset":2152,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2275,"record_offset":2716,"parent_record_offset":2620,"ancestor_record_offset":2152,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2275,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":2152,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2275,"record_offset":5916,"parent_record_offset":5888,"ancestor_record_offset":2152,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2275,"record_offset":2400,"parent_record_offset":2364,"ancestor_record_offset":2152,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2275,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2152,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2275,"record_offset":3020,"parent_record_offset":2988,"ancestor_record_offset":2152,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2275,"record_offset":3304,"parent_record_offset":3020,"ancestor_record_offset":2152,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2275,"record_offset":3276,"parent_record_offset":3020,"ancestor_record_offset":2152,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2275,"record_offset":3080,"parent_record_offset":3020,"ancestor_record_offset":2152,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2275,"record_offset":9148,"parent_record_offset":9088,"ancestor_record_offset":2152,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2275,"record_offset":8756,"parent_record_offset":8720,"ancestor_record_offset":2152,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2275,"record_offset":8344,"parent_record_offset":8124,"ancestor_record_offset":2152,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2275,"record_offset":8288,"parent_record_offset":8124,"ancestor_record_offset":2152,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2275,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":2152,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2275,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":2152,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2275,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":2152,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2275,"record_offset":5848,"parent_record_offset":5820,"ancestor_record_offset":2152,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2275,"record_offset":8936,"parent_record_offset":8908,"ancestor_record_offset":2152,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2275,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":2152,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":2275,"record_offset":9988,"parent_record_offset":9964,"ancestor_record_offset":2152,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2275,"record_offset":6544,"parent_record_offset":6128,"ancestor_record_offset":2152,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2275,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":2152,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2275,"record_offset":6644,"parent_record_offset":6620,"ancestor_record_offset":2152,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2275,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":2152,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2275,"record_offset":9676,"parent_record_offset":9648,"ancestor_record_offset":2152,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2275,"record_offset":9708,"parent_record_offset":9676,"ancestor_record_offset":2152,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2275,"record_offset":9648,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2275,"record_offset":9508,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2275,"record_offset":10060,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2275,"record_offset":9592,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2275,"record_offset":9480,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2275,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":2152,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2275,"record_offset":9244,"parent_record_offset":9216,"ancestor_record_offset":2152,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2275,"record_offset":9272,"parent_record_offset":9244,"ancestor_record_offset":2152,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2275,"record_offset":9916,"parent_record_offset":9808,"ancestor_record_offset":2152,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2275,"record_offset":9308,"parent_record_offset":9272,"ancestor_record_offset":2152,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2275,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":2152,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2275,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":2152,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2275,"record_offset":10152,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2275,"record_offset":9364,"parent_record_offset":9336,"ancestor_record_offset":2152,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2275,"record_offset":9336,"parent_record_offset":9272,"ancestor_record_offset":2152,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2275,"record_offset":9396,"parent_record_offset":9364,"ancestor_record_offset":2152,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2275,"record_offset":9452,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2275,"record_offset":9808,"parent_record_offset":9396,"ancestor_record_offset":2152,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2275,"record_offset":7440,"parent_record_offset":7416,"ancestor_record_offset":2152,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2275,"record_offset":8720,"parent_record_offset":8612,"ancestor_record_offset":2152,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2275,"record_offset":9088,"parent_record_offset":9064,"ancestor_record_offset":2152,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":2275,"record_offset":8812,"parent_record_offset":8720,"ancestor_record_offset":2152,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":2275,"record_offset":9120,"parent_record_offset":9088,"ancestor_record_offset":2152,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2275,"record_offset":3832,"parent_record_offset":3796,"ancestor_record_offset":2152,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2275,"record_offset":10532,"parent_record_offset":10500,"ancestor_record_offset":2152,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2275,"record_offset":11032,"parent_record_offset":10772,"ancestor_record_offset":2152,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2275,"record_offset":3976,"parent_record_offset":3948,"ancestor_record_offset":2152,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2275,"record_offset":4052,"parent_record_offset":3904,"ancestor_record_offset":2152,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2275,"record_offset":4312,"parent_record_offset":4052,"ancestor_record_offset":2152,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2275,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":2152,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2275,"record_offset":3904,"parent_record_offset":3868,"ancestor_record_offset":2152,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2275,"record_offset":3868,"parent_record_offset":3832,"ancestor_record_offset":2152,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/assemble_roof_tiles.rs
//     evidence={"module":2275,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":2152,"inlinee":93275,"source_line":45,"archive":"_pdb/B/modules/02275.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// 记录形式: compiled_procedure; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3474,"record_offset":31176,"rva":35335328,"va":5404044448}
// 编译实例 RVAs: 0x21b2ca0, 0x21b4510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21400416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e8a074b9fe","size":5,"soft_fail":false,"target_va":5382644528,"va":5404044939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35061679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e85100e9fd","size":5,"soft_fail":false,"target_va":5368983552,"va":5404045226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140542890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29819989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e8abfb38fe","size":5,"soft_fail":false,"target_va":5374224528,"va":5404044512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=2
//     target_aliases=102 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29231083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e815f841fe","size":5,"soft_fail":false,"target_va":5374814320,"va":5404045398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28963076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e8fc0e46fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5404044559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28963786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e8360c46fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5404045285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28528794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e866af4cfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5404044821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a3c070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7826510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e8b29388ff","size":5,"soft_fail":false,"target_va":5396217968,"va":5404044473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8024562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e80e8e85ff","size":5,"soft_fail":false,"target_va":5396020224,"va":5404044781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7503713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e8617f7200","size":5,"soft_fail":false,"target_va":5411548368,"va":5404044650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7495769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e859607200","size":5,"soft_fail":false,"target_va":5411540963,"va":5404045189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7497710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e8ee677200","size":5,"soft_fail":false,"target_va":5411542880,"va":5404045165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7493510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e886577200","size":5,"soft_fail":false,"target_va":5411538608,"va":5404045093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7493869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e8ed587200","size":5,"soft_fail":false,"target_va":5411538960,"va":5404045086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::once::futex::Once::call
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7493937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404044448,"raw_hex":"e831597200","size":5,"soft_fail":false,"target_va":5411538976,"va":5404045034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\set.rs
//     evidence={"module":3474,"record_offset":33468,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":6029,"source_line":905,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$26::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3474,"record_offset":32052,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":19790,"source_line":2642,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3474,"record_offset":31532,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3474,"record_offset":33580,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3474,"record_offset":33788,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3474,"record_offset":31488,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3474,"record_offset":33612,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3474,"record_offset":33496,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":3474,"record_offset":33412,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":11201,"source_line":663,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\once.rs
//     evidence={"module":3474,"record_offset":31368,"parent_record_offset":31176,"ancestor_record_offset":31176,"inlinee":19549,"source_line":145,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3474,"record_offset":31892,"parent_record_offset":31868,"ancestor_record_offset":31176,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3474,"record_offset":31916,"parent_record_offset":31892,"ancestor_record_offset":31176,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3474,"record_offset":32624,"parent_record_offset":32600,"ancestor_record_offset":31176,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3474,"record_offset":31868,"parent_record_offset":31840,"ancestor_record_offset":31176,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3474,"record_offset":32600,"parent_record_offset":32544,"ancestor_record_offset":31176,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32316,"parent_record_offset":32280,"ancestor_record_offset":31176,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32220,"parent_record_offset":32196,"ancestor_record_offset":31176,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":31780,"parent_record_offset":31752,"ancestor_record_offset":31176,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32964,"parent_record_offset":32940,"ancestor_record_offset":31176,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32572,"parent_record_offset":32544,"ancestor_record_offset":31176,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32544,"parent_record_offset":32520,"ancestor_record_offset":31176,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32940,"parent_record_offset":32908,"ancestor_record_offset":31176,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32364,"parent_record_offset":32340,"ancestor_record_offset":31176,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32340,"parent_record_offset":32316,"ancestor_record_offset":31176,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32244,"parent_record_offset":32220,"ancestor_record_offset":31176,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":31840,"parent_record_offset":31808,"ancestor_record_offset":31176,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":31808,"parent_record_offset":31780,"ancestor_record_offset":31176,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":32520,"parent_record_offset":32496,"ancestor_record_offset":31176,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3474,"record_offset":31716,"parent_record_offset":31688,"ancestor_record_offset":31176,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3474,"record_offset":31688,"parent_record_offset":31660,"ancestor_record_offset":31176,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3474,"record_offset":31632,"parent_record_offset":31588,"ancestor_record_offset":31176,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3474,"record_offset":31660,"parent_record_offset":31632,"ancestor_record_offset":31176,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3474,"record_offset":31588,"parent_record_offset":31560,"ancestor_record_offset":31176,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3474,"record_offset":31560,"parent_record_offset":31532,"ancestor_record_offset":31176,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3474,"record_offset":32088,"parent_record_offset":32052,"ancestor_record_offset":31176,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":32160,"parent_record_offset":32136,"ancestor_record_offset":31176,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":32280,"parent_record_offset":32160,"ancestor_record_offset":31176,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":32112,"parent_record_offset":32088,"ancestor_record_offset":31176,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":32404,"parent_record_offset":32160,"ancestor_record_offset":31176,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":32196,"parent_record_offset":32160,"ancestor_record_offset":31176,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":31752,"parent_record_offset":31716,"ancestor_record_offset":31176,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3474,"record_offset":32136,"parent_record_offset":32112,"ancestor_record_offset":31176,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3474,"record_offset":33436,"parent_record_offset":33412,"ancestor_record_offset":31176,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3474,"record_offset":31992,"parent_record_offset":31968,"ancestor_record_offset":31176,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3474,"record_offset":33668,"parent_record_offset":33636,"ancestor_record_offset":31176,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3474,"record_offset":33696,"parent_record_offset":33636,"ancestor_record_offset":31176,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3474,"record_offset":31968,"parent_record_offset":31716,"ancestor_record_offset":31176,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3474,"record_offset":32448,"parent_record_offset":32052,"ancestor_record_offset":31176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3474,"record_offset":34040,"parent_record_offset":33852,"ancestor_record_offset":31176,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3474,"record_offset":31424,"parent_record_offset":31400,"ancestor_record_offset":31176,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3474,"record_offset":34132,"parent_record_offset":34100,"ancestor_record_offset":31176,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3474,"record_offset":33520,"parent_record_offset":33496,"ancestor_record_offset":31176,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3474,"record_offset":31448,"parent_record_offset":31424,"ancestor_record_offset":31176,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3474,"record_offset":34064,"parent_record_offset":34040,"ancestor_record_offset":31176,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3474,"record_offset":34156,"parent_record_offset":34132,"ancestor_record_offset":31176,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3474,"record_offset":33724,"parent_record_offset":33636,"ancestor_record_offset":31176,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3474,"record_offset":33636,"parent_record_offset":33612,"ancestor_record_offset":31176,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3474,"record_offset":33748,"parent_record_offset":33724,"ancestor_record_offset":31176,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":3474,"record_offset":33940,"parent_record_offset":33912,"ancestor_record_offset":31176,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":3474,"record_offset":33912,"parent_record_offset":33884,"ancestor_record_offset":31176,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":3474,"record_offset":33852,"parent_record_offset":33816,"ancestor_record_offset":31176,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":3474,"record_offset":33816,"parent_record_offset":33788,"ancestor_record_offset":31176,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":3474,"record_offset":34100,"parent_record_offset":33816,"ancestor_record_offset":31176,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\once\futex.rs
//     evidence={"module":3474,"record_offset":31400,"parent_record_offset":31368,"ancestor_record_offset":31176,"inlinee":19550,"source_line":77,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":3474,"record_offset":33884,"parent_record_offset":33852,"ancestor_record_offset":31176,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/03474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16C5B
// 首条位置: {"module":2275,"inlinee":93275}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2275,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":10232,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/02275.jsonl.gz"}
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
// 完整结构及原始类型记录: assemble_roof_tiles.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: assemble_roof_tiles.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: assemble_roof_tiles.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// 信息 ID: dff8183cc9a86cc6e6a006df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8b9dad12a9b582e5eb314b57: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0xddfbae -> 0x1a36700 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x1a26696 -> 0x1a36700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0
// 信息 ID: e2f305cd6b231eadf9709126
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::visual::assemble_roof_tiles::assemble_roof_tiles", "captures": "unknown"}
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0 <- system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 5 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21afbbc -> 0x21b2ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b1bf2 -> 0x21b2ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b1e8e -> 0x21b2ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b042c -> 0x21b2ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b05f0 -> 0x21b2ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b3be3 -> 0x21b4510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b3e6c -> 0x21b4510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// 信息 ID: fb0ac1885840d6c12bbf9901
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_roof::visual::assemble_roof_tiles::assemble_roof_tiles", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=2275, inline_record=10232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
